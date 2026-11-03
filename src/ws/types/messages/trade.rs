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
    #[serde(default)]
    #[deprecated(note = "use taker_outcome_side")]
    pub taker_side: Option<TradeTakerSide>,
    pub taker_outcome_side: TradeTakerSide,
    pub taker_book_side: BookSide,
    pub is_block_trade: bool,
    #[serde(default)]
    #[deprecated(note = "use ts_ms")]
    pub ts: Option<i64>,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
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
    #[serde(default)]
    #[deprecated(note = "use taker_outcome_side")]
    pub taker_side: Option<TradeTakerSide>,
    pub taker_outcome_side: TradeTakerSide,
    pub taker_book_side: BookSide,
    pub is_block_trade: bool,
    #[serde(default)]
    #[deprecated(note = "use ts_ms")]
    pub ts: Option<i64>,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

impl<'a> WsTradeRef<'a> {
    #[allow(deprecated)]
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
            is_block_trade: self.is_block_trade,
            ts: self.ts,
            ts_ms: self.ts_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::*;

    #[test]
    fn deprecated_trade_fields_accept_absent_null_and_present() {
        for (fields, expected_side, expected_ts) in [
            ("", None, None),
            (r#","taker_side":null,"ts":null"#, None, None),
            (
                r#","taker_side":"yes","ts":1704067200"#,
                Some("yes"),
                Some(1704067200),
            ),
            (r#","taker_side":"no","ts":null"#, Some("no"), None),
            (
                r#","taker_side":null,"ts":1704067200"#,
                None,
                Some(1704067200),
            ),
        ] {
            let json = format!(
                r#"{{"trade_id":"t1","market_ticker":"TEST","count_fp":"1.00","yes_price_dollars":"0.50","no_price_dollars":"0.50","taker_outcome_side":"yes","taker_book_side":"bid","is_block_trade":false,"ts_ms":1704067200000{fields}}}"#
            );
            let owned: WsTrade = serde_json::from_slice(json.as_bytes()).unwrap();
            let borrowed: WsTradeRef<'_> = serde_json::from_slice(json.as_bytes()).unwrap();
            assert_eq!(owned.taker_side.map(TradeTakerSide::as_str), expected_side);
            assert_eq!(owned.ts, expected_ts);
            assert_eq!(owned.ts_ms, Some(1704067200000));
            assert_eq!(
                borrowed.taker_side.map(TradeTakerSide::as_str),
                expected_side
            );
            assert_eq!(borrowed.ts, expected_ts);
            assert!(matches!(&borrowed.trade_id, Cow::Borrowed("t1")));
            let converted = borrowed.into_owned();
            assert_eq!(
                converted.taker_side.map(TradeTakerSide::as_str),
                expected_side
            );
            assert_eq!(converted.ts, expected_ts);
            assert_eq!(converted.ts_ms, Some(1704067200000));
        }
    }
}
