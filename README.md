# kalshi-fast-rs

[![Crates.io](https://img.shields.io/crates/v/kalshi-fast-rs.svg)](https://crates.io/crates/kalshi-fast-rs)
[![Documentation](https://docs.rs/kalshi-fast-rs/badge.svg)](https://docs.rs/kalshi-fast-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance async Rust client for the [Kalshi](https://kalshi.com) Trade API.

## Highlights

- REST parity with Kalshi Trade API `3.8.0` OpenAPI snapshot (`77/77` path+method operations)
- WebSocket parity with current AsyncAPI snapshot, including `user_orders`
- Deterministic REST resilience: retries, exponential backoff+jitter, `429 Retry-After` support
- Builder-based transport controls: timeout, connect timeout, headers, user-agent, proxy, custom `reqwest::Client`
- Explicit WebSocket lifecycle controls: `close()` + configurable `shutdown_timeout(...)`
- Pager/stream/all helpers for cursor endpoints

## Installation

```sh
cargo add kalshi-fast-rs
```

## REST Quick Start (Builder + Retry)

```rust
use std::time::Duration;

use kalshi_fast::{
    KalshiEnvironment, KalshiRestClient, RateLimitConfig, RetryConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = KalshiRestClient::builder(KalshiEnvironment::demo())
        .with_rate_limit_config(RateLimitConfig { read_rps: 30, write_rps: 15 })
        .with_retry_config(RetryConfig {
            max_retries: 4,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(2),
            jitter: 0.2,
            retry_non_idempotent: false,
        })
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(3))
        .build()?;

    let status = client.get_exchange_status().await?;
    println!("exchange_active={}", status.exchange_active);
    Ok(())
}
```

## WebSocket V2 Quick Start (`user_orders`)

```rust
use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannel, WsDataMessage, WsEvent, WsMessage,
    WsReconnectConfig, WsSubscriptionParamsV2,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;

    let mut ws = KalshiWsClient::connect_authenticated(
        KalshiEnvironment::demo(),
        auth,
        WsReconnectConfig::default(),
    ).await?;

    ws.subscribe_v2(WsSubscriptionParamsV2 {
        channels: vec![WsChannel::UserOrders],
        ..Default::default()
    }).await?;

    while let Ok(event) = ws.next_event_v2().await {
        if let WsEvent::Message(WsMessage::Data(WsDataMessage::UserOrder { msg, .. })) = event {
            println!("order={} ticker={} status={:?}", msg.order_id, msg.ticker, msg.status);
        }
    }

    Ok(())
}
```

## Examples

- `examples/rest_retry_config.rs`
- `examples/rfq_quotes_order_groups.rs`
- `examples/ws_user_orders_v2.rs`
- `examples/list_open_markets.rs`
- `examples/orderbook_stream.rs`

## Spec Parity Artifacts

- OpenAPI snapshot: `docs/specs/kalshi/openapi.yaml`
- AsyncAPI snapshot: `docs/specs/kalshi/asyncapi.yaml`
- Parity report: `docs/spec-parity.md`
- Regeneration script: `scripts/generate_spec_parity.py`

## Environment Variables

- `KALSHI_KEY_ID`
- `KALSHI_PRIVATE_KEY_PATH`
- Optional for some examples: `KALSHI_MARKET_TICKER`

## References

- [Kalshi OpenAPI](https://docs.kalshi.com/openapi.yaml)
- [Kalshi AsyncAPI](https://docs.kalshi.com/asyncapi.yaml)
- [Kalshi WebSocket Quickstart](https://docs.kalshi.com/getting_started/quick_start_websockets)

## License

MIT
