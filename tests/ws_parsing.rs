#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{WsChannelV2, WsDataMessageV2, WsMessageV2, WsSubscriptionParamsV2};

#[tokio::test]
async fn ws_demo_ticker_payload_parses_into_ws_ticker() {
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        })
        .await
        .expect("ticker subscribe failed");

    common::wait_for_subscribed(&mut ws, subscribe_id).await;
    let ticker = common::require_ticker_data(
        common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
            matches!(msg, WsMessageV2::Data(WsDataMessageV2::Ticker { .. }))
        })
        .await,
    );

    assert!(!ticker.market_ticker.is_empty());
    assert!(!ticker.market_id.is_empty());
    assert!(ticker.price_dollars.parse::<f64>().is_ok());
    assert!(ticker.yes_bid_dollars.parse::<f64>().is_ok());
    assert!(ticker.yes_ask_dollars.parse::<f64>().is_ok());
    assert!(ticker.last_trade_size_fp.parse::<f64>().is_ok());
    assert!(ticker.open_interest_fp.parse::<f64>().is_ok());
    assert!(ticker.ts > 0);
    assert!(ticker.ts_ms.map_or(true, |v| v > 0));
}

#[tokio::test]
async fn ws_demo_trade_payload_parses_into_ws_trade() {
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Trade],
            ..Default::default()
        })
        .await
        .expect("trade subscribe failed");

    common::wait_for_subscribed(&mut ws, subscribe_id).await;
    let trade = common::require_trade_data(
        common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
            matches!(msg, WsMessageV2::Data(WsDataMessageV2::Trade { .. }))
        })
        .await,
    );

    assert!(!trade.trade_id.is_empty());
    assert!(!trade.market_ticker.is_empty());
    assert!(trade.count_fp.parse::<f64>().is_ok());
    assert!(trade.yes_price_dollars.parse::<f64>().is_ok());
    assert!(trade.no_price_dollars.parse::<f64>().is_ok());
    assert!(trade.ts > 0);
    assert!(trade.ts_ms.map_or(true, |v| v > 0));
}

#[tokio::test]
async fn ws_demo_orderbook_delta_subscription_parses_snapshot_and_delta() {
    let active_market_tickers = common::active_demo_market_tickers_via_trade(3).await;

    let mut ws = common::connect_demo_ws().await;
    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(active_market_tickers.clone()),
            ..Default::default()
        })
        .await
        .expect("orderbook subscribe failed");

    common::wait_for_subscribed(&mut ws, subscribe_id).await;

    let snapshot = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { .. })
        )
    })
    .await;

    match snapshot {
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { msg, .. }) => {
            assert!(active_market_tickers.contains(&msg.market_ticker));
            assert!(!msg.market_id.is_empty());
            assert!(
                !msg.yes_dollars_fp.is_empty() || !msg.no_dollars_fp.is_empty(),
                "expected at least one populated orderbook side"
            );
        }
        other => panic!("expected orderbook snapshot, got {other:?}"),
    }

    let delta = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { .. })
        )
    })
    .await;

    match delta {
        WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { msg, .. }) => {
            assert!(active_market_tickers.contains(&msg.market_ticker));
            assert!(!msg.market_id.is_empty());
            assert!(msg.price_dollars.parse::<f64>().is_ok());
            assert!(msg.delta_fp.parse::<f64>().is_ok());
        }
        other => panic!("expected orderbook delta, got {other:?}"),
    }
}

#[test]
fn user_order_last_update_reason_and_sending_ts_ignored() {
    let json = r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"ExpiryCancel","some_future":1}"#;
    let o: kalshi_fast::WsUserOrder = serde_json::from_str(json).unwrap();
    assert_eq!(
        o.last_update_reason,
        Some(kalshi_fast::WsLastUpdateReason::ExpiryCancel)
    );
    let json = r#"{"order_id":"o","user_id":"u","ticker":"T","last_update_reason":"Brand New"}"#;
    let o: kalshi_fast::WsUserOrder = serde_json::from_str(json).unwrap();
    assert_eq!(
        o.last_update_reason,
        Some(kalshi_fast::WsLastUpdateReason::Unknown)
    );
}
