//! Parsing/serialization tests for the 2026-10 spec refresh.

pub(crate) use cargo_husky as _;
use kalshi_fast::{
    CreateMarginOrderRequest, ExchangeInstance, GetFillsParams,
    GetIntraExchangeInstanceTransferResponse, GetIntraExchangeInstanceTransfersResponse,
    GetMarginTradesParams, GetMultivariateEventCollectionsResponse, GetPositionsParams,
    IntraExchangeInstanceTransferRequest, IntraExchangeInstanceTransferStatus,
    MarginFundingLimitUnit, MarginMarket, PositionSettlementStatus,
};
use serde_json::json;

fn query<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_value(v).unwrap().to_string()
}

#[test]
fn margin_market_new_fields_parse() {
    let m: MarginMarket = serde_json::from_value(json!({
        "ticker":"X","status":"active","title":"t","contract_size":"1.000000",
        "underlying_multiplier":"1","tick_size":"0.01","fractional_trading_enabled":true,
        "schedule":null,"exchange_index":0,"market_version":2,
        "funding_deadband":0.5,"funding_cap":2.0,"funding_limit_unit":"percentage",
        "bid_size_fp":"10.00","ask_size_fp":"0.00"
    }))
    .unwrap();
    assert_eq!(m.market_version, Some(2));
    assert_eq!(
        m.funding_limit_unit,
        Some(MarginFundingLimitUnit::Percentage)
    );
    assert_eq!(m.funding_cap, Some(2.0));
    assert_eq!(m.bid_size_fp.as_deref(), Some("10.00"));
    assert_eq!(m.ask_size_fp.as_deref(), Some("0.00"));
}

#[test]
fn margin_market_without_new_fields_still_parses() {
    let m: MarginMarket = serde_json::from_value(json!({
        "ticker":"X","status":"active","title":"t","contract_size":"1",
        "underlying_multiplier":"1","tick_size":"0.01","fractional_trading_enabled":true,
        "exchange_index":0
    }))
    .unwrap();
    assert!(m.market_version.is_none() && m.bid_size_fp.is_none());
}

#[test]
fn margin_trades_ticker_optional() {
    assert_eq!(query(&GetMarginTradesParams::default()), "{}");
    let p = GetMarginTradesParams {
        ticker: Some("X".into()),
        ..Default::default()
    };
    assert_eq!(query(&p), r#"{"ticker":"X"}"#);
}

#[test]
fn margin_order_market_version_serializes_only_when_set() {
    let mut v = serde_json::to_value(CreateMarginOrderRequestFixture::build(None)).unwrap();
    assert!(v.get("market_version").is_none());
    v = serde_json::to_value(CreateMarginOrderRequestFixture::build(Some(3))).unwrap();
    assert_eq!(v["market_version"], 3);
}

struct CreateMarginOrderRequestFixture;
impl CreateMarginOrderRequestFixture {
    fn build(market_version: Option<i32>) -> CreateMarginOrderRequest {
        CreateMarginOrderRequest {
            ticker: "X".into(),
            client_order_id: "c".into(),
            side: kalshi_fast::BookSide::Bid,
            count: "1".into(),
            price: "0.5".into(),
            time_in_force: kalshi_fast::MarginTimeInForce::GoodTillCanceled,
            self_trade_prevention_type: kalshi_fast::MarginSelfTradePreventionType::TakerAtCross,
            expiration_time: None,
            post_only: None,
            cancel_order_on_pause: None,
            reduce_only: None,
            subaccount: None,
            order_group_id: None,
            market_version,
        }
    }
}

#[test]
fn positions_settlement_status_serializes() {
    let p = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::Settled),
        ..Default::default()
    };
    assert_eq!(query(&p), r#"{"settlement_status":"settled"}"#);
    assert_eq!(query(&GetPositionsParams::default()), "{}");
}

#[test]
fn fills_ticker_accepts_csv() {
    let p = GetFillsParams {
        ticker: Some("A,B".into()),
        ..Default::default()
    };
    assert_eq!(query(&p), r#"{"ticker":"A,B"}"#);
}

#[test]
fn intra_exchange_transfer_roundtrip() {
    let req = IntraExchangeInstanceTransferRequest {
        client_transfer_id: Some("4f9c0f2e-0000-4000-8000-000000000000".into()),
        source: ExchangeInstance::EventContract,
        destination: ExchangeInstance::Margined,
        amount: 10_000,
        source_exchange_shard: None,
        destination_exchange_shard: None,
        source_subaccount: None,
        destination_subaccount: None,
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["source"], "event_contract");
    assert_eq!(v["destination"], "margined");
    assert!(v.get("source_subaccount").is_none());

    let list: GetIntraExchangeInstanceTransfersResponse = serde_json::from_value(json!({
        "transfers":[{"transfer_id":"t","source":"event_contract","destination":"margined",
        "source_exchange_shard":0,"destination_exchange_shard":1,"amount":"1.0000",
        "status":"pending","created_ts":1}]
    }))
    .unwrap();
    assert_eq!(
        list.transfers[0].status,
        IntraExchangeInstanceTransferStatus::Pending
    );
    assert!(list.cursor.is_none());

    let one: GetIntraExchangeInstanceTransferResponse = serde_json::from_value(json!({
        "transfer":{"transfer_id":"t","source":"margined","destination":"event_contract",
        "source_exchange_shard":0,"destination_exchange_shard":0,"amount":"1.0000",
        "status":"complete","created_ts":1}
    }))
    .unwrap();
    assert_eq!(one.transfer.source, ExchangeInstance::Margined);
}

#[test]
fn multivariate_collection_price_structure_parses() {
    let r: GetMultivariateEventCollectionsResponse = serde_json::from_value(json!({
        "multivariate_contracts":[{
            "collection_ticker":"C","series_ticker":"S","title":"t","description":"d",
            "open_date":"a","close_date":"b","associated_events":[],"associated_event_tickers":[],
            "is_ordered":false,"is_single_market_per_event":false,"is_all_yes":false,
            "size_min":1,"size_max":2,"functional_description":"f",
            "price_level_structure":"linear_cent",
            "price_ranges":[{"start":"0.0000","end":"1.0000","step":"0.0100"}]
        }]
    }))
    .unwrap();
    let c = &r.multivariate_contracts[0];
    assert_eq!(c.price_level_structure.as_deref(), Some("linear_cent"));
    assert_eq!(c.price_ranges.as_ref().unwrap().len(), 1);
}
