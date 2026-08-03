use serde::Deserialize;
use std::borrow::Cow;

/// Message payload for the `pyth_value` WebSocket channel.
///
/// Delivers deduplicated real-time Pyth prices by underlying ticker.
/// Duplicate and out-of-order source timestamps are ignored independently
/// per underlying ticker. Added 2026-07-23.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythValue {
    /// Qualified Pyth underlying ticker (e.g. `"Metal.XAU/USD"`).
    pub underlying_ticker: String,
    /// USD value formatted to 8 decimal places.
    pub value_usd: String,
    /// Pyth source timestamp (unix ms).
    pub source_ts_ms: i64,
    /// When Kalshi received the Pyth update (unix ms).
    pub received_at: i64,
}

/// Borrowed version of [`WsPythValue`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythValueRef<'a> {
    #[serde(borrow)]
    pub underlying_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub value_usd: Cow<'a, str>,
    pub source_ts_ms: i64,
    pub received_at: i64,
}

impl<'a> WsPythValueRef<'a> {
    pub fn into_owned(self) -> WsPythValue {
        WsPythValue {
            underlying_ticker: self.underlying_ticker.into_owned(),
            value_usd: self.value_usd.into_owned(),
            source_ts_ms: self.source_ts_ms,
            received_at: self.received_at,
        }
    }
}

/// Response to the `underlying_list` action on a `pyth_value` subscription.
/// Lists underlying tickers observed on the Pyth stream in the last two hours.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingList {
    pub underlying_tickers: Vec<String>,
}

/// Borrowed version of [`WsPythUnderlyingList`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingListRef<'a> {
    #[serde(borrow)]
    pub underlying_tickers: Vec<Cow<'a, str>>,
}

impl<'a> WsPythUnderlyingListRef<'a> {
    pub fn into_owned(self) -> WsPythUnderlyingList {
        WsPythUnderlyingList {
            underlying_tickers: self
                .underlying_tickers
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
        }
    }
}
