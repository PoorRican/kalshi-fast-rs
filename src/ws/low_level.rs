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
use tokio_tungstenite::connect_async;
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

impl<P: WsProtocol> WsLowLevelClient<P> {
    pub async fn connect_authenticated(
        env: KalshiEnvironment,
        auth: KalshiAuth,
    ) -> Result<Self, KalshiError> {
        let url = P::ws_url(&env);
        let mut req: Request<()> = url
            .into_client_request()
            .map_err(|e| KalshiError::Ws(e.to_string()))?;

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

        let (ws_stream, _resp) = connect_async(req)
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
}
