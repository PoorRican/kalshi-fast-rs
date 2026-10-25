//! Serde tests for the Margin (perps) REST and WebSocket types.
//!
//! Fixtures are built from `perps_openapi.yaml` (v0.0.1) and `perps_asyncapi.yaml`
//! (v2.0.0): every `required` property is present unless a test says otherwise.

pub(crate) use cargo_husky as _;
use kalshi_fast::margin::ws::message::{CorporateActionType, MarginLastUpdateReason, MarginSide};
use kalshi_fast::{
    AmendMarginOrderRequest, BookSide, CreateMarginFcmSubtraderRequest, CreateMarginOrderRequest,
    DeleteFcmSubtraderRiskControlsParams, ExitTrigger, ExitTriggerKind, ExitTriggerLeg,
    ExitTriggerReason, ExitTriggerStatus, GetExitTriggersResponse,
    GetFcmSubtraderRiskControlsParams, GetFcmSubtraderRiskControlsResponse,
    GetMarginCorporateActionsParams, GetMarginCorporateActionsResponse,
    GetMarginExchangeStatusResponse, GetMarginFeeTierRatesResponse,
    GetMarginFundingRateEstimateResponse, GetMarginMarketOrderbooksResponse,
    GetMarginMarketsResponse, GetMarginOrderResponse, GetMarginPremiumIndexParams,
    GetMarginPremiumIndexResponse, GetMarginTradesParams, GetMarginTradesResponse, MarginChannel,
    MarginCorporateActionStatus, MarginCorporateActionType, MarginDataMessage, MarginFcmAssetClass,
    MarginFeeSchedule, MarginFundingInvalidPremiumHandling, MarginFundingLimitUnit,
    MarginMarketResponse, MarginMarketStatus, MarginSelfTradePreventionType, MarginSubscribeParams,
    MarginTimeInForce, SetCrossExitTriggerRequest, SetIsolatedExitTriggerRequest,
    UpdateExitTriggerRequest, UpdateFcmNotionalRiskLimitRequest,
    UpdateFcmSubtraderRiskControlsRequest,
};
use serde_json::{Value, json};

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// Margin market with every YAML `required` property and the optional new ones.
fn market_json() -> Value {
    json!({
        "market_id": "7b1f0c9e-2f53-4a58-9a3b-0d9f5f1e1c11",
        "ticker": "KXBTCPERP",
        "title": "Bitcoin perp",
        "market_version": 3,
        "exchange_index": 0,
        "contract_size": "0.001000",
        "underlying_multiplier": "1.000000",
        "tick_size": "0.1000",
        "status": "active",
        "fractional_trading_enabled": true,
        "schedule": null,
        "funding_deadband": 0.0005,
        "funding_cap": 0.75,
        "funding_limit_unit": "percentage",
        "funding_invalid_premium_handling": "dampen",
        "price": "64000.0000",
        "bid": "63999.0000",
        "bid_size_fp": "12.50",
        "ask": "64001.0000",
        "ask_size_fp": "0.00",
        "reference_price": {"price": "64010.0000", "ts_ms": 1_760_000_000_000_i64}
    })
}

/// Only the YAML `required` properties of MarginMarket.
fn market_required_only_json() -> Value {
    json!({
        "ticker": "XAUPERP",
        "status": "inactive",
        "title": "Gold perp",
        "contract_size": "1.000000",
        "underlying_multiplier": "1.000000",
        "tick_size": "0.0100",
        "fractional_trading_enabled": false,
        "schedule": {"is_open": false, "next_close_ts": null, "next_open_ts": 1_760_003_600_i64},
        "exchange_index": 0,
        "market_version": 1,
        "funding_deadband": 5.0,
        "funding_cap": 100.0,
        "funding_limit_unit": "dollar",
        "funding_invalid_premium_handling": "exclude"
    })
}

fn orderbook_json() -> Value {
    json!({"bids": [["63999.0000", "1.00"]], "asks": [["64001.0000", "2.50"], ["64002.0000", "1.00"]]})
}

// ---------------------------------------------------------------------------
// GET /margin/markets, /margin/markets/{ticker}
// ---------------------------------------------------------------------------

#[test]
fn market_parses_new_fields() {
    let resp: MarginMarketResponse =
        serde_json::from_value(json!({"market": market_json()})).unwrap();
    let m = resp.market;
    assert_eq!(
        m.market_id.as_deref(),
        Some("7b1f0c9e-2f53-4a58-9a3b-0d9f5f1e1c11")
    );
    assert_eq!(m.market_version, 3);
    assert_eq!(m.bid_size_fp.as_deref(), Some("12.50"));
    assert_eq!(m.ask_size_fp.as_deref(), Some("0.00"));
    assert_eq!(m.funding_deadband, 0.0005);
    assert_eq!(m.funding_cap, 0.75);
    assert_eq!(m.funding_limit_unit, MarginFundingLimitUnit::Percentage);
    assert_eq!(
        m.funding_invalid_premium_handling,
        MarginFundingInvalidPremiumHandling::Dampen
    );
    assert!(m.schedule.is_none(), "schedule is required-nullable");
    assert_eq!(m.reference_price.unwrap().ts_ms, 1_760_000_000_000);
}

#[test]
fn market_parses_with_only_required_fields() {
    let resp: GetMarginMarketsResponse =
        serde_json::from_value(json!({"markets": [market_required_only_json()]})).unwrap();
    let m = &resp.markets[0];
    assert!(matches!(m.status, MarginMarketStatus::Inactive));
    assert_eq!(m.market_version, 1);
    assert!(m.market_id.is_none());
    assert!(m.bid_size_fp.is_none() && m.ask_size_fp.is_none());
    assert_eq!(m.funding_limit_unit, MarginFundingLimitUnit::Dollar);
    assert_eq!(
        m.funding_invalid_premium_handling,
        MarginFundingInvalidPremiumHandling::Exclude
    );
    let schedule = m.schedule.as_ref().unwrap();
    assert!(!schedule.is_open);
    assert_eq!(schedule.next_close_ts, None);
    assert_eq!(schedule.next_open_ts, Some(1_760_003_600));
}

#[test]
fn market_rejects_missing_yaml_required_fields() {
    for field in [
        "market_version",
        "funding_deadband",
        "funding_cap",
        "funding_limit_unit",
        "funding_invalid_premium_handling",
    ] {
        let mut v = market_required_only_json();
        v.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<kalshi_fast::MarginMarket>(v).is_err(),
            "{field} is required by the spec"
        );
    }
}

#[test]
fn market_rejects_unknown_funding_enums() {
    let mut v = market_required_only_json();
    v["funding_limit_unit"] = json!("bps");
    assert!(serde_json::from_value::<kalshi_fast::MarginMarket>(v).is_err());
    let mut v = market_required_only_json();
    v["funding_invalid_premium_handling"] = json!("ignore");
    assert!(serde_json::from_value::<kalshi_fast::MarginMarket>(v).is_err());
}

// ---------------------------------------------------------------------------
// GET /margin/markets/orderbooks
// ---------------------------------------------------------------------------

#[test]
fn batch_orderbooks_preserve_request_order() {
    let resp: GetMarginMarketOrderbooksResponse = serde_json::from_value(json!({
        "orderbooks": [
            {"ticker": "B", "orderbook": orderbook_json()},
            {"ticker": "A", "orderbook": {"bids": [], "asks": []}}
        ]
    }))
    .unwrap();
    assert_eq!(resp.orderbooks[0].ticker, "B");
    assert_eq!(resp.orderbooks[0].orderbook.asks.len(), 2);
    assert_eq!(resp.orderbooks[1].ticker, "A");
    assert!(resp.orderbooks[1].orderbook.bids.is_empty());
}

#[test]
fn batch_orderbooks_tolerate_null_levels() {
    let resp: GetMarginMarketOrderbooksResponse = serde_json::from_value(json!({
        "orderbooks": [{"ticker": "A", "orderbook": {"bids": null, "asks": null}}]
    }))
    .unwrap();
    assert!(resp.orderbooks[0].orderbook.bids.is_empty());
}

// ---------------------------------------------------------------------------
// GET /margin/trades
// ---------------------------------------------------------------------------

#[test]
fn trades_params_ticker_is_optional() {
    let none = serde_json::to_value(GetMarginTradesParams::default()).unwrap();
    assert_eq!(none, json!({}));

    let with_ticker = serde_json::to_value(GetMarginTradesParams {
        ticker: Some("KXBTCPERP".into()),
        limit: Some(50),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(with_ticker, json!({"ticker": "KXBTCPERP", "limit": 50}));
}

#[test]
fn trades_response_parses() {
    let resp: GetMarginTradesResponse = serde_json::from_value(json!({
        "trades": [{
            "trade_id": "t1", "ticker": "KXBTCPERP", "count": "1.00", "price": "64000.0000",
            "created_time": "2026-10-20T12:00:00Z", "taker_side": "ask"
        }],
        "cursor": ""
    }))
    .unwrap();
    assert!(matches!(resp.trades[0].taker_side, BookSide::Ask));
}

// ---------------------------------------------------------------------------
// POST /margin/orders, amend
// ---------------------------------------------------------------------------

fn create_request() -> CreateMarginOrderRequest {
    CreateMarginOrderRequest {
        ticker: Some("KXBTCPERP".into()),
        market_id: None,
        client_order_id: "c1".into(),
        side: BookSide::Bid,
        count: "1.00".into(),
        price: "64000.0000".into(),
        time_in_force: MarginTimeInForce::GoodTillCanceled,
        self_trade_prevention_type: MarginSelfTradePreventionType::TakerAtCross,
        expiration_time: None,
        post_only: None,
        cancel_order_on_pause: None,
        reduce_only: None,
        subaccount: None,
        order_group_id: None,
        market_version: None,
    }
}

#[test]
fn create_order_serializes_required_fields_only_by_default() {
    let v = serde_json::to_value(create_request()).unwrap();
    assert_eq!(
        v,
        json!({
            "ticker": "KXBTCPERP", "client_order_id": "c1", "side": "bid", "count": "1.00",
            "price": "64000.0000", "time_in_force": "good_till_canceled",
            "self_trade_prevention_type": "taker_at_cross"
        })
    );
}

#[test]
fn create_order_serializes_market_version_and_market_id() {
    let req = CreateMarginOrderRequest {
        ticker: None,
        market_id: Some("7b1f0c9e-2f53-4a58-9a3b-0d9f5f1e1c11".into()),
        market_version: Some(3),
        reduce_only: Some(true),
        time_in_force: MarginTimeInForce::ImmediateOrCancel,
        ..create_request()
    };
    let v = serde_json::to_value(req).unwrap();
    assert!(v.get("ticker").is_none());
    assert_eq!(v["market_id"], "7b1f0c9e-2f53-4a58-9a3b-0d9f5f1e1c11");
    assert_eq!(v["market_version"], 3);
    assert_eq!(v["reduce_only"], true);
    assert_eq!(v["time_in_force"], "immediate_or_cancel");
}

#[test]
fn amend_order_serializes_expiration_time_including_zero() {
    let req = AmendMarginOrderRequest {
        ticker: Some("KXBTCPERP".into()),
        market_id: None,
        side: BookSide::Ask,
        price: "64000.0000".into(),
        count: "2.00".into(),
        client_order_id: None,
        updated_client_order_id: None,
        expiration_time: Some(0),
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["expiration_time"], 0, "0 clears expiry and must be sent");
    assert_eq!(v["side"], "ask");
    assert!(v.get("market_id").is_none());

    let no_expiry = AmendMarginOrderRequest {
        expiration_time: None,
        ticker: None,
        market_id: Some("m".into()),
        ..req
    };
    let v = serde_json::to_value(&no_expiry).unwrap();
    assert!(v.get("expiration_time").is_none());
    assert!(v.get("ticker").is_none());
    assert_eq!(v["market_id"], "m");
}

#[test]
fn order_parses_reduce_only_cancel_reason_and_system_fields() {
    let resp: GetMarginOrderResponse = serde_json::from_value(json!({
        "order": {
            "order_id": "o1", "user_id": "u1", "client_order_id": "c1", "ticker": "KXBTCPERP",
            "side": "bid", "price": "64000.0000", "fill_count": "0.00", "remaining_count": "0.00",
            "last_update_reason": "ReduceOnlyCancel",
            "expiration_time": null, "created_time": null, "last_update_time": null,
            "self_trade_prevention_type": null,
            "order_source": "system", "order_reason": "take_profit_stop_loss"
        }
    }))
    .unwrap();
    assert_eq!(resp.order.last_update_reason, "ReduceOnlyCancel");
    assert_eq!(resp.order.order_source.as_deref(), Some("system"));
    assert!(resp.order.self_trade_prevention_type.is_none());
}

// ---------------------------------------------------------------------------
// Funding: estimate + premium index
// ---------------------------------------------------------------------------

#[test]
fn funding_estimate_parses_premium_index() {
    let resp: GetMarginFundingRateEstimateResponse = serde_json::from_value(json!({
        "market_ticker": "KXBTCPERP", "computed_time": "2026-10-20T12:00:00Z",
        "funding_rate": 0.0001, "mark_price": "64000.0000",
        "next_funding_time": "2026-10-20T13:00:00Z",
        "premium_index": "-0.000123", "premium_index_ts": "2026-10-20T11:59:59Z"
    }))
    .unwrap();
    assert_eq!(resp.premium_index.as_deref(), Some("-0.000123"));
    assert_eq!(
        resp.premium_index_ts.as_deref(),
        Some("2026-10-20T11:59:59Z")
    );
}

#[test]
fn funding_estimate_parses_without_premium_index() {
    let resp: GetMarginFundingRateEstimateResponse =
        serde_json::from_value(json!({"next_funding_time": "2026-10-20T13:00:00Z"})).unwrap();
    assert!(resp.premium_index.is_none() && resp.premium_index_ts.is_none());
    assert!(resp.funding_rate.is_none());
}

#[test]
fn premium_index_response_and_params() {
    let resp: GetMarginPremiumIndexResponse = serde_json::from_value(json!({
        "points": [
            {"second_ts": "2026-10-20T12:00:00Z", "premium_index": "0.0004"},
            {"second_ts": "2026-10-20T12:00:01Z", "premium_index": "0"}
        ]
    }))
    .unwrap();
    assert_eq!(resp.points.len(), 2);
    assert_eq!(resp.points[1].premium_index, "0");

    let empty: GetMarginPremiumIndexResponse =
        serde_json::from_value(json!({"points": null})).unwrap();
    assert!(empty.points.is_empty());

    let q = serde_json::to_value(GetMarginPremiumIndexParams {
        ticker: "KXBTCPERP".into(),
        start_ts: 1_760_000_000,
        end_ts: 1_760_003_600,
    })
    .unwrap();
    assert_eq!(
        q,
        json!({"ticker": "KXBTCPERP", "start_ts": 1_760_000_000_i64, "end_ts": 1_760_003_600_i64})
    );
}

// ---------------------------------------------------------------------------
// GET /margin/corporate_actions
// ---------------------------------------------------------------------------

#[test]
fn corporate_actions_cover_every_terms_variant() {
    let resp: GetMarginCorporateActionsResponse = serde_json::from_value(json!({
        "corporate_actions": [
            {
                "id": "n1", "market_id": "m1", "market_ticker": "AAPLPERP", "type": "cash_dividend",
                "status": "scheduled", "updated_ts": "2026-10-20T00:00:00Z",
                "execution_time": "2026-10-23T13:30:00Z",
                "cash": {"cash_per_share": "0.2500", "cash_per_contract": "0.0025", "ex_date": "2026-10-23"},
                "split": null, "final_settlement": null, "ticker_change": null
            },
            {
                "id": "n2", "market_id": "m2", "market_ticker": "NVDAPERP", "type": "stock_split",
                "status": "applied", "updated_ts": "2026-10-20T00:00:00Z",
                "execution_time": "2026-10-21T13:30:00Z",
                "split": {"ratio_old": "1", "ratio_new": "4", "ex_date": "2026-10-21"}
            },
            {
                "id": "n3", "market_id": "m3", "market_ticker": "XYZPERP", "type": "final_settlement",
                "status": "lapsed", "updated_ts": "2026-10-20T00:00:00Z",
                "final_settlement": {"reason": "merger", "settlement_price_per_share": "10.0000",
                                     "settlement_price_per_contract": "0.1000"}
            },
            {
                "id": "n4", "market_id": "m4", "market_ticker": "NEWPERP", "type": "ticker_change",
                "status": "applied", "updated_ts": "2026-10-20T00:00:00Z",
                "ticker_change": {"old_ticker": "OLD", "new_ticker": "NEW"}
            },
            {
                "id": "n5", "market_id": "m5", "market_ticker": "SPINPERP", "type": "spin_off",
                "status": "cancelled", "updated_ts": "2026-10-20T00:00:00Z"
            },
            {
                "id": "n6", "market_id": "m6", "market_ticker": "SPECPERP", "type": "special_dividend",
                "status": "scheduled", "updated_ts": "2026-10-20T00:00:00Z",
                "execution_time": null, "cash": {}
            }
        ]
    }))
    .unwrap();
    let a = &resp.corporate_actions;
    assert_eq!(a[0].action_type, MarginCorporateActionType::CashDividend);
    assert_eq!(a[0].status, MarginCorporateActionStatus::Scheduled);
    let cash = a[0].cash.as_ref().unwrap();
    assert_eq!(cash.cash_per_contract.as_deref(), Some("0.0025"));
    assert_eq!(cash.ex_date.as_deref(), Some("2026-10-23"));
    assert!(a[0].split.is_none());

    assert_eq!(a[1].action_type, MarginCorporateActionType::StockSplit);
    assert_eq!(a[1].split.as_ref().unwrap().ratio_new.as_deref(), Some("4"));

    assert_eq!(a[2].status, MarginCorporateActionStatus::Lapsed);
    let fs = a[2].final_settlement.as_ref().unwrap();
    assert_eq!(fs.reason, "merger");
    assert_eq!(fs.settlement_price_per_contract.as_deref(), Some("0.1000"));

    let tc = a[3].ticker_change.as_ref().unwrap();
    assert_eq!(
        (tc.old_ticker.as_str(), tc.new_ticker.as_str()),
        ("OLD", "NEW")
    );

    assert_eq!(a[4].action_type, MarginCorporateActionType::SpinOff);
    assert_eq!(a[4].status, MarginCorporateActionStatus::Cancelled);
    assert!(a[4].execution_time.is_none());
    assert!(a[4].cash.is_none() && a[4].split.is_none());

    assert_eq!(a[5].action_type, MarginCorporateActionType::SpecialDividend);
    assert!(a[5].cash.as_ref().unwrap().cash_per_share.is_none());
}

#[test]
fn corporate_action_requires_yaml_required_fields() {
    let base = json!({
        "id": "n", "market_id": "m", "market_ticker": "T", "type": "ticker_change",
        "status": "applied", "updated_ts": "2026-10-20T00:00:00Z"
    });
    assert!(serde_json::from_value::<kalshi_fast::MarginCorporateAction>(base.clone()).is_ok());
    for field in [
        "id",
        "market_id",
        "market_ticker",
        "type",
        "status",
        "updated_ts",
    ] {
        let mut v = base.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<kalshi_fast::MarginCorporateAction>(v).is_err(),
            "{field} is required"
        );
    }
    // final_settlement.reason and ticker_change.{old,new}_ticker are required.
    assert!(
        serde_json::from_value::<kalshi_fast::MarginCorporateActionFinalSettlementTerms>(json!({}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<kalshi_fast::MarginCorporateActionTickerChangeTerms>(
            json!({"old_ticker": "A"})
        )
        .is_err()
    );
}

#[test]
fn corporate_actions_params_serialize_filters() {
    assert_eq!(
        serde_json::to_value(GetMarginCorporateActionsParams::default()).unwrap(),
        json!({})
    );
    assert_eq!(
        serde_json::to_value(GetMarginCorporateActionsParams {
            ticker: Some("AAPLPERP".into()),
            market_id: Some("m1".into()),
        })
        .unwrap(),
        json!({"ticker": "AAPLPERP", "market_id": "m1"})
    );
}

// ---------------------------------------------------------------------------
// GET /margin/exchange/status, fee tier rates
// ---------------------------------------------------------------------------

#[test]
fn exchange_status_parses_estimated_resume_time() {
    let s: GetMarginExchangeStatusResponse = serde_json::from_value(json!({
        "exchange_active": false, "trading_active": false,
        "exchange_estimated_resume_time": "2026-10-20T14:00:00Z"
    }))
    .unwrap();
    assert_eq!(
        s.exchange_estimated_resume_time.as_deref(),
        Some("2026-10-20T14:00:00Z")
    );

    let null: GetMarginExchangeStatusResponse = serde_json::from_value(json!({
        "exchange_active": true, "trading_active": true, "exchange_estimated_resume_time": null
    }))
    .unwrap();
    assert!(null.exchange_estimated_resume_time.is_none());

    let absent: GetMarginExchangeStatusResponse =
        serde_json::from_value(json!({"exchange_active": true, "trading_active": true})).unwrap();
    assert!(absent.exchange_estimated_resume_time.is_none());
}

#[test]
fn fee_tier_rates_parse() {
    let r: GetMarginFeeTierRatesResponse = serde_json::from_value(json!({
        "fee_tier_rates": [
            {"fee_schedule": "self_clearing_members", "tier": 0, "maker_fee_rate": 0.0005, "taker_fee_rate": 0.0012},
            {"fee_schedule": "kalshi_prime", "tier": 2, "maker_fee_rate": 0.0, "taker_fee_rate": 0.0008},
            {"fee_schedule": "fcm", "tier": 1, "maker_fee_rate": 0.0001, "taker_fee_rate": 0.0009}
        ]
    }))
    .unwrap();
    assert_eq!(
        r.fee_tier_rates[0].fee_schedule,
        MarginFeeSchedule::SelfClearingMembers
    );
    assert_eq!(r.fee_tier_rates[1].tier, 2);
    assert_eq!(r.fee_tier_rates[2].fee_schedule, MarginFeeSchedule::Fcm);
    assert_eq!(r.fee_tier_rates[0].taker_fee_rate, 0.0012);
}

// ---------------------------------------------------------------------------
// Exit triggers
// ---------------------------------------------------------------------------

#[test]
fn exit_trigger_bracket_and_trailing_parse() {
    let resp: GetExitTriggersResponse = serde_json::from_value(json!({
        "exit_triggers": [
            {
                "id": "x1", "ticker": "KXBTCPERP", "kind": "bracket", "status": "pending_on_entry",
                "anchor_order_id": "o1", "client_trigger_id": "ct1",
                "stop_loss_price": "60000.0000", "take_profit_price": "70000.0000",
                "count": "0.00", "filled_count": "0.00",
                "created_time": "2026-10-20T12:00:00Z", "updated_time": "2026-10-20T12:00:00Z"
            },
            {
                "id": "x2", "ticker": "KXBTCPERP", "kind": "trailing", "status": "filled",
                "status_reason": "position_closed", "triggered_leg": "stop_loss",
                "triggered_order_id": "o9", "trail_bps": 150, "watermark_price": "65000.0000",
                "effective_stop_price": "64025.0000", "count": "2.00", "filled_count": "2.00",
                "created_time": "2026-10-20T12:00:00Z", "updated_time": "2026-10-20T12:05:00Z"
            }
        ]
    }))
    .unwrap();
    let t = &resp.exit_triggers;
    assert_eq!(t[0].kind, ExitTriggerKind::Bracket);
    assert_eq!(t[0].status, ExitTriggerStatus::PendingOnEntry);
    assert_eq!(t[0].take_profit_price.as_deref(), Some("70000.0000"));
    assert!(t[0].status_reason.is_none() && t[0].trail_bps.is_none());
    assert_eq!(t[1].kind, ExitTriggerKind::Trailing);
    assert_eq!(t[1].status_reason, Some(ExitTriggerReason::PositionClosed));
    assert_eq!(t[1].triggered_leg, Some(ExitTriggerLeg::StopLoss));
    assert_eq!(t[1].trail_bps, Some(150));
    assert_eq!(t[1].effective_stop_price.as_deref(), Some("64025.0000"));
}

#[test]
fn exit_trigger_empty_list_and_required_fields() {
    let empty: GetExitTriggersResponse =
        serde_json::from_value(json!({"exit_triggers": null})).unwrap();
    assert!(empty.exit_triggers.is_empty());

    let base = json!({
        "id": "x", "ticker": "T", "kind": "bracket", "status": "active", "count": "1.00",
        "filled_count": "0.00", "created_time": "2026-10-20T12:00:00Z", "updated_time": "2026-10-20T12:00:00Z"
    });
    assert!(serde_json::from_value::<ExitTrigger>(base.clone()).is_ok());
    for field in [
        "id",
        "ticker",
        "kind",
        "status",
        "count",
        "filled_count",
        "created_time",
        "updated_time",
    ] {
        let mut v = base.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<ExitTrigger>(v).is_err(),
            "{field} is required"
        );
    }
    for status in [
        "pending_on_entry",
        "active",
        "filled",
        "failed",
        "canceled",
        "unknown",
    ] {
        let mut v = base.clone();
        v["status"] = json!(status);
        assert!(serde_json::from_value::<ExitTrigger>(v).is_ok(), "{status}");
    }
}

#[test]
fn exit_trigger_requests_serialize_only_set_fields() {
    let bracket = serde_json::to_value(SetIsolatedExitTriggerRequest {
        stop_loss_price: Some("60000.00".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(bracket, json!({"stop_loss_price": "60000.00"}));

    let trailing = serde_json::to_value(SetIsolatedExitTriggerRequest {
        kind: Some(ExitTriggerKind::Trailing),
        trail_bps: Some(250),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(trailing, json!({"kind": "trailing", "trail_bps": 250}));

    let cross = serde_json::to_value(SetCrossExitTriggerRequest {
        count: Some("2".into()),
        client_trigger_id: Some("my-key".into()),
        take_profit_price: Some("70000.00".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        cross,
        json!({"count": "2", "client_trigger_id": "my-key", "take_profit_price": "70000.00"})
    );

    let anchored = serde_json::to_value(SetCrossExitTriggerRequest {
        anchor_order_id: Some("o1".into()),
        kind: Some(ExitTriggerKind::Bracket),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        anchored,
        json!({"anchor_order_id": "o1", "kind": "bracket"})
    );

    let update = serde_json::to_value(UpdateExitTriggerRequest {
        stop_loss_price: None,
        take_profit_price: Some("71000.00".into()),
    })
    .unwrap();
    assert_eq!(update, json!({"take_profit_price": "71000.00"}));
}

// ---------------------------------------------------------------------------
// FCM
// ---------------------------------------------------------------------------

#[test]
fn fcm_risk_controls_response_parses() {
    let r: GetFcmSubtraderRiskControlsResponse = serde_json::from_value(json!({
        "risk_controls": [
            {"subtrader_id": "acct_a", "im_cap": "100.0000", "current_im": "42.0000"},
            {"subtrader_id": "acct_a", "market_ticker": "KXBTCPERP", "im_cap": "50.0000", "current_im": "0.0000"},
            {"subtrader_id": "acct_a", "asset_class": "Metals", "im_cap": "25.0000", "current_im": "1.0000"}
        ],
        "notional_limits": [
            {"subtrader_id": "acct_a", "notional_value_risk_limit": "5000.0000", "current_notional": "1250.0000"},
            {"subtrader_id": "acct_a", "market_ticker": "KXBTCPERP", "notional_value_risk_limit": "1000.0000", "current_notional": "0.0000"}
        ]
    }))
    .unwrap();
    assert!(r.risk_controls[0].market_ticker.is_none() && r.risk_controls[0].asset_class.is_none());
    assert_eq!(
        r.risk_controls[1].market_ticker.as_deref(),
        Some("KXBTCPERP")
    );
    assert_eq!(
        r.risk_controls[2].asset_class,
        Some(MarginFcmAssetClass::Metals)
    );
    assert!(r.notional_limits[0].market_ticker.is_none());
    assert_eq!(r.notional_limits[1].current_notional, "0.0000");
}

#[test]
fn fcm_requests_serialize() {
    assert_eq!(
        serde_json::to_value(CreateMarginFcmSubtraderRequest {
            subtrader_suffix: "desk1".into(),
            require_category_cap: false,
        })
        .unwrap(),
        json!({"subtrader_suffix": "desk1", "require_category_cap": false})
    );
    assert_eq!(
        serde_json::to_value(UpdateFcmSubtraderRiskControlsRequest {
            subtrader_id: "acct_a".into(),
            market_ticker: None,
            asset_class: Some(MarginFcmAssetClass::Equities),
            im_cap: "100.0000".into(),
        })
        .unwrap(),
        json!({"subtrader_id": "acct_a", "asset_class": "Equities", "im_cap": "100.0000"})
    );
    assert_eq!(
        serde_json::to_value(UpdateFcmNotionalRiskLimitRequest {
            notional_value_risk_limit: "5000.0000".into()
        })
        .unwrap(),
        json!({"notional_value_risk_limit": "5000.0000"})
    );
    assert_eq!(
        serde_json::to_value(GetFcmSubtraderRiskControlsParams::default()).unwrap(),
        json!({})
    );
    assert_eq!(
        serde_json::to_value(DeleteFcmSubtraderRiskControlsParams {
            subtrader_id: "acct_a".into(),
            market_ticker: Some("KXBTCPERP".into()),
            asset_class: None,
        })
        .unwrap(),
        json!({"subtrader_id": "acct_a", "market_ticker": "KXBTCPERP"})
    );
}

// ---------------------------------------------------------------------------
// WebSocket
// ---------------------------------------------------------------------------

fn ws(v: Value) -> MarginDataMessage {
    MarginDataMessage::from_bytes(&serde_json::to_vec(&v).unwrap()).expect("parse ok")
}

#[test]
fn ws_corporate_actions_channel_serializes() {
    assert_eq!(
        serde_json::to_value(MarginChannel::CorporateActions).unwrap(),
        "corporate_actions"
    );
    assert_eq!(
        <MarginChannel as kalshi_fast::ws::protocol::Channel>::as_str(
            &MarginChannel::CorporateActions
        ),
        "corporate_actions"
    );
    let params = MarginSubscribeParams {
        channels: vec![MarginChannel::CorporateActions],
        market_tickers: Some(vec!["AAPLPERP".into()]),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(params).unwrap(),
        json!({"channels": ["corporate_actions"], "market_tickers": ["AAPLPERP"]})
    );
}

#[test]
fn ws_corporate_actions_delta_split() {
    let msg = ws(json!({
        "type": "corporate_actions_delta", "sid": 7, "seq": 3, "sending_ts_ms": 1_760_000_000_123_i64,
        "msg": {"corporate_action": {
            "id": "n2", "market_id": "m2", "market_ticker": "NVDAPERP", "type": "stock_split",
            "ts_ms": 1_760_000_000_000_i64,
            "split": {"ratio_old": "1", "ratio_new": "4",
                      "pre_split_mark_price": "480.0000", "post_split_mark_price": "120.0000"}
        }}
    }));
    let MarginDataMessage::CorporateActionsDelta(e) = msg else {
        panic!("expected CorporateActionsDelta");
    };
    assert_eq!((e.sid, e.seq), (7, Some(3)));
    assert_eq!(e.sending_ts_ms, Some(1_760_000_000_123));
    let ca = e.msg.corporate_action;
    assert_eq!(ca.action_type, CorporateActionType::StockSplit);
    assert_eq!(ca.ts_ms, 1_760_000_000_000);
    let split = ca.split.unwrap();
    assert_eq!(split.ratio_new, "4");
    assert_eq!(split.post_split_mark_price, "120.0000");
    assert!(ca.final_settlement.is_none() && ca.ticker_change.is_none());
}

#[test]
fn ws_corporate_actions_delta_final_settlement_and_ticker_change() {
    let MarginDataMessage::CorporateActionsDelta(fs) = ws(json!({
        "type": "corporate_actions_delta", "sid": 7, "seq": 4,
        "msg": {"corporate_action": {
            "id": "n3", "market_id": "m3", "market_ticker": "XYZPERP", "type": "final_settlement",
            "ts_ms": 1, "final_settlement": {"settlement_price_per_contract": "0.1000"}
        }}
    })) else {
        panic!("expected CorporateActionsDelta");
    };
    assert_eq!(fs.sending_ts_ms, None);
    assert_eq!(
        fs.msg
            .corporate_action
            .final_settlement
            .unwrap()
            .settlement_price_per_contract,
        "0.1000"
    );

    let MarginDataMessage::CorporateActionsDelta(tc) = ws(json!({
        "type": "corporate_actions_delta", "sid": 7, "seq": 5,
        "msg": {"corporate_action": {
            "id": "n4", "market_id": "m4", "market_ticker": "NEW", "type": "ticker_change",
            "ts_ms": 2, "ticker_change": {"old_ticker": "OLD", "new_ticker": "NEW"}
        }}
    })) else {
        panic!("expected CorporateActionsDelta");
    };
    assert_eq!(
        tc.msg.corporate_action.action_type,
        CorporateActionType::TickerChange
    );
    assert_eq!(
        tc.msg.corporate_action.ticker_change.unwrap().old_ticker,
        "OLD"
    );
}

#[test]
fn ws_corporate_actions_delta_validates_required_fields() {
    // seq is required on corporate_actions_delta.
    let no_seq = br#"{"type":"corporate_actions_delta","sid":1,"msg":{"corporate_action":{"id":"n","market_id":"m","market_ticker":"T","type":"ticker_change","ts_ms":1}}}"#;
    assert!(MarginDataMessage::from_bytes(no_seq).is_err());
    // cash corporate actions are not streamed.
    let cash = br#"{"type":"corporate_actions_delta","sid":1,"seq":1,"msg":{"corporate_action":{"id":"n","market_id":"m","market_ticker":"T","type":"cash_dividend","ts_ms":1}}}"#;
    assert!(MarginDataMessage::from_bytes(cash).is_err());
    // split terms require all four properties.
    let bad_split = br#"{"type":"corporate_actions_delta","sid":1,"seq":1,"msg":{"corporate_action":{"id":"n","market_id":"m","market_ticker":"T","type":"stock_split","ts_ms":1,"split":{"ratio_old":"1","ratio_new":"2"}}}}"#;
    assert!(MarginDataMessage::from_bytes(bad_split).is_err());
}

#[test]
fn ws_sending_ts_ms_is_captured_on_data_messages() {
    let MarginDataMessage::Trade(t) = ws(json!({
        "type": "trade", "sid": 3, "seq": 42, "sending_ts_ms": 1_760_000_000_500_i64,
        "msg": {"trade_id": "t", "market_ticker": "KXBTCPERP", "price": "1.0000", "count": "1.00",
                "taker_side": "bid", "ts_ms": 1_760_000_000_400_i64}
    })) else {
        panic!("expected Trade");
    };
    assert_eq!(t.sending_ts_ms, Some(1_760_000_000_500));
    assert_eq!(t.msg.taker_side, MarginSide::Bid);

    // Unsequenced channel (ticker) too.
    let MarginDataMessage::Ticker(tk) = ws(json!({
        "type": "ticker", "sid": 2, "sending_ts_ms": 9,
        "msg": {
            "market_ticker": "XAUPERP", "price": "2400.0000", "bid": "2399.0000", "ask": "2401.0000",
            "bid_size_fp": "1.00", "ask_size_fp": "2.00", "last_trade_size_fp": "0.50",
            "volume": "10.00", "volume_notional_value_dollars": "24000.0000",
            "volume_24h": "5.00", "volume_24h_notional_value_dollars": "12000.0000",
            "open_interest": "7.00", "open_interest_notional_value_dollars": "16800.0000",
            "reference_price": {"price": "2400.5000", "ts_ms": 1_760_000_000_000_i64},
            "ts_ms": 1_760_000_000_000_i64
        }
    })) else {
        panic!("expected Ticker");
    };
    assert_eq!(tk.sending_ts_ms, Some(9));
    // Pyth-indexed perps carry reference_price on the ticker channel.
    assert_eq!(tk.msg.reference_price.unwrap().price, "2400.5000");
}

#[test]
fn ws_sending_ts_ms_absent_is_none() {
    let MarginDataMessage::OrderbookDelta(d) = ws(json!({
        "type": "orderbook_delta", "sid": 1, "seq": 1,
        "msg": {"market_ticker": "KXBTCPERP", "price": "1.0000", "delta": "-1.00", "side": "ask"}
    })) else {
        panic!("expected OrderbookDelta");
    };
    assert_eq!(d.sending_ts_ms, None);
}

#[test]
fn ws_user_order_reduce_only_cancel() {
    let MarginDataMessage::UserOrder(o) = ws(json!({
        "type": "user_order", "sid": 5, "sending_ts_ms": 11,
        "msg": {
            "order_id": "o", "user_id": "u", "client_order_id": "c", "ticker": "KXBTCPERP",
            "side": "ask", "price": "1.0000", "fill_count": "0.00", "remaining_count": "0.00",
            "created_ts_ms": 1_760_000_000_000_i64, "last_update_reason": "ReduceOnlyCancel",
            "order_source": "system"
        }
    })) else {
        panic!("expected UserOrder");
    };
    assert_eq!(
        o.msg.last_update_reason,
        Some(MarginLastUpdateReason::ReduceOnlyCancel)
    );
    assert_eq!(o.sending_ts_ms, Some(11));
}

#[test]
fn ws_every_yaml_last_update_reason_parses() {
    for reason in [
        "Decrease",
        "Amend",
        "MarginCancel",
        "SelfTradeCancel",
        "ExpiryCancel",
        "CloseCancel",
        "HaltCancel",
        "Trade",
        "PostOnlyCrossCancel",
        "ReduceOnlyCancel",
    ] {
        let m = ws(json!({
            "type": "orderbook_delta", "sid": 1, "seq": 1,
            "msg": {"market_ticker": "T", "price": "1.0000", "delta": "1.00", "side": "bid",
                    "last_update_reason": reason}
        }));
        assert!(
            matches!(m, MarginDataMessage::OrderbookDelta(_)),
            "{reason}"
        );
    }
}

#[test]
fn ws_unrecognised_future_type_is_unknown_not_error() {
    assert_eq!(
        ws(json!({"type": "corporate_actions_snapshot", "sid": 1, "seq": 1, "msg": {}})),
        MarginDataMessage::Unknown {
            msg_type: Some("corporate_actions_snapshot".into())
        }
    );
}
