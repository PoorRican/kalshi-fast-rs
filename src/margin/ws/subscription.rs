use crate::error::KalshiError;
use crate::margin::ws::channel::MarginChannel;
use serde::{Deserialize, Serialize};

/// Subscription parameters for the margin/perpetuals WebSocket.
///
/// These mirror the structure of event-contract subscription params but
/// use margin-specific channels and accept perps market tickers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarginSubscribeParams {
    /// Channels to subscribe to.
    pub channels: Vec<MarginChannel>,
    /// A single perpetual market ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    /// Perps market tickers (required for orderbook_delta).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    /// Omits ticker acknowledgements when enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    /// Orderbook conflation interval: 0 (realtime), 100 or 1000 milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval_ms: Option<i64>,
}

/// Parameters for modifying one margin subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginUpdateSubscriptionParams {
    pub action: MarginUpdateAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginUpdateAction {
    AddMarkets,
    DeleteMarkets,
    GetSnapshot,
}

impl MarginUpdateSubscriptionParams {
    pub(crate) fn target_sid(&self) -> Option<u64> {
        self.sid
            .or_else(|| self.sids.as_ref().and_then(|sids| sids.first().copied()))
    }
}

pub(crate) fn validate_subscription(params: &MarginSubscribeParams) -> Result<(), KalshiError> {
    if params.channels.is_empty() {
        return Err(KalshiError::InvalidParams(
            "subscribe: at least one channel is required".into(),
        ));
    }
    if params.market_tickers.as_ref().is_some_and(Vec::is_empty) {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_tickers cannot be empty".into(),
        ));
    }
    let has_markets = params.market_ticker.is_some() || params.market_tickers.is_some();
    if params.channels.contains(&MarginChannel::OrderbookDelta) && !has_markets {
        return Err(KalshiError::InvalidParams(
            "subscribe: orderbook_delta requires market tickers".into(),
        ));
    }
    if params.channels.contains(&MarginChannel::MarketLifecycleV2) && has_markets {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_lifecycle_v2 does not support market filters".into(),
        ));
    }
    if params
        .update_interval_ms
        .is_some_and(|v| !matches!(v, 0 | 100 | 1000))
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: update_interval_ms must be 0, 100 or 1000".into(),
        ));
    }
    if params.update_interval_ms.is_some()
        && !params.channels.contains(&MarginChannel::OrderbookDelta)
    {
        return Err(KalshiError::InvalidParams(
            "subscribe: update_interval_ms requires orderbook_delta".into(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_update(params: &MarginUpdateSubscriptionParams) -> Result<(), KalshiError> {
    if params.sid.is_some() == params.sids.is_some()
        || params.sids.as_ref().is_some_and(|sids| sids.len() != 1)
    {
        return Err(KalshiError::InvalidParams(
            "update_subscription: provide exactly one sid".into(),
        ));
    }
    if params.market_tickers.as_ref().is_some_and(Vec::is_empty) {
        return Err(KalshiError::InvalidParams(
            "update_subscription: market_tickers cannot be empty".into(),
        ));
    }
    if params.action != MarginUpdateAction::GetSnapshot
        && params.market_ticker.is_none()
        && params.market_tickers.is_none()
    {
        return Err(KalshiError::InvalidParams(
            "update_subscription: market tickers are required".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_ticker_flags_are_serialized_only_when_set() {
        let default = MarginSubscribeParams::default();
        let value = serde_json::to_value(default).unwrap();
        assert!(value.get("send_initial_snapshot").is_none());
        assert!(value.get("skip_ticker_ack").is_none());
        assert!(value.get("update_interval_ms").is_none());

        let params = MarginSubscribeParams {
            send_initial_snapshot: Some(true),
            skip_ticker_ack: Some(false),
            update_interval_ms: Some(100),
            ..MarginSubscribeParams::default()
        };
        let value = serde_json::to_value(params).unwrap();
        assert_eq!(value["send_initial_snapshot"], true);
        assert_eq!(value["skip_ticker_ack"], false);
        assert_eq!(value["update_interval_ms"], 100);
    }

    #[test]
    fn channels_are_required_and_serialize_as_an_array_when_empty() {
        assert!(serde_json::from_str::<MarginSubscribeParams>("{}").is_err());

        let params = MarginSubscribeParams::default();
        let value = serde_json::to_value(params).unwrap();
        assert_eq!(value["channels"], serde_json::json!([]));
    }

    #[test]
    fn margin_channel_and_update_validation() {
        let params = MarginSubscribeParams {
            channels: vec![MarginChannel::OrderbookDelta],
            market_ticker: Some("T".into()),
            update_interval_ms: Some(1000),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
        let mut invalid = params.clone();
        invalid.update_interval_ms = Some(50);
        assert!(validate_subscription(&invalid).is_err());
        let update = MarginUpdateSubscriptionParams {
            action: MarginUpdateAction::GetSnapshot,
            sid: Some(4),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            send_initial_snapshot: None,
        };
        assert!(validate_update(&update).is_ok());
        assert_eq!(update.target_sid(), Some(4));
        assert_eq!(
            serde_json::to_value(update).unwrap()["action"],
            "get_snapshot"
        );
        assert_eq!(
            serde_json::to_value(MarginChannel::CorporateActions).unwrap(),
            "corporate_actions"
        );
        assert_eq!(
            serde_json::to_value(MarginChannel::MarketLifecycleV2).unwrap(),
            "market_lifecycle_v2"
        );
    }
}
