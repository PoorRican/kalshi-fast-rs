use std::time::Duration;

use kalshi_fast::{KalshiEnvironment, KalshiRestClient, RateLimitConfig, RetryConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = KalshiRestClient::builder(KalshiEnvironment::demo())
        .with_rate_limit_config(RateLimitConfig {
            read_rps: 30,
            write_rps: 15,
        })
        .with_retry_config(RetryConfig {
            max_retries: 4,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(2),
            jitter: 0.2,
            retry_non_idempotent: false,
        })
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(3))
        .with_user_agent("kalshi-fast-rs/example-rest-retry")
        .build()?;

    let status = client.get_exchange_status().await?;
    println!(
        "exchange_active={} trading_active={}",
        status.exchange_active, status.trading_active
    );

    Ok(())
}
