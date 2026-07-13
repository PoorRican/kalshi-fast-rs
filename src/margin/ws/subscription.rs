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
    /// Perpetual market tickers (required for orderbook_delta).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    /// Sub-account ID for private channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_id: Option<String>,
}
