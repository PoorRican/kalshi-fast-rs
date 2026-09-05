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

- **Exchange sharding / `exchange_index`.** Starting mid-2026 Kalshi began provisioning dedicated
  exchange shards for high-volume categories (crypto, tennis, baseball, and later commodities,
  basketball). Most REST responses and WebSocket messages that identify a market, order, fill,
  position, or balance now carry an `exchange_index` field identifying which shard it lives on.
  These were added as `Option<i32>` throughout (REST: `Market`, `EventData`, `Series`,
  `MultivariateEventCollection`, `Fill`, `Settlement`, `MarketPosition`, `SubaccountBalance`,
  `ApiKey`; WS: `WsMarketLifecycleV2`/`WsEventLifecycle` (top-level, `created`/`event_lifecycle`
  events only), `WsFill`, `WsUserOrder`) even where the live OpenAPI/AsyncAPI schema marks the field
  required, to tolerate any shard not yet reporting it. `GetBalanceResponse.balance_breakdown` and
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` use a shared
  `IndexedBalance { exchange_index, balance }` type. `GetExchangeStatusResponse` gained
  `intra_exchange_transfers_active` and an `exchange_index_statuses: Vec<ExchangeIndexStatus>`
  per-shard breakdown. `get_balance` now takes a `GetBalanceParams { subaccount, exchange_index }`
  (previously took no arguments); `GetPositionsParams` / `GetFillsParams` gained an `exchange_index`
  filter; `create_subaccount` now takes a `CreateSubaccountRequest { exchange_index }` body
  (previously took no arguments).

- **`WsMarketLifecycleV2` `metadata_updated` top-level fields.** Alongside the existing top-level
  `floor_strike` / `yes_sub_title` (added 2026-05-11), the AsyncAPI now documents `strike_type`,
  `cap_strike`, and `custom_strike` at the same top level for `metadata_updated` events (distinct
  from the nested `additional_metadata.*` copies emitted on `created`). `price_ranges` (a
  `Vec<WsPriceRange>` of `{start, end, step}` dollar-string bands) is emitted alongside
  `price_level_structure` on `created` and `price_level_structure_updated` events.

- **`price_level_structure` stays a raw `String`, never an enum.** Kalshi has added many new
  price-level-structure values since this was first modeled (`center_deci_edge_centi_cent` for
  tapered sub-cent multivariate pricing, plus seven `center_*_edge_*_cent` variants introduced
  2026-07-23). Because the field round-trips as an opaque string on both `Market` and the
  `market_lifecycle_v2` messages, none of these additions required a crate change — this is by
  design, not an oversight. Always derive valid order prices from the `price_ranges` array, never
  from the structure name.

- **`multivariate` WebSocket channel and its REST lookup endpoints were removed by Kalshi (fully
  gone by 2026-08-06),** not merely deprecated. This crate removed the corresponding surface
  entirely rather than keeping it as dead code: `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate` / `MultivariateLookup`, `WsDataMessageV2::Multivariate` /
  `WsDataMessageRef::Multivariate`, and the `WsMultivariate` / `WsMultivariateRef` message types;
  `KalshiRestClient::get_multivariate_event_collection_lookup_history` and
  `::lookup_tickers_for_market_in_multivariate_event_collection` (and their request/response
  types). Use the `multivariate_market_lifecycle` WebSocket channel and
  `POST /multivariate_event_collections/{collection_ticker}` instead.

- **`ErrorResponse.service` was removed** from the OpenAPI `ErrorResponse` schema (announced
  deprecated 2026-07-28, removed 2026-08-06); the crate dropped the field to match. `code` is the
  documented, stable way to branch on error kind.

- **`Market.response_price_units` / `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed** from the OpenAPI schema (2026-07-09) and are
  no longer returned live; the crate removed the corresponding struct fields.
  `GET /exchange/announcements` was removed from the API (2026-07-04) along with the
  `get_exchange_announcements` method, `Announcement`, `AnnouncementType`, `AnnouncementStatus`, and
  `GetExchangeAnnouncementsResponse` types. Legacy order-mutation methods (`create_order`,
  `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) are
  marked `#[deprecated]` in favor of their `_v2` equivalents (deprecated by Kalshi 2026-06-18; the
  V2 endpoints also cost fewer rate-limit tokens) but are kept working since Kalshi has not removed
  the underlying routes.

- **`GetQuotesParams` no longer supports `market_ticker` / `event_ticker` filters** (removed by
  Kalshi 2026-06-20); it gained `min_ts`, `max_ts`, and `user_filter` instead. RFQ-scoped quote
  action endpoints (`{get,delete,accept,confirm}_quote_for_rfq`, path-scoped by `rfq_id`) were added
  alongside the pre-existing quote-ID-only methods, which Kalshi deprecated (not removed) in favor
  of the scoped variants.

- **`Market` and `EventData` carry several fields that no longer appear in the current OpenAPI
  schema** (e.g. `Market`'s legacy integer/`_dollars`-duplicate price and count fields, `market_id`,
  `series_id`; `EventData`'s `status`, `can_trade`, `can_settle`, `volume`, `occurrence_datetime`,
  etc.). These were intentionally left in place rather than removed in this pass: the live schema is
  known to be incomplete relative to actual traffic in places (see the top of this file), and there
  is no changelog entry confirming these specific fields stopped being sent. They remain `Option`
  and harmless if truly gone. A dedicated follow-up refresh should verify each against live/demo
  traffic before removing them.

- **Known gaps (not yet implemented).** The following upstream additions are not yet modeled and
  are tracked here rather than silently dropped: `GET /account/api_usage_level/volume_progress` and
  `POST /account/api_usage_level/upgrade`; `GET /historical/positions` (+ its `subaccount` filter);
  `GET /live_data/weather/{city}` and `GET /live_data/weather/{city}/calibrations`; `GET
  /margin/fee_tier_rates`; `POST`/`GET /portfolio/target_balance_allocation`; `GET`/`POST
  /portfolio/intra_exchange_instance_transfer(s)`; the Predictions/Margin cancel-all-orders
  endpoints; `client_order_ids` filtering on `GET /fcm/orders`; and the `pyth_value` and
  `cfbenchmarks_value_5hz` WebSocket channels.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
