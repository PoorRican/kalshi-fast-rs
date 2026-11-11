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
    /// Orderbook no-side price convention; omission follows the server default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_yes_price: Option<bool>,
    /// Orderbook conflation interval: 0, 100, or 1000 milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval_ms: Option<i64>,
    /// Communications RFQ filter: empty or `self`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
    /// Pyth underlyings to seed, or `["all"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_tickers: Option<Vec<String>>,
    /// eBattle match IDs (Kalshi milestone IDs), or `["all"]`, for an
    /// `ebattle_scores` subscription.
    ///
    /// The exchange's subscribe command has no match filter: a new
    /// `ebattle_scores` subscription is empty until `subscribe_matches` is sent.
    /// This field is therefore never put on the wire in the subscribe command.
    /// The managed client sends a follow-up `subscribe_matches` update once the
    /// subscription is acknowledged, and repeats it after a reconnect replay.
    /// The low-level client rejects it; send `subscribe_matches` yourself.
    #[serde(default, skip_serializing)]
    pub match_ids: Option<Vec<String>>,
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

/// A subscription returned by `list_subscriptions`.
#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfo {
    pub sid: u64,
    pub channel: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfoRef<'a> {
    pub sid: u64,
    #[serde(borrow)]
    pub channel: Cow<'a, str>,
}

impl<'a> WsSubscriptionInfoRef<'a> {
    pub fn into_owned(self) -> WsSubscriptionInfo {
        WsSubscriptionInfo {
            sid: self.sid,
            channel: self.channel.into_owned(),
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
    /// CF Benchmarks index IDs to add or remove. Required for the
    /// `subscribe_indices` / `unsubscribe_indices` actions; use `["all"]` to
    /// track every available index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_tickers: Option<Vec<String>>,
    /// eBattle match IDs to add or remove. Required for `subscribe_matches` /
    /// `unsubscribe_matches`; use `["all"]` to select every match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_ids: Option<Vec<String>>,
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
    /// Request the available CF Benchmarks index IDs without modifying the subscription.
    Indexlist,
    /// Add Pyth underlyings to the subscription.
    SubscribeUnderlyings,
    /// Remove Pyth underlyings from the subscription.
    UnsubscribeUnderlyings,
    /// List recently observed Pyth underlyings without modifying the subscription.
    UnderlyingList,
    /// Add eBattle match IDs to an `ebattle_scores` subscription.
    SubscribeMatches,
    /// Remove eBattle match IDs from an `ebattle_scores` subscription.
    UnsubscribeMatches,
    /// List eBattle match IDs seen within the past two hours without modifying
    /// the subscription.
    Matchlist,
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

    pub fn is_underlying_action(self) -> bool {
        matches!(
            self,
            Self::SubscribeUnderlyings | Self::UnsubscribeUnderlyings | Self::UnderlyingList
        )
    }

    /// Whether this action operates on eBattle match IDs.
    pub fn is_match_action(self) -> bool {
        matches!(
            self,
            Self::SubscribeMatches | Self::UnsubscribeMatches | Self::Matchlist
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
    let has_underlyings = params
        .underlying_tickers
        .as_ref()
        .is_some_and(|v| !v.is_empty());

    let has_match_ids = params.match_ids.as_ref().is_some_and(|v| !v.is_empty());
    if params.action.is_match_action() {
        if params.market_ticker.is_some()
            || params.market_tickers.is_some()
            || params.market_id.is_some()
            || params.market_ids.is_some()
            || params.index_ids.is_some()
            || params.underlying_tickers.is_some()
        {
            return Err(KalshiError::InvalidParams(
                "update_subscription: match actions do not support market, index or underlying targets"
                    .to_string(),
            ));
        }
        if params.action != WsUpdateAction::Matchlist && !has_match_ids {
            return Err(KalshiError::InvalidParams(
                "update_subscription: subscribe_matches/unsubscribe_matches require match_ids"
                    .to_string(),
            ));
        }
        return Ok(());
    } else if params.match_ids.is_some() {
        return Err(KalshiError::InvalidParams(
            "update_subscription: match_ids requires a match action".to_string(),
        ));
    }

    if params.action.is_underlying_action() {
        if params.market_ticker.is_some()
            || params.market_tickers.is_some()
            || params.market_id.is_some()
            || params.market_ids.is_some()
            || params.index_ids.is_some()
        {
            return Err(KalshiError::InvalidParams(
                "update_subscription: underlying actions do not support market or index targets"
                    .to_string(),
            ));
        }
        if matches!(
            params.action,
            WsUpdateAction::SubscribeUnderlyings | WsUpdateAction::UnsubscribeUnderlyings
        ) && !has_underlyings
        {
            return Err(KalshiError::InvalidParams(
                "update_subscription: subscribe_underlyings/unsubscribe_underlyings require underlying_tickers".to_string(),
            ));
        }
    } else if params.underlying_tickers.is_some() {
        return Err(KalshiError::InvalidParams(
            "update_subscription: underlying_tickers requires an underlying action".to_string(),
        ));
    }

    // CF Benchmarks index actions are mutually exclusive with market targets and
    // have their own `index_ids` requirements.
    if params.action.is_index_action() {
        if params.market_ticker.is_some()
            || params.market_tickers.is_some()
            || params.market_id.is_some()
            || params.market_ids.is_some()
        {
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
    if params.market_tickers.as_ref().is_some_and(Vec::is_empty)
        || params.market_ids.as_ref().is_some_and(Vec::is_empty)
        || params.index_ids.as_ref().is_some_and(Vec::is_empty)
        || params
            .underlying_tickers
            .as_ref()
            .is_some_and(Vec::is_empty)
        || params.match_ids.as_ref().is_some_and(Vec::is_empty)
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: provided target lists must not be empty".to_string(),
        ));
    }
    if params
        .update_interval_ms
        .is_some_and(|v| !matches!(v, 0 | 100 | 1000))
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: update_interval_ms must be 0, 100 or 1000".to_string(),
        ));
    }
    if params
        .user_filter
        .as_deref()
        .is_some_and(|v| !matches!(v, "" | "self"))
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: user_filter must be empty or self".to_string(),
        ));
    }
    if params.shard_factor.is_some_and(|v| !(1..=100).contains(&v))
        || matches!((params.shard_key, params.shard_factor), (Some(key), Some(factor)) if key >= factor)
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: invalid shard factor or key".to_string(),
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

    if has_any_market_ids && has_market_positions {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_positions only supports market_tickers".to_string(),
        ));
    }

    if params.match_ids.is_some() && !params.channels.contains(&WsChannelV2::EbattleScores) {
        return Err(KalshiError::InvalidParams(
            "subscribe: match_ids is only valid with ebattle_scores".to_string(),
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
    fn validate_subscription_allows_ticker_initial_snapshot() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: Some(vec!["BRTI".to_string()]),
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: Some(vec!["BRTI".to_string()]),
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: Some(vec!["BRTI".to_string()]),
            underlying_tickers: None,
            match_ids: None,
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
    #[test]
    fn pyth_actions_enforce_documented_target_combinations() {
        let decode = |value| serde_json::from_value::<WsUpdateSubscriptionParamsV2>(value).unwrap();
        let valid = decode(
            serde_json::json!({"action":"subscribe_underlyings","sid":7,"underlying_tickers":["Metal.XAU/USD"]}),
        );
        assert!(validate_update(&valid).is_ok());
        assert_eq!(
            serde_json::to_value(&valid).unwrap()["underlying_tickers"],
            serde_json::json!(["Metal.XAU/USD"])
        );
        for value in [
            serde_json::json!({"action":"subscribe_underlyings","sid":7}),
            serde_json::json!({"action":"unsubscribe_underlyings","sid":7,"underlying_tickers":[]}),
            serde_json::json!({"action":"subscribe_underlyings","sid":7,"underlying_tickers":["all"],"index_ids":["BRTI"]}),
            serde_json::json!({"action":"underlying_list","sid":7,"market_ticker":"A"}),
            serde_json::json!({"action":"add_markets","sid":7,"underlying_tickers":["all"]}),
        ] {
            assert!(validate_update(&decode(value)).is_err());
        }
        assert!(
            validate_update(&decode(
                serde_json::json!({"action":"underlying_list","sid":7})
            ))
            .is_ok()
        );
    }

    #[test]
    fn subscription_options_serialize_exactly_and_validate_enums() {
        let mut params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ticker: Some("A".into()),
            use_yes_price: Some(false),
            update_interval_ms: Some(1000),
            ..Default::default()
        };
        let serialized = serde_json::to_value(&params).unwrap();
        assert_eq!(serialized["use_yes_price"], false);
        assert_eq!(serialized["update_interval_ms"], 1000);
        assert!(validate_subscription(&params).is_ok());
        params.update_interval_ms = Some(1);
        assert!(validate_subscription(&params).is_err());
        params.update_interval_ms = None;
        params.user_filter = Some("other".into());
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn ebattle_match_actions_and_subscribe_targets_are_validated() {
        let base: WsUpdateSubscriptionParamsV2 = serde_json::from_value(
            serde_json::json!({"action":"subscribe_matches","sid":1,"match_ids":["all"]}),
        )
        .unwrap();
        assert!(validate_update(&base).is_ok());
        assert_eq!(
            serde_json::to_value(&base).unwrap(),
            serde_json::json!({"action":"subscribe_matches","sid":1,"match_ids":["all"]})
        );
        for invalid in [
            serde_json::json!({"action":"subscribe_matches","sid":1}),
            serde_json::json!({"action":"unsubscribe_matches","sid":1,"match_ids":[]}),
            serde_json::json!({"action":"subscribe_matches","sid":1,"match_ids":["a"],"market_ticker":"T"}),
            serde_json::json!({"action":"add_markets","sid":1,"market_ticker":"T","match_ids":["a"]}),
        ] {
            let params: WsUpdateSubscriptionParamsV2 = serde_json::from_value(invalid).unwrap();
            assert!(validate_update(&params).is_err());
        }
        let list: WsUpdateSubscriptionParamsV2 =
            serde_json::from_value(serde_json::json!({"action":"matchlist","sid":1})).unwrap();
        assert!(validate_update(&list).is_ok());

        let ok = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::EbattleScores],
            match_ids: Some(vec!["all".into()]),
            ..Default::default()
        };
        assert!(validate_subscription(&ok).is_ok());
        assert_eq!(
            serde_json::to_value(&ok).unwrap(),
            serde_json::json!({"channels":["ebattle_scores"]})
        );
        let wrong_channel = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            match_ids: Some(vec!["all".into()]),
            ..Default::default()
        };
        assert!(validate_subscription(&wrong_channel).is_err());
        let empty = WsSubscriptionParamsV2 {
            match_ids: Some(vec![]),
            ..ok
        };
        assert!(validate_subscription(&empty).is_err());
    }
}
