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

- The legacy `/portfolio/orders` mutation endpoints (`POST /portfolio/orders`,
  `DELETE /portfolio/orders/{order_id}`, `POST .../amend`, `POST .../decrease`,
  `POST /portfolio/orders/batched`, `DELETE /portfolio/orders/batched`) were removed from the live
  OpenAPI spec and from the published docs index between the 2026-06-08 and 2026-07-24 refreshes
  (upstream deprecated them "sometime between June 18 and June 25, 2026"; calls now return "Please
  switch to the V2 endpoints"). The corresponding crate methods (`create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) and their
  request/response types (`CreateOrderRequest`, `CancelOrderParams`, `AmendOrderRequest`,
  `DecreaseOrderRequest`, `BatchCreateOrdersRequest`, `BatchCancelOrdersRequest`, and their
  response counterparts) were removed rather than kept as compatibility shims. Use the V2
  event-order endpoints (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`,
  `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead. `get_orders`,
  `get_order`, and the `Order` type are unaffected (the read-only `GET` operations remain live).

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema (2026-07-09) and are
  no longer modeled on the REST `Market` / `MarketPosition` structs (or the zero-copy
  `MarketPositionRef`). `Market.price_level_structure`, `Market.price_ranges`, and the fixed-point
  count/dollar fields remain the canonical replacements.
- The WebSocket `market_lifecycle_v2` payload dropped `fractional_trading_enabled` and the
  `fractional_trading_updated` `event_type` value from the AsyncAPI at the same time; both were
  removed from `WsMarketLifecycleV2` / `WsMarketLifecycleV2Ref` and from
  `WsMarketLifecycleEventType`.

- Seven new `price_level_structure` values were introduced (2026-07-23; pilot rollout the week of
  July 27, expanding the week of August 3): `center_whole_edge_half_cent`,
  `center_whole_edge_quint_cent`, `center_half_edge_half_cent`, `center_half_edge_quint_cent`,
  `center_half_edge_deci_cent`, `center_quint_edge_quint_cent`, `center_quint_edge_deci_cent`.
  `price_level_structure` is modeled as a plain `Option<String>` on both the REST `Market` and the
  WS lifecycle messages, so no crate change was required for the new values themselves — only for
  the accompanying `price_ranges` field described below.
- `market_lifecycle_v2` gained a `price_ranges` field (2026-07-23), emitted alongside
  `price_level_structure` on market creation and `price_level_structure_updated` events. It reuses
  the existing REST `PriceRange` type (`{ start, end, step }` fixed-point dollar strings) on both
  `WsMarketLifecycleV2` and `WsMarketLifecycleV2Ref`.

- `GET /exchange/announcements` was removed from the Predictions REST API (2026-07-04). The
  `get_exchange_announcements` method and the `Announcement` / `AnnouncementType` /
  `AnnouncementStatus` / `GetExchangeAnnouncementsResponse` types were removed. Exchange schedule
  remains available through `get_exchange_schedule`.

- `GET /communications/quotes` no longer accepts `market_ticker` or `event_ticker` query
  parameters (removed 2026-06-20); both fields were removed from `GetQuotesParams`. The endpoint
  gained `min_ts` / `max_ts` (2026-06-18) and a `user_filter` field (2026-06-25, "self" to filter
  by the authenticated user, distinct from the existing `rfq_user_filter`). `GetRFQsParams` gained
  the same `user_filter` field. `quote_creator_user_id` / `rfq_creator_user_id` (on
  `GetQuotesParams`) and `creator_user_id` (on `GetRFQsParams`) are marked `#[deprecated]` — the
  spec still accepts them but flags them `deprecated: true` in favor of `user_filter`.
- RFQ-scoped quote lookup/action endpoints were added (`GET`/`DELETE`
  `/communications/rfqs/{rfq_id}/quotes/{quote_id}` and `.../accept` / `.../confirm`; rolled out
  2026-06-25 through 2026-07-09) and are modeled as `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`. The quote-ID-only equivalents (`get_quote`,
  `delete_quote`, `accept_quote`, `confirm_quote`) remain supported by the exchange but are marked
  `#[deprecated]` per the OpenAPI `deprecated: true` flags. Per the same changelog entry, open RFQ
  quotes are no longer guaranteed durable — only `accepted`/`confirmed`/`executed` quotes are
  reliably queryable; a stale open quote may 404 without ever surfacing a `cancelled` state.

- `lookup_tickers_for_market_in_multivariate_event_collection` (`PUT
  /multivariate_event_collections/{collection_ticker}/lookup`) is marked `deprecated: true` in the
  OpenAPI spec ("predates RFQs; do not use for new integrations") and is now `#[deprecated]` in the
  crate. The companion `GET .../lookup` "lookup history" endpoint was removed from the spec
  entirely; `get_multivariate_event_collection_lookup_history` and its
  `GetMultivariateEventCollectionLookupHistoryParams` / `...Response` / `LookupPoint` types were
  removed.

- `EventData` (REST `GET /events`, `GET /events/{event_ticker}`) gained `settlement_sources`
  (2026-06-18), mirroring the field already on `Series`/`EventMetadata`. `GetEventsParams` gained a
  `tickers` field (comma-separated event tickers, 2026-06-18).

- `GET /historical/positions` (`get_historical_positions`) was added (2026-07-23) for settled
  positions archived per whole event; it reuses the existing `GetPositionsResponse` shape (same as
  `GET /portfolio/positions`). `GetHistoricalCutoffResponse` gained
  `market_positions_last_updated_ts: Option<String>`, the cutoff separating live from archived
  positions.

- API keys can now be restricted to a single sub-account (0-63) at creation (2026-07-02). `ApiKey`,
  `CreateApiKeyRequest`, and `GenerateApiKeyRequest` gained `subaccount: Option<u32>`; absent means
  unrestricted. `ApiKeyScope` values remain modeled as plain `Vec<String>` (not a typed enum) so new
  scopes like `write::trade`, `write::transfer`, `read::block_trade_accept`,
  `write::block_trade_accept`, and `read::portfolio_balance` round-trip without a crate update.

- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade` were
  added (2026-06-11) as `get_account_api_usage_level_volume_progress` and
  `upgrade_account_api_usage_level`. The upgrade endpoint has no response body (`EmptyResponse`).

- `GET /exchange/status` gained `intra_exchange_transfers_active` and a per-index
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` breakdown (2026-07-02); the top-level
  fields continue to reflect the default exchange index (0).
- `GET /portfolio/subaccounts/balances` now returns one entry per exchange index per subaccount
  (2026-07-02); `SubaccountBalance` gained `exchange_index: Option<ExchangeIndex>` (`ExchangeIndex`
  is a `pub type ExchangeIndex = i32` alias, since only index `0` is currently supported in
  production and the OpenAPI models it as a bare integer, not an enum).

- `pyth_value` is a new authenticated AsyncAPI channel (2026-07-23) delivering deduplicated Pyth
  prices by underlying ticker. It mirrors the `cfbenchmarks_value` pattern: `underlying_tickers`
  (not market tickers) seeds the initial subscription (`["all"]` for every underlying), and
  `update_subscription_v2` supports `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` plus an `underlying_tickers` field on
  `WsUpdateSubscriptionParamsV2`. `validate_update` rejects mixing underlying actions with market
  targets and requires `underlying_tickers` for the add/remove actions. Messages are modeled as
  `WsPythValue` (`pyth_value`) and `WsPythUnderlyingList` (`pyth_value_underlying_list`), routed
  through the standard `WsDataMessageV2` enum. Unlike `cfbenchmarks_value`, `pyth_value` requires
  authentication, so `WsChannelV2::PythValue::is_private()` returns `true`.

- The following upstream changes required no crate change because the affected surface is not
  modeled (margin markets/orders/positions/perps, FIX sessions) or the crate already tolerates the
  new values generically (scope strings, rate-limit/retention/volume thresholds, hidden-event
  filtering, per-request rate-limit costs): margin order system-order-reasons, margin
  mark-prices/`is_portfolio`/`margin_used`/risk-metric changes, margin subaccount positions,
  trade-scoped and block-trade-accept API key scopes, the June 2026 API-usage-tier threshold
  changes, the RFQ/quote retention-window reduction, the order-group 25,000-per-user cap, the
  500k-subscription/10k-commands-per-second WebSocket sanity limits, `Get Quote` rate-limit cost,
  hidden-event incentive-program filtering, and all FIX-tagged changelog entries (the crate does
  not implement a FIX client).

- Fixed a pre-existing test bug: `tests/rest_auth.rs::test_get_account_api_limits` referenced the
  removed `GetAccountApiLimitsResponse::{read_limit, write_limit}` fields (replaced by
  `read`/`write: BucketLimit` in the 0.6.0 refresh) and only compiled under the `live-tests`
  feature, which CI does not exercise (`cargo clippy --all-targets` omits `--features
  live-tests`), so the break went unnoticed. Updated to `resp.read.refill_rate` /
  `resp.write.refill_rate`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
