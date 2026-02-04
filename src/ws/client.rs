use crate::auth::KalshiAuth;
use crate::env::{KalshiEnvironment, WS_PATH};
use crate::error::KalshiError;
use crate::ws::types::{WsChannel, WsEnvelope, WsSubscribeCmd, WsSubscribeParams};

use futures::{SinkExt, StreamExt};

use tokio_tungstenite::tungstenite::http::{HeaderValue, Request};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

type WsStream = tokio_tungstenite::WebSocketStream<
    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
>;

pub struct KalshiWsClient {
    write: futures::stream::SplitSink<WsStream, Message>,
    read: futures::stream::SplitStream<WsStream>,
    next_id: u64,
    authenticated: bool,
}

impl KalshiWsClient {
    /// Connect without auth (public channels only).
    pub async fn connect(env: KalshiEnvironment) -> Result<Self, KalshiError> {
        let (ws_stream, _resp) = connect_async(&env.ws_url)
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        let (write, read) = ws_stream.split();
        Ok(Self {
            write,
            read,
            next_id: 1,
            authenticated: false,
        })
    }

    /// Connect with auth headers so you can subscribe to private channels.
    pub async fn connect_authenticated(env: KalshiEnvironment, auth: KalshiAuth) -> Result<Self, KalshiError> {
        let mut req: Request<()> = env
            .ws_url
            .into_client_request()
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        // WS signing: timestamp + "GET" + "/trade-api/ws/v2"
        let headers = auth.build_headers("GET", WS_PATH)?;

        req.headers_mut().insert(
            "KALSHI-ACCESS-KEY",
            HeaderValue::from_str(&headers.key).map_err(|e| KalshiError::Header(e.to_string()))?,
        );
        req.headers_mut().insert(
            "KALSHI-ACCESS-SIGNATURE",
            HeaderValue::from_str(&headers.signature).map_err(|e| KalshiError::Header(e.to_string()))?,
        );
        req.headers_mut().insert(
            "KALSHI-ACCESS-TIMESTAMP",
            HeaderValue::from_str(&headers.timestamp_ms).map_err(|e| KalshiError::Header(e.to_string()))?,
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

    /// Subscribe to channels; add `market_tickers` when required (e.g. orderbook_delta).
    pub async fn subscribe(
        &mut self,
        channels: Vec<WsChannel>,
        market_tickers: Option<Vec<String>>,
    ) -> Result<u64, KalshiError> {
        let needs_auth = channels.iter().any(|c| c.is_private());
        if needs_auth && !self.authenticated {
            // Server would emit code 9 "Authentication required"
            return Err(KalshiError::AuthRequired("WebSocket private channel subscription"));
        }

        let id = self.next_id;
        self.next_id += 1;

        let cmd = WsSubscribeCmd {
            id,
            cmd: "subscribe",
            params: WsSubscribeParams {
                channels,
                market_tickers,
            },
        };

        let text = serde_json::to_string(&cmd)?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

        Ok(id)
    }

    /// Read the next JSON message from the stream.
    pub async fn next_envelope(&mut self) -> Result<WsEnvelope, KalshiError> {
        while let Some(msg) = self.read.next().await {
            let msg = msg.map_err(|e| KalshiError::Ws(e.to_string()))?;
            match msg {
                Message::Text(s) => return Ok(serde_json::from_str::<WsEnvelope>(&s)?),
                Message::Binary(b) => {
                    // If server ever sends binary JSON, attempt decode.
                    let s = String::from_utf8(b).map_err(|e| KalshiError::Ws(e.to_string()))?;
                    return Ok(serde_json::from_str::<WsEnvelope>(&s)?);
                }
                Message::Ping(payload) => {
                    self.write
                        .send(Message::Pong(payload))
                        .await
                        .map_err(|e| KalshiError::Ws(e.to_string()))?;
                    self.write
                        .flush()
                        .await
                        .map_err(|e| KalshiError::Ws(e.to_string()))?;
                }
                Message::Pong(_) => {}
                Message::Close(_) => return Err(KalshiError::Ws("websocket closed".to_string())),
                _ => {}
            }
        }
        Err(KalshiError::Ws("websocket stream ended".to_string()))
    }
}
