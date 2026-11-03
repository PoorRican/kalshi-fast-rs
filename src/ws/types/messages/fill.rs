use crate::types::{BookSide, BuySell, YesNo};
use serde::Deserialize;
use std::borrow::Cow;

/// Fill channel message (type: "fill")
#[derive(Debug, Clone, Deserialize)]
pub struct WsFill {
    pub trade_id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(alias = "ticker")]
    pub market_ticker: String,
    pub exchange_index: i32,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub action: Option<BuySell>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub is_taker: bool,
    pub fee_cost: String,
    #[deprecated(note = "use ts_ms instead")]
    #[serde(default)]
    pub ts: Option<i64>,
    pub ts_ms: i64,
    pub post_position_fp: String,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub purchased_side: Option<YesNo>,
    #[serde(default)]
    #[serde(alias = "subaccount_number")]
    pub subaccount: Option<i64>,
}

/// Fill channel message (type: "fill")
#[derive(Debug, Clone, Deserialize)]
pub struct WsFillRef<'a> {
    #[serde(borrow)]
    pub trade_id: Cow<'a, str>,
    #[serde(borrow)]
    pub order_id: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub client_order_id: Option<Cow<'a, str>>,
    #[serde(alias = "ticker", borrow)]
    pub market_ticker: Cow<'a, str>,
    pub exchange_index: i32,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub action: Option<BuySell>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_price_dollars: Cow<'a, str>,
    pub is_taker: bool,
    #[serde(borrow)]
    pub fee_cost: Cow<'a, str>,
    #[deprecated(note = "use ts_ms instead")]
    #[serde(default)]
    pub ts: Option<i64>,
    pub ts_ms: i64,
    #[serde(borrow)]
    pub post_position_fp: Cow<'a, str>,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub purchased_side: Option<YesNo>,
    #[serde(default)]
    #[serde(alias = "subaccount_number")]
    pub subaccount: Option<i64>,
}

impl<'a> WsFillRef<'a> {
    #[allow(deprecated)]
    pub fn into_owned(self) -> WsFill {
        WsFill {
            trade_id: self.trade_id.into_owned(),
            order_id: self.order_id.into_owned(),
            client_order_id: self.client_order_id.map(Cow::into_owned),
            market_ticker: self.market_ticker.into_owned(),
            exchange_index: self.exchange_index,
            side: self.side,
            action: self.action,
            outcome_side: self.outcome_side,
            book_side: self.book_side,
            count_fp: self.count_fp.into_owned(),
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            is_taker: self.is_taker,
            fee_cost: self.fee_cost.into_owned(),
            ts: self.ts,
            ts_ms: self.ts_ms,
            post_position_fp: self.post_position_fp.into_owned(),
            purchased_side: self.purchased_side,
            subaccount: self.subaccount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    #[test]
    fn ws_fill_legacy_side_action_parse() {
        let json = r#"{
            "trade_id":"t",
            "order_id":"o",
            "market_ticker":"T",
            "exchange_index":0,
            "outcome_side":"no",
            "book_side":"ask",
            "side":"no",
            "action":"buy",
            "count_fp":"1",
            "yes_price_dollars":"0.01",
            "is_taker":true,
            "fee_cost":"0.00",
            "ts":0,
            "ts_ms":0,
            "post_position_fp":"1.00",
            "purchased_side":"yes"
        }"#;
        let fill: WsFill = serde_json::from_str(json).unwrap();
        assert!(matches!(fill.side, Some(YesNo::No)));
        assert!(matches!(fill.action, Some(BuySell::Buy)));
    }

    #[test]
    fn ws_fill_normalized_fields_parse() {
        let json = r#"{
            "trade_id":"t",
            "order_id":"o",
            "market_ticker":"T",
            "exchange_index":0,
            "side":"yes",
            "action":"buy",
            "outcome_side":"yes",
            "book_side":"bid",
            "count_fp":"1",
            "yes_price_dollars":"0.01",
            "is_taker":true,
            "fee_cost":"0.00",
            "ts":0,
            "ts_ms":0,
            "post_position_fp":"1.00",
            "purchased_side":"yes"
        }"#;
        let fill: WsFill = serde_json::from_str(json).unwrap();
        assert!(matches!(fill.outcome_side, YesNo::Yes));
        assert!(matches!(fill.book_side, BookSide::Bid));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_fill_fields_accept_present_absent_and_null() {
        for (legacy_fields, present) in [
            (
                r#","side":"no","action":"buy","purchased_side":"yes","ts":123"#,
                true,
            ),
            ("", false),
            (
                r#","side":null,"action":null,"purchased_side":null,"ts":null"#,
                false,
            ),
        ] {
            let json = format!(
                r#"{{"trade_id":"t","order_id":"o","market_ticker":"T",
                    "exchange_index":0,"outcome_side":"no","book_side":"ask",
                    "count_fp":"1","yes_price_dollars":"0.01","is_taker":true,
                    "fee_cost":"0.00","ts_ms":123000,"post_position_fp":"1.00"{legacy_fields}}}"#
            );
            let owned: WsFill = serde_json::from_slice(json.as_bytes()).unwrap();
            let borrowed: WsFillRef<'_> = serde_json::from_slice(json.as_bytes()).unwrap();

            assert!(matches!(&borrowed.trade_id, Cow::Borrowed("t")));
            if present {
                assert!(matches!(borrowed.side, Some(YesNo::No)));
                assert!(matches!(borrowed.action, Some(BuySell::Buy)));
                assert!(matches!(borrowed.purchased_side, Some(YesNo::Yes)));
                assert_eq!(borrowed.ts, Some(123));
            } else {
                assert!(borrowed.side.is_none());
                assert!(borrowed.action.is_none());
                assert!(borrowed.purchased_side.is_none());
                assert!(borrowed.ts.is_none());
            }
            for fill in [owned, borrowed.into_owned()] {
                if present {
                    assert!(matches!(fill.side, Some(YesNo::No)));
                    assert!(matches!(fill.action, Some(BuySell::Buy)));
                    assert!(matches!(fill.purchased_side, Some(YesNo::Yes)));
                    assert_eq!(fill.ts, Some(123));
                } else {
                    assert!(fill.side.is_none());
                    assert!(fill.action.is_none());
                    assert!(fill.purchased_side.is_none());
                    assert!(fill.ts.is_none());
                }
                assert!(matches!(fill.outcome_side, YesNo::No));
                assert!(matches!(fill.book_side, BookSide::Ask));
                assert_eq!(fill.ts_ms, 123000);
            }
        }
    }
}
