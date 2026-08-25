# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-25

### Compatibility

- Docs snapshot: 2026-08-25
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-27

**Scope note:** this crate targets the Predictions REST/WebSocket API only (no
Margin or FIX support). Changelog entries tagged `Margin`-only or `FIX`-only
with no `Predictions` REST/WebSocket component are out of scope and are
omitted from the table below (perps mark prices, margin fee-tier rates,
margin risk metrics, margin order groups, margin order reasons, sided
leverage estimates, margin maker-volume incentive programs, margin exit
triggers, and all FIX-only tag/session/session-flag entries).

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Entry (date) | Action |
|---|---|
| API usage volume progress endpoint (6/11) | Added `get_account_api_usage_level_volume_progress`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal`, `GetAccountApiUsageLevelVolumeProgressResponse` |
| Self-serve Advanced API usage tier upgrade (6/11) | Added `upgrade_account_api_usage_level` |
| Fractional quantities for RFQs (6/11) | No code change — `contracts_fp` already present on `CreateRFQRequest` |
| `settlement_sources` added to events API (6/18) | Added `EventData.settlement_sources` |
| Strike type and cap strike on `metadata_updated` (6/18) | Added top-level `strike_type`/`cap_strike` to `WsMarketLifecycleV2`/`Ref` |
| Legacy order mutation endpoints deprecated (6/18) | No code change — advance notice only, endpoints still modeled |
| Event tickers filter on `GET /events` (6/18) | Added `GetEventsParams.tickers` (+ `min_updated_ts`, found via field-by-field check) |
| Block-trade accept API key permissions (6/18) | No code change — scopes already modeled as `Vec<String>` |
| Sanity limits enforced on orderbook subscriptions (6/18) | No code change — server-side rejection only, surfaced through the existing generic error path |
| Quote time filters and pagination fix (6/18) | Added `GetQuotesParams.min_ts`/`max_ts` |
| Communications RFQ/quote retention window reduced to 7d (6/19) | No code change — operational retention window only; noted in `docs/spec-parity.md` |
| RFQ quote `market_ticker`/`event_ticker` filters removed (6/20) | **Breaking** — removed `GetQuotesParams.market_ticker`/`event_ticker` |
| Get Quote rate-limit cost reduced (6/23) | No code change — rate-limit/operational only |
| RFQ quote retention & RFQ-scoped quote actions (6/25, 7/9) | Added `get_rfq_quote`/`delete_rfq_quote`; deprecated `get_quote`/`delete_quote` |
| API usage tier qualification requirements halved (6/25) | No code change — operational only |
| Trade-scoped API key permissions (6/30) | No code change — scopes already modeled as `Vec<String>` |
| Multivariate lookup history endpoints fully deprecated (7/2) | No code change at this point; see removal below (8/6) |
| `price_ranges` added to `market_lifecycle_v2` events (7/2) | Added `price_ranges: Option<Vec<PriceRange>>` to `WsMarketLifecycleV2`/`Ref` |
| Per-index exchange status (7/2) | Added `exchange_index_statuses`, `intra_exchange_transfers_active` to `GetExchangeStatusResponse`; new `ExchangeIndexStatus` |
| Per-index subaccount balances (7/2) | Added `SubaccountBalance.exchange_index` |
| Sub-account-restricted API keys (7/2) | Added `ApiKey.subaccount`, `CreateApiKeyRequest.subaccount` |
| Exchange announcements endpoint removed (7/4) | **Breaking** — removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| RFQ-scoped quote lookup endpoint (7/9) | Same as 6/25 entry above |
| Deprecated Predictions REST schema fields removed (7/9) | **Breaking** — removed `Market.response_price_units`/`fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| Incentive programs on hidden events excluded (7/22) | No code change — server-side filtering only |
| Order groups limited to 25,000 per user (7/23) | No code change — operational limit only |
| Historical positions endpoint (7/23) | Added `get_historical_positions`, `GetHistoricalPositionsParams`; added `GetHistoricalCutoffResponse.market_positions_last_updated_ts` |
| Subaccount-restricted API keys can open WebSocket sessions (7/23) | No code change — permission-only |
| Pyth value WebSocket channel (7/23) | Added full `pyth_value` channel: `WsChannelV2::PythValue`, `WsPythValue(Ref)`, `WsPythUnderlyingList(Ref)`, `WsUpdateAction::{SubscribeUnderlyings,UnsubscribeUnderlyings,UnderlyingList}`, `underlying_tickers` params, subscription-tracker resubscribe support |
| New price level structures (7/23) | No code change — `price_level_structure` already modeled as a tolerant raw string |
| `service` field on error responses deprecated (7/28) | No code change yet; see removal below (8/6) |
| Richer combo-validation errors on multivariate market creation (7/30) | No code change — conveyed via the existing `ErrorResponse.details` string |
| Lifecycle creation messages include `exchange_index` (7/30) | Added `WsMarketLifecycleV2.exchange_index` |
| Series responses include `exchange_index` (7/30) | Added `Series.exchange_index` |
| New endpoint for event-keyed live data (7/30) | Added `get_event_live_data`, `EventLiveData`, `GetEventLiveDataResponse`, `GetEventLiveDataParams` |
| Subaccount-restricted API keys can read order queue positions (7/30) | No code change — permission-only |
| Event `product_metadata` now includes `cadence` (7/30) | Added `EventMetadata.cadence` (see `docs/spec-parity.md` for the opaque-object nuance) |
| Subaccount-restricted API keys can use batch order endpoints (7/30) | No code change — permission-only |
| Subaccount on `quote_created` (7/30) | Added `WsQuoteCreated.subaccount` (+ `WsQuoteAccepted`/`WsQuoteExecuted`, found via field-by-field check) |
| Subaccount-restricted API keys can manage order groups (7/30) | No code change — permission-only |
| Multivariate lookup endpoint and channel removed (8/6) | **Breaking** — removed REST lookup endpoints/types (see below); removed `WsChannelV2::Multivariate` from the subscribable channel enum |
| Order group limit updates support subaccounts (8/6) | **Breaking** — `update_order_group_limit` now takes a `SubaccountExchangeIndexQueryParams`; also applied to `delete_order_group`/`reset_order_group`/`trigger_order_group` (found via field-by-field check against `DeleteOrderGroup`/`ResetOrderGroup`/`TriggerOrderGroup`) |
| Multivariate event collections include `exchange_index` (8/6) | Added `MultivariateEventCollection.exchange_index` |
| `service` field removed from error responses (8/6) | **Breaking** — removed `ErrorResponse.service` |
| New `center_deci_edge_centi_cent` price level structure (8/13, 8/27) | No code change — raw string already tolerant |
| Balance reads scoped by `exchange_index` (8/13, 8/20) | Added `GetBalanceParams`, `GetBalanceResponse.balance_breakdown`, `IndexedBalance` |
| Block trade indicator for WebSocket trades (8/13) | Added `WsTrade.is_block_trade` |
| Exchange shard descriptions (8/13) | Added `ExchangeIndexStatus.description` |
| Order group maximum increased to 100,000 (8/13) | No code change — operational limit only |
| Intra-account transfer history endpoints (8/13) | Added `intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfer(s)` + pager/stream, and supporting types |
| API key location attestation expiry (8/16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| VPC peering for Prime members (8/20) | No code change — account/infra feature, not an API surface change |
| Kalshi Weather Index endpoint (8/20) | Added `get_weather_index`, `GetWeatherIndexResponse`, `WeatherIndexPoint`, `WeatherIndexStationReading`, `GetWeatherIndexParams` |
| Maker fee exemption for independent NFL combo markets (8/20) | No code change — fee computation is server-side |
| Cross-shard subaccount transfers (8/20) | Added `ApplySubaccountTransferRequest.exchange_index` |
| Target balance allocation endpoints (8/20) | Added `get_target_balance_allocation`, `set_target_balance_allocation`, `TargetBalanceAllocation`, `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest` |
| Resting order value breakdown by exchange index (8/13, 8/20) | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| Exchange index on portfolio and WebSocket fill records (8/20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition` (REST) and `WsFill`/`WsFillRef` (WS) |
| Exchange index filters for portfolio lists (8/20) | Added `exchange_index` filter to `GetPositionsParams`, `GetFillsParams` |
| RFQs and combo-market creation for sub-account-restricted API keys (8/20) | No code change — permission-only |
| Optional balance reads by exchange_index (8/20) | Same as 8/13 balance entry above |
| Post-only quotes preserved; crossing rate limits may apply (8/22) | No code change — operational/rate-limit policy only |
| Combo RFQ fee assignment for briefly resting orders (8/22) | No code change — fee computation is server-side |
| Upcoming exchange sharding (8/24) | No code change — advance notice only |
| Exchange index on user order messages (8/27) | Added `WsUserOrder.exchange_index` |
| Historical CF Benchmarks values via REST passthrough (8/27) | No code change — documentation-only; existing passthrough endpoint unaffected |
| `available_on_brokers` field deprecated (8/27) | Marked `EventData.available_on_brokers` `#[deprecated]` |
| Exchange auto-routing enabled by default (8/27) | No code change — server-side routing behavior only |
| Tapered sub-cent pricing on multivariate (combo) markets (8/27) | No code change — `price_level_structure`/`price_ranges` already tolerant |

**Additional field-by-field findings** (grepped while touching each struct above, not tied to a single changelog title):

- `EventData` gained `exchange_index`, `fee_type_override`, `fee_multiplier_override` (all present in the current `required`-adjacent schema but not driven by a single changelog entry).
- `RFQ.creator_subaccount`; `Quote.post_only`, `.creator_subaccount`, `.rfq_creator_subaccount`; `CreateQuoteRequest.post_only`.
- `Order.exchange_index`; `OrderGroup.exchange_index`; `CreateOrderGroupRequest.exchange_index`.
- `GetOrderGroupResponse` fixed to match the live schema: `orders` is `Vec<String>` (order IDs), not `Vec<Order>`; removed the non-existent plain `contracts_limit` field (only `contracts_limit_fp` is in the current schema).
- `Market.exchange_index`.
- `WsFill.purchased_side` changed to `Option<YesNo>` (spec marks it `deprecated: true`, matching the crate's existing `side`/`action` tolerance pattern).
- `MarketPositionRef`/`WsFillRef`/`WsMarketLifecycleV2Ref` zero-copy paths updated to keep parity with their owned counterparts.
- Pre-existing test-suite bugs unrelated to this cycle's spec drift, fixed while touching the surrounding code: a stale `ListSubscriptions { id, subscriptions }` match arm (missing `sid`/`seq` added in a prior release) that failed to compile under `--tests`, and a live-test assertion still reading the removed `GetAccountApiLimitsResponse.read_limit`/`write_limit` fields.

### Added

- [Rust API] `POST /account/api_usage_level/upgrade` and `GET /account/api_usage_level/volume_progress`
  (`upgrade_account_api_usage_level`, `get_account_api_usage_level_volume_progress`).
- [Rust API] Full intra-exchange transfer support: `intra_exchange_instance_transfer`,
  `get_intra_exchange_instance_transfer(s)` with pager/stream/`_all`, and
  `IntraExchangeInstanceTransfer(Request/Response/Status)`.
- [Rust API] `GetApiKeysResponse.api_key_region_expiration_ts`; `ApiKey.subaccount` and
  `CreateApiKeyRequest.subaccount` for sub-account-restricted API keys.
- [Rust API] `portfolio/balance` now accepts `GetBalanceParams { subaccount, exchange_index }` and
  the response carries `balance_breakdown: Option<Vec<IndexedBalance>>`.
- [Rust API] `GET`/`POST /portfolio/target_balance_allocation` (`get_target_balance_allocation`,
  `set_target_balance_allocation`).
- [Rust API] `exchange_index` filters on `GetPositionsParams`/`GetFillsParams`; `exchange_index` on
  `Fill`, `Settlement`, `MarketPosition`; `resting_order_value_breakdown` on
  `GetPortfolioRestingOrderTotalValueResponse`.
- [Rust API] `EventData.settlement_sources`, `.exchange_index`, `.fee_type_override`,
  `.fee_multiplier_override`; `EventMetadata.cadence`; `GetEventsParams.tickers`/`.min_updated_ts`;
  `Series.exchange_index`; `EventStatus::Unopened` variant.
  `MultivariateEventCollection.exchange_index`; `Market.exchange_index`.
- [Rust API] `GET /historical/positions` (`get_historical_positions`); `GetHistoricalCutoffResponse
  .market_positions_last_updated_ts`.
- [Rust API] `GET /live_data/events/{event_ticker}` (`get_event_live_data`) and
  `GET /live_data/weather/{city}` (`get_weather_index`), with full response types.
- [Rust API] `Order.exchange_index`; `OrderGroup.exchange_index`; `CreateOrderGroupRequest
  .exchange_index`; `SubaccountExchangeIndexQueryParams` (used by `delete_order_group`,
  `reset_order_group`, `trigger_order_group`, `update_order_group_limit`).
- [Rust API] `get_rfq_quote`/`delete_rfq_quote` (RFQ-scoped quote lookup/delete);
  `GetQuotesParams.min_ts`/`.max_ts`; `RFQ.creator_subaccount`; `Quote.post_only`
  `.creator_subaccount`/`.rfq_creator_subaccount`; `CreateQuoteRequest.post_only`.
- [Rust API] `exchange_index_statuses`/`intra_exchange_transfers_active` on
  `GetExchangeStatusResponse`; new `ExchangeIndexStatus`.
- [WebSocket] Full `pyth_value` channel support (see disposition table above).
- [WebSocket] `WsMarketLifecycleV2`/`Ref`: top-level `exchange_index`, `price_ranges`, `strike_type`,
  `cap_strike`.
- [WebSocket] `WsUserOrder.exchange_index`; `WsFill`/`Ref.exchange_index`;
  `WsTrade`/`Ref.is_block_trade`; `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted.subaccount`.
- [Rust API] `FeeType::QuadraticWithComboMakerFees` variant.

### Changed

- [Rust API] `create_subaccount` now takes `Option<CreateSubaccountRequest>` (was `&self` only) to
  support the new `exchange_index` field.
- [Rust API] `get_balance` now takes `GetBalanceParams` (was `&self` only).
- [WebSocket] `WsFill.purchased_side` is now `Option<YesNo>` (was `YesNo`) to tolerate the field's
  new `deprecated: true` status.

### Breaking

- [Rust API] Removed `ErrorResponse.service` (removed from the OpenAPI schema 2026-08-06).
  Downstream code reading `.service` no longer compiles.
- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` (endpoint removed from the API 2026-07-04).
- [Rust API] Removed `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types
  (endpoint removed from the API 2026-08-06). Removed `WsChannelV2::Multivariate` from the
  subscribable channel enum for the same reason; the AsyncAPI-vestigial `multivariate`/
  `multivariate_lookup` wire/envelope parsing is kept (never reachable via the typed subscribe API)
  to tolerate any lingering server messages — see `docs/spec-parity.md`.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` (removed from the OpenAPI schema 2026-07-09). Removed the
  corresponding `WsMarketLifecycleV2`/`Ref.fractional_trading_enabled` field and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant (not in the current AsyncAPI
  `event_type` enum).
- [Rust API] Removed `GetQuotesParams.market_ticker`/`.event_ticker` (removed from the OpenAPI
  schema 2026-06-20).
- [Rust API] `GetOrderGroupResponse.orders` changed from `Vec<Order>` to `Vec<String>` (order IDs) to
  match the live schema; removed `GetOrderGroupResponse.contracts_limit` (only `contracts_limit_fp`
  exists in the current schema).
- [Rust API] `create_subaccount`, `get_balance` signatures changed (see Changed above).
- [Rust API] `update_order_group_limit` now takes an additional `SubaccountExchangeIndexQueryParams`
  argument; `delete_order_group`/`reset_order_group`/`trigger_order_group` now take
  `SubaccountExchangeIndexQueryParams` instead of `SubaccountQueryParams`.
- [WebSocket] `WsUpdateAction` gained `SubscribeUnderlyings`, `UnsubscribeUnderlyings`,
  `UnderlyingList` variants; `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2` gained an
  `underlying_tickers` field. Downstream exhaustive matches or struct-literal construction must be
  updated.
- [WebSocket] `WsFill.purchased_side` type changed (see Changed above).


## [0.7.0] - 2026-08-12

### Compatibility

- Docs snapshot: 2026-06-08
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-06-08

### Fixed

- [Rust API] Preserved Kalshi WebSocket subscription cursor metadata on unknown frames in both
  owned and borrowed parsing paths. `WsMessageV2::subscription_id()` / `.sequence()` and
  `WsMessageRef::subscription_id()` / `.sequence()` now return unknown-frame `sid` / `seq`.

### Breaking

- [Rust API] All public `WsMessageV2` and `WsMessageRef` control and `Unknown` variants carry
  `sid` and `seq`. Downstream constructors and exhaustive matches must supply or handle the new
  fields (or use `..`). Consumers accounting for the per-subscription cursor MUST use
  `subscription_id()` and `sequence()` rather than matching message variants.


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
