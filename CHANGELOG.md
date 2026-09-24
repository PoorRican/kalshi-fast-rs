# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-24

### Compatibility

- Docs snapshot: 2026-09-24
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-10-01

Per [`VERSIONING.md`](VERSIONING.md): this release contains multiple intentional
breaking Rust API changes (removed methods, removed fields, changed method
signatures), so pre-1.0 the correct bump is **minor** (0.7.0 → 0.8.0).

**Changelog entries since the 0.7.0 watermark (2026-06-08) and disposition.**
Entries tagged only `FIX` or only `Margin` in the upstream changelog are
omitted below — this crate implements only the Kalshi Predictions REST/WebSocket
surface, not the FIX protocol and not the separate Margin exchange's dedicated
markets/positions/orders (a couple of Margin-adjacent trade-api endpoints, like
`GET /margin/fee_tiers`, are already covered from an earlier refresh and were
unaffected this pass).

<details>
<summary>99 upstream entries reviewed, expand for the full disposition table</summary>

| Entry (date) | Disposition |
|---|---|
| API usage volume progress endpoint (06-11) | Added `get_account_api_usage_level_volume_progress()` |
| Fractional quantities for RFQs (06-11) | No change — `contracts_fp` already fixed-point string |
| Self-serve Advanced API usage tier upgrade (06-11) | Added `upgrade_account_api_usage_level()` |
| Block-trade accept API key permissions (06-18) | No change — scopes are unvalidated free-form strings |
| Event tickers filter on GET /events (06-18) | Added `GetEventsParams.tickers` |
| Legacy order mutation endpoints deprecated (06-18) | **Removed** `create/cancel/amend/decrease_order`, `batch_{create,cancel}_orders` (confirmed gone from spec; use `_v2`) |
| Quote time filters and pagination fix (06-18) | Added `GetQuotesParams.min_ts`/`max_ts` |
| Sanity limits enforced on orderbook subscriptions (06-18) | No change — server-enforced operational cap only |
| Strike type/cap strike on metadata_updated (06-18) | Added `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| settlement_sources added to events API (06-18) | Added `EventData.settlement_sources` |
| Communications RFQ/quote retention window reduced (06-19) | No change — data-lifecycle policy only |
| RFQ quote market/event filters removed (06-20) | Removed `GetQuotesParams.market_ticker`/`event_ticker` |
| Get Quote rate-limit cost reduced (06-23) | No change — token costs not hardcoded client-side |
| API usage tier qualification halved (06-25) | No change — server-side threshold only |
| RFQ quote retention and RFQ-scoped actions (06-25) | Added `get/delete/accept/confirm_rfq_quote()`; deprecated quote-ID-only equivalents |
| Trade-scoped API key permissions (06-30) | No change — free-form scope string |
| Multivariate lookup history fully deprecated (07-02) | **Removed** lookup REST endpoints/types (confirmed gone from spec) |
| Per-index exchange status (07-02) | Added `ExchangeIndexStatus`/`exchange_index_statuses` to `GetExchangeStatusResponse` |
| Per-index subaccount balances (07-02) | Added `SubaccountBalance.exchange_index` |
| Sub-account-restricted API keys (07-02) | Added `subaccount` to `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey` |
| price_ranges on market_lifecycle_v2 events (07-02) | Added `WsMarketLifecycleV2.price_ranges` |
| Exchange announcements endpoint removed (07-04) | **Removed** `get_exchange_announcements()` and types (confirmed gone from spec) |
| Deprecated Predictions REST fields removed (07-09) | **Removed** `Market.response_price_units`/`fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| RFQ-scoped quote lookup endpoint (07-09) | Covered by 06-25 RFQ-scoped methods |
| Incentive programs on hidden events excluded (07-22) | No change — server-side visibility filter only |
| Historical positions endpoint (07-23) | Added `get_historical_positions()` |
| New price level structures (07-23) | No change — untyped string passthrough already accepts new values |
| Order groups limited to 25,000/user (07-23) | No change — server-side limit only |
| Pyth value WebSocket channel (07-23) | Added `WsChannelV2::PythValue` + full channel support |
| Subaccount-restricted keys can open WS sessions (07-23) | No change — server-side authorization only |
| service field on error responses deprecated (07-28) | Superseded by 08-06 removal |
| Event product_metadata includes cadence (07-30) | Added `EventMetadata.cadence` |
| Lifecycle creation messages include exchange_index (07-30) | Added `exchange_index` to `WsMarketLifecycleV2`/`WsEventLifecycle` (latter was silently dropping it — bug fix) |
| New endpoint for event-keyed live data (07-30) | Added `get_event_live_data()` |
| Richer combo-validation errors (07-30) | No change — generic `ErrorResponse.message`/`details` already capture it |
| Series responses include exchange_index (07-30) | Added `Series.exchange_index` (+ `extra` catch-all, previously had none) |
| Subaccount on quote_created (07-30) | Added `subaccount` to `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` |
| Subaccount-restricted keys can manage order groups (07-30) | No change — server-side authorization only |
| Subaccount-restricted keys can read queue positions (07-30) | No change — already supported |
| Subaccount-restricted keys can use batch orders (07-30) | No change — already supported |
| Multivariate event collections include exchange_index (08-06) | Added `MultivariateEventCollection.exchange_index` |
| Multivariate lookup endpoint/channel removed (08-06) | **Removed** WS `multivariate`/`multivariate_lookup` channel + message types (confirmed gone from spec) |
| Order group limit updates support subaccounts (08-06) | Added `subaccount` query param to `update_order_group_limit()` |
| service field removed from error responses (08-06) | **Removed** `ErrorResponse.service` (confirmed gone from spec) |
| Balance reads scoped by exchange_index (08-13) | Covered by 08-20 balance changes |
| Block trade indicator for WS trades (08-13) | Added `WsTrade.is_block_trade` (was silently dropped — bug fix) |
| Exchange shard descriptions (08-13) | Covered by 07-02 `ExchangeIndexStatus` |
| Intra-account transfer history endpoints (08-13) | Added `get_intra_exchange_instance_transfers()`/`get_intra_exchange_instance_transfer()` |
| New center_deci_edge_centi_cent price structure (08-13) | No change — untyped string passthrough |
| Order group max increased to 100,000/user (08-13) | No change — server-side limit only |
| API key location attestation expiry (08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| Cross-shard subaccount transfers (08-20) | Added `create_intra_exchange_instance_transfer()` |
| Exchange index filters for portfolio lists (08-20) | Added `exchange_index` to `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` |
| Exchange index on portfolio/WS fill records (08-20) | Added `exchange_index` to `Fill`/`Settlement`/`MarketPosition`/`WsFill` |
| Kalshi Weather Index endpoint (08-20) | Added `get_weather_index()` (new module) |
| Maker fee exemption, NFL combo markets (08-20) | No change — fee business rule, surfaced via existing fee fields |
| Optional balance reads by exchange_index (08-20) | Added `subaccount`/`exchange_index` params + `balance_breakdown` to `get_balance()` |
| RFQs/combo creation for subaccount-restricted keys (08-20) | No change — server-side authorization only |
| Resting order value breakdown by exchange index (08-20) | Added `resting_order_value_breakdown` to `GetPortfolioRestingOrderTotalValueResponse` |
| Target balance allocation endpoints (08-20) | Added `get/set_target_balance_allocation()` + `RestingMarginReservation` |
| VPC peering for Prime members (08-20) | No change — infra offering, docs-only |
| Combo RFQ fee assignment, briefly resting orders (08-22) | No change — fee logic surfaced via existing fields |
| Post-only quotes preserved; crossing rate limits (08-22) | Added `Quote`/`CreateQuoteRequest.post_only` |
| Upcoming exchange sharding (08-24) | No change — `exchange_index` already generic integer |
| Cancel-all-orders endpoints (08-27) | Added `cancel_all_orders()` |
| Exchange auto-routing enabled by default (08-27) | No change — `exchange_index` already optional everywhere |
| Exchange index on user order messages (08-27) | Added `WsUserOrder.exchange_index` |
| Historical CF Benchmarks via REST passthrough (08-27) | No change — passthrough isn't part of the versioned OpenAPI schema |
| Localized market content in REST responses (08-27) | No change — `Accept-Language` header, same JSON shape |
| available_on_brokers deprecated (08-27) | Superseded by 09-10 removal |
| Structured target images, Trade API v2 (08-29) | No change — `StructuredTarget.details` already untyped map |
| Weather index calibration history (08-31) | Added `get_weather_index_calibrations()` |
| CF Benchmarks 5Hz value WS channel (09-03) | Added `WsChannelV2::CfbenchmarksValue5Hz` + full channel support |
| Correct remaining counts after amendments (09-03) | No change — value-correctness fix only, shape unchanged |
| Filter FCM orders by client order IDs (09-03) | Added `GetFcmOrdersParams.client_order_ids`, `subtrader_id` now optional |
| Filter historical positions by subaccount (09-03) | Covered by 07-23 `get_historical_positions()` (`subaccount` param) |
| Lower rate-limit cost, cancel all orders (09-03) | No change — token costs not hardcoded client-side |
| Shard rebalance margin reservation (09-03) | Covered by 08-20 target balance allocation |
| Tapered sub-cent pricing, combo markets (09-03) | No change — changelog states no field/format changes |
| Principal-only sizing for target-cost RFQs (09-10) | Added `CreateRFQRequest`/`RFQ.target_cost_excludes_fees` |
| center_deci_edge_centi_cent emitted again (09-10) | No change — untyped string passthrough |
| available_on_brokers field removed (09-10) | **Removed** `EventData.available_on_brokers` (confirmed gone from spec) |
| Upcoming sharding, commodities/basketball (09-10) | No change — no crate logic keyed off shard assignment |
| Weather index points expose receipt_basis (09-10) | No change — field already typed on `WeatherIndexPoint` added this pass |
| WebSocket schemas corrected (09-10) | No change — crate's `Option`/flatten-`extra` modeling already tolerant |
| Historical fills/orders support min_ts (09-17) | Added `min_ts` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| RFQ/quote writes share shard-1 budget (09-17) | No change — rate limiter is generic local RPS throttle |
| Reduced rate limit, QuoteConfirm with RFQ ID (09-17) | No change — same reasoning |
| Returning to idiomatic MVE series (09-17) | No change — tickers are opaque strings, no hardcoded prefix logic |
| Series responses include categories list (09-17) | Added `Series.categories` |
| Target balance allocations include reservation policy (09-17) | Covered by 08-20 `RestingMarginReservation` |
| WebSocket schema corrections (09-17) | No change — Predictions fields already correctly typed |
| WS subscriptions ready when acknowledged (09-17) | No change — client already forwards messages in receipt order |
| 20% higher read/write rate limits (09-24) | No change — numeric tiers not modeled client-side |
| Ed25519 API keys (09-24) | Added `ApiKeyType`/`key_type` to REST key types; **signing remains RSA-only** (see Known Gaps below) |
| Optional WebSocket compression (09-24) | No change — uncompressed connections remain valid per spec wording |
| Orders historical cutoff advances independently (09-24) | No change — `GetHistoricalCutoffResponse` shape unchanged |
| Rebalancing without resting-order reservation (09-24) | Covered by 08-20 `RestingMarginReservation::None` |
| Subaccount-scoped historical fills/orders (09-24) | Added `subaccount` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |

</details>

### Added

- [Rust API] New REST endpoints: `get_account_api_usage_level_volume_progress`,
  `upgrade_account_api_usage_level`, `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`
  (RFQ-scoped quote actions), `get_historical_positions`, `get_event_live_data`,
  `cancel_all_orders`, `create_intra_exchange_instance_transfer`/`get_intra_exchange_instance_transfers`/`get_intra_exchange_instance_transfer`,
  `get_target_balance_allocation`/`set_target_balance_allocation`, and a new
  Kalshi Weather Index module (`get_weather_index`, `get_weather_index_calibrations`).
- [Rust API] New WebSocket channels: `WsChannelV2::PythValue` (Pyth reference price
  feed, paid/authenticated) and `WsChannelV2::CfbenchmarksValue5Hz` (5Hz sibling of
  the existing `cfbenchmarks_value` channel, paid/authenticated), with full
  subscribe/update-subscription support (`SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`
  actions for Pyth; the 5Hz channel reuses the existing index-action commands).
- [Rust API] `exchange_index` surfaced across the crate wherever the live spec now
  carries it: `Series`, `EventData`, `MultivariateEventCollection`, `SubaccountBalance`,
  `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams`, `Fill`, `Settlement`,
  `MarketPosition`, `WsFill`, `WsUserOrder`, `WsMarketLifecycleV2`, and `WsEventLifecycle`.
  The `WsEventLifecycle` addition is a correctness fix: the field is spec-required
  and was previously silently dropped (no `extra` catch-all existed on that struct).
- [Rust API] `WsTrade.is_block_trade` — also a correctness fix; the field is
  spec-required on `trade` messages and was previously silently dropped.
- [Rust API] `GetEventsParams.tickers`, `EventData.settlement_sources`,
  `EventMetadata.cadence`, `Series.categories` (distinct from the existing singular
  `category`), `GetQuotesParams.min_ts`/`max_ts`, `CreateRFQRequest`/`RFQ.target_cost_excludes_fees`,
  `Quote`/`CreateQuoteRequest.post_only`, `GetHistoricalFillsParams`/`GetHistoricalOrdersParams.min_ts`/`subaccount`,
  `GetHistoricalCutoffResponse.market_positions_last_updated_ts`,
  `GetFcmOrdersParams.client_order_ids`, `GetApiKeysResponse.api_key_region_expiration_ts`,
  `GetBalanceResponse.balance_breakdown` (+ new `IndexedBalance` type),
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`,
  `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses`
  (+ new `ExchangeIndexStatus` type), `WsMarketLifecycleV2.strike_type`/`cap_strike`/`custom_strike`/`price_ranges`,
  and `subaccount` on `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted`.
- [Rust API] `subaccount` on `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey`;
  `key_type: Option<ApiKeyType>` (`rsa` | `ed25519`) on `GenerateApiKeyRequest`/`GenerateApiKeyResponse`
  for the new Ed25519 API keys (see Known Gaps — signing itself is not yet implemented).
- [Tests] Extensive new coverage across every touched module: serde round-trips for
  every new/changed field and type, method-shape tests for every new endpoint, and
  dedicated regression tests for the two silently-dropped-field bugs fixed above.

### Changed

- [Rust API] `get_balance` now takes `subaccount: Option<u32>, exchange_index: Option<u32>`
  (previously no parameters).
- [Rust API] `update_order_group_limit` now takes a `SubaccountQueryParams` argument
  (previously order-group ID and body only).
- [Rust API] `GetFcmOrdersParams.subtrader_id` is now `Option<String>` (previously
  required), since it's now optional whenever `client_order_ids` is supplied instead.

### Removed

- [Upstream] [Rust API] The legacy (non-V2) order-mutation REST surface —
  `create_order`, `cancel_order`, `amend_order`, `decrease_order`,
  `batch_create_orders`, `batch_cancel_orders` and their request/response types —
  is gone from the live OpenAPI spec (confirmed via direct grep: no non-`V2`
  order-mutation `operationId` remains). Removed from the crate; use the
  `_v2` equivalents, already present since a prior release.
- [Upstream] [Rust API] The multivariate-event-collection lookup REST endpoints
  (`get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection` and their types)
  and the WebSocket `multivariate`/`multivariate_lookup` channel and message types
  are gone from the live specs. Removed from the crate. `multivariate_market_lifecycle`
  and `TickerPair` (used by the still-valid `create_market_in_multivariate_event_collection`)
  are unaffected.
- [Upstream] [Rust API] `get_exchange_announcements` and its types — the
  `/exchange/announcements` endpoint no longer exists in the live spec. Removed.
- [Upstream] [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`,
  and `MarketPosition.resting_orders_count` — all three fields are gone from the live
  OpenAPI schema (their replacements, `price_level_structure`/`price_ranges`, were
  already modeled). Removed.
- [Upstream] [Rust API] `EventData.available_on_brokers` — deprecated 2026-08-27,
  actually removed from the live schema by 2026-09-10. Removed.
- [Upstream] [Rust API] `ErrorResponse.service` — removed from the live
  `ErrorResponse` schema (now `code`/`message`/`details` only). Removed.

### Breaking

- [Rust API] All removals above are breaking. Downstream code calling any of the
  six removed legacy order-mutation methods must migrate to the `_v2` equivalents
  (single price + `BookSide` instead of separate yes/no prices). Code calling the
  removed multivariate-lookup or exchange-announcements methods, or matching
  exhaustively on `WsChannelV2`/`WsMsgType`/`WsDataMessageV2`/`WsDataMessageRef`
  (which lost the `Multivariate` variant and gained four new ones for the Pyth and
  CF Benchmarks 5Hz channels), must update accordingly. Code reading
  `Market.response_price_units`/`.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`, `EventData.available_on_brokers`, or
  `ErrorResponse.service` must remove those references.
- [Rust API] `get_balance()` and `update_order_group_limit()` signatures changed
  (see Changed above); call sites must be updated.

### Known Gaps

- [Docs] Ed25519 API keys: the live spec now lets `POST /api_keys`/`POST /api_keys/generate`
  register or generate an Ed25519 key alongside RSA (`key_type` field, now modeled
  on the REST request/response types — see Added above). Actually *signing* requests
  with an Ed25519 key is **not** implemented in this release: `src/auth.rs` remains
  RSA-PSS/SHA256-only end to end (REST headers and the WebSocket handshake).
  Supporting Ed25519 signing is a real, non-trivial follow-up (a new signing
  backend alongside the existing `RsaPrivateKey`-based one) tracked as a known gap
  rather than attempted in this refresh. See `docs/spec-parity.md`.
- [Docs] `CreateApiKeyRequest`/`GenerateApiKeyRequest` also gained an `fcm_subtrader_id`
  field, and `CreateApiKeyResponse`/`GenerateApiKeyResponse` gained a `warning` field,
  in the same docs snapshot; neither was added this pass. See `docs/spec-parity.md`.
- [Docs] `WsQuoteCreated`/`WsQuoteAccepted` are missing `rfq_creator_id`, which the
  AsyncAPI spec marks required on both messages (only `WsQuoteExecuted` has it
  modeled today). Pre-existing gap, not introduced by this refresh. See `docs/spec-parity.md`.


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
