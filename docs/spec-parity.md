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

- `GET /exchange/announcements` was removed from the OpenAPI spec (2026-07-04). `get_exchange_announcements`,
  `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, and `AnnouncementStatus` were removed from
  the crate accordingly (0.6.0 → 0.7.0, breaking).

- `GET /exchange/status` (`GetExchangeStatusResponse`) gained an optional per-index breakdown (2026-06-26):
  `intra_exchange_transfers_active: Option<bool>` and `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>`.
  The top-level fields still reflect the default exchange index (`0`); the per-index array is absent when the
  breakdown is unavailable. `ExchangeIndexStatus.exchange_index` is `i64` (the OpenAPI `ExchangeIndex` schema is a
  bare integer, not an enum — only `0` is documented as supported today).

- `market.response_price_units`, `market.fractional_trading_enabled`, and `market_positions.resting_orders_count`
  were removed from the Predictions REST OpenAPI schema (2026-07-03) and deleted from `Market` and `MarketPosition`
  (REST and the WS `market_position`-shaped mirror in `ws::types`) rather than kept as stale `Option` fields, per the
  refresh policy for confirmed-removed schema. Same for the WS `market_lifecycle_v2` `fractional_trading_enabled`
  field and the `fractional_trading_updated` event type, both absent from the current AsyncAPI (0.6.0 → 0.7.0,
  breaking).

- `price_ranges` (array of `{start, end, step}` fixed-point-dollar strings, reusing the REST `PriceRange` shape) was
  added to the WS `market_lifecycle_v2` payload (2026-06-30). It is emitted alongside `price_level_structure` on
  `created` and `price_level_structure_updated` events.

- Seven additional `price_level_structure` values (center/edge tick combinations) were introduced upstream
  (2026-07-07, rolling out July 27 – August 3). No crate change was needed: `price_level_structure` is already
  modeled as `Option<String>` on both `Market` and the WS lifecycle payload precisely so new values round-trip
  without a release.

- `GET /communications/rfqs/{rfq_id}/quotes/{quote_id}` (plus `DELETE`, `PUT .../accept`, `PUT .../confirm`) were
  added as the RFQ-scoped replacements for the quote-ID-only endpoints (2026-07-07). `get_rfq_quote`,
  `delete_rfq_quote`, `accept_rfq_quote`, and `confirm_rfq_quote` were added; `get_quote`, `delete_quote`,
  `accept_quote`, and `confirm_quote` are marked `#[deprecated]` (the OpenAPI spec marks all four `deprecated: true`
  but keeps them present, so they are not removed).

- `GET /multivariate_event_collections/{ticker}/lookup` was removed from the OpenAPI spec entirely (only the `PUT`
  on the same path remains, now `deprecated: true` — "predates RFQs, do not use for new integrations").
  `get_multivariate_event_collection_lookup_history`, `GetMultivariateEventCollectionLookupHistoryParams`/
  `Response`, and `LookupPoint` were removed; `lookup_tickers_for_market_in_multivariate_event_collection` is marked
  `#[deprecated]` (0.6.0 → 0.7.0, breaking).

- `pyth_value` is a new authenticated AsyncAPI channel (added 2026-07-13) delivering deduplicated real-time Pyth
  price updates, modeled as `WsPythValue` (`underlying_ticker`, `value_usd`, `source_ts_ms`, `received_at`). The
  companion `pyth_value_underlying_list` message (`WsPythUnderlyingList`) lists underlyings observed in the last two
  hours. Subscribe by seeding `underlying_tickers` (use `["all"]` for every underlying) and manage the set post-hoc
  via `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList`, mirroring the
  `cfbenchmarks_value` / `index_ids` pattern.

- `WsChannelV2::is_private()` was missing `CfbenchmarksValue` even though its AsyncAPI channel description states
  "Requires authentication" — a pre-existing gap from the 0.6.0 `cfbenchmarks_value` addition, fixed alongside the
  new `PythValue` channel (both now correctly force an authenticated connection when subscribed).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
