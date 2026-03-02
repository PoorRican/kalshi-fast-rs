#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{
    EventStatus, GetEventForecastPercentileHistoryParams, GetEventsParams, GetFillsParams,
    GetOrderQueuePositionsParams, GetOrdersParams, GetPositionsParams, GetQuotesParams,
    GetRFQsParams, GetSettlementsParams, GetSubaccountTransfersParams, KalshiError,
    KalshiRestClient, SubaccountQueryParams,
};

#[tokio::test]
async fn test_get_balance() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async { client.get_balance().await })
        .await
        .expect("timeout")
        .expect("request failed");

    // Balance fields should exist (may be 0)
    assert!(resp.balance >= 0);
    assert!(resp.portfolio_value >= 0);
    assert!(resp.updated_ts > 0);
}

#[tokio::test]
async fn test_get_positions() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_positions(GetPositionsParams {
                limit: Some(10),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    // Positions may be empty, but the vectors should exist
    assert!(resp.market_positions.len() <= 10);
}

#[tokio::test]
async fn test_get_orders() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_orders(GetOrdersParams {
                limit: Some(10),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    // Orders may be empty, but the vector should exist
    assert!(resp.orders.len() <= 10);
}

#[tokio::test]
async fn test_get_fills() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_fills(GetFillsParams {
                limit: Some(1),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.fills.len() <= 1);
}

#[tokio::test]
async fn test_get_settlements() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_settlements(GetSettlementsParams {
                limit: Some(1),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.settlements.len() <= 1);
}

#[tokio::test]
async fn test_get_account_api_limits() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_account_api_limits().await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.read_limit >= 0);
    assert!(resp.write_limit >= 0);
}

#[tokio::test]
async fn test_get_subaccount_balances() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_subaccount_balances().await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    if let Some(first) = resp.subaccount_balances.first() {
        assert!(first.updated_ts > 0);
    }
}

#[tokio::test]
async fn test_get_subaccount_transfers() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_subaccount_transfers(GetSubaccountTransfersParams {
                limit: Some(1),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.subaccount_transfers.len() <= 1);
}

#[tokio::test]
async fn test_auth_required_without_auth() {
    let client = KalshiRestClient::new(common::demo_env());

    let result =
        tokio::time::timeout(common::TEST_TIMEOUT, async { client.get_balance().await }).await;

    match result {
        Ok(Err(KalshiError::AuthRequired(_))) => {
            // Expected: auth required error from client
        }
        Ok(Err(e)) => panic!("Expected AuthRequired, got: {:?}", e),
        Ok(Ok(_)) => panic!("Expected error, got success"),
        Err(_) => panic!("timeout"),
    }
}

#[tokio::test]
async fn test_get_portfolio_total_resting_order_value() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_portfolio_total_resting_order_value().await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(resp.total_resting_order_value >= 0);
}

#[tokio::test]
async fn test_get_api_keys() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let _resp = tokio::time::timeout(common::TEST_TIMEOUT, async { client.get_api_keys().await })
        .await
        .expect("timeout")
        .expect("request failed");
}

#[tokio::test]
async fn test_get_communications_id() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_communications_id().await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    assert!(!resp.communications_id.is_empty());
}

#[tokio::test]
async fn test_get_order_groups() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let _resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_order_groups(SubaccountQueryParams::default())
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");
}

#[tokio::test]
async fn test_get_order_queue_positions() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let _resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_order_queue_positions(GetOrderQueuePositionsParams::default())
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");
}

#[tokio::test]
async fn test_get_rfqs() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_rfqs(GetRFQsParams::default()).await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    // RFQs may be empty, but the response should parse
    let _ = resp.rfqs;
}

#[tokio::test]
async fn test_get_quotes() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_quotes(GetQuotesParams::default()).await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    // Quotes may be empty, but the response should parse
    let _ = resp.quotes;
}

#[tokio::test]
async fn test_get_event_forecast_percentile_history() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    // First find an open event with a series_ticker
    let events_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_events(GetEventsParams {
                limit: Some(5),
                status: Some(EventStatus::Open),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    let event = events_resp
        .events
        .iter()
        .find(|e| e.series_ticker.is_some());

    let event = match event {
        Some(e) => e,
        None => return,
    };

    let series_ticker = event.series_ticker.as_ref().unwrap().clone();
    let event_ticker = event.event_ticker.clone();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let _resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_event_forecast_percentile_history(
                &series_ticker,
                &event_ticker,
                GetEventForecastPercentileHistoryParams {
                    percentiles: vec![25, 50, 75],
                    start_ts: now - 86400 * 7,
                    end_ts: now,
                    period_interval: 3600,
                },
            )
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");
}

#[tokio::test]
async fn test_get_subaccount_transfers_all() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let transfers = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_subaccount_transfers_all(GetSubaccountTransfersParams {
                limit: Some(100),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    // Transfers may be empty on demo
    let _ = transfers;
}

#[tokio::test]
async fn test_get_rfqs_all() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let rfqs = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_rfqs_all(GetRFQsParams::default()).await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    let _ = rfqs;
}

#[tokio::test]
async fn test_get_quotes_all() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let quotes = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_quotes_all(GetQuotesParams::default()).await
    })
    .await
    .expect("timeout")
    .expect("request failed");

    let _ = quotes;
}

#[tokio::test]
async fn test_get_subaccount_netting() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    let _resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client.get_subaccount_netting().await
    })
    .await
    .expect("timeout")
    .expect("request failed");
}
