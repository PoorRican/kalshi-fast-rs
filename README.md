# kalshi-fast-rs

[![Crates.io](https://img.shields.io/crates/v/kalshi-fast-rs.svg)](https://crates.io/crates/kalshi-fast-rs)
[![Documentation](https://docs.rs/kalshi-fast-rs/badge.svg)](https://docs.rs/kalshi-fast-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance async Rust client for the [Kalshi](https://kalshi.com) Trade API.

## Highlights

- Predictions and perps REST aligned to Kalshi's published specifications
- Typed owned/borrowed WebSocket messages, including private orders and external reference feeds
- Deterministic REST resilience: retries, exponential backoff+jitter, `429 Retry-After` support
- Builder-based transport controls: timeout, connect timeout, headers, user-agent, proxy, custom `reqwest::Client`
- Explicit WebSocket lifecycle controls: `close()` + configurable `shutdown_timeout(...)`
- Pager/stream/all helpers for cursor endpoints
- RSA-PSS and Ed25519 authentication with reusable signing keys

## Release Notes and Versioning

- [`CHANGELOG.md`](CHANGELOG.md) records release history, compatibility blocks, and categorized change notes.
- [`VERSIONING.md`](VERSIONING.md) defines crate version bump rules and explains how upstream Kalshi compatibility is tracked separately from the Cargo version.

The unreleased `0.12.0` refresh uses the November 12, 2026 documentation snapshot
(Predictions OpenAPI `3.36.0`; AsyncAPI `2.0.0`). See the changelog for the full
compatibility tuple and [migration guidance](docs/README.md). Published
future-dated schemas are not a claim that Kalshi has deployed those features.
Retired APIs, deprecated quote-detail aliases, unpublished `x-excluded` REST
operations, FIX, and the separate Clearing Member API are not exposed.

## Installation

For the latest published release:

```sh
cargo add kalshi-fast-rs
```

To use the unreleased `0.12.0` API from an adjacent local checkout instead:

```sh
cargo add kalshi-fast-rs --path ../kalshi-fast-rs
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
kalshi-fast-rs = { path = "../kalshi-fast-rs", features = ["timed-reader"] }
```

The feature adds an owned WebSocket event delivery API that includes this availability timestamp.
Existing WebSocket event-reading APIs remain available without enabling the feature.

## Examples

- `examples/rest_retry_config.rs`
- `examples/rfq_quotes_order_groups.rs`
- `examples/ws_user_orders.rs`
- `examples/list_open_markets.rs`
- `examples/orderbook_stream.rs`

## Spec Notes

- Notes on spec-to-crate distinctions: `docs/spec-parity.md`

## Environment Variables

- `KALSHI_KEY_ID`
- `KALSHI_PRIVATE_KEY_PATH`
- Optional for some examples: `KALSHI_MARKET_TICKER`

`KalshiAuth::from_pem_file` and `from_pem_str` detect Ed25519 PKCS#8 and RSA
PKCS#8/PKCS#1 keys from their contents. A `BEGIN PRIVATE KEY` PEM header alone
does not identify the algorithm.

## References

- [Project changelog](CHANGELOG.md)
- [Project versioning policy](VERSIONING.md)
- [Kalshi OpenAPI](https://docs.kalshi.com/openapi.yaml)
- [Kalshi AsyncAPI](https://docs.kalshi.com/asyncapi.yaml)
- [Kalshi WebSocket Quickstart](https://docs.kalshi.com/getting_started/quick_start_websockets)

## License

MIT
