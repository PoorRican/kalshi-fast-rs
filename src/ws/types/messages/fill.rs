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
    /// Legacy direction fields remain part of the published payload.
    pub side: YesNo,
    pub action: BuySell,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub is_taker: bool,
    pub fee_cost: String,
    pub ts: i64,
    pub ts_ms: i64,
    pub post_position_fp: String,
    pub purchased_side: YesNo,
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
    pub side: YesNo,
    pub action: BuySell,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_price_dollars: Cow<'a, str>,
    pub is_taker: bool,
    #[serde(borrow)]
    pub fee_cost: Cow<'a, str>,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(borrow)]
    pub post_position_fp: Cow<'a, str>,
    pub purchased_side: YesNo,
    #[serde(default)]
    #[serde(alias = "subaccount_number")]
    pub subaccount: Option<i64>,
}

impl<'a> WsFillRef<'a> {
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
        assert!(matches!(fill.side, YesNo::No));
        assert!(matches!(fill.action, BuySell::Buy));
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
}
