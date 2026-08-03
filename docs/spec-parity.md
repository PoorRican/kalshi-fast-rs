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

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were removed from the OpenAPI spec
  (2026-06-18 deprecation announcement, confirmed removed by 2026-08-03). The corresponding Rust
  methods and request/response types were removed rather than kept as dead HTTP calls. Use the V2
  event-order endpoints (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead.

- The quote-ID-only communications endpoints (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) and `lookup_tickers_for_market_in_multivariate_event_collection` are marked
  `#[deprecated]` in Rust, matching `deprecated: true` in the live OpenAPI spec. Unlike the legacy
  order endpoints above, these are *not* removed — the spec still documents and supports them, so the
  crate keeps them callable while steering new code at the preferred replacements (`get_rfq_quote` /
  `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`, and RFQs generally).

- `price_level_structure` (on both the REST `Market` struct and the `market_lifecycle_v2` WebSocket
  message) is modeled as a raw `String`, not a closed Rust enum. When Kalshi introduced seven new
  price-level-structure values in 2026-07, no crate change was needed — new string values just parse
  through unchanged.

- `EventData.product_metadata` reuses the `EventMetadata` type (the schema for
  `GET /events/{ticker}/metadata`) even though the OpenAPI models the two as distinct, unrelated
  shapes (the top-level `product_metadata` field is an untyped free-form object; `EventMetadata` is a
  curated schema with `image_url`/`market_details`/`settlement_sources`). This is a pre-existing
  modeling choice, not changed by this refresh. It is not lossy: `EventMetadata` carries a
  `#[serde(flatten)] extra` catch-all, so free-form keys like the 2026-07-30 `cadence` addition land in
  `extra` rather than being dropped.

- `ErrorResponse.service` was deprecated by Kalshi on 2026-07-28 and removed from all REST error
  bodies by 2026-08-06. The field was already `Option<String>`, so no crate change was required;
  it now simply always deserializes to `None`. Branch on `ErrorResponse.code` instead.

- The `market_lifecycle_v2` WebSocket channel's `fractional_trading_updated` event type and the
  `fractional_trading_enabled` field were removed from the AsyncAPI in April 2026 (predating the
  0.6.0 watermark), but the crate had not caught up: `WsMarketLifecycleEventType` still had a
  `FractionalTradingUpdated` variant and `WsMarketLifecycleV2` still had a `fractional_trading_enabled`
  field. Both were removed as part of this refresh's field-by-field verification pass.

- The `pyth_value` WebSocket channel (added 2026-07-23) mirrors the `cfbenchmarks_value` channel's
  shape: a private/authenticated channel using a dedicated subscription-parameter field
  (`underlying_tickers` instead of `index_ids`) and three update actions (`subscribe_underlyings` /
  `unsubscribe_underlyings` / `underlying_list` instead of `subscribe_indices` /
  `unsubscribe_indices` / `indexlist`). Unlike `cfbenchmarks_value`, the `pyth_value` message payload
  has no windowed-average companion data — just `underlying_ticker`, `value_usd`, `source_ts_ms`, and
  `received_at`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
