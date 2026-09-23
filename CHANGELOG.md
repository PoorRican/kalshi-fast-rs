# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-24

### Compatibility

- Docs snapshot: 2026-09-23
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-24

**Changelog entries since the 0.7.0 watermark (2026-06-08) and disposition.** FIX-only entries are
out of scope (this crate covers REST + WebSocket only) and Margin-exchange-only entries are out of
scope per existing precedent (margin market types are not modeled). Both are marked "No change —
FIX/Margin" below without further comment.

| Date | Entry | Action |
|---|---|---|
| 2026-06-11 | API usage volume progress endpoint | Added `get_account_api_usage_level_volume_progress()`, `GetAccountApiUsageLevelVolumeProgressResponse`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal` |
| 2026-06-11 | Perps mark prices on margin markets | No change — Margin |
| 2026-06-11 | Self-serve Advanced API usage tier upgrade | Added `upgrade_account_api_usage_level()` |
| 2026-06-11 | Margin fee-tier endpoint returns active rates | No change — Margin |
| 2026-06-11 | Perps volume and open interest notional fields | No change — Margin |
| 2026-06-11 | Tick size added to GET Margin Markets | No change — Margin |
| 2026-06-11 | Fractional quantities for RFQs | No change — `contracts_fp` already present on `CreateRFQRequest`/`Quote` |
| 2026-06-18 | settlement_sources added to the events API | Added `EventData.settlement_sources: Vec<SettlementSource>` |
| 2026-06-18 | Strike type and cap strike on market_lifecycle_v2 metadata_updated | Added top-level `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2`(`Ref`) |
| 2026-06-18 | Subaccount on margin positions | No change — Margin |
| 2026-06-18 | RFQ quote identity on FIX | No change — FIX |
| 2026-06-18 | Trade entries in FIX market data | No change — FIX |
| 2026-06-18 | Legacy order mutation endpoints deprecated | **Breaking** — removed (now fully absent from OpenAPI): `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` and their request/response types. Use the V2 event-order endpoints |
| 2026-06-18 | Event tickers filter on GET /trade-api/v2/events | Added `GetEventsParams.tickers` (CSV); also added `min_updated_ts` (present in the same spec section) |
| 2026-06-18 | Block-trade accept API key permissions | No change — scopes already modeled as `Vec<String>` |
| 2026-06-18 | Sanity limits enforced on orderbook subscriptions | No change — operational limit, not a shape change |
| 2026-06-18 | Quote time filters and pagination fix | Added `GetQuotesParams.min_ts` / `max_ts`; pagination fix is server-side |
| 2026-06-19 | Communications RFQ and quote retention window reduced | No change — operational retention policy |
| 2026-06-20 | RFQ quote market and event filters removed | **Breaking** — removed `GetQuotesParams.market_ticker` / `event_ticker` (endpoint no longer accepts them) |
| 2026-06-23 | Get Quote rate-limit cost reduced to 2 tokens | No change — operational |
| 2026-06-24 | RFQ quotes support post-only on FIX | No change — FIX |
| 2026-06-25 | RFQ quote retention and RFQ-scoped quote actions | Deferred — new `rfq_id`-scoped quote action/lookup endpoints not added this pass; existing quote-ID-only endpoints remain supported per spec |
| 2026-06-25 | API usage tier qualification requirements halved | No change — operational |
| 2026-06-25 | FIX exchange index routing | No change — FIX |
| 2026-06-26 | Margin risk per-market metrics limited | No change — Margin |
| 2026-06-29 | Margin positions margin_used omitted for jointly-margined positions | No change — Margin |
| 2026-06-30 | Trade-scoped API key permissions | No change — scopes already modeled as `Vec<String>` |
| 2026-07-02 | Multivariate lookup history endpoints are fully deprecated | Superseded by the 2026-08-06 removal below |
| 2026-07-02 | Margin positions now include an is_portfolio flag | No change — Margin |
| 2026-07-02 | price_ranges added to market_lifecycle_v2 events | Added `WsMarketLifecycleV2.price_ranges: Option<Vec<PriceRange>>` |
| 2026-07-02 | Per-index exchange status | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`, `.exchange_index_statuses`, new `ExchangeIndexStatus` |
| 2026-07-02 | Per-index subaccount balances | Added `SubaccountBalance.exchange_index` |
| 2026-07-02 | AcceptQuote rejects carry a specific reason on FIX | No change — FIX |
| 2026-07-02 | More specific FIX rejects for cancel/replace failures | No change — FIX |
| 2026-07-02 | Sub-account-restricted API keys | Added `subaccount` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| 2026-07-04 | Exchange announcements endpoint removed | **Breaking** — removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| 2026-07-09 | Support for FIX Tag 2446 on Incremental Refresh | No change — FIX |
| 2026-07-09 | RFQ-scoped quote lookup endpoint | Deferred — see 2026-06-25 |
| 2026-07-09 | Deprecated Predictions REST schema fields removed | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (confirmed absent from current OpenAPI) |
| 2026-07-09 | Margin orders now identify system order reasons | No change — Margin |
| 2026-07-22 | Incentive programs on hidden events excluded from listing | No change — server-side filtering behavior |
| 2026-07-23 | Order groups limited to 25,000 per user | No change — operational (superseded by 2026-08-13 increase to 100,000) |
| 2026-07-23 | Historical positions endpoint | Deferred — `GET /historical/positions` not added this pass |
| 2026-07-23 | Subaccount-restricted API keys can open WebSocket sessions | No change — access-control behavior only |
| 2026-07-23 | Subaccount-restricted API keys can quote on RFQ FIX sessions | No change — FIX |
| 2026-07-23 | Pyth value WebSocket channel | Deferred — `pyth_value` channel not added this pass |
| 2026-07-23 | New price level structures | No change — `price_level_structure` already modeled as a raw string/`Option<String>`, not a closed enum |
| 2026-07-28 | The service field on error responses is deprecated | Superseded by the 2026-08-06 removal below |
| 2026-07-30 | Richer combo-validation errors on multivariate market creation | No change — error bodies already generic (`message`/`details` as `Option<String>`) |
| 2026-07-30 | Lifecycle creation messages now include exchange_index | Added `WsMarketLifecycleV2.exchange_index` (covers `market_lifecycle_v2` and, via the shared struct, `multivariate_market_lifecycle`) and `WsEventLifecycle.exchange_index` |
| 2026-07-30 | Series responses include exchange_index | Added `Series.exchange_index` |
| 2026-07-30 | New endpoint for event-keyed live data | Deferred — `GET /live_data/events/{event_ticker}` not added this pass |
| 2026-07-30 | Subaccount-restricted API keys can read order queue positions | No change — access-control behavior only |
| 2026-07-30 | Event product_metadata now includes cadence | No change — not a formal OpenAPI schema field on `EventMetadata`; captured by the existing `extra` flatten catch-all |
| 2026-07-30 | Subaccount-restricted API keys can use batch order endpoints | No change — V2 batch types already carry `subaccount` per order/cancel entry |
| 2026-07-30 | Subaccount on quote_created | Added `WsQuoteCreated.subaccount` |
| 2026-07-30 | Subaccount-restricted API keys can manage order groups | No change — access-control behavior only |
| 2026-08-06 | Multivariate lookup endpoint and channel removed | **Breaking** — removed REST `lookup_tickers_for_market_in_multivariate_event_collection` (PUT) and `get_multivariate_event_collection_lookup_history` (GET) plus their types; removed WS `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate(Ref)`, `WsMultivariateSelectedMarket(Ref)` (confirmed absent from current specs) |
| 2026-08-06 | FIX execution reports identify the source exchange index | No change — FIX |
| 2026-08-06 | Sided leverage estimates on margin markets | No change — Margin |
| 2026-08-06 | Order group limit updates support subaccounts | **Breaking** — `update_order_group_limit` now takes a `SubaccountQueryParams` argument |
| 2026-08-06 | Multivariate event collections include exchange_index | Added `MultivariateEventCollection.exchange_index` |
| 2026-08-06 | The service field has been removed from error responses | **Breaking** — removed `ErrorResponse.service` (confirmed absent from current OpenAPI); `rest/retry.rs` no longer checks it |
| 2026-08-13 | New center_deci_edge_centi_cent price level structure | No change — raw string, see 2026-07-23 |
| 2026-08-13 | Balance reads scoped by exchange_index | Deferred — `GET /portfolio/balance` `exchange_index` scoping not added this pass |
| 2026-08-13 | Block trade indicator for WebSocket trades | Added `WsTrade.is_block_trade: bool` (`#[serde(default)]`) |
| 2026-08-13 | Exchange shard descriptions | Added `ExchangeIndexStatus.description` (bundled with the 2026-07-02 entry above) |
| 2026-08-13 | Margin order groups bind to single exchange_index | No change — Margin |
| 2026-08-13 | Order group maximum increased to 100,000 per user | No change — operational |
| 2026-08-13 | Richer combo-validation errors on FIX RFQ creation | No change — FIX |
| 2026-08-13 | Intra-account transfer history endpoints | Deferred — `/portfolio/intra_exchange_instance_transfers*` not added this pass |
| 2026-08-16 | API key location attestation expiry | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| 2026-08-20 | VPC peering for Prime members | No change — connectivity/infra, not an API shape |
| 2026-08-20 | Kalshi Weather Index endpoint | Deferred — `GET /live_data/weather/{city}` not added this pass |
| 2026-08-20 | Maker fee exemption for independent NFL combo markets | No change — fee/policy behavior only |
| 2026-08-20 | Entry timestamps for FIX market data | No change — FIX |
| 2026-08-20 | Cross-shard subaccount transfers | Deferred — `POST /portfolio/intra_exchange_instance_transfer` not added this pass |
| 2026-08-20 | Target balance allocation endpoints | Deferred — not added this pass |
| 2026-08-20 | Resting order value breakdown by exchange index | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Vec<IndexedBalance>`, new `IndexedBalance` |
| 2026-08-20 | Exchange index on portfolio and WebSocket fill records | Added `exchange_index` to REST `Fill`, `Settlement`, `MarketPosition`, and `WsFill` |
| 2026-08-20 | Exchange index filters for portfolio lists | Added `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| 2026-08-20 | RFQs and combo-market creation for sub-account-restricted API keys | No change — access-control behavior only |
| 2026-08-20 | Optional balance reads by exchange_index | Deferred — see 2026-08-13 balance-scoping entry |
| 2026-08-20 | Exit triggers on margin positions | No change — Margin |
| 2026-08-22 | Post-only quotes preserved; crossing rate limits may apply | No change — policy/rate-limit behavior only |
| 2026-08-22 | Combo RFQ fee assignment for briefly resting orders | No change — fee/policy behavior only |
| 2026-08-24 | Upcoming exchange sharding | No change — informational, no new fields beyond `exchange_index` already added |
| 2026-08-27 | Localized market content in REST responses | Deferred — no per-request `Accept-Language` header support added this pass; no response shape changes |
| 2026-08-27 | Trade type on FIX market data | No change — FIX |
| 2026-08-27 | Exchange index on user order messages | Added `WsUserOrder.exchange_index` |
| 2026-08-27 | Cancel-all-orders endpoints | Deferred — not added this pass |
| 2026-08-27 | Historical CF Benchmarks values via the REST passthrough | No change — documents the existing `GET /cfbenchmarks/*` endpoint |
| 2026-08-27 | The available_on_brokers field on event responses is deprecated | Superseded by the 2026-09-10 removal below |
| 2026-08-27 | Exchange auto-routing enabled by default | No change — server-side routing behavior only |
| 2026-08-27 | Margin maker-volume incentive programs | No change — Margin |
| 2026-08-29 | Structured target images in Trade API v2 | No change — target responses already flow through generic/catch-all fields |
| 2026-08-31 | Weather index calibration history | Deferred — depends on the deferred weather-index endpoint (2026-08-20) |
| 2026-09-03 | CF Benchmarks 5Hz value websocket channel | Deferred — `cfbenchmarks_value_5hz` channel not added this pass |
| 2026-09-03 | Higher FIX market data session limit | No change — FIX |
| 2026-09-03 | Order identity on FIX market data | No change — FIX |
| 2026-09-03 | Margin fee tier rates | No change — Margin |
| 2026-09-03 | Filter FCM orders by client order IDs | Added `GetFcmOrdersParams.client_order_ids`; changed `subtrader_id` to `Option<String>` (now conditionally required alongside `client_order_ids`) — **breaking** |
| 2026-09-03 | Filter historical positions by subaccount | No change — depends on the deferred historical-positions endpoint (2026-07-23) |
| 2026-09-03 | Correct remaining counts after crossing order amendments | No change — server-side bug fix, not a shape change |
| 2026-09-03 | Lower rate-limit cost for cancel all orders | No change — depends on the deferred cancel-all endpoints (2026-08-27) |
| 2026-09-03 | Shard rebalance margin reservation | No change — depends on the deferred target-balance-allocation endpoints (2026-08-20) |
| 2026-09-03 | ClearingBusinessDate on FIX trade execution reports | No change — FIX |
| 2026-09-03 | Tapered sub-cent pricing on multivariate (combo) markets | No change — dollar fields already used; no new fields |
| 2026-09-10 | Per-shard margin order rate limits | No change — Margin |
| 2026-09-10 | The deprecated available_on_brokers field is removed from event responses | **Breaking** — removed `EventData.available_on_brokers` (confirmed absent from current OpenAPI) |
| 2026-09-10 | Principal-only sizing for target-cost RFQs | Added `target_cost_excludes_fees: Option<bool>` to `CreateRFQRequest`, `RFQ`, `Quote` |
| 2026-09-10 | Margin taker-volume incentive programs | No change — Margin |
| 2026-09-10 | Weather index points expose receipt_basis | No change — depends on the deferred weather-index endpoint (2026-08-20) |
| 2026-09-10 | Margin markets expose asset_class | No change — Margin |
| 2026-09-10 | Upcoming exchange sharding for commodities and basketball | No change — informational |
| 2026-09-10 | The center_deci_edge_centi_cent price level structure is emitted again | No change — bug fix (empty-string serialization); raw-string modeling already tolerant |
| 2026-09-10 | WebSocket schemas corrected to match the messages the service sends | No change — doc-only corrections describing existing behavior (retired error codes 6/16/17, `sid`/`seq` documented on already-sequenced channels); the crate does not hardcode specific error codes |
| 2026-09-17 | Margin market important information | No change — Margin |
| 2026-09-17 | Margin market responses return the configured tick size | No change — Margin |
| 2026-09-17 | WebSocket schema corrections (Predictions/Margin nullable fields, enum values) | No change — `WsTicker.dollar_volume` / `dollar_open_interest` are already signed `i64` |
| 2026-09-17 | FIX EventResendRequest (35=U1) Gated | No change — FIX |
| 2026-09-17 | Historical fills and orders support min_ts | Added `min_ts` to `GetHistoricalFillsParams`, `GetHistoricalOrdersParams` |
| 2026-09-17 | Reduced rate limit cost for QuoteConfirm when providing the RFQ ID | No change — operational |
| 2026-09-17 | Target balance allocations include their reservation policy | No change — depends on the deferred target-balance-allocation endpoints (2026-08-20) |
| 2026-09-17 | Series responses include a categories list | Added `Series.categories: Vec<String>` |
| 2026-09-17 | Returning to idiomatic MVE series | No change — ticker-naming convention only |
| 2026-09-17 | WebSocket subscriptions are ready when acknowledged | No change — server-side race-condition fix |
| 2026-09-17 | RFQ and quote writes share the shard 1 rate-limit budget | No change — operational |
| 2026-09-24 | RFQ creation timestamps over FIX | No change — FIX |
| 2026-09-24 | 20% higher read and write rate limits | No change — operational |
| 2026-09-24 | Optional WebSocket compression | Deferred — `permessage-deflate` negotiation not implemented this pass |
| 2026-09-24 | Rebalancing without resting-order reservation | No change — depends on the deferred target-balance-allocation endpoints (2026-08-20) |
| 2026-09-24 | Subaccount-scoped historical fills and orders | Added `subaccount` to `GetHistoricalFillsParams`, `GetHistoricalOrdersParams` |
| 2026-09-24 | Orders historical cutoff advances independently | No change — `GetHistoricalCutoffResponse` fields already present; semantics-only change |

### Added

- [Rust API] `get_account_api_usage_level_volume_progress()` and `upgrade_account_api_usage_level()`
  account endpoints, with `GetAccountApiUsageLevelVolumeProgressResponse`,
  `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal`.
- [Rust API] `exchange_index: Option<i64>` added across the exchange-sharding rollout:
  `EventData`, `Series`, `SubaccountBalance`, `MultivariateEventCollection`, REST `Fill` /
  `Settlement` / `MarketPosition`, and WS `WsMarketLifecycleV2` (created events),
  `WsEventLifecycle`, `WsFill`, `WsUserOrder`. New `ExchangeIndexStatus` and
  `GetExchangeStatusResponse.exchange_index_statuses` / `.intra_exchange_transfers_active`.
  `exchange_index` filter added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] `EventData.settlement_sources: Vec<SettlementSource>`; `GetEventsParams.tickers` and
  `.min_updated_ts` filters.
- [Rust API] `Series.categories: Vec<String>`.
- [Rust API] `WsMarketLifecycleV2` top-level `strike_type`, `cap_strike`, `custom_strike` (on
  `metadata_updated`) and `price_ranges: Option<Vec<PriceRange>>` (on `created` /
  `price_level_structure_updated`).
- [Rust API] `WsTrade.is_block_trade: bool`; `WsQuoteCreated.subaccount: Option<u32>`.
- [Rust API] `ApiKey.subaccount`, `CreateApiKeyRequest.subaccount`,
  `GenerateApiKeyRequest.subaccount`; `GetApiKeysResponse.api_key_region_expiration_ts`.
- [Rust API] `GetQuotesParams.min_ts` / `.max_ts` / `.user_filter`.
- [Rust API] `CreateRFQRequest` / `RFQ` / `Quote`: `target_cost_excludes_fees: Option<bool>`.
- [Rust API] `GetFcmOrdersParams.client_order_ids: Option<Vec<String>>`.
- [Rust API] `GetHistoricalFillsParams` / `GetHistoricalOrdersParams`: `min_ts`, `subaccount`.
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown:
  Vec<IndexedBalance>`; new `IndexedBalance` type.
- [Tests] Unit coverage for all of the above (`tests/parsing.rs`, and inline tests in
  `src/ws/types/messages/{lifecycle,trade,fill}.rs`).

### Changed

- [Rust API] `update_order_group_limit` now takes a `SubaccountQueryParams` argument (the
  endpoint's `subaccount` query parameter is required by the current API surface for correct
  routing, even though it's optional/defaulted server-side).
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from required `String` to
  `Option<String>` — the endpoint now accepts either `subtrader_id` or `client_order_ids`.

### Deprecated

None this release — every field the upstream changelog marked deprecated in this window was
either already `Option` (no crate action needed) or has since been fully removed upstream and is
handled under Removed below.

### Removed

- [Rust API] **Breaking.** Legacy order-mutation endpoints and types: `create_order`,
  `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`,
  `CreateOrderRequest`, `CreateOrderResponse`, `CancelOrderParams`, `CancelOrderResponse`,
  `AmendOrderRequest`, `AmendOrderResponse`, `DecreaseOrderRequest`, `DecreaseOrderResponse`,
  `BatchCreateOrdersRequest`, `BatchCreateOrdersResponse`, `BatchCreateOrdersIndividualResponse`,
  `BatchCancelOrdersRequestOrder`, `BatchCancelOrdersRequest`, `BatchCancelOrdersResponse`,
  `BatchCancelOrdersIndividualResponse`. Use the V2 event-order endpoints instead.
- [Rust API] **Breaking.** `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `MultivariateLookup`, `WsMultivariate`, `WsMultivariateRef`, `WsMultivariateSelectedMarket`,
  `WsMultivariateSelectedMarketRef`; REST `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response`,
  `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`.
- [Rust API] **Breaking.** `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus`.
- [Rust API] **Breaking.** `ErrorResponse.service`.
- [Rust API] **Breaking.** `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`.
- [Rust API] **Breaking.** `EventData.available_on_brokers`.
- [Rust API] **Breaking.** `GetQuotesParams.market_ticker`, `GetQuotesParams.event_ticker`.
- [Rust API] Dead code: unused `MarketPositionRef` / `EventPositionRef` stubs in
  `ws/types/mod.rs` that did not correspond to any real WebSocket payload (the actual
  `market_position` message is modeled correctly by `WsMarketPosition` /
  `WsMarketPositionRef` in `ws/types/messages/positions.rs`).

### Fixed

- [Rust API] Pre-existing compile bugs unrelated to this refresh, found while verifying the crate
  builds/tests clean: an `envelope.rs` unit test matched `ListSubscriptions` without its `sid`/`seq`
  fields (added in 0.7.0); `tests/rest_auth.rs` still referenced the pre-0.6.0
  `GetAccountApiLimitsResponse.read_limit` / `.write_limit` flat fields.

### Breaking

- [Rust API] See the Removed and Changed sections above. Net effect: any downstream code using the
  legacy (non-V2) order-mutation methods/types, the `multivariate` WS channel or multivariate
  lookup REST/WS surface, `ErrorResponse.service`, the three removed `Market`/`MarketPosition`
  fields, `EventData.available_on_brokers`, `GetQuotesParams.market_ticker`/`.event_ticker`,
  `get_exchange_announcements`, or `update_order_group_limit`'s old 2-argument signature must be
  updated. Per [`VERSIONING.md`](VERSIONING.md), any breaking Rust API change pre-1.0 is a minor
  bump: 0.7.0 → 0.8.0.



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
