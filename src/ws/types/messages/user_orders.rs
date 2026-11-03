use crate::types::{BookSide, OrderStatus, SelfTradePreventionType, YesNo};
use serde::Deserialize;
use std::borrow::Cow;

/// Reason for the most recent order update.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum WsLastUpdateReason {
    Decrease,
    Amend,
    MarginCancel,
    SelfTradeCancel,
    ExpiryCancel,
    CloseCancel,
    HaltCancel,
    Trade,
    PostOnlyCrossCancel,
    ReduceOnlyCancel,
    SettlementBoundsCancel,
}

/// User order update payload (type: "user_order").
#[derive(Debug, Clone, Deserialize)]
pub struct WsUserOrder {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    pub exchange_index: i32,
    pub status: OrderStatus,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub is_yes: Option<bool>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    pub yes_price_dollars: String,
    pub fill_count_fp: String,
    pub remaining_count_fp: String,
    pub initial_count_fp: String,
    pub taker_fill_cost_dollars: String,
    pub maker_fill_cost_dollars: String,
    pub taker_fees_dollars: String,
    pub maker_fees_dollars: String,
    pub client_order_id: String,
    pub order_group_id: Option<String>,
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[deprecated(note = "use created_ts_ms instead")]
    #[serde(default)]
    pub created_time: Option<String>,
    #[deprecated(note = "use last_updated_ts_ms instead")]
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[deprecated(note = "use expiration_ts_ms instead")]
    #[serde(default)]
    pub expiration_time: Option<String>,
    pub expiration_ts_ms: Option<i64>,
    #[serde(deserialize_with = "super::deserialize_required_option")]
    pub created_ts_ms: Option<i64>,
    pub last_updated_ts_ms: Option<i64>,
    pub last_update_reason: Option<WsLastUpdateReason>,
    pub subaccount_number: Option<u32>,
}

/// User order update payload (type: "user_order").
#[derive(Debug, Clone, Deserialize)]
pub struct WsUserOrderRef<'a> {
    #[serde(borrow)]
    pub order_id: Cow<'a, str>,
    #[serde(borrow)]
    pub user_id: Cow<'a, str>,
    #[serde(borrow)]
    pub ticker: Cow<'a, str>,
    pub exchange_index: i32,
    pub status: OrderStatus,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    #[deprecated(note = "use outcome_side and book_side instead")]
    #[serde(default)]
    pub is_yes: Option<bool>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
    #[serde(borrow)]
    pub yes_price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub fill_count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub remaining_count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub initial_count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub taker_fill_cost_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub maker_fill_cost_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub taker_fees_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub maker_fees_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub client_order_id: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub order_group_id: Option<Cow<'a, str>>,
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[deprecated(note = "use created_ts_ms instead")]
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub created_time: Option<Cow<'a, str>>,
    #[deprecated(note = "use last_updated_ts_ms instead")]
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub last_update_time: Option<Cow<'a, str>>,
    #[deprecated(note = "use expiration_ts_ms instead")]
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub expiration_time: Option<Cow<'a, str>>,
    pub expiration_ts_ms: Option<i64>,
    #[serde(deserialize_with = "super::deserialize_required_option")]
    pub created_ts_ms: Option<i64>,
    pub last_updated_ts_ms: Option<i64>,
    pub last_update_reason: Option<WsLastUpdateReason>,
    pub subaccount_number: Option<u32>,
}

impl<'a> WsUserOrderRef<'a> {
    #[allow(deprecated)]
    pub fn into_owned(self) -> WsUserOrder {
        WsUserOrder {
            order_id: self.order_id.into_owned(),
            user_id: self.user_id.into_owned(),
            ticker: self.ticker.into_owned(),
            exchange_index: self.exchange_index,
            status: self.status,
            side: self.side,
            is_yes: self.is_yes,
            outcome_side: self.outcome_side,
            book_side: self.book_side,
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            fill_count_fp: self.fill_count_fp.into_owned(),
            remaining_count_fp: self.remaining_count_fp.into_owned(),
            initial_count_fp: self.initial_count_fp.into_owned(),
            taker_fill_cost_dollars: self.taker_fill_cost_dollars.into_owned(),
            maker_fill_cost_dollars: self.maker_fill_cost_dollars.into_owned(),
            taker_fees_dollars: self.taker_fees_dollars.into_owned(),
            maker_fees_dollars: self.maker_fees_dollars.into_owned(),
            client_order_id: self.client_order_id.into_owned(),
            order_group_id: self.order_group_id.map(Cow::into_owned),
            self_trade_prevention_type: self.self_trade_prevention_type,
            created_time: self.created_time.map(Cow::into_owned),
            last_update_time: self.last_update_time.map(Cow::into_owned),
            expiration_time: self.expiration_time.map(Cow::into_owned),
            expiration_ts_ms: self.expiration_ts_ms,
            created_ts_ms: self.created_ts_ms,
            last_updated_ts_ms: self.last_updated_ts_ms,
            last_update_reason: self.last_update_reason,
            subaccount_number: self.subaccount_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn deprecated_user_order_fields_accept_present_absent_and_null() {
        for (legacy_fields, present) in [
            (
                r#","side":"yes","is_yes":true,"created_time":"created","last_update_time":"updated","expiration_time":"expires""#,
                true,
            ),
            ("", false),
            (
                r#","side":null,"is_yes":null,"created_time":null,"last_update_time":null,"expiration_time":null"#,
                false,
            ),
        ] {
            let json = format!(
                r#"{{"order_id":"o","user_id":"u","ticker":"T","exchange_index":0,
                    "status":"resting","outcome_side":"yes","book_side":"bid",
                    "yes_price_dollars":"0.50","fill_count_fp":"1.00",
                    "remaining_count_fp":"2.00","initial_count_fp":"3.00",
                    "taker_fill_cost_dollars":"0.00","maker_fill_cost_dollars":"0.50",
                    "taker_fees_dollars":"0.00","maker_fees_dollars":"0.00",
                    "client_order_id":"c","created_ts_ms":1000,
                    "last_updated_ts_ms":2000,"expiration_ts_ms":3000{legacy_fields}}}"#
            );
            let owned: WsUserOrder = serde_json::from_slice(json.as_bytes()).unwrap();
            let borrowed: WsUserOrderRef<'_> = serde_json::from_slice(json.as_bytes()).unwrap();

            assert!(matches!(&borrowed.order_id, Cow::Borrowed("o")));
            if present {
                assert!(matches!(borrowed.side, Some(YesNo::Yes)));
                assert_eq!(borrowed.is_yes, Some(true));
                assert!(matches!(
                    &borrowed.created_time,
                    Some(Cow::Borrowed("created"))
                ));
                assert!(matches!(
                    &borrowed.last_update_time,
                    Some(Cow::Borrowed("updated"))
                ));
                assert!(matches!(
                    &borrowed.expiration_time,
                    Some(Cow::Borrowed("expires"))
                ));
            } else {
                assert!(borrowed.side.is_none());
                assert!(borrowed.is_yes.is_none());
                assert!(borrowed.created_time.is_none());
                assert!(borrowed.last_update_time.is_none());
                assert!(borrowed.expiration_time.is_none());
            }
            for order in [owned, borrowed.into_owned()] {
                if present {
                    assert!(matches!(order.side, Some(YesNo::Yes)));
                    assert_eq!(order.is_yes, Some(true));
                    assert_eq!(order.created_time.as_deref(), Some("created"));
                    assert_eq!(order.last_update_time.as_deref(), Some("updated"));
                    assert_eq!(order.expiration_time.as_deref(), Some("expires"));
                } else {
                    assert!(order.side.is_none());
                    assert!(order.is_yes.is_none());
                    assert!(order.created_time.is_none());
                    assert!(order.last_update_time.is_none());
                    assert!(order.expiration_time.is_none());
                }
                assert!(matches!(order.outcome_side, YesNo::Yes));
                assert!(matches!(order.book_side, BookSide::Bid));
                assert_eq!(order.created_ts_ms, Some(1000));
                assert_eq!(order.last_updated_ts_ms, Some(2000));
                assert_eq!(order.expiration_ts_ms, Some(3000));
            }
        }
    }
}
