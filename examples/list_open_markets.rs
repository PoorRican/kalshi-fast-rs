/// Example of using the Public REST endpoints: lists open markets
use kalshi_fast::{GetMarketsParams, KalshiEnvironment, KalshiRestClient, MarketStatusQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = KalshiEnvironment::demo();
    let client = KalshiRestClient::new(env);

    let resp = client
        .get_markets(GetMarketsParams {
            limit: Some(1),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        })
        .await?;

    println!("markets: {}", resp.markets.len());
    Ok(())
}
