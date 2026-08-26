use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
use crate::ws::event::{ReaderItem, WsEvent, WsReaderMode};
use crate::ws::low_level::KalshiWsLowLevelClient;
use crate::ws::reconnect::WsReconnectConfig;
use crate::ws::subscription::SubscriptionTracker;
use crate::ws::types::{WsMessageV2, WsRawEvent};

use bytes::Bytes;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, watch};
#[cfg(feature = "timed-reader")]
use tokio::time::Instant;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum WsControlMessage {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        #[serde(default)]
        msg: Option<WsControlSubscribedMsg>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed { sid: Option<u64> },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WsControlSubscribedMsg {
    #[allow(dead_code)]
    channel: Option<String>,
    #[serde(default)]
    sid: Option<u64>,
}

pub(crate) async fn reader_loop(
    mut client: KalshiWsLowLevelClient,
    env: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    config: WsReconnectConfig,
    tracker: Arc<Mutex<SubscriptionTracker>>,
    event_tx: mpsc::Sender<ReaderItem>,
    mut outgoing_rx: mpsc::Receiver<Message>,
    mut shutdown_rx: watch::Receiver<bool>,
    mode: WsReaderMode,
) {
    let mut outgoing_closed = false;

    loop {
        if *shutdown_rx.borrow() {
            return;
        }

        let result: Result<(), KalshiError> = tokio::select! {
            shutdown = shutdown_rx.changed() => {
                let _ = shutdown;
                return;
            }
            frame = client.next_frame() => {
                match frame {
                    Ok(msg) => handle_incoming_message(msg, &mut client, &tracker, &event_tx, mode).await,
                    Err(err) => Err(err),
                }
            }
            maybe_out = outgoing_rx.recv(), if !outgoing_closed => {
                match maybe_out {
                    Some(msg) => client.send_raw(msg).await,
                    None => {
                        outgoing_closed = true;
                        Ok(())
                    }
                }
            }
        };

        if let Err(_err) = result {
            match handle_reconnect(
                &mut client,
                &env,
                &auth,
                &config,
                &tracker,
                &event_tx,
                &mut shutdown_rx,
            )
            .await
            {
                Ok(()) => {}
                Err(err) => {
                    if *shutdown_rx.borrow() {
                        return;
                    }
                    let _ = event_tx
                        .send(wrap_event(WsEvent::Disconnected { error: err }, None))
                        .await;
                    return;
                }
            }
        }
    }
}

pub(crate) async fn handle_incoming_message(
    msg: Message,
    client: &mut KalshiWsLowLevelClient,
    tracker: &Arc<Mutex<SubscriptionTracker>>,
    event_tx: &mpsc::Sender<ReaderItem>,
    mode: WsReaderMode,
) -> Result<(), KalshiError> {
    match msg {
        Message::Ping(payload) => {
            client.send_raw(Message::Pong(payload)).await?;
            Ok(())
        }
        Message::Pong(_) => Ok(()),
        Message::Close(_) => Err(KalshiError::Ws("websocket closed".to_string())),
        Message::Text(text) => handle_payload(Bytes::from(text), tracker, event_tx, mode).await,
        Message::Binary(data) => handle_payload(Bytes::from(data), tracker, event_tx, mode).await,
        _ => Ok(()),
    }
}

pub(crate) async fn handle_payload(
    bytes: Bytes,
    tracker: &Arc<Mutex<SubscriptionTracker>>,
    event_tx: &mpsc::Sender<ReaderItem>,
    mode: WsReaderMode,
) -> Result<(), KalshiError> {
    match mode {
        WsReaderMode::Owned => {
            let msg = WsMessageV2::from_bytes(&bytes)?;
            let available_at = {
                #[cfg(feature = "timed-reader")]
                {
                    Some(Instant::now())
                }
                #[cfg(not(feature = "timed-reader"))]
                {
                    None
                }
            };
            {
                let mut tracker = tracker.lock().await;
                tracker.handle_message(&msg);
            }
            event_tx
                .send(wrap_event(WsEvent::Message(msg), available_at))
                .await
                .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
        }
        WsReaderMode::Raw => {
            let raw_event = WsRawEvent::new(bytes);
            let available_at = {
                #[cfg(feature = "timed-reader")]
                {
                    Some(Instant::now())
                }
                #[cfg(not(feature = "timed-reader"))]
                {
                    None
                }
            };
            if let Ok(control) = serde_json::from_slice::<WsControlMessage>(raw_event.as_slice()) {
                let mut tracker = tracker.lock().await;
                match control {
                    WsControlMessage::Subscribed { id, sid, msg } => {
                        tracker
                            .handle_subscribed(id, sid.or_else(|| msg.and_then(|value| value.sid)));
                    }
                    WsControlMessage::Unsubscribed { sid } => {
                        tracker.handle_unsubscribed(sid);
                    }
                    WsControlMessage::Other => {}
                }
            }

            event_tx
                .send(wrap_event(WsEvent::Raw(raw_event), available_at))
                .await
                .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
        }
    }

    Ok(())
}

pub(crate) fn wrap_event(event: WsEvent, available_at: Option<tokio::time::Instant>) -> ReaderItem {
    #[cfg(feature = "timed-reader")]
    {
        crate::ws::event::WsTimedEvent {
            event,
            available_at: available_at.unwrap_or_else(Instant::now),
        }
    }

    #[cfg(not(feature = "timed-reader"))]
    {
        let _ = available_at;
        event
    }
}

pub(crate) async fn handle_reconnect(
    client: &mut KalshiWsLowLevelClient,
    env: &KalshiEnvironment,
    auth: &Option<KalshiAuth>,
    config: &WsReconnectConfig,
    tracker: &Arc<Mutex<SubscriptionTracker>>,
    event_tx: &mpsc::Sender<ReaderItem>,
    shutdown_rx: &mut watch::Receiver<bool>,
) -> Result<(), KalshiError> {
    let mut attempt: u32 = 0;
    let mut last_err = KalshiError::Ws("websocket disconnected".to_string());

    loop {
        if *shutdown_rx.borrow() {
            return Ok(());
        }

        attempt = attempt.saturating_add(1);
        if let Some(max) = config.max_retries
            && attempt > max
        {
            return Err(last_err);
        }

        let delay = config.backoff_delay(attempt);
        if !delay.is_zero() {
            tokio::select! {
                _ = sleep(delay) => {}
                changed = shutdown_rx.changed() => {
                    let _ = changed;
                    return Ok(());
                }
            }
        }

        let reconnect_future = async {
            match auth {
                Some(auth) => {
                    KalshiWsLowLevelClient::connect_authenticated(env.clone(), auth.clone()).await
                }
                None => Err(KalshiError::AuthRequired("WebSocket connection")),
            }
        };
        let reconnect = tokio::select! {
            result = reconnect_future => result,
            changed = shutdown_rx.changed() => {
                let _ = changed;
                return Ok(());
            }
        };

        match reconnect {
            Ok(new_client) => {
                *client = new_client;
                if config.resubscribe {
                    let params = {
                        let mut tracker = tracker.lock().await;
                        tracker.prepare_resubscribe()
                    };
                    let mut resubscribe_err: Option<KalshiError> = None;
                    for p in params {
                        match client.subscribe_v2(p.clone()).await {
                            Ok(id) => {
                                let mut tracker = tracker.lock().await;
                                tracker.record_subscribe_cmd(id, p);
                            }
                            Err(err) => {
                                resubscribe_err = Some(err);
                                break;
                            }
                        }
                    }
                    if let Some(err) = resubscribe_err {
                        last_err = err;
                        continue;
                    }
                }

                if *shutdown_rx.borrow() {
                    return Ok(());
                }
                let _ = event_tx
                    .send(wrap_event(WsEvent::Reconnected { attempt }, None))
                    .await;
                return Ok(());
            }
            Err(err) => {
                last_err = err;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KalshiEnvironment;
    use crate::auth::tests::load_test_auth;
    use crate::ws::event::{WsReaderConfig, WsReaderMode};
    use crate::ws::{KalshiWsClient, WsReconnectConfig};
    use futures::SinkExt;
    use serde_json::json;
    use tokio::net::TcpListener;
    use tokio::time::{Duration, timeout};
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::Message;
    use url::Url;

    fn ticker_frame(market_ticker: &str, market_id: &str, sid: u64, seq: u64) -> String {
        json!({
            "type": "ticker",
            "sid": sid,
            "seq": seq,
            "msg": {
                "market_ticker": market_ticker,
                "market_id": market_id,
                "price_dollars": "0.01",
                "yes_bid_dollars": "0.01",
                "yes_ask_dollars": "0.02",
                "yes_bid_size_fp": "1.00",
                "yes_ask_size_fp": "2.00",
                "last_trade_size_fp": "1.00",
                "volume_fp": "0.00",
                "open_interest_fp": "0.00",
                "dollar_volume": 0,
                "dollar_open_interest": 0,
                "ts": 0,
                "ts_ms": 0,
                "time": "1970-01-01T00:00:00Z"
            }
        })
        .to_string()
    }

    #[tokio::test]
    async fn reader_backpressure_preserves_messages() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            ws.send(Message::Text(ticker_frame("A", "1", 1, 1)))
                .await
                .expect("send 1");
            ws.send(Message::Text(ticker_frame("B", "2", 2, 2)))
                .await
                .expect("send 2");
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),

            ws_url: format!("ws://{}", addr),
        };
        let mut client =
            KalshiWsClient::connect_authenticated(env, auth, WsReconnectConfig::default())
                .await
                .expect("connect");

        let receiver = client
            .start_reader_v2(WsReaderConfig {
                buffer_size: 1,
                mode: WsReaderMode::Owned,
            })
            .await
            .expect("start reader");

        let first = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout 1")
            .expect("event 1");
        let second = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout 2")
            .expect("event 2");

        assert!(matches!(first, WsEvent::Message(_)));
        assert!(matches!(second, WsEvent::Message(_)));

        server.await.expect("server");
    }

    #[cfg(feature = "timed-reader")]
    fn ticker_sequence(event: &WsEvent) -> u64 {
        match event {
            WsEvent::Message(crate::ws::types::WsMessageV2::Data(
                crate::ws::types::WsDataMessageV2::Ticker {
                    seq: Some(sequence),
                    ..
                },
            )) => *sequence,
            other => panic!("expected ticker with sequence, got {other:?}"),
        }
    }

    #[cfg(feature = "timed-reader")]
    #[tokio::test]
    async fn timed_reader_stamps_before_backpressure_and_preserves_sequence() {
        let (event_tx, event_rx) = mpsc::channel(1);
        let receiver = crate::ws::event::WsEventReceiver::new(event_rx);
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));

        let first = WsEvent::Message(
            WsMessageV2::from_bytes(&Bytes::from(ticker_frame("A", "1", 1, 1)))
                .expect("decode first"),
        );
        event_tx
            .send(wrap_event(first, None))
            .await
            .expect("fill channel");

        let tracker_guard = tracker.lock().await;
        let blocked_tracker = Arc::clone(&tracker);
        let blocked_tx = event_tx.clone();
        let blocked = tokio::spawn(async move {
            handle_payload(
                Bytes::from(ticker_frame("B", "2", 2, 2)),
                &blocked_tracker,
                &blocked_tx,
                WsReaderMode::Owned,
            )
            .await
        });
        tokio::task::yield_now().await;
        drop(tracker_guard);

        let tracker_guard = tracker.lock().await;
        drop(tracker_guard);
        let released_at = tokio::time::Instant::now();

        let first = receiver.next().await.expect("first event");
        let second = receiver.next_timed().await.expect("second event");
        blocked
            .await
            .expect("blocked task join")
            .expect("blocked task result");

        handle_payload(
            Bytes::from(ticker_frame("C", "3", 3, 3)),
            &tracker,
            &event_tx,
            WsReaderMode::Owned,
        )
        .await
        .expect("third payload");
        let third = receiver.next().await.expect("third event");

        assert_eq!(ticker_sequence(&first), 1);
        assert_eq!(ticker_sequence(&second.event), 2);
        assert_eq!(ticker_sequence(&third), 3);
        assert!(
            second.available_at < released_at,
            "second event timestamp must precede the channel release"
        );
    }

    #[cfg(feature = "timed-reader")]
    #[tokio::test]
    async fn timed_raw_reader_stamps_before_tracker_work() {
        let (event_tx, event_rx) = mpsc::channel(1);
        let receiver = crate::ws::event::WsEventReceiver::new(event_rx);
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));
        let tracker_guard = tracker.lock().await;
        let blocked_tracker = Arc::clone(&tracker);
        let payload = ticker_frame("A", "1", 1, 1);

        let blocked = tokio::spawn(async move {
            handle_payload(
                Bytes::from(payload),
                &blocked_tracker,
                &event_tx,
                WsReaderMode::Raw,
            )
            .await
        });
        tokio::time::sleep(Duration::from_millis(10)).await;
        let tracker_released_at = tokio::time::Instant::now();
        drop(tracker_guard);

        let event = receiver.next_timed().await.expect("raw event");
        blocked
            .await
            .expect("blocked task join")
            .expect("blocked task result");

        assert!(matches!(event.event, WsEvent::Raw(_)));
        assert!(
            event.available_at < tracker_released_at,
            "raw event timestamp must precede tracker work"
        );
    }

    #[tokio::test]
    async fn reader_reconnect_emits_reconnected_event() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept 1");
            let mut ws = accept_async(stream).await.expect("accept ws 1");
            ws.send(Message::Text(ticker_frame("A", "1", 1, 1)))
                .await
                .expect("send 1");
            ws.close(None).await.expect("close 1");

            let (stream, _) = listener.accept().await.expect("accept 2");
            let mut ws = accept_async(stream).await.expect("accept ws 2");
            ws.send(Message::Text(ticker_frame("B", "2", 2, 2)))
                .await
                .expect("send 2");
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            max_retries: Some(3),
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(50),
            jitter: 0.0,
            resubscribe: false,
        };
        let mut client = KalshiWsClient::connect_authenticated(env, auth, config)
            .await
            .expect("connect");

        let receiver = client
            .start_reader_v2(WsReaderConfig {
                buffer_size: 4,
                mode: WsReaderMode::Owned,
            })
            .await
            .expect("start reader");

        let first = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout 1")
            .expect("event 1");
        assert!(matches!(first, WsEvent::Message(_)));

        let reconnect = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout reconnect")
            .expect("event reconnect");
        assert!(matches!(reconnect, WsEvent::Reconnected { .. }));

        let second = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout 2")
            .expect("event 2");
        assert!(matches!(second, WsEvent::Message(_)));

        server.await.expect("server");
    }
}
