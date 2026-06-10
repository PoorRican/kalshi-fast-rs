#![cfg(feature = "live-tests")]

mod common;

use kalshi_fast::{
    WsChannelV2, WsDataMessageV2, WsMessageV2, WsSubscriptionParamsV2, WsUpdateAction,
    WsUpdateSubscriptionParamsV2,
};

#[tokio::test]
async fn ws_demo_orderbook_get_snapshot_returns_snapshot_without_mutating_subscription() {
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

    // Drain until we see the first orderbook message so the subscription is live.
    let initial = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { .. })
                | WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { .. })
        )
    })
    .await;

    match initial {
        WsMessageV2::Data(WsDataMessageV2::OrderbookDelta {
            sid: msg_sid, msg, ..
        }) => {
            assert_eq!(msg_sid, Some(sid));
            assert_eq!(msg.market_ticker, market_ticker);
        }
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
            sid: msg_sid, msg, ..
        }) => {
            assert_eq!(msg_sid, Some(sid));
            assert_eq!(msg.market_ticker, market_ticker);
        }
        other => panic!("unexpected initial orderbook message: {other:?}"),
    }

    let update_id = ws
        .update_subscription_v2(WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(sid),
            sids: None,
            market_ticker: Some(market_ticker.clone()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        })
        .await
        .expect("get_snapshot update failed");

    let snapshot = common::wait_for_message(&mut ws, common::CHANNEL_TIMEOUT, |msg| {
        matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
                sid: Some(msg_sid),
                msg,
                ..
            }) if *msg_sid == sid && msg.market_ticker == market_ticker
        )
    })
    .await;

    match snapshot {
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
            sid: msg_sid, msg, ..
        }) => {
            assert_eq!(msg_sid, Some(sid));
            assert_eq!(msg.market_ticker, market_ticker);
            assert!(!msg.market_id.is_empty());
            assert!(
                msg.yes_dollars_fp
                    .iter()
                    .all(|(p, q)| !p.is_empty() && !q.is_empty())
            );
            assert!(
                msg.no_dollars_fp
                    .iter()
                    .all(|(p, q)| !p.is_empty() && !q.is_empty())
            );
        }
        other => panic!("expected orderbook_snapshot after get_snapshot, got {other:?}"),
    }

    let list_id = ws
        .list_subscriptions()
        .await
        .expect("list_subscriptions failed after get_snapshot");

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
                .expect("subscription missing after get_snapshot");
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

    assert!(update_id > 0);
}
