use super::channel::WsChannelV2;
use crate::error::KalshiError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Subscription parameters for WebSocket channels.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WsSubscriptionParamsV2 {
    pub channels: Vec<WsChannelV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_factor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<u32>,
    /// CF Benchmarks index IDs for `cfbenchmarks_value` subscriptions.
    /// Use `["all"]` to receive every available index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_ids: Option<Vec<String>>,
}

impl WsSubscriptionParamsV2 {
    /// Collect all market tickers from both singular and plural fields.
    pub fn all_market_tickers(&self) -> Vec<&str> {
        let mut out = Vec::new();
        if let Some(single) = &self.market_ticker {
            out.push(single.as_str());
        }
        if let Some(multi) = &self.market_tickers {
            out.extend(multi.iter().map(String::as_str));
        }
        out
    }

    /// Collect all market IDs from both singular and plural fields.
    pub fn all_market_ids(&self) -> Vec<&str> {
        let mut out = Vec::new();
        if let Some(single) = &self.market_id {
            out.push(single.as_str());
        }
        if let Some(multi) = &self.market_ids {
            out.extend(multi.iter().map(String::as_str));
        }
        out
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfo {
    pub sid: u64,
    #[serde(default)]
    pub channels: Vec<WsChannelV2>,
    #[serde(default)]
    pub channel: Option<WsChannelV2>,
    #[serde(default)]
    pub market_tickers: Option<Vec<String>>,
    #[serde(default)]
    pub market_ids: Option<Vec<String>>,
    #[serde(default)]
    pub event_tickers: Option<Vec<String>>,
    #[serde(default)]
    pub send_initial_snapshot: Option<bool>,
    #[serde(default)]
    pub skip_ticker_ack: Option<bool>,
    #[serde(default)]
    pub shard_factor: Option<u32>,
    #[serde(default)]
    pub shard_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfoRef<'a> {
    pub sid: u64,
    #[serde(default)]
    pub channels: Vec<WsChannelV2>,
    #[serde(default)]
    pub channel: Option<WsChannelV2>,
    #[serde(default, borrow)]
    pub market_tickers: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow)]
    pub market_ids: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow)]
    pub event_tickers: Option<Vec<Cow<'a, str>>>,
    #[serde(default)]
    pub send_initial_snapshot: Option<bool>,
    #[serde(default)]
    pub skip_ticker_ack: Option<bool>,
    #[serde(default)]
    pub shard_factor: Option<u32>,
    #[serde(default, borrow)]
    pub shard_key: Option<Cow<'a, str>>,
}

impl<'a> WsSubscriptionInfoRef<'a> {
    pub fn into_owned(self) -> WsSubscriptionInfo {
        WsSubscriptionInfo {
            sid: self.sid,
            channels: self.channels,
            channel: self.channel,
            market_tickers: self
                .market_tickers
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            market_ids: self
                .market_ids
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            event_tickers: self
                .event_tickers
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            send_initial_snapshot: self.send_initial_snapshot,
            skip_ticker_ack: self.skip_ticker_ack,
            shard_factor: self.shard_factor,
            shard_key: self.shard_key.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsUnsubscribeParamsV2 {
    pub sids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsUpdateSubscriptionParamsV2 {
    pub action: WsUpdateAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    /// CF Benchmarks index IDs to add or remove. Required for the
    /// `subscribe_indices` / `unsubscribe_indices` actions; use `["all"]` to
    /// track every available index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_ids: Option<Vec<String>>,
}

impl WsUpdateSubscriptionParamsV2 {
    pub fn target_sid(&self) -> Option<u64> {
        self.sid.or_else(|| {
            self.sids
                .as_ref()
                .and_then(|values| values.first().copied())
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WsUpdateAction {
    AddMarkets,
    DeleteMarkets,
    GetSnapshot,
    /// Add the supplied `index_ids` to a `cfbenchmarks_value` subscription.
    SubscribeIndices,
    /// Remove the supplied `index_ids` from a `cfbenchmarks_value` subscription.
    UnsubscribeIndices,
    /// Request the available CF Benchmarks index IDs without modifying the
    /// subscription (server replies with a `cfbenchmarks_value_indexlist`).
    Indexlist,
}

impl WsUpdateAction {
    /// Whether this action operates on CF Benchmarks index IDs rather than
    /// market targets.
    pub fn is_index_action(self) -> bool {
        matches!(
            self,
            WsUpdateAction::SubscribeIndices
                | WsUpdateAction::UnsubscribeIndices
                | WsUpdateAction::Indexlist
        )
    }
}

pub(crate) fn validate_update(params: &WsUpdateSubscriptionParamsV2) -> Result<(), KalshiError> {
    let has_sid = params.sid.is_some();
    let has_sids = params.sids.is_some();
    if has_sid == has_sids {
        return Err(KalshiError::InvalidParams(
            "update_subscription: provide exactly one of sid or sids".to_string(),
        ));
    }
    if let Some(sids) = &params.sids
        && sids.len() != 1
    {
        return Err(KalshiError::InvalidParams(
            "update_subscription: sids must contain exactly one sid".to_string(),
        ));
    }

    let has_market_ticker = params.market_ticker.is_some();
    let has_market_tickers = params
        .market_tickers
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_market_id = params.market_id.is_some();
    let has_market_ids = params
        .market_ids
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_any_market_tickers = has_market_ticker || has_market_tickers;
    let has_any_market_ids = has_market_id || has_market_ids;
    let has_index_ids = params
        .index_ids
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);

    // CF Benchmarks index actions are mutually exclusive with market targets and
    // have their own `index_ids` requirements.
    if params.action.is_index_action() {
        if has_any_market_tickers || has_any_market_ids {
            return Err(KalshiError::InvalidParams(
                "update_subscription: index actions do not support market_ticker(s) or market_id(s)"
                    .to_string(),
            ));
        }
        if matches!(
            params.action,
            WsUpdateAction::SubscribeIndices | WsUpdateAction::UnsubscribeIndices
        ) && !has_index_ids
        {
            return Err(KalshiError::InvalidParams(
                "update_subscription: subscribe_indices/unsubscribe_indices require index_ids"
                    .to_string(),
            ));
        }
    } else if params.index_ids.is_some() {
        return Err(KalshiError::InvalidParams(
            "update_subscription: index_ids is only valid for subscribe_indices, \
             unsubscribe_indices, or indexlist actions"
                .to_string(),
        ));
    }

    if has_market_ticker && has_market_tickers {
        return Err(KalshiError::InvalidParams(
            "update_subscription: provide at most one of market_ticker or market_tickers"
                .to_string(),
        ));
    }
    if has_market_id && has_market_ids {
        return Err(KalshiError::InvalidParams(
            "update_subscription: provide at most one of market_id or market_ids".to_string(),
        ));
    }
    if has_any_market_tickers && has_any_market_ids {
        return Err(KalshiError::InvalidParams(
            "update_subscription: market_ticker(s) and market_id(s) are mutually exclusive"
                .to_string(),
        ));
    }

    if params.action == WsUpdateAction::GetSnapshot {
        if !has_any_market_tickers {
            return Err(KalshiError::InvalidParams(
                "update_subscription: get_snapshot requires market_ticker or market_tickers"
                    .to_string(),
            ));
        }
        if has_any_market_ids {
            return Err(KalshiError::InvalidParams(
                "update_subscription: get_snapshot does not support market_id or market_ids"
                    .to_string(),
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_subscription(params: &WsSubscriptionParamsV2) -> Result<(), KalshiError> {
    if params.channels.is_empty() {
        return Err(KalshiError::InvalidParams(
            "subscribe: at least one channel is required".to_string(),
        ));
    }

    let has_market_ticker = params.market_ticker.is_some();
    let has_market_tickers = params
        .market_tickers
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_market_id = params.market_id.is_some();
    let has_market_ids = params
        .market_ids
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_any_market_tickers = has_market_ticker || has_market_tickers;
    let has_any_market_ids = has_market_id || has_market_ids;

    if has_market_ticker && has_market_tickers {
        return Err(KalshiError::InvalidParams(
            "subscribe: provide at most one of market_ticker or market_tickers".to_string(),
        ));
    }
    if has_market_id && has_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: provide at most one of market_id or market_ids".to_string(),
        ));
    }
    if has_any_market_tickers && has_any_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_ticker(s) and market_id(s) are mutually exclusive".to_string(),
        ));
    }

    let has_orderbook_delta = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::OrderbookDelta));
    let has_market_positions = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::MarketPositions));
    let has_communications = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::Communications));

    if has_orderbook_delta && !has_any_market_tickers {
        return Err(KalshiError::InvalidParams(
            "subscribe: orderbook_delta requires market_ticker or market_tickers".to_string(),
        ));
    }

    if has_orderbook_delta && has_any_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: orderbook_delta does not support market_id or market_ids".to_string(),
        ));
    }

    if params.send_initial_snapshot.is_some() && !has_orderbook_delta {
        return Err(KalshiError::InvalidParams(
            "subscribe: send_initial_snapshot only allowed for orderbook_delta".to_string(),
        ));
    }

    if has_any_market_ids && has_market_positions {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_positions only supports market_tickers".to_string(),
        ));
    }

    if params.shard_key.is_some() && params.shard_factor.is_none() {
        return Err(KalshiError::InvalidParams(
            "subscribe: shard_factor is required when shard_key is set".to_string(),
        ));
    }

    if (params.shard_factor.is_some() || params.shard_key.is_some()) && !has_communications {
        return Err(KalshiError::InvalidParams(
            "subscribe: shard_factor/shard_key only allowed for communications".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_subscription_requires_market_tickers_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_subscription_send_initial_snapshot_only_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_orderbook_delta_rejects_market_ids() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ids: Some(vec!["mid-1".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_rejects_market_positions_with_market_ids() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::MarketPositions],
            market_ids: Some(vec!["mid-1".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_shard_fields_require_communications() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            shard_factor: Some(2),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Communications],
            shard_factor: Some(2),
            shard_key: Some(1),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_subscription_send_initial_snapshot_with_orderbook_delta_ok() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_update_requires_exactly_one_sid_target() {
        let both = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(1),
            sids: Some(vec![2]),
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&both).is_err());

        let multi = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: None,
            sids: Some(vec![1, 2]),
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&multi).is_err());

        let valid = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::DeleteMarkets,
            sid: Some(1),
            sids: None,
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&valid).is_ok());
    }

    #[test]
    fn validate_update_get_snapshot_requires_market_target() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&params).is_err());
    }

    #[test]
    fn validate_update_get_snapshot_rejects_market_ids() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(1),
            sids: None,
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: Some("uuid".to_string()),
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&params).is_err());
    }

    #[test]
    fn validate_update_get_snapshot_accepts_market_tickers() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["TICKER".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&params).is_ok());
    }

    #[test]
    fn validate_update_subscribe_indices_requires_index_ids() {
        let missing = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::SubscribeIndices,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&missing).is_err());

        let ok = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::SubscribeIndices,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: Some(vec!["BRTI".to_string()]),
        };
        assert!(validate_update(&ok).is_ok());
    }

    #[test]
    fn validate_update_indexlist_allows_empty_index_ids() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::Indexlist,
            sid: Some(1),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        assert!(validate_update(&params).is_ok());
    }

    #[test]
    fn validate_update_index_actions_reject_market_targets() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::SubscribeIndices,
            sid: Some(1),
            sids: None,
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: Some(vec!["BRTI".to_string()]),
        };
        assert!(validate_update(&params).is_err());
    }

    #[test]
    fn validate_update_index_ids_rejected_for_market_actions() {
        let params = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(1),
            sids: None,
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: Some(vec!["BRTI".to_string()]),
        };
        assert!(validate_update(&params).is_err());
    }

    #[test]
    fn validate_subscription_enforces_market_target_exclusivity() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_tickers: Some(vec!["B".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_id: Some("uuid".to_string()),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }
}
