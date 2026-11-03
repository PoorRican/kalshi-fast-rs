#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{WsChannelV2, WsDataMessageV2, WsMessageV2, WsSubscriptionParamsV2};

#[tokio::test]
async fn ws_demo_connects_with_authenticated_handshake() {
    let ws = common::connect_demo_ws().await;
    drop(ws);
}

#[tokio::test]
async fn ws_demo_ticker_without_market_filters_receives_typed_data() {
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        })
        .await
        .expect("ticker subscribe failed");

    let sid = common::wait_for_subscribed(&mut ws, subscribe_id).await;
    let message = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(msg, WsMessageV2::Data(WsDataMessageV2::Ticker { .. }))
    })
    .await;

    match message {
        WsMessageV2::Data(WsDataMessageV2::Ticker {
            sid: msg_sid, msg, ..
        }) => {
            assert_eq!(msg_sid, Some(sid));
            assert!(!msg.market_ticker.is_empty());
            assert!(!msg.market_id.is_empty());
            assert!(msg.price_dollars.parse::<f64>().is_ok());
            assert!(msg.volume_fp.parse::<f64>().is_ok());
            assert!(msg.ts_ms.map_or(true, |v| v > 0));
        }
        other => panic!("expected ticker data message, got {other:?}"),
    }
}

#[tokio::test]
async fn ws_demo_trade_without_market_filters_receives_typed_data() {
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Trade],
            ..Default::default()
        })
        .await
        .expect("trade subscribe failed");

    let sid = common::wait_for_subscribed(&mut ws, subscribe_id).await;
    let message = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(msg, WsMessageV2::Data(WsDataMessageV2::Trade { .. }))
    })
    .await;

    match message {
        WsMessageV2::Data(WsDataMessageV2::Trade {
            sid: msg_sid, msg, ..
        }) => {
            assert_eq!(msg_sid, Some(sid));
            assert!(!msg.trade_id.is_empty());
            assert!(!msg.market_ticker.is_empty());
            assert!(msg.yes_price_dollars.parse::<f64>().is_ok());
            assert!(msg.no_price_dollars.parse::<f64>().is_ok());
            assert!(msg.count_fp.parse::<f64>().is_ok());
            assert!(msg.ts_ms.map_or(true, |v| v > 0));
        }
        other => panic!("expected trade data message, got {other:?}"),
    }
}
