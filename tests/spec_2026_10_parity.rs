//! Parsing/serialization coverage for the 2026-10 spec refresh (OpenAPI 3.34.0, AsyncAPI 2.0.0).

use kalshi_fast::margin::ws::message::{MarginCorporateActionType, MarginDataMessage};
use kalshi_fast::*;

#[test]
fn market_drops_liquidity_dollars_and_parses_settlement_bounds() {
    let m: Market = serde_json::from_str(
        r#"{"ticker":"T","settlement_bounds_type":"floor","settlement_floor_dollars":"0.2500"}"#,
    )
    .unwrap();
    assert_eq!(
        m.settlement_bounds_type,
        Some(kalshi_fast::SettlementBoundsType::Floor)
    );
    assert_eq!(m.settlement_floor_dollars.as_deref(), Some("0.2500"));

    let m: Market =
        serde_json::from_str(r#"{"ticker":"T","settlement_bounds_type":"weird"}"#).unwrap();
    assert_eq!(
        m.settlement_bounds_type,
        Some(kalshi_fast::SettlementBoundsType::Unknown)
    );
}

#[test]
fn orderbook_messages_carry_optional_clock() {
    let snap: WsOrderbookSnapshot =
        serde_json::from_str(r#"{"market_ticker":"T","market_id":"1","clock":4182057}"#).unwrap();
    assert_eq!(snap.clock, Some(4182057));
    let snap: WsOrderbookSnapshot =
        serde_json::from_str(r#"{"market_ticker":"T","market_id":"1"}"#).unwrap();
    assert_eq!(snap.clock, None);
    let delta: WsOrderbookDelta = serde_json::from_str(
        r#"{"market_ticker":"T","market_id":"1","clock":7,"price_dollars":"0.01","delta_fp":"1","side":"yes"}"#,
    )
    .unwrap();
    assert_eq!(delta.clock, Some(7));
}

#[test]
fn user_order_last_update_reason_and_envelope_sending_ts() {
    let order: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"SettlementBoundsCancel"}"#,
    )
    .unwrap();
    assert_eq!(
        order.last_update_reason,
        Some(WsLastUpdateReason::SettlementBoundsCancel)
    );
    let order: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"Brand New"}"#,
    )
    .unwrap();
    assert_eq!(order.last_update_reason, Some(WsLastUpdateReason::Unknown));

    let env: WsEnvelope =
        serde_json::from_str(r#"{"type":"ok","id":1,"sending_ts_ms":1700000000123}"#).unwrap();
    assert_eq!(env.sending_ts_ms, Some(1700000000123));
}

#[test]
fn rfq_request_serializes_new_flags_only_when_set() {
    let mut req = CreateRFQRequest {
        market_ticker: "T".into(),
        contracts: None,
        contracts_fp: None,
        target_cost_centi_cents: None,
        target_cost_dollars: None,
        rest_remainder: false,
        replace_existing: None,
        subtrader_id: None,
        subaccount: None,
        obscure_creator_id: None,
        target_cost_excludes_fees: None,
    };
    let v = serde_json::to_value(&req).unwrap();
    assert!(v.get("obscure_creator_id").is_none());
    req.obscure_creator_id = Some(true);
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["obscure_creator_id"], true);
}

#[test]
fn api_key_type_round_trips() {
    let req = GenerateApiKeyRequest {
        name: "k".into(),
        key_type: Some(ApiKeyType::Ed25519),
        scopes: vec![],
    };
    assert_eq!(serde_json::to_value(&req).unwrap()["key_type"], "ed25519");
    let resp: GenerateApiKeyResponse =
        serde_json::from_str(r#"{"api_key_id":"a","private_key":"p","key_type":"ed25519"}"#)
            .unwrap();
    assert_eq!(resp.key_type, Some(ApiKeyType::Ed25519));
    let resp: GenerateApiKeyResponse =
        serde_json::from_str(r#"{"api_key_id":"a","private_key":"p"}"#).unwrap();
    assert_eq!(resp.key_type, None);
}

#[test]
fn ed25519_pem_is_accepted() {
    // RFC 8410 example Ed25519 PKCS#8 private key.
    let pem = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIN4S2tL3zT1c0nMlwCqMhWdqQKbYy5eYZl+Bo0wLHqIv\n-----END PRIVATE KEY-----\n";
    let auth = KalshiAuth::from_pem_str("id", pem).unwrap();
    let headers = auth.build_headers("GET", "/trade-api/v2/markets").unwrap();
    // Ed25519 signatures are 64 bytes => 88 base64 chars.
    assert_eq!(headers.signature.len(), 88);
}

#[test]
fn margin_market_new_fields() {
    let m: MarginMarket = serde_json::from_str(
        r#"{"ticker":"T-PERP","status":"active","title":"t","contract_size":"1","underlying_multiplier":"1",
        "tick_size":"0.01","fractional_trading_enabled":true,"exchange_index":0,"market_version":2,
        "funding_deadband":0.0001,"funding_cap":0.01,"funding_limit_unit":"percentage",
        "funding_invalid_premium_handling":"exclude","bid_size_fp":"3.00","ask_size_fp":"0.00"}"#,
    )
    .unwrap();
    assert_eq!(m.market_version, Some(2));
    assert_eq!(
        m.funding_limit_unit,
        Some(MarginFundingLimitUnit::Percentage)
    );
    assert_eq!(
        m.funding_invalid_premium_handling,
        Some(MarginFundingInvalidPremiumHandling::Exclude)
    );
    assert_eq!(m.bid_size_fp.as_deref(), Some("3.00"));
}

#[test]
fn margin_rest_new_shapes() {
    let est: GetMarginFundingRateEstimateResponse = serde_json::from_str(
        r#"{"next_funding_time":"2026-10-22T17:00:00Z","premium_index":"0.0012","premium_index_ts":"2026-10-22T16:59:59Z"}"#,
    )
    .unwrap();
    assert_eq!(est.premium_index.as_deref(), Some("0.0012"));

    let pi: GetMarginPremiumIndexResponse = serde_json::from_str(
        r#"{"points":[{"second_ts":"2026-10-22T16:00:00Z","premium_index":"0"}]}"#,
    )
    .unwrap();
    assert_eq!(pi.points.len(), 1);

    let ca: GetMarginCorporateActionsResponse = serde_json::from_str(
        r#"{"corporate_actions":[
        {"id":"1","market_id":"m","market_ticker":"T","type":"stock_split","status":"scheduled",
         "updated_ts":"2026-10-22T00:00:00Z","execution_time":"2026-10-25T00:00:00Z",
         "split":{"ratio_old":"1","ratio_new":"2","ex_date":"2026-10-25"}},
        {"id":"2","market_id":"m","market_ticker":"T","type":"cash_dividend","status":"cancelled",
         "updated_ts":"2026-10-22T00:00:00Z"}]}"#,
    )
    .unwrap();
    assert_eq!(
        ca.corporate_actions[0].action_type,
        MarginCorporateActionKind::StockSplit
    );
    assert!(ca.corporate_actions[1].execution_time.is_none());

    let obs: GetMarginMarketOrderbooksResponse = serde_json::from_str(
        r#"{"orderbooks":[{"ticker":"A","orderbook":{"bids":[["1","2"]],"asks":[]}}]}"#,
    )
    .unwrap();
    assert_eq!(obs.orderbooks[0].ticker, "A");

    let req = reqwest::Client::new()
        .get("http://localhost/x")
        .query(&GetMarginMarketOrderbooksParams {
            tickers: vec!["A".into(), "B".into()],
        })
        .build()
        .unwrap();
    assert_eq!(req.url().query(), Some("tickers=A%2CB"));

    let order = serde_json::to_value(GetMarginTradesParams::default()).unwrap();
    assert!(order.get("ticker").is_none());
}

#[test]
fn margin_ws_corporate_actions_delta() {
    let raw = br#"{"type":"corporate_actions_delta","sid":3,"seq":1,"sending_ts_ms":1,
        "msg":{"corporate_action":{"id":"1","market_id":"m","market_ticker":"T","type":"stock_split","ts_ms":5,
        "split":{"ratio_old":"1","ratio_new":"2","pre_split_mark_price":"10.0000","post_split_mark_price":"5.0000"}}}}"#;
    match MarginDataMessage::from_bytes(raw).unwrap() {
        MarginDataMessage::CorporateActionsDelta(e) => {
            assert_eq!(e.sid, 3);
            assert_eq!(
                e.msg.corporate_action.action_type,
                MarginCorporateActionType::StockSplit
            );
            assert_eq!(e.msg.corporate_action.split.unwrap().ratio_new, "2");
        }
        other => panic!("unexpected {other:?}"),
    }
}
