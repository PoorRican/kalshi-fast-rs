//! Deterministic coverage for the October 2026 spec refresh (OpenAPI 3.34.0).

use kalshi_fast::*;
use serde_json::json;

#[test]
fn margin_market_parses_new_fields() {
    let m: MarginMarket = serde_json::from_value(json!({
        "market_id": "7f3a2c1e-0000-4000-8000-000000000001",
        "ticker": "BTC-PERP",
        "status": "active",
        "title": "Bitcoin perp",
        "contract_size": "1",
        "underlying_multiplier": "1",
        "tick_size": "0.01",
        "fractional_trading_enabled": true,
        "exchange_index": 1,
        "market_version": 2,
        "funding_deadband": 0.0001,
        "funding_cap": 0.75,
        "funding_limit_unit": "percentage",
        "funding_invalid_premium_handling": "dampen",
        "bid": "100.00",
        "bid_size_fp": "12.00",
        "ask": "100.10",
        "ask_size_fp": "0.00"
    }))
    .unwrap();
    assert_eq!(m.market_version, 2);
    assert_eq!(m.funding_limit_unit, MarginFundingLimitUnit::Percentage);
    assert_eq!(
        m.funding_invalid_premium_handling,
        MarginFundingInvalidPremiumHandling::Dampen
    );
    assert_eq!(m.bid_size_fp.as_deref(), Some("12.00"));
    assert_eq!(m.ask_size_fp.as_deref(), Some("0.00"));
    assert!(m.market_id.is_some());
}

#[test]
fn margin_market_sizes_optional_without_stats() {
    let m: MarginMarket = serde_json::from_value(json!({
        "ticker": "BTC-PERP", "status": "inactive", "title": "t",
        "contract_size": "1", "underlying_multiplier": "1", "tick_size": "0.01",
        "fractional_trading_enabled": false, "exchange_index": 1, "market_version": 1,
        "funding_deadband": 0.0, "funding_cap": 1.0,
        "funding_limit_unit": "dollar", "funding_invalid_premium_handling": "exclude"
    }))
    .unwrap();
    assert!(m.bid_size_fp.is_none() && m.ask_size_fp.is_none());
}

#[test]
fn margin_batch_orderbooks_and_premium_index_parse() {
    let r: GetMarginMarketOrderbooksResponse = serde_json::from_value(json!({
        "orderbooks": [
            {"ticker": "A", "orderbook": {"bids": [["1.00", "2.00"]], "asks": null}},
            {"ticker": "B", "orderbook": {"bids": [], "asks": []}}
        ]
    }))
    .unwrap();
    assert_eq!(r.orderbooks.len(), 2);
    assert_eq!(r.orderbooks[0].ticker, "A");
    assert_eq!(r.orderbooks[0].orderbook.bids.len(), 1);
    assert!(r.orderbooks[0].orderbook.asks.is_empty());

    let p: GetMarginPremiumIndexResponse = serde_json::from_value(json!({
        "points": [{"second_ts": "2026-10-14T00:00:00Z", "premium_index": "0"}]
    }))
    .unwrap();
    assert_eq!(p.points[0].premium_index, "0");

    let e: GetMarginFundingRateEstimateResponse = serde_json::from_value(json!({
        "next_funding_time": "2026-10-14T01:00:00Z",
        "premium_index": "0.0004",
        "premium_index_ts": "2026-10-14T00:59:59Z"
    }))
    .unwrap();
    assert_eq!(e.premium_index.as_deref(), Some("0.0004"));
}

#[test]
fn margin_trades_ticker_optional_and_order_market_version_serialize() {
    let q = serde_json::to_value(GetMarginTradesParams::default()).unwrap();
    assert_eq!(q, json!({}));

    let order = CreateMarginOrderRequest {
        ticker: "BTC-PERP".into(),
        client_order_id: "c1".into(),
        side: BookSide::Bid,
        count: "1.00".into(),
        price: "100.00".into(),
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
    assert_eq!(serde_json::to_value(&order).unwrap()["market_version"], 3);
}

#[test]
fn market_settlement_bounds_parse_and_liquidity_dollars_is_gone() {
    let m: Market = serde_json::from_value(json!({
        "ticker": "T", "settlement_bounds_type": "floor",
        "settlement_floor_dollars": "0.2500", "exchange_index": 0
    }))
    .unwrap();
    assert_eq!(m.settlement_bounds_type.as_deref(), Some("floor"));
    assert_eq!(m.settlement_floor_dollars.as_deref(), Some("0.2500"));
    // The removed field no longer has a typed home; it falls into `extra`.
    let m: Market =
        serde_json::from_value(json!({"ticker": "T", "liquidity_dollars": "0.0000"})).unwrap();
    assert!(m.extra.contains_key("liquidity_dollars"));
}

#[test]
fn positions_fills_event_params_serialize() {
    let p = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::All),
        exchange_index: Some(1),
        ..Default::default()
    };
    let v = serde_json::to_value(&p).unwrap();
    assert_eq!(v["settlement_status"], "all");
    assert_eq!(v["exchange_index"], 1);

    let f = GetFillsParams {
        ticker: Some("A,B".into()),
        ..Default::default()
    };
    assert_eq!(serde_json::to_value(&f).unwrap()["ticker"], "A,B");

    let e = GetEventParams {
        exclude_markets: Some(true),
        ..Default::default()
    };
    assert_eq!(serde_json::to_value(&e).unwrap()["exclude_markets"], true);
}

#[test]
fn amend_v2_expiration_time_serializes_and_zero_clears() {
    let req = AmendOrderV2Request {
        ticker: "T".into(),
        side: BookSide::Bid,
        price: "0.5000".into(),
        count: "1.00".into(),
        client_order_id: None,
        updated_client_order_id: None,
        expiration_time: Some(0),
        exchange_index: None,
    };
    assert_eq!(serde_json::to_value(&req).unwrap()["expiration_time"], 0);
}

#[test]
fn rfq_obscure_creator_id_and_quote_fields() {
    let req = CreateRFQRequest {
        market_ticker: "T".into(),
        rest_remainder: true,
        obscure_creator_id: Some(true),
        ..Default::default()
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["obscure_creator_id"], true);
    assert!(v.get("target_cost_excludes_fees").is_none());

    let q = CreateQuoteRequest {
        rfq_id: "r".into(),
        yes_bid: "0.5".into(),
        no_bid: "0.5".into(),
        rest_remainder: false,
        post_only: Some(true),
        ..Default::default()
    };
    assert_eq!(serde_json::to_value(&q).unwrap()["post_only"], true);
}

#[test]
fn multivariate_collection_price_level_structure_parses() {
    let c: MultivariateEventCollection = serde_json::from_value(json!({
        "collection_ticker": "C", "series_ticker": "S", "title": "t", "description": "d",
        "open_date": "2026-01-01T00:00:00Z", "close_date": "2026-12-31T00:00:00Z",
        "associated_events": [], "associated_event_tickers": [],
        "is_ordered": false, "is_single_market_per_event": true, "is_all_yes": false,
        "size_min": 2, "size_max": 5, "functional_description": "f",
        "price_level_structure": "linear_cent",
        "price_ranges": [{"start": "0.0000", "end": "1.0000", "step": "0.0100"}]
    }))
    .unwrap();
    assert_eq!(c.price_level_structure.as_deref(), Some("linear_cent"));
    assert_eq!(c.price_ranges.unwrap().len(), 1);
}

#[test]
fn intra_exchange_transfer_and_api_key_type_serialize() {
    let t = IntraExchangeInstanceTransferRequest {
        client_transfer_id: Some("0b1c2d3e-0000-4000-8000-000000000002".into()),
        source: ExchangeInstance::EventContract,
        destination: ExchangeInstance::Margined,
        amount: 10_000,
        source_exchange_shard: None,
        destination_exchange_shard: None,
        source_subaccount: None,
        destination_subaccount: None,
    };
    let v = serde_json::to_value(&t).unwrap();
    assert_eq!(v["source"], "event_contract");
    assert_eq!(v["destination"], "margined");
    assert_eq!(
        v["client_transfer_id"],
        "0b1c2d3e-0000-4000-8000-000000000002"
    );

    let g = GenerateApiKeyRequest {
        name: "k".into(),
        scopes: vec![],
        key_type: Some(ApiKeyType::Ed25519),
    };
    assert_eq!(serde_json::to_value(&g).unwrap()["key_type"], "ed25519");
    let r: GenerateApiKeyResponse = serde_json::from_value(
        json!({"api_key_id": "id", "private_key": "pem", "key_type": "ed25519"}),
    )
    .unwrap();
    assert_eq!(r.key_type, Some(ApiKeyType::Ed25519));
}

#[test]
fn user_order_last_update_reason_parses_with_unknown_fallback() {
    let base = |reason: &str| {
        json!({
            "order_id": "o", "user_id": "u", "ticker": "T", "exchange_index": 2,
            "last_update_reason": reason
        })
    };
    let o: WsUserOrder = serde_json::from_value(base("SettlementBoundsCancel")).unwrap();
    assert_eq!(
        o.last_update_reason,
        Some(WsOrderUpdateReason::SettlementBoundsCancel)
    );
    assert_eq!(o.exchange_index, Some(2));
    let o: WsUserOrder = serde_json::from_value(base("SomethingNew")).unwrap();
    assert_eq!(o.last_update_reason, Some(WsOrderUpdateReason::Unknown));
}
