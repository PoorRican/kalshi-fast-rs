//! Parsing/serialization coverage for the OpenAPI 3.34.0 / AsyncAPI 2.0.0 refresh.

use kalshi_fast::*;

#[test]
fn intra_exchange_transfer_roundtrip() {
    let mut req = IntraExchangeInstanceTransferRequest::new(
        ExchangeInstance::EventContract,
        ExchangeInstance::Margined,
        5000,
    );
    req.client_transfer_id = Some("7b0b4a2e-3c5f-4f1e-9d6a-0d4b6c2a1f10".into());
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["source"], "event_contract");
    assert_eq!(v["destination"], "margined");
    assert_eq!(v["amount"], 5000);
    assert!(v.get("source_subaccount").is_none());

    let json = r#"{"transfers":[{"transfer_id":"t1","client_transfer_id":"c1","source":"event_contract",
        "destination":"margined","source_exchange_shard":0,"destination_exchange_shard":1,
        "amount":"0.5000","status":"pending","created_ts":1700000000}],"cursor":"abc"}"#;
    let resp: GetIntraExchangeInstanceTransfersResponse = serde_json::from_str(json).unwrap();
    let t = &resp.transfers[0];
    assert_eq!(t.client_transfer_id.as_deref(), Some("c1"));
    assert_eq!(t.status, IntraExchangeInstanceTransferStatus::Pending);
    assert_eq!(t.destination, ExchangeInstance::Margined);

    let one: GetIntraExchangeInstanceTransferResponse = serde_json::from_str(
        r#"{"transfer":{"transfer_id":"t2","source":"margined","destination":"event_contract",
        "source_exchange_shard":0,"destination_exchange_shard":0,"amount":"1.0000",
        "status":"complete","created_ts":1}}"#,
    )
    .unwrap();
    assert_eq!(one.transfer.client_transfer_id, None);
}

#[test]
fn subaccount_balance_carries_exchange_index() {
    let r: GetSubaccountBalancesResponse = serde_json::from_str(
        r#"{"subaccount_balances":[{"subaccount_number":1,"exchange_index":2,"balance":"3.0000","updated_ts":9}]}"#,
    )
    .unwrap();
    assert_eq!(r.subaccount_balances[0].exchange_index, 2);
}

#[test]
fn positions_and_orders_new_query_params_serialize() {
    let p = GetPositionsParams {
        settlement_status: Some(PositionSettlementStatus::All),
        exchange_index: Some(1),
        ..Default::default()
    };
    let q = serde_json::to_value(&p).unwrap();
    assert_eq!(q["settlement_status"], "all");
    assert_eq!(q["exchange_index"], 1);

    let e = GetEventParams {
        exclude_markets: Some(true),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&e).unwrap(),
        serde_json::json!({"exclude_markets": true})
    );
}

#[test]
fn rfq_request_obscure_creator_id_serializes() {
    let req = CreateRFQRequest {
        market_ticker: "M".into(),
        contracts: None,
        contracts_fp: Some("1.00".into()),
        target_cost_centi_cents: None,
        target_cost_dollars: None,
        target_cost_excludes_fees: None,
        obscure_creator_id: Some(true),
        rest_remainder: true,
        replace_existing: None,
        subtrader_id: None,
        subaccount: None,
    };
    let v = serde_json::to_value(&req).unwrap();
    assert_eq!(v["obscure_creator_id"], true);
    assert!(v.get("target_cost_excludes_fees").is_none());
}

#[test]
fn amend_v2_expiration_time_serializes() {
    let req = AmendOrderV2Request {
        ticker: "M".into(),
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
fn user_order_last_update_reason_and_exchange_index() {
    let o: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","exchange_index":2,
        "last_update_reason":"SettlementBoundsCancel"}"#,
    )
    .unwrap();
    assert_eq!(o.exchange_index, Some(2));
    assert_eq!(
        o.last_update_reason,
        Some(WsLastUpdateReason::SettlementBoundsCancel)
    );
    let o: WsUserOrder = serde_json::from_str(
        r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"SomethingNew"}"#,
    )
    .unwrap();
    assert_eq!(o.last_update_reason, Some(WsLastUpdateReason::Unknown));
}

#[test]
fn orderbook_messages_carry_clock_and_envelope_sending_ts() {
    let snap: WsOrderbookSnapshotRef = serde_json::from_str(
        r#"{"market_ticker":"T","market_id":"m","clock":4182057,"yes_dollars_fp":[["0.0800","300.00"]]}"#,
    )
    .unwrap();
    assert_eq!(snap.clock, Some(4182057));
    assert_eq!(snap.into_owned().clock, Some(4182057));

    let delta: WsOrderbookDeltaRef = serde_json::from_str(
        r#"{"market_ticker":"T","market_id":"m","clock":4182061,"price_dollars":"0.9600","delta_fp":"-54.00","side":"yes"}"#,
    )
    .unwrap();
    assert_eq!(delta.into_owned().clock, Some(4182061));

    let env: WsEnvelope = serde_json::from_str(
        r#"{"type":"orderbook_delta","sid":2,"seq":3,"sending_ts_ms":1669149841123,
        "msg":{"market_ticker":"T","market_id":"m","clock":1,"price_dollars":"0.1","delta_fp":"1.00","side":"no"}}"#,
    )
    .unwrap();
    assert_eq!(env.sending_ts_ms, Some(1669149841123));
}

#[test]
fn quote_messages_carry_rfq_creator_and_batch_id() {
    let q: WsQuoteCreatedRef = serde_json::from_str(
        r#"{"quote_id":"q","rfq_id":"r","quote_creator_id":"qc","rfq_creator_id":"e3b0c442",
        "market_ticker":"T","yes_bid_dollars":"0.5000","no_bid_dollars":"0.5000",
        "created_ts":"2026-10-25T00:00:00Z","subaccount":3}"#,
    )
    .unwrap();
    assert_eq!(q.into_owned().subaccount, Some(3));
    let x: WsQuoteExecutedRef = serde_json::from_str(
        r#"{"quote_id":"q","rfq_id":"r","quote_creator_id":"a","rfq_creator_id":"b","order_id":"o",
        "client_order_id":"c","market_ticker":"T","executed_ts":"t","rfq_batch_id":"rb"}"#,
    )
    .unwrap();
    assert_eq!(x.into_owned().rfq_batch_id.as_deref(), Some("rb"));
}

#[test]
fn rfq_and_quote_rest_new_fields() {
    let r: GetRFQResponse = serde_json::from_str(
        r#"{"rfq":{"id":"r","creator_id":"c","market_ticker":"T","contracts_fp":"1.00",
        "status":"open","created_ts":"t","target_cost_excludes_fees":true,"creator_subaccount":2}}"#,
    )
    .unwrap();
    assert_eq!(r.rfq.target_cost_excludes_fees, Some(true));
    assert_eq!(r.rfq.creator_subaccount, Some(2));
}
