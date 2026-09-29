use crate::margin::ws::channel::MarginChannel;
use serde::{Deserialize, Serialize};

/// Subscription parameters for the margin/perpetuals WebSocket.
///
/// These mirror the structure of event-contract subscription params but
/// use margin-specific channels and accept perps market tickers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarginSubscribeParams {
    /// Channels to subscribe to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<MarginChannel>>,
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

        let params = MarginSubscribeParams {
            send_initial_snapshot: Some(true),
            skip_ticker_ack: Some(false),
            ..MarginSubscribeParams::default()
        };
        let value = serde_json::to_value(params).unwrap();
        assert_eq!(value["send_initial_snapshot"], true);
        assert_eq!(value["skip_ticker_ack"], false);
    }
}
