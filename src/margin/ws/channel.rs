use serde::{Deserialize, Serialize};

/// WebSocket channel types for the Kalshi margin/perpetuals API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarginChannel {
    /// L2 order-book delta updates.
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta,
    /// Price / volume ticker snapshots.
    #[serde(rename = "ticker")]
    Ticker,
    /// Public trade events.
    #[serde(rename = "trade")]
    Trade,
    /// Your fill events (requires auth).
    #[serde(rename = "fill")]
    Fill,
    /// User order lifecycle updates (requires auth).
    #[serde(rename = "user_orders")]
    UserOrders,
    /// Order-group lifecycle updates (requires auth).
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates,
}

impl crate::ws::protocol::Channel for MarginChannel {
    fn is_private(&self) -> bool {
        matches!(
            self,
            Self::Fill | Self::UserOrders | Self::OrderGroupUpdates
        )
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::OrderbookDelta => "orderbook_delta",
            Self::Ticker => "ticker",
            Self::Trade => "trade",
            Self::Fill => "fill",
            Self::UserOrders => "user_orders",
            Self::OrderGroupUpdates => "order_group_updates",
        }
    }
}
