use crate::auth::KalshiAuth;
use crate::env::KalshiEnvironment;
use crate::error::KalshiError;
#[cfg(doc)]
use crate::ws::KalshiWsClient;
use crate::ws::protocol::{Channel, WsProtocol};
use crate::ws::types::{
    WsListSubscriptionsCmd, WsMessageV2, WsSubscribeCmd, WsSubscriptionParamsV2, WsUnsubscribeCmd,
    WsUnsubscribeParamsV2, WsUpdateSubscriptionCmd, WsUpdateSubscriptionParamsV2,
    validate_subscription, validate_update,
};

use bytes::Bytes;

pub type KalshiWsLowLevelClient = WsLowLevelClient<super::protocol::EventContractProtocol>;
use futures::{SinkExt, StreamExt};
use std::marker::PhantomData;
use std::time::Duration;
use tokio_tungstenite::client_async_tls_with_config;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::{HeaderValue, Request, header::HeaderName};

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct WsLowLevelClient<P: WsProtocol> {
    write: futures::stream::SplitSink<WsStream, Message>,
    read: futures::stream::SplitStream<WsStream>,
    next_id: u64,
    authenticated: bool,
    _protocol: PhantomData<P>,
}

/// Build the transport error for a peer Close frame. Keeps the close code and
/// reason so disconnect logs show why the server closed the socket.
pub(crate) fn close_error(
    frame: Option<&tokio_tungstenite::tungstenite::protocol::CloseFrame>,
) -> KalshiError {
    match frame {
        Some(frame) => KalshiError::Ws(format!(
            "websocket closed by peer: code {} reason {:?}",
            u16::from(frame.code),
            frame.reason
        )),
        None => KalshiError::Ws("websocket closed by peer without a close frame".to_string()),
    }
}

impl<P: WsProtocol> WsLowLevelClient<P> {
    pub async fn connect_authenticated(
        env: KalshiEnvironment,
        auth: KalshiAuth,
    ) -> Result<Self, KalshiError> {
        Self::connect_authenticated_within(env, auth, None).await
    }

    /// Connect and authenticate. When `connect_timeout` is set, the whole
    /// attempt (TCP, TLS and upgrade) is bounded by it and a timed-out attempt
    /// returns `KalshiError::Ws` so the caller's retry policy applies.
    pub(crate) async fn connect_authenticated_within(
        env: KalshiEnvironment,
        auth: KalshiAuth,
        connect_timeout: Option<Duration>,
    ) -> Result<Self, KalshiError> {
        let attempt = Self::connect_attempt(env, auth);
        match connect_timeout {
            None => attempt.await,
            Some(limit) => tokio::time::timeout(limit, attempt)
                .await
                .unwrap_or_else(|_| {
                    Err(KalshiError::Ws(format!(
                        "websocket connect timed out after {limit:?}"
                    )))
                }),
        }
    }

    async fn connect_attempt(
        env: KalshiEnvironment,
        auth: KalshiAuth,
    ) -> Result<Self, KalshiError> {
        let url = P::ws_url(&env);
        let mut req: Request<()> = url
            .into_client_request()
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        // Open TCP before signing so the signed timestamp is as fresh as the
        // library allows. Kalshi rejects a stale KALSHI-ACCESS-TIMESTAMP with
        // 401, and a stalled TCP connect must not age the signature.
        let uri = req.uri().clone();
        let host = uri
            .host()
            .ok_or_else(|| KalshiError::Ws("websocket URL has no host".to_string()))?;
        let default_port = if uri.scheme_str() == Some("wss") {
            443
        } else {
            80
        };
        let port = uri.port_u16().unwrap_or(default_port);
        let tcp = tokio::net::TcpStream::connect((host, port))
            .await
            .map_err(|e| KalshiError::Ws(format!("tcp connect to {host}:{port} failed: {e}")))?;

        let headers = auth.build_headers("GET", P::signing_path())?;

        req.headers_mut().insert(
            HeaderName::from_static("kalshi-access-key"),
            HeaderValue::from_str(&headers.key).map_err(|e| KalshiError::Header(e.to_string()))?,
        );
        req.headers_mut().insert(
            HeaderName::from_static("kalshi-access-signature"),
            HeaderValue::from_str(&headers.signature)
                .map_err(|e| KalshiError::Header(e.to_string()))?,
        );
        req.headers_mut().insert(
            HeaderName::from_static("kalshi-access-timestamp"),
            HeaderValue::from_str(&headers.timestamp_ms)
                .map_err(|e| KalshiError::Header(e.to_string()))?,
        );

        let (ws_stream, _resp) = client_async_tls_with_config(req, tcp, None, None)
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        let (write, read) = ws_stream.split();
        Ok(Self {
            write,
            read,
            next_id: 1,
            authenticated: true,
            _protocol: PhantomData,
        })
    }
    pub fn next_cmd_id(&self) -> u64 {
        self.next_id
    }

    pub async fn send_raw(&mut self, msg: Message) -> Result<(), KalshiError> {
        self.write
            .send(msg)
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;
        self.write
            .flush()
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;
        Ok(())
    }

    /// Read the next raw frame.
    ///
    /// Blocks until a frame arrives — there is no read deadline at this
    /// layer. Wrap in [`tokio::time::timeout`] if you need one, or use
    /// [`KalshiWsClient`] with
    /// [`WsReconnectConfig::ping_interval`](crate::WsReconnectConfig::ping_interval)
    /// for transport keepalive.
    pub async fn next_frame(&mut self) -> Result<Message, KalshiError> {
        match self.read.next().await {
            Some(Ok(msg)) => Ok(msg),
            Some(Err(e)) => Err(KalshiError::Ws(e.to_string())),
            None => Err(KalshiError::Ws("websocket stream ended".to_string())),
        }
    }

    pub async fn next_json_bytes(&mut self) -> Result<Bytes, KalshiError> {
        loop {
            let msg = self.next_frame().await?;
            match msg {
                Message::Text(s) => return Ok(Bytes::from(s)),
                Message::Binary(b) => return Ok(Bytes::from(b)),
                Message::Ping(payload) => {
                    self.send_raw(Message::Pong(payload)).await?;
                }
                Message::Pong(_) => {}
                Message::Close(frame) => {
                    return Err(close_error(frame.as_ref()));
                }
                _ => {}
            }
        }
    }

    /// Subscribe to one or more channels. Returns the command `id`.
    pub async fn subscribe(&mut self, params: P::SubscribeParams) -> Result<u64, KalshiError> {
        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsSubscribeCmd {
            id,
            cmd: "subscribe",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    pub async fn unsubscribe(&mut self, params: WsUnsubscribeParamsV2) -> Result<u64, KalshiError> {
        if params.sids.is_empty() {
            return Err(KalshiError::InvalidParams(
                "unsubscribe: at least one sid is required".to_string(),
            ));
        }

        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsUnsubscribeCmd {
            id,
            cmd: "unsubscribe",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    pub async fn list_subscriptions(&mut self) -> Result<u64, KalshiError> {
        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsListSubscriptionsCmd {
            id,
            cmd: "list_subscriptions",
        };
        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    pub async fn next_message(&mut self) -> Result<P::Message, KalshiError> {
        let bytes = self.next_json_bytes().await?;
        P::parse_message(&bytes)
    }

    pub async fn close(&mut self) -> Result<(), KalshiError> {
        self.send_raw(Message::Close(None)).await
    }
}

impl WsLowLevelClient<super::protocol::EventContractProtocol> {
    /// Connect without auth. Kalshi requires authentication at handshake time.
    pub async fn connect(_env: KalshiEnvironment) -> Result<Self, KalshiError> {
        Err(KalshiError::AuthRequired("WebSocket connection"))
    }

    /// Read the next message envelope.
    pub async fn next_envelope(&mut self) -> Result<super::types::WsEnvelope, KalshiError> {
        let bytes = self.next_json_bytes().await?;
        serde_json::from_slice(&bytes)
            .map_err(|source| KalshiError::parse_json("websocket envelope", &bytes, source))
    }

    pub async fn subscribe_v2(
        &mut self,
        params: WsSubscriptionParamsV2,
    ) -> Result<u64, KalshiError> {
        let needs_auth = params.channels.iter().any(|c| c.is_private());
        if needs_auth && !self.authenticated {
            return Err(KalshiError::AuthRequired(
                "WebSocket private channel subscription",
            ));
        }

        validate_subscription(&params)?;

        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsSubscribeCmd {
            id,
            cmd: "subscribe",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    pub async fn unsubscribe_v2(
        &mut self,
        params: WsUnsubscribeParamsV2,
    ) -> Result<u64, KalshiError> {
        self.unsubscribe(params).await
    }

    pub async fn update_subscription_v2(
        &mut self,
        params: WsUpdateSubscriptionParamsV2,
    ) -> Result<u64, KalshiError> {
        validate_update(&params)?;

        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsUpdateSubscriptionCmd {
            id,
            cmd: "update_subscription",
            params,
        };

        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    pub async fn next_message_v2(&mut self) -> Result<WsMessageV2, KalshiError> {
        self.next_message().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KalshiEnvironment;
    use crate::auth::tests::load_test_auth;
    use crate::ws::protocol::EventContractProtocol;
    use serde_json::{Value, json};
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::accept_hdr_async;
    use tokio_tungstenite::tungstenite::handshake::server::{
        Request as ServerRequest, Response as ServerResponse,
    };
    use url::Url;

    #[tokio::test]
    async fn low_level_unsubscribe_sends_sids_array() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let mut ws = accept_async(stream).await.expect("accept ws");

            let frame = ws.next().await.expect("frame").expect("ok frame");
            let text = match frame {
                Message::Text(text) => text,
                other => panic!("expected text frame, got {other:?}"),
            };

            let payload: Value = serde_json::from_str(&text).expect("valid json");
            assert_eq!(payload["cmd"], json!("unsubscribe"));
            assert_eq!(payload["params"]["sids"], json!([7, 9]));
        });

        let auth = load_test_auth();
        let env = KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
            margin_ws_url: format!("ws://{}", addr),
        };
        let mut client =
            WsLowLevelClient::<EventContractProtocol>::connect_authenticated(env, auth)
                .await
                .expect("connect");
        client
            .unsubscribe(WsUnsubscribeParamsV2 { sids: vec![7, 9] })
            .await
            .expect("unsubscribe");

        server.await.expect("server");
    }

    fn env_for(addr: std::net::SocketAddr) -> KalshiEnvironment {
        KalshiEnvironment {
            rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
            ws_url: format!("ws://{}", addr),
            margin_ws_url: format!("ws://{}", addr),
        }
    }

    fn now_ms() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_millis()
    }

    #[tokio::test]
    async fn connect_timeout_bounds_stalled_handshake_and_next_attempt_succeeds() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            // First TCP connection: accept it, then never speak WebSocket.
            let (_stalled, _) = listener.accept().await.expect("accept stalled");
            let (stream, _) = listener.accept().await.expect("accept retry");
            let _ws = accept_async(stream).await.expect("accept ws");
            tokio::time::sleep(Duration::from_millis(200)).await;
        });

        // Key generation (no test key env set) is slow on CI; keep it outside
        // the timed window so the bound measures only the connect attempt.
        let first_auth = load_test_auth();
        let retry_auth = load_test_auth();
        let started = std::time::Instant::now();
        let first = WsLowLevelClient::<EventContractProtocol>::connect_authenticated_within(
            env_for(addr),
            first_auth,
            Some(Duration::from_millis(300)),
        )
        .await;
        let err = match first {
            Ok(_) => panic!("stalled handshake must not connect"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains("timed out"),
            "unexpected error: {err}"
        );
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "timeout did not bound the attempt"
        );

        WsLowLevelClient::<EventContractProtocol>::connect_authenticated_within(
            env_for(addr),
            retry_auth,
            Some(Duration::from_secs(5)),
        )
        .await
        .expect("retry connects");
        server.await.expect("server");
    }

    #[tokio::test]
    async fn each_connect_attempt_signs_a_fresh_timestamp() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let server = tokio::spawn(async move {
            for _ in 0..2 {
                let (stream, _) = listener.accept().await.expect("accept");
                let tx = tx.clone();
                let ws =
                    accept_hdr_async(stream, move |req: &ServerRequest, resp: ServerResponse| {
                        let ts = req
                            .headers()
                            .get("kalshi-access-timestamp")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_string();
                        let _ = tx.send(ts);
                        Ok(resp)
                    })
                    .await
                    .expect("accept ws");
                drop(ws);
            }
        });

        let auth_first = load_test_auth();
        let auth_second = load_test_auth();
        let first_started = now_ms();
        let first = WsLowLevelClient::<EventContractProtocol>::connect_authenticated(
            env_for(addr),
            auth_first,
        )
        .await
        .expect("first connect");
        drop(first);
        tokio::time::sleep(Duration::from_millis(1100)).await;
        let second_started = now_ms();
        let second = WsLowLevelClient::<EventContractProtocol>::connect_authenticated(
            env_for(addr),
            auth_second,
        )
        .await
        .expect("second connect");
        drop(second);
        server.await.expect("server");

        let ts1: u128 = rx.recv().await.expect("first ts").parse().expect("ms");
        let ts2: u128 = rx.recv().await.expect("second ts").parse().expect("ms");
        assert!(
            ts1 >= first_started && ts1 <= first_started + 2_000,
            "first signature not fresh: {ts1} vs {first_started}"
        );
        assert!(
            ts2 >= second_started && ts2 <= second_started + 2_000,
            "second signature not fresh: {ts2} vs {second_started}"
        );
        assert!(
            ts2 >= ts1 + 1_000,
            "attempts reused a signature: {ts1} {ts2}"
        );
    }
}
