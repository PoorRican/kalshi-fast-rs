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

- `EventData.available_on_brokers` was deprecated on 2026-08-27: it is no longer populated by the
  exchange and always returns `false`. The Rust field carries `#[deprecated]` in addition to the
  existing `Option<bool>` typing so downstream reads get a compiler warning pointing at the
  replacement behavior, without a breaking removal while the field remains present on the wire.

- `EventData.product_metadata` is documented in the OpenAPI spec only as an opaque `object` (no
  schema ref), yet the crate reuses the `EventMetadata` struct (the shape of the dedicated
  `GET /events/{event_ticker}/metadata` response) for it. `EventMetadata.cadence: Option<String>`
  was added for the 2026-07-30 "event product_metadata now includes cadence" changelog entry, but
  because the OpenAPI schema is opaque for `product_metadata` specifically, the real key name/shape
  is sourced from the changelog prose rather than a machine-checkable schema. Treat `cadence` as
  best-effort.

- `Market.response_price_units` and `Market.fractional_trading_enabled` were removed from the
  OpenAPI schema on 2026-07-09 (deprecated Predictions REST schema field cleanup) and are no longer
  modeled on `Market`. The WebSocket analog (`WsMarketLifecycleV2.fractional_trading_enabled`) is
  also removed, along with the `WsMarketLifecycleEventType::FractionalTradingUpdated` variant (not
  present in the current AsyncAPI `event_type` enum). `MarketPosition.resting_orders_count` was
  removed from the same cleanup.

- `GetOrderGroupResponse.orders` is `Vec<String>` (order IDs), not `Vec<Order>` — this was a
  pre-existing mismatch against the live OpenAPI schema (`items: {type: string}`), fixed while
  reconciling the 2026-08-06 "order group limit updates support subaccounts" entry. The response
  also has no plain `contracts_limit` field, only `contracts_limit_fp`.

- `ErrorResponse.service` was fully removed from the OpenAPI `ErrorResponse` schema on 2026-08-06
  (deprecated 2026-07-28). The field is removed from the Rust struct entirely rather than kept as
  `Option`, since the exchange no longer emits it at all.

- The `multivariate` WebSocket channel (ticker-pair lookups) and its REST counterpart
  (`GET`/`PUT .../lookup`) were deprecated then fully removed from both the OpenAPI and AsyncAPI
  specs on 2026-08-06. The REST client methods and request/response types were removed. On the
  WebSocket side, `WsChannelV2::Multivariate` was removed from the subscribable channel enum (so
  the typed API can no longer construct a subscribe request for it), but the `multivariate` /
  `multivariate_lookup` message parsing in `ws/types/wire.rs` and `ws/types/envelope.rs` was
  deliberately left in place as inert, vestigial tolerance: it can never be triggered by an outgoing
  subscribe (the channel enum variant is gone) and costs nothing to keep, in case a lingering server
  message ever arrives during the deprecation window. `multivariate_market_lifecycle` is a distinct,
  still-live channel and is unaffected.

- `exchange_index` (an `ExchangeIndex` integer, default `0`) was added across most REST response
  objects and several WebSocket messages during 2026-07 through 2026-08 as Kalshi rolled out
  multi-shard exchange instances. Where the OpenAPI/AsyncAPI schema marks the field `required` on an
  established (pre-existing) struct, it is modeled as `#[serde(default)] pub exchange_index: u32`
  rather than `Option<u32>` — `0` is both the documented default shard and a safe fallback for
  payloads captured before the field existed, so this preserves ergonomic non-`Option` access
  without risking a hard deserialization failure against slightly-earlier server versions. Fields
  introduced on brand-new structs (e.g. `IndexedBalance`, `TargetBalanceAllocation`,
  `ExchangeIndexStatus`) are modeled as plain required `u32` since there is no legacy payload shape
  to tolerate.

- `WsFill.purchased_side` is marked `deprecated: true` in the current AsyncAPI `fillPayload` schema
  (in favor of `outcome_side`/`book_side`) despite still being listed as required. Following the
  same pattern already used for `side`/`action`/`taker_side` elsewhere in this crate, it is modeled
  as `Option<YesNo>` rather than a bare `YesNo`.

- `pyth_value` is a new, authenticated-only AsyncAPI channel (introduced 2026-07-23) that delivers
  deduplicated real-time Pyth price updates for configured underlying tickers. It mirrors the
  `cfbenchmarks_value` channel's shape: seed `underlying_tickers` in the initial subscribe (or omit
  for an empty subscription), then use `update_subscription_v2` with
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` plus the
  `underlying_tickers` field on `WsUpdateSubscriptionParamsV2` to manage it post-subscribe. Modeled
  as `WsPythValue` / `WsPythUnderlyingList` (plus zero-copy `Ref` variants) routed through the
  standard `WsDataMessageV2` enum. `validate_update` rejects mixing underlying actions with market
  targets and requires `underlying_tickers` for the add/remove actions, matching the AsyncAPI error
  semantics (the same rules already enforced for CF Benchmarks index actions).

- `GET /account/api_usage_level/upgrade` returns an empty `201` body (no response schema); it is
  modeled with `EmptyResponse` like other empty-body endpoints in this crate.

- `IntraExchangeInstanceTransferRequest.source`/`.destination` (and the corresponding field on
  `IntraExchangeInstanceTransfer`) are kept as a raw `String` type alias (`ExchangeInstance`) rather
  than an enum, for the same forward-compatibility reason already documented above for
  `ApiUsageLevelGrant.exchange_instance`: known values today are `"event_contract"` and
  `"margined"`, but this crate does not model Margin-instance behavior beyond passing the string
  through losslessly.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
