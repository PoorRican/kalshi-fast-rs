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
    pub side: YesNo,
    pub is_yes: bool,
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
    #[serde(deserialize_with = "super::deserialize_required_option")]
    pub created_time: Option<String>,
    pub last_update_time: Option<String>,
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
    pub side: YesNo,
    pub is_yes: bool,
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
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_option")]
    pub created_time: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub last_update_time: Option<Cow<'a, str>>,
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
