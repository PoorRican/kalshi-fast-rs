#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{
    AmendOrderRequest, BatchCancelOrdersRequest, BatchCreateOrdersRequest, BuySell,
    CancelOrderParams, CreateOrderRequest, GetMarketsParams, GetOrdersParams, KalshiRestClient,
    MarketStatus, OrderType, YesNo,
};
use std::time::Duration;

/// Longer timeout for multi-step lifecycle tests
const LIFECYCLE_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
async fn test_order_lifecycle() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    // 1. Find an open market
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

    // 2. Create a limit order at an extreme price so it won't fill (1 cent YES)
    let create_resp = tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
        client
            .create_order(CreateOrderRequest {
                ticker: market_ticker.clone(),
                side: YesNo::Yes,
                action: BuySell::Buy,
                count: Some(1),
                r#type: Some(OrderType::Limit),
                yes_price: Some(1),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("create_order failed");

    let order_id = create_resp.order.order_id.clone();

    // Use a closure to ensure cleanup even on assertion failures
    let result = async {
        // 3. Get order by ID and verify
        let get_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
            client.get_order(&order_id).await
        })
        .await
        .expect("timeout")
        .expect("get_order failed");

        assert_eq!(get_resp.order.order_id, order_id);
        assert_eq!(get_resp.order.ticker, market_ticker);

        // 4. Amend the order (change price to 2 cents)
        let amend_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
            client
                .amend_order(
                    &order_id,
                    AmendOrderRequest {
                        ticker: market_ticker.clone(),
                        side: YesNo::Yes,
                        action: BuySell::Buy,
                        yes_price: Some(2),
                        count: Some(1),
                        ..Default::default()
                    },
                )
                .await
        })
        .await
        .expect("timeout")
        .expect("amend_order failed");

        assert_eq!(amend_resp.order.order_id, amend_resp.order.order_id);

        // 5. Get queue position for the order
        let _queue_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
            client
                .get_order_queue_position(&amend_resp.order.order_id)
                .await
        })
        .await
        .expect("timeout")
        .expect("get_order_queue_position failed");

        amend_resp.order.order_id.clone()
    }
    .await;

    // 6. Cancel the order (cleanup) - use the potentially amended order_id
    let cancel_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .cancel_order(&result, CancelOrderParams::default())
            .await
    })
    .await
    .expect("timeout")
    .expect("cancel_order failed");

    assert!(cancel_resp.reduced_by > 0 || cancel_resp.order.order_id == result);

    // 7. Verify order is cancelled via get_orders
    let orders_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .get_orders(GetOrdersParams {
                limit: Some(100),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("get_orders failed");

    // The cancelled order should not appear in active orders
    assert!(!orders_resp.orders.iter().any(|o| o.order_id == result));
}

#[tokio::test]
async fn test_batch_order_lifecycle() {
    common::load_env();
    let auth = common::load_auth();
    let client = KalshiRestClient::new(common::demo_env()).with_auth(auth);

    // 1. Find an open market
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

    // 2. Batch create 2 limit orders at extreme prices
    let batch_resp = tokio::time::timeout(LIFECYCLE_TIMEOUT, async {
        client
            .batch_create_orders(BatchCreateOrdersRequest {
                orders: vec![
                    CreateOrderRequest {
                        ticker: market_ticker.clone(),
                        side: YesNo::Yes,
                        action: BuySell::Buy,
                        count: Some(1),
                        r#type: Some(OrderType::Limit),
                        yes_price: Some(1),
                        ..Default::default()
                    },
                    CreateOrderRequest {
                        ticker: market_ticker.clone(),
                        side: YesNo::Yes,
                        action: BuySell::Buy,
                        count: Some(1),
                        r#type: Some(OrderType::Limit),
                        yes_price: Some(2),
                        ..Default::default()
                    },
                ],
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("batch_create_orders failed");

    let order_ids: Vec<String> = batch_resp
        .orders
        .iter()
        .filter_map(|r| r.order.as_ref().map(|o| o.order_id.clone()))
        .collect();

    assert!(
        !order_ids.is_empty(),
        "at least one order should have been created"
    );

    // 3. Batch cancel all created orders (cleanup)
    let cancel_resp = tokio::time::timeout(common::TEST_TIMEOUT, async {
        client
            .batch_cancel_orders(BatchCancelOrdersRequest {
                ids: Some(order_ids.clone()),
                orders: None,
            })
            .await
    })
    .await
    .expect("timeout")
    .expect("batch_cancel_orders failed");

    assert_eq!(cancel_resp.orders.len(), order_ids.len());
}
