# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-25

### Compatibility

- Docs snapshot: 2026-10-01
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-10-01

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| API usage volume progress endpoint (Jun 11) | Added `get_account_api_usage_level_volume_progress()` + `GetAccountApiUsageLevelVolumeProgressResponse` |
| Perps mark prices on margin markets (Jun 11) | No code change — margin market types not in crate |
| Self-serve Advanced API usage tier upgrade (Jun 11) | Added `upgrade_account_api_usage_level()` |
| Margin fee-tier endpoint returns active rates (Jun 11) | No code change — margin market types not in crate |
| Perps volume and open interest notional fields (Jun 11) | No code change — margin market types not in crate |
| Tick size added to GET Margin Markets (Jun 11) | No code change — margin market types not in crate |
| Fractional quantities for RFQs (Jun 11) | No code change — `contracts_fp` already modeled on `CreateRFQRequest`/`Quote`; FIX out of scope |
| settlement_sources added to the events API (Jun 18) | Added `EventData::settlement_sources: Option<Vec<SettlementSource>>` |
| Strike type and cap strike on market_lifecycle_v2 metadata_updated (Jun 18) | Added top-level `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2`/`Ref` |
| RFQ quote identity on FIX (Jun 18) | No code change — FIX not implemented in crate |
| Trade entries in FIX market data (Jun 18) | No code change — FIX not implemented in crate |
| Legacy order mutation endpoints deprecated (Jun 18) | No code change — legacy and V2 order methods both already present; legacy kept until actually removed |
| Event tickers filter on GET /trade-api/v2/events (Jun 18) | Added `GetEventsParams::tickers` |
| Subaccount on margin positions (Jun 18) | No code change — margin position types not in crate |
| Block-trade accept API key permissions (Jun 18) | No code change — scopes stored as untyped `Vec<String>` already |
| Sanity limits enforced on orderbook subscriptions (Jun 18) | No code change — server-side enforcement only |
| Quote time filters and pagination fix (Jun 18) | Added `GetQuotesParams::min_ts`/`max_ts` |
| Communications RFQ and quote retention window reduced (Jun 19) | No code change — server-side retention window only |
| RFQ quote market and event filters removed (Jun 20) | Deprecated `GetQuotesParams::market_ticker`/`event_ticker` (`#[deprecated]`, now server-ignored no-ops) |
| Get Quote rate-limit cost reduced to 2 tokens (Jun 23) | No code change — operational rate-limit change only |
| RFQ quotes support post-only on FIX (Jun 24) | No code change — FIX not implemented in crate |
| RFQ quote retention and RFQ-scoped quote actions (Jun 25) | Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`; deprecated the quote-ID-only equivalents; FIX parts out of scope |
| API usage tier qualification requirements halved (Jun 25) | No code change — numeric threshold change only |
| FIX exchange index routing (Jun 25) | No code change — FIX not implemented in crate |
| Margin risk per-market metrics limited to single-position subaccounts and gross margin markets (Jun 26) | No code change — margin market types not in crate |
| Margin positions margin_used omitted for jointly-margined portfolio positions (Jun 29) | No code change — margin position types not in crate |
| Trade-scoped API key permissions (Jun 30) | No code change — scopes stored as untyped `Vec<String>` already |
| Multivariate lookup history endpoints are fully deprecated (Jul 2) | See Aug 6 removal below — same endpoint |
| Margin positions now include an is_portfolio flag (Jul 2) | No code change — margin position types not in crate |
| price_ranges added to market_lifecycle_v2 events (Jul 2) | Added `WsMarketLifecycleV2::price_ranges: Option<Vec<PriceRange>>` (+ `WsPriceRangeRef`) |
| Per-index exchange status (Jul 2) | Added `GetExchangeStatusResponse::{intra_exchange_transfers_active, exchange_index_statuses}` + `ExchangeIndexStatus` |
| Per-index subaccount balances (Jul 2) | **Breaking** — added required `SubaccountBalance::exchange_index: u32` (was silently dropped, no catch-all) |
| AcceptQuote rejects carry a specific reason on FIX (Jul 2) | No code change — FIX not implemented in crate |
| More specific FIX rejects for cancel/replace failures (Jul 2) | No code change — FIX not implemented in crate |
| Sub-account-restricted API keys (Jul 2) | Added `subaccount: Option<u32>` to `ApiKey`/`CreateApiKeyRequest`/`GenerateApiKeyRequest`; FIX parts out of scope |
| Exchange announcements endpoint removed (Jul 4) | **Breaking** — removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` (endpoint 404s) |
| Support for FIX Tag 2446 on Incremental Refresh (Jul 9) | No code change — FIX not implemented in crate |
| RFQ-scoped quote lookup endpoint (Jul 9) | Added `get_rfq_quote()` (see Jun 25 entry) |
| Deprecated Predictions REST schema fields removed (Jul 9) | No code change — `response_price_units`/`fractional_trading_enabled`/`resting_orders_count` already `Option`; now always `None` |
| Margin orders now identify system order reasons (Jul 9) | No code change — margin order types not in crate |
| Incentive programs on hidden events excluded from listing (Jul 22) | No code change — server-side filtering behavior only |
| Order groups limited to 25,000 per user (Jul 23) | No code change — quota/validation change only, Margin half out of scope |
| Historical positions endpoint (Jul 23) | Added `get_historical_positions()` + `GetHistoricalPositionsParams`, reusing `GetPositionsResponse` |
| Subaccount-restricted API keys can open WebSocket sessions (Jul 23) | No code change — server-side authorization change only |
| Subaccount-restricted API keys can quote on RFQ FIX sessions (Jul 23) | No code change — FIX not implemented in crate |
| Pyth value WebSocket channel (Jul 23) | Deferred — new `pyth_value` channel; tracked in `docs/spec-parity.md` |
| New price level structures (Jul 23) | No code change — `price_level_structure` already modeled as opaque `Option<String>` |
| The service field on error responses is deprecated (Jul 28) | No code change — `ErrorResponse::service` already `Option<String>` |
| Richer combo-validation errors on multivariate market creation (Jul 30) | No code change — `ErrorResponse::message`/`details` already `Option<String>` |
| Lifecycle creation messages now include exchange_index (Jul 30) | **Breaking** — added required `WsEventLifecycle::exchange_index` (was silently dropped, no catch-all); added `WsMarketLifecycleV2::exchange_index` (already had a catch-all) |
| Series responses include exchange_index (Jul 30) | **Breaking** — added `Series::exchange_index: Option<u32>` (was silently dropped, no catch-all) |
| New endpoint for event-keyed live data (Jul 30) | Added `get_event_live_data()` + `EventLiveData`/`GetEventLiveDataResponse`/`GetEventLiveDataParams` |
| Subaccount-restricted API keys can read order queue positions (Jul 30) | No code change — `GetOrderQueuePositionsParams::subaccount` already present |
| Event product_metadata now includes cadence (Jul 30) | No code change — `Event::product_metadata` already flattens unknown keys losslessly |
| Subaccount-restricted API keys can use batch order endpoints (Jul 30) | No code change — batch request types already wrap `subaccount`-carrying order requests |
| Subaccount on quote_created (Jul 30) | Added `subaccount: Option<u32>` + required `rfq_creator_id` to `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` |
| Subaccount-restricted API keys can manage order groups (Jul 30) | No code change — order-group request types already carry `subaccount` |
| Multivariate lookup endpoint and channel removed (Aug 6) | **Breaking** — removed `lookup_tickers_for_market_in_multivariate_event_collection`/`get_multivariate_event_collection_lookup_history` + types, and the dead `WsChannelV2::Multivariate`/`WsMsgType::{Multivariate,MultivariateLookup}`/`WsDataMessageV2::Multivariate` variants + `WsMultivariate(Ref)` types |
| FIX execution reports identify the source exchange index (Aug 6) | No code change — FIX not implemented in crate |
| Sided leverage estimates on margin markets (Aug 6) | No code change — margin market types not in crate |
| Order group limit updates support subaccounts (Aug 6) | **Breaking** — `update_order_group_limit()` now takes a `SubaccountQueryParams` argument |
| Multivariate event collections include exchange_index (Aug 6) | No code change — `MultivariateEventCollection` already flattens unknown keys losslessly |
| The service field has been removed from error responses (Aug 6) | No code change — same `Option<String>` field as Jul 28 |
| New center_deci_edge_centi_cent price level structure (Aug 13) | No code change — `price_level_structure` already modeled as opaque `Option<String>`; FIX out of scope |
| Balance reads scoped by exchange_index (Aug 13) | **Breaking** — `get_balance()` now takes a `GetBalanceParams` argument; added `GetBalanceResponse::balance_breakdown` + `IndexedBalance` |
| Block trade indicator for WebSocket trades (Aug 13) | Added required `WsTrade::is_block_trade: bool` (`#[serde(default)]`, mirrors REST `Trade::is_block_trade`) |
| Exchange shard descriptions (Aug 13) | Added `ExchangeIndexStatus::description` (see Jul 2 entry) |
| Margin order groups bind to single exchange_index (Aug 13) | No code change — margin order-group types not in crate |
| Order group maximum increased to 100,000 per user (Aug 13) | No code change — operational limit change only |
| Richer combo-validation errors on FIX RFQ creation (Aug 13) | No code change — FIX not implemented in crate |
| Intra-account transfer history endpoints (Aug 13) | Deferred — new `/portfolio/intra_exchange_instance_transfer(s)` endpoints; tracked in `docs/spec-parity.md` |
| API key location attestation expiry (Aug 16) | No code change — `ApiKey` already flattens unknown keys losslessly (`api_key_region_expiration_ts` round-trips via `extra`) |
| VPC peering for Prime members (Aug 20) | No code change — infrastructure/connectivity offering, no API shape affected |
| Kalshi Weather Index endpoint (Aug 20) | Deferred — new `/live_data/weather/{city}` endpoint family; tracked in `docs/spec-parity.md` |
| Maker fee exemption for independent NFL combo markets (Aug 20) | No code change — fee eligibility keyed by series ticker, not a modeled field |
| Entry timestamps for FIX market data (Aug 20) | No code change — FIX not implemented in crate |
| Cross-shard subaccount transfers (Aug 20) | Deferred — new `/portfolio/intra_exchange_instance_transfer` endpoint; tracked in `docs/spec-parity.md` |
| Target balance allocation endpoints (Aug 20) | Deferred — new `/portfolio/target_balance_allocation` endpoint family; tracked in `docs/spec-parity.md` |
| Resting order value breakdown by exchange index (Aug 20) | Added `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown: Vec<IndexedBalance>` |
| Exchange index on portfolio and WebSocket fill records (Aug 20) | Added `WsFill::exchange_index`/`WsFillRef::exchange_index` (REST `Fill`/`Settlement`/positions already tolerate unknown fields via `extra`) |
| Exchange index filters for portfolio lists (Aug 20) | Added `exchange_index: Option<u32>` to `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` |
| RFQs and combo-market creation for sub-account-restricted API keys (Aug 20) | No code change — pure authorization/scoping behavior; RFQ/Quote types already carry `subaccount` |
| Optional balance reads by exchange_index (Aug 20) | See Aug 13 `get_balance()` entry — same change |
| Exit triggers on margin positions (Aug 20) | No code change — margin position types not in crate |
| Post-only quotes preserved; crossing rate limits may apply (Aug 22) | No code change — `post_only` already modeled; rate-limit behavior is operational |
| Combo RFQ fee assignment for briefly resting orders (Aug 22) | No code change — fee multiplier already a generic `i64` field |
| Upcoming exchange sharding (Aug 24) | No code change — forward-looking notice, covered by the `exchange_index` work above |
| Localized market content in REST responses (Aug 27) | No code change — pure `Accept-Language` request header; `with_default_headers()` already supports custom headers |
| Trade type on FIX market data (Aug 27) | No code change — FIX not implemented in crate |
| Exchange index on user order messages (Aug 27) | **Breaking** — added required `WsUserOrder::exchange_index: Option<u32>` |
| Cancel-all-orders endpoints (Aug 27) | Added `cancel_all_orders()` (`DELETE /portfolio/events/orders`); Margin half out of scope |
| Historical CF Benchmarks values via the REST passthrough (Aug 27) | No code change — documentation-only clarification of an untyped passthrough path |
| The available_on_brokers field on event responses is deprecated (Aug 27) | No code change — already `Option<bool>`, tolerates always-`false` |
| Exchange auto-routing enabled by default (Aug 27) | No code change — `exchange_index: Option<u32>` already optional on V2 order types |
| Margin maker-volume incentive programs (Aug 27) | No code change — margin types not in crate |
| Structured target images in Trade API v2 (Aug 29) | No code change — `StructuredTarget::details` already a loose `Map<String, Value>` |
| Weather index calibration history (Aug 31) | Deferred — bundled with the Aug 20 weather-index endpoint work; tracked in `docs/spec-parity.md` |
| CF Benchmarks 5Hz value websocket channel (Sep 3) | Deferred — new `cfbenchmarks_value_5hz` channel; tracked in `docs/spec-parity.md` |
| Higher FIX market data session limit (Sep 3) | No code change — FIX not implemented in crate |
| Order identity on FIX market data (Sep 3) | No code change — FIX not implemented in crate |
| Margin fee tier rates (Sep 3) | No code change — margin types not in crate |
| Filter FCM orders by client order IDs (Sep 3) | No code change — `GetFcmOrdersParams` filter shape unaffected by this crate's typed surface |
| Filter historical positions by subaccount (Sep 3) | Covered by the new `get_historical_positions()`/`GetHistoricalPositionsParams::subaccount` (Jul 23 entry) |
| Correct remaining counts after crossing order amendments (Sep 3) | No code change — already `Option<FixedPointCount>`; server-side value-correctness fix only |
| Lower rate-limit cost for cancel all orders (Sep 3) | No code change — operational rate-limit change only |
| Shard rebalance margin reservation (Sep 3) | Deferred — part of the target-balance-allocation feature; tracked in `docs/spec-parity.md` |
| ClearingBusinessDate on FIX trade execution reports (Sep 3) | No code change — FIX not implemented in crate |
| Tapered sub-cent pricing on multivariate (combo) markets (Sep 3) | No code change — `price_ranges`/`price_level_structure`/`*_dollars` fields already modeled generically |
| Per-shard margin order rate limits (Sep 10) | No code change — margin types not in crate |
| The deprecated available_on_brokers field is removed from event responses (Sep 10) | **Breaking** — removed `EventData::available_on_brokers` (field fully gone upstream) |
| Principal-only sizing for target-cost RFQs (Sep 10) | Added `target_cost_excludes_fees: Option<bool>` to `CreateRFQRequest`/`RFQ`/`Quote`; FIX out of scope |
| Margin taker-volume incentive programs (Sep 10) | No code change — margin types not in crate |
| Weather index points expose receipt_basis (Sep 10) | No code change — weather-index endpoints deferred (see Aug 20 entry) |
| Margin markets expose asset_class (Sep 10) | No code change — margin market types not in crate |
| Upcoming exchange sharding for commodities and basketball (Sep 10) | No code change — operational shard assignment, no schema change |
| The center_deci_edge_centi_cent price level structure is emitted again (Sep 10) | No code change — already modeled as opaque `Option<String>` |
| WebSocket schemas corrected to match the messages the service sends (Sep 10) | No code change — `WsError`, `sid`/`seq`, and error codes already generic/optional; `order_source` is Margin-only |
| Margin market important information (Sep 17) | No code change — margin market types not in crate |
| Margin market responses return the configured tick size (Sep 17) | No code change — margin market types not in crate |
| WebSocket schema corrections (Sep 17) | No code change — `WsTicker` dollar-volume fields already signed `i64` |
| FIX EventResendRequest (35=U1) Gated (Sep 17) | No code change — FIX not implemented in crate |
| Historical fills and orders support min_ts (Sep 17) | Added `min_ts: Option<i64>` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| Reduced rate limit cost for QuoteConfirm when providing the RFQ ID. (Sep 17) | No code change — operational rate-limit change only; `confirm_rfq_quote()` already provides the RFQ ID |
| Target balance allocations include their reservation policy (Sep 17) | Deferred — part of the target-balance-allocation feature; tracked in `docs/spec-parity.md` |
| Series responses include a categories list (Sep 17) | Added `Series::categories: Vec<String>` |
| Returning to idiomatic MVE series (Sep 17) | No code change — `mve_collection_ticker` already a plain `Option<String>` |
| WebSocket subscriptions are ready when acknowledged (Sep 17) | No code change — server-side ordering fix, no message shape change |
| RFQ and quote writes share the shard 1 rate-limit budget (Sep 17) | No code change — client-side rate limiter is a simple global token bucket |
| Ed25519 API keys (Sep 24) | Deferred — Ed25519 signing in `auth.rs` needs dedicated design/testing; tracked in `docs/spec-parity.md` |
| 20% higher read and write rate limits (Sep 24) | No code change — named rate-limit tiers aren't hardcoded in the crate |
| Optional WebSocket compression (Sep 24) | Deferred — `tokio-tungstenite` 0.24 has no `permessage-deflate` support; tracked in `docs/spec-parity.md` |
| Rebalancing without resting-order reservation (Sep 24) | Deferred — part of the target-balance-allocation feature; tracked in `docs/spec-parity.md` |
| Subaccount-scoped historical fills and orders (Sep 24) | Added `subaccount: Option<u32>` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| Orders historical cutoff advances independently (Sep 24) | No code change — `GetHistoricalCutoffResponse::orders_updated_ts` already modeled; timing change only |
| ClearingBusinessDate on Margin FIX trade execution reports (Oct 1) | No code change — FIX and Margin not implemented in crate |
| Filter communications RFQs to your own user (Oct 1) | Deferred — new `user_filter` on the WS `communications` subscription; tracked in `docs/spec-parity.md` |
| The deprecated liquidity_dollars field is removed from market responses (Oct 1) | **Breaking** — removed `Market::liquidity_dollars` (field fully gone upstream) |
| RFQ and quote creation timestamps over FIX (Oct 1) | No code change — FIX not implemented in crate |
| Ticker reference_price for Pyth-indexed perps (Oct 1) | No code change — margin ticker types not in crate |

### Breaking

- [Rust API] Removed `Market::liquidity_dollars` and `EventData::available_on_brokers` — both fields are now fully absent upstream (not merely deprecated), so per this repo's refresh policy they were deleted rather than kept as compatibility shims.
- [Rust API] Removed the dead exchange-announcements surface: `KalshiRestClient::get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` (`GET /exchange/announcements` no longer exists upstream).
- [Rust API] Removed the dead multivariate ticker-pair lookup surface: `KalshiRestClient::lookup_tickers_for_market_in_multivariate_event_collection`, `get_multivariate_event_collection_lookup_history`, their request/response types, and the corresponding dead WebSocket surface (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate` / `MultivariateLookup`, `WsDataMessageV2::Multivariate` / `WsDataMessageRef::Multivariate`, `WsMultivariate` / `WsMultivariateRef` / `WsMultivariateSelectedMarket(Ref)`). The `.../lookup` REST path and the `multivariate` WS channel no longer exist upstream (2026-08-06); `multivariate_market_lifecycle` is unaffected.
- [Rust API] `KalshiRestClient::get_balance` now takes a `GetBalanceParams` argument (`subaccount` / `exchange_index` query filters).
- [Rust API] `KalshiRestClient::update_order_group_limit` now takes an additional `SubaccountQueryParams` argument (query-scoped `subaccount` / `exchange_index`, matching `reset_order_group` / `trigger_order_group`).
- [Rust API] `SubaccountBalance::exchange_index: u32` and `WsUserOrder::exchange_index: Option<u32>` are new required-shape fields on structs that previously had no catch-all, so exhaustive external constructors of these structs need updating.

### Added

- [Rust API] Account: `get_account_api_usage_level_volume_progress()` + `GetAccountApiUsageLevelVolumeProgressResponse` / `AccountApiUsageLevelVolumeProgress` / `AccountApiUsageLevelVolumeGoal`; `upgrade_account_api_usage_level()`.
- [Rust API] `ApiKey` / `CreateApiKeyRequest` / `GenerateApiKeyRequest`: `subaccount: Option<u32>`.
- [Rust API] Orders: `cancel_all_orders()` (`DELETE /portfolio/events/orders`).
- [Rust API] Trades/historical: `get_historical_positions()` + `GetHistoricalPositionsParams` (reuses `GetPositionsResponse`); `min_ts` and `subaccount` added to `GetHistoricalFillsParams` / `GetHistoricalOrdersParams`.
- [Rust API] Live data: `get_event_live_data()` + `EventLiveData` / `GetEventLiveDataResponse` / `GetEventLiveDataParams` (event-keyed live data: crypto price charts, commodity timeseries, weather observations).
- [Rust API] Communications: `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote` (RFQ-scoped quote actions, preferred over the now-deprecated quote-ID-only equivalents); `GetQuotesParams::min_ts` / `max_ts`; `target_cost_excludes_fees: Option<bool>` on `CreateRFQRequest` / `RFQ` / `Quote`.
- [Rust API] Events: `EventData::settlement_sources: Option<Vec<SettlementSource>>`; `GetEventsParams::tickers`.
- [Rust API] Series: `categories: Vec<String>`, `exchange_index: Option<u32>`.
- [Rust API] Exchange: `GetExchangeStatusResponse::{intra_exchange_transfers_active, exchange_index_statuses}` + new `ExchangeIndexStatus` struct.
- [Rust API] Portfolio: `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown` + new `IndexedBalance` struct (also used by the new `GetBalanceResponse::balance_breakdown`); `exchange_index: Option<u32>` query filter on `GetOrdersParams` / `GetPositionsParams` / `GetFillsParams`.
- [Rust API] WebSocket lifecycle: top-level `strike_type` / `cap_strike` / `custom_strike` / `price_ranges` (+ new `WsPriceRangeRef`) on `WsMarketLifecycleV2` / `Ref`; `exchange_index` on `WsMarketLifecycleV2` / `Ref` and `WsEventLifecycle` / `Ref`.
- [Rust API] WebSocket trade/fill/order: required `WsTrade::is_block_trade: bool`; `WsFill::exchange_index`; `WsUserOrder::exchange_index`.
- [Rust API] WebSocket communications: `subaccount: Option<u32>` and required `rfq_creator_id` on `WsQuoteCreated`, `WsQuoteAccepted`, `WsQuoteExecuted`.
- [Tests] Added parsing/round-trip coverage for every field/struct above.

### Deprecated

- [Rust API] `GetQuotesParams::market_ticker` / `event_ticker` — removed from the live `GET /communications/quotes` filter set (2026-06-20); the server now silently ignores them. Marked `#[deprecated]` rather than removed since they're harmless no-ops, not a parse hazard.
- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` — superseded by the RFQ-scoped `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`. The quote-ID-only endpoints still work but the live docs deprecate them.

### Removed

- [Rust API] `Market::liquidity_dollars` (deprecated 2026-02, always returned `"0.0000"`, removed upstream 2026-10-01).
- [Rust API] `EventData::available_on_brokers` (deprecated 2026-08-27, removed upstream 2026-09-10).
- [Rust API] `get_exchange_announcements` / `GetExchangeAnnouncementsResponse` / `Announcement` / `AnnouncementType` / `AnnouncementStatus` (`GET /exchange/announcements` removed upstream, no earlier than 2026-07-04).
- [Rust API] `lookup_tickers_for_market_in_multivariate_event_collection`, `get_multivariate_event_collection_lookup_history`, and their request/response types (`.../lookup` REST path removed upstream 2026-08-06).
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` / `MultivariateLookup`, `WsDataMessageV2::Multivariate`, `WsDataMessageRef::Multivariate`, `WsMultivariate`, `WsMultivariateRef`, `WsMultivariateSelectedMarket`, `WsMultivariateSelectedMarketRef` (the `multivariate` WS channel and `multivariate_lookup` message type were removed upstream 2026-08-06; unrelated to `multivariate_market_lifecycle`, which is unaffected).

### Fixed

- [Tests] Fixed a pre-existing compile break in `cargo test --features live-tests`: `tests/rest_auth.rs` asserted on `GetAccountApiLimitsResponse::{read_limit, write_limit}`, which the 0.6.0 rate-limit-tier restructure had already replaced with nested `read`/`write: BucketLimit`.
- [Tests] Fixed a pre-existing compile break in the lib unit tests: the `ws_message_from_bytes_list_subscriptions_accepts_numeric_shard_key` test matched `WsMessageV2::ListSubscriptions { id, subscriptions }` / `WsMessageRef::ListSubscriptions { id, subscriptions }` without the `sid` / `seq` fields the 0.7.0 release added to those variants.

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
