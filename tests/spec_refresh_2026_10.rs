//! Parsing/serialization tests for the 2026-10 upstream refresh (changelog 2026-10-01..2026-10-29).

pub(crate) use cargo_husky as _;
use kalshi_fast::{
    AmendOrderV2Request, BookSide, CreateRFQRequest, GetEventParams, GetFillsParams,
    GetMarginCorporateActionsResponse, GetMarginExchangeStatusResponse,
    GetMarginFundingRateEstimateResponse, GetMarginMarketOrderbooksResponse,
    GetMarginPremiumIndexResponse, GetMarginTradesParams, GetOrdersParams, GetPositionsParams,
    MarginChannel, MarginDataMessage, MarginFundingInvalidPremiumHandling, MarginFundingLimitUnit,
    MarginMarket, PositionSettlementStatus, WsChannelV2, WsMessageV2, WsOrderUpdateReason,
    WsSubscriptionParamsV2, WsUserFilter,
};
use kalshi_fast::{WsDataMessageV2, WsEnvelope};
use serde_json::json;

// ---------------------------------------------------------------------------
// Predictions REST
// ---------------------------------------------------------------------------

#[test]
fn positions_params_serialize_settlement_status_and_exchange_index() {
    let params = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::All),
        exchange_index: Some(1),
        ..Default::default()
    };
    let v = serde_json::to_value(&params).unwrap();
    assert_eq!(v["settlement_status"], "all");
    assert_eq!(v["exchange_index"], 1);
    let v = serde_json::to_value(GetPositionsParams::default()).unwrap();
    assert!(v.get("settlement_status").is_none());
}

#[test]
fn fills_params_serialize_comma_separated_tickers() {
    let params = GetFillsParams {
        ticker: Some("MKT-A,MKT-B".to_string()),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&params).unwrap()["ticker"],
        "MKT-A,MKT-B"
    );
}

#[test]
fn orders_params_allow_up_to_100_event_tickers() {
    let params = GetOrdersParams {
        event_ticker: Some((0..100).map(|i| format!("E{i}")).collect()),
        ..Default::default()
    };
    assert!(params.validate().is_ok());
}

#[test]
fn event_params_serialize_exclude_markets() {
    let params = GetEventParams {
        exclude_markets: Some(true),
        ..Default::default()
    };
    let v = serde_json::to_value(&params).unwrap();
    assert_eq!(v["exclude_markets"], true);
    assert!(v.get("with_nested_markets").is_none());
}

#[test]
fn amend_order_v2_serializes_expiration_time_only_when_set() {
    let mut req = AmendOrderV2Request {
        ticker: "MKT".into(),
        side: BookSide::Bid,
        price: "0.5000".into(),
        count: "10.00".into(),
        client_order_id: None,
        updated_client_order_id: None,
        expiration_time: None,
        exchange_index: None,
    };
    assert!(
        serde_json::to_value(&req)
            .unwrap()
            .get("expiration_time")
            .is_none()
    );
    req.expiration_time = Some(0);
    assert_eq!(serde_json::to_value(&req).unwrap()["expiration_time"], 0);
}

#[test]
fn create_rfq_serializes_obscure_creator_id() {
    let req = CreateRFQRequest {
        market_ticker: "MKT".into(),
        rest_remainder: true,
        obscure_creator_id: Some(true),
        target_cost_excludes_fees: Some(false),
        ..Default::default()
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["obscure_creator_id"], true);
    assert_eq!(v["target_cost_excludes_fees"], false);
    assert!(v.get("target_cost_centi_cents").is_none());
}

// ---------------------------------------------------------------------------
// Predictions WebSocket
// ---------------------------------------------------------------------------

#[test]
fn orderbook_messages_carry_exchange_clock() {
    let snap = br#"{"type":"orderbook_snapshot","sid":2,"seq":2,"sending_ts_ms":1669149841234,"msg":{"market_ticker":"M","market_id":"id","clock":4182057,"yes_dollars_fp":[["0.0800","300.00"]],"no_dollars_fp":[]}}"#;
    let delta = br#"{"type":"orderbook_delta","sid":2,"seq":3,"sending_ts_ms":1669149841123,"msg":{"market_ticker":"M","market_id":"id","clock":4182061,"price_dollars":"0.9600","delta_fp":"-54.00","side":"yes","ts_ms":1669149841000}}"#;
    match WsMessageV2::from_bytes(snap).unwrap() {
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { msg, .. }) => {
            assert_eq!(msg.clock, Some(4182057))
        }
        other => panic!("unexpected {other:?}"),
    }
    match WsMessageV2::from_bytes(delta).unwrap() {
        WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { msg, .. }) => {
            assert_eq!(msg.clock, Some(4182061))
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn envelope_exposes_sending_ts_ms() {
    let env: WsEnvelope = serde_json::from_slice(
        br#"{"type":"ok","id":1,"sid":2,"seq":3,"sending_ts_ms":1669149841234}"#,
    )
    .unwrap();
    assert_eq!(env.sending_ts_ms, Some(1669149841234));
    let env: WsEnvelope = serde_json::from_slice(br#"{"type":"ok","id":1}"#).unwrap();
    assert_eq!(env.sending_ts_ms, None);
}

#[test]
fn user_order_last_update_reason_parses_new_and_unknown_values() {
    let frame = |reason: &str| {
        format!(
            r#"{{"type":"user_order","sid":1,"seq":1,"msg":{{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"{reason}"}}}}"#
        )
    };
    for (raw, expected) in [
        (
            "SettlementBoundsCancel",
            WsOrderUpdateReason::SettlementBoundsCancel,
        ),
        ("ReduceOnlyCancel", WsOrderUpdateReason::ReduceOnlyCancel),
        ("SomethingNew", WsOrderUpdateReason::Unknown),
    ] {
        match WsMessageV2::from_bytes(frame(raw).as_bytes()).unwrap() {
            WsMessageV2::Data(WsDataMessageV2::UserOrder { msg, .. }) => {
                assert_eq!(msg.last_update_reason, Some(expected))
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}

#[test]
fn communications_user_filter_serializes_and_is_communications_only() {
    let params = WsSubscriptionParamsV2 {
        channels: vec![WsChannelV2::Communications],
        user_filter: Some(WsUserFilter::Own),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&params).unwrap()["user_filter"],
        "self"
    );
    let all = serde_json::to_value(WsUserFilter::All).unwrap();
    assert_eq!(all, "");
}

// ---------------------------------------------------------------------------
// Margin REST
// ---------------------------------------------------------------------------

#[test]
fn margin_market_parses_version_sizes_and_funding_config() {
    let market: MarginMarket = serde_json::from_value(json!({
        "ticker": "KXBTCPERP",
        "title": "BTC perp",
        "market_version": 2,
        "exchange_index": 0,
        "contract_size": "1",
        "underlying_multiplier": "1",
        "tick_size": "0.0100",
        "status": "active",
        "fractional_trading_enabled": true,
        "bid": "99.00",
        "bid_size_fp": "3.00",
        "ask": "101.00",
        "ask_size_fp": "0.00",
        "schedule": {"is_open": true},
        "funding_deadband": 0.0005,
        "funding_cap": 0.01,
        "funding_limit_unit": "percentage",
        "funding_invalid_premium_handling": "dampen"
    }))
    .unwrap();
    assert_eq!(market.market_version, 2);
    assert_eq!(market.bid_size_fp.as_deref(), Some("3.00"));
    assert_eq!(market.ask_size_fp.as_deref(), Some("0.00"));
    assert_eq!(
        market.funding_limit_unit,
        MarginFundingLimitUnit::Percentage
    );
    assert_eq!(
        market.funding_invalid_premium_handling,
        MarginFundingInvalidPremiumHandling::Dampen
    );
}

#[test]
fn margin_trades_ticker_is_optional() {
    let v = serde_json::to_value(GetMarginTradesParams::default()).unwrap();
    assert!(v.get("ticker").is_none());
}

#[test]
fn margin_exchange_status_parses_resume_time() {
    let r: GetMarginExchangeStatusResponse = serde_json::from_value(json!({
        "exchange_active": false,
        "trading_active": false,
        "exchange_estimated_resume_time": "2026-10-23T14:00:00Z"
    }))
    .unwrap();
    assert_eq!(
        r.exchange_estimated_resume_time.as_deref(),
        Some("2026-10-23T14:00:00Z")
    );
}

#[test]
fn margin_funding_estimate_parses_premium_index() {
    let r: GetMarginFundingRateEstimateResponse = serde_json::from_value(json!({
        "next_funding_time": "2026-10-23T16:00:00Z",
        "premium_index": "0.0012",
        "premium_index_ts": "2026-10-23T15:59:59Z"
    }))
    .unwrap();
    assert_eq!(r.premium_index.as_deref(), Some("0.0012"));
    assert_eq!(r.premium_index_ts.as_deref(), Some("2026-10-23T15:59:59Z"));
}

#[test]
fn margin_batch_orderbooks_premium_index_and_corporate_actions_parse() {
    let books: GetMarginMarketOrderbooksResponse = serde_json::from_value(json!({
        "orderbooks": [
            {"ticker": "A", "orderbook": {"bids": [["99.00", "1.00"]], "asks": []}},
            {"ticker": "B", "orderbook": {"bids": [], "asks": [["101.00", "2.00"]]}}
        ]
    }))
    .unwrap();
    assert_eq!(books.orderbooks[1].ticker, "B");
    assert_eq!(books.orderbooks[0].orderbook.bids[0][0], "99.00");

    let prem: GetMarginPremiumIndexResponse = serde_json::from_value(json!({
        "points": [{"second_ts": "2026-10-23T15:59:58Z", "premium_index": "0"}]
    }))
    .unwrap();
    assert_eq!(prem.points.len(), 1);

    let ca: GetMarginCorporateActionsResponse = serde_json::from_value(json!({
        "corporate_actions": [
            {"id": "n1", "market_id": "m", "market_ticker": "AAPL", "type": "stock_split",
             "status": "scheduled", "updated_ts": "2026-10-22T00:00:00Z",
             "execution_time": "2026-10-30T13:30:00Z",
             "split": {"ratio_old": "1", "ratio_new": "4", "ex_date": "2026-10-30"}},
            {"id": "n2", "market_id": "m", "market_ticker": "AAPL", "type": "cash_dividend",
             "status": "cancelled", "updated_ts": "2026-10-22T00:00:00Z"}
        ]
    }))
    .unwrap();
    assert_eq!(
        ca.corporate_actions[0]
            .split
            .as_ref()
            .unwrap()
            .ratio_new
            .as_deref(),
        Some("4")
    );
    assert!(ca.corporate_actions[1].execution_time.is_none());
}

// ---------------------------------------------------------------------------
// Margin WebSocket
// ---------------------------------------------------------------------------

#[test]
fn margin_corporate_actions_delta_parses_with_sending_ts() {
    let frame = br#"{"type":"corporate_actions_delta","sid":4,"seq":1,"sending_ts_ms":1733047200123,"msg":{"corporate_action":{"id":"n1","market_id":"m","market_ticker":"AAPL","type":"stock_split","ts_ms":1733047200000,"split":{"ratio_old":"1","ratio_new":"4","pre_split_mark_price":"200.00","post_split_mark_price":"50.00"}}}}"#;
    match MarginDataMessage::from_bytes(frame).unwrap() {
        MarginDataMessage::CorporateActionsDelta(env) => {
            assert_eq!(env.sid, 4);
            assert_eq!(env.seq, Some(1));
            assert_eq!(env.sending_ts_ms, Some(1733047200123));
            let split = env.msg.corporate_action.split.unwrap();
            assert_eq!(split.post_split_mark_price, "50.00");
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn margin_corporate_actions_channel_is_public_and_named() {
    use kalshi_fast::Channel;
    assert_eq!(
        MarginChannel::CorporateActions.as_str(),
        "corporate_actions"
    );
    assert!(!MarginChannel::CorporateActions.is_private());
}
