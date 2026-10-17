//! Deterministic coverage for the 2026-10 spec refresh (OpenAPI 3.34.0 / AsyncAPI 2.0.0).

pub(crate) use cargo_husky as _;
use kalshi_fast::{
    ApiKeyType, CreateMarginOrderRequest, CreateRFQRequest, GenerateApiKeyRequest,
    GenerateApiKeyResponse, GetEventParams, GetMarginFundingRateEstimateResponse,
    GetMarginMarketOrderbooksResponse, GetMarginPremiumIndexResponse, GetMarginTradesParams,
    GetPositionsParams, MarginFundingInvalidPremiumHandling, MarginFundingLimitUnit, MarginMarket,
    MultivariateEventCollection, PositionSettlementStatus, WsOrderUpdateReason, WsQuoteAccepted,
    WsQuoteCreated, WsQuoteExecuted, WsUserFilter, WsUserOrder,
};

#[test]
fn positions_params_serialize_settlement_status() {
    let params = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::All),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&params).unwrap()["settlement_status"],
        "all"
    );
    let params = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::Settled),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&params).unwrap()["settlement_status"],
        "settled"
    );
    assert!(
        GetPositionsParams {
            subaccount: Some(63),
            ..Default::default()
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetPositionsParams {
            subaccount: Some(64),
            ..Default::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn get_event_params_serialize_exclude_markets() {
    let params = GetEventParams {
        exclude_markets: Some(true),
        ..Default::default()
    };
    let json = serde_json::to_value(&params).unwrap();
    assert_eq!(json["exclude_markets"], true);
    assert!(json.get("with_nested_markets").is_none());
}

#[test]
fn api_key_type_round_trips_on_generate() {
    let req = GenerateApiKeyRequest {
        name: "k".into(),
        key_type: Some(ApiKeyType::Ed25519),
        scopes: vec![],
    };
    assert_eq!(serde_json::to_value(&req).unwrap()["key_type"], "ed25519");
    let req = GenerateApiKeyRequest {
        name: "k".into(),
        key_type: None,
        scopes: vec![],
    };
    assert!(
        serde_json::to_value(&req)
            .unwrap()
            .get("key_type")
            .is_none()
    );

    let resp: GenerateApiKeyResponse =
        serde_json::from_str(r#"{"api_key_id":"id","key_type":"ed25519","private_key":"pem"}"#)
            .unwrap();
    assert_eq!(resp.key_type, Some(ApiKeyType::Ed25519));
    let resp: GenerateApiKeyResponse =
        serde_json::from_str(r#"{"api_key_id":"id","private_key":"pem"}"#).unwrap();
    assert_eq!(resp.key_type, None);
}

#[test]
fn create_rfq_serializes_obscure_creator_id() {
    let req = CreateRFQRequest {
        market_ticker: "M".into(),
        contracts: Some(1),
        contracts_fp: None,
        target_cost_centi_cents: None,
        target_cost_dollars: None,
        rest_remainder: false,
        obscure_creator_id: Some(true),
        replace_existing: None,
        subtrader_id: None,
        subaccount: None,
    };
    assert_eq!(
        serde_json::to_value(&req).unwrap()["obscure_creator_id"],
        true
    );
}

#[test]
fn multivariate_collection_parses_price_levels() {
    let json = r#"{
        "collection_ticker":"C","series_ticker":"S","title":"t","description":"d",
        "open_date":"2026-01-01T00:00:00Z","close_date":"2026-02-01T00:00:00Z",
        "associated_events":[],"associated_event_tickers":[],
        "is_ordered":false,"is_single_market_per_event":true,"is_all_yes":false,
        "size_min":2,"size_max":5,"functional_description":"f",
        "price_level_structure":"linear_cent",
        "price_ranges":[{"start":"0.0000","end":"1.0000","step":"0.0100"}]
    }"#;
    let c: MultivariateEventCollection = serde_json::from_str(json).unwrap();
    assert_eq!(c.price_level_structure.as_deref(), Some("linear_cent"));
    assert_eq!(c.price_ranges.len(), 1);
    // Payloads predating the fields still parse.
    let mut legacy: serde_json::Value = serde_json::from_str(json).unwrap();
    legacy
        .as_object_mut()
        .unwrap()
        .remove("price_level_structure");
    legacy.as_object_mut().unwrap().remove("price_ranges");
    let legacy = legacy.to_string();
    let c: MultivariateEventCollection = serde_json::from_str(&legacy).unwrap();
    assert!(c.price_level_structure.is_none() && c.price_ranges.is_empty());
}

#[test]
fn ws_quote_messages_parse_rfq_creator_and_subaccount() {
    let created: WsQuoteCreated = serde_json::from_str(
        r#"{"quote_id":"q","rfq_id":"r","quote_creator_id":"qc","rfq_creator_id":"e3b0c442",
            "market_ticker":"M","yes_bid_dollars":"0.5000","no_bid_dollars":"0.5000",
            "created_ts":"2026-10-01T00:00:00Z","subaccount":3}"#,
    )
    .unwrap();
    assert_eq!(created.rfq_creator_id.as_deref(), Some("e3b0c442"));
    assert_eq!(created.subaccount, Some(3));

    let accepted: WsQuoteAccepted = serde_json::from_str(
        r#"{"quote_id":"q","rfq_id":"r","quote_creator_id":"qc","rfq_creator_id":"rc",
            "market_ticker":"M","yes_bid_dollars":"0.5000","no_bid_dollars":"0.5000"}"#,
    )
    .unwrap();
    assert_eq!(accepted.rfq_creator_id.as_deref(), Some("rc"));
    assert_eq!(accepted.subaccount, None);

    let executed: WsQuoteExecuted = serde_json::from_str(
        r#"{"quote_id":"q","rfq_id":"r","quote_creator_id":"qc","rfq_creator_id":"rc",
            "order_id":"o","client_order_id":"c","market_ticker":"M",
            "executed_ts":"2026-10-01T00:00:00Z","subaccount":0}"#,
    )
    .unwrap();
    assert_eq!(executed.subaccount, Some(0));
}

#[test]
fn ws_user_order_parses_last_update_reason() {
    let base = |reason: &str| {
        format!(
            r#"{{"order_id":"o","user_id":"u","ticker":"M","exchange_index":0,"last_update_reason":"{reason}"}}"#
        )
    };
    let order: WsUserOrder = serde_json::from_str(&base("SettlementBoundsCancel")).unwrap();
    assert_eq!(
        order.last_update_reason,
        Some(WsOrderUpdateReason::SettlementBoundsCancel)
    );
    assert_eq!(order.exchange_index, Some(0));
    let order: WsUserOrder = serde_json::from_str(&base("SomethingNew")).unwrap();
    assert_eq!(order.last_update_reason, Some(WsOrderUpdateReason::Unknown));
}

#[test]
fn ws_user_filter_serializes_wire_values() {
    assert_eq!(
        serde_json::to_string(&WsUserFilter::Own).unwrap(),
        r#""self""#
    );
    assert_eq!(serde_json::to_string(&WsUserFilter::All).unwrap(), r#""""#);
}

#[test]
fn margin_market_parses_funding_and_version_fields() {
    let json = r#"{
        "ticker":"BTC-PERP","status":"active","title":"BTC","contract_size":"1",
        "underlying_multiplier":"1","tick_size":"0.01","fractional_trading_enabled":true,
        "exchange_index":0,"market_version":2,"bid_size_fp":"3.00","ask_size_fp":"0.00",
        "funding_deadband":0.01,"funding_cap":0.75,"funding_limit_unit":"percentage",
        "funding_invalid_premium_handling":"dampen"
    }"#;
    let m: MarginMarket = serde_json::from_str(json).unwrap();
    assert_eq!(m.market_version, Some(2));
    assert_eq!(m.bid_size_fp.as_deref(), Some("3.00"));
    assert_eq!(m.funding_cap, Some(0.75));
    assert_eq!(
        m.funding_limit_unit,
        Some(MarginFundingLimitUnit::Percentage)
    );
    assert_eq!(
        m.funding_invalid_premium_handling,
        Some(MarginFundingInvalidPremiumHandling::Dampen)
    );
    let unknown = json.replace("dampen", "interpolate");
    let m: MarginMarket = serde_json::from_str(&unknown).unwrap();
    assert_eq!(
        m.funding_invalid_premium_handling,
        Some(MarginFundingInvalidPremiumHandling::Unknown)
    );
}

#[test]
fn margin_batch_orderbooks_and_premium_index_parse() {
    let r: GetMarginMarketOrderbooksResponse = serde_json::from_str(
        r#"{"orderbooks":[{"ticker":"A","orderbook":{"bids":[["1.0","2.00"]],"asks":null}}]}"#,
    )
    .unwrap();
    assert_eq!(r.orderbooks[0].ticker, "A");
    assert_eq!(r.orderbooks[0].orderbook.bids.len(), 1);
    assert!(r.orderbooks[0].orderbook.asks.is_empty());

    let p: GetMarginPremiumIndexResponse = serde_json::from_str(
        r#"{"points":[{"second_ts":"2026-10-01T00:00:00Z","premium_index":"0.0004"}]}"#,
    )
    .unwrap();
    assert_eq!(p.points[0].premium_index, "0.0004");

    let e: GetMarginFundingRateEstimateResponse = serde_json::from_str(
        r#"{"next_funding_time":"t","premium_index":"0.001","premium_index_ts":"t2"}"#,
    )
    .unwrap();
    assert_eq!(e.premium_index.as_deref(), Some("0.001"));
}

#[test]
fn margin_trades_ticker_is_optional_and_order_carries_market_version() {
    let json = serde_json::to_value(GetMarginTradesParams::default()).unwrap();
    assert!(json.get("ticker").is_none());

    let req: CreateMarginOrderRequest = {
        use kalshi_fast::{BookSide, MarginSelfTradePreventionType, MarginTimeInForce};
        CreateMarginOrderRequest {
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
        }
    };
    assert_eq!(serde_json::to_value(&req).unwrap()["market_version"], 3);
}
