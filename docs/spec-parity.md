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

- `ErrorResponse.service` was deprecated 2026-07-28 and removed from the OpenAPI `ErrorResponse`
  schema 2026-08-06. The field is kept as `Option<String>` but marked `#[deprecated]`; it will
  always be `None` going forward. Callers should branch on `code` instead.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema 2026-07-09. All three
  were already `Option`, so removal is non-breaking for parsing; the fields are now marked
  `#[deprecated]` and will always be `None`. Use `price_level_structure` / `price_ranges` instead
  of the market fields.

- `WsMarketLifecycleV2::fractional_trading_enabled` and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` event type no longer appear in the
  AsyncAPI `marketLifecycleV2Payload` schema or its `event_type` enum (undated removal, found by
  direct spec diff rather than a changelog entry). The field is kept and marked `#[deprecated]`
  for lossless round-trip of any historical/cached payloads; the enum variant is kept unchanged
  (removing it would be a breaking enum-variant removal) but is expected to never fire again.

- `price_level_structure` is modeled as a raw `String` (REST `Market` and WS
  `WsMarketLifecycleV2`), not a typed enum, so the seven new `price_level_structure` values
  introduced 2026-07-23 (`center_whole_edge_half_cent`, `center_whole_edge_quint_cent`,
  `center_half_edge_half_cent`, `center_half_edge_quint_cent`, `center_half_edge_deci_cent`,
  `center_quint_edge_quint_cent`, `center_quint_edge_deci_cent`) required no crate change. The
  source of truth for valid order prices remains `price_ranges` (`{start, end, step}` in
  fixed-point dollars), which is present on both the REST `Market` object and (added 2026-07-02)
  the WS `market_lifecycle_v2` `created` / `price_level_structure_updated` events.

- `GET /exchange/announcements` was removed from the OpenAPI spec 2026-07-04 and now errors.
  `get_exchange_announcements` and its response types are kept (marked `#[deprecated]`) for source
  compatibility; downstream code should stop calling it. Exchange schedule remains available via
  `get_exchange_schedule`.

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were removed from
  the OpenAPI spec between 2026-06-18 and 2026-06-25 and now return an error directing callers to
  the V2 endpoints. All six methods are kept (marked `#[deprecated]`) for source compatibility;
  use `create_order_v2` / `cancel_order_v2` / `amend_order_v2` / `decrease_order_v2` /
  `batch_create_orders_v2` / `batch_cancel_orders_v2` instead. `GET /portfolio/orders`,
  `GET /portfolio/orders/{order_id}`, and the queue-position endpoints are unaffected.

- `get_multivariate_event_collection_lookup_history` (`GET .../lookup`) is fully deprecated
  upstream (2026-07-02) and no longer appears in the OpenAPI spec at all.
  `lookup_tickers_for_market_in_multivariate_event_collection` (`PUT .../lookup`) remains in the
  spec but is marked `deprecated: true` ("predates RFQs, do not use for new integrations"). Both
  crate methods are kept and marked `#[deprecated]`.

- `GET /communications/quotes` (`GetQuotesParams`) lost its `market_ticker` / `event_ticker` query
  filters 2026-06-20 ("effective immediately") — they are no longer documented and no longer
  filter anything, so the fields were removed from `GetQuotesParams` entirely (breaking Rust API
  change, 0.6.0 → 0.7.0) rather than kept as dead parameters. `min_ts` / `max_ts` (added
  2026-06-18) and `user_filter` were added; `quote_creator_user_id` / `rfq_creator_user_id` are
  marked `deprecated: true` upstream and kept as `#[deprecated]` fields.
  `GET /communications/rfqs` (`GetRFQsParams`) keeps its `market_ticker` / `event_ticker` filters
  unchanged; `user_filter` was added and `creator_user_id` marked `#[deprecated]`.

- RFQ-scoped quote endpoints were added 2026-06-25 / 2026-07-09:
  `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`
  (`/communications/rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`). The quote-ID-only
  endpoints (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) are marked
  `deprecated: true` upstream and kept as `#[deprecated]` methods.

- `exchange_index` (identifies an exchange shard; `0` is currently the only value in production)
  was added across several REST and WS surfaces in 2026-07: `Series`, `EventData`,
  `GetExchangeStatusResponse` (plus a new `exchange_index_statuses: Vec<ExchangeIndexStatus>` /
  `intra_exchange_transfers_active` per-index breakdown), `SubaccountBalance` (required — a
  subaccount with funds on multiple indexes now appears as multiple entries), and the WS
  `market_lifecycle_v2` (`created` events) / `event_lifecycle` messages. All are modeled as
  `Option<i32>` (or `i64` on the WS lifecycle messages, matching the surrounding timestamp field
  types) except `SubaccountBalance.exchange_index: i32`, which the OpenAPI spec marks required.

- `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest` gained an optional
  `subaccount: Option<u32>` field (2026-07-02) for restricting an API key to a single sub-account.

- `WsMarketLifecycleV2` gained top-level `strike_type: Option<String>`, `cap_strike: Option<f64>`,
  and `custom_strike: Option<Map<String, Value>>` fields (2026-06-18), present only on
  `metadata_updated` events and distinct from the same-named fields nested under
  `additional_metadata` (which are emitted on market creation instead) — mirroring the existing
  `floor_strike` / `yes_sub_title` top-level-vs-nested distinction.

- `WsQuoteCreated`, `WsQuoteAccepted`, and `WsQuoteExecuted` gained an optional
  `subaccount: Option<i32>` field (2026-07-30), present only when the authenticated user's side of
  the quote used a subaccount.

- `pyth_value` is a new AsyncAPI channel (2026-07-23) delivering deduplicated real-time Pyth price
  updates by underlying ticker. Unlike `cfbenchmarks_value`, it requires authentication. It uses
  `underlying_tickers` (not market tickers) for subscription parameters; pass `["all"]` to track
  every available underlying, or subscribe with none to create an empty subscription and discover
  recently-streamed underlyings via `underlying_list`. The channel emits `pyth_value`
  (per-underlying value + source/received timestamps) and `pyth_value_underlying_list` (recently
  streamed underlyings), modeled as `WsPythValue` / `WsPythUnderlyingList` and routed through the
  standard `WsDataMessageV2` enum, mirroring the `cfbenchmarks_value` implementation. Subscription
  updates use `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList`
  plus the `underlying_tickers` field on `WsUpdateSubscriptionParamsV2`; `validate_update` rejects
  mixing underlying actions with market targets and requires `underlying_tickers` for the
  add/remove actions, matching the AsyncAPI error semantics.

- `GET /events` (`GetEventsParams`) gained `tickers` (comma-separated event ticker filter,
  2026-06-18) and `min_updated_ts` (poll-for-changes filter; present in the live OpenAPI spec but
  not tied to a specific changelog entry) query parameters. `EventStatus` gained an `Unopened`
  variant (present in the live spec's `status` filter enum, likewise undated in the changelog).

- `EventData` (`GET /events` / `GET /events/{event_ticker}`) gained `settlement_sources:
  Vec<SettlementSource>` (2026-06-18, mirroring the field already on `Series`) and
  `exchange_index: Option<i32>` (2026-07-30).

- `EventMetadata.cadence: Option<String>` (2026-07-30) — how often the event recurs (e.g.
  `"fifteen_min"`). Not covered by the OpenAPI spec (`product_metadata` is untyped there for
  events); the changelog is the only source of truth for this field's shape.

- New endpoints added in this pass: `GET /historical/positions` (`get_historical_positions`,
  2026-07-23, reuses `GetPositionsResponse`), `GET /live_data/events/{event_ticker}`
  (`get_event_live_data`, 2026-07-30, new `EventLiveData` type), `GET
  /account/api_usage_level/volume_progress` (`get_account_api_usage_level_volume_progress`,
  2026-06-11) and `POST /account/api_usage_level/upgrade`
  (`upgrade_account_api_usage_level`, 2026-06-11).

- Several 2026-06 through 2026-07 changelog entries required no code change because they describe
  subaccount-restricted API key *permission* behavior only (no new fields/shapes) on already-
  modeled endpoints (order queue positions, batch order endpoints, order groups, WebSocket
  sessions), operational-only changes (rate limits, retention windows, per-user object limits), or
  margin-market / FIX surfaces this crate does not model at all (margin market data, margin
  risk/positions restructuring, all FIX API changes).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
