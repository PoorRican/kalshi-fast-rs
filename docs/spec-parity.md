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

- `GET /events` gained `tickers` (comma-separated event ticker filter) and `min_updated_ts` query
  parameters, and `EventData` gained a top-level `settlement_sources: Vec<SettlementSource>` field
  (2026-06-18), mirroring the field already available on `product_metadata`/`Series`.

- `market_lifecycle_v2` `metadata_updated` events now carry `strike_type`, `cap_strike`, and
  (for custom/structured markets) `custom_strike` at the top level, alongside the existing
  top-level `floor_strike`/`yes_sub_title` (2026-06-18). `created` and
  `price_level_structure_updated` events also gained an optional `price_ranges` array (2026-07-02),
  reusing the REST `PriceRange` type (`{start, end, step}` in fixed-point dollars). Seven new
  `price_level_structure` string values were introduced (2026-07-23); no crate change was needed
  since `price_level_structure` is modeled as a raw `Option<String>`, not an enum.

- Kalshi removed several deprecated fields/endpoints from the published OpenAPI/AsyncAPI outright
  (not just marked deprecated) between 2026-06 and 2026-07: `GET /exchange/announcements`,
  the multivariate lookup-history endpoint (`GET .../lookup`), `Market.response_price_units`,
  `Market.fractional_trading_enabled` (REST and the WS `market_lifecycle_v2`
  `fractional_trading_updated` event/field), and `MarketPosition.resting_orders_count`. These were
  removed from the crate (not just deprecated) since calling them can no longer succeed; this was
  the reason for the 0.6.0 → 0.7.0 bump alongside the additions below.

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) are absent from the current
  OpenAPI spec (only `GET /portfolio/orders` and `GET /portfolio/orders/{order_id}` remain), per
  Kalshi's 2026-06-18/2026-06-25 deprecation announcement. Because it could not be confirmed live
  whether the server still accepts these requests during a grace period, the methods were marked
  `#[deprecated]` (pointing to the `_v2` equivalents) rather than removed outright.

- `GET /communications/quotes` no longer supports filtering by `market_ticker` or `event_ticker`
  (2026-06-20); those `GetQuotesParams` fields are marked `#[deprecated]` (kept for source
  compatibility, but the server ignores them). `min_ts`/`max_ts` and a new `user_filter` (distinct
  from `rfq_user_filter`) were added. `GetRFQsParams` gained `user_filter`; `quote_creator_user_id`,
  `rfq_creator_user_id` (on `GetQuotesParams`), and `creator_user_id` (on `GetRFQsParams`) are
  `#[deprecated]` (server spec marks them deprecated, but they remain functional).

- RFQ-scoped quote lookup/cancel/accept/confirm endpoints
  (`/communications/rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`) were added 2026-06-25/07-09
  as `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`. The quote-ID-only
  equivalents (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) are marked
  `#[deprecated]` in favor of the RFQ-scoped versions.

- `GET /exchange/status` gained `intra_exchange_transfers_active` and a per-index
  `exchange_index_statuses` breakdown (2026-06-26); `SubaccountBalance`
  (`GET /portfolio/subaccounts/balances`) gained a required `exchange_index: i64` field
  (2026-07-02), since a subaccount with funds on multiple exchange indexes now returns one row per
  index instead of a single combined row. `ApiKey`/`CreateApiKeyRequest`/`GenerateApiKeyRequest`
  gained an optional `subaccount: Option<u32>` for single-subaccount-restricted API keys
  (2026-07-02).

- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade` were
  added (2026-06-11/2026-07-02) as `get_account_api_usage_level_volume_progress` and
  `upgrade_account_api_usage_level`.

- `pyth_value` is a new authenticated AsyncAPI channel (2026-07-13) delivering deduplicated
  real-time Pyth prices by underlying ticker, closely mirroring the existing `cfbenchmarks_value`
  channel: it uses `underlying_tickers` (not market tickers) for subscription parameters (pass
  `["all"]` for every available underlying), and the post-subscribe workflow
  (`underlying_list` / `subscribe_underlyings` / `unsubscribe_underlyings`) is supported through
  `update_subscription_v2` via new `WsUpdateAction` variants and an `underlying_tickers` field on
  `WsUpdateSubscriptionParamsV2`. Messages are modeled as `WsPythValue` / `WsPythUnderlyingList` and
  routed through the standard `WsDataMessageV2` enum.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
