use crate::env::{KalshiEnvironment, MARGIN_WS_PATH, WS_PATH};
use crate::error::KalshiError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

mod private {
    pub trait Sealed {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ControlAction {
        Subscribed {
            cmd_id: Option<u64>,
            sid: u64,
        },
        Unsubscribed {
            cmd_id: Option<u64>,
            sid: Option<u64>,
        },
        Ok {
            cmd_id: u64,
        },
        Error {
            cmd_id: u64,
        },
    }
}

pub(crate) use private::ControlAction;

pub trait Channel:
    Serialize + DeserializeOwned + Debug + Clone + PartialEq + Send + 'static
{
    fn is_private(&self) -> bool;
    fn as_str(&self) -> &'static str;
}

pub trait WsProtocol: private::Sealed + Send + 'static {
    type Message: Clone + Send + 'static;
    type Channel: Channel;
    type SubscribeParams: Clone + Default + Serialize + Send + 'static;
    #[doc(hidden)]
    type UpdateParams: Clone + Send + 'static;

    fn ws_url(env: &KalshiEnvironment) -> &str;
    fn signing_path() -> &'static str;
    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError>;
    #[doc(hidden)]
    fn control_action(msg: &Self::Message) -> Option<private::ControlAction>;
    #[doc(hidden)]
    fn apply_update(active: &mut HashMap<u64, Self::SubscribeParams>, update: &Self::UpdateParams);
    #[doc(hidden)]
    fn records_update(update: &Self::UpdateParams) -> bool;
}

pub struct EventContractProtocol;
impl private::Sealed for EventContractProtocol {}

impl WsProtocol for EventContractProtocol {
    type Message = crate::ws::types::WsMessageV2;
    type Channel = crate::ws::types::WsChannelV2;
    type SubscribeParams = crate::ws::types::WsSubscriptionParamsV2;
    type UpdateParams = crate::ws::types::WsUpdateSubscriptionParamsV2;

    fn ws_url(env: &KalshiEnvironment) -> &str {
        &env.ws_url
    }

    fn signing_path() -> &'static str {
        WS_PATH
    }

    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError> {
        crate::ws::types::WsMessageV2::from_bytes(bytes)
    }

    fn control_action(msg: &Self::Message) -> Option<private::ControlAction> {
        match msg {
            crate::ws::types::WsMessageV2::Subscribed {
                id, sid: Some(sid), ..
            } => Some(private::ControlAction::Subscribed {
                cmd_id: *id,
                sid: *sid,
            }),
            crate::ws::types::WsMessageV2::Unsubscribed { id, sid, .. } => {
                if id.is_none() && sid.is_none() {
                    None
                } else {
                    Some(private::ControlAction::Unsubscribed {
                        cmd_id: *id,
                        sid: *sid,
                    })
                }
            }
            crate::ws::types::WsMessageV2::Ok { id: Some(id), .. } => {
                Some(private::ControlAction::Ok { cmd_id: *id })
            }
            crate::ws::types::WsMessageV2::Error { id: Some(id), .. } => {
                Some(private::ControlAction::Error { cmd_id: *id })
            }
            _ => None,
        }
    }

    fn apply_update(active: &mut HashMap<u64, Self::SubscribeParams>, update: &Self::UpdateParams) {
        use crate::ws::types::WsUpdateAction;

        let sid = match update.target_sid() {
            Some(sid) => sid,
            None => return,
        };

        let Some(params) = active.get_mut(&sid) else {
            return;
        };

        let mut incoming_tickers = update.market_tickers.clone().unwrap_or_default();
        if let Some(single) = update.market_ticker.clone() {
            incoming_tickers.push(single);
        }

        let mut incoming_ids = update.market_ids.clone().unwrap_or_default();
        if let Some(single) = update.market_id.clone() {
            incoming_ids.push(single);
        }

        let apply_vec =
            |target: &mut Option<Vec<String>>, incoming: Vec<String>, action: WsUpdateAction| {
                if incoming.is_empty() {
                    return;
                }

                match action {
                    WsUpdateAction::AddMarkets | WsUpdateAction::SubscribeIndices => {
                        let values = target.get_or_insert_with(Vec::new);
                        for value in incoming {
                            if !values.iter().any(|v| v == &value) {
                                values.push(value);
                            }
                        }
                    }
                    WsUpdateAction::DeleteMarkets | WsUpdateAction::UnsubscribeIndices => {
                        let Some(values) = target.as_mut() else {
                            return;
                        };
                        values.retain(|current| !incoming.iter().any(|value| value == current));
                        if values.is_empty() {
                            *target = None;
                        }
                    }
                    WsUpdateAction::GetSnapshot | WsUpdateAction::Indexlist => {}
                }
            };

        if update.action.is_index_action() {
            let incoming_indices = update.index_ids.clone().unwrap_or_default();
            apply_vec(&mut params.index_ids, incoming_indices, update.action);
        } else {
            apply_vec(&mut params.market_tickers, incoming_tickers, update.action);
            apply_vec(&mut params.market_ids, incoming_ids, update.action);
        }

        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
        if let Some(value) = update.skip_ticker_ack {
            params.skip_ticker_ack = Some(value);
        }
    }

    fn records_update(update: &Self::UpdateParams) -> bool {
        !matches!(
            update.action,
            crate::ws::types::WsUpdateAction::GetSnapshot
                | crate::ws::types::WsUpdateAction::Indexlist
        )
    }
}

#[allow(dead_code)]
pub struct MarginProtocol;
impl private::Sealed for MarginProtocol {}

impl WsProtocol for MarginProtocol {
    type Message = crate::margin::ws::message::MarginDataMessage;
    type Channel = crate::margin::ws::channel::MarginChannel;
    type SubscribeParams = crate::margin::ws::subscription::MarginSubscribeParams;
    type UpdateParams = std::convert::Infallible;

    fn ws_url(env: &KalshiEnvironment) -> &str {
        &env.margin_ws_url
    }

    fn signing_path() -> &'static str {
        MARGIN_WS_PATH
    }

    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError> {
        crate::margin::ws::message::MarginDataMessage::from_bytes(bytes)
    }

    fn control_action(msg: &Self::Message) -> Option<private::ControlAction> {
        use crate::margin::ws::message::MarginDataMessage;

        match msg {
            MarginDataMessage::Subscribed { id, sid: Some(sid) } => {
                Some(private::ControlAction::Subscribed {
                    cmd_id: *id,
                    sid: *sid,
                })
            }
            MarginDataMessage::Unsubscribed { id, sid, .. } => {
                if id.is_none() && sid.is_none() {
                    None
                } else {
                    Some(private::ControlAction::Unsubscribed {
                        cmd_id: *id,
                        sid: *sid,
                    })
                }
            }
            MarginDataMessage::Ok { id: Some(id), .. } => {
                Some(private::ControlAction::Ok { cmd_id: *id })
            }
            MarginDataMessage::Error { id: Some(id), .. } => {
                Some(private::ControlAction::Error { cmd_id: *id })
            }
            _ => None,
        }
    }

    fn apply_update(
        _active: &mut HashMap<u64, Self::SubscribeParams>,
        update: &Self::UpdateParams,
    ) {
        match *update {}
    }

    fn records_update(update: &Self::UpdateParams) -> bool {
        match *update {}
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
        Unsubscribed { id: Option<u64>, sid: Option<u64> },
        #[serde(rename = "ok")]
        Ok { id: Option<u64> },
        #[serde(rename = "error")]
        Error { id: Option<u64> },
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
        Ok(WsControlMessage::Unsubscribed { id, sid }) => {
            if id.is_none() && sid.is_none() {
                Ok(None)
            } else {
                Ok(Some(ControlAction::Unsubscribed { cmd_id: id, sid }))
            }
        }
        Ok(WsControlMessage::Ok { id: Some(cmd_id) }) => Ok(Some(ControlAction::Ok { cmd_id })),
        Ok(WsControlMessage::Ok { id: None }) => Ok(None),
        Ok(WsControlMessage::Error { id: Some(cmd_id) }) => {
            Ok(Some(ControlAction::Error { cmd_id }))
        }
        Ok(WsControlMessage::Error { id: None }) => Ok(None),
        Ok(WsControlMessage::Other) => Ok(None),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_control_message_maps_ok_error_and_unsubscribed_ids() {
        let ok_bytes = br#"{"type":"ok","id":12}"#;
        assert_eq!(
            parse_control_message(ok_bytes).unwrap(),
            Some(ControlAction::Ok { cmd_id: 12 })
        );

        let err_bytes = br#"{"type":"error","id":34,"msg":{"code":"invalid","message":"bad"}}"#;
        assert_eq!(
            parse_control_message(err_bytes).unwrap(),
            Some(ControlAction::Error { cmd_id: 34 })
        );

        let unsub_both = br#"{"type":"unsubscribed","id":56,"sid":78}"#;
        assert_eq!(
            parse_control_message(unsub_both).unwrap(),
            Some(ControlAction::Unsubscribed {
                cmd_id: Some(56),
                sid: Some(78)
            })
        );

        let unsub_sid_only = br#"{"type":"unsubscribed","sid":90}"#;
        assert_eq!(
            parse_control_message(unsub_sid_only).unwrap(),
            Some(ControlAction::Unsubscribed {
                cmd_id: None,
                sid: Some(90)
            })
        );

        let unsub_id_only = br#"{"type":"unsubscribed","id":91}"#;
        assert_eq!(
            parse_control_message(unsub_id_only).unwrap(),
            Some(ControlAction::Unsubscribed {
                cmd_id: Some(91),
                sid: None
            })
        );

        let unsub_empty = br#"{"type":"unsubscribed"}"#;
        assert_eq!(parse_control_message(unsub_empty).unwrap(), None);

        let ok_no_id = br#"{"type":"ok"}"#;
        assert_eq!(parse_control_message(ok_no_id).unwrap(), None);

        let err_no_id = br#"{"type":"error"}"#;
        assert_eq!(parse_control_message(err_no_id).unwrap(), None);
    }
}
