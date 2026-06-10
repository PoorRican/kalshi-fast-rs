use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannelV2, WsDataMessageV2, WsEvent,
    WsMessageV2, WsReconnectConfig, WsSubscriptionParamsV2, WsUpdateAction,
    WsUpdateSubscriptionParamsV2,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_id = std::env::var("KALSHI_KEY_ID")?;
    let private_key_path = std::env::var("KALSHI_PRIVATE_KEY_PATH")?;
    let auth = KalshiAuth::from_pem_file(key_id, private_key_path)?;

    let mut ws = KalshiWsClient::connect_authenticated(
        KalshiEnvironment::demo(),
        auth,
        WsReconnectConfig::default(),
    )
    .await?;

    let sub_cmd_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::UserOrders],
            ..Default::default()
        })
        .await?;

    while let Ok(event) = ws.next_event_v2().await {
        match event {
            WsEvent::Message(WsMessageV2::Subscribed {
                id: Some(id),
                sid: Some(subscription_id),
            }) if id == sub_cmd_id => {
                println!("subscribed sid={subscription_id}");

                // Update action example (for channels supporting market filters).
                let _ = ws
                    .update_subscription_v2(WsUpdateSubscriptionParamsV2 {
                        action: WsUpdateAction::AddMarkets,
                        sid: Some(subscription_id),
                        sids: None,
                        market_ticker: std::env::var("KALSHI_MARKET_TICKER").ok(),
                        market_tickers: None,
                        market_id: None,
                        market_ids: None,
                        send_initial_snapshot: None,
                        skip_ticker_ack: None,
                        index_ids: None,
                    })
                    .await;
            }
            WsEvent::Message(WsMessageV2::Data(WsDataMessageV2::UserOrder { msg, .. })) => {
                println!(
                    "order={} ticker={} status={:?}",
                    msg.order_id, msg.ticker, msg.status
                );
            }
            WsEvent::Disconnected { error } => {
                eprintln!("websocket disconnected: {error}");
                break;
            }
            _ => {}
        }
    }

    let _ = ws.close().await;
    Ok(())
}
