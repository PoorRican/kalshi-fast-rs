use crate::env::{KalshiEnvironment, MARGIN_WS_PATH, WS_PATH};
use crate::error::KalshiError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Channel:
    Serialize + DeserializeOwned + Debug + Clone + PartialEq + Send + 'static
{
    fn is_private(&self) -> bool;
    fn as_str(&self) -> &'static str;
}

pub trait WsProtocol: Send + 'static {
    type Message: Clone + Send + 'static;
    type Channel: Channel;
    type SubscribeParams: Clone + Default + Serialize + Send + 'static;

    fn ws_url(env: &KalshiEnvironment) -> &str;
    fn signing_path() -> &'static str;
    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError>;
}

pub struct EventContractProtocol;

impl WsProtocol for EventContractProtocol {
    type Message = crate::ws::types::WsMessageV2;
    type Channel = crate::ws::types::WsChannelV2;
    type SubscribeParams = crate::ws::types::WsSubscriptionParamsV2;

    fn ws_url(env: &KalshiEnvironment) -> &str {
        &env.ws_url
    }

    fn signing_path() -> &'static str {
        WS_PATH
    }

    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError> {
        crate::ws::types::WsMessageV2::from_bytes(bytes)
    }
}

#[allow(dead_code)]
pub struct MarginProtocol;

impl WsProtocol for MarginProtocol {
    type Message = crate::margin::ws::message::MarginDataMessage;
    type Channel = crate::margin::ws::channel::MarginChannel;
    type SubscribeParams = crate::margin::ws::subscription::MarginSubscribeParams;

    fn ws_url(env: &KalshiEnvironment) -> &str {
        &env.margin_ws_url
    }

    fn signing_path() -> &'static str {
        MARGIN_WS_PATH
    }

    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError> {
        crate::margin::ws::message::MarginDataMessage::from_bytes(bytes)
    }
}

pub(crate) fn parse_control_message(bytes: &[u8]) -> Result<Option<ControlAction>, KalshiError> {
    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    enum WsControlMessage {
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
    struct WsControlSubscribedMsg {
        sid: Option<u64>,
    }

    match serde_json::from_slice::<WsControlMessage>(bytes) {
        Ok(WsControlMessage::Subscribed { id, sid, msg }) => {
            let sid = sid.or_else(|| msg.and_then(|m| m.sid));
            Ok(sid.map(|sid| ControlAction::Subscribed { cmd_id: id, sid }))
        }
        Ok(WsControlMessage::Unsubscribed { sid }) => {
            Ok(sid.map(|sid| ControlAction::Unsubscribed { sid }))
        }
        Ok(WsControlMessage::Other) => Ok(None),
        Err(_) => Ok(None),
    }
}

pub(crate) enum ControlAction {
    Subscribed { cmd_id: Option<u64>, sid: u64 },
    Unsubscribed { sid: u64 },
}
