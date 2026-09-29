use crate::ws::protocol::Channel;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsChannelV2 {
    Ticker,
    Trade,
    MarketLifecycleV2,
    MultivariateMarketLifecycle,
    Multivariate,
    OrderbookDelta,
    Fill,
    MarketPositions,
    Communications,
    OrderGroupUpdates,
    UserOrders,
    /// CF Benchmarks reference index value feed. Added 2026-06-08 (AsyncAPI 2.0.0).
    CfbenchmarksValue,
}
impl WsChannelV2 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ticker => "ticker",
            Self::Trade => "trade",
            Self::MarketLifecycleV2 => "market_lifecycle_v2",
            Self::MultivariateMarketLifecycle => "multivariate_market_lifecycle",
            Self::Multivariate => "multivariate",
            Self::OrderbookDelta => "orderbook_delta",
            Self::Fill => "fill",
            Self::MarketPositions => "market_positions",
            Self::Communications => "communications",
            Self::OrderGroupUpdates => "order_group_updates",
            Self::UserOrders => "user_orders",
            Self::CfbenchmarksValue => "cfbenchmarks_value",
        }
    }

    pub fn is_private(self) -> bool {
        matches!(
            self,
            Self::OrderbookDelta
                | Self::Fill
                | Self::MarketPositions
                | Self::Communications
                | Self::OrderGroupUpdates
                | Self::UserOrders
        )
    }
}

impl crate::ws::protocol::Channel for WsChannelV2 {
    fn is_private(&self) -> bool {
        WsChannelV2::is_private(*self)
    }

    fn as_str(&self) -> &'static str {
        WsChannelV2::as_str(*self)
    }
}

impl fmt::Display for WsChannelV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_channel_check() {
        assert!(WsChannelV2::Fill.is_private());
        assert!(WsChannelV2::OrderbookDelta.is_private());
        assert!(WsChannelV2::MarketPositions.is_private());
        assert!(WsChannelV2::Communications.is_private());
        assert!(WsChannelV2::OrderGroupUpdates.is_private());

        assert!(!WsChannelV2::Ticker.is_private());
        assert!(!WsChannelV2::Trade.is_private());
        assert!(!WsChannelV2::MarketLifecycleV2.is_private());
        assert!(!WsChannelV2::Multivariate.is_private());
    }
}
