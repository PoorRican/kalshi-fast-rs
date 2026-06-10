use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
use crate::ws::event::{WsEvent, WsEventReceiver, WsReaderConfig};
use crate::ws::low_level::KalshiWsLowLevelClient;
use crate::ws::reader::reader_loop;
use crate::ws::reconnect::WsReconnectConfig;
use crate::ws::subscription::SubscriptionTracker;
use crate::ws::types::{
    WsListSubscriptionsCmd, WsSubscribeCmd, WsSubscriptionParamsV2, WsUnsubscribeCmd,
    WsUnsubscribeParamsV2, WsUpdateSubscriptionCmd, WsUpdateSubscriptionParamsV2,
    validate_subscription, validate_update,
};

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep, timeout as tokio_timeout};
use tokio_tungstenite::tungstenite::Message;

/// High-level WebSocket client with automatic reconnection and resubscription.
///
/// Wraps [`KalshiWsLowLevelClient`] and adds:
/// - Exponential-backoff reconnection (configurable via [`WsReconnectConfig`])
/// - Subscription tracking — active channels are resubscribed after reconnect
/// - A unified event loop via [`next_event`](Self::next_event)
///
/// # Example
///
/// ```no_run
/// use kalshi_fast::{
///     KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannelV2,
///     WsDataMessageV2, WsEvent, WsMessageV2, WsReconnectConfig, WsSubscriptionParamsV2,
/// };
///
/// # async fn run() -> Result<(), kalshi_fast::KalshiError> {
/// let auth = KalshiAuth::from_pem_file(
///     std::env::var("KALSHI_KEY_ID").unwrap(),
///     std::env::var("KALSHI_PRIVATE_KEY_PATH").unwrap(),
/// )?;
/// let mut ws = KalshiWsClient::connect_authenticated(
///     KalshiEnvironment::demo(),
///     auth,
///     WsReconnectConfig::default(),
/// ).await?;
///
/// ws.subscribe_v2(WsSubscriptionParamsV2 {
///     channels: vec![WsChannelV2::Trade],
///     ..Default::default()
/// }).await?;
///
/// loop {
///     match ws.next_event_v2().await? {
///         WsEvent::Message(WsMessageV2::Data(WsDataMessageV2::Trade { msg, .. })) => {
///             println!("trade: {} @ {}", msg.market_ticker, msg.yes_price_dollars);
///         }
///         WsEvent::Disconnected { .. } => break,
///         _ => {}
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct KalshiWsClient {
    env: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    client: Option<KalshiWsLowLevelClient>,
    config: WsReconnectConfig,
    tracker: Arc<Mutex<SubscriptionTracker>>,
    reader: Option<WsEventReceiver>,
    outgoing: Option<mpsc::Sender<Message>>,
    shutdown: Option<watch::Sender<bool>>,
    reader_task: Option<JoinHandle<()>>,
    reader_shutdown_timeout: Duration,
    next_id: u64,
}

impl KalshiWsClient {
    // -----------------------------------------------
    // Connection
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

    /// Connect with auth headers for private channels.
    ///
    /// **Requires auth.**
    pub async fn connect_authenticated(
        env: KalshiEnvironment,
        auth: KalshiAuth,
        config: WsReconnectConfig,
    ) -> Result<Self, KalshiError> {
        let client =
            KalshiWsLowLevelClient::connect_authenticated(env.clone(), auth.clone()).await?;
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
        })
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

    // -----------------------------------------------
    // Commands
    // -----------------------------------------------

    /// Subscribe to one or more channels. Returns the command `id`.
    ///
    /// The subscription is tracked internally so it can be resubscribed
    /// automatically after a reconnect.
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

    /// Update an existing subscription (e.g. change market tickers). Returns the command `id`.
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

    /// Request a list of active subscriptions from the server. Returns the command `id`.
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

    pub async fn start_reader_v2(
        &mut self,
        config: WsReaderConfig,
    ) -> Result<WsEventReceiver, KalshiError> {
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
        let mode = config.mode;

        let task = tokio::spawn(async move {
            reader_loop(
                client,
                env,
                auth,
                reconnect_cfg,
                tracker,
                event_tx,
                outgoing_rx,
                shutdown_rx,
                mode,
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

    // -----------------------------------------------
    // Event loop
    // -----------------------------------------------

    /// Wait for the next event (message, reconnect, or disconnect).
    ///
    /// This is the primary event-loop driver. On connection loss it
    /// automatically attempts reconnection per [`WsReconnectConfig`],
    /// returning [`WsEvent::Reconnected`] on success or
    /// [`WsEvent::Disconnected`] when retries are exhausted.
    pub async fn next_event_v2(&mut self) -> Result<WsEvent, KalshiError> {
        if let Some(reader) = &self.reader {
            return reader
                .next()
                .await
                .ok_or_else(|| KalshiError::Ws("websocket reader closed".to_string()));
        }

        let client = self
            .client
            .as_mut()
            .ok_or_else(|| KalshiError::Ws("websocket client not connected".to_string()))?;

        match client.next_message_v2().await {
            Ok(msg) => {
                let mut tracker = self.tracker.lock().await;
                tracker.handle_message(&msg);
                Ok(WsEvent::Message(msg))
            }
            Err(err) => self.reconnect_loop(err).await,
        }
    }

    async fn reconnect_loop(&mut self, mut err: KalshiError) -> Result<WsEvent, KalshiError> {
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
                KalshiWsLowLevelClient::connect_authenticated(self.env.clone(), auth.clone())
                    .await?
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
                let id = self.next_id;
                self.next_id = self.next_id.saturating_add(1);

                let cmd = WsSubscribeCmd {
                    id,
                    cmd: "subscribe",
                    params: p.clone(),
                };
                let text = serde_json::to_string(&cmd)?;
                let client = self
                    .client
                    .as_mut()
                    .ok_or_else(|| KalshiError::Ws("websocket client not connected".to_string()))?;
                client.send_raw(Message::Text(text)).await?;
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

impl Drop for KalshiWsClient {
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
    use crate::ws::event::{WsReaderConfig, WsReaderMode};
    use tokio::net::TcpListener;
    use tokio::time::{Duration, Instant};
    use tokio_tungstenite::accept_async;
    use url::Url;

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
        };
        let config = WsReconnectConfig {
            max_retries: None,
            base_delay: Duration::from_secs(5),
            max_delay: Duration::from_secs(5),
            jitter: 0.0,
            resubscribe: false,
            ..Default::default()
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
