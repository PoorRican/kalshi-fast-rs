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
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub action: Option<BuySell>,
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub purchased_side: Option<YesNo>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub is_taker: bool,
    pub fee_cost: String,
    pub ts: i64,
    pub ts_ms: i64,
    pub post_position_fp: String,
    #[serde(default)]
    pub created_time: Option<String>,
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
    #[serde(default, borrow)]
    pub client_order_id: Option<Cow<'a, str>>,
    #[serde(alias = "ticker", borrow)]
    pub market_ticker: Cow<'a, str>,
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub action: Option<BuySell>,
    /// Deprecated — use `outcome_side` instead. Still required until removed upstream.
    #[deprecated(note = "use outcome_side instead")]
    #[serde(default)]
    pub purchased_side: Option<YesNo>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    pub yes_price_dollars: Cow<'a, str>,
    pub is_taker: bool,
    #[serde(borrow)]
    pub fee_cost: Cow<'a, str>,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(borrow)]
    pub post_position_fp: Cow<'a, str>,
    #[serde(default, borrow)]
    pub created_time: Option<Cow<'a, str>>,
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
            side: self.side,
            action: self.action,
            purchased_side: self.purchased_side,
            outcome_side: self.outcome_side,
            book_side: self.book_side,
            count_fp: self.count_fp.into_owned(),
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            is_taker: self.is_taker,
            fee_cost: self.fee_cost.into_owned(),
            ts: self.ts,
            ts_ms: self.ts_ms,
            post_position_fp: self.post_position_fp.into_owned(),
            created_time: self.created_time.map(Cow::into_owned),
            subaccount: self.subaccount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    #[test]
    fn ws_fill_side_action_parse() {
        let json = r#"{
            "trade_id":"t",
            "order_id":"o",
            "market_ticker":"T",
            "side":"no",
            "action":"buy",
            "count_fp":"1",
            "yes_price_dollars":"0.01",
            "is_taker":true,
            "fee_cost":"0.00",
            "ts":0,
            "ts_ms":0,
            "post_position_fp":"1.00",
            "purchased_side":"yes",
            "outcome_side":"no",
            "book_side":"ask"
        }"#;
        let fill: WsFill = serde_json::from_str(json).unwrap();
        assert!(matches!(fill.side, Some(YesNo::No)));
        assert!(matches!(fill.action, Some(BuySell::Buy)));
        assert!(matches!(fill.outcome_side, YesNo::No));
        assert!(matches!(fill.book_side, BookSide::Ask));
    }
}
