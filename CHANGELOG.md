# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.7.0] - 2026-08-02

### Compatibility

- Docs snapshot: 2026-08-02
- OpenAPI: 3.27.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-02

**Version bump: minor (0.6.0 → 0.7.0).** Per [`VERSIONING.md`](VERSIONING.md) §"Crate Version
Scope": "While the crate remains below `1.0.0`... Minor releases are for any intentional breaking
change to the public Rust API, or any change likely to require downstream code changes." Two
changes here are breaking: `GetQuotesParams` lost `market_ticker` / `event_ticker` (Kalshi removed
the corresponding query params), and `update_order_group_limit` gained a required
`SubaccountQueryParams` argument. Per the same section, major releases are reserved for the future
`1.0.0` transition, so pre-1.0 breaking changes bump minor, not major.

**Changelog entries since 0.6.0 watermark (2026-06-08) and disposition.** The crate covers
Predictions REST/WebSocket plus the single `get_margin_fee_tiers` endpoint; it has no FIX support
and does not otherwise model the Margin exchange, so entries tagged `FIX`-only or `Margin`-only
(beyond fee tiers) are dispositioned "No code change — out of crate scope" without further detail.

| Entry | Action |
|---|---|
| API usage volume progress endpoint (2026-06-11) | Added `get_account_api_usage_level_volume_progress`, `GetAccountApiUsageLevelVolumeProgressResponse`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal` |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Added `upgrade_account_api_usage_level` |
| Perps mark prices / tick size / volume-OI notional on margin markets (2026-06-11) | No code change — out of crate scope |
| Margin fee-tier endpoint returns active rates (2026-06-11) | Already handled in 0.6.0 (watermark predates this entry's effective date) |
| Fractional quantities for RFQs (2026-06-11) | Already handled in 0.6.0 (`contracts_fp` already present) |
| settlement_sources added to events API (2026-06-18) | Added `EventData.settlement_sources` |
| Strike type and cap strike on market_lifecycle_v2 metadata_updated (2026-06-18) | Added top-level `strike_type` / `cap_strike` / `custom_strike` to `WsMarketLifecycleV2` |
| RFQ quote identity on FIX / trade entries in FIX market data (2026-06-18) | No code change — out of crate scope |
| Legacy order mutation endpoints deprecated / removed from spec (2026-06-18) | Deprecated `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`; `place_order.rs` example migrated to `create_order_v2` |
| Event tickers filter on GET /events (2026-06-18) | Added `GetEventsParams.tickers` |
| Subaccount on margin positions (2026-06-18) | No code change — out of crate scope |
| Block-trade accept API key permissions (2026-06-18) | No code change — scopes stored as `Vec<String>` already |
| Sanity limits enforced on orderbook subscriptions (2026-06-18) | No code change — operational limit only |
| Quote time filters and pagination fix (2026-06-18) | Added `GetQuotesParams.min_ts` / `.max_ts` |
| Communications RFQ and quote retention window reduced (2026-06-19) | No code change — operational retention only |
| RFQ quote market and event filters removed (2026-06-20) | Removed `GetQuotesParams.market_ticker` / `.event_ticker` (**breaking**) |
| Get Quote rate-limit cost reduced (2026-06-23) | No code change — rate-limit costs are queried dynamically via `get_account_endpoint_costs`, not hardcoded |
| RFQ quotes support post-only on FIX (2026-06-24) | No code change — out of crate scope |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; deprecated `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` |
| API usage tier qualification requirements halved (2026-06-25) | No code change — operational threshold only |
| FIX exchange index routing (2026-06-25) | No code change — out of crate scope |
| Trade-scoped API key permissions (write::trade) (2026-06-30) | No code change — scopes stored as `Vec<String>` already |
| price_ranges added to market_lifecycle_v2 events (2026-07-02) | Added `WsMarketLifecycleV2.price_ranges` |
| Margin positions margin_used / is_portfolio / per-market risk metrics (2026-06-26/06-29/07-02) | No code change — out of crate scope |
| Per-index exchange status (2026-07-02) | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`, `.exchange_index_statuses`, new `ExchangeIndexStatus` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index` |
| AcceptQuote / OrderCancelReject FIX rejects carry specific reasons (2026-07-02) | No code change — out of crate scope |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| Multivariate lookup history endpoints fully deprecated / removed from spec (2026-07-02) | Deprecated `get_multivariate_event_collection_lookup_history` |
| Exchange announcements endpoint removed (2026-07-04) | Deprecated `get_exchange_announcements`; removed the now-dead live test |
| Support for FIX Tag 2446 on Incremental Refresh (2026-07-09) | No code change — out of crate scope |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote` (see 2026-06-25 entry above) |
| Deprecated Predictions REST schema fields removed (2026-07-09) | Deprecated `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (and the mirrored WS-position-page `resting_orders_count` in `src/ws/types/mod.rs`) |
| Margin orders identify system order reasons (2026-07-09) | No code change — out of crate scope |
| Historical positions endpoint (2026-07-22/23) | Added `get_historical_positions`, `GetHistoricalPositionsParams`, `GetHistoricalCutoffResponse.market_positions_last_updated_ts` |
| Incentive programs on hidden events excluded from listing (2026-07-22) | No code change — server-side filtering only |
| Order groups limited to 25,000 per user (2026-07-22/23) | No code change — server-side limit only |
| Subaccount-restricted API keys can open WebSocket sessions (2026-07-23) | No code change — behavior only, no new fields |
| Subaccount-restricted API keys can quote on RFQ FIX sessions (2026-07-23) | No code change — out of crate scope |
| Pyth value WebSocket channel (2026-07-23) | Added `pyth_value` channel: `WsChannelV2::PythValue`, `WsPythValue`/`WsPythUnderlyingList` (+ `Ref` variants), `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`, `underlying_tickers` on `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2`, full wire/envelope routing. Also fixed `WsChannelV2::is_private()`, which was missing `CfbenchmarksValue` (pre-existing gap) and needed `PythValue` |
| New price level structures (2026-07-23/27, rollout through 08-03) | No code change — `price_level_structure` is already a plain `String`; consume `price_ranges` for valid prices |
| Subaccount-restricted API keys can manage order groups (2026-07-24/30) | No code change — behavior only |
| Subaccount on quote_created (2026-07-26) | Added `subaccount` to `WsQuoteCreated` (and, closing a pre-existing gap, to `WsQuoteAccepted` / `WsQuoteExecuted`, which the AsyncAPI already documented as having it) |
| Subaccount-restricted API keys can use batch order endpoints (2026-07-27/30) | No code change — behavior only |
| Event product_metadata now includes cadence (2026-07-28/30) | Added `EventMetadata.cadence` |
| Subaccount-restricted API keys can read order queue positions (2026-07-28/30) | No code change — `GetOrderQueuePositionsParams.subaccount` already present |
| The service field on error responses is deprecated / removed (2026-07-28/29) | Deprecated `ErrorResponse.service` |
| Series responses include exchange_index (2026-07-28/30) | Added `Series.exchange_index` |
| Lifecycle creation messages now include exchange_index (2026-07-28/30) | Added `exchange_index` to `WsMarketLifecycleV2` and `WsEventLifecycle` |
| New endpoint for event-keyed live data (2026-07-28/30) | Added `get_event_live_data`, `GetEventLiveDataParams`, `GetEventLiveDataResponse`, `EventLiveData` |
| Richer combo-validation errors on multivariate market creation (2026-07-29/30) | No code change — `ErrorResponse.message` / `.details` already untyped `Option<String>`; `code` values unchanged |
| Order group limit updates support subaccounts (2026-07-30/08-06) | `update_order_group_limit` now takes a `SubaccountQueryParams` argument (**breaking**) |
| Event data on the `EventData` schema: `exchange_index` (2026-07-30) | Added `EventData.exchange_index` |

### Added

- [Rust API] `get_account_api_usage_level_volume_progress()`, `upgrade_account_api_usage_level()`,
  `get_historical_positions()`, `get_event_live_data()`, `get_rfq_quote()`, `delete_rfq_quote()`,
  `accept_rfq_quote()`, `confirm_rfq_quote()` — see the disposition table above for the upstream
  endpoints each covers.
- [Rust API] `pyth_value` WebSocket channel support, modeled on the existing `cfbenchmarks_value`
  channel: `WsChannelV2::PythValue`, `WsPythValue`, `WsPythUnderlyingList` (+ `Ref` variants),
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList`, and
  `underlying_tickers` on `WsSubscriptionParamsV2` / `WsUpdateSubscriptionParamsV2`.
- [Rust API] `exchange_index` fields across `Market`, `EventData`, `Series`, `SubaccountBalance`,
  `WsMarketLifecycleV2`, and `WsEventLifecycle`, plus `GetExchangeStatusResponse.exchange_index_statuses`
  / `.intra_exchange_transfers_active` and the new `ExchangeIndexStatus` type.
- [Rust API] `subaccount` fields on `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest`, and on
  the `communications` channel's `quote_created` / `quote_accepted` / `quote_executed` messages.
- [Rust API] `settlement_sources` and `cadence` on event-related types; `price_ranges` and
  `strike_type` / `cap_strike` / `custom_strike` on `WsMarketLifecycleV2`.

### Deprecated

- [Rust API] Legacy `/portfolio/orders` mutation methods (`create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) — removed from the
  Kalshi OpenAPI spec 2026-06-18. Use the V2 event-order methods instead.
- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` — use the RFQ-scoped
  `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote` instead.
- [Rust API] `get_exchange_announcements` and `get_multivariate_event_collection_lookup_history` —
  both endpoints were removed from the Kalshi OpenAPI spec (2026-07-04 and 2026-07-02 respectively).
- [Rust API] `ErrorResponse.service`, `Market.response_price_units`,
  `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (and the mirrored
  `MarketPositionRef.resting_orders_count`) — removed from the Kalshi API schema; always `None` now.

### Removed

- [Tests] Removed the now-dead `test_get_exchange_announcements` and
  `test_get_multivariate_event_collection_lookup_history` live tests (the endpoints they exercised
  no longer exist).

### Fixed

- [Rust API] `WsChannelV2::is_private()` was missing `CfbenchmarksValue` (pre-existing gap from
  0.6.0), so subscribing to `cfbenchmarks_value` without an authenticated connection previously
  slipped past the client's auth gate. Fixed alongside adding `PythValue`, which needs the same
  treatment.
- [Tests] `tests/rest_auth.rs::test_get_account_api_limits` referenced the pre-0.6.0
  `read_limit`/`write_limit` fields removed by the 0.5.0→0.6.0 `GetAccountApiLimitsResponse`
  restructure and did not compile under `--all-features --all-targets`; updated to
  `resp.read.bucket_capacity` / `resp.write.bucket_capacity`.

### Breaking

- [Rust API] `GetQuotesParams` no longer has `market_ticker` / `event_ticker` fields — Kalshi
  removed support for filtering `GET /communications/quotes` by market or event ticker.
- [Rust API] `update_order_group_limit` now takes an additional `SubaccountQueryParams` argument
  (the `subaccount` selector moved to a query parameter, matching `reset_order_group` /
  `trigger_order_group`).

## [0.6.0] - 2026-06-08

### Compatibility

- Docs snapshot: 2026-06-08
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-06-08

**Changelog entries since 0.5.0 watermark (2026-06-04) and disposition:**

| Entry | Action |
|---|---|
| Margin fee-tier returns active rates (2026-06-03/11) | No code change — exchange bug fix only |
| Perps volume/OI notional fields on margin markets (2026-06-05/11) | No code change — margin market types not in crate |
| Tick size on `GET /margin/markets` (2026-06-03/11) | No code change — margin market types not in crate |
| Automated API rate-limit tiers / grants (2026-06-06) | **Breaking** — replaced `GetAccountApiLimitsResponse`; added `BucketLimit`, `ApiUsageLevelGrant`; added `GET /account/endpoint_costs` (`get_account_endpoint_costs`, `GetAccountEndpointCostsResponse`, `EndpointTokenCost`) |
| Fractional contract quantities for RFQs (2026-05-26/2026-06-11) | No code change — `contracts_fp` already present in `CreateRfqRequest` |
| Legacy order endpoints cost 10× rate-limit tokens (2026-06-04) | No code change — operational rate-limit change only |
| Post Only Cross Cancel `last_update_reason` value (2026-06-04) | No code change — `last_update_reason` not modeled in `Order`; tolerated by existing `extra` flatten if present |
| Transfer-scoped API key permissions (2026-06-03) | No code change — scopes stored as `Vec<String>` already |
| Block trade indicators on public trade endpoints (2026-05-29/2026-06-01) | Added `is_block_trade` to `Trade` and `GetTradesParams` |
| V2 event-order endpoints (`/portfolio/events/orders/*`) | Added all V2 types and six new `KalshiRestClient` methods |
| `cfbenchmarks_value` AsyncAPI channel | Added full channel, subscription, and message support |
| `FeeType::quadratic_with_maker_fees` | Added `QuadraticWithMakerFees` variant to `FeeType` enum |

### Added

- [Rust API] Added `is_block_trade: bool` (with `#[serde(default)]`) to the public REST `Trade`
  struct (2026-05-29). Defaults to `false` for payloads predating the flag.
- [Rust API] Added `is_block_trade: Option<bool>` filter to `GetTradesParams` so callers can filter
  by block-trade status on `GET /markets/trades` and `GET /historical/trades`.
- [Rust API] Added all V2 event-order types and six new `KalshiRestClient` methods for the lower-cost
  `/portfolio/events/orders/*` endpoints: `create_order_v2`, `cancel_order_v2`, `amend_order_v2`,
  `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`. These endpoints use a
  single price + `BookSide` instead of separate yes/no prices.
  New request/response types: `CreateOrderV2Request`, `CreateOrderV2Response`,
  `CancelOrderV2Params`, `CancelOrderV2Response`, `AmendOrderV2Request`, `AmendOrderV2Response`,
  `DecreaseOrderV2Request`, `DecreaseOrderV2Response`, `BatchCreateOrdersV2Request`,
  `BatchCreateOrderV2OrderResponse`, `BatchCreateOrdersV2Response`,
  `BatchCancelOrderV2RequestOrder`, `BatchCancelOrdersV2Request`,
  `BatchCancelOrderV2OrderResponse`, `BatchCancelOrdersV2Response`.
- [Rust API] Added `BucketLimit` and `ApiUsageLevelGrant` structs (2026-06-06). `BucketLimit` holds
  `refill_rate: i64` and `bucket_capacity: i64`. `ApiUsageLevelGrant` holds `exchange_instance`,
  `level`, `source: String`, and `expires_ts: Option<i64>` (absent for non-expiring grants).
- [Rust API] Added `get_account_endpoint_costs()` method and `GetAccountEndpointCostsResponse` /
  `EndpointTokenCost` structs for the new public `GET /account/endpoint_costs` endpoint, which lists
  API v2 endpoints whose token cost differs from the default cost.
- [Rust API] Added CF Benchmarks subscription-update support so the documented post-subscribe
  workflow is reachable: `WsUpdateAction::SubscribeIndices` / `UnsubscribeIndices` / `Indexlist`
  variants and an `index_ids: Option<Vec<String>>` field on `WsUpdateSubscriptionParamsV2`. The
  subscription tracker now folds index add/remove updates into the resubscribe state, and
  `validate_update` enforces that index actions carry no market targets and that
  `subscribe_indices` / `unsubscribe_indices` include `index_ids`.
- [Rust API] Added `FeeType::QuadraticWithMakerFees` variant (serialized
  `quadratic_with_maker_fees`). `FeeType` now also carries an `#[serde(other)] Unknown` catch-all
  so unknown future variants never panic.
- [Rust API] Added full `cfbenchmarks_value` channel support:
  - `WsChannelV2::CfbenchmarksValue` variant
  - `index_ids: Option<Vec<String>>` parameter on `WsSubscriptionParamsV2` (use `["all"]` for all
    indices)
  - `WsMsgType::CfbenchmarksValue` and `WsMsgType::CfbenchmarksValueIndexlist` variants
  - New types `WsCfBenchmarksValue`, `WsCfBenchmarksValueRef`, `WsCfBenchmarksAvgData`,
    `WsCfBenchmarksIndexList`, `WsCfBenchmarksIndexListRef` in `ws::types::messages::cfbenchmarks`
  - `WsDataMessageV2::CfbenchmarksValue` and `WsDataMessageV2::CfbenchmarksValueIndexlist` variants
    routed through both the wire and envelope parse paths

### Changed

- [Rust API] `GetAccountApiLimitsResponse` now reflects the current OpenAPI shape: nested
  `read: BucketLimit` and `write: BucketLimit` objects plus `grants: Vec<ApiUsageLevelGrant>`.
  The old flat `read_limit: i64` / `write_limit: i64` fields are removed.

### Breaking

- [Rust API] `GetAccountApiLimitsResponse` field layout changed (automated API rate-limit tiers,
  2026-06-06). Replace `resp.read_limit` → `resp.read.refill_rate` (or `.bucket_capacity`) and
  `resp.write_limit` → `resp.write.refill_rate`. The `grants` field is new; downstream exhaustive
  struct destructuring must add it.
- [Rust API] `WsUpdateAction` gained `SubscribeIndices`, `UnsubscribeIndices`, and `Indexlist`
  variants, and `WsUpdateSubscriptionParamsV2` gained an `index_ids` field. Downstream code with
  exhaustive matches over `WsUpdateAction` or struct-literal construction of
  `WsUpdateSubscriptionParamsV2` must be updated.


## [0.5.0] - 2026-05-29

### Compatibility

- Docs snapshot: 2026-05-29
- Validated through changelog: 2026-06-04

### Added

- [Rust API] Added `BookSide` enum (`Bid` | `Ask` | `Unknown`) to `types.rs` for the normalized
  `book_side` field added to order/fill responses on 2026-05-07.
- [Rust API] Added `outcome_side: Option<YesNo>` and `book_side: Option<BookSide>` fields to
  `Order`, `Fill`, `WsFill`, `WsFillRef`, and `WsUserOrder`. These are the normalized direction
  fields Kalshi added on 2026-05-07 (`bid` ≡ `yes`, `ask` ≡ `no`).
- [Rust API] Added `taker_outcome_side: Option<TradeTakerSide>` and `taker_book_side:
  Option<BookSide>` to the public `Trade` (REST) and `WsTrade` / `WsTradeRef` (WebSocket) objects,
  matching the normalized taker-direction fields added to trade responses on 2026-05-07.
- [Rust API] Added `balance_dollars: Option<FixedPointDollars>` to `GetBalanceResponse` for the
  centi-cent precision balance field added on 2026-05-28 (direct members only).
- [Rust API] Added `subaccount: Option<u32>` to `CreateOrderGroupResponse` for the field added on
  2026-05-07 (0 = primary, 1–32 = subaccount).
- [Rust API] Added `rfq_user_filter: Option<String>` to `GetQuotesParams` for the filter parameter
  added on 2026-05-07. Pass `"self"` to restrict to quotes on the authenticated user's RFQs.
- [Rust API] Added `WsMarketLifecycleEventType::MetadataUpdated` variant for the new lifecycle event
  type added on 2026-05-11, fired when market metadata (name, title, subtitles) changes.
- [Rust API] Surfaced the top-level `metadata_updated` payload values on `WsMarketLifecycleV2` /
  `WsMarketLifecycleV2Ref`: added `floor_strike: Option<f64>` and `yes_sub_title: Option<String>`
  (per AsyncAPI these appear at the top level only on `metadata_updated`, distinct from the
  `additional_metadata.*` copies emitted on creation), plus a top-level flatten `extra` map so other
  conditional lifecycle keys are no longer silently discarded.
- [Rust API] Added the `event_fee_update` WebSocket message: new `WsEventFeeUpdate` /
  `WsEventFeeUpdateRef` types, a `WsMsgType::EventFeeUpdate` variant, and
  `WsDataMessageV2::EventFeeUpdate` / `WsDataMessageRef::EventFeeUpdate` variants. This message is
  delivered on the existing `market_lifecycle_v2` channel and carries `event_ticker`,
  `fee_type_override`, and `fee_multiplier_override` (both overrides `null` when cleared).
  Previously these messages surfaced as `WsMessageV2::Unknown`.
- [Rust API] Added the spec-required `ts_ms` (matching-engine timestamp, ms) to `WsOrderGroupUpdate`
  and `WsOrderGroupUpdateRef`, which were previously dropping the field.
- [Rust API] Added `get_margin_fee_tiers()` method and `GetMarginFeeTiersResponse` struct for the
  `GET /margin/fee_tiers` endpoint. The response uses `maker_fee_rates` / `taker_fee_rates` (market
  ticker → decimal fee rate maps, fee = `notional * rate`).
- [Tests] Added `ws_fill_normalized_fields_parse` test covering the new `outcome_side` / `book_side`
  fields on `WsFill`.

### Changed

- [Rust API] Updated `KalshiEnvironment::demo()` and `KalshiEnvironment::production()` to use the
  dedicated external API hosts introduced on 2026-05-07. REST hosts: `external-api.demo.kalshi.co` /
  `external-api.kalshi.com`. WS hosts: `external-api-ws.demo.kalshi.co` /
  `external-api-ws.kalshi.com`. The old hosts (`demo-api.kalshi.co`, `api.elections.kalshi.com`)
  are no longer used.

### Breaking

- [Rust API] `Order.side` changed from `YesNo` to `Option<YesNo>`. The `side` field was deprecated
  by Kalshi on 2026-05-07 and removed ~2026-05-28. Downstream code must use `outcome_side` (or
  handle `None`).
- [Rust API] `Order.action` changed from `BuySell` to `Option<BuySell>`. Same deprecation/removal
  timeline as `Order.side`. Use `book_side` instead.
- [Rust API] `Fill.side` changed from `YesNo` to `Option<YesNo>` for the same reason.
- [Rust API] `Fill.action` changed from `BuySell` to `Option<BuySell>` for the same reason.
- [Rust API] `WsFill.side` changed from `YesNo` to `Option<YesNo>` for the same reason.
- [Rust API] `WsFill.action` changed from `BuySell` to `Option<BuySell>` for the same reason.
- [Rust API] `Trade.taker_side` and `WsTrade.taker_side` changed from `TradeTakerSide` to
  `Option<TradeTakerSide>`. The `taker_side` field was deprecated on 2026-05-07 in favor of
  `taker_outcome_side` / `taker_book_side`. Downstream code must handle `None`.
- [Rust API] `KalshiEnvironment::demo()` and `KalshiEnvironment::production()` now point to the new
  dedicated external API hostnames. Code that hard-coded the old host strings must update.
- [Upstream] `GET /margin/fee_tiers` response no longer returns `maker_fee_tiers` /
  `taker_fee_tiers` tier-name maps; it now returns `maker_fee_rates` / `taker_fee_rates` decimal
  maps. `GetMarginFeeTiersResponse` was added with the new shape (no old shape existed in this
  crate).


## [0.4.0] - 2026-04-18

### Compatibility

- Docs snapshot: 2026-04-18
- OpenAPI: 3.13.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-04-16

### Added

- [Rust API] Added REST helpers for current Kalshi endpoints and aliases, including `get_market_orderbooks`, `get_trades_historical`, `get_fills_historical`, `get_live_data_by_milestone`, `get_game_stats`, and `get_market_candlesticks_historical`.
- [Rust API] Added current OpenAPI fields used by the refreshed docs, including `occurrence_datetime` on event and market payloads, `series_ticker` on historical market filters, and fixed-point quote contract fields.
- [Docs] Added `VERSIONING.md` plus repo guidance that points refresh work at the live Kalshi docs, changelog RSS, OpenAPI, and AsyncAPI documents instead of checked-in spec snapshots.

### Changed

- [Rust API] Restored `GetOrderQueuePositionsParams` to the current OpenAPI behavior by allowing unfiltered queue-position requests.
- [Rust API] Migrated the WebSocket public surface to the current V2 contract, including `WsChannelV2`, `WsMessageV2`, `WsDataMessageV2`, `WsSubscriptionParamsV2`, and the `subscribe_v2` / `unsubscribe_v2` / `update_subscription_v2` / `start_reader_v2` / `next_event_v2` methods.
- [Rust API] Aligned authenticated REST response structs with the current OpenAPI fixed-point contract for `Order`, `Trade`, `Fill`, `Settlement`, `MarketPosition`, and `EventPosition`.
- [Rust API] Aligned communications REST and WebSocket quote/RFQ payloads with the current fixed-point-only docs by removing stale integer compatibility fields and relying on `*_dollars` and `*_fp` fields.
- [Upstream] Validated the current Kalshi docs snapshot against the changelog items covering historical `series_ticker` filtering, fixed-point response cleanup, millisecond WebSocket timestamps, and `occurrence_datetime` on market responses.
- [Tests] Refreshed parsing fixtures to the current OpenAPI/AsyncAPI field sets, added coverage for `occurrence_datetime`, and added deterministic V2 WebSocket command-behavior coverage.
- [Tests] Updated live integration coverage to use the filters and account-scope assumptions required by the current communications, queue-position, and FCM-only portfolio endpoints.
- [Upstream] Updated docs, examples, and tests for Kalshi's current WebSocket handshake behavior, which now requires authenticated connections even when subscribing only to public channels.
- [Docs] Tightened the refresh workflow to remove upstream-removed schema fields and response shapes from the public Rust API instead of preserving compatibility shims by default.

### Removed

- [Docs] Removed vendored OpenAPI/AsyncAPI snapshots, spec manifest artifacts, the parity generation script, and raw spec contract tests in favor of live upstream docs plus concise `docs/spec-parity.md` notes.
- [Rust API] Removed stale REST compatibility fields and aliases that are no longer present in the current OpenAPI, including legacy fill/settlement fixed-point aliases.
- [Rust API] Removed stale WebSocket fill aliases for `yes_price_fixed` and `no_price_fixed` so parsing follows the current AsyncAPI names.
- [Rust API] Removed stale quote and RFQ integer compatibility fields from REST and WebSocket communications payloads.
- [Rust API] Removed stale WebSocket compatibility fields and shapes from `WsTicker`, `WsTrade`, `WsOrderbookSnapshot`, `WsOrderbookDelta`, and `WsFill`; downstream consumers must use the current `*_dollars` and `*_fp` fields from the live AsyncAPI contract.
- [Rust API] Removed the stale `GetMarketOrderbookResponse.orderbook` compatibility view and its synthesized integer orderbook shape; the current OpenAPI response is `orderbook_fp` only.

### Breaking

- [Rust API] Downstream WebSocket code must migrate from the pre-V2 types and methods such as `WsChannel`, `WsMessage`, `WsDataMessage`, `subscribe`, `unsubscribe`, `update_subscription`, `start_reader`, and `next_event` to the V2 names and `*_v2` methods.
- [Rust API] `KalshiWsClient::connect` and `KalshiWsLowLevelClient::connect` no longer provide an unauthenticated public-channel path; downstream code must use `connect_authenticated`, even for public subscriptions.
- [Rust API] V2 subscription validation is stricter: `orderbook_delta` requires `market_ticker` or `market_tickers`, rejects `market_id` and `market_ids`, and enforces exclusive market-target fields on subscribe and update commands.
- [Rust API] Downstream code must update authenticated REST response field access to the current spec names such as `fill_count_fp`, `remaining_count_fp`, `initial_count_fp`, `last_update_time`, `subaccount_number`, `total_traded_dollars`, `market_exposure_dollars`, `total_cost_dollars`, and `total_cost_shares_fp`.
- [Rust API] Legacy integer/count response fields and compatibility aliases previously accepted by `Order`, `Trade`, `Fill`, `Settlement`, `MarketPosition`, and `EventPosition` are no longer exposed by the public Rust types.
- [Rust API] Downstream WebSocket code can no longer access removed compatibility fields such as `price`, `yes_bid`, `yes_ask`, `volume`, `open_interest`, `count`, `yes_price`, `no_price`, `delta`, `no_price_dollars`, or the legacy integer orderbook snapshot levels on current V2 message types.
- [Rust API] Downstream REST code must read `GetMarketOrderbookResponse.orderbook_fp` directly; the legacy `orderbook` field has been removed.

## [0.3.0] - 2026-03-05

### Compatibility

- Not recorded for this historical release.

### Added

- [Rust API] Added `MarketStatusConversionError` for strict lifecycle/query status conversions.
- [Rust API] Added best-effort `From` conversions between lifecycle `MarketStatus` and query `MarketStatusQuery`.
- [Rust API] Added strict `TryFrom<&...>` conversions for exact one-to-one status mapping.
- [Tests] Added and expanded parsing tests for status serialization and conversion behavior.
- [Rust API] Added `KalshiError::Parse` with parse context, human-readable reason, raw payload bytes, and optional serde source error.
- [Rust API] Added public parse accessors on `KalshiError`: `parse_context()`, `parse_error_reason()`, and `parse_raw_bytes()`.
- [Tests] Added regression tests covering REST and WebSocket parse failures to verify reason text and raw-byte preservation.

### Changed

- [Rust API] Renamed query enum `MarketStatus` to `MarketStatusQuery`.
- [Rust API] Renamed REST market lifecycle enum `MarketState` to `MarketStatus`.
- [Rust API] Updated `GetMarketsParams.status` to use `Option<MarketStatusQuery>`.
- [Rust API] Updated `Market.status` to use `Option<MarketStatus>`.
- [Docs] Updated examples, tests, and REST module docs to use the new names.
- [Rust API] REST success-response decoding now returns `KalshiError::Parse` with raw bytes instead of a plain serde JSON error.
- [Rust API] WebSocket envelope and message parsing now returns `KalshiError::Parse` with clearer parse-failure context and preserved raw payload bytes.

### Removed

- [Rust API] Removed old `MarketState` and old query `MarketStatus` names without aliases.

### Breaking

- [Rust API] Downstream consumers must update imports and enum references to the new names.
- [Rust API] Downstream exhaustive `match` statements over `KalshiError` must handle the new `Parse` variant.
