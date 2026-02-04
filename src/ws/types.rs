use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum WsChannel {
    // Public (no auth required)
    Ticker,
    TickerV2,
    Trade,
    MarketLifecycleV2,
    Multivariate,

    // Private (auth required)
    OrderbookDelta,
    Fill,
    MarketPositions,
    Communications,
    OrderGroupUpdates,
}

impl WsChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            WsChannel::Ticker => "ticker",
            WsChannel::TickerV2 => "ticker_v2",
            WsChannel::Trade => "trade",
            WsChannel::MarketLifecycleV2 => "market_lifecycle_v2",
            WsChannel::Multivariate => "multivariate",
            WsChannel::OrderbookDelta => "orderbook_delta",
            WsChannel::Fill => "fill",
            WsChannel::MarketPositions => "market_positions",
            WsChannel::Communications => "communications",
            WsChannel::OrderGroupUpdates => "order_group_updates",
        }
    }

    pub fn is_private(self) -> bool {
        matches!(
            self,
            WsChannel::OrderbookDelta
                | WsChannel::Fill
                | WsChannel::MarketPositions
                | WsChannel::Communications
                | WsChannel::OrderGroupUpdates
        )
    }
}

impl fmt::Display for WsChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for WsChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Ticker channel message (type: "ticker")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTicker {
    pub market_ticker: String,
    pub market_id: String,
    pub price: i64,
    pub yes_bid: i64,
    pub yes_ask: i64,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub volume: i64,
    pub volume_fp: String,
    pub open_interest: i64,
    pub open_interest_fp: String,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub ts: i64,
}

/// Orderbook snapshot message (type: "orderbook_snapshot")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookSnapshot {
    pub market_ticker: String,
    pub market_id: String,
    /// Price levels: (price_cents, quantity)
    #[serde(default)]
    pub yes: Vec<(i64, i64)>,
    /// Price levels: (price_cents, quantity)
    #[serde(default)]
    pub no: Vec<(i64, i64)>,
    /// Price levels: (price_dollars, quantity)
    #[serde(default)]
    pub yes_dollars: Vec<(String, i64)>,
    /// Price levels: (price_dollars, quantity)
    #[serde(default)]
    pub no_dollars: Vec<(String, i64)>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default)]
    pub yes_dollars_fp: Vec<(String, String)>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default)]
    pub no_dollars_fp: Vec<(String, String)>,
}

/// Orderbook delta message (type: "orderbook_delta")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookDelta {
    pub market_ticker: String,
    pub market_id: String,
    pub price: i64,
    pub price_dollars: String,
    pub delta: i64,
    pub delta_fp: String,
    pub side: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default)]
    pub ts: Option<String>,
}

/// Fill channel message (type: "fill")
#[derive(Debug, Clone, Deserialize)]
pub struct WsFill {
    pub fill_id: String,
    pub trade_id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub ticker: String,
    pub market_ticker: String,
    pub side: String,
    pub action: String,
    pub count: i64,
    pub count_fp: String,
    pub yes_price: i64,
    pub no_price: i64,
    pub yes_price_fixed: String,
    pub no_price_fixed: String,
    pub is_taker: bool,
    pub fee_cost: String,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub subaccount_number: Option<i64>,
    #[serde(default)]
    pub ts: Option<i64>,
}

/// Envelope used by Kalshi WS (data + errors use "type")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEnvelope {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub sid: Option<u64>,
    pub seq: Option<u64>,
    pub msg: Option<serde_json::Value>,
}

impl WsEnvelope {
    /// Parse inner message as a ticker update.
    pub fn parse_ticker(&self) -> Result<WsTicker, serde_json::Error> {
        serde_json::from_value(self.msg.clone().unwrap_or_default())
    }

    /// Parse inner message as an orderbook snapshot.
    pub fn parse_orderbook_snapshot(&self) -> Result<WsOrderbookSnapshot, serde_json::Error> {
        serde_json::from_value(self.msg.clone().unwrap_or_default())
    }

    /// Parse inner message as an orderbook delta.
    pub fn parse_orderbook_delta(&self) -> Result<WsOrderbookDelta, serde_json::Error> {
        serde_json::from_value(self.msg.clone().unwrap_or_default())
    }

    /// Parse inner message as a fill.
    pub fn parse_fill(&self) -> Result<WsFill, serde_json::Error> {
        serde_json::from_value(self.msg.clone().unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsSubscribeCmd {
    pub id: u64,
    pub cmd: &'static str, // "subscribe"
    pub params: WsSubscribeParams,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsSubscribeParams {
    pub channels: Vec<WsChannel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
}
