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

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST API schema on
  2026-07-09 and are absent from the current OpenAPI/AsyncAPI documents (including the
  `market_lifecycle_v2` `fractional_trading_enabled` field and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` event, both also gone from AsyncAPI). Per
  `VERSIONING.md`, these were removed from the public Rust API rather than kept as optional shims
  (minor version bump).

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were scheduled for deprecation by
  the exchange between 2026-06-18 and 2026-06-25, and their operations are no longer present in the
  published OpenAPI document. The changelog states calls now return a message directing callers to
  the V2 endpoints rather than a hard failure, so — unlike a genuine schema removal — these methods
  are kept and marked `#[deprecated]` (pointing at the corresponding `*_v2` method) instead of being
  deleted. This is the explicit exception referenced by the refresh workflow's "don't preserve
  removed endpoints" rule.

- `GET /communications/quotes/{quote_id}`, `DELETE .../quotes/{quote_id}`, `PUT .../accept`, and
  `PUT .../confirm` (quote-ID-only) are marked `deprecated: true` in the OpenAPI spec as of
  2026-06-25 / 2026-07-09 in favor of the RFQ-scoped equivalents
  (`GET/DELETE /communications/rfqs/{rfq_id}/quotes/{quote_id}`, `.../accept`, `.../confirm`). The
  crate keeps the old methods as `#[deprecated]` and adds `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`. `GetQuotesParams.market_ticker` / `.event_ticker` were
  removed (exchange dropped the filters 2026-06-20); `min_ts`, `max_ts`, and `user_filter` were
  added (2026-06-18 / current spec parity).

- `lookup_tickers_for_market_in_multivariate_event_collection` (`PUT
  /multivariate_event_collections/{collection_ticker}/lookup`) is marked `deprecated: true` in the
  OpenAPI spec ("predates RFQs") and is kept as `#[deprecated]`. Its sibling GET (lookup history) is
  fully absent from the current OpenAPI document (fully deprecated 2026-07-02), so
  `get_multivariate_event_collection_lookup_history` and its types were removed outright rather than
  deprecated, matching the "endpoint gone from the schema" rule.

- `GET /exchange/announcements` was removed from the OpenAPI document on 2026-07-04
  (`GET /exchange/schedule` remains the source for operational hours). `get_exchange_announcements`,
  `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, and `AnnouncementStatus`
  were removed from the public Rust API accordingly.

- `pyth_value` is a new AsyncAPI channel (2026-07-13, authenticated) that delivers deduplicated Pyth
  price updates by underlying ticker, mirroring the `cfbenchmarks_value` shape: seed
  `underlying_tickers` on subscribe (or `["all"]` for every underlying), and use
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` via
  `update_subscription_v2` to manage the set post-subscribe. Modeled as `WsPythValue` /
  `WsPythUnderlyingList` and routed through `WsDataMessageV2`.

- `market_lifecycle_v2` gained a top-level `price_ranges` array (2026-07-02, emitted alongside
  `price_level_structure` on `created` / `price_level_structure_updated`) and top-level
  `strike_type` / `cap_strike` / `custom_strike` (2026-06-18, `metadata_updated` only, alongside the
  pre-existing top-level `floor_strike` / `yes_sub_title`). All four follow the same "top-level,
  event-scoped, distinct from `additional_metadata`" pattern already used for `floor_strike`.

- Seven new `price_level_structure` values were introduced 2026-07-23 (`center_*_edge_*_cent`
  variants). No crate change was needed: `price_level_structure` is already modeled as an opaque
  `Option<String>` (both in `Market` and in `market_lifecycle_v2`) specifically so new values pass
  through without a release. Consumers should key off `price_ranges` for valid order prices, not the
  structure label.

- `POST /api_keys`, `POST /api_keys/generate`, and `GET /api_keys` gained a `subaccount` field
  (2026-07-02) to restrict an API key to a single sub-account (0-63). Modeled as `Option<u32>` on
  `CreateApiKeyRequest`, `GenerateApiKeyRequest`, and `ApiKey`.

- `SubaccountBalance.exchange_index: u32` (required, non-`Option`) was added 2026-07-02:
  `GET /portfolio/subaccounts/balances` now returns one row per exchange index instead of one
  combined row per subaccount.

- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade` are new
  endpoints (2026-06-11 / 2026-06-25) for self-service volume-based API tier progress and upgrades.
  Modeled as `get_account_api_usage_level_volume_progress` /
  `GetAccountApiUsageLevelVolumeProgressResponse` and `upgrade_account_api_usage_level` (returns
  `EmptyResponse`; the endpoint has no response body).

- `GET /historical/positions` is a new endpoint (2026-07-23) for settled positions archived from the
  live position set; it reuses the `GetPositionsResponse` shape. `GetHistoricalCutoffResponse` gained
  `market_positions_last_updated_ts: Option<String>`, the cutoff separating live from historical
  positions.

- `GetEventsParams` gained `tickers: Option<String>` (comma-separated event tickers) and
  `min_updated_ts: Option<i64>` (poll for metadata changes), matching the live OpenAPI parameter list
  for `GET /events`; `EventData` gained `settlement_sources`, `fee_type_override`,
  `fee_multiplier_override`, and `exchange_index` to match the current `EventData` schema (all
  additive, `Option`).

- `WsQuoteCreated`/`WsQuoteAccepted` gained `rfq_creator_id: Option<String>` (present in the AsyncAPI
  schema but previously unmodeled) and all three quote lifecycle messages
  (`quote_created`/`quote_accepted`/`quote_executed`) gained `subaccount: Option<u32>` (2026-07-30):
  the subaccount your side of the quote used, recipient-scoped (never the counterparty's).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
