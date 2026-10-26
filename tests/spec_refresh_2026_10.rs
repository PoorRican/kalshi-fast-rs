//! Deterministic parsing/serialization checks for the 2026-10 spec refresh
//! (OpenAPI 3.35.0, AsyncAPI 2.0.0, Perps OpenAPI 0.0.1, Perps AsyncAPI 2.0.0).

pub(crate) use cargo_husky as _;
use kalshi_fast::{
    AmendOrderV2Request, BookSide, CreateMarginOrderRequest, ExchangeInstance, GetEventParams,
    GetExchangeStatusResponse, GetIntraExchangeInstanceTransferResponse,
    GetMarginCorporateActionsResponse, GetMarginMarketOrderbooksResponse, GetMarginTradesParams,
    GetOrdersParams, GetSubaccountBalancesResponse, IntraExchangeInstanceTransferRequest,
    IntraExchangeInstanceTransferStatus, MarginChannel, MarginCorporateActionStatus,
    MarginCorporateActionType, MarginFundingInvalidPremiumHandling, MarginFundingLimitUnit,
    MarginMarket, MarginSelfTradePreventionType, MarginTimeInForce, Market,
    MultivariateEventCollection, SettlementBoundsType, WsCommunications, WsCommunicationsRef,
    WsDataMessageRef, WsDataMessageV2, WsMessageRef, WsMessageV2,
};

fn qs<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap()
}

#[test]
fn margin_market_parses_new_required_fields() {
    let m: MarginMarket = serde_json::from_str(
        r#"{"ticker":"KXAAPLPERP","status":"inactive","title":"AAPL","contract_size":"1.000000",
        "underlying_multiplier":"1","tick_size":"0.01","fractional_trading_enabled":true,
        "schedule":null,"exchange_index":1,"market_version":3,"funding_deadband":0.5,
        "funding_cap":2.0,"funding_limit_unit":"percentage",
        "funding_invalid_premium_handling":"dampen","halt_sources":["initial_setup"],
        "bid_size_fp":"12.00","ask_size_fp":"0.00"}"#,
    )
    .unwrap();
    assert_eq!(m.market_version, 3);
    assert_eq!(m.funding_limit_unit, MarginFundingLimitUnit::Percentage);
    assert_eq!(
        m.funding_invalid_premium_handling,
        MarginFundingInvalidPremiumHandling::Dampen
    );
    assert_eq!(m.halt_sources, ["initial_setup"]);
    assert_eq!(m.bid_size_fp.as_deref(), Some("12.00"));
}

#[test]
fn margin_market_tolerates_null_halt_sources_and_unknown_enums() {
    let m: MarginMarket = serde_json::from_str(
        r#"{"ticker":"X","status":"active","title":"t","contract_size":"1","underlying_multiplier":"1",
        "tick_size":"0.01","fractional_trading_enabled":false,"exchange_index":0,"market_version":1,
        "funding_deadband":0,"funding_cap":0,"funding_limit_unit":"bps",
        "funding_invalid_premium_handling":"other","halt_sources":null}"#,
    )
    .unwrap();
    assert!(m.halt_sources.is_empty());
    assert_eq!(m.funding_limit_unit, MarginFundingLimitUnit::Unknown);
}

#[test]
fn margin_trades_ticker_is_optional_and_create_order_market_version_serializes() {
    assert_eq!(qs(&GetMarginTradesParams::default()), "{}");
    assert_eq!(
        qs(&GetMarginTradesParams {
            ticker: Some("T".into()),
            ..Default::default()
        }),
        r#"{"ticker":"T"}"#
    );

    let mut req = CreateMarginOrderRequest {
        ticker: "T".into(),
        client_order_id: "c".into(),
        side: BookSide::Bid,
        count: "1".into(),
        price: "1".into(),
        time_in_force: MarginTimeInForce::GoodTillCanceled,
        self_trade_prevention_type: MarginSelfTradePreventionType::Maker,
        expiration_time: None,
        post_only: None,
        cancel_order_on_pause: None,
        reduce_only: None,
        subaccount: None,
        order_group_id: None,
        market_version: None,
    };
    assert!(
        serde_json::to_value(&req)
            .unwrap()
            .get("market_version")
            .is_none()
    );
    req.market_version = Some(2);
    assert_eq!(serde_json::to_value(&req).unwrap()["market_version"], 2);
}

#[test]
fn margin_batch_orderbooks_and_corporate_actions_parse() {
    let r: GetMarginMarketOrderbooksResponse = serde_json::from_str(
        r#"{"orderbooks":[{"ticker":"A","orderbook":{"bids":[["1.00","2.00"]],"asks":[]}}]}"#,
    )
    .unwrap();
    assert_eq!(r.orderbooks.len(), 1);
    assert_eq!(r.orderbooks[0].ticker, "A");

    let r: GetMarginCorporateActionsResponse = serde_json::from_str(
        r#"{"corporate_actions":[
        {"id":"1","market_id":"m","market_ticker":"A","type":"stock_split","status":"scheduled",
         "updated_ts":"2026-10-22T00:00:00Z","execution_time":"2026-10-30T00:00:00Z",
         "split":{"ratio_old":"1","ratio_new":"4","ex_date":"2026-10-30"}},
        {"id":"2","market_id":"m","market_ticker":"A","type":"final_settlement","status":"cancelled",
         "updated_ts":"2026-10-22T00:00:00Z"},
        {"id":"3","market_id":"m","market_ticker":"A","type":"ticker_change","status":"applied",
         "updated_ts":"2026-10-22T00:00:00Z",
         "ticker_change":{"old_ticker":"O","new_ticker":"N"}}]}"#,
    )
    .unwrap();
    assert_eq!(
        r.corporate_actions[0].action_type,
        MarginCorporateActionType::StockSplit
    );
    assert_eq!(
        r.corporate_actions[0]
            .split
            .as_ref()
            .unwrap()
            .ratio_new
            .as_deref(),
        Some("4")
    );
    assert_eq!(
        r.corporate_actions[1].status,
        MarginCorporateActionStatus::Cancelled
    );
    assert!(r.corporate_actions[1].execution_time.is_none());
    assert_eq!(
        r.corporate_actions[2]
            .ticker_change
            .as_ref()
            .unwrap()
            .new_ticker,
        "N"
    );
}

#[test]
fn margin_channels_serialize() {
    assert_eq!(
        serde_json::to_string(&MarginChannel::CorporateActions).unwrap(),
        "\"corporate_actions\""
    );
    assert_eq!(
        serde_json::to_string(&MarginChannel::MarketLifecycleV2).unwrap(),
        "\"market_lifecycle_v2\""
    );
}

#[test]
fn predictions_market_settlement_bounds_parse() {
    let m: Market = serde_json::from_str(
        r#"{"ticker":"T","settlement_bounds_type":"floor","settlement_floor_dollars":"0.2500"}"#,
    )
    .unwrap();
    assert_eq!(m.settlement_bounds_type, Some(SettlementBoundsType::Floor));
    assert_eq!(m.settlement_floor_dollars.as_deref(), Some("0.2500"));
    let m: Market = serde_json::from_str(r#"{"ticker":"T"}"#).unwrap();
    assert!(m.settlement_bounds_type.is_none());
}

#[test]
fn multivariate_collection_requires_price_structure() {
    let c: MultivariateEventCollection = serde_json::from_str(
        r#"{"collection_ticker":"C","series_ticker":"S","title":"t","description":"d",
        "open_date":"a","close_date":"b","associated_events":[],"associated_event_tickers":[],
        "is_ordered":false,"is_single_market_per_event":false,"is_all_yes":false,"size_min":1,
        "size_max":2,"functional_description":"f","price_level_structure":"linear_cent",
        "price_ranges":[{"start":"0.00","end":"1.00","step":"0.01"}]}"#,
    )
    .unwrap();
    assert_eq!(c.price_level_structure, "linear_cent");
    assert_eq!(c.price_ranges.len(), 1);
}

#[test]
fn event_params_exclude_markets_and_orders_event_ticker_limit() {
    assert_eq!(
        qs(&GetEventParams {
            exclude_markets: Some(true),
            ..Default::default()
        }),
        r#"{"exclude_markets":true}"#
    );

    let tickers: Vec<String> = (0..100).map(|i| format!("E{i}")).collect();
    let ok = GetOrdersParams {
        event_ticker: Some(tickers.clone()),
        ..Default::default()
    };
    assert!(ok.validate().is_ok());
    let mut too_many = tickers;
    too_many.push("EXTRA".into());
    let bad = GetOrdersParams {
        event_ticker: Some(too_many),
        ..Default::default()
    };
    assert!(bad.validate().is_err());
}

#[test]
fn amend_v2_expiration_time_serializes_only_when_set() {
    let mut req = AmendOrderV2Request {
        ticker: "T".into(),
        side: BookSide::Bid,
        price: "0.5000".into(),
        count: "1.00".into(),
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
fn subaccount_balances_require_exchange_index() {
    let r: GetSubaccountBalancesResponse = serde_json::from_str(
        r#"{"subaccount_balances":[{"subaccount_number":1,"exchange_index":2,"balance":"1.50","updated_ts":5}]}"#,
    )
    .unwrap();
    assert_eq!(r.subaccount_balances[0].exchange_index, 2);
}

#[test]
fn exchange_status_index_breakdown_is_optional() {
    let r: GetExchangeStatusResponse =
        serde_json::from_str(r#"{"exchange_active":true,"trading_active":true}"#).unwrap();
    assert!(r.exchange_index_statuses.is_none());
    let r: GetExchangeStatusResponse = serde_json::from_str(
        r#"{"exchange_active":true,"trading_active":false,"intra_exchange_transfers_active":true,
        "exchange_estimated_resume_time":null,"exchange_index_statuses":[
        {"exchange_index":0,"description":"d","exchange_active":true,"trading_active":false,
         "intra_exchange_transfers_active":true}]}"#,
    )
    .unwrap();
    assert_eq!(r.exchange_index_statuses.unwrap()[0].exchange_index, 0);
}

#[test]
fn intra_exchange_transfer_types_roundtrip() {
    let body = serde_json::to_value(IntraExchangeInstanceTransferRequest {
        client_transfer_id: Some("uuid".into()),
        source: ExchangeInstance::EventContract,
        destination: ExchangeInstance::Margined,
        amount: 10_000,
        source_exchange_shard: None,
        destination_exchange_shard: Some(1),
        source_subaccount: None,
        destination_subaccount: None,
    })
    .unwrap();
    assert_eq!(body["source"], "event_contract");
    assert_eq!(body["destination"], "margined");
    assert_eq!(body["destination_exchange_shard"], 1);
    assert!(body.get("source_exchange_shard").is_none());

    let r: GetIntraExchangeInstanceTransferResponse = serde_json::from_str(
        r#"{"transfer":{"transfer_id":"t","client_transfer_id":"c","source":"event_contract",
        "destination":"margined","source_exchange_shard":0,"destination_exchange_shard":1,
        "amount":"1.0000","status":"pending","created_ts":9}}"#,
    )
    .unwrap();
    assert_eq!(r.transfer.client_transfer_id.as_deref(), Some("c"));
    assert_eq!(
        r.transfer.status,
        IntraExchangeInstanceTransferStatus::Pending
    );
}

// ---- WebSocket ------------------------------------------------------------

#[test]
fn orderbook_messages_carry_exchange_clock() {
    let snap = br#"{"type":"orderbook_snapshot","sid":1,"seq":1,"sending_ts_ms":5,
        "msg":{"market_ticker":"T","market_id":"m","clock":42,"yes_dollars_fp":[["0.50","1.00"]]}}"#;
    match WsMessageV2::from_bytes(snap).unwrap() {
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { msg, .. }) => {
            assert_eq!(msg.clock, 42)
        }
        other => panic!("unexpected {other:?}"),
    }
    let delta = br#"{"type":"orderbook_delta","sid":1,"seq":2,
        "msg":{"market_ticker":"T","market_id":"m","clock":43,"price_dollars":"0.50",
        "delta_fp":"1.00","side":"yes"}}"#;
    match WsMessageRef::from_bytes(delta).unwrap() {
        WsMessageRef::Data(WsDataMessageRef::OrderbookDelta { msg, .. }) => {
            assert_eq!(msg.clock, 43)
        }
        other => panic!("unexpected {other:?}"),
    }
    // `clock` is required by the AsyncAPI.
    let missing = br#"{"type":"orderbook_delta","sid":1,"seq":2,
        "msg":{"market_ticker":"T","market_id":"m","price_dollars":"0.50","delta_fp":"1","side":"yes"}}"#;
    assert!(WsMessageV2::from_bytes(missing).is_err());
}

fn comms(bytes: &[u8]) -> WsCommunications {
    let owned = match WsMessageV2::from_bytes(bytes).unwrap() {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => msg,
        other => panic!("unexpected {other:?}"),
    };
    // The borrowed path must agree on the variant.
    match WsMessageRef::from_bytes(bytes).unwrap() {
        WsMessageRef::Data(WsDataMessageRef::Communications { msg, .. }) => {
            let via_ref: WsCommunications = WsCommunicationsRef::into_owned(msg);
            assert_eq!(
                std::mem::discriminant(&owned),
                std::mem::discriminant(&via_ref)
            );
        }
        other => panic!("unexpected {other:?}"),
    }
    owned
}

#[test]
fn rfq_batch_messages_parse() {
    match comms(
        br#"{"type":"rfq_batch_created","sid":1,"seq":1,"msg":{"id":"b","creator_id":"c",
        "created_ts":"2026-10-25T00:00:00Z",
        "legs":[{"event_ticker":"E","market_ticker":"M","side":"yes"}],
        "rfqs":[{"id":"r","market_ticker":"M","contracts_fp":"1.00","leg_indices":[0]}]}}"#,
    ) {
        WsCommunications::RfqBatchCreated(m) => {
            assert_eq!(m.legs.len(), 1);
            assert_eq!(m.rfqs[0].leg_indices, [0]);
        }
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"rfq_batch_deleted","sid":1,"seq":2,"msg":{"id":"b","creator_id":"c",
        "rfq_ids":["r"],"deleted_ts":"t"}}"#,
    ) {
        WsCommunications::RfqBatchDeleted(m) => assert_eq!(m.rfq_ids, ["r"]),
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"batch_quote_created","sid":1,"seq":3,"msg":{"batch_quote_id":"q","rfq_batch_id":"b",
        "quote_creator_id":"c","created_ts":"t","subaccount":2,
        "quotes":[{"quote_id":"x","rfq_id":"r","market_ticker":"M","yes_bid_dollars":"0.40","no_bid_dollars":"0.55"}]}}"#,
    ) {
        WsCommunications::BatchQuoteCreated(m) => {
            assert_eq!(m.subaccount, Some(2));
            assert_eq!(m.quotes[0].yes_bid_dollars, "0.40");
        }
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"rfq_batch_best_quotes","sid":1,"seq":4,"msg":{"rfq_batch_id":"b","version":3,
        "updated_ts":"t","yes_complete":true,"no_complete":false,
        "rfqs":[{"rfq_id":"r","market_ticker":"M","yes_bid_dollars":"0.40"}]}}"#,
    ) {
        WsCommunications::RfqBatchBestQuotes(m) => {
            assert!(m.yes_complete && !m.no_complete);
            assert_eq!(m.rfqs[0].no_bid_dollars, None);
        }
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"batch_quote_accepted","sid":1,"seq":5,"msg":{"batch_quote_id":"q","rfq_batch_id":"b",
        "quote_creator_id":"c","rfq_creator_id":"d","accepted_side":"no","accepted_ts":"t",
        "quotes":[{"quote_id":"x","rfq_id":"r","market_ticker":"M","yes_bid_dollars":"0.40",
        "no_bid_dollars":"0.55","contracts_accepted_fp":"1.00"}]}}"#,
    ) {
        WsCommunications::BatchQuoteAccepted(m) => {
            assert_eq!(m.quotes[0].contracts_accepted_fp.as_deref(), Some("1.00"))
        }
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"rfq_batch_executed","sid":1,"seq":6,"msg":{"rfq_batch_id":"b","rfq_creator_id":"d",
        "executed_rfq_ids":["r"],"failed_rfq_ids":[],"executed_ts":"t","close_reason":"executed"}}"#,
    ) {
        WsCommunications::RfqBatchExecuted(m) => assert_eq!(m.executed_rfq_ids, ["r"]),
        other => panic!("unexpected {other:?}"),
    }
    match comms(
        br#"{"type":"batch_quote_executed","sid":1,"seq":7,"msg":{"batch_quote_id":"q","rfq_batch_id":"b",
        "rfq_creator_id":"d","executed_rfq_ids":[],"failed_rfq_ids":["r"],"executed_ts":"t",
        "close_reason":"failed"}}"#,
    ) {
        WsCommunications::BatchQuoteExecuted(m) => assert_eq!(m.failed_rfq_ids, ["r"]),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn quote_executed_reports_batch_child() {
    match comms(
        br#"{"type":"quote_executed","sid":1,"seq":1,"msg":{"quote_id":"q","rfq_id":"r",
        "quote_creator_id":"a","rfq_creator_id":"b","order_id":"o","client_order_id":"c",
        "market_ticker":"M","executed_ts":"t","rfq_batch_id":"batch"}}"#,
    ) {
        WsCommunications::QuoteExecuted(m) => {
            assert_eq!(m.rfq_batch_id.as_deref(), Some("batch"))
        }
        other => panic!("unexpected {other:?}"),
    }
}
