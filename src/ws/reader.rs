use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
use crate::ws::event::{ReaderItem, WsEvent, WsReaderMode};
use crate::ws::low_level::WsLowLevelClient;
use crate::ws::protocol::{WsProtocol, parse_control_message};
use crate::ws::reconnect::WsReconnectConfig;
use crate::ws::subscription::SubscriptionTracker;

use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, watch};
#[cfg(feature = "timed-reader")]
use tokio::time::Instant;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

pub(crate) async fn reader_loop<P: WsProtocol + 'static>(
    mut client: WsLowLevelClient<P>,
    env: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    config: WsReconnectConfig,
    tracker: Arc<Mutex<SubscriptionTracker<P::SubscribeParams>>>,
    event_tx: mpsc::Sender<ReaderItem<P::Message>>,
    mut outgoing_rx: mpsc::Receiver<Message>,
    mut shutdown_rx: watch::Receiver<bool>,
    mode: WsReaderMode,
) {
    let mut outgoing_closed = false;
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
                }
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

pub(crate) async fn handle_incoming_message<P: WsProtocol>(
    msg: Message,
    client: &mut WsLowLevelClient<P>,
    tracker: &Arc<Mutex<SubscriptionTracker<P::SubscribeParams>>>,
    event_tx: &mpsc::Sender<ReaderItem<P::Message>>,
    mode: WsReaderMode,
) -> Result<(), KalshiError> {
    match msg {
        Message::Ping(payload) => {
            client.send_raw(Message::Pong(payload)).await?;
            Ok(())
        }
        Message::Pong(_) => Ok(()),
        Message::Close(_) => Err(KalshiError::Ws("websocket closed".to_string())),
        Message::Text(text) => {
            handle_payload::<P>(Bytes::from(text), tracker, event_tx, mode).await
        }
        Message::Binary(data) => {
            handle_payload::<P>(Bytes::from(data), tracker, event_tx, mode).await
        }
        _ => Ok(()),
    }
}

pub(crate) fn handle_payload<'a, P: WsProtocol>(
    bytes: Bytes,
    tracker: &'a Arc<Mutex<SubscriptionTracker<P::SubscribeParams>>>,
    event_tx: &'a mpsc::Sender<ReaderItem<P::Message>>,
    mode: WsReaderMode,
) -> impl Future<Output = Result<(), KalshiError>> + 'a {
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
    async move {
        match mode {
            WsReaderMode::Owned => {
                let msg = P::parse_message(&bytes)?;
                if let Some(action) = P::control_action(&msg) {
                    tracker.lock().await.handle_control_action(action);
                }
                event_tx
                    .send(wrap_event(WsEvent::Message(msg), available_at))
                    .await
                    .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
            }
            WsReaderMode::Raw => {
                let raw = crate::ws::types::WsRawEvent::new(bytes);
                if let Ok(Some(action)) = parse_control_message(raw.as_slice()) {
                    tracker.lock().await.handle_control_action(action);
                }
                event_tx
                    .send(wrap_event(WsEvent::Raw(raw), available_at))
                    .await
                    .map_err(|_| KalshiError::Ws("websocket reader closed".to_string()))?;
            }
        }
        Ok(())
    }
}

pub(crate) fn wrap_event<M>(
    event: WsEvent<M>,
    available_at: Option<tokio::time::Instant>,
) -> ReaderItem<M> {
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

pub(crate) async fn handle_reconnect<P: WsProtocol>(
    client: &mut WsLowLevelClient<P>,
    env: &KalshiEnvironment,
    auth: &Option<KalshiAuth>,
    config: &WsReconnectConfig,
    tracker: &Arc<Mutex<SubscriptionTracker<P::SubscribeParams>>>,
    event_tx: &mpsc::Sender<ReaderItem<P::Message>>,
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
                _ = event_tx.closed() => return Ok(()),
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
                    WsLowLevelClient::<P>::connect_authenticated(env.clone(), auth.clone()).await
                }
                None => Err(KalshiError::AuthRequired("WebSocket connection")),
            }
        };
        let reconnect = tokio::select! {
            result = reconnect_future => result,
            _ = event_tx.closed() => return Ok(()),
            changed = shutdown_rx.changed() => {
                let _ = changed;
                return Ok(());
            }
        };
        match reconnect {
            Ok(new_client) => {
                *client = new_client;
                if config.resubscribe {
                    let params = tracker.lock().await.prepare_resubscribe();
                    let mut resubscribe_err: Option<KalshiError> = None;
                    for p in params {
                        match client.subscribe(p.clone()).await {
                            Ok(id) => tracker.lock().await.record_subscribe_cmd(id, p),
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
                if *shutdown_rx.borrow() || event_tx.is_closed() {
                    return Ok(());
                }
                let _ = event_tx
                    .send(wrap_event(WsEvent::Reconnected { attempt }, None))
                    .await;
                return Ok(());
            }
            Err(err) => last_err = err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KalshiEnvironment;
    use crate::auth::tests::load_test_auth;
    use crate::ws::event::WsReaderConfig;
    use crate::ws::protocol::EventContractProtocol;
    use crate::ws::types::{WsChannelV2, WsMessageV2, WsSubscriptionParamsV2};
    use crate::ws::{KalshiWsClient, WsReconnectConfig};
    use futures::SinkExt;
    use serde_json::json;
    use tokio::net::TcpListener;
    use tokio::time::{Duration, timeout};
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::Message;
    use url::Url;

    fn item_event<M>(item: ReaderItem<M>) -> WsEvent<M> {
        #[cfg(feature = "timed-reader")]
        {
            item.event
        }
        #[cfg(not(feature = "timed-reader"))]
        {
            item
        }
    }

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
            margin_ws_url: format!("ws://{}", addr),
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
            margin_ws_url: format!("ws://{}", addr),
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

    #[tokio::test]
    async fn raw_reader_forwards_all_frames_and_tracks_subscriptions() {
        let frames = [
            r#"{"type":"subscribed","id":1,"msg":{"channel":"ticker","sid":7}}"#,
            r#"{"type":"ticker","sid":7,"seq":2,"msg":{"market_ticker":"X"}}"#,
            r#"{"type":"mystery"}"#,
        ];
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server_frames = frames.map(|frame| frame.to_owned());
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            for frame in server_frames {
                ws.send(Message::Text(frame)).await.expect("send frame");
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        });

        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{addr}"),
            margin_ws_url: format!("ws://{addr}"),
        };
        let auth = load_test_auth();
        let client = WsLowLevelClient::<EventContractProtocol>::connect_authenticated(env, auth)
            .await
            .expect("connect");
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));
        tracker.lock().await.record_subscribe_cmd(
            1,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker],
                ..Default::default()
            },
        );
        let (event_tx, mut event_rx) = mpsc::channel(4);
        let (_out_tx, out_rx) = mpsc::channel(1);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(reader_loop::<EventContractProtocol>(
            client,
            KalshiEnvironment {
                rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
                ws_url: format!("ws://{addr}"),
                margin_ws_url: format!("ws://{addr}"),
            },
            Some(load_test_auth()),
            WsReconnectConfig {
                max_retries: Some(0),
                ..WsReconnectConfig::default()
            },
            tracker.clone(),
            event_tx,
            out_rx,
            shutdown_rx,
            WsReaderMode::Raw,
        ));
        for expected in frames {
            let item = timeout(Duration::from_secs(2), event_rx.recv())
                .await
                .expect("frame timeout")
                .expect("frame event");
            match item_event(item) {
                WsEvent::Raw(raw) => assert_eq!(raw.as_slice(), expected.as_bytes()),
                other => panic!("expected raw frame, got {other:?}"),
            }
        }
        assert_eq!(tracker.lock().await.prepare_resubscribe().len(), 1);
        task.abort();
        server.await.expect("server");
    }

    #[tokio::test]
    async fn owned_reader_tracks_subscription_from_parsed_message() {
        let subscribed = r#"{"type":"subscribed","id":1,"msg":{"channel":"ticker","sid":7}}"#;
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            ws.send(Message::Text(subscribed.to_owned()))
                .await
                .expect("send frame");
            tokio::time::sleep(Duration::from_millis(50)).await;
        });
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{addr}"),
            margin_ws_url: format!("ws://{addr}"),
        };
        let auth = load_test_auth();
        let client = WsLowLevelClient::<EventContractProtocol>::connect_authenticated(
            env.clone(),
            auth.clone(),
        )
        .await
        .expect("connect");
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));
        tracker.lock().await.record_subscribe_cmd(
            1,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker],
                ..Default::default()
            },
        );
        let (event_tx, mut event_rx) = mpsc::channel(1);
        let (_out_tx, out_rx) = mpsc::channel(1);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(reader_loop::<EventContractProtocol>(
            client,
            env,
            Some(auth),
            WsReconnectConfig::default(),
            tracker.clone(),
            event_tx,
            out_rx,
            shutdown_rx,
            WsReaderMode::Owned,
        ));
        let item = timeout(Duration::from_secs(2), event_rx.recv())
            .await
            .expect("frame timeout")
            .expect("frame event");
        assert!(matches!(
            item_event(item),
            WsEvent::Message(WsMessageV2::Subscribed {
                id: Some(1),
                sid: Some(7),
                ..
            })
        ));
        assert_eq!(tracker.lock().await.prepare_resubscribe().len(), 1);
        task.abort();
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
            margin_ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            max_retries: None,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
            jitter: 0.0,
            resubscribe: false,
        };
        let client = WsLowLevelClient::<EventContractProtocol>::connect_authenticated(
            env.clone(),
            auth.clone(),
        )
        .await
        .expect("connect");
        let tracker = Arc::new(Mutex::new(SubscriptionTracker::default()));
        let (event_tx, mut event_rx) = mpsc::channel(1);
        let (_outgoing_tx, outgoing_rx) = mpsc::channel(1);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        let reader = tokio::spawn(reader_loop::<EventContractProtocol>(
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
        assert!(matches!(item_event(first), WsEvent::Message(_)));
        drop(event_rx);

        timeout(Duration::from_secs(2), reader)
            .await
            .expect("reader should exit after receiver closes")
            .expect("reader task should not panic");
        assert!(!server.await.expect("server should not panic"));
    }
}
