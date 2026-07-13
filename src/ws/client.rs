use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
use crate::ws::event::{WsEvent, WsEventReceiver, WsReaderConfig};
use crate::ws::low_level::WsLowLevelClient;
use crate::ws::protocol::{Channel, EventContractProtocol, WsProtocol, parse_control_message};
use crate::ws::reader::reader_loop;
use crate::ws::reconnect::WsReconnectConfig;
use crate::ws::subscription::SubscriptionTracker;
use crate::ws::types::{
    WsListSubscriptionsCmd, WsMessageV2, WsSubscribeCmd, WsSubscriptionParamsV2, WsUnsubscribeCmd,
    WsUnsubscribeParamsV2, WsUpdateSubscriptionCmd, WsUpdateSubscriptionParamsV2,
    validate_subscription, validate_update,
};

use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep, timeout as tokio_timeout};
use tokio_tungstenite::tungstenite::Message;

pub type KalshiWsClient = GenericWsClient<EventContractProtocol>;
pub type MarginWsClient = GenericWsClient<super::protocol::MarginProtocol>;

pub struct GenericWsClient<P: WsProtocol> {
    env: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    client: Option<WsLowLevelClient<P>>,
    config: WsReconnectConfig,
    tracker: Arc<Mutex<SubscriptionTracker<P::SubscribeParams>>>,
    reader: Option<WsEventReceiver<P::Message>>,
    outgoing: Option<mpsc::Sender<Message>>,
    shutdown: Option<watch::Sender<bool>>,
    reader_task: Option<JoinHandle<()>>,
    reader_shutdown_timeout: Duration,
    next_id: u64,
    _protocol: PhantomData<P>,
}

impl<P: WsProtocol> GenericWsClient<P> {
    /// Connect with auth headers for private channels.
    pub async fn connect_authenticated(
        env: KalshiEnvironment,
        auth: KalshiAuth,
        config: WsReconnectConfig,
    ) -> Result<Self, KalshiError> {
        let client =
            WsLowLevelClient::<P>::connect_authenticated(env.clone(), auth.clone()).await?;
        Ok(Self {
            env,
            auth: Some(auth),
            client: Some(client),
            config,
            tracker: Arc::new(Mutex::new(SubscriptionTracker::default())),
            reader: None,
            outgoing: None,
            shutdown: None,
            reader_task: None,
            reader_shutdown_timeout: Duration::from_secs(5),
            next_id: 1,
            _protocol: PhantomData,
        })
    }

    /// Subscribe to one or more channels using this protocol's subscription params.
    pub async fn subscribe(&mut self, params: P::SubscribeParams) -> Result<u64, KalshiError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        {
            let mut tracker = self.tracker.lock().await;
            tracker.record_subscribe_cmd(id, params.clone());
        }

        let cmd = WsSubscribeCmd {
            id,
            cmd: "subscribe",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.send_command(Message::Text(text)).await?;
        Ok(id)
    }

    async fn send_command(&mut self, msg: Message) -> Result<(), KalshiError> {
        if let Some(sender) = &self.outgoing {
            sender
                .send(msg)
                .await
                .map_err(|_| KalshiError::Ws("websocket writer closed".to_string()))?;
            return Ok(());
        }
        if let Some(client) = &mut self.client {
            return client.send_raw(msg).await;
        }
        Err(KalshiError::Ws(
            "websocket client not connected".to_string(),
        ))
    }

    /// Unsubscribe from one or more subscriptions by SID.
    pub async fn unsubscribe(&mut self, params: WsUnsubscribeParamsV2) -> Result<u64, KalshiError> {
        if params.sids.is_empty() {
            return Err(KalshiError::InvalidParams(
                "unsubscribe: at least one sid is required".to_string(),
            ));
        }

        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        {
            let mut tracker = self.tracker.lock().await;
            for sid in &params.sids {
                tracker.drop_active(*sid);
            }
        }

        let cmd = WsUnsubscribeCmd {
            id,
            cmd: "unsubscribe",
            params,
        };
        let text = serde_json::to_string(&cmd)?;
        self.send_command(Message::Text(text)).await?;
        Ok(id)
    }

    /// Request a list of active subscriptions from the server.
    pub async fn list_subscriptions(&mut self) -> Result<u64, KalshiError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        let cmd = WsListSubscriptionsCmd {
            id,
            cmd: "list_subscriptions",
        };
        let text = serde_json::to_string(&cmd)?;
        self.send_command(Message::Text(text)).await?;
        Ok(id)
    }

    pub async fn start_reader(
        &mut self,
        config: WsReaderConfig,
    ) -> Result<WsEventReceiver<P::Message>, KalshiError> {
        if self.reader.is_some() {
            return Err(KalshiError::InvalidParams(
                "websocket reader already started".to_string(),
            ));
        }
        if config.buffer_size == 0 {
            return Err(KalshiError::InvalidParams(
                "websocket reader buffer_size must be > 0".to_string(),
            ));
        }

        let client = self
            .client
            .take()
            .ok_or_else(|| KalshiError::Ws("websocket client not connected".to_string()))?;

        let (event_tx, event_rx) = mpsc::channel(config.buffer_size);
        let (outgoing_tx, outgoing_rx) = mpsc::channel(config.buffer_size);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let tracker = self.tracker.clone();
        let env = self.env.clone();
        let auth = self.auth.clone();
        let reconnect_cfg = self.config.clone();

        let task = tokio::spawn(async move {
            reader_loop::<P>(
                client,
                env,
                auth,
                reconnect_cfg,
                tracker,
                event_tx,
                outgoing_rx,
                shutdown_rx,
                config.mode,
            )
            .await;
        });

        let receiver = WsEventReceiver::new(event_rx);
        self.reader = Some(receiver.clone());
        self.outgoing = Some(outgoing_tx);
        self.shutdown = Some(shutdown_tx);
        self.reader_task = Some(task);

        Ok(receiver)
    }

    /// Configure how long [`close`](Self::close) waits for the reader task.
    pub fn shutdown_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.reader_shutdown_timeout = timeout;
        self
    }

    /// Gracefully close the WebSocket and stop background tasks.
    pub async fn close(&mut self) -> Result<(), KalshiError> {
        if let Some(sender) = &self.outgoing {
            let _ = sender.send(Message::Close(None)).await;
        } else if let Some(client) = &mut self.client {
            let _ = client.close().await;
        }

        self.signal_shutdown();
        self.outgoing = None;

        if let Some(mut task) = self.reader_task.take() {
            match tokio_timeout(self.reader_shutdown_timeout, &mut task).await {
                Ok(joined) => {
                    if let Err(err) = joined
                        && !err.is_cancelled()
                    {
                        return Err(KalshiError::Ws(format!(
                            "websocket reader task failed: {err}",
                        )));
                    }
                }
                Err(_) => {
                    task.abort();
                    return Err(KalshiError::Ws(format!(
                        "websocket reader shutdown timed out after {:?}",
                        self.reader_shutdown_timeout
                    )));
                }
            }
        }

        self.reader = None;
        self.shutdown = None;
        self.client = None;

        Ok(())
    }

    /// Wait for the next event (message, reconnect, or disconnect).
    pub async fn next_event(&mut self) -> Result<WsEvent<P::Message>, KalshiError> {
        if let Some(reader) = &self.reader {
            return reader
                .next()
                .await
                .ok_or_else(|| KalshiError::Ws("websocket reader closed".to_string()));
        }

        let message = {
            let client = self
                .client
                .as_mut()
                .ok_or_else(|| KalshiError::Ws("websocket client not connected".to_string()))?;
            client.next_json_bytes().await
        };

        match message {
            Ok(bytes) => {
                if let Ok(Some(action)) = parse_control_message(&bytes) {
                    self.tracker.lock().await.handle_control_action(action);
                }
                Ok(WsEvent::Message(P::parse_message(&bytes)?))
            }
            Err(err) => self.reconnect_loop(err).await,
        }
    }

    async fn reconnect_loop(
        &mut self,
        mut err: KalshiError,
    ) -> Result<WsEvent<P::Message>, KalshiError> {
        let mut attempt: u32 = 0;
        loop {
            attempt = attempt.saturating_add(1);
            if let Some(max) = self.config.max_retries
                && attempt > max
            {
                return Ok(WsEvent::Disconnected { error: err });
            }

            let delay = self.config.backoff_delay(attempt);
            if !delay.is_zero() {
                sleep(delay).await;
            }

            match self.reconnect().await {
                Ok(()) => return Ok(WsEvent::Reconnected { attempt }),
                Err(e) => {
                    err = e;
                    continue;
                }
            }
        }
    }

    async fn reconnect(&mut self) -> Result<(), KalshiError> {
        let new_client = match &self.auth {
            Some(auth) => {
                WsLowLevelClient::<P>::connect_authenticated(self.env.clone(), auth.clone()).await?
            }
            None => return Err(KalshiError::AuthRequired("WebSocket connection")),
        };
        self.client = Some(new_client);

        if self.config.resubscribe {
            let params = {
                let mut tracker = self.tracker.lock().await;
                tracker.prepare_resubscribe()
            };
            for p in params {
                let client = self
                    .client
                    .as_mut()
                    .ok_or_else(|| KalshiError::Ws("websocket client not connected".to_string()))?;
                let id = client.subscribe(p.clone()).await?;
                let mut tracker = self.tracker.lock().await;
                tracker.record_subscribe_cmd(id, p);
            }
        }

        Ok(())
    }

    fn signal_shutdown(&mut self) {
        if let Some(tx) = &self.shutdown {
            let _ = tx.send(true);
        }
    }
}

impl GenericWsClient<EventContractProtocol> {
    // -----------------------------------------------
    // Connection (no-reader mode)
    // -----------------------------------------------

    /// Connect without auth.
    ///
    /// Kalshi now requires authentication at WebSocket handshake time for all
    /// connections, including subscriptions to public channels.
    pub async fn connect(
        _env: KalshiEnvironment,
        _config: WsReconnectConfig,
    ) -> Result<Self, KalshiError> {
        Err(KalshiError::AuthRequired("WebSocket connection"))
    }

    // -----------------------------------------------
    // Commands
    // -----------------------------------------------

    /// Subscribe to one or more channels. Returns the command `id`.
    pub async fn subscribe_v2(
        &mut self,
        params: WsSubscriptionParamsV2,
    ) -> Result<u64, KalshiError> {
        let needs_auth = params.channels.iter().any(|c| c.is_private());
        if needs_auth && self.auth.is_none() {
            return Err(KalshiError::AuthRequired(
                "WebSocket private channel subscription",
            ));
        }

        validate_subscription(&params)?;

        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        {
            let mut tracker = self.tracker.lock().await;
            tracker.record_subscribe_cmd(id, params.clone());
        }

        let cmd = WsSubscribeCmd {
            id,
            cmd: "subscribe",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.send_command(Message::Text(text)).await?;
        Ok(id)
    }

    /// Unsubscribe from one or more subscriptions by SID. Returns the command `id`.
    pub async fn unsubscribe_v2(
        &mut self,
        params: WsUnsubscribeParamsV2,
    ) -> Result<u64, KalshiError> {
        self.unsubscribe(params).await
    }

    /// Update an existing subscription (e.g. change market tickers).
    pub async fn update_subscription_v2(
        &mut self,
        params: WsUpdateSubscriptionParamsV2,
    ) -> Result<u64, KalshiError> {
        validate_update(&params)?;

        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        {
            let mut tracker = self.tracker.lock().await;
            tracker.apply_update(&params);
        }

        let cmd = WsUpdateSubscriptionCmd {
            id,
            cmd: "update_subscription",
            params,
        };
        let text = serde_json::to_string(&cmd)?;
        self.send_command(Message::Text(text)).await?;
        Ok(id)
    }

    // -----------------------------------------------
    // Reader + event loop
    // -----------------------------------------------

    /// Start the background reader task.
    pub async fn start_reader_v2(
        &mut self,
        config: WsReaderConfig,
    ) -> Result<WsEventReceiver<WsMessageV2>, KalshiError> {
        self.start_reader(config).await
    }

    /// Wait for the next event.
    pub async fn next_event_v2(&mut self) -> Result<WsEvent<WsMessageV2>, KalshiError> {
        self.next_event().await
    }
}

impl<P: WsProtocol> Drop for GenericWsClient<P> {
    fn drop(&mut self) {
        self.signal_shutdown();
        if let Some(task) = &self.reader_task {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KalshiEnvironment;
    use crate::auth::tests::load_test_auth;
    use crate::ws::event::WsReaderMode;
    use crate::ws::types::{WsChannelV2, WsUpdateAction};
    use futures::{SinkExt, StreamExt};
    use serde_json::json;
    use tokio::net::TcpListener;
    use tokio::time::{Duration, Instant};
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::Message;
    use url::Url;

    fn test_env(addr: std::net::SocketAddr) -> KalshiEnvironment {
        KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
            margin_ws_url: format!("ws://{}", addr),
        }
    }

    #[tokio::test]
    async fn inline_next_event_promotes_subscribed_tracker() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            let request = ws
                .next()
                .await
                .expect("subscribe frame")
                .expect("valid subscribe frame");
            let Message::Text(request) = request else {
                panic!("expected text subscribe frame");
            };
            let id =
                serde_json::from_str::<serde_json::Value>(&request).expect("subscribe JSON")["id"]
                    .as_u64()
                    .expect("subscribe command id");
            ws.send(Message::Text(
                json!({"type": "subscribed", "id": id, "sid": 42}).to_string(),
            ))
            .await
            .expect("send subscribed");
        });

        let mut client = KalshiWsClient::connect_authenticated(
            test_env(addr),
            load_test_auth(),
            WsReconnectConfig::default(),
        )
        .await
        .expect("connect");
        client
            .subscribe_v2(WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker],
                market_tickers: Some(vec!["A".to_string()]),
                ..Default::default()
            })
            .await
            .expect("subscribe");

        assert!(matches!(
            client.next_event_v2().await.expect("next event"),
            WsEvent::Message(WsMessageV2::Subscribed {
                id: Some(1),
                sid: Some(42),
            })
        ));
        client
            .update_subscription_v2(WsUpdateSubscriptionParamsV2 {
                action: WsUpdateAction::AddMarkets,
                sid: Some(42),
                sids: None,
                market_ticker: Some("B".to_string()),
                market_tickers: None,
                market_id: None,
                market_ids: None,
                send_initial_snapshot: None,
                skip_ticker_ack: None,
                index_ids: None,
            })
            .await
            .expect("update");
        let replay = client.tracker.lock().await.prepare_resubscribe();
        assert_eq!(
            replay[0].market_tickers,
            Some(vec!["A".to_string(), "B".to_string()])
        );

        server.await.expect("server");
    }

    #[tokio::test]
    async fn inline_reconnect_matches_resubscribe_acknowledgement_id() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept initial");
            let mut ws = accept_async(stream).await.expect("accept initial ws");
            let request = ws
                .next()
                .await
                .expect("initial subscribe frame")
                .expect("valid initial subscribe frame");
            let Message::Text(request) = request else {
                panic!("expected initial text subscribe frame");
            };
            let initial_id = serde_json::from_str::<serde_json::Value>(&request)
                .expect("initial subscribe JSON")["id"]
                .as_u64()
                .expect("initial command id");
            ws.send(Message::Text(
                json!({"type": "subscribed", "id": initial_id, "sid": 41}).to_string(),
            ))
            .await
            .expect("send initial subscribed");
            ws.close(None).await.expect("close initial");

            let (stream, _) = listener.accept().await.expect("accept reconnect");
            let mut ws = accept_async(stream).await.expect("accept reconnect ws");
            let request = ws
                .next()
                .await
                .expect("resubscribe frame")
                .expect("valid resubscribe frame");
            let Message::Text(request) = request else {
                panic!("expected resubscribe text frame");
            };
            let resubscribe_id = serde_json::from_str::<serde_json::Value>(&request)
                .expect("resubscribe JSON")["id"]
                .as_u64()
                .expect("resubscribe command id");
            ws.send(Message::Text(
                json!({"type": "subscribed", "id": resubscribe_id, "sid": 42}).to_string(),
            ))
            .await
            .expect("send resubscribed");
        });

        let reconnect = WsReconnectConfig {
            max_retries: Some(1),
            base_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            jitter: 0.0,
            resubscribe: true,
        };
        let mut client =
            KalshiWsClient::connect_authenticated(test_env(addr), load_test_auth(), reconnect)
                .await
                .expect("connect");
        client
            .subscribe_v2(WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker],
                market_tickers: Some(vec!["A".to_string()]),
                ..Default::default()
            })
            .await
            .expect("subscribe");

        assert!(matches!(
            client
                .next_event_v2()
                .await
                .expect("initial acknowledgement"),
            WsEvent::Message(WsMessageV2::Subscribed {
                id: Some(1),
                sid: Some(41),
            })
        ));
        assert!(matches!(
            client.next_event_v2().await.expect("reconnect"),
            WsEvent::Reconnected { attempt: 1 }
        ));
        assert!(matches!(
            client
                .next_event_v2()
                .await
                .expect("resubscribe acknowledgement"),
            WsEvent::Message(WsMessageV2::Subscribed {
                id: Some(1),
                sid: Some(42),
            })
        ));
        client
            .update_subscription_v2(WsUpdateSubscriptionParamsV2 {
                action: WsUpdateAction::AddMarkets,
                sid: Some(42),
                sids: None,
                market_ticker: Some("B".to_string()),
                market_tickers: None,
                market_id: None,
                market_ids: None,
                send_initial_snapshot: None,
                skip_ticker_ack: None,
                index_ids: None,
            })
            .await
            .expect("update after reconnect");
        let replay = client.tracker.lock().await.prepare_resubscribe();
        assert_eq!(
            replay[0].market_tickers,
            Some(vec!["A".to_string(), "B".to_string()])
        );

        server.await.expect("server");
    }

    #[tokio::test]
    async fn close_stops_reader_without_waiting_for_reconnect_backoff() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");
            let _ = ws.close(None).await;
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
            margin_ws_url: format!("ws://{}", addr),
        };
        let config = WsReconnectConfig {
            max_retries: None,
            base_delay: Duration::from_secs(5),
            max_delay: Duration::from_secs(5),
            jitter: 0.0,
            resubscribe: false,
        };
        let mut client = KalshiWsClient::connect_authenticated(env, auth, config)
            .await
            .expect("connect");

        client
            .start_reader_v2(WsReaderConfig {
                buffer_size: 4,
                mode: WsReaderMode::Owned,
            })
            .await
            .expect("start reader");

        tokio::time::sleep(Duration::from_millis(100)).await;
        client.shutdown_timeout(Duration::from_secs(1));
        let start = Instant::now();
        client.close().await.expect("close");

        assert!(start.elapsed() < Duration::from_secs(1));
        assert!(client.reader_task.is_none());

        server.await.expect("server");
    }
}
