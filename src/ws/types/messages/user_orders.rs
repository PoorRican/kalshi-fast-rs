use crate::types::{
    BookSide, FixedPointCount, FixedPointDollars, OrderStatus, SelfTradePreventionType, YesNo,
};
use serde::Deserialize;

/// User order update payload (type: "user_order").
#[derive(Debug, Clone, Deserialize)]
pub struct WsUserOrder {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    /// Identifier for the exchange shard where the order resides. Added
    /// 2026-08-27. Spec marks this required, but modeled as `Option` and
    /// defensively defaulted so payloads predating the flag still parse.
    #[serde(default)]
    pub exchange_index: Option<i64>,
    #[serde(default)]
    pub status: Option<OrderStatus>,
    /// Deprecated 2026-05-07; removed ~2026-05-28. Use `outcome_side`.
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default)]
    pub is_yes: Option<bool>,
    /// Normalized outcome side (yes | no). Added 2026-05-07.
    #[serde(default)]
    pub outcome_side: Option<YesNo>,
    /// Normalized book side (bid | ask). Added 2026-05-07.
    #[serde(default)]
    pub book_side: Option<BookSide>,
    #[serde(default)]
    pub yes_price_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub fill_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub remaining_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub initial_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub taker_fill_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub maker_fill_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub taker_fees_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub maker_fees_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub created_ts_ms: Option<i64>,
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[serde(default)]
    pub last_updated_ts_ms: Option<i64>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub expiration_ts_ms: Option<i64>,
    #[serde(default)]
    pub subaccount_number: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::types::{WsDataMessageV2, WsMessageRef, WsMessageV2};

    /// `exchange_index` was added 2026-08-27. Spec marks it required, but it is
    /// modeled defensively as `Option` so payloads predating the field still
    /// parse; the borrowed (envelope) path must round-trip the same value.
    #[test]
    fn exchange_index_parses_and_defaults_none() {
        let json = r#"{
            "type": "user_order",
            "sid": 22,
            "seq": 1,
            "msg": {
                "order_id": "ee587a1c-8b87-4dcf-b721-9f6f790619fa",
                "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                "ticker": "FED-23DEC-T3.00",
                "exchange_index": 2,
                "status": "resting"
            }
        }"#;

        let owned = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match owned {
            WsMessageV2::Data(WsDataMessageV2::UserOrder { msg, .. }) => {
                assert_eq!(msg.exchange_index, Some(2));
            }
            other => panic!("expected user_order data message, got {other:?}"),
        }

        let borrowed = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        let round_tripped = borrowed.into_owned().unwrap();
        match round_tripped {
            WsMessageV2::Data(WsDataMessageV2::UserOrder { msg, .. }) => {
                assert_eq!(msg.exchange_index, Some(2));
            }
            other => panic!("expected user_order data message, got {other:?}"),
        }

        let without_field = r#"{
            "order_id": "ee587a1c-8b87-4dcf-b721-9f6f790619fa",
            "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
            "ticker": "FED-23DEC-T3.00",
            "status": "resting"
        }"#;
        let missing: WsUserOrder = serde_json::from_str(without_field).unwrap();
        assert_eq!(missing.exchange_index, None);
    }
}
