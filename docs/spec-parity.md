# Spec Notes

This repository follows Kalshi's published OpenAPI and AsyncAPI documents
directly.

Those upstream specs are the baseline for contract review, but they do not
fully define every public behavior in the crate. The most important behavior
checks live in tests, especially where the YAML specs are underspecified or
examples are ambiguous.

## Known Distinctions

- `MarketStatusQuery` is the query/filter enum used by list endpoints.
- `MarketStatus` is the lifecycle/status enum returned on market objects.
- They overlap, but they are not one-to-one. Lifecycle states such as
  `determined`, `disputed`, and `amended` collapse differently when converted
  into query status. The conversion behavior is covered in `tests/parsing.rs`.

- The AsyncAPI examples imply both singular and plural market ticker fields for
  websocket subscriptions.
- The crate accepts `market_ticker` or `market_tickers`, but not both.
- `orderbook_delta` requires market tickers and rejects `market_id` and
  `market_ids`.
- `skip_ticker_ack` is supported on subscription updates.
- These behaviors are covered by `tests/ws_command_behavior.rs` and
  `tests/ws_parsing.rs`.

- The AsyncAPI spec marks `ts_ms` as required on both the `trade` and
  `ticker` channel messages (`WsTrade`, `WsTicker`).
- In practice the field is occasionally omitted by the exchange. Consumers
  should treat `ts_ms` as best-effort and fall back to `ts` (seconds) when
  precise millisecond timing matters.

- The `side` and `action` fields on `Order`, `Fill`, and `WsFill` were deprecated by Kalshi on
  2026-05-07. The new normalized fields are `outcome_side` (`yes` | `no`) and `book_side`
  (`bid` | `ask`), where `bid` ≡ `yes` and `ask` ≡ `no`. The OpenAPI/AsyncAPI specs still mark the
  legacy fields required ("not removed before May 14, 2026"), but the changelog scheduled removal
  for 2026-05-28. To survive either state, the legacy fields are modeled as `Option`, and the new
  normalized fields are also `Option` so older payloads (lacking them) still parse.
- The public `Trade` object (REST `Trade`, WebSocket `WsTrade`) uses the taker-prefixed variants:
  `taker_side` (deprecated) plus `taker_outcome_side` / `taker_book_side`. These follow the same
  `Option` treatment for the same reasons.

- The `/margin/fee_tiers` response was restructured on 2026-05-11. The previous tier-name maps
  (`maker_fee_tiers`, `taker_fee_tiers`) were replaced by per-ticker decimal-rate maps
  (`maker_fee_rates`, `taker_fee_rates`). Fee is computed as `notional * rate`.

- `event_fee_update` is an AsyncAPI message delivered on the `market_lifecycle_v2` channel (it is
  not a separately-subscribable channel). It is modeled by `WsEventFeeUpdate`. `fee_type_override`
  is kept as `Option<String>` rather than reusing the `FeeType` enum so the raw string survives any
  future fee-type additions without a crate update. Both override fields are nullable (`None` when
  the override is cleared).

- `FeeType` enum now includes `QuadraticWithMakerFees` (serialized `quadratic_with_maker_fees`),
  added to the OpenAPI spec in 2026. An `#[serde(other)] Unknown` catch-all is also present so
  unrecognised future variants never panic during deserialization. `fee_type_override` on
  `WsEventFeeUpdate` remains `Option<String>` for lossless round-trip regardless.

- `is_block_trade: bool` was added to the public REST `Trade` struct (2026-05-29). The field is
  `#[serde(default)]` (defaults to `false`) so payloads predating the flag still parse. The query
  filter `GetTradesParams::is_block_trade: Option<bool>` lets callers filter by block-trade status.

- `GET /account/limits` (`get_account_api_limits`) response was restructured in 2026-06 (automated
  API rate-limit tiers). The old flat shape (`read_limit: i64, write_limit: i64`) was replaced by
  nested `BucketLimit` objects (`read: BucketLimit, write: BucketLimit`) plus a `grants:
  Vec<ApiUsageLevelGrant>` array. The `GetAccountApiLimitsResponse` struct was updated accordingly;
  old field access will not compile (intentional minor-version break, 0.5.0 → 0.6.0).
  `ApiUsageLevelGrant.expires_ts` is `Option<i64>` because the field is absent for non-expiring
  grants.

- `cfbenchmarks_value` is a new AsyncAPI channel (introduced 2026-06) that delivers CF Benchmarks
  index values. It uses `index_ids` (not market tickers) for subscription parameters; pass
  `["all"]` to receive all available indices. The channel emits two message types:
  `cfbenchmarks_value` (per-index value + 60-second windowed average) and
  `cfbenchmarks_value_indexlist` (the full set of available index IDs). Both are modeled as
  `WsCfBenchmarksValue` / `WsCfBenchmarksIndexList` and routed through the standard
  `WsDataMessageV2` enum. `last_60s_windowed_average_15min` on `WsCfBenchmarksValue` is `Option`
  because the spec marks it conditional. The documented post-subscribe workflow (discover indices
  via `indexlist`, then add/remove with `subscribe_indices` / `unsubscribe_indices`) is supported
  through `update_subscription_v2` using the `WsUpdateAction::SubscribeIndices` /
  `UnsubscribeIndices` / `Indexlist` actions plus the `index_ids` field on
  `WsUpdateSubscriptionParamsV2`. `validate_update` rejects mixing index actions with market targets
  and requires `index_ids` for the add/remove actions, matching the AsyncAPI error semantics.

- `GET /account/endpoint_costs` (`get_account_endpoint_costs`) is modeled as a public (unauthed)
  endpoint because the OpenAPI operation declares no `security` requirement, unlike `/account/limits`.
  `ApiUsageLevelGrant.exchange_instance` is kept as `String` rather than an `ExchangeInstance` enum
  (`event_contract` | `margined`); the raw string round-trips losslessly and tolerates any future
  exchange-instance values without a crate update.
- The AsyncAPI marks several timestamp/required fields that the exchange may omit in practice
  (`ts_ms` on ticker/trade/order-group messages, the legacy direction fields). These are modeled as
  `Option` so parsing never fails on their absence.

- `exchange_index` (an integer exchange-shard identifier) was added by Kalshi across most
  REST and WebSocket surfaces during 2026-07 through 2026-09 as part of exchange sharding.
  It is modeled as `Option<u32>` everywhere except where the OpenAPI/AsyncAPI schema's
  `required` list makes it unconditional (`Fill`, `Settlement`, `MarketPosition`,
  `SubaccountBalance` are non-optional `u32`); this keeps the crate tolerant of shards
  that predate the field. `Series` previously had no `extra` catch-all field at all (unlike
  every sibling response struct), so unrecognized fields — including `exchange_index` before
  it was promoted to a typed field — were being silently dropped by serde; a flatten `extra`
  map was added for consistency with the rest of the crate.

- `WsEventLifecycle.exchange_index` and `WsTrade.is_block_trade` were parsing correctness bugs,
  not new-field additions: both fields are spec-required on their respective messages, but
  neither struct had a flatten `extra` catch-all, so the values were being silently dropped
  rather than merely left untyped. Both are now typed fields, covered by regression tests
  that fail on the pre-fix code.

- `cfbenchmarks_value` (the original 1Hz CF Benchmarks channel) is a public market-data feed;
  `cfbenchmarks_value_5hz` and `pyth_value` are paid/authenticated add-on feeds per their
  AsyncAPI channel descriptions. `WsChannelV2::is_private()` reflects this: the two paid
  channels return `true` (client-side guard requiring configured auth before subscribing),
  while `cfbenchmarks_value` returns `false`, consistent with other public market-data
  channels (`ticker`, `trade`, `market_lifecycle_v2`). This is unrelated to the WebSocket
  connection-level handshake, which always requires authentication regardless of channel
  (see `GOTCHAS.md`).

- The legacy (non-V2) order-mutation REST endpoints (`create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`), the
  multivariate-event-collection lookup endpoints (REST and the WebSocket `multivariate`/
  `multivariate_lookup` channel), and `GET /exchange/announcements` were all confirmed
  fully absent from the current OpenAPI/AsyncAPI specs (not merely flagged deprecated) and
  were removed from the crate rather than kept as dead code that would 404/never-match at
  runtime. The `_v2` order methods and `multivariate_market_lifecycle` remain fully supported.

- **Known gap — Ed25519 API keys (added to the exchange 2026-09-24):** the REST surface
  (`ApiKeyType`, `key_type` on `GenerateApiKeyRequest`/`GenerateApiKeyResponse`) is modeled,
  but request *signing* with an Ed25519 key is not implemented. `src/auth.rs` remains
  RSA-PSS/SHA256-only end to end (REST headers and the WebSocket handshake pre-sign text).
  Adding Ed25519 support requires a second signing backend (e.g. `ed25519-dalek`) selected
  by key type, touching `auth.rs`, the WS handshake, and (out of scope for this crate) FIX
  `RawData` signing. Tracked as a follow-up rather than attempted piecemeal.

- **Known gap:** `CreateApiKeyRequest`/`GenerateApiKeyRequest` also carry an `fcm_subtrader_id`
  field, and `CreateApiKeyResponse`/`GenerateApiKeyResponse` carry a `warning` field, per the
  live OpenAPI schema. Neither is modeled yet.

- **Known gap:** the AsyncAPI spec marks `rfq_creator_id` required on both `quoteCreatedPayload.msg`
  and `quoteAcceptedPayload.msg`, but only `WsQuoteExecuted` has it modeled today. Pre-existing
  gap, not introduced by the 2026-09 refresh.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
