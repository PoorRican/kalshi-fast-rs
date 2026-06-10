/// Example: Find a high-volume event and stream orderbook deltas for all its markets
///
/// 1. Queries markets to find one with 24h volume > threshold
/// 2. Fetches all markets for that event
/// 3. Subscribes to orderbook deltas for all markets in the event
/// 4. Prints each delta update via debug logging
///
/// Requires KALSHI_KEY_ID and KALSHI_PRIVATE_KEY_PATH env vars (or .env file)
use kalshi_fast::{
    GetMarketsParams, KalshiAuth, KalshiEnvironment, KalshiRestClient, KalshiWsClient, Market,
    MarketStatusQuery, MveFilter, WsDataMessageV2, WsEvent, WsMessageV2, WsReconnectConfig,
    WsSubscriptionParamsV2, parse_f64_opt,
};
use std::time::Duration;
use tokio::time::sleep;

const MIN_VOLUME_24H: f64 = 1_000.0;
const MAX_PAGES: usize = 50;

fn get_volume_24h(market: &Market) -> f64 {
    parse_f64_opt(market.volume_24h_fp.as_deref()).expect("market volume_24h_fp missing")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = KalshiEnvironment::production();
    let client = KalshiRestClient::new(env.clone());

    // Step 1: Paginate through markets to find one with high volume
    println!("Searching for markets with 24h volume > {MIN_VOLUME_24H}...");

    let mut cursor: Option<String> = None;
    let mut target_market: Option<Market> = None;
    let mut best_market: Option<(f64, Market)> = None;

    for _page in 1..=MAX_PAGES {
        let resp = client
            .get_markets(GetMarketsParams {
                limit: Some(100),
                status: Some(MarketStatusQuery::Open),
                mve_filter: Some(MveFilter::Exclude),
                cursor: cursor.clone(),
                ..Default::default()
            })
            .await?;

        print!(".");

        for market in resp.markets {
            let volume_24h = get_volume_24h(&market);
            if best_market
                .as_ref()
                .is_none_or(|(best_volume, _)| volume_24h > *best_volume)
            {
                best_market = Some((volume_24h, market.clone()));
            }
            if volume_24h > MIN_VOLUME_24H {
                target_market = Some(market);
                break;
            }
        }

        if target_market.is_some() {
            break;
        }

        match resp.cursor {
            Some(c) if !c.is_empty() => cursor = Some(c),
            _ => break,
        }

        // Rate limit: 100ms between requests (extra safety; client also rate limits)
        sleep(Duration::from_millis(100)).await;
    }

    println!();

    let target_market = match target_market {
        Some(m) => m,
        None => match best_market {
            Some((best_volume, market)) => {
                eprintln!(
                    "No market found with 24h volume > {MIN_VOLUME_24H}; falling back to highest scanned 24h volume {best_volume:.2}"
                );
                market
            }
            None => {
                anyhow::bail!("No open, non-MVE market found while scanning production markets");
            }
        },
    };

    let event_ticker = target_market
        .event_ticker
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Market missing event_ticker"))?;

    println!(
        "Found event: {} (24h volume: {:.2})",
        event_ticker,
        get_volume_24h(&target_market)
    );

    // Step 2: Fetch all markets for this event
    let event_markets = client
        .get_markets(GetMarketsParams {
            limit: Some(100),
            event_ticker: Some(vec![event_ticker.to_string()]),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        })
        .await?;

    let market_tickers: Vec<String> = event_markets
        .markets
        .iter()
        .map(|m| m.ticker.clone())
        .collect();

    println!(
        "Subscribing to {} markets: {:?}",
        market_tickers.len(),
        market_tickers
    );

    // Step 3: Connect authenticated WebSocket
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;

    let mut ws =
        KalshiWsClient::connect_authenticated(env, auth, WsReconnectConfig::default()).await?;

    // Step 4: Subscribe to orderbook deltas
    let sub_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![kalshi_fast::WsChannelV2::OrderbookDelta],
            market_tickers: Some(market_tickers),
            ..Default::default()
        })
        .await?;

    println!("Subscribed (id={}), streaming...\n", sub_id);

    // Step 5: Stream updates
    loop {
        match ws.next_event_v2().await? {
            WsEvent::Message(msg) => match msg {
                WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { msg, seq, .. }) => {
                    println!(
                        "[SNAPSHOT] {} | yes_depth={} no_depth={} | seq={:?}",
                        msg.market_ticker,
                        msg.yes_dollars_fp.len(),
                        msg.no_dollars_fp.len(),
                        seq
                    );
                }
                WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { msg, seq, .. }) => {
                    println!(
                        "[DELTA] {} | {}@{} {:+} | seq={:?}",
                        msg.market_ticker, msg.side, msg.price_dollars, msg.delta_fp, seq
                    );
                }
                WsMessageV2::Subscribed { sid, .. } => println!("[SUBSCRIBED] sid={:?}", sid),
                WsMessageV2::Error { error, .. } => println!("[ERROR] {:?}", error),
                other => println!("[OTHER] {:?}", other),
            },
            WsEvent::Raw(_) => {}
            WsEvent::Reconnected { attempt } => println!("[RECONNECTED] attempt={}", attempt),
            WsEvent::Disconnected { error } => {
                println!("[DISCONNECTED] {:?}", error);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kalshi_fast::parse_f64;
    use serde_json::json;

    fn market_from_value(value: serde_json::Value) -> Market {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn parse_f64_accepts_decimal_strings() {
        assert_eq!(parse_f64(Some("123.45")), 123.45);
        assert_eq!(parse_f64(Some("0.00")), 0.0);
        assert_eq!(parse_f64(None), 0.0);
        assert_eq!(parse_f64(Some("not-a-number")), 0.0);
    }

    #[test]
    fn get_volume_24h_uses_volume_24h_fp_not_lifetime_volume() {
        let market = market_from_value(json!({
            "ticker": "MKT-1",
            "volume_fp": "9999.00",
            "volume_24h_fp": "12.50"
        }));

        assert_eq!(get_volume_24h(&market), 12.5);
    }
}
