use serde::Deserialize;
use std::borrow::Cow;

/// Ticker channel message (type: "ticker")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTicker {
    pub market_ticker: String,
    pub market_id: String,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub yes_bid_size_fp: String,
    pub yes_ask_size_fp: String,
    pub last_trade_size_fp: String,
    pub volume_fp: String,
    pub open_interest_fp: String,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    #[serde(default)]
    #[deprecated(note = "use ts_ms")]
    pub ts: Option<i64>,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default)]
    #[deprecated(note = "use ts_ms")]
    pub time: Option<String>,
}

/// Ticker channel message (type: "ticker")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTickerRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    #[serde(borrow)]
    pub price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_bid_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_ask_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_bid_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_ask_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub last_trade_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub volume_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub open_interest_fp: Cow<'a, str>,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    #[serde(default)]
    #[deprecated(note = "use ts_ms")]
    pub ts: Option<i64>,
    /// Spec marks `ts_ms` as required, but the exchange occasionally omits it.
    /// See `docs/spec-parity.md`.
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    #[deprecated(note = "use ts_ms")]
    pub time: Option<Cow<'a, str>>,
}

impl<'a> WsTickerRef<'a> {
    #[allow(deprecated)]
    pub fn into_owned(self) -> WsTicker {
        WsTicker {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            price_dollars: self.price_dollars.into_owned(),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            yes_ask_dollars: self.yes_ask_dollars.into_owned(),
            yes_bid_size_fp: self.yes_bid_size_fp.into_owned(),
            yes_ask_size_fp: self.yes_ask_size_fp.into_owned(),
            last_trade_size_fp: self.last_trade_size_fp.into_owned(),
            volume_fp: self.volume_fp.into_owned(),
            open_interest_fp: self.open_interest_fp.into_owned(),
            dollar_volume: self.dollar_volume,
            dollar_open_interest: self.dollar_open_interest,
            ts: self.ts,
            ts_ms: self.ts_ms,
            time: self.time.map(Cow::into_owned),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::*;

    #[test]
    fn deprecated_ticker_fields_accept_absent_null_and_present() {
        for (fields, expected_ts, expected_time) in [
            ("", None, None),
            (r#","ts":null,"time":null"#, None, None),
            (
                r#","ts":1704067200,"time":"2024-01-01T00:00:00Z""#,
                Some(1704067200),
                Some("2024-01-01T00:00:00Z"),
            ),
            (r#","ts":1704067200,"time":null"#, Some(1704067200), None),
            (
                r#","ts":null,"time":"2024-01-01T00:00:00Z""#,
                None,
                Some("2024-01-01T00:00:00Z"),
            ),
        ] {
            let json = format!(
                r#"{{"market_ticker":"TEST","market_id":"m1","price_dollars":"0.50","yes_bid_dollars":"0.49","yes_ask_dollars":"0.51","yes_bid_size_fp":"1.00","yes_ask_size_fp":"1.00","last_trade_size_fp":"1.00","volume_fp":"10.00","open_interest_fp":"5.00","dollar_volume":5,"dollar_open_interest":2,"ts_ms":1704067200000{fields}}}"#
            );
            let owned: WsTicker = serde_json::from_slice(json.as_bytes()).unwrap();
            let borrowed: WsTickerRef<'_> = serde_json::from_slice(json.as_bytes()).unwrap();
            assert_eq!(owned.ts, expected_ts);
            assert_eq!(owned.time.as_deref(), expected_time);
            assert_eq!(owned.ts_ms, Some(1704067200000));
            assert_eq!(borrowed.ts, expected_ts);
            assert_eq!(borrowed.time.as_deref(), expected_time);
            if expected_time.is_some() {
                assert!(matches!(&borrowed.time, Some(Cow::Borrowed(_))));
            }
            let converted = borrowed.into_owned();
            assert_eq!(converted.ts, expected_ts);
            assert_eq!(converted.time.as_deref(), expected_time);
            assert_eq!(converted.ts_ms, Some(1704067200000));
        }
    }

    #[test]
    fn deprecated_ticker_time_accepts_escaped_strings() {
        let json = br#"{"market_ticker":"TEST","market_id":"m1","price_dollars":"0.50","yes_bid_dollars":"0.49","yes_ask_dollars":"0.51","yes_bid_size_fp":"1.00","yes_ask_size_fp":"1.00","last_trade_size_fp":"1.00","volume_fp":"10.00","open_interest_fp":"5.00","dollar_volume":5,"dollar_open_interest":2,"time":"2024\u002d01"}"#;
        let owned: WsTicker = serde_json::from_slice(json).unwrap();
        let borrowed: WsTickerRef<'_> = serde_json::from_slice(json).unwrap();
        assert_eq!(owned.time.as_deref(), Some("2024-01"));
        assert!(matches!(&borrowed.time, Some(Cow::Owned(_))));
        assert_eq!(borrowed.time.as_deref(), Some("2024-01"));
        assert_eq!(borrowed.into_owned().time.as_deref(), Some("2024-01"));
    }
}
