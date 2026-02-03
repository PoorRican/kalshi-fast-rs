/// Example: Find a high-volume event and stream orderbook deltas for all its markets
///
/// 1. Queries markets to find one with 24h volume > threshold
/// 2. Fetches all markets for that event
/// 3. Subscribes to orderbook deltas for all markets in the event
/// 4. Prints each delta update via debug logging
///
/// Requires KALSHI_KEY_ID and KALSHI_PRIVATE_KEY_PATH env vars (or .env file)

use kalshi::{
    GetMarketsParams, KalshiAuth, KalshiEnvironment, KalshiRestClient, KalshiWsClient,
    MarketStatus, MveFilter, WsChannel,
};

const MIN_VOLUME_24H: i64 = 10_000;
const MAX_PAGES: usize = 50;

fn get_volume(market: &serde_json::Value) -> i64 {
    market
        .get("volume_24h")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = KalshiEnvironment::demo();
    let client = KalshiRestClient::new(env.clone());

    // Step 1: Paginate through markets to find one with high volume (exclude MVE)
    println!("Searching for markets with 24h volume > {MIN_VOLUME_24H}...");

    let mut cursor: Option<String> = None;
    let mut target_market: Option<serde_json::Value> = None;

    for page in 1..=MAX_PAGES {
        let resp = client
            .get_markets(GetMarketsParams {
                limit: Some(100),
                status: Some(MarketStatus::Open),
                mve_filter: Some(MveFilter::Exclude),
                cursor: cursor.clone(),
                ..Default::default()
            })
            .await?;

        println!("Page {}: {} markets", page, resp.markets.len());

        // Find first market meeting volume threshold
        if let Some(m) = resp.markets.into_iter().find(|m| get_volume(m) > MIN_VOLUME_24H) {
            target_market = Some(m);
            break;
        }

        // Check if there are more pages
        match resp.cursor {
            Some(c) if !c.is_empty() => cursor = Some(c),
            _ => {
                println!("No more pages to fetch");
                break;
            }
        }
    }

    let target_market = match target_market {
        Some(m) => m,
        None => {
            println!("No market found with 24h volume > {MIN_VOLUME_24H} after {MAX_PAGES} pages");
            return Ok(());
        }
    };

    let event_ticker = target_market
        .get("event_ticker")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Market missing event_ticker"))?;

    let volume = get_volume(&target_market);

    println!(
        "Found event: {} (first market 24h volume: {})",
        event_ticker, volume
    );

    // Step 3: Fetch all markets for this event
    let event_markets = client
        .get_markets(GetMarketsParams {
            limit: Some(100),
            event_ticker: Some(vec![event_ticker.to_string()]),
            status: Some(MarketStatus::Open),
            ..Default::default()
        })
        .await?;

    let market_tickers: Vec<String> = event_markets
        .markets
        .iter()
        .filter_map(|m| m.get("ticker").and_then(|v| v.as_str()).map(String::from))
        .collect();

    println!(
        "Subscribing to orderbook deltas for {} markets: {:?}",
        market_tickers.len(),
        market_tickers
    );

    // Step 4: Connect authenticated WebSocket
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;

    let mut ws = KalshiWsClient::connect_authenticated(env, auth).await?;

    // Step 5: Subscribe to orderbook deltas for all markets
    let sub_id = ws
        .subscribe(vec![WsChannel::OrderbookDelta], Some(market_tickers))
        .await?;

    println!("Subscribed with id={}, streaming deltas...\n", sub_id);

    // Step 6: Stream and print updates
    loop {
        let envelope = ws.next_envelope().await?;

        match envelope.msg_type.as_str() {
            "orderbook_snapshot" => {
                let snap = envelope.parse_orderbook_snapshot()?;
                println!(
                    "[SNAPSHOT] {} | yes_levels={} no_levels={} | seq={:?}",
                    snap.market_ticker,
                    snap.yes.len(),
                    snap.no.len(),
                    envelope.seq
                );
            }
            "orderbook_delta" => {
                let delta = envelope.parse_orderbook_delta()?;
                println!(
                    "[DELTA] {} | side={} price={} delta={} | seq={:?}",
                    delta.market_ticker, delta.side, delta.price, delta.delta, envelope.seq
                );
            }
            "subscribed" => {
                println!("[SUBSCRIBED] id={:?} sid={:?}", envelope.id, envelope.sid);
            }
            "error" => {
                println!("[ERROR] {:?}", envelope.msg);
            }
            other => {
                println!("[{}] {:?}", other, envelope.msg);
            }
        }
    }
}
