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
- The November 2 snapshot includes November 5 and TBD changelog entries.
  Published parsing/subscription support is not evidence of their live rollout.
  [Settlement floors](https://docs.kalshi.com/getting_started/settlement_bounds.md)
  are explicitly not live yet; their metadata does not authorize floor trading.

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

- Deprecated-but-present fields (`no_sub_title`, Perps `exchange_index`, and
  legacy WebSocket direction and timestamp fields) are `Option` plus
  `#[deprecated]`, even where the spec lists them as required, so their eventual
  removal is not a second break. Perps ignores `exchange_index`; order groups can
  hold orders on any Perps market. The same treatment applies to the REST
  fields the OpenAPI marks `deprecated`: `Order`/`Fill` `side` and `action`,
  `Trade::taker_side`, `Market` `title` and `subtitle`, `EventData::category`,
  and the request field `CreateRFQRequest::target_cost_centi_cents`.
- Market `expiration_time` is gone from the OpenAPI and from `Market`, ahead of
  the November 19, 2026 removal date in the changelog. Production still
  returned it on November 15; until the server drops it, it lands untyped in
  `Market::extra`. Read `latest_expiration_time`. Order `expiration_time` is
  unrelated.
- Perps `GetMarginBalanceResponse::settled_funds` is `Option` because keys
  restricted to one subaccount do not receive the account-wide total.
- The per-user `user-ws` hosts (`fill` and `user_orders` today) are documented
  in prose, not the AsyncAPI; `KalshiEnvironment::user_ws_url()` and
  `margin_user_ws_url()` are conveniences, not spec-derived values. The
  `warp-ws` helpers remain, `#[deprecated]`, while Kalshi keeps those hosts up.
- `ebattle_scores` subscriptions start empty: `subscribe` accepts no match
  filter, so matches are selected afterwards with `subscribe_matches`. For that
  reason match filters are not folded into replayable subscription state;
  after a reconnect the channel is resubscribed empty and the caller must send
  `subscribe_matches` again. The score state in `data` is a JSON-encoded string
  and is left undecoded. Public access to the feed is temporary.
- On-request markets (`KXRFQ` series) come from the weather-point and
  concert-cancellation endpoints. Their 409 `*_market_unavailable` and 429
  `*_market_limit_exceeded` refusals are distinguished only by the error
  `code`. Those 429s are not request-rate limiting, yet the client retries on
  status alone: the creation limits when `retry_non_idempotent` is set, and
  the `GET` search limit (`concert_search_limit_exceeded`) by default. The
  changelog says concert markets are not live yet.
- REST `X-RateLimit-*` response headers are not surfaced. A 429 carries no
  `Retry-After`, so retries fall back to the configured exponential backoff,
  which is what Kalshi recommends.

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

## Verification boundaries

Deterministic behavior checks cover parsing, command validation, subscription
intent/replay and mock transports. Network-gated callers must also compile via
`cargo test --all-targets --features live-tests --no-run`. Actual demo reads
and authenticated subscriptions exercise the exchange independently; offline
tests and schema alignment do not prove production deployment or write-path
qualification. Never use `--all-features` as an offline test command.

The November 2 refresh directly exercised public demo REST market, event,
series, multi-orderbook, trade, event-fee-change and historical-cutoff reads.
It also replayed 45 published event-contract and 3 published margin server
examples, plus 12 captured control/orderbook frames. Published-example replay
is not live-feed qualification.

The November 15 refresh compared top-level response keys from unauthenticated
production reads of markets, perps markets and trades, incentive programs and
schedules, and the weather and concert availability endpoints against the new
models; it did not run the crate's live test suites. The demo hosts were
unreachable from that environment. The new `ebattle_scores` frames, perps
WebSocket `is_block_trade`, `netting_enabled`, `key_type` and the
subaccount-restricted balance shape are aligned to the YAML only.

Authenticated demo REST and WebSocket probes could not run in either refresh
because the checkout lacked `.env.test` and `test_key.pem`, with no
authentication variables set. No order, quote, funding or administrative
mutation was exercised; authenticated and perps runtime qualification remains
separate.
