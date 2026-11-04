# Kalshi Rust Client

## Spec Notes
- Spec notes: `docs/spec-parity.md`

The unreleased `0.12.0` refresh is source-breaking (new `MarketPosition::netting_enabled` field; since `0.11.0`, deprecated `no_sub_title` and
Perps `exchange_index` are now optional and `#[deprecated]`) on top of the `0.10.0` cutover. The
compatibility tuple in [`CHANGELOG.md`](../CHANGELOG.md) records the exact
published Predictions and perps specifications, rather than treating the crate
version or unchanged AsyncAPI version as a freshness signal.

## WebSocket Auth
Kalshi WebSocket connections require authentication, even when subscribing to public channels. Use `KalshiWsClient::connect_authenticated` and provide `KALSHI_KEY_ID` and `KALSHI_PRIVATE_KEY_PATH`.

Authentication accepts Ed25519 PKCS#8 and RSA PKCS#8/PKCS#1 PEM keys. Kalshi's
web key generator now defaults to Ed25519; existing RSA keys remain supported.
The same canonical signed path and headers apply to both algorithms.

## Environment Variables
The examples load environment variables via `dotenvy`. Set these in your shell or a `.env` file:
- `KALSHI_KEY_ID`
- `KALSHI_PRIVATE_KEY_PATH`

Integration tests use demo credentials from `.env.test`; production credentials
in `.env` are not interchangeable. Order, order-group, and RFQ lifecycle suites
mutate demo state and are not part of the offline gate.

## Pagination & Truncation
There are two pagination styles:

- **Pager (page-level)**: `CursorPager<T>` yields full pages (`Vec<T>`). You control when to stop by **not** calling `next_page()` again. This is ideal when you want to checkpoint the cursor between calls.
- **Stream (item-level)**: `stream_*` yields items one-by-one. You can stop early with `.take(n)` or by providing `max_items`, and it won’t fetch extra pages once that limit is hit.

Examples:

Page-level:
```rust
let mut pager = client.markets_pager(GetMarketsParams::default());
while let Some(page) = pager.next_page().await? {
    for market in page {
        println!("{}", market.ticker);
    }
}
```

Item-level:
```rust
use futures::stream::TryStreamExt;

let markets: Vec<_> = client
    .stream_markets(GetMarketsParams::default(), Some(250))
    .try_collect()
    .await?;
```

Note: `get_*_all` loads **all pages into memory** and is intended for convenience only.

## WebSocket Reconnect
`KalshiWsClient` is the high-level WebSocket client with auto reconnect + resubscribe. It emits explicit connection events:

- `WsEvent::Message(...)`
- `WsEvent::Raw(...)` (when using `WsReaderMode::Raw`)
- `WsEvent::Reconnected { attempt }`
- `WsEvent::Disconnected { error }` (the error names the transport cause: keepalive timeout, peer Close code and reason, or the last reconnect failure)

Reconnect uses exponential backoff with jitter, resubscribes to active channels by default, and **does not** attempt sequence resync (callers must handle any gaps).

Application commands and reconnect replays share one command-ID sequence.
Multi-channel acknowledgements track each channel independently, and replay
keeps channel-restricted options only on their owning channels, including
requests that were only partially acknowledged before disconnecting.
Channel-independent acknowledgement options such as `skip_ticker_ack` remain
set regardless of channel.
`Reconnected` means subscriptions were resent, not that their acknowledgements
or replacement orderbook snapshots have arrived.

Orderbook `clock` and envelope `sending_ts_ms` are not replacements for
per-subscription `seq` or the timed reader's process-local availability time.
In real-time mode, several deltas can share a clock; in conflated mode clocks
can decrease and cannot deduplicate independent connections. Apply snapshots
as replacements. A repeated orderbook subscribe adds markets to the same SID
but does not resynchronize existing markets; request fresh snapshots or
unsubscribe/acknowledge/resubscribe after a gap.

`use_yes_price` is an explicit subscription option, not an implicit price
conversion by this client. Kalshi currently defaults it to false and plans a
future default flip followed by removal of the flag. Existing NO-leg consumers
must coordinate their conversion and persisted data regime before opting in.
Likewise, `update_interval_ms` remains opt-in; accepting its published schema
does not establish that its future-dated rollout has occurred.

## Migrating from 0.9

- Replace V1 order mutations with `create_order_v2`, `cancel_order_v2`,
  `amend_order_v2`, `decrease_order_v2`, and the V2 batch methods. V2 prices and
  counts are fixed-point strings and order sides use the YES book's `bid`/`ask`
  vocabulary. Supply the market ticker for automatic shard routing; an order
  ID alone cannot identify its shard.
- Use parameter objects for balance, API-key listing and event detail:
  `get_balance(GetBalanceParams::default())`,
  `get_api_keys(GetApiKeysParams::default())`, and
  `get_event(ticker, GetEventParams::default())`. Series detail takes an
  `include_volume` option; subaccount creation accepts an optional request body.
- Read normalized `outcome_side`/`book_side` or `taker_*` direction fields.
  Removed integer price/count fields, liquidity, broker availability,
  announcement/lookup types and compatibility aliases are no longer provided.
  Use canonical nested RFQ quote methods with both RFQ and quote IDs.
- Market and position event filters accept one event; order event filters and
  fill market filters support their documented lists. Subaccount identities
  and `exchange_index` are independent, and a balance may aggregate shards.
- For complete position history, page live positions with
  `settlement_status=all` first, then historical positions for the same
  subaccount, deduplicating by subaccount and ticker. The historical cutoff is
  not proof that a whole event has completed its archival handoff. Archived
  positions do not reproduce every field of live settlement records.

## Consumer budgeting

`RateLimitConfig` is a request-rate throttle, not automatic endpoint-token,
shard-write or balance-bucket quota enforcement. Consumers must budget against
their current account limits and endpoint costs.

## Deterministic vs Live Tests
- Deterministic tests run by default: `cargo test --all-targets`
- Live integration tests are feature-gated:
  - `cargo test --features live-tests --test rest_public`
  - `cargo test --features live-tests --test rest_auth`
  - `cargo test --features live-tests --test ws_public`
  - `cargo test --features live-tests --test ws_auth`

- Compile network-gated callers without executing them:
  `cargo test --all-targets --features live-tests --no-run`
- The offline CI additionally checks formatting, clippy, build, and default
  and `timed-reader` tests. Do not run `--all-features` as an offline test gate:
  `live-prod-tests` includes production-network probes.
