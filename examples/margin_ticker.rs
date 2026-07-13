/// Example: Subscribe to the margin/perpetuals WebSocket ticker channel
/// for a specific market.
///
/// Requires KALSHI_KEY_ID and KALSHI_PRIVATE_KEY_PATH env vars (or .env file).
///
/// Usage:
///   KALSHI_KEY_ID=... KALSHI_PRIVATE_KEY_PATH=... cargo run --example margin_ticker -- PERP_MARKET
use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, MarginChannel, MarginSubscribeParams, MarginWsClient, WsEvent,
    WsReconnectConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = KalshiEnvironment::demo();
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;

    let market_ticker = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "KXBTCPERP1".to_string());

    let mut ws =
        MarginWsClient::connect_authenticated(env, auth, WsReconnectConfig::default()).await?;

    ws.subscribe(MarginSubscribeParams {
        channels: Some(vec![MarginChannel::Ticker]),
        market_tickers: Some(vec![market_ticker.clone()]),
        sub_account_id: None,
    })
    .await?;

    println!("streaming ticker for {market_ticker}...");

    loop {
        match ws.next_event().await? {
            WsEvent::Message(msg) => println!("{msg:#?}"),
            WsEvent::Reconnected { attempt } => {
                println!("reconnected (attempt {attempt})")
            }
            WsEvent::Disconnected { error } => {
                println!("disconnected: {error:?}");
                break;
            }
            WsEvent::Raw(_) => {}
        }
    }

    Ok(())
}
