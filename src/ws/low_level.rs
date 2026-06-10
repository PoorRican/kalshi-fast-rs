use crate::auth::KalshiAuth;
use crate::env::{KalshiEnvironment, WS_PATH};
use crate::error::KalshiError;
#[cfg(doc)]
use crate::ws::KalshiWsClient;
use crate::ws::types::{
    WsEnvelope, WsListSubscriptionsCmd, WsMessageV2, WsSubscribeCmd, WsSubscriptionParamsV2,
    WsUnsubscribeCmd, WsUnsubscribeParamsV2, WsUpdateSubscriptionCmd, WsUpdateSubscriptionParamsV2,
    validate_subscription, validate_update,
};

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::{HeaderValue, Request, header::HeaderName};

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// Low-level WebSocket client with split read/write streams.
///
/// Provides direct access to the Kalshi WebSocket protocol: subscribe,
/// unsubscribe, update, and read raw envelopes or parsed messages.
/// No automatic reconnection or subscription tracking — use
/// [`KalshiWsClient`] if you need those.
///
/// Read and write halves are split at construction time so sending a
/// command never blocks receiving data.
pub struct KalshiWsLowLevelClient {
    write: futures::stream::SplitSink<WsStream, Message>,
    read: futures::stream::SplitStream<WsStream>,
    next_id: u64,
    authenticated: bool,
}

impl KalshiWsLowLevelClient {
    // -----------------------------------------------
    // Connection
    // -----------------------------------------------

    /// Connect without auth.
    ///
    /// Kalshi now requires authentication at WebSocket handshake time for all
    /// connections, including subscriptions to public channels.
    pub async fn connect(_env: KalshiEnvironment) -> Result<Self, KalshiError> {
        Err(KalshiError::AuthRequired("WebSocket connection"))
    }

    /// Connect with auth headers so you can subscribe to private channels.
    ///
    /// **Requires auth.**
    pub async fn connect_authenticated(
        env: KalshiEnvironment,
        auth: KalshiAuth,
    ) -> Result<Self, KalshiError> {
        let mut req: Request<()> = env
            .ws_url
            .into_client_request()
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        // WS signing: timestamp + "GET" + "/trade-api/ws/v2"
        let headers = auth.build_headers("GET", WS_PATH)?;

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

        let (ws_stream, _resp) = connect_async(req)
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        let (write, read) = ws_stream.split();
        Ok(Self {
            write,
            read,
            next_id: 1,
            authenticated: true,
        })
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
                Message::Close(_) => {
                    return Err(KalshiError::Ws("websocket closed".to_string()));
                }
                _ => {}
            }
        }
    }

    // -----------------------------------------------
    // Commands
    // -----------------------------------------------

    /// Subscribe to one or more channels. Returns the command `id`.
    ///
    /// Private channels (e.g. [`WsChannelV2::Fill`](crate::WsChannelV2::Fill))
    /// require an authenticated connection — see [`connect_authenticated`](Self::connect_authenticated).
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

    ///
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

    /// Update an existing subscription (e.g. change market tickers). Returns the command `id`.
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

    /// Request a list of active subscriptions from the server. Returns the command `id`.
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

    // -----------------------------------------------
    // Reading
    // -----------------------------------------------

    /// Read the next JSON envelope from the stream.
    ///
    /// Transparently handles Ping/Pong frames. Returns an error on
    /// Close frames or stream termination.
    pub async fn next_envelope(&mut self) -> Result<WsEnvelope, KalshiError> {
        let bytes = self.next_json_bytes().await?;
        serde_json::from_slice::<WsEnvelope>(&bytes)
            .map_err(|source| KalshiError::parse_json("websocket envelope", &bytes, source))
    }

    /// Read the next message and parse it into a typed [`WsMessageV2`].
    ///
    /// Equivalent to calling [`next_envelope`](Self::next_envelope) followed
    /// by [`WsEnvelope::into_message`].
    pub async fn next_message_v2(&mut self) -> Result<WsMessageV2, KalshiError> {
        let bytes = self.next_json_bytes().await?;
        WsMessageV2::from_bytes(&bytes)
    }

    /// Gracefully close the underlying WebSocket.
    pub async fn close(&mut self) -> Result<(), KalshiError> {
        self.send_raw(Message::Close(None)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KalshiEnvironment;
    use crate::auth::tests::load_test_auth;
    use serde_json::{Value, json};
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
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
        };
        let mut client = KalshiWsLowLevelClient::connect_authenticated(env, auth)
            .await
            .expect("connect");
        client
            .unsubscribe_v2(WsUnsubscribeParamsV2 { sids: vec![7, 9] })
            .await
            .expect("unsubscribe");

        server.await.expect("server");
    }
}
