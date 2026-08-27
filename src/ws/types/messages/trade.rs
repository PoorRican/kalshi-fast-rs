use crate::types::{BookSide, TradeTakerSide};
use serde::Deserialize;
use std::borrow::Cow;

/// Trade channel message (type: "trade")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTrade {
    pub trade_id: String,
    #[serde(alias = "ticker")]
    pub market_ticker: String,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub no_price_dollars: String,
    /// Deprecated 2026-05-07. Use `taker_outcome_side` / `taker_book_side`.
    /// Optional to tolerate eventual removal by the exchange.
    #[serde(default)]
    pub taker_side: Option<TradeTakerSide>,
    /// Normalized taker outcome side (yes | no). Added 2026-05-07.
    #[serde(default)]
    pub taker_outcome_side: Option<TradeTakerSide>,
    /// Normalized taker book side (bid | ask). Added 2026-05-07.
    #[serde(default)]
    pub taker_book_side: Option<BookSide>,
    pub ts: i64,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default)]
    pub created_time: Option<String>,
    /// True if the trade was matched off-book as a block trade. Added 2026-08-13.
    /// Mirrors the REST `Trade::is_block_trade` field. Defaults to `false` for
    /// payloads predating this field.
    #[serde(default)]
    pub is_block_trade: bool,
}

/// Trade channel message (type: "trade")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTradeRef<'a> {
    #[serde(borrow)]
    pub trade_id: Cow<'a, str>,
    #[serde(alias = "ticker", borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub no_price_dollars: Cow<'a, str>,
    /// Deprecated 2026-05-07. Use `taker_outcome_side` / `taker_book_side`.
    /// Optional to tolerate eventual removal by the exchange.
    #[serde(default)]
    pub taker_side: Option<TradeTakerSide>,
    /// Normalized taker outcome side (yes | no). Added 2026-05-07.
    #[serde(default)]
    pub taker_outcome_side: Option<TradeTakerSide>,
    /// Normalized taker book side (bid | ask). Added 2026-05-07.
    #[serde(default)]
    pub taker_book_side: Option<BookSide>,
    pub ts: i64,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default, borrow)]
    pub created_time: Option<Cow<'a, str>>,
    /// True if the trade was matched off-book as a block trade. Added 2026-08-13.
    /// Mirrors the REST `Trade::is_block_trade` field. Defaults to `false` for
    /// payloads predating this field.
    #[serde(default)]
    pub is_block_trade: bool,
}

impl<'a> WsTradeRef<'a> {
    pub fn into_owned(self) -> WsTrade {
        WsTrade {
            trade_id: self.trade_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            count_fp: self.count_fp.into_owned(),
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            no_price_dollars: self.no_price_dollars.into_owned(),
            taker_side: self.taker_side,
            taker_outcome_side: self.taker_outcome_side,
            taker_book_side: self.taker_book_side,
            ts: self.ts,
            ts_ms: self.ts_ms,
            created_time: self.created_time.map(Cow::into_owned),
            is_block_trade: self.is_block_trade,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `is_block_trade` was added 2026-08-13; payloads predating the field must
    /// still parse and default to `false`, and the borrowed path must round-trip.
    #[test]
    fn is_block_trade_defaults_false_and_round_trips() {
        let without_flag = r#"{
            "trade_id": "t1",
            "market_ticker": "TST",
            "count_fp": "2.00",
            "yes_price_dollars": "0.10",
            "no_price_dollars": "0.90",
            "taker_side": "yes",
            "ts": 1704067200,
            "ts_ms": 1704067200000
        }"#;

        let owned: WsTrade = serde_json::from_str(without_flag).unwrap();
        assert!(!owned.is_block_trade);

        let borrowed: WsTradeRef = serde_json::from_str(without_flag).unwrap();
        assert!(!borrowed.into_owned().is_block_trade);

        let with_flag = r#"{
            "trade_id": "t2",
            "market_ticker": "TST",
            "count_fp": "136.00",
            "yes_price_dollars": "0.360",
            "no_price_dollars": "0.640",
            "taker_side": "no",
            "is_block_trade": true,
            "ts": 1669149841,
            "ts_ms": 1669149841000
        }"#;

        let owned: WsTrade = serde_json::from_str(with_flag).unwrap();
        assert!(owned.is_block_trade);

        let borrowed: WsTradeRef = serde_json::from_str(with_flag).unwrap();
        assert!(borrowed.into_owned().is_block_trade);
    }
}
