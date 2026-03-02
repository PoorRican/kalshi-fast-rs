#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{
    CreateQuoteRequest, CreateRFQRequest, GetMarketsParams, KalshiRestClient, MarketStatus,
};
use std::time::Duration;

const LIFECYCLE_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
async fn test_rfq_lifecycle() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    // Find an open market
    let markets_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_markets(GetMarketsParams {
                limit: Some(1),
                status: Some(MarketStatus::Open),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    if markets_resp.markets.is_empty() {
        return;
    }

    let market_ticker = markets_resp.markets[0].ticker.clone();

    // 1. Create an RFQ
    let create_resp = tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
        client
            .create_rfq(CreateRFQRequest {
                market_ticker: market_ticker.clone(),
                contracts: Some(1),
                contracts_fp: None,
                target_cost_centi_cents: None,
                target_cost_dollars: None,
                rest_remainder: false,
                replace_existing: None,
                subtrader_id: None,
                subaccount: None,
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("create_rfq failed");

    let rfq_id = create_resp.id.clone();

    // 2. Get the RFQ and verify
    let get_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_rfq(&rfq_id).await
    })
    .await
    .expect("timeout")
    .expect("get_rfq failed");

    assert_eq!(get_resp.rfq.id, rfq_id);
    assert_eq!(get_resp.rfq.market_ticker, market_ticker);

    // 3. Delete the RFQ (cleanup)
    let _delete_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.delete_rfq(&rfq_id).await
    })
    .await
    .expect("timeout")
    .expect("delete_rfq failed");
}

#[tokio::test]
async fn test_quote_lifecycle() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    // Find an open market
    let markets_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_markets(GetMarketsParams {
                limit: Some(1),
                status: Some(MarketStatus::Open),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    if markets_resp.markets.is_empty() {
        return;
    }

    let market_ticker = markets_resp.markets[0].ticker.clone();

    // 1. Create an RFQ as prerequisite
    let rfq_resp = tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
        client
            .create_rfq(CreateRFQRequest {
                market_ticker: market_ticker.clone(),
                contracts: Some(1),
                contracts_fp: None,
                target_cost_centi_cents: None,
                target_cost_dollars: None,
                rest_remainder: false,
                replace_existing: None,
                subtrader_id: None,
                subaccount: None,
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("create_rfq failed");

    let rfq_id = rfq_resp.id.clone();

    // 2. Create a quote on the RFQ
    let quote_resp = tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
        client
            .create_quote(CreateQuoteRequest {
                rfq_id: rfq_id.clone(),
                yes_bid: "0.01".to_string(),
                no_bid: "0.01".to_string(),
                rest_remainder: false,
                subaccount: None,
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("create_quote failed");

    let quote_id = quote_resp.id.clone();

    // 3. Get the quote and verify
    let get_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_quote(&quote_id).await
    })
    .await
    .expect("timeout")
    .expect("get_quote failed");

    assert_eq!(get_resp.quote.id, quote_id);
    assert_eq!(get_resp.quote.rfq_id, rfq_id);

    // 4. Delete the quote (cleanup)
    let _delete_quote = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.delete_quote(&quote_id).await
    })
    .await
    .expect("timeout")
    .expect("delete_quote failed");

    // 5. Delete the RFQ (cleanup)
    let _delete_rfq = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.delete_rfq(&rfq_id).await
    })
    .await
    .expect("timeout")
    .expect("delete_rfq failed");
}
