use crate::types::YesNo;
use serde::Deserialize;
use std::borrow::Cow;

/// Orderbook snapshot message (type: "orderbook_snapshot")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookSnapshot {
    pub market_ticker: String,
    pub market_id: String,
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
    pub price_dollars: String,
    pub delta_fp: String,
    pub side: YesNo,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[deprecated(note = "spec marks this as deprecated, yet optional. use ts_ms instead")]
    #[serde(default)]
    pub ts: Option<String>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

/// Orderbook snapshot message (type: "orderbook_snapshot")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookSnapshotRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default, borrow)]
    pub yes_dollars_fp: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default, borrow)]
    pub no_dollars_fp: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> WsOrderbookSnapshotRef<'a> {
    pub fn into_owned(self) -> WsOrderbookSnapshot {
        WsOrderbookSnapshot {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            yes_dollars_fp: self
                .yes_dollars_fp
                .into_iter()
                .map(|(p, q)| (p.into_owned(), q.into_owned()))
                .collect(),
            no_dollars_fp: self
                .no_dollars_fp
                .into_iter()
                .map(|(p, q)| (p.into_owned(), q.into_owned()))
                .collect(),
        }
    }
}

/// Orderbook delta message (type: "orderbook_delta")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookDeltaRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    #[serde(borrow)]
    pub price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub delta_fp: Cow<'a, str>,
    pub side: YesNo,
    #[serde(default, borrow)]
    pub client_order_id: Option<Cow<'a, str>>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default, borrow)]
    pub ts: Option<Cow<'a, str>>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

impl<'a> WsOrderbookDeltaRef<'a> {
    pub fn into_owned(self) -> WsOrderbookDelta {
        WsOrderbookDelta {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            price_dollars: self.price_dollars.into_owned(),
            delta_fp: self.delta_fp.into_owned(),
            side: self.side,
            client_order_id: self.client_order_id.map(Cow::into_owned),
            subaccount: self.subaccount,
            #[allow(deprecated)]
            ts: self.ts.map(Cow::into_owned),
            ts_ms: self.ts_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::*;

    #[test]
    fn ws_orderbook_delta_side_parse() {
        let json = r#"{
            "market_ticker":"TEST",
            "market_id":"1",
            "price_dollars":"0.01",
            "delta_fp":"1",
            "side":"yes"
        }"#;
        let delta: WsOrderbookDelta = serde_json::from_str(json).unwrap();
        assert!(matches!(delta.side, YesNo::Yes));
    }

    #[test]
    fn ws_orderbook_delta_timestamps_are_optional() {
        let json_missing_both = r#"{
            "market_ticker":"TEST",
            "market_id":"1",
            "price_dollars":"0.01",
            "delta_fp":"1",
            "side":"yes"
        }"#;
        let delta_missing_both: WsOrderbookDelta = serde_json::from_str(json_missing_both).unwrap();
        assert_eq!(delta_missing_both.ts, None);
        assert_eq!(delta_missing_both.ts_ms, None);

        let json_missing_ts_only = r#"{
            "market_ticker":"TEST",
            "market_id":"1",
            "price_dollars":"0.01",
            "delta_fp":"1",
            "side":"yes",
            "ts_ms": 1669149841000
        }"#;
        let delta_missing_ts: WsOrderbookDelta =
            serde_json::from_str(json_missing_ts_only).unwrap();
        assert_eq!(delta_missing_ts.ts, None);
        assert_eq!(delta_missing_ts.ts_ms, Some(1669149841000));

        let json_missing_ts_ms_only = r#"{
            "market_ticker":"TEST",
            "market_id":"1",
            "price_dollars":"0.01",
            "delta_fp":"1",
            "side":"yes",
            "ts":"2022-11-22T20:44:01Z"
        }"#;
        let delta_missing_ts_ms: WsOrderbookDelta =
            serde_json::from_str(json_missing_ts_ms_only).unwrap();
        assert_eq!(
            delta_missing_ts_ms.ts.as_deref(),
            Some("2022-11-22T20:44:01Z")
        );
        assert_eq!(delta_missing_ts_ms.ts_ms, None);
    }
}
