# Spec Notes

The compatibility tuple in [`CHANGELOG.md`](../CHANGELOG.md) identifies the
published schema snapshot. Kalshi can change payloads without changing its
AsyncAPI `info.version`; the snapshot date matters.

## Surface and rollout boundaries

- Predictions and perps use their separate Trade REST and WebSocket specs:
  [OpenAPI](https://docs.kalshi.com/openapi.yaml),
  [AsyncAPI](https://docs.kalshi.com/asyncapi.yaml),
  [perps OpenAPI](https://docs.kalshi.com/perps_openapi.yaml), and
  [perps AsyncAPI](https://docs.kalshi.com/perps_asyncapi.yaml).
- The separate Clearing Member `/klear-api/v1` and FIX APIs are not implemented.
  Hidden `x-excluded` REST operations are not a supported public contract.
  Deprecated unnested quote-detail aliases are deliberately replaced by the
  canonical RFQ-scoped operations; retired order mutations, announcements and
  multivariate lookups are removed rather than shimmed.
- Changelog entries can be dated ahead of the snapshot (or TBD), and the YAML
  can lead production. Published parsing/subscription support is not evidence
  of live rollout.
  [Settlement floors](https://docs.kalshi.com/getting_started/settlement_bounds.md)
  are explicitly not live yet; their metadata does not authorize floor trading.
- On November 14 the YAML had already dropped market `expiration_time`, which
  the changelog removes on November 19; production still returned it. The
  field is gone from `Market` and lands in `Market::extra` until Kalshi stops
  sending it. Production also ignored the new `series_ticker` filter on
  `GET /incentive_programs` while already returning the field. Concert
  cancellation markets are announced as not live yet.

## Intentional parsing distinctions

- `MarketStatusQuery` filters and response `MarketStatus` lifecycle values are
  different taxonomies. The `paused` filter maps to `Inactive`; `determined`,
  `disputed`, and `amended` map to the closed filter. The Get Markets prose
  summary omits `paused`, but its referenced query schema and the
  [lifecycle guide](https://docs.kalshi.com/getting_started/market_lifecycle.md)
  explicitly support it.
- Market/event objects retain partial-response tolerance where previously
  supported, but removed fields are not synthesized. Current REST normalized
  trade/order/fill direction fields are modeled directly. Legacy direction
  fields still present as deprecated in the specs remain optional; read the
  normalized fields instead. See [order direction](https://docs.kalshi.com/getting_started/order_direction.md).
- Fixed-point values remain strings. Live candles use suffixed dollar/count
  fields; historical candle distributions have their own unsuffixed
  fixed-point dollar fields. Do not substitute integer cents or assume the two
  response schemas are identical.
- Current orderbook snapshots/deltas specify `clock`. Owned and borrowed
  payloads retain `Option<i64>` so the captured pre-clock frames in
  `tests/fixtures/control_frame_mutation.jsonl` still decode. An absent clock
  stays absent; it is never fabricated as zero.
- Ticker/trade `ts_ms` remains best-effort because captured exchange messages
  can omit it. Deprecated `ts` may also be absent; neither timestamp is guaranteed.
  `sending_ts_ms` is a separate server-transmission timestamp, not event time
  or the timed reader's process-local availability time.
- Unknown event-contract frames retain their raw payload. Both event and perps
  unknown frames retain subscription cursor metadata; future message types
  are not a reason to discard `sid`/`seq`.
- `event_fee_update` belongs to `market_lifecycle_v2`, not a separate channel.
  Its nullable override strings preserve raw fee-type values; REST `FeeType`
  also includes the published maker/combo-maker variants and an unknown case.
- Rate-limit grants may omit `expires_ts` for permanent grants. Exchange-lane
  identity stays a string. `GET /account/endpoint_costs` is public as specified;
  its costs are tokens, not requests per second.

- Deprecated-but-present fields (`no_sub_title`, Perps `exchange_index`,
  legacy REST and WebSocket direction fields, legacy WebSocket timestamps,
  `EventData::category`, and `Market::title`/`subtitle`) are `Option` plus
  `#[deprecated]`, even where the spec lists them as required, so their eventual
  removal is not a second break. Perps ignores `exchange_index`; order groups can
  hold orders on any Perps market. `CreateRFQRequest::target_cost_centi_cents`
  is deprecated upstream but carries only a doc note: the request has no
  `Default`, so the attribute would warn on every construction.
- Required-but-conditional fields are `Option`: perps `settled_funds` is
  omitted for subaccount-restricted keys. `MarketPosition::netting_enabled` is
  required as published; it was not exercised against an authenticated
  account.
- Fill/order-only `user-ws` hosts (and the deprecated `warp-ws` hosts) are
  documented in prose, not the AsyncAPI; `KalshiEnvironment::user_ws_url()` is
  a convenience, not a spec-derived value.
- `X-RateLimit-*` response headers are prose-only and not surfaced. A `429`
  carrying a code other than `too_many_requests` is a capacity limit (open
  RFQs, on-request market creation, concert search) and is never retried.
- On-request enums (`WeatherPointVariable`, `ConcertCover`) keep an `Unknown`
  case for values in availability responses; it is rejected in requests.

## Subscription semantics

- Market ticker and ID filters follow each channel's documented vocabulary;
  orderbooks require tickers. CF reference feeds use `index_ids`, while Pyth
  uses `underlying_tickers`; their list actions do not mutate tracked selection.
- Pricing mode and conflation are explicit options. The client does not
  convert NO-leg prices, enable conflation, or infer book completeness from a
  clock. Follow each subscription's sequence and replace books on snapshots.
- The current `use_yes_price` default is false, with an announced future
  default flip and eventual flag removal but no concrete dates. Conflation
  accepts only the published 0/100/1000 millisecond options. See
  [orderbook updates](https://docs.kalshi.com/websockets/orderbook-updates.md)
  for same-SID repeated subscriptions and recovery rules.

- `ebattle_scores` has no subscribe-time seed: a new subscription is empty
  until `subscribe_matches` is sent, so reconnect replay restores the
  subscription but not its match filter. Resend `subscribe_matches` after the
  replayed `subscribed` acknowledgement. `msg.data` stays a JSON-encoded string.

## Verification boundaries

Deterministic behavior checks cover parsing, command validation, subscription
intent/replay and mock transports. Network-gated callers must also compile via
`cargo test --all-targets --features live-tests --no-run`. Actual demo reads
and authenticated subscriptions exercise the exchange independently; offline
tests and schema alignment do not prove production deployment or write-path
qualification. Never use `--all-features` as an offline test command.

The November 14 refresh read public production endpoints only: markets,
events with nested markets, margin markets and trades, incentive programs and
schedules, and weather and concert availability all decoded through the crate.
Demo hosts were unreachable from the refresh environment, so no demo read ran.

No authenticated REST or WebSocket probe ran (no credentials in the checkout),
so positions, API keys, perps balances, concert listings, the create
operations and the `ebattle_scores` feed are aligned to the published schemas
only. No order, quote, funding or administrative mutation was exercised;
authenticated and perps runtime qualification remains separate.
