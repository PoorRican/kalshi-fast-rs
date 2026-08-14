#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{WsChannelV2, WsMessageV2, WsSubscriptionParamsV2, WsUnsubscribeParamsV2};

#[tokio::test]
async fn ws_demo_ticker_subscription_can_be_listed_and_unsubscribed() {
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        })
        .await
        .expect("ticker subscribe failed");

    let sid = common::wait_for_subscribed(&mut ws, subscribe_id).await;

    let list_id = ws
        .list_subscriptions()
        .await
        .expect("list_subscriptions failed");

    let listed = common::wait_for_message(
        &mut ws,
        common::CHANNEL_TIMEOUT,
        |msg| matches!(msg, WsMessageV2::ListSubscriptions { id: Some(id), .. } if *id == list_id),
    )
    .await;

    match listed {
        WsMessageV2::ListSubscriptions { subscriptions, .. } => {
            let subscription = subscriptions
                .into_iter()
                .find(|sub| sub.sid == sid)
                .expect("ticker subscription missing from list_subscriptions");
            assert!(
                subscription.channels.contains(&WsChannelV2::Ticker)
                    || subscription.channel == Some(WsChannelV2::Ticker)
            );
            assert!(subscription.market_tickers.is_none());
            assert!(subscription.market_ids.is_none());
        }
        other => panic!("expected list_subscriptions response, got {other:?}"),
    }

    let unsubscribe_id = ws
        .unsubscribe_v2(WsUnsubscribeParamsV2 { sids: vec![sid] })
        .await
        .expect("unsubscribe failed");

    let unsubscribed = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(
            msg,
            WsMessageV2::Unsubscribed {
                id: Some(id),
                sid: Some(msg_sid),
                ..
            } if *id == unsubscribe_id && *msg_sid == sid
        )
    })
    .await;

    match unsubscribed {
        WsMessageV2::Unsubscribed {
            id: Some(id),
            sid: Some(msg_sid),
            ..
        } => {
            assert_eq!(id, unsubscribe_id);
            assert_eq!(msg_sid, sid);
        }
        other => panic!("expected unsubscribe acknowledgement, got {other:?}"),
    }
}

#[tokio::test]
async fn ws_demo_orderbook_subscription_is_listed_with_market_filter() {
    let market_ticker = common::first_open_demo_market_ticker().await;
    let mut ws = common::connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ticker: Some(market_ticker.clone()),
            ..Default::default()
        })
        .await
        .expect("orderbook subscribe failed");

    let sid = common::wait_for_subscribed(&mut ws, subscribe_id).await;

    let list_id = ws
        .list_subscriptions()
        .await
        .expect("list_subscriptions failed");

    let listed = common::wait_for_message(
        &mut ws,
        common::CHANNEL_TIMEOUT,
        |msg| matches!(msg, WsMessageV2::ListSubscriptions { id: Some(id), .. } if *id == list_id),
    )
    .await;

    match listed {
        WsMessageV2::ListSubscriptions { subscriptions, .. } => {
            let subscription = subscriptions
                .into_iter()
                .find(|sub| sub.sid == sid)
                .expect("orderbook subscription missing from list_subscriptions");
            assert!(
                subscription.channels.contains(&WsChannelV2::OrderbookDelta)
                    || subscription.channel == Some(WsChannelV2::OrderbookDelta)
            );
            if let Some(market_tickers) = subscription.market_tickers {
                assert_eq!(market_tickers, vec![market_ticker]);
            }
        }
        other => panic!("expected list_subscriptions response, got {other:?}"),
    }
}
