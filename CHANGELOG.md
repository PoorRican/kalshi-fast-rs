# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-26

### Compatibility

- Docs snapshot: 2026-08-26
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-27

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

Per `VERSIONING.md`/the refresh workflow: `[Rust API]` marks a concrete code diff; unmarked
entries are dispositioned "no code change" with the reason given. Entries tagged only `FIX` are
out of scope (this crate does not implement the FIX API). Entries tagged only `Margin` are out of
scope for endpoints/fields under `margin market types not in crate` (per the 0.6.0 precedent);
`/margin/fee_tiers` remains the one Margin-tagged surface this crate models, and is called out
explicitly below where relevant.

| Entry (date) | Disposition |
|---|---|
| API usage volume progress endpoint (Jun 11) | **[Rust API]** Added `get_account_api_usage_level_volume_progress()` / `GetAccountApiUsageLevelVolumeProgressResponse` |
| Perps mark prices on margin markets (Jun 11) | No code change — margin market types not in crate |
| Self-serve Advanced API usage tier upgrade (Jun 11) | **[Rust API]** Added `upgrade_account_api_usage_level()` |
| Margin fee-tier endpoint returns active rates (Jun 11) | No code change — data-only change; `GetMarginFeeTiersResponse`'s untyped rate maps already tolerate it |
| Perps volume/OI notional fields (Jun 11) | No code change — margin market types/`margin_ticker` channel not in crate |
| Tick size added to GET Margin Markets (Jun 11) | No code change — margin market types not in crate |
| Fractional quantities for RFQs (Jun 11) | No code change — `contracts_fp`/`yes_contracts_offered_fp`/`no_contracts_offered_fp` already present; FIX not modeled |
| settlement_sources added to the events API (Jun 18) | **[Rust API]** Added `EventData.settlement_sources` |
| Strike type and cap strike on market_lifecycle_v2 metadata_updated (Jun 18) | **[Rust API]** Added `WsMarketLifecycleV2.strike_type`/`.cap_strike`/`.custom_strike` (top-level) |
| RFQ quote identity on FIX (Jun 18) | No code change — FIX not modeled |
| Trade entries in FIX market data (Jun 18) | No code change — FIX not modeled |
| Legacy order mutation endpoints deprecated (Jun 18) | **[Rust API]** Marked `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/`batch_cancel_orders` `#[deprecated]` in favor of the V2 equivalents |
| Event tickers filter on GET /events (Jun 18) | **[Rust API]** Added `GetEventsParams.tickers` |
| Subaccount on margin positions (Jun 18) | No code change — margin market types not in crate |
| Block-trade accept API key permissions (Jun 18) | No code change — scopes already modeled as `Vec<String>` |
| Sanity limits enforced on orderbook subscriptions (Jun 18) | No code change — operational rate-limit only |
| Quote time filters and pagination fix (Jun 18) | **[Rust API]** Added `GetQuotesParams.min_ts`/`.max_ts` |
| Communications RFQ/quote retention window reduced (Jun 19) | No code change — data-retention window only |
| RFQ quote market/event filters removed (Jun 20) | **[Rust API] Breaking** Removed `GetQuotesParams.market_ticker`/`.event_ticker` |
| Get Quote rate-limit cost reduced (Jun 23) | No code change — operational rate-limit only |
| RFQ quotes support post-only on FIX (Jun 24) | No code change — FIX not modeled |
| FIX exchange index routing (Jun 25) | No code change — FIX not modeled |
| API usage tier qualification requirements halved (Jun 25) | No code change — operational threshold only |
| RFQ quote retention and RFQ-scoped quote actions (Jun 25) | **[Rust API]** Added `get_quote_for_rfq`/`delete_quote_for_rfq`/`accept_quote_for_rfq`/`confirm_quote_for_rfq`; deprecated the quote-ID-only actions. FIX `RfqId` not modeled (FIX out of scope) |
| Margin risk per-market metrics limited (Jun 26) | No code change — margin market types not in crate |
| Margin positions margin_used omitted for portfolio positions (Jun 29) | No code change — margin market types not in crate |
| Trade-scoped API key permissions, `write::trade` (Jun 30) | No code change — scopes already modeled as `Vec<String>` |
| Sub-account-restricted API keys, `POST /api_keys` (Jul 2) | **[Rust API]** Added `CreateApiKeyRequest.subaccount`, `GenerateApiKeyRequest.subaccount`, `ApiKey.subaccount` |
| Per-index exchange status (Jul 2) | **[Rust API]** Added `GetExchangeStatusResponse.intra_exchange_transfers_active`/`.exchange_index_statuses`; added `ExchangeIndexStatus` |
| Per-index subaccount balances (Jul 2) | **[Rust API]** Added `SubaccountBalance.exchange_index` |
| price_ranges added to market_lifecycle_v2 events (Jul 2) | **[Rust API]** Added `WsMarketLifecycleV2.price_ranges` (reuses `rest::PriceRange`) |
| AcceptQuote rejects carry a specific reason on FIX (Jul 2) | No code change — FIX not modeled |
| More specific FIX rejects for cancel/replace failures (Jul 2) | No code change — FIX not modeled |
| Multivariate lookup history endpoints fully deprecated (Jul 2) | Superseded by the Aug 6 removal below |
| Margin positions `is_portfolio` flag (Jul 2) | No code change — margin market types not in crate |
| Exchange announcements endpoint removed (Jul 4) | **[Rust API] Breaking** Removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| RFQ-scoped quote lookup endpoint (Jul 9) | **[Rust API]** Added `get_quote_for_rfq()`; deprecated `get_quote()` (see Jun 25 entry above) |
| Deprecated Predictions REST schema fields removed (Jul 9) | **[Rust API] Breaking** Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| Margin orders identify system order reasons (Jul 9) | No code change — margin market types not in crate |
| Support for FIX Tag 2446 on Incremental Refresh (Jul 9) | No code change — FIX not modeled |
| Incentive programs on hidden events excluded (Jul 22) | No code change — server-side filtering only, no shape change |
| Historical positions endpoint (Jul 23) | Deferred — new endpoint (`GET /historical/positions`), not yet implemented; see `docs/spec-parity.md` |
| Subaccount-restricted API keys can open WebSocket sessions (Jul 23) | No code change — auth/session behavior only, no new fields |
| Subaccount-restricted API keys can quote on RFQ FIX sessions (Jul 23) | No code change — FIX not modeled |
| Pyth value WebSocket channel (Jul 23) | Deferred — new channel (`pyth_value`/`pyth_value_underlying_list`), not yet implemented; see `docs/spec-parity.md` |
| New price level structures ×7 (Jul 23) | No code change — `price_level_structure` already modeled as a raw `String`; no new fields |
| Order groups limited to 25,000 per user (Jul 23) | No code change — operational limit only |
| Multivariate lookup endpoint and channel removed (Aug 6) | **[Rust API] Breaking** Removed `lookup_tickers_for_market_in_multivariate_event_collection()`, `get_multivariate_event_collection_lookup_history()`, `LookupPoint`, related request/response types, and the WebSocket `multivariate` channel (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate(Ref)`) |
| Richer combo-validation errors on FIX RFQ creation (Aug 13) | No code change — FIX not modeled |
| Intra-account transfer history endpoints (Aug 13) | Deferred — new endpoints (`GET /portfolio/intra_exchange_instance_transfers[/{id}]`), not yet implemented; see `docs/spec-parity.md` |
| FIX execution reports identify source exchange index (Aug 6) | No code change — FIX not modeled |
| Sided leverage estimates on margin markets (Aug 6) | No code change — margin market types not in crate |
| Order group limit updates support subaccounts (Aug 6) | **[Rust API] Breaking** `update_order_group_limit()` now takes a `SubaccountQueryParams` parameter |
| Multivariate event collections include exchange_index (Aug 6) | **[Rust API]** Added `MultivariateEventCollection.exchange_index` |
| Richer combo-validation errors on multivariate market creation (Jul 30) | No code change — error body enrichment (`message`/`details`); crate's error type is not a rich per-endpoint model |
| The service field on error responses is deprecated (Jul 28) | No code change — `service` was never modeled on the crate's `ErrorResponse` |
| The service field has been removed from error responses (Aug 6) | No code change — see above |
| Lifecycle creation messages include exchange_index (Jul 30) | **[Rust API]** Added `WsMarketLifecycleV2.exchange_index`, `WsEventLifecycle.exchange_index` |
| Series responses include exchange_index (Jul 30) | **[Rust API]** Added `Series.exchange_index` |
| New endpoint for event-keyed live data (Jul 30) | Deferred — new endpoint (`GET /live_data/events/{event_ticker}`), not yet implemented; see `docs/spec-parity.md` |
| Subaccount-restricted keys read order queue positions (Jul 30) | No code change — auth/session behavior only |
| Event product_metadata now includes cadence (Jul 30) | No code change — `product_metadata` is an opaque object in the OpenAPI schema; `cadence`, when present, is preserved through `EventMetadata.extra` |
| Subaccount-restricted keys use batch order endpoints (Jul 30) | No code change — auth/session behavior only |
| Subaccount on quote_created (Jul 30) | **[Rust API]** Added `WsQuoteCreated.subaccount`/`.rfq_creator_id`; also added `WsQuoteAccepted.subaccount`/`.rfq_creator_id` and `WsQuoteExecuted.subaccount` for consistency with the same AsyncAPI schema family |
| Subaccount-restricted keys manage order groups (Jul 30) | No code change — auth/session behavior only |
| Historical CF Benchmarks values via REST passthrough (Aug 27) | No code change — documentation of an existing endpoint, no new shape |
| Exchange index on user order messages (Aug 27) | **[Rust API]** Added `WsUserOrder.exchange_index` |
| Trade type on FIX market data (Aug 27) | No code change — FIX not modeled |
| The available_on_brokers field is deprecated (Aug 27) | **[Rust API]** Marked `EventData.available_on_brokers` `#[deprecated]` (kept `Option<bool>`, not removed — Kalshi has not removed it yet) |
| Exchange auto-routing enabled by default (Aug 27) | No code change — routing/operational behavior only |
| Margin maker-volume incentive programs (Aug 27) | No code change — `IncentiveProgram`'s `extra` catch-all already tolerates the new `type`/`max_reward_per_account`; margin market types otherwise not in crate |
| Kalshi Weather Index endpoint (Aug 20) | Deferred — new endpoint (`GET /live_data/weather/{city}`), not yet implemented; see `docs/spec-parity.md` |
| Tapered sub-cent pricing on multivariate markets (Aug 27) | No code change — no new fields; `price_ranges`/`price_level_structure` already dynamic |
| Upcoming exchange sharding (Aug 24) | No code change — informational notice |
| Post-only quotes preserved; crossing rate limits (Aug 22) | No code change — fee/behavior only |
| Combo RFQ fee assignment for briefly resting orders (Aug 22) | No code change — fee/behavior only |
| Maker fee exemption for independent NFL combo markets (Aug 20) | No code change — fee/business-rule only |
| Entry timestamps for FIX market data (Aug 20) | No code change — FIX not modeled |
| Cross-shard subaccount transfers (Aug 20) | Deferred — `source_subaccount`/`destination_subaccount` on the intra-exchange-instance transfer request not yet added; see `docs/spec-parity.md` |
| Target balance allocation endpoints (Aug 20) | Deferred — new endpoints (`GET`/`POST /portfolio/target_balance_allocation`), not yet implemented; see `docs/spec-parity.md` |
| Resting order value breakdown by exchange index (Aug 20) | Deferred — `resting_order_value_breakdown` field not yet added to `GetPortfolioRestingOrderTotalValueResponse`; see `docs/spec-parity.md` |
| Exchange index on portfolio and WebSocket fill records (Aug 20) | **[Rust API]** Added `Fill.exchange_index`, `WsFill.exchange_index`, `MarketPosition.exchange_index` |
| Exchange index filters for portfolio lists (Aug 20) | **[Rust API]** Added `GetOrdersParams.exchange_index`, `GetPositionsParams.exchange_index`, `GetFillsParams.exchange_index` |
| RFQs/combo-market creation for sub-account-restricted keys (Aug 20) | No code change — auth/scoping behavior only |
| Optional balance reads by exchange_index (Aug 20) | **[Rust API]** Covered by the `GetBalanceParams` addition below |
| API key location attestation expiry (Aug 16) | **[Rust API]** Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| Exit triggers on margin positions (Aug 20) | No code change — margin market types not in crate |
| New center_deci_edge_centi_cent price level structure (Aug 13) | No code change — `price_level_structure` already modeled as a raw `String` |
| Balance reads scoped by exchange_index, `subaccount=0` semantics (Aug 13) | **[Rust API]** Covered by the `GetBalanceParams` addition below |
| Block trade indicator for WebSocket trades (Aug 13) | **[Rust API]** Added `WsTrade.is_block_trade` |
| Exchange shard descriptions (Aug 13) | **[Rust API]** Added `ExchangeIndexStatus.description` |
| Margin order groups bind to single exchange_index (Aug 13) | No code change — margin market types not in crate |
| Order group maximum increased to 100,000 (Aug 13) | No code change — operational limit only |
| VPC peering for Prime members (Aug 20) | No code change — connectivity/docs only |

### Added

- **[Rust API]** `Fill.exchange_index`, `MarketPosition.exchange_index`, `Series.exchange_index`,
  `EventData.exchange_index`, `MultivariateEventCollection.exchange_index`,
  `SubaccountBalance.exchange_index`, `WsFill.exchange_index`, `WsUserOrder.exchange_index`,
  `WsMarketLifecycleV2.exchange_index`, `WsEventLifecycle.exchange_index` — all `Option<u32>`,
  since the multi-shard rollout is still in progress. Added matching `exchange_index` filters to
  `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`, and the new `GetBalanceParams`.
- **[Rust API]** `GetExchangeStatusResponse.intra_exchange_transfers_active` /
  `.exchange_index_statuses`, and the new `ExchangeIndexStatus` struct
  (`exchange_index`, `description`, `exchange_active`, `trading_active`,
  `intra_exchange_transfers_active`).
- **[Rust API]** `GetBalanceParams` (`subaccount`, `exchange_index`); `get_balance()` now takes it.
- **[Rust API]** `WsMarketLifecycleV2` (and `Ref`) top-level `price_ranges: Option<Vec<PriceRange>>`,
  `strike_type: Option<String>`, `cap_strike: Option<f64>`, `custom_strike:
  Option<BTreeMap<String, String>>` (also covers the `multivariate_market_lifecycle` channel, which
  reuses this type).
- **[Rust API]** `WsTrade.is_block_trade: Option<bool>`.
- **[Rust API]** `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted`: added `subaccount:
  Option<u32>`; `WsQuoteCreated`/`WsQuoteAccepted` also gained `rfq_creator_id: Option<String>`
  (already present on `WsQuoteExecuted`).
- **[Rust API]** `EventData.settlement_sources: Vec<SettlementSource>`; `GetEventsParams.tickers:
  Option<Vec<String>>`.
- **[Rust API]** `ApiKey.subaccount`, `CreateApiKeyRequest.subaccount`,
  `GenerateApiKeyRequest.subaccount`, `GetApiKeysResponse.api_key_region_expiration_ts`.
- **[Rust API]** `get_account_api_usage_level_volume_progress()` /
  `GetAccountApiUsageLevelVolumeProgressResponse` / `AccountApiUsageLevelVolumeProgress` /
  `AccountApiUsageLevelVolumeGoal`; `upgrade_account_api_usage_level()`.
- **[Rust API]** RFQ-scoped quote actions: `get_quote_for_rfq`, `delete_quote_for_rfq`,
  `accept_quote_for_rfq`, `confirm_quote_for_rfq`.
- **[Rust API]** `GetQuotesParams.min_ts`/`.max_ts`.

### Changed

- **[Rust API] Breaking** `get_balance()` now takes a `GetBalanceParams` argument.
- **[Rust API] Breaking** `update_order_group_limit()` now takes a `SubaccountQueryParams`
  argument (before the request body).

### Deprecated

- **[Rust API]** `create_order`, `cancel_order`, `amend_order`, `decrease_order`,
  `batch_create_orders`, `batch_cancel_orders` — Kalshi deprecated the legacy `/portfolio/orders`
  mutation endpoints (2026-06-18); use the V2 event-order equivalents.
- **[Rust API]** `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` (quote-ID-only) — use
  the RFQ-scoped `*_for_rfq` equivalents.
- **[Rust API]** `EventData.available_on_brokers` — Kalshi deprecated it (2026-08-27); it is no
  longer populated and always returns `false`.

### Removed

- **[Rust API] Breaking** `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` — removed from the OpenAPI schema (2026-07-09).
- **[Rust API] Breaking** `WsMarketLifecycleV2.fractional_trading_enabled` and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant — absent from the current
  AsyncAPI schema (the `WsMarketLifecycleEventType::Unknown` fallback still tolerates an
  exchange that emits the old event type).
- **[Rust API] Breaking** `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was
  removed (2026-07-04).
- **[Rust API] Breaking** `lookup_tickers_for_market_in_multivariate_event_collection()`,
  `get_multivariate_event_collection_lookup_history()`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response`,
  `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint` — the multivariate
  lookup and lookup-history REST endpoints were removed (2026-08-06).
- **[Rust API] Breaking** `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/
  `MultivariateLookup`, `WsMultivariate`/`WsMultivariateRef` — the `multivariate` WebSocket
  channel (`multivariate_lookup` message type) was removed (2026-08-06); an unrecognized
  `multivariate`/`multivariate_lookup` frame now falls through to `WsMsgType::Unknown`, matching
  Kalshi's documented "unknown-channel error" behavior.
- **[Rust API] Breaking** `GetQuotesParams.market_ticker`/`.event_ticker` — Kalshi stopped honoring
  these filters (2026-06-20); use `min_ts`/`max_ts` or the other supported filters instead.

### Fixed

- **[Tests]** Fixed a pre-existing compile error in `ws::types::envelope`'s test module: two
  `WsMessageV2::ListSubscriptions`/`WsMessageRef::ListSubscriptions` match arms didn't account for
  the `sid`/`seq` fields added in 0.7.0, which broke `cargo test --all-targets`.
- **[Tests]** Fixed a pre-existing compile error in `tests/rest_auth.rs` (`--features live-tests`):
  `test_get_account_api_limits` still referenced `GetAccountApiLimitsResponse.read_limit`/
  `.write_limit`, removed by the 0.6.0 `read`/`write: BucketLimit` restructure.

### Breaking

- See the `[Rust API] Breaking` bullets above (`Changed`, `Deprecated`, `Removed`). Downstream code
  that: calls `get_balance()` or `update_order_group_limit()` with the old signatures; constructs
  `GetQuotesParams` with `market_ticker`/`event_ticker`; matches exhaustively on
  `WsMarketLifecycleEventType`, `WsChannelV2`, or `WsMsgType`; or references the removed
  announcements/multivariate-lookup types, must be updated.


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
