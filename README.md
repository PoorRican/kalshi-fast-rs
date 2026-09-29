# kalshi-fast-rs

[![Crates.io](https://img.shields.io/crates/v/kalshi-fast-rs.svg)](https://crates.io/crates/kalshi-fast-rs)
[![Documentation](https://docs.rs/kalshi-fast-rs/badge.svg)](https://docs.rs/kalshi-fast-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance async Rust client for the [Kalshi](https://kalshi.com) Trade API.

## Highlights

- REST parity with the current Kalshi Trade API OpenAPI surface
- WebSocket parity with the current AsyncAPI surface, including `user_orders`
- Deterministic REST resilience: retries, exponential backoff+jitter, `429 Retry-After` support
- Builder-based transport controls: timeout, connect timeout, headers, user-agent, proxy, custom `reqwest::Client`
- Explicit WebSocket lifecycle controls: `close()` + configurable `shutdown_timeout(...)`
- Pager/stream/all helpers for cursor endpoints

## Release Notes and Versioning

- [`CHANGELOG.md`](CHANGELOG.md) records release history, compatibility blocks, and categorized change notes.
- [`VERSIONING.md`](VERSIONING.md) defines crate version bump rules and explains how upstream Kalshi compatibility is tracked separately from the Cargo version.

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
    KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannelV2, WsDataMessageV2, WsEvent, WsMessageV2,
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
        channels: vec![WsChannelV2::UserOrders],
        ..Default::default()
    }).await?;

    while let Ok(event) = ws.next_event_v2().await {
        if let WsEvent::Message(WsMessageV2::Data(WsDataMessageV2::UserOrder { msg, .. })) = event {
            println!("order={} ticker={} status={:?}", msg.order_id, msg.ticker, msg.status);
        }
    }

    Ok(())
}
```

## Optional Timed Reader

The `timed-reader` Cargo feature is disabled by default. Enable it when a background WebSocket
reader needs the process-local availability time captured before its bounded handoff:

```toml
kalshi-fast-rs = { version = "0.7.1", features = ["timed-reader"] }
```

The feature adds an owned WebSocket event delivery API that includes this availability timestamp.
Existing WebSocket event-reading APIs remain available without enabling the feature.

## Examples

- `examples/rest_retry_config.rs`
- `examples/rfq_quotes_order_groups.rs`
- `examples/ws_user_orders_v2.rs`
- `examples/list_open_markets.rs`
- `examples/orderbook_stream.rs`

## Spec Notes

- Notes on spec-to-crate distinctions: `docs/spec-parity.md`

## Environment Variables

- `KALSHI_KEY_ID`
- `KALSHI_PRIVATE_KEY_PATH`
- Optional for some examples: `KALSHI_MARKET_TICKER`

## References

- [Project changelog](CHANGELOG.md)
- [Project versioning policy](VERSIONING.md)
- [Kalshi OpenAPI](https://docs.kalshi.com/openapi.yaml)
- [Kalshi AsyncAPI](https://docs.kalshi.com/asyncapi.yaml)
- [Kalshi WebSocket Quickstart](https://docs.kalshi.com/getting_started/quick_start_websockets)

## License

MIT
