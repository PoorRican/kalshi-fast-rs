/// Example of margin/perpetuals REST API usage for KXBTCPERP1:
///   1. Check if margin trading is enabled
///   2. List active margin markets
///   3. Fetch balance and positions
use kalshi_fast::{
    GetMarginPositionsParams, KalshiAuth, KalshiEnvironment, KalshiRestClient, MarginMarketStatus,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = KalshiEnvironment::demo();
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;
    let client = KalshiRestClient::new(env).with_auth(auth);

    let enabled = client.get_margin_enabled().await?;
    println!("margin trading enabled: {enabled:#?}");

    let markets = client
        .get_margin_markets(Some(MarginMarketStatus::Active))
        .await?;
    println!("active margin markets: {markets:#?}");

    let balance = client.get_margin_balance(Some(true)).await?;
    println!("margin balance: {balance:#?}");

    let positions = client
        .get_margin_positions(GetMarginPositionsParams::default())
        .await?;
    println!("open positions: {positions:#?}");

    Ok(())
}
