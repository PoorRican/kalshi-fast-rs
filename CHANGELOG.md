# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.7.0] - 2026-07-24

### Compatibility

- Docs snapshot: 2026-07-24
- OpenAPI: 3.26.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-07-24

**Changelog entries since 0.6.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| Legacy `/portfolio/orders` mutation endpoints deprecated/removed (2026-06-18 → confirmed gone by 2026-07-24) | **Breaking** — removed `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` and their V1-only request/response types. Use the `_v2` equivalents |
| Deprecated Predictions REST schema fields removed (2026-07-09) | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (REST and WS `MarketPositionRef`) |
| `market_lifecycle_v2` dropped `fractional_trading_enabled` / `fractional_trading_updated` (alongside the above) | **Breaking** — removed from `WsMarketLifecycleV2`/`Ref` and `WsMarketLifecycleEventType` |
| Exchange announcements endpoint removed (2026-07-04) | **Breaking** — removed `get_exchange_announcements` and `Announcement`/`AnnouncementType`/`AnnouncementStatus`/`GetExchangeAnnouncementsResponse` |
| RFQ quote market and event filters removed (2026-06-20) | **Breaking** — removed `market_ticker`/`event_ticker` from `GetQuotesParams` |
| Multivariate lookup history endpoints are fully deprecated (2026-07-02) | **Breaking** — removed `get_multivariate_event_collection_lookup_history` and its params/response/`LookupPoint` types (endpoint gone from spec); deprecated `lookup_tickers_for_market_in_multivariate_event_collection` |
| New price level structures (2026-07-23) | No code change — `price_level_structure` already `Option<String>` |
| `price_ranges` added to `market_lifecycle_v2` events (2026-07-23) | Added `price_ranges: Option<Vec<PriceRange>>` to `WsMarketLifecycleV2`/`Ref` |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote`; deprecated `get_quote` |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; deprecated `delete_quote`, `accept_quote`, `confirm_quote` |
| Quote time filters and pagination fix (2026-06-18) | Added `min_ts`/`max_ts` to `GetQuotesParams`; pagination fix is server-side |
| Communications RFQ and quote retention window reduced (2026-06-19) | No code change — retention window only |
| API usage tier qualification requirements halved (2026-06-25) | No code change — threshold only |
| Get Quote rate-limit cost reduced to 2 tokens (2026-06-23) | No code change — rate-limit cost only |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount: Option<u32>` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| Subaccount-restricted API keys can open WebSocket sessions (2026-07-23) | No code change — server-side authorization only |
| Trade-scoped / block-trade-accept API key permissions (2026-06-30, 2026-06-18) | No code change — `scopes: Vec<String>` already accepts any scope string |
| `settlement_sources` added to the events API (2026-06-18) | Added `EventData.settlement_sources: Vec<SettlementSource>` |
| Event tickers filter on `GET /trade-api/v2/events` (2026-06-18) | Added `GetEventsParams.tickers: Option<String>` |
| Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` (2026-06-18) | No code change — `strike_type`/`cap_strike`/`custom_strike` already modeled on `WsMarketLifecycleAdditionalMetadata` |
| Historical positions endpoint (2026-07-23) | Added `get_historical_positions` (reuses `GetPositionsResponse`) and `GetHistoricalCutoffResponse.market_positions_last_updated_ts` |
| API usage volume progress endpoint (2026-06-11) | Added `get_account_api_usage_level_volume_progress`, `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal` |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Added `upgrade_account_api_usage_level` |
| Per-index exchange status (2026-07-02) | Added `intra_exchange_transfers_active` and `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` to `GetExchangeStatusResponse` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index: Option<ExchangeIndex>` |
| Pyth value WebSocket channel (2026-07-23) | Added full `pyth_value` channel, subscription, and message support |
| Order groups limited to 25,000 per user (2026-07-23) | No code change — operational limit only |
| Sanity limits enforced on orderbook subscriptions (2026-06-18) | No code change — operational rate limit only |
| Incentive programs on hidden events excluded from listing (2026-07-22) | No code change — server-side filtering only |
| Margin order/position/market/perps changes (system order reasons, mark prices, `is_portfolio`, `margin_used`, risk-metric scoping, subaccount positions; various dates) | No code change — margin markets/orders/positions not modeled in crate |
| FIX-tagged entries (Tag 2446, exchange index routing, RFQ post-only, RFQ quote identity, trade entries, AcceptQuote/cancel-replace reject reasons, subaccount-restricted quoting; various dates) | No code change — crate does not implement a FIX client |

### Fixed

- [Tests] `tests/rest_auth.rs::test_get_account_api_limits` referenced the removed
  `GetAccountApiLimitsResponse::{read_limit, write_limit}` fields from before the 0.6.0
  `BucketLimit` restructure. Only compiles under `--features live-tests`, which CI's
  `cargo clippy --all-targets` does not exercise, so the break went unnoticed until this refresh.
  Updated to `resp.read.refill_rate` / `resp.write.refill_rate`.

### Added

- [Rust API] Added RFQ-scoped quote endpoints: `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote` (`GET`/`DELETE`/`PUT` `/communications/rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`).
- [Rust API] Added `GetQuotesParams::{min_ts, max_ts, user_filter}` and `GetRFQsParams::user_filter`.
- [Rust API] Added `EventData.settlement_sources: Vec<SettlementSource>` and
  `GetEventsParams.tickers: Option<String>`.
- [Rust API] Added `get_historical_positions` / `GetHistoricalPositionsParams` (`GET
  /historical/positions`) and `GetHistoricalCutoffResponse.market_positions_last_updated_ts`.
- [Rust API] Added `ApiKey.subaccount`, `CreateApiKeyRequest.subaccount`,
  `GenerateApiKeyRequest.subaccount` for sub-account-restricted API keys.
- [Rust API] Added `get_account_api_usage_level_volume_progress` and
  `upgrade_account_api_usage_level`, plus `AccountApiUsageLevelVolumeProgress` /
  `AccountApiUsageLevelVolumeGoal` / `GetAccountApiUsageLevelVolumeProgressResponse`.
- [Rust API] Added `GetExchangeStatusResponse.{intra_exchange_transfers_active,
  exchange_index_statuses}` and the `ExchangeIndexStatus` / `ExchangeIndex` types.
- [Rust API] Added `SubaccountBalance.exchange_index`.
- [Rust API] Added `WsMarketLifecycleV2::price_ranges` (and the `Ref` variant).
- [Rust API] Added the `pyth_value` WebSocket channel: `WsChannelV2::PythValue` (private),
  `WsSubscriptionParamsV2::underlying_tickers`, `WsUpdateSubscriptionParamsV2::underlying_tickers`,
  `WsUpdateAction::{SubscribeUnderlyings, UnsubscribeUnderlyings, UnderlyingList}`, and
  `WsPythValue` / `WsPythUnderlyingList` message types routed through `WsDataMessageV2`.

### Changed

- [Rust API] Deprecated `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and
  `GetQuotesParams::{quote_creator_user_id, rfq_creator_user_id}` /
  `GetRFQsParams::creator_user_id` with `#[deprecated]`, matching their `deprecated: true` flags in
  the live OpenAPI spec.

### Removed

- [Rust API] Removed the legacy `/portfolio/orders` mutation surface: `create_order`,
  `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`,
  and their request/response types (`CreateOrderRequest`, `CreateOrderResponse`,
  `CancelOrderParams`, `CancelOrderResponse`, `AmendOrderRequest`, `AmendOrderResponse`,
  `DecreaseOrderRequest`, `DecreaseOrderResponse`, `BatchCreateOrdersRequest`,
  `BatchCreateOrdersResponse`, `BatchCreateOrdersIndividualResponse`,
  `BatchCancelOrdersRequestOrder`, `BatchCancelOrdersRequest`, `BatchCancelOrdersResponse`,
  `BatchCancelOrdersIndividualResponse`). These endpoints are absent from the live OpenAPI spec and
  from the published docs index. `get_orders`, `get_order`, and `Order` are unaffected.
- [Rust API] Removed `get_exchange_announcements` and `Announcement` / `AnnouncementType` /
  `AnnouncementStatus` / `GetExchangeAnnouncementsResponse` (endpoint removed upstream).
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` (REST and WS `MarketPositionRef`).
- [Rust API] Removed `WsMarketLifecycleV2::fractional_trading_enabled` (and `Ref`) and
  `WsMarketLifecycleEventType::FractionalTradingUpdated`.
- [Rust API] Removed `GetQuotesParams::{market_ticker, event_ticker}` (endpoint no longer accepts
  these filters).
- [Rust API] Removed `get_multivariate_event_collection_lookup_history` and
  `GetMultivariateEventCollectionLookupHistoryParams` / `...Response` / `LookupPoint` (endpoint
  removed upstream).

### Breaking

- [Rust API] See "Removed" above — six order-mutation methods, four exchange-announcement types,
  three `Market`/`MarketPosition` fields, one WS lifecycle field and one WS lifecycle enum variant,
  two `GetQuotesParams` fields, and the multivariate lookup-history endpoint/types were removed
  from the public API. Per `VERSIONING.md`, any breaking pre-1.0 Rust API change is a minor bump
  (0.6.0 → 0.7.0).


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
