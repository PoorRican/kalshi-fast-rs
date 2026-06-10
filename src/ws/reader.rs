use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
use crate::ws::event::{WsEvent, WsReaderMode};
use crate::ws::low_level::KalshiWsLowLevelClient;
use crate::ws::reconnect::WsReconnectConfig;
use crate::ws::subscription::SubscriptionTracker;
use crate::ws::types::{WsMessageV2, WsRawEvent};

use bytes::Bytes;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, watch};
use tokio::time::{Duration, Instant, MissedTickBehavior, interval_at, sleep, sleep_until};
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
    event_tx: mpsc::Sender<WsEvent>,
    mut outgoing_rx: mpsc::Receiver<Message>,
    mut shutdown_rx: watch::Receiver<bool>,
    mode: WsReaderMode,
) {
    let mut outgoing_closed = false;

    // Keepalive: ping every `ping_interval`; any inbound frame proves the read
    // path and clears the pong deadline. Both arms are gated off when disabled,
    // so the placeholder period never fires.
    let ping_enabled = config.ping_interval.is_some();
    let ping_interval = config.ping_interval.unwrap_or(Duration::from_secs(3600));
    let mut keepalive = interval_at(Instant::now() + ping_interval, ping_interval);
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut pong_deadline: Option<Instant> = None;

    loop {
        if *shutdown_rx.borrow() || event_tx.is_closed() {
            return;
        }

        let result: Result<(), KalshiError> = tokio::select! {
            shutdown = shutdown_rx.changed() => {
                let _ = shutdown;
                return;
            }
            frame = client.next_frame() => {
                match frame {
                    Ok(msg) => {
                        pong_deadline = None;
                        handle_incoming_message(msg, &mut client, &tracker, &event_tx, mode).await
                    }
                    Err(err) => Err(err),
                }
            }
            _ = keepalive.tick(), if ping_enabled => {
                if pong_deadline.is_none() {
                    pong_deadline = Some(Instant::now() + config.pong_timeout);
                }
                client.send_raw(Message::Ping(Vec::new())).await
            }
            _ = sleep_until(pong_deadline.unwrap_or_else(Instant::now)), if pong_deadline.is_some() => {
                Err(KalshiError::Ws("websocket keepalive timed out".to_string()))
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
            if event_tx.is_closed() {
                return;
            }

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
                Ok(()) => {
                    if event_tx.is_closed() {
                        return;
                    }
                    keepalive.reset();
                    pong_deadline = None;
                }
                Err(err) => {
                    if *shutdown_rx.borrow() {
                        return;
                    }
                    let _ = event_tx.send(WsEvent::Disconnected { error: err }).await;
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
    event_tx: &mpsc::Sender<WsEvent>,
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
    event_tx: &mpsc::Sender<WsEvent>,
    mode: WsReaderMode,
) -> Result<(), KalshiError> {
    match mode {
        WsReaderMode::Owned => {
            let msg = WsMessageV2::from_bytes(&bytes)?;
            {
                let mut tracker = tracker.lock().await;
                tracker.handle_message(&msg);
            }
            event_tx
                .send(WsEvent::Message(msg))
                .await
                .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
        }
        WsReaderMode::Raw => {
            if let Ok(control) = serde_json::from_slice::<WsControlMessage>(&bytes) {
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
                .send(WsEvent::Raw(WsRawEvent::new(bytes)))
                .await
                .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
        }
    }

    Ok(())
}

pub(crate) async fn handle_reconnect(
    client: &mut KalshiWsLowLevelClient,
    env: &KalshiEnvironment,
    auth: &Option<KalshiAuth>,
    config: &WsReconnectConfig,
    tracker: &Arc<Mutex<SubscriptionTracker>>,
    event_tx: &mpsc::Sender<WsEvent>,
    shutdown_rx: &mut watch::Receiver<bool>,
) -> Result<(), KalshiError> {
    let mut attempt: u32 = 0;
    let mut last_err = KalshiError::Ws("websocket disconnected".to_string());

    loop {
        if *shutdown_rx.borrow() || event_tx.is_closed() {
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
                _ = event_tx.closed() => {
                    return Ok(());
                }
                changed = shutdown_rx.changed() => {
                    let _ = changed;
                    return Ok(());
                }
            }
        }

        if event_tx.is_closed() {
            return Ok(());
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
            _ = event_tx.closed() => {
                return Ok(());
            }
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
                let _ = event_tx.send(WsEvent::Reconnected { attempt }).await;
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
    use futures::{SinkExt, StreamExt};
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

    #[tokio::test]
    async fn reader_loop_exits_when_event_receiver_closes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept 1");
            let mut ws = accept_async(stream).await.expect("accept ws 1");
            ws.send(Message::Text(ticker_frame("A", "1", 1, 1)))
                .await
                .expect("send 1");
            tokio::time::sleep(Duration::from_millis(50)).await;
            ws.send(Message::Text(ticker_frame("B", "2", 2, 2)))
                .await
                .expect("send 2");

            timeout(Duration::from_millis(250), listener.accept())
                .await
                .is_ok()
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            max_retries: None,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
            jitter: 0.0,
            resubscribe: false,
            ..Default::default()
        };
        let client = KalshiWsLowLevelClient::connect_authenticated(env.clone(), auth.clone())
            .await
            .expect("connect");
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));
        let (event_tx, mut event_rx) = mpsc::channel(1);
        let (_outgoing_tx, outgoing_rx) = mpsc::channel(1);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        let reader = tokio::spawn(reader_loop(
            client,
            env,
            Some(auth),
            config,
            tracker,
            event_tx,
            outgoing_rx,
            shutdown_rx,
            WsReaderMode::Owned,
        ));

        let first = timeout(Duration::from_secs(2), event_rx.recv())
            .await
            .expect("timeout first")
            .expect("first event");
        assert!(matches!(first, WsEvent::Message(_)));
        drop(event_rx);

        timeout(Duration::from_secs(2), reader)
            .await
            .expect("reader should exit after receiver closes")
            .expect("reader task should not panic");
        assert!(!server.await.expect("server should not panic"));
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
            ..Default::default()
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

    #[tokio::test]
    async fn reader_keepalive_sends_pings() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            loop {
                let frame = timeout(Duration::from_secs(2), ws.next())
                    .await
                    .expect("ping within window")
                    .expect("frame")
                    .expect("ok frame");
                if matches!(frame, Message::Ping(_)) {
                    return;
                }
            }
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            ping_interval: Some(Duration::from_millis(25)),
            pong_timeout: Duration::from_secs(1),
            ..Default::default()
        };
        let mut client = KalshiWsClient::connect_authenticated(env, auth, config)
            .await
            .expect("connect");
        let _receiver = client
            .start_reader_v2(WsReaderConfig {
                buffer_size: 4,
                mode: WsReaderMode::Owned,
            })
            .await
            .expect("start reader");

        server.await.expect("server");
    }

    #[tokio::test]
    async fn reader_keepalive_timeout_triggers_reconnect() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept 1");
            // Handshake, then never poll: pings are swallowed, no pong or
            // data ever goes out, but the TCP connection stays open.
            let ws = accept_async(stream).await.expect("accept ws 1");

            let (stream, _) = timeout(Duration::from_secs(2), listener.accept())
                .await
                .expect("reconnect within window")
                .expect("accept 2");
            let _ws2 = accept_async(stream).await.expect("accept ws 2");
            drop(ws);
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            max_retries: Some(3),
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(10),
            jitter: 0.0,
            resubscribe: false,
            ping_interval: Some(Duration::from_millis(25)),
            pong_timeout: Duration::from_millis(50),
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

        let reconnect = timeout(Duration::from_secs(2), receiver.next())
            .await
            .expect("timeout reconnect")
            .expect("event reconnect");
        assert!(matches!(reconnect, WsEvent::Reconnected { .. }));

        server.await.expect("server");
    }

    #[tokio::test]
    async fn reader_default_config_sends_no_pings() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            loop {
                match timeout(Duration::from_millis(300), ws.next()).await {
                    Ok(frame) => {
                        let frame = frame.expect("frame").expect("ok frame");
                        assert!(
                            !matches!(frame, Message::Ping(_)),
                            "ping sent with keepalive disabled"
                        );
                    }
                    // Quiet window elapsed without a ping.
                    Err(_) => return,
                }
            }
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
        let _receiver = client
            .start_reader_v2(WsReaderConfig {
                buffer_size: 4,
                mode: WsReaderMode::Owned,
            })
            .await
            .expect("start reader");

        server.await.expect("server");
    }
}
