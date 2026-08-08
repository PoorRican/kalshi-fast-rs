# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.7.0] - 2026-08-08

### Compatibility

- Docs snapshot: 2026-08-08
- OpenAPI: 3.27.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-13

**Changelog entries since 0.6.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| Block trade indicator for WebSocket trades (2026-08-13) | Added `is_block_trade: bool` to `WsTrade` / `WsTradeRef` |
| Exchange shard descriptions (2026-08-13) | Added `description: String` to new `ExchangeIndexStatus` |
| Margin order groups bind to single `exchange_index` (2026-08-13) | No code change — margin order groups not modeled |
| Order group maximum increased to 100,000 per user (2026-08-13) | No code change — quota/rate-limit change only |
| Multivariate lookup endpoint and channel removed (2026-08-06) | **Breaking** — removed the PUT/GET `.../lookup` REST endpoints and the WS `multivariate` channel (`multivariate_lookup` message) |
| Centicent pricing on multivariate (combo) markets (2026-08-17, scheduled) | No code change — `price_level_structure` is an untyped `String`; `price_ranges`/`step` already carry the tick data |
| Richer combo-validation errors on FIX RFQ creation (2026-08-13) | No code change — FIX not supported by this crate |
| Intra-account transfer history endpoints (2026-08-13) | Added `get_intra_exchange_instance_transfers`(`_pager`/`_all`)/`get_intra_exchange_instance_transfer` + `IntraExchangeInstanceTransfer` types |
| FIX execution reports identify the source exchange index (2026-08-06) | No code change — FIX not supported |
| Sided leverage estimates on margin markets (2026-08-06) | No code change — margin market types not modeled |
| Order group limit updates support subaccounts (2026-08-06) | **Breaking** — `update_order_group_limit` gained a `params: SubaccountQueryParams` argument |
| Multivariate event collections include `exchange_index` (2026-08-06) | Added `exchange_index: Option<u32>` to `MultivariateEventCollection` |
| Richer combo-validation errors on multivariate market creation (2026-07-30) | No code change — `message`/`details` already modeled on `ErrorResponse` |
| The `service` field has been removed from error responses (2026-08-06) | Documented — `ErrorResponse.service` kept as `Option<String>` (explicit exception), now always `None` |
| The `service` field on error responses is deprecated (2026-07-28) | No code change — precursor to the removal above |
| Lifecycle creation messages now include `exchange_index` (2026-07-30) | Added `exchange_index: Option<i64>` to `WsMarketLifecycleV2`/`Ref` and `WsEventLifecycle`/`Ref` |
| Series responses include `exchange_index` (2026-07-30) | Added `exchange_index: Option<u32>` to `Series` |
| New endpoint for event-keyed live data (2026-07-30) | Added `get_event_live_data` + `EventLiveData`, `GetEventLiveDataParams`, `GetEventLiveDataResponse` |
| Subaccount-restricted API keys can read order queue positions (2026-07-30) | No code change — permission-only, no shape change |
| Event `product_metadata` now includes `cadence` (2026-07-30) | Added `cadence: Option<String>` to `EventMetadata` |
| Subaccount-restricted API keys can use batch order endpoints (2026-07-30) | No code change — permission-only |
| Subaccount on `quote_created` (2026-07-30) | Added `subaccount: Option<u32>` to `WsQuoteCreated`/`Ref` |
| Subaccount-restricted API keys can manage order groups (2026-07-30) | No code change — permission-only |
| Order groups limited to 25,000 per user (2026-07-23) | No code change — quota/rate-limit change only |
| Incentive programs on hidden events excluded from listing (2026-07-22) | No code change — filtering/visibility behavior only |
| Historical positions endpoint (2026-07-23) | Added `get_historical_positions` + `GetHistoricalPositionsParams`; added `market_positions_last_updated_ts` to `GetHistoricalCutoffResponse` |
| Subaccount-restricted API keys can open WebSocket sessions (2026-07-23) | No code change — permission-only |
| Subaccount-restricted API keys can quote on RFQ FIX sessions (2026-07-23) | No code change — FIX not supported |
| Pyth value WebSocket channel (2026-07-23) | Added full `pyth_value` channel, subscription, and message support |
| Support for FIX Tag 2446 on Incremental Refresh (2026-07-09) | No code change — FIX not supported |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote`; deprecated `get_quote` |
| Exchange announcements endpoint removed (2026-07-04) | **Breaking** — removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| Deprecated Predictions REST schema fields removed (2026-07-09) | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (and the mirrored `MarketPositionRef.resting_orders_count`) |
| Margin orders now identify system order reasons (2026-07-09) | No code change — margin orders not modeled |
| New price level structures (2026-07-23) | No code change — `price_level_structure` already an untyped `String`; new values documented in `docs/spec-parity.md` |
| Multivariate lookup history endpoints are fully deprecated (2026-07-02) | No code change at the time; the surface was fully removed 2026-08-06 (see above) |
| Margin positions now include an `is_portfolio` flag (2026-07-02) | No code change — margin positions not modeled |
| Trade-scoped API key permissions (2026-06-30) | No code change — scopes already stored as `Vec<String>` |
| `price_ranges` added to `market_lifecycle_v2` events (2026-07-02) | Added `price_ranges: Option<Vec<PriceRange>>` to `WsMarketLifecycleV2`/`Ref` |
| Margin positions `margin_used` omitted for jointly-margined portfolio positions (2026-06-29) | No code change — margin positions not modeled |
| Margin risk per-market metrics limited to single-position subaccounts and gross margin markets (2026-06-26) | No code change — margin risk not modeled |
| Per-index exchange status (2026-07-02) | Added `intra_exchange_transfers_active`, `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` to `GetExchangeStatusResponse` |
| Per-index subaccount balances (2026-07-02) | Added required `exchange_index: u32` to `SubaccountBalance` |
| AcceptQuote rejects carry a specific reason on FIX (2026-07-02) | No code change — FIX not supported |
| More specific FIX rejects for cancel/replace failures (2026-07-02) | No code change — FIX not supported |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; deprecated quote-ID-only action endpoints; retention behavior itself needs no code change |
| API usage tier qualification requirements halved (2026-06-25) | No code change — no shape change |
| FIX exchange index routing (2026-06-25) | No code change — FIX not supported |
| RFQ quotes support post-only on FIX (2026-06-24) | No code change — FIX not supported |
| Get Quote rate-limit cost reduced to 2 tokens (2026-06-23) | No code change — rate-limit cost only |
| RFQ quote market and event filters removed (2026-06-20) | **Breaking** — removed `market_ticker` / `event_ticker` from `GetQuotesParams` |
| Communications RFQ and quote retention window reduced (2026-06-19) | No code change — retention window only |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount: Option<u32>` to `CreateApiKeyRequest`, `GenerateApiKeyRequest`, `ApiKey` |
| `settlement_sources` added to the events API (2026-06-18) | Added `settlement_sources: Vec<SettlementSource>` to `EventData` |
| Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` (2026-06-18) | Added top-level `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2`/`Ref` |
| RFQ quote identity on FIX (2026-06-18) | No code change — FIX not supported |
| Trade entries in FIX market data (2026-06-18) | No code change — FIX not supported |
| Legacy order mutation endpoints deprecated (2026-06-18) | Added `#[deprecated]` to `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` |
| Event tickers filter on `GET /events` (2026-06-18) | Added `tickers: Option<Vec<String>>` to `GetEventsParams` |
| Subaccount on margin positions (2026-06-18) | No code change — margin positions not modeled |
| Block-trade accept API key permissions (2026-06-18) | No code change — scopes already stored as `Vec<String>` |
| Sanity limits enforced on orderbook subscriptions (2026-06-18) | No code change — server-side rate/subscription-count limits only |
| Quote time filters and pagination fix (2026-06-18) | Added `min_ts`/`max_ts` to `GetQuotesParams`; the pagination fix itself is server-side only |
| API usage volume progress endpoint (2026-06-11) | **New** (not covered by 0.6.0) — added `get_account_api_usage_level_volume_progress` + `GetAccountApiUsageLevelVolumeProgressResponse` types |
| Perps mark prices on margin markets (2026-06-11) | No code change — margin markets not modeled |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | **New** (not covered by 0.6.0) — added `upgrade_account_api_usage_level` |
| Margin fee-tier endpoint returns active rates (2026-06-11) | Already covered by 0.6.0 refresh (see prior watermark table) |
| Perps volume and open interest notional fields (2026-06-11) | Already covered by 0.6.0 refresh (see prior watermark table) — margin/WS `margin_ticker` not modeled |
| Tick size added to `GET /margin/markets` (2026-06-11) | Already covered by 0.6.0 refresh (see prior watermark table) |
| Fractional quantities for RFQs (2026-06-11) | Already covered by 0.6.0 refresh (see prior watermark table) |
| Introducing automated API rate-limit tiers (2026-06-05, live 2026-06-11) | Already covered by 0.6.0 refresh (see prior watermark table) |

### Added

- [Rust API] Added `is_block_trade: bool` (`#[serde(default)]`) to `WsTrade` / `WsTradeRef`, matching
  the REST `Trade.is_block_trade` precedent (2026-08-13).
- [Rust API] Added `ExchangeIndexStatus` and `GetExchangeStatusResponse.intra_exchange_transfers_active`
  / `.exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` for the per-exchange-index status
  breakdown (2026-07-02, `description` 2026-08-13).
- [Rust API] Added `get_intra_exchange_instance_transfers` (+ `_pager` / `_all` / `stream_*`) and
  `get_intra_exchange_instance_transfer`, plus `IntraExchangeInstanceTransfer`,
  `GetIntraExchangeInstanceTransfersParams`, `GetIntraExchangeInstanceTransfersResponse`,
  `GetIntraExchangeInstanceTransferResponse` for the new `/portfolio/intra_exchange_instance_transfers`
  endpoints (2026-08-13).
- [Rust API] Added `exchange_index: Option<u32>` to `MultivariateEventCollection` (2026-08-06).
- [Rust API] Added `exchange_index: Option<i64>` to `WsMarketLifecycleV2` / `Ref` (present only on
  `created` events) and `WsEventLifecycle` / `Ref` (2026-07-30).
- [Rust API] Added `exchange_index: Option<u32>` to `Series` (2026-07-30).
- [Rust API] Added `get_event_live_data` and `EventLiveData` / `GetEventLiveDataParams` /
  `GetEventLiveDataResponse` for the new public `GET /live_data/events/{event_ticker}` endpoint
  (2026-07-30).
- [Rust API] Added `cadence: Option<String>` to `EventMetadata` (2026-07-30).
- [Rust API] Added `subaccount: Option<u32>` to `WsQuoteCreated` / `Ref` (2026-07-30).
- [Rust API] Added `get_historical_positions` + `GetHistoricalPositionsParams` for the new
  `GET /historical/positions` endpoint, and `market_positions_last_updated_ts: Option<String>` to
  `GetHistoricalCutoffResponse` (2026-07-23).
- [Rust API] Added full `pyth_value` AsyncAPI channel support, mirroring the `cfbenchmarks_value`
  pattern added in 0.6.0 (2026-07-23):
  - `WsChannelV2::PythValue` variant (private/authenticated channel)
  - `underlying_tickers: Option<Vec<String>>` on `WsSubscriptionParamsV2` (seed on initial subscribe;
    use `["all"]` for every underlying) and on `WsUpdateSubscriptionParamsV2`
  - `WsMsgType::PythValue` and `WsMsgType::PythValueUnderlyingList` variants
  - New types `WsPythValue`, `WsPythValueRef`, `WsPythUnderlyingList`, `WsPythUnderlyingListRef` in
    `ws::types::messages::pyth`
  - `WsDataMessageV2::PythValue` / `PythValueUnderlyingList` and `WsDataMessageRef` equivalents, routed
    through both the wire and envelope parse paths
  - `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` variants, with
    `validate_update` enforcing the same mutual-exclusivity/required-field rules as the CF Benchmarks
    index actions, and `SubscriptionTracker` folding underlying add/remove updates the same way
- [Rust API] Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` for the
  new RFQ-scoped quote lookup/action endpoints (lookup 2026-07-09; delete/accept/confirm 2026-06-25).
- [Rust API] Added `min_ts: Option<i64>` / `max_ts: Option<i64>` to `GetQuotesParams` (2026-06-18).
- [Rust API] Added `subaccount: Option<u32>` to `CreateApiKeyRequest`, `GenerateApiKeyRequest`, and
  `ApiKey` for subaccount-restricted API keys (2026-07-02).
- [Rust API] Added top-level `strike_type: Option<String>`, `cap_strike: Option<f64>`,
  `custom_strike: Option<BTreeMap<String, String>>` to `WsMarketLifecycleV2` / `Ref`, present only on
  `metadata_updated` events alongside the pre-existing `floor_strike` / `yes_sub_title` (2026-06-18).
- [Rust API] Added `price_ranges: Option<Vec<PriceRange>>` to `WsMarketLifecycleV2` / `Ref`, present on
  `created` and `price_level_structure_updated` events (2026-07-02).
- [Rust API] Added `tickers: Option<Vec<String>>` to `GetEventsParams` (2026-06-18).
- [Rust API] Added `settlement_sources: Vec<SettlementSource>` to `EventData` (mirrors
  `Series.settlement_sources`) and `exchange_index: Option<u32>` (2026-06-18 / current schema).
- [Rust API] Added `get_account_api_usage_level_volume_progress` +
  `GetAccountApiUsageLevelVolumeProgressResponse` / `AccountApiUsageLevelVolumeProgress` /
  `AccountApiUsageLevelVolumeGoal` for `GET /account/api_usage_level/volume_progress` (2026-06-11).
- [Rust API] Added `upgrade_account_api_usage_level` for `POST /account/api_usage_level/upgrade`
  (2026-06-11).
- [Rust API] Added required `exchange_index: u32` to `SubaccountBalance` (2026-07-02).
- [Tests] Added/extended coverage: `ExchangeIndexStatus` parsing, `WsMarketLifecycleV2`
  `exchange_index` / `price_ranges` / `strike_type` / `cap_strike` / `custom_strike`, `WsEventLifecycle`
  `exchange_index`, `pyth_value` / `pyth_value_underlying_list` envelope + subscription-tracker +
  `validate_update` tests, `GetHistoricalCutoffResponse.market_positions_last_updated_ts`,
  `SubaccountBalance.exchange_index`, `MultivariateEventCollection.exchange_index`.

### Changed

- [Rust API] `update_order_group_limit` now takes a `params: SubaccountQueryParams` argument
  (`subaccount` is a query parameter, matching the sibling order-group endpoints) (2026-08-06).
- [Docs] `ErrorResponse.service` doc comment updated to note it is fully removed upstream (2026-08-06)
  and always deserializes to `None` going forward; kept as `Option<String>` rather than removed.

### Deprecated

- [Rust API] `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders` are marked `#[deprecated(since = "0.7.0")]` — the legacy (pre-V2)
  `/portfolio/orders` mutation endpoints were deprecated upstream 2026-06-18 in favor of the V2
  `/portfolio/events/orders/*` endpoints added in 0.6.0. `get_order` / `get_orders` /
  `get_order_queue_position(s)` are read-only and were not deprecated upstream.
- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` are marked
  `#[deprecated(since = "0.7.0")]` in favor of the new RFQ-scoped `get_rfq_quote` / `delete_rfq_quote` /
  `accept_rfq_quote` / `confirm_rfq_quote`. Still supported upstream for now.

### Removed

- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed from the
  Predictions REST API (2026-07-04). Use `GET /exchange/schedule` instead.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` (and the mirrored `MarketPositionRef.resting_orders_count`) —
  removed from the OpenAPI schema (2026-07-09). Use `Market.price_level_structure` /
  `Market.price_ranges` instead.
- [Rust API] Removed `fractional_trading_enabled` from `WsMarketLifecycleV2` / `Ref` and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant — no longer present in the current
  AsyncAPI `market_lifecycle_v2` payload or event-type enum.
- [Rust API] Removed the multivariate lookup surface (2026-08-06): `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`,
  `LookupTickersForMarketInMultivariateEventCollectionResponse`,
  `GetMultivariateEventCollectionLookupHistoryParams`,
  `GetMultivariateEventCollectionLookupHistoryResponse`, `LookupPoint`; `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate` / `MultivariateLookup`, `WsMultivariate`, `WsMultivariateRef`,
  `WsMultivariateSelectedMarket`, `WsMultivariateSelectedMarketRef`,
  `WsDataMessageV2::Multivariate` / `WsDataMessageRef::Multivariate`. The `multivariate_market_lifecycle`
  channel is unaffected.
- [Rust API] Removed `market_ticker` / `event_ticker` from `GetQuotesParams` — removed from
  `GET /communications/quotes` upstream, effective 2026-06-20.

### Breaking

- [Rust API] `get_exchange_announcements()` and all `Announcement*` / `GetExchangeAnnouncementsResponse`
  types no longer exist. Downstream code using them must remove those call sites.
- [Rust API] `Market` no longer has `response_price_units` or `fractional_trading_enabled`.
  `MarketPosition` no longer has `resting_orders_count`. Downstream exhaustive struct destructuring
  must drop these fields.
- [Rust API] `WsMarketLifecycleV2` / `Ref` no longer has `fractional_trading_enabled`, and
  `WsMarketLifecycleEventType` no longer has `FractionalTradingUpdated` (payloads with this event type
  now fall through to `Unknown`). Downstream exhaustive matches must update.
- [Rust API] The multivariate lookup surface (REST methods/types and WS `Multivariate` channel/message
  types) is fully removed; see the Removed section above for the full list.
- [Rust API] `GetQuotesParams` no longer has `market_ticker` or `event_ticker`. Downstream struct
  literals using these fields must drop them (use `..Default::default()` or `rfq_id` / status / user
  filters instead).
- [Rust API] `update_order_group_limit(order_group_id, body)` is now
  `update_order_group_limit(order_group_id, params, body)`. Downstream call sites must pass a
  `SubaccountQueryParams` (use `SubaccountQueryParams::default()` to preserve prior behavior).
- [Rust API] `SubaccountBalance` gained a required `exchange_index: u32` field; downstream struct
  literals constructing this type must add it.

### Fixed

- [Tests] Fixed a pre-existing `tests/rest_auth.rs` compile error under `--features live-tests`
  (`resp.read_limit` / `resp.write_limit` no longer exist post-0.6.0) by updating to
  `resp.read.refill_rate` / `resp.write.refill_rate`.
- [Docs] Fixed a stale `examples/ws_user_orders_v2.rs` reference in `README.md` (the file is
  `examples/ws_user_orders.rs`).


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
