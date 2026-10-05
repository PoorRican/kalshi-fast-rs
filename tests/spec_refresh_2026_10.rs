//! Parsing/serialization coverage for the 2026-09-28..2026-10-02 Kalshi changelog entries.

pub(crate) use cargo_husky as _;
use kalshi_fast::{
    AmendOrderV2Request, BookSide, CreateMarginOrderRequest, CreateRFQRequest, GetPositionsParams,
    MarginMarket, MarginSelfTradePreventionType, MarginTimeInForce, Market,
    PositionSettlementStatus, SettlementBoundsType, WsEnvelope, WsEnvelopeRef, WsOrderUpdateReason,
    WsUserOrder,
};

#[test]
fn market_parses_settlement_bounds() {
    let m: Market = serde_json::from_str(
        r#"{"ticker":"T","settlement_bounds_type":"floor","settlement_floor_dollars":"0.250000"}"#,
    )
    .unwrap();
    assert_eq!(m.settlement_bounds_type, Some(SettlementBoundsType::Floor));
    assert_eq!(m.settlement_floor_dollars.as_deref(), Some("0.250000"));
    assert!(!m.extra.contains_key("settlement_bounds_type"));

    let m: Market =
        serde_json::from_str(r#"{"ticker":"T","settlement_bounds_type":"default"}"#).unwrap();
    assert_eq!(
        m.settlement_bounds_type,
        Some(SettlementBoundsType::Default)
    );
    assert!(m.settlement_floor_dollars.is_none());

    let m: Market =
        serde_json::from_str(r#"{"ticker":"T","settlement_bounds_type":"ceiling"}"#).unwrap();
    assert_eq!(
        m.settlement_bounds_type,
        Some(SettlementBoundsType::Unknown)
    );

    let m: Market = serde_json::from_str(r#"{"ticker":"T"}"#).unwrap();
    assert!(m.settlement_bounds_type.is_none());
}

#[test]
fn positions_params_serialize_settlement_status() {
    for (status, wire) in [
        (PositionSettlementStatus::All, "all"),
        (PositionSettlementStatus::Unsettled, "unsettled"),
        (PositionSettlementStatus::Settled, "settled"),
    ] {
        let params = GetPositionsParams {
            settlement_status: Some(status),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["settlement_status"], wire);
    }
    let json = serde_json::to_value(GetPositionsParams::default()).unwrap();
    assert!(json.get("settlement_status").is_none());
}

#[test]
fn amend_order_v2_serializes_expiration_time() {
    let mut req = AmendOrderV2Request {
        ticker: "T".into(),
        side: BookSide::Bid,
        price: "0.5000".into(),
        count: "1.00".into(),
        client_order_id: None,
        updated_client_order_id: None,
        expiration_time: Some(0),
        exchange_index: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["expiration_time"], 0);
    req.expiration_time = None;
    assert!(
        serde_json::to_value(&req)
            .unwrap()
            .get("expiration_time")
            .is_none()
    );
}

#[test]
fn create_rfq_serializes_obscure_creator_id() {
    let req = CreateRFQRequest {
        market_ticker: "T".into(),
        contracts: None,
        contracts_fp: None,
        target_cost_centi_cents: None,
        target_cost_dollars: None,
        rest_remainder: false,
        replace_existing: None,
        subtrader_id: None,
        subaccount: None,
        obscure_creator_id: Some(true),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["obscure_creator_id"], true);
}

#[test]
fn margin_market_version_round_trips() {
    let m: MarginMarket = serde_json::from_str(
        r#"{"ticker":"T","status":"active","title":"t","contract_size":"1","underlying_multiplier":"1",
            "tick_size":"0.01","fractional_trading_enabled":false,"exchange_index":0,"market_version":3}"#,
    )
    .unwrap();
    assert_eq!(m.market_version, Some(3));

    let req = CreateMarginOrderRequest {
        ticker: "T".into(),
        client_order_id: "c".into(),
        side: BookSide::Bid,
        count: "1".into(),
        price: "1".into(),
        time_in_force: MarginTimeInForce::GoodTillCanceled,
        self_trade_prevention_type: MarginSelfTradePreventionType::TakerAtCross,
        expiration_time: None,
        post_only: None,
        cancel_order_on_pause: None,
        reduce_only: None,
        subaccount: None,
        order_group_id: None,
        market_version: Some(3),
    };
    assert_eq!(serde_json::to_value(&req).unwrap()["market_version"], 3);
}

#[test]
fn user_order_parses_settlement_bounds_cancel() {
    let o: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"SettlementBoundsCancel"}"#,
    )
    .unwrap();
    assert_eq!(
        o.last_update_reason,
        Some(WsOrderUpdateReason::SettlementBoundsCancel)
    );

    let o: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"SomethingNew"}"#,
    )
    .unwrap();
    assert_eq!(o.last_update_reason, Some(WsOrderUpdateReason::Unknown));
}

#[test]
fn envelope_captures_sending_ts_ms() {
    let json = r#"{"type":"unsubscribed","sid":1,"seq":2,"sending_ts_ms":1669149841234}"#;
    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    assert_eq!(env.sending_ts_ms, Some(1669149841234));
    let env: WsEnvelopeRef<'_> = serde_json::from_str(json).unwrap();
    assert_eq!(env.sending_ts_ms, Some(1669149841234));
    let env: WsEnvelope = serde_json::from_str(r#"{"type":"unsubscribed","sid":1}"#).unwrap();
    assert!(env.sending_ts_ms.is_none());
}
