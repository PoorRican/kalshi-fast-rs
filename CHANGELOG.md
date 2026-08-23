# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-23

### Compatibility

- Docs snapshot: 2026-08-23
- OpenAPI: 3.28.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-27

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition.** Grouped where several
entries share one justification (91 entries total since watermark).

| Entry (date) | Action |
|---|---|
| `settlement_sources` on events (06-18) | Added `EventData.settlement_sources` |
| Strike type/cap strike/custom strike on `metadata_updated` (06-18) | Added top-level `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2`(`Ref`) |
| Event tickers filter on `GET /events` (06-18) | Added `GetEventsParams.tickers` |
| Quote `min_ts`/`max_ts` filters (06-18) | Added `GetQuotesParams.min_ts`/`max_ts` (and the previously-missing `user_filter`) |
| Quote pagination fix (06-18) | No code change — server-side fix |
| RFQ quote market/event filters removed (06-20) | **Breaking** — removed `GetQuotesParams.market_ticker`/`event_ticker` |
| RFQ-scoped quote lookup/actions (06-25, 07-09) | Added `get_quote_scoped`, `delete_quote_scoped`, `accept_quote_scoped`, `confirm_quote_scoped`; legacy quote-ID-only methods kept (now doc-marked deprecated) |
| `price_ranges` on `market_lifecycle_v2` events (07-02) | Added `WsMarketLifecycleV2.price_ranges` |
| Per-index exchange status (07-02) | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses`, new `ExchangeIndexStatus` |
| Per-index subaccount balances (07-02) | **Breaking** — added `SubaccountBalance.exchange_index`; a subaccount with funds on multiple indexes now returns multiple rows |
| Sub-account-restricted API keys (07-02) | Added `subaccount` to `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey` |
| Exchange announcements endpoint removed (07-04) | **Breaking** — removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| Deprecated Predictions REST fields removed (07-09) | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| Historical positions endpoint (07-23) | Added `get_historical_positions` (reuses `GetPositionsResponse`) |
| Pyth value WebSocket channel (07-23) | Added full `pyth_value` channel: `WsChannelV2::PythValue`, `WsPythValue(Ref)`, `WsPythUnderlyingList(Ref)`, `WsUpdateAction::{SubscribeUnderlyings,UnsubscribeUnderlyings,UnderlyingList}`, `underlying_tickers` on subscribe/update params |
| New/renamed `price_level_structure` values (06-13, 07-23, 08-13, 08-27) | No code change — modeled as a raw `String`, not an enum |
| Lifecycle creation messages include `exchange_index` (07-30) | Added `exchange_index` to `WsMarketLifecycleV2`, `WsEventLifecycle` |
| Series responses include `exchange_index` (07-30) | Added `Series.exchange_index` |
| Event-keyed live data endpoint (07-30) | Added `get_event_live_data`, `EventLiveData` |
| Event `product_metadata.cadence` (07-30) | No code change — `EventData.product_metadata` already captures unknown keys via `EventMetadata.extra` |
| Subaccount on `quote_created` (07-30) | Added `WsQuoteCreated.subaccount` (and the previously-missing `rfq_creator_id`) |
| Multivariate lookup endpoint + channel removed (07-02 deprecation, 08-06 removal) | **Breaking** — removed `PUT .../lookup`, `GET .../lookup` history endpoint, `lookup_tickers_for_market_in_multivariate_event_collection`, `WsChannelV2::Multivariate`, `WsMsgType::Multivariate(Lookup)`, `WsDataMessageV2/Ref::Multivariate`, the `ws::types::messages::multivariate` module |
| Order group limit supports `subaccount` (08-06) | **Breaking** — `update_order_group_limit` now takes a `SubaccountQueryParams` argument (which also gained `exchange_index`, applied to the other single-order-group endpoints) |
| Multivariate event collections include `exchange_index` (08-06) | Added `MultivariateEventCollection.exchange_index` |
| `service` field removed from error responses (07-28 deprecation, 08-06 removal) | **Breaking** — removed `ErrorResponse.service` |
| Balance reads scoped by `exchange_index` (08-13, 08-20) | **Breaking** — `get_balance` now takes a `GetBalanceParams { subaccount, exchange_index }` argument; added `GetBalanceResponse.balance_breakdown` |
| Block trade indicator for WS trades (08-13) | Added `WsTrade.is_block_trade` |
| Exchange shard descriptions (08-13) | Added `ExchangeIndexStatus.description` |
| Intra-account transfer history endpoints (08-13) | Added `intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfer(s)` and supporting types |
| API key location attestation expiry (08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| Kalshi Weather Index endpoint (08-20) | Added `get_weather_index`, `GetWeatherIndexResponse` |
| Cross-shard subaccount transfers (08-20) | Added `source_subaccount`/`destination_subaccount` to `IntraExchangeInstanceTransferRequest` |
| Target balance allocation endpoints (08-20) | Added `get_target_balance_allocation`, `set_target_balance_allocation` |
| Resting order value breakdown by exchange index (08-20) | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| Exchange index on portfolio/WS fill records (08-20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition`, `Order`, `OrderGroup` family, `WsFill` |
| Exchange index filters for portfolio lists (08-20) | Added `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| Optional balance reads by `exchange_index` (08-20) | Covered by the `GetBalanceParams` change above |
| `available_on_brokers` deprecated (08-27) | No code change — already `Option<bool>` |
| Margin maker-volume incentive programs (08-27) | Added `IncentiveProgram.max_reward_per_account` |
| Post-only quotes preserved (08-22 policy reversal) | Added the previously-missing `CreateQuoteRequest.post_only` (the underlying field predates this watermark; the reversal announcement surfaced the gap) |
| `FeeType::QuadraticWithComboMakerFees` (OpenAPI enum, undated) | Added variant (the pre-existing `#[serde(other)] Unknown` catch-all meant this was never a hard-failure gap) |
| API usage volume progress endpoint (06-11) | Added `get_account_api_usage_level_volume_progress` |
| Self-serve Advanced API usage tier upgrade (06-11) | Added `upgrade_account_api_usage_level` |
| Fractional quantities for RFQs (06-11) | No code change — `contracts_fp` and fixed-point quote fields already present |
| FIX-only entries (06-11, 06-18×2, 06-24, 06-25, 07-02×2, 07-09, 07-23, 08-06, 08-13, 08-20) | No code change — FIX protocol is out of scope (crate is REST/WebSocket only, per `CLAUDE.md`) |
| Margin-only entries (06-11×3, 06-26, 06-29, 07-02, 07-09, 08-06, 08-13, 08-20, 08-27 margin risk/positions/orders/markets/order-groups/exit-triggers) | No code change — margin market/position types are not modeled by this crate (only `get_margin_fee_tiers` is) |
| Operational/limit/permission/policy entries with no response-shape change (06-18 sanity limits & block-trade scopes, 06-19, 06-23, 06-25 tier qualification, 06-30, 07-22, 07-23 order-group limit, 07-23/07-30/08-20 subaccount-restricted-key permission grants, 07-28 deprecation notice, 08-13 order-group max, 08-20 maker-fee exemption & RFQ/combo permission, 08-22 combo fee assignment, 08-24 sharding notice, 08-27 auto-routing default) | No code change — operational, rate-limit, or permission-scope behavior with no new/changed response field |
| Historical CF Benchmarks values via REST passthrough (08-27, docs-only) | No code change — documents an existing upstream endpoint the crate does not model (`GET /cfbenchmarks/*`); noted for a future refresh |

### Added

- [Rust API] `EventData`: `settlement_sources: Option<Vec<SettlementSource>>`, `exchange_index: Option<i64>`, `fee_type_override: Option<String>`, `fee_multiplier_override: Option<f64>`.
- [Rust API] `GetEventsParams.tickers: Option<Vec<String>>` (comma-separated event ticker filter).
- [Rust API] `GetQuotesParams`: `user_filter`, `min_ts`, `max_ts`.
- [Rust API] RFQ-scoped quote methods: `get_quote_scoped`, `delete_quote_scoped`, `accept_quote_scoped`, `confirm_quote_scoped` (all take `rfq_id` + `quote_id`).
- [Rust API] `WsMarketLifecycleV2`(`Ref`): top-level `exchange_index`, `price_ranges`, `strike_type`, `cap_strike`, `custom_strike` (in addition to the pre-existing top-level `floor_strike`/`yes_sub_title`, all present only on the relevant event types per the AsyncAPI).
- [Rust API] `WsEventLifecycle`(`Ref`): `exchange_index`.
- [Rust API] `GetExchangeStatusResponse`: `intra_exchange_transfers_active`, `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>`. New `ExchangeIndexStatus` type (`exchange_index`, `description`, `exchange_active`, `trading_active`, `intra_exchange_transfers_active`).
- [Rust API] `SubaccountBalance.exchange_index`, `ApiKey.subaccount`, `GetApiKeysResponse.api_key_region_expiration_ts`, `CreateApiKeyRequest.subaccount`, `GenerateApiKeyRequest.subaccount`.
- [Rust API] `CreateSubaccountRequest { exchange_index }` and `create_subaccount` now takes it as a parameter.
- [Rust API] `get_historical_positions` (reuses `GetPositionsResponse`).
- [Rust API] Full `pyth_value` WebSocket channel: `WsChannelV2::PythValue`; `WsPythValue`/`WsPythValueRef`, `WsPythUnderlyingList`/`WsPythUnderlyingListRef`; `WsMsgType::PythValue`/`PythValueUnderlyingList`; `WsDataMessageV2`/`WsDataMessageRef::PythValue`/`PythValueUnderlyingList`; `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`; `underlying_tickers` on `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2`. The subscription tracker folds underlying add/remove updates into resubscribe state the same way it already did for CF Benchmarks indices. `WsUpdateSubscriptionParamsV2` and `WsUpdateAction` now derive `Default`.
- [Rust API] `Series.exchange_index`.
- [Rust API] `get_event_live_data` / `EventLiveData` / `GetEventLiveDataResponse` / `GetEventLiveDataParams` for `GET /live_data/events/{event_ticker}`.
- [Rust API] `get_weather_index` / `GetWeatherIndexResponse` / `WeatherIndexPoint` / `WeatherIndexStationReading` / `GetWeatherIndexParams` for `GET /live_data/weather/{city}`.
- [Rust API] `WsQuoteCreated`(`Ref`): `rfq_creator_id`, `subaccount`.
- [Rust API] `MultivariateEventCollection.exchange_index`.
- [Rust API] `SubaccountQueryParams.exchange_index`; `update_order_group_limit` now takes a `SubaccountQueryParams` argument.
- [Rust API] `GetBalanceParams { subaccount, exchange_index }`; `GetBalanceResponse.balance_breakdown: Option<Vec<IndexedBalance>>`. New `IndexedBalance` type.
- [Rust API] `WsTrade`(`Ref`).`is_block_trade`.
- [Rust API] `get_intra_exchange_instance_transfer(s)` / `intra_exchange_instance_transfer` and supporting types (`IntraExchangeInstanceTransfer`, `IntraExchangeInstanceTransferRequest`/`Response`, `IntraExchangeInstanceTransferStatus`, pager/stream helpers).
- [Rust API] `get_target_balance_allocation` / `set_target_balance_allocation` and supporting types (`TargetBalanceAllocation`, `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest`).
- [Rust API] `get_account_api_usage_level_volume_progress` / `upgrade_account_api_usage_level` and supporting types (`AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal`, `GetAccountApiUsageLevelVolumeProgressResponse`).
- [Rust API] `exchange_index: Option<i64>` added to: `Market`, `MarketPosition`, `Settlement`, `Fill`, `Order`, `OrderGroup`, `GetOrderGroupResponse`, `CreateOrderGroupRequest`, `CreateOrderGroupResponse`, `CreateOrderRequest`, `AmendOrderRequest`, `BatchCancelOrdersRequestOrder` (which also gained `market_ticker`), and `WsFill`(`Ref`). `exchange_index` filter added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] `IncentiveProgram.max_reward_per_account`.
- [Rust API] `CreateQuoteRequest.post_only` (field existed upstream before this watermark; added now while touching this type for the RFQ-scoping work).
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Option<Vec<IndexedBalance>>`.
- [Rust API] `FeeType::QuadraticWithComboMakerFees`.

### Changed

- [Rust API] `get_balance` now takes a `GetBalanceParams` argument instead of no parameters.
- [Rust API] `create_subaccount` now takes a `CreateSubaccountRequest` argument instead of no parameters.
- [Rust API] `update_order_group_limit` now takes an additional `SubaccountQueryParams` argument.

### Fixed

- [Rust API] `WsChannelV2::CfbenchmarksValue` was missing from `WsChannelV2::is_private()`, so `subscribe_v2` did not proactively reject an unauthenticated `cfbenchmarks_value` subscription with `KalshiError::AuthRequired` even though the AsyncAPI states the channel requires authentication. Fixed; `PythValue` was added to the same list from the start.

### Removed

- [Rust API] `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed upstream (2026-07-04).
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` — removed from the OpenAPI schema (2026-07-09).
- [Rust API] `ErrorResponse.service` — deprecated 2026-07-28, removed from the OpenAPI schema 2026-08-06; use `.code` instead.
- [Rust API] `get_multivariate_event_collection_lookup_history`, `lookup_tickers_for_market_in_multivariate_event_collection`, `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`, `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response` — the multivariate lookup REST surface was removed upstream (2026-08-06).
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsDataMessageV2`/`WsDataMessageRef::Multivariate`, `WsMultivariate`/`WsMultivariateRef` — the `multivariate` WebSocket channel (message type `multivariate_lookup`) was removed upstream (2026-08-06); use `MultivariateMarketLifecycle`.
- [Rust API] `GetQuotesParams.market_ticker`/`event_ticker` — `GET /communications/quotes` no longer supports these filters (2026-06-20).

### Breaking

- [Rust API] `get_balance`, `create_subaccount`, and `update_order_group_limit` all gained required parameters; call sites must be updated.
- [Rust API] Fields removed as listed under Removed above must be dropped from downstream code; exhaustive `match`/struct-literal construction over `WsChannelV2`, `WsMsgType`, `WsDataMessageV2`, `WsDataMessageRef`, and `WsUpdateAction` must account for the removed `Multivariate(Lookup)` variants and the new `PythValue`/`PythValueUnderlyingList`/`SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` variants.
- [Rust API] Every additive field listed above breaks exhaustive struct-literal construction of the affected type (most call sites already use `..Default::default()` and are unaffected).
- [Upstream] `SubaccountBalance` now appears once per exchange index a subaccount holds funds on, instead of one combined row per subaccount.


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
