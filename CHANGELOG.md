# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-14

### Compatibility

- Docs snapshot: 2026-09-14
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-17

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Date | Entry | Action |
|---|---|---|
| 06-11 | Fractional quantities for RFQs | No code change — already handled in 0.6.0 (`contracts_fp` present) |
| 06-11 | Tick size on `GET /margin/markets`; Perps mark prices / volume-OI notional fields on margin markets; Margin fee-tier endpoint returns active rates | No code change — margin market types not in crate |
| 06-11 | Self-serve Advanced API usage tier upgrade | Added `upgrade_account_api_usage_level()` |
| 06-11 | API usage volume progress endpoint | Added `get_account_api_usage_level_volume_progress()`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal` |
| 06-18 | Quote time filters (`min_ts`/`max_ts`) and pagination fix on `GET /communications/quotes` | Added `min_ts`/`max_ts` to `GetQuotesParams` |
| 06-18 | Sanity limits enforced on orderbook subscriptions | No code change — server-side rate/subscription limit, no schema change |
| 06-18 | Block-trade accept API key permissions | No code change — permission-scope only; block-trade-proposal endpoints not in crate |
| 06-18 | Event `tickers` filter on `GET /events` | Added `GetEventsParams.tickers` |
| 06-18 | Legacy `/portfolio/orders` mutation endpoints deprecated | No code change — legacy methods kept callable; V2 event-order endpoints already recommended in docs |
| 06-18 | Trade entries / RFQ quote identity on FIX market data | No code change — FIX protocol not in crate |
| 06-18 | Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` | Added top-level `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| 06-18 | `settlement_sources` added to the events API | Added `EventData.settlement_sources` |
| 06-19 | Communications RFQ/quote retention window reduced | No code change — operational retention window, no schema change |
| 06-20 | RFQ quote `market_ticker`/`event_ticker` filters removed | **Breaking** — removed from `GetQuotesParams` |
| 06-23 | Get Quote rate-limit cost reduced to 2 tokens | No code change — operational rate-limit only |
| 06-24/25 | RFQ quotes support post-only / FIX exchange index routing on FIX | No code change — FIX protocol not in crate |
| 06-25 | API usage tier qualification requirements halved | No code change — operational threshold only |
| 06-25 | RFQ quote retention and RFQ-scoped quote actions | Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` |
| 06-26/29/30, 07-02, 08-06/13/20/27, 09-03/10 | Margin-exchange-only entries (risk metrics, `margin_used`, trade-scoped key perms already tolerated, `is_portfolio` flag, exit triggers, order groups bind to shard, leverage estimates, asset_class, taker/maker-volume incentives, per-shard rate limits, fee tier rates) | No code change — margin market types not in crate |
| 07-02 | AcceptQuote / cancel-replace FIX reject reasons; FIX Tag 2446 | No code change — FIX protocol not in crate |
| 07-02 | Per-index subaccount balances | Added `SubaccountBalance.exchange_index` |
| 07-02 | Per-index exchange status | Added `GetExchangeStatusResponse.intra_exchange_transfers_active` / `exchange_index_statuses`, `ExchangeIndexStatus` |
| 07-02 | `price_ranges` added to `market_lifecycle_v2` events | Added `WsMarketLifecycleV2.price_ranges` |
| 07-02 | Sub-account-restricted API keys | Added `subaccount` to `CreateApiKeyRequest`/`GenerateApiKeyRequest`, `ApiKey.subaccount` |
| 07-02 | Multivariate lookup history endpoints fully deprecated | No further action — endpoint removed outright 2026-08-06, see below |
| 07-04 | `GET /exchange/announcements` removed | **Breaking** — `get_exchange_announcements()` and `Announcement`/`AnnouncementType`/`AnnouncementStatus`/`GetExchangeAnnouncementsResponse` removed |
| 07-09 | Deprecated Predictions REST schema fields removed | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| 07-09 | RFQ-scoped quote lookup endpoint | Covered by `get_rfq_quote()` above |
| 07-22 | Incentive programs on hidden events excluded from listing | No code change — server-side filtering behavior, no schema change |
| 07-23 | Seven new `price_level_structure` values | No code change — `Market.price_level_structure` already an open `Option<String>`, not a closed enum |
| 07-23 | `pyth_value` WebSocket channel | Not implemented — tracked as a known gap in `docs/spec-parity.md` |
| 07-23 | Subaccount-restricted API keys can open WebSocket sessions / quote on RFQ FIX sessions | No code change — permission-scoping only, no new fields |
| 07-23 | Historical positions endpoint | Added `get_historical_positions()`, `GetHistoricalPositionsParams`, `GetHistoricalPositionsResponse` |
| 07-23 | Order groups limited to 25,000 per user | No code change — operational limit only |
| 07-28, 08-06 | `service` field deprecated then removed from error responses | **Breaking** — removed `ErrorResponse.service` |
| 07-30 | Subaccount on `quote_created` | Added `WsQuoteCreated.subaccount` |
| 07-30 | Subaccount-restricted API keys can batch-order / read queue positions / manage order groups | No code change — permission-scoping only, no new fields |
| 07-30 | Event `product_metadata.cadence` | Added `EventMetadata.cadence` |
| 07-30 | New endpoint for event-keyed live data | Not implemented — tracked as a known gap |
| 07-30 | Series responses include `exchange_index` | Added `Series.exchange_index` |
| 07-30 | Lifecycle creation messages include `exchange_index` | Added `exchange_index` to `WsMarketLifecycleV2` and `WsEventLifecycle` |
| 07-30 | Richer combo-validation errors on multivariate market creation | No code change — `ErrorResponse.message`/`.details` already generic `Option<String>` |
| 08-06 | Multivariate event collections include `exchange_index` | Added `MultivariateEventCollection.exchange_index` |
| 08-06 | Order group limit updates support subaccounts | Added `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) to `update_order_group_limit()` |
| 08-06 | Multivariate lookup endpoint and WebSocket channel removed | **Breaking** — removed `lookup_tickers_for_market_in_multivariate_event_collection()`, `get_multivariate_event_collection_lookup_history()`, `WsChannelV2::Multivariate`, `WsMultivariate`/`WsMultivariateRef` |
| 08-13 | Intra-account transfer history endpoints | Added `intra_exchange_instance_transfer()`, `get_intra_exchange_instance_transfers()`, `get_intra_exchange_instance_transfer()` |
| 08-13 | Order group maximum increased to 100,000 | No code change — operational limit only |
| 08-13 | Exchange shard descriptions | Added `ExchangeIndexStatus.description` |
| 08-13 | Block trade indicator for WebSocket trades | Added `WsTrade.is_block_trade` |
| 08-13, 08-20 | Balance reads scoped by `exchange_index`; optional balance reads by `exchange_index` | Added `GetBalanceParams` (`subaccount`, `exchange_index`); `get_balance()` now takes params |
| 08-16 | API key location attestation expiry | Added `ApiKey.api_key_region_expiration_ts` |
| 08-20 | RFQs/combo-market creation for sub-account-restricted keys | No code change — permission-scoping only |
| 08-20 | Exchange index filters for portfolio lists | Added `exchange_index` filter to `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` |
| 08-20 | Exchange index on portfolio and WebSocket fill records | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition` (REST, required per spec) and `WsFill` (optional) |
| 08-20 | Resting order value breakdown by exchange index | Added `IndexedBalance`, `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| 08-20 | Target balance allocation endpoints | Added `get_target_balance_allocation()`, `set_target_balance_allocation()`, `TargetBalanceAllocation`, `RestingMarginReservation` |
| 08-20 | Cross-shard subaccount transfers | Covered by `source_subaccount`/`destination_subaccount` on `IntraExchangeInstanceTransferRequest` |
| 08-20 | Entry timestamps for FIX market data | No code change — FIX protocol not in crate |
| 08-20/22, 09-03 | Maker fee exemption for independent NFL combos; combo RFQ fee assignment for briefly-resting orders; post-only quotes preserved; tapered sub-cent pricing on combos | No code change — fee/pricing computed server-side; prices already read from `*_dollars` fields |
| 08-20, 08-24, 09-10 | Upcoming exchange sharding announcements (crypto/tennis/baseball, then commodities/basketball) | No code change — announcements only, no schema change |
| 08-20 | Kalshi Weather Index endpoint; weather index calibration history (08-31); `receipt_basis` on weather index points (09-10) | Not implemented — tracked as a known gap |
| 08-20 | VPC peering for Prime members | No code change — connectivity/infra only |
| 08-27 | Exchange auto-routing enabled by default | No code change — server routing behavior, no schema change |
| 08-27, 09-10 | `available_on_brokers` deprecated then removed from event responses | **Breaking** — removed `EventData.available_on_brokers` |
| 08-27 | Historical CF Benchmarks values via the REST passthrough | No code change — documents the existing `get_cfbenchmarks_*` REST passthrough, no new fields |
| 08-27 | Cancel-all-orders endpoints | Added `cancel_all_orders()` (Predictions `/portfolio/events/orders` only; margin variant out of scope) |
| 08-27 | Exchange index on user order messages | Added `WsUserOrder.exchange_index` |
| 08-27 | Localized market content (`Accept-Language`) | No code change — header passthrough via existing `with_default_headers`, no response schema change |
| 08-29 | Structured target images in Trade API v2 | No code change — `StructuredTarget.details` already an untyped `Map<String, Value>` |
| 09-03 | Shard rebalance margin reservation | Added `resting_margin_reservation` to `SetTargetBalanceAllocationRequest`/`GetTargetBalanceAllocationResponse` |
| 09-03 | Lower rate-limit cost for cancel-all; correct `remaining_count` after crossing amendments | No code change — operational rate-limit / server-side bug fix, no schema change |
| 09-03 | Filter historical positions by subaccount | Added `subaccount` to `GetHistoricalPositionsParams` |
| 09-03 | Filter FCM orders by client order IDs | Not implemented — tracked as a known gap |
| 09-03 | `cfbenchmarks_value_5hz` WebSocket channel | Not implemented — tracked as a known gap |
| 09-10 | WebSocket AsyncAPI schema corrections (documented `seq`/`order_source`/`sid`, retired error codes) | No code change — documentation-only corrections; Predictions message shapes unaffected |
| 09-10 | `center_deci_edge_centi_cent` price level structure emitted again | No code change — already an open `Option<String>` |
| 09-10 | Target-cost RFQs opt out of fee-inclusive sizing | Added `CreateRFQRequest.target_cost_excludes_fees` |
| 09-17 | RFQ/quote writes share shard-1 rate-limit budget | No code change — operational rate-limit only |
| 09-17 | WebSocket subscriptions ready when acknowledged | No code change — server-side ordering fix, no schema change |
| 09-17 | Returning to idiomatic MVE series | No code change — ticker/series naming only, no schema change |
| 09-17 | Series responses include a `categories` list | Added `Series.categories` |
| 09-17 | Target balance allocations include their reservation policy | Covered by `resting_margin_reservation` on `GetTargetBalanceAllocationResponse` above |
| 09-17 | More accurate WebSocket event processing metrics | No code change — internal metrics only, no schema change |
| (spec) | `FeeType::quadratic_with_combo_maker_fees` | Added `FeeType::QuadraticWithComboMakerFees` |

### Breaking

- [Rust API] Removed `ErrorResponse.service` (upstream removed the field from every error response 2026-08-06).
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` (upstream removed 2026-07-09; `price_level_structure` /
  `price_ranges` and fixed-point fields are the canonical replacements).
- [Rust API] Removed `EventData.available_on_brokers` (upstream stopped populating it 2026-08-27,
  removed the field 2026-09-10).
- [Rust API] Removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` (`GET /exchange/announcements` removed
  upstream 2026-07-04; use `get_exchange_schedule()`).
- [Rust API] Removed `lookup_tickers_for_market_in_multivariate_event_collection()`,
  `get_multivariate_event_collection_lookup_history()`, and their request/response types (the
  multivariate lookup REST surface predated RFQs and was removed upstream 2026-08-06).
- [Rust API] Removed `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `WsMsgType::MultivariateLookup`, and the `WsMultivariate` / `WsMultivariateRef` types (the
  `multivariate` WebSocket channel was removed upstream 2026-08-06; use
  `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes).
- [Rust API] Removed `market_ticker` and `event_ticker` from `GetQuotesParams` (upstream removed
  these filters from `GET /communications/quotes` 2026-06-20; filter by `rfq_id`, status, user, or
  update time instead).
- [Rust API] `get_balance()` now takes a `GetBalanceParams` argument (`subaccount`, `exchange_index`)
  instead of no arguments, to support the upstream per-exchange-index balance scoping added
  2026-07-02–08-13. Existing callers should pass `GetBalanceParams::default()` (or `Default::default()`)
  to preserve prior aggregate-balance behavior.
- [Rust API] `update_order_group_limit()` now takes an additional `UpdateOrderGroupLimitParams`
  argument (`subaccount`, `exchange_index`) to support the upstream subaccount-scoped limit update
  added 2026-08-06. Existing callers should pass `UpdateOrderGroupLimitParams::default()`.
- [Rust API] `Fill`, `Settlement`, and `MarketPosition` gained a required `exchange_index: u32`
  field, matching their upstream `required` status. Constructors and exhaustive matches must supply
  or handle the new field.

### Added

- [Rust API] Exchange-sharding support across the modeled surface: `exchange_index` on `Series`,
  `MultivariateEventCollection`, `Fill`, `Settlement`, `MarketPosition`, `SubaccountBalance`,
  `WsFill`, `WsUserOrder`, `WsMarketLifecycleV2`, `WsEventLifecycle`; `exchange_index` filters on
  `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams`/`GetBalanceParams`; per-index status via
  `GetExchangeStatusResponse.exchange_index_statuses` / `ExchangeIndexStatus`.
- [Rust API] New portfolio endpoints: `get_target_balance_allocation()`, `set_target_balance_allocation()`
  (with `RestingMarginReservation`), `get_historical_positions()`, `cancel_all_orders()`,
  `intra_exchange_instance_transfer()`, `get_intra_exchange_instance_transfers()`,
  `get_intra_exchange_instance_transfer()`, `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`.
  New account endpoints: `upgrade_account_api_usage_level()`, `get_account_api_usage_level_volume_progress()`.
  New RFQ-scoped quote endpoints: `get_rfq_quote()`, `delete_rfq_quote()`, `accept_rfq_quote()`,
  `confirm_rfq_quote()`.
- [Rust API] `Series.categories`, `EventData.settlement_sources`, `EventMetadata.cadence`,
  `GetEventsParams.tickers`, `ApiKey.api_key_region_expiration_ts` / `.subaccount`,
  `CreateApiKeyRequest.subaccount`, `GenerateApiKeyRequest.subaccount`,
  `CreateRFQRequest.target_cost_excludes_fees`, `FeeType::QuadraticWithComboMakerFees`.
- [Rust API] WebSocket: `WsMarketLifecycleV2.strike_type` / `.cap_strike` / `.custom_strike` /
  `.price_ranges`, `WsTrade.is_block_trade`, `WsQuoteCreated.subaccount`.
- [Docs] Documented known gaps (upstream weather-index live-data endpoints, the `pyth_value` and
  `cfbenchmarks_value_5hz` WebSocket channels, and the FCM `client_order_ids` filter) in
  `docs/spec-parity.md`.

### Removed

- See Breaking section above.


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
