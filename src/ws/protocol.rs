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
            channel: Option<String>,
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
    fn validate_subscribe(params: &Self::SubscribeParams) -> Result<(), KalshiError>;
    #[doc(hidden)]
    fn control_action(msg: &Self::Message) -> Option<private::ControlAction>;
    #[doc(hidden)]
    fn apply_update(active: &mut HashMap<u64, Self::SubscribeParams>, update: &Self::UpdateParams);
    #[doc(hidden)]
    fn records_update(update: &Self::UpdateParams) -> bool;
    #[doc(hidden)]
    fn merge_subscribe(active: &mut Self::SubscribeParams, incoming: Self::SubscribeParams) {
        *active = incoming;
    }
    #[doc(hidden)]
    fn take_subscribed(
        pending: &mut HashMap<u64, Self::SubscribeParams>,
        id: u64,
        _channel: Option<&str>,
    ) -> Option<Self::SubscribeParams> {
        pending.remove(&id)
    }
}

pub struct EventContractProtocol;
impl private::Sealed for EventContractProtocol {}
fn project_event_options(params: &mut crate::ws::types::WsSubscriptionParamsV2) {
    use crate::ws::types::WsChannelV2 as C;
    let has = |channel| params.channels.contains(&channel);
    if !has(C::OrderbookDelta) {
        params.use_yes_price = None;
        params.update_interval_ms = None;
    }
    if !has(C::OrderbookDelta) && !has(C::Ticker) {
        params.send_initial_snapshot = None;
    }
    if !has(C::Communications) {
        params.shard_factor = None;
        params.shard_key = None;
        params.user_filter = None;
    }
    if !has(C::CfbenchmarksValue) && !has(C::CfbenchmarksValue5hz) {
        params.index_ids = None;
    }
    if !has(C::PythValue) {
        params.underlying_tickers = None;
    }
}

fn project_margin_options(params: &mut crate::margin::ws::subscription::MarginSubscribeParams) {
    use crate::margin::ws::channel::MarginChannel as C;
    if !params.channels.contains(&C::OrderbookDelta) {
        params.update_interval_ms = None;
    }
    if !params.channels.contains(&C::OrderbookDelta) && !params.channels.contains(&C::Ticker) {
        params.send_initial_snapshot = None;
    }
}

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
    fn validate_subscribe(params: &Self::SubscribeParams) -> Result<(), KalshiError> {
        crate::ws::types::validate_subscription(params)
    }

    fn control_action(msg: &Self::Message) -> Option<private::ControlAction> {
        match msg {
            crate::ws::types::WsMessageV2::Subscribed {
                id,
                sid: Some(sid),
                channel,
                ..
            } => Some(private::ControlAction::Subscribed {
                cmd_id: *id,
                sid: *sid,
                channel: channel.as_ref().map(|value| value.as_str().to_owned()),
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

        fn apply_values(target: &mut Option<Vec<String>>, incoming: &[String], add: bool) {
            if incoming.is_empty() {
                return;
            }
            if add {
                let values = target.get_or_insert_with(Vec::new);
                for value in incoming {
                    if !values.contains(value) {
                        values.push(value.clone());
                    }
                }
            } else if let Some(values) = target.as_mut() {
                values.retain(|value| !incoming.contains(value));
                if values.is_empty() {
                    *target = None;
                }
            }
        }

        let add = matches!(
            update.action,
            WsUpdateAction::AddMarkets
                | WsUpdateAction::SubscribeIndices
                | WsUpdateAction::SubscribeUnderlyings
        );
        let remove = matches!(
            update.action,
            WsUpdateAction::DeleteMarkets
                | WsUpdateAction::UnsubscribeIndices
                | WsUpdateAction::UnsubscribeUnderlyings
        );
        if !add && !remove {
            return;
        }
        if update.action.is_index_action() {
            apply_values(
                &mut params.index_ids,
                update.index_ids.as_deref().unwrap_or_default(),
                add,
            );
        } else if update.action.is_underlying_action() {
            apply_values(
                &mut params.underlying_tickers,
                update.underlying_tickers.as_deref().unwrap_or_default(),
                add,
            );
        } else {
            let mut tickers = update.market_tickers.clone().unwrap_or_default();
            if let Some(single) = &update.market_ticker {
                tickers.push(single.clone());
            }
            let mut ids = update.market_ids.clone().unwrap_or_default();
            if let Some(single) = &update.market_id {
                ids.push(single.clone());
            }
            // A singular seed is part of the same filter as the plural field.
            if remove
                && params
                    .market_ticker
                    .as_ref()
                    .is_some_and(|v| tickers.contains(v))
            {
                params.market_ticker = None;
            }
            if remove && params.market_id.as_ref().is_some_and(|v| ids.contains(v)) {
                params.market_id = None;
            }
            if add {
                // A replay command cannot contain both singular and plural selectors.
                if !tickers.is_empty()
                    && let Some(single) = params.market_ticker.take()
                {
                    apply_values(&mut params.market_tickers, &[single], true);
                }
                if !ids.is_empty()
                    && let Some(single) = params.market_id.take()
                {
                    apply_values(&mut params.market_ids, &[single], true);
                }
            }
            apply_values(&mut params.market_tickers, &tickers, add);
            apply_values(&mut params.market_ids, &ids, add);
        }

        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
    }

    fn records_update(update: &Self::UpdateParams) -> bool {
        !matches!(
            update.action,
            crate::ws::types::WsUpdateAction::GetSnapshot
                | crate::ws::types::WsUpdateAction::Indexlist
                | crate::ws::types::WsUpdateAction::UnderlyingList
                | crate::ws::types::WsUpdateAction::SubscribeMatches
                | crate::ws::types::WsUpdateAction::UnsubscribeMatches
                | crate::ws::types::WsUpdateAction::Matchlist
        )
    }

    fn merge_subscribe(active: &mut Self::SubscribeParams, incoming: Self::SubscribeParams) {
        use crate::ws::types::WsChannelV2;
        // The exchange merges repeated orderbook subscribes into the same SID.
        // Other channels retain the previous replacement semantics.
        if !active.channels.contains(&WsChannelV2::OrderbookDelta)
            || !incoming.channels.contains(&WsChannelV2::OrderbookDelta)
        {
            *active = incoming;
            return;
        }
        let mut values = active.market_tickers.take().unwrap_or_default();
        if let Some(single) = active.market_ticker.take()
            && !values.contains(&single)
        {
            values.push(single);
        }
        for ticker in incoming
            .market_tickers
            .into_iter()
            .flatten()
            .chain(incoming.market_ticker)
        {
            if !values.contains(&ticker) {
                values.push(ticker);
            }
        }
        active.market_tickers = if values.is_empty() {
            None
        } else {
            Some(values)
        };
    }

    fn take_subscribed(
        pending: &mut HashMap<u64, Self::SubscribeParams>,
        id: u64,
        channel: Option<&str>,
    ) -> Option<Self::SubscribeParams> {
        let Some(channel) = channel else {
            return pending.remove(&id);
        };
        let params = pending.get_mut(&id)?;
        let index = params
            .channels
            .iter()
            .position(|value| value.as_str() == channel)?;
        let selected = params.channels.remove(index);
        let mut acknowledged = params.clone();
        acknowledged.channels = vec![selected];
        project_event_options(&mut acknowledged);
        project_event_options(params);
        if params.channels.is_empty() {
            pending.remove(&id);
        }
        Some(acknowledged)
    }
}

#[allow(dead_code)]
pub struct MarginProtocol;
impl private::Sealed for MarginProtocol {}

impl WsProtocol for MarginProtocol {
    type Message = crate::margin::ws::message::MarginDataMessage;
    type Channel = crate::margin::ws::channel::MarginChannel;
    type SubscribeParams = crate::margin::ws::subscription::MarginSubscribeParams;
    type UpdateParams = crate::margin::ws::subscription::MarginUpdateSubscriptionParams;

    fn ws_url(env: &KalshiEnvironment) -> &str {
        &env.margin_ws_url
    }

    fn signing_path() -> &'static str {
        MARGIN_WS_PATH
    }

    fn parse_message(bytes: &[u8]) -> Result<Self::Message, KalshiError> {
        crate::margin::ws::message::MarginDataMessage::from_bytes(bytes)
    }
    fn validate_subscribe(params: &Self::SubscribeParams) -> Result<(), KalshiError> {
        crate::margin::ws::subscription::validate_subscription(params)
    }

    fn control_action(msg: &Self::Message) -> Option<private::ControlAction> {
        use crate::margin::ws::message::MarginDataMessage;

        match msg {
            MarginDataMessage::Subscribed {
                id,
                sid: Some(sid),
                channel,
                ..
            } => Some(private::ControlAction::Subscribed {
                cmd_id: *id,
                sid: *sid,
                channel: Some(channel.clone()),
            }),
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

    fn apply_update(active: &mut HashMap<u64, Self::SubscribeParams>, update: &Self::UpdateParams) {
        use crate::margin::ws::subscription::MarginUpdateAction;
        let Some(params) = update.target_sid().and_then(|sid| active.get_mut(&sid)) else {
            return;
        };
        if matches!(update.action, MarginUpdateAction::GetSnapshot) {
            return;
        }
        let mut incoming = update.market_tickers.clone().unwrap_or_default();
        if let Some(ticker) = &update.market_ticker {
            incoming.push(ticker.clone());
        }
        if matches!(update.action, MarginUpdateAction::AddMarkets) {
            if !incoming.is_empty() {
                let mut values = params.market_tickers.take().unwrap_or_default();
                if let Some(single) = params.market_ticker.take()
                    && !values.contains(&single)
                {
                    values.push(single);
                }
                for ticker in incoming {
                    if !values.contains(&ticker) {
                        values.push(ticker);
                    }
                }
                params.market_tickers = Some(values);
            }
        } else {
            if params
                .market_ticker
                .as_ref()
                .is_some_and(|value| incoming.contains(value))
            {
                params.market_ticker = None;
            }
            if let Some(values) = params.market_tickers.as_mut() {
                values.retain(|value| !incoming.contains(value));
                if values.is_empty() {
                    params.market_tickers = None;
                }
            }
        }
        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
    }

    fn records_update(update: &Self::UpdateParams) -> bool {
        !matches!(
            update.action,
            crate::margin::ws::subscription::MarginUpdateAction::GetSnapshot
        )
    }

    fn merge_subscribe(active: &mut Self::SubscribeParams, incoming: Self::SubscribeParams) {
        use crate::margin::ws::channel::MarginChannel;
        if !active.channels.contains(&MarginChannel::OrderbookDelta)
            || !incoming.channels.contains(&MarginChannel::OrderbookDelta)
        {
            *active = incoming;
            return;
        }
        let mut values = active.market_tickers.take().unwrap_or_default();
        if let Some(single) = active.market_ticker.take()
            && !values.contains(&single)
        {
            values.push(single);
        }
        for ticker in incoming
            .market_tickers
            .into_iter()
            .flatten()
            .chain(incoming.market_ticker)
        {
            if !values.contains(&ticker) {
                values.push(ticker);
            }
        }
        active.market_tickers = if values.is_empty() {
            None
        } else {
            Some(values)
        };
    }

    fn take_subscribed(
        pending: &mut HashMap<u64, Self::SubscribeParams>,
        id: u64,
        channel: Option<&str>,
    ) -> Option<Self::SubscribeParams> {
        let Some(channel) = channel else {
            return pending.remove(&id);
        };
        let params = pending.get_mut(&id)?;
        let index = params
            .channels
            .iter()
            .position(|value| value.as_str() == channel)?;
        let selected = params.channels.remove(index);
        let mut acknowledged = params.clone();
        acknowledged.channels = vec![selected];
        project_margin_options(&mut acknowledged);
        project_margin_options(params);
        if params.channels.is_empty() {
            pending.remove(&id);
        }
        Some(acknowledged)
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
        channel: Option<String>,
    }

    match serde_json::from_slice::<WsControlMessage>(bytes) {
        Ok(WsControlMessage::Subscribed { id, sid, msg }) => {
            let sid = sid.or_else(|| msg.as_ref().and_then(|m| m.sid));
            let channel = msg.and_then(|m| m.channel);
            Ok(sid.map(|sid| ControlAction::Subscribed {
                cmd_id: id,
                sid,
                channel,
            }))
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
