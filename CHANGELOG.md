# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-18

### Compatibility

- Docs snapshot: 2026-09-18
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-18 (the live changelog also already listed a Sep 24, 2026
  entry — see table; treated as validated since it was visible in this run's fetch)

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| API usage volume progress endpoint (2026-06-11) | Added `get_account_api_usage_level_volume_progress()`, `GetAccountApiUsageLevelVolumeProgressResponse`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal` |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Added `upgrade_account_api_usage_level()` |
| Fractional quantities for RFQs (2026-06-11) | No code change — `contracts_fp` already present (see 0.6.0) |
| `settlement_sources` added to the events API (2026-06-18) | Added `EventData.settlement_sources: Vec<SettlementSource>` |
| Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` (2026-06-18) | Added top-level `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2`/`Ref` |
| Legacy order mutation endpoints deprecated (2026-06-18–25) | `#[deprecated]` on `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` (routes no longer in OpenAPI); migrated tests/examples to the V2 endpoints |
| Event `tickers` filter on `GET /events` (2026-06-18) | Added `GetEventsParams.tickers`; also added missing `min_updated_ts` found during required-field review |
| Block-trade accept API key permissions (2026-06-18) | No code change — scopes stored as `Vec<String>` already |
| Sanity limits on orderbook subscriptions (2026-06-18) | No code change — operational limits only |
| Quote `min_ts`/`max_ts` filters, pagination fix (2026-06-18) | Added `GetQuotesParams.min_ts`/`max_ts` (also added `user_filter`) |
| Communications retention window reduced (2026-06-19) | No code change — operational retention policy only |
| RFQ quote `market_ticker`/`event_ticker` filters removed (2026-06-20) | Removed `GetQuotesParams.market_ticker`/`event_ticker` |
| Get Quote rate-limit cost reduced (2026-06-23) | No code change — operational rate-limit only |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`; deprecated the quote-ID-only `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote` |
| API usage tier qualification halved (2026-06-25) | No code change — operational threshold only |
| Margin risk/positions changes (2026-06-26, 06-29, 07-02) | No code change — Margin exchange not modeled in crate |
| Trade-scoped API key permissions (2026-06-30) | No code change — scopes stored as `Vec<String>` already |
| Multivariate lookup history endpoints fully deprecated (2026-07-02), then removed (2026-08-06) | Removed `get_multivariate_event_collection_lookup_history`, `lookup_tickers_for_market_in_multivariate_event_collection` and their request/response types (routes no longer in OpenAPI); removed the `multivariate`/`multivariate_lookup` WS channel, message type, and `WsMultivariate`/`Ref` types (channel no longer in AsyncAPI's valid-channel enum) |
| `price_ranges` added to `market_lifecycle_v2` events (2026-07-02) | Added `WsMarketLifecycleV2.price_ranges: Option<Vec<PriceRange>>` |
| Per-index exchange status (2026-07-02) | Added `intra_exchange_transfers_active`, `exchange_index_statuses` (+ `ExchangeIndexStatus`) to `GetExchangeStatusResponse` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index` |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount`/`fcm_subtrader_id` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest`. Server-side authorization scoping of existing endpoints by a restricted key requires no crate change (endpoints/params already generic) |
| Exchange announcements endpoint removed (2026-07-04) | Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` (path no longer in OpenAPI) |
| Deprecated Predictions REST schema fields removed (2026-07-09) | Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (and WS mirrors) |
| Incentive programs on hidden events excluded (2026-07-22) | No code change — server-side filtering only |
| Order groups limited to 25,000 (2026-07-23), raised to 100,000 (2026-08-13) | No code change — operational limit only |
| Historical positions endpoint (2026-07-23) | Added `get_historical_positions()`, `GetHistoricalPositionsParams` (reuses `GetPositionsResponse`) |
| Subaccount-restricted keys: WS sessions, queue positions, batch orders, order groups, combo/RFQ creation (2026-07-23–08-20) | No code change — server-side authorization scoping only; no new message/response shapes |
| Pyth value WebSocket channel (2026-07-23) | **Deferred** — new channel, not modeled; see `docs/spec-parity.md` |
| New price level structures incl. `center_deci_edge_centi_cent` (2026-07-23, 08-13, 09-03) | No code change — `price_level_structure` is an untyped `String` |
| `service` field on error responses deprecated (2026-07-28), removed (2026-08-06) | Removed `ErrorResponse.service` (and the `retry.rs`/test references) |
| Richer combo-validation errors (2026-07-30, FIX 08-13) | No code change — `ErrorResponse.message`/`details` already generic `Option<String>` |
| Lifecycle messages include `exchange_index` (2026-07-30) | Added `exchange_index` to `WsMarketLifecycleV2`/`Ref` and `WsEventLifecycle`/`Ref` |
| Series responses include `exchange_index` (2026-07-30) | Added `Series.exchange_index` |
| Event-keyed live data endpoint (2026-07-30) | **Deferred** — new endpoint, not modeled; see `docs/spec-parity.md` |
| Event `product_metadata.cadence` (2026-07-30) | Added `EventMetadata.cadence` |
| `subaccount` on `quote_created` (2026-07-30) | Added `subaccount` to `WsQuoteCreated`/`Ref`; also added missing `rfq_creator_id` (required in spec) and `subaccount` to `WsQuoteAccepted`/`Ref` and `WsQuoteExecuted`/`Ref` found during required-field review |
| Multivariate event collections include `exchange_index` (2026-08-06) | Added `MultivariateEventCollection.exchange_index` |
| New `center_deci_edge_centi_cent` structure, exchange shard descriptions (2026-08-13) | Added `ExchangeIndexStatus.description`; price structure untyped (no change) |
| Balance/exchange-index scoping (`GET /portfolio/balance`, 2026-08-13/08-20) | **Deferred** — `get_balance()` takes no params; adding `exchange_index`/`subaccount` scoping is a signature change tracked as follow-up; see `docs/spec-parity.md` |
| Block trade indicator for WS trades (2026-08-13) | Added `WsTrade.is_block_trade` |
| Intra-account transfer history endpoints (2026-08-13) | **Deferred** — new endpoints, not modeled; see `docs/spec-parity.md` |
| API key location attestation expiry (2026-08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| VPC peering, entry timestamps, maker-fee/rate-limit/rollout notices (2026-08-20–09-17, various) | No code change — infra/operational/fee notices with no schema impact |
| Kalshi Weather Index endpoint, calibration history (2026-08-20, 08-31, receipt_basis 09-10) | **Deferred** — new `live_data/weather/*` endpoints, not modeled; see `docs/spec-parity.md` |
| Target balance allocation endpoints (2026-08-20, reservation policy 09-17) | **Deferred** — new endpoints, not modeled; see `docs/spec-parity.md` |
| Resting order value breakdown by exchange index (2026-08-20) | **Deferred** — `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` not added; see `docs/spec-parity.md` |
| Exchange index on fill/settlement/position responses and WS fill (2026-08-20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition` (REST) and `WsFill`/`Ref` |
| Exchange index filters for portfolio lists (2026-08-20) | Added `exchange_index` filter to `GetPositionsParams`, `GetFillsParams`, `GetOrdersParams` |
| Exit triggers on margin positions (2026-08-20) | No code change — Margin exchange not modeled |
| Localized market content via `Accept-Language` (2026-08-27) | No code change — header-based, no response shape change |
| Exchange index on user order messages (2026-08-27) | Added `WsUserOrder.exchange_index` |
| Cancel-all-orders endpoints (2026-08-27, rate-limit 09-03) | **Deferred** — new endpoints, not modeled; see `docs/spec-parity.md` |
| CF Benchmarks REST passthrough docs, structured target `image_url` (2026-08-27, 08-29) | No code change — passthrough is already untyped/no crate surface; structured target `details` already a generic `serde_json::Value` |
| `available_on_brokers` deprecated (2026-08-27), removed (2026-09-10) | Removed `EventData.available_on_brokers` |
| Exchange auto-routing enabled by default (2026-08-27) | No code change — server routing behavior only |
| CF Benchmarks 5Hz value WebSocket channel (2026-09-03) | **Deferred** — new channel, not modeled; see `docs/spec-parity.md` |
| Filter FCM orders by client order IDs (2026-09-03) | Added `GetFcmOrdersParams.client_order_ids`; made `subtrader_id` optional to match relaxed OpenAPI requirement |
| Filter historical positions by subaccount (2026-09-03) | Covered by `GetHistoricalPositionsParams.subaccount` above |
| Correct remaining counts after crossing amendments, lower cancel-all cost, shard rebalance (2026-09-03) | No code change — behavior/operational fixes only |
| Tapered sub-cent pricing on combo markets (2026-09-03) | No code change — untyped `price_level_structure`/dollar-string fields already support sub-cent precision |
| `available_on_brokers` breaking removal confirmed (2026-09-10) | Covered above |
| Principal-only sizing for target-cost RFQs (2026-09-10) | Added `target_cost_excludes_fees` to `CreateRFQRequest`, `RFQ`, `Quote` |
| Upcoming exchange sharding (2026-08-24, 09-10) | No code change — operational routing announcement |
| `center_deci_edge_centi_cent` emitted again (2026-09-10) | No code change — untyped string; `price_ranges` already modeled |
| WebSocket schema corrections: `seq`/`sid` on more channels, error codes 6/16/17 retired, `market_id`/`market_ticker` never on error schema (2026-09-10) | No code change — `seq`/`sid` already carried on `Unknown`/control frames (0.7.0); `WsError`/`WsErrorRef` never had `market_id`/`market_ticker`; error codes are plain `i64`, not an enum |
| Margin-only WS/FIX corrections and features (throughout) | No code change — Margin exchange not modeled |
| `PUT .../order_groups/{id}/limit` supports `subaccount` (2026-08-06) | `update_order_group_limit()` now takes a `SubaccountQueryParams` (breaking signature change) |
| Historical fills/orders support `min_ts` (2026-09-17) | Added `min_ts` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| Series responses include `categories` (2026-09-17) | Added `Series.categories: Vec<String>` (required in spec) |
| Returning to idiomatic MVE series naming (2026-09-17) | No code change — ticker/series naming convention only |
| WS subscriptions ready when acknowledged (race fix, 2026-09-17) | No code change — server-side ordering fix |
| RFQ/quote and order-group rate-limit budget changes (throughout) | No code change — operational rate-limit only |
| Orders historical cutoff advances independently (2026-09-24) | No code change — `orders_updated_ts` already a plain `String`; semantics-only |
| All FIX-only entries (throughout) | No code change — FIX API not modeled in crate (REST/WS only, per `CLAUDE.md`) |

### Breaking

- [Rust API] Removed fields no longer present in the live OpenAPI/AsyncAPI specs: `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (+ WS mirrors), `EventData.available_on_brokers`, `ErrorResponse.service`.
- [Rust API] Removed the fully-retired multivariate lookup surface: `get_multivariate_event_collection_lookup_history`, `lookup_tickers_for_market_in_multivariate_event_collection`, `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`, `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response`; the `multivariate`/`multivariate_lookup` WS channel and message type (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate`, `WsMultivariate`, `WsMultivariateRef`, and their `WsWireMessage`/`WsDataMessageV2` variants).
- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` (endpoint retired).
- [Rust API] `KalshiRestClient::update_order_group_limit` now takes an additional `SubaccountQueryParams` argument.
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from `String` to `Option<String>` (now optional when `client_order_ids` is supplied).
- [Rust API] `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` are `#[deprecated]`; the underlying `/portfolio/orders` mutation endpoints are being retired by Kalshi in favor of the V2 event-order endpoints (`create_order_v2` etc.), already present since 0.6.0.

### Added

- [Rust API] RFQ-scoped quote action endpoints: `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`.
- [Rust API] `get_historical_positions` / `GetHistoricalPositionsParams` for `GET /historical/positions`.
- [Rust API] `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level`.
- [Rust API] `exchange_index` plumbed through `EventData`, `Series`, `MultivariateEventCollection`, `MarketPosition`, `Fill`, `Settlement`, `SubaccountBalance`, `GetExchangeStatusResponse` (+ `ExchangeIndexStatus`), `WsMarketLifecycleV2`, `WsEventLifecycle`, `WsFill`, `WsUserOrder`, and as a filter on `GetPositionsParams`/`GetFillsParams`/`GetOrdersParams`.
- [Rust API] `target_cost_excludes_fees` on `CreateRFQRequest`, `RFQ`, `Quote`; `post_only` on `CreateQuoteRequest`/`Quote`.
- [Rust API] `min_ts`/`max_ts`/`user_filter` on `GetQuotesParams`; `min_ts` on `GetHistoricalFillsParams`/`GetHistoricalOrdersParams`; `tickers`/`min_updated_ts` on `GetEventsParams`; `client_order_ids` on `GetFcmOrdersParams`.
- [Rust API] `EventData.settlement_sources`, `EventMetadata.cadence`, `Series.categories`, `WsMarketLifecycleV2.price_ranges`/`strike_type`/`cap_strike`/`custom_strike`, `WsTrade.is_block_trade`, `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted.subaccount` (+ missing `rfq_creator_id`), `ApiKey.subaccount`/`fcm_subtrader_id`, `GetApiKeysResponse.api_key_region_expiration_ts`.

### Deferred

The following upstream changes are new endpoints/channels not yet modeled. Each is additive (not
a removal), so leaving them unimplemented does not desync existing crate surface from the spec;
tracked as follow-up work. See `docs/spec-parity.md` for details.

- `pyth_value` and `cfbenchmarks_value_5hz` WebSocket channels.
- `GET /live_data/events/{event_ticker}`, `GET /live_data/weather/{city}`, `GET /live_data/weather/{city}/calibrations`.
- Target balance allocation endpoints (`/portfolio/target_balance_allocation`).
- Intra-exchange-instance transfer endpoints (`/portfolio/intra_exchange_instance_transfer*`).
- Cancel-all-orders endpoints (`/portfolio/*/cancel_all` family).
- `exchange_index`/`subaccount` scoping on `GET /portfolio/balance` (endpoint currently takes no params).
- `resting_order_value_breakdown` on `GetPortfolioRestingOrderTotalValueResponse`.


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
