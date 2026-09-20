# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-24

### Compatibility

- Docs snapshot: 2026-09-24
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-24

**Changelog entries since the 0.7.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| Orders historical cutoff advances independently (2026-09-24) | Documented on `GetHistoricalCutoffResponse.orders_updated_ts`; added `market_positions_last_updated_ts` |
| Margin market important information (2026-09-17) | No code change — margin market types not in crate |
| Margin market responses return the configured tick size (2026-09-17) | No code change — margin market types not in crate |
| WebSocket schema corrections (2026-09-17) | No code change — `dollar_volume` / `dollar_open_interest` on `WsTicker` are already `i64` (signed) |
| FIX EventResendRequest gated (2026-09-17) | No change needed — FIX not implemented |
| Historical fills and orders support `min_ts` (2026-09-17) | Added `min_ts` to `GetHistoricalFillsParams` and `GetHistoricalOrdersParams` |
| Reduced rate-limit cost for QuoteConfirm with RFQ ID (2026-09-17) | No code change — noted on `confirm_rfq_quote` |
| Target balance allocations include reservation policy (2026-09-17) | Added `resting_margin_reservation` to `GetTargetBalanceAllocationResponse` |
| Series responses include a categories list (2026-09-17) | Added `Series.categories` |
| Returning to idiomatic MVE series (2026-09-17) | No code change — ticker/series naming only |
| WebSocket subscriptions ready when acknowledged (2026-09-17) | No code change — exchange-side race fix |
| RFQ and quote writes share shard 1 budget (2026-09-17) | No code change — rate-limit accounting only |
| Per-shard margin order rate limits (2026-09-10) | Out of scope — Margin + FIX only |
| **`available_on_brokers` removed from event responses (2026-09-10)** | **Breaking** — removed `EventData.available_on_brokers` |
| Principal-only sizing for target-cost RFQs (2026-09-10) | Added `target_cost_excludes_fees` to `CreateRFQRequest` |
| Margin taker-volume incentive programs (2026-09-10) | Added `IncentiveProgram.max_reward_per_account` (shared program shape the crate already models) |
| Weather index points expose `receipt_basis` (2026-09-10) | Added `WeatherIndexPoint.receipt_basis` |
| Margin markets expose `asset_class` (2026-09-10) | No code change — margin market types not in crate |
| Upcoming exchange sharding (2026-09-10) | No code change — operational |
| `center_deci_edge_centi_cent` emitted again (2026-09-10) | No code change — `price_level_structure` is a raw `String` |
| WebSocket schemas corrected (2026-09-10) | No shape change — `sid`/`seq` already surfaced on errors; retired codes 6/16/17 tolerated because `WsError.code` is a plain `i64`. Documented in `docs/spec-parity.md` |
| **CF Benchmarks 5Hz value channel (2026-09-03)** | Added `WsChannelV2::CfbenchmarksValue5Hz`, `WsCfBenchmarksValue5Hz`, and the two new message types |
| Higher FIX market data session limit (2026-09-03) | No change needed — FIX not implemented |
| Order identity on FIX market data (2026-09-03) | No change needed — FIX not implemented |
| Margin fee tier rates (2026-09-03) | No code change — margin-only endpoint, no existing crate surface |
| Filter FCM orders by client order IDs (2026-09-03) | **Breaking** — `GetFcmOrdersParams.subtrader_id` is now `Option<String>`; added `client_order_ids` |
| Filter historical positions by subaccount (2026-09-03) | Added `subaccount` to `GetHistoricalPositionsParams` |
| Correct remaining counts after crossing amendments (2026-09-03) | No code change — exchange-side response fix |
| Lower rate-limit cost for cancel all orders (2026-09-03) | No code change — rate-limit accounting only |
| Shard rebalance margin reservation (2026-09-03) | Added `resting_margin_reservation` to `SetTargetBalanceAllocationRequest` |
| ClearingBusinessDate on FIX execution reports (2026-09-03) | No change needed — FIX not implemented |
| Weather index calibration history (2026-08-31) | Added `get_weather_index_calibrations()` and its response types |
| Structured target images in Trade API v2 (2026-08-29) | No code change — `StructuredTarget.details` is already an untyped JSON map |
| Localized market content via `Accept-Language` (2026-08-27) | No code change — set the header via the transport builder; response shape unchanged |
| Trade type on FIX market data (2026-08-27) | No change needed — FIX not implemented |
| Exchange index on user order messages (2026-08-27) | Added `WsUserOrder.exchange_index` |
| **Cancel-all-orders endpoints (2026-08-27)** | Added `cancel_all_orders()` (`DELETE /portfolio/events/orders`) |
| Historical CF Benchmarks via REST passthrough (2026-08-27) | No code change — documentation of an existing passthrough endpoint |
| `available_on_brokers` deprecated (2026-08-27) | Superseded by the 2026-09-10 removal |
| Exchange auto-routing enabled by default (2026-08-27) | No code change — routing behaviour, no field change |
| VPC peering for Prime members (2026-08-20) | No code change — connectivity/commercial |
| Margin maker-volume incentive programs (2026-08-27) | Covered by `IncentiveProgram.max_reward_per_account` |
| Kalshi Weather Index endpoint (2026-08-20) | Added `get_weather_index()` and its response types |
| Tapered sub-cent pricing on combo markets (2026-09-03) | No code change — no field or message change; documented in `docs/spec-parity.md` |
| Upcoming exchange sharding (2026-08-24) | No code change — operational |
| Post-only quotes preserved (2026-08-22) | No code change — behaviour/fee change only |
| Combo RFQ fee assignment (2026-08-22) | No code change — fee schedule only |
| Maker fee exemption for NFL combos (2026-08-20) | No code change — fee schedule only |
| Entry timestamps for FIX market data (2026-08-20) | No change needed — FIX not implemented |
| Cross-shard subaccount transfers (2026-08-20) | Added `intra_exchange_instance_transfer()` with `source_subaccount` / `destination_subaccount` |
| **Target balance allocation endpoints (2026-08-20)** | Added `get_target_balance_allocation()` / `set_target_balance_allocation()` and their types |
| Resting order value breakdown by exchange index (2026-08-20) | Added `resting_order_value_breakdown` |
| Exchange index on portfolio and WebSocket fill records (2026-08-20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition`, `WsFill` |
| Exchange index filters for portfolio lists (2026-08-20) | Added `exchange_index` to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| RFQs and combo creation for restricted keys (2026-08-20) | No code change — permission scoping only |
| Optional balance reads by `exchange_index` (2026-08-20) | Added `get_balance_scoped()` + `GetBalanceParams` and `balance_breakdown` |
| API key location attestation expiry (2026-08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| Exit triggers on margin positions (2026-08-20) | Out of scope — margin-only endpoints, not in crate |
| New `center_deci_edge_centi_cent` structure (2026-08-13) | No code change — no new fields |
| Balance reads scoped by `exchange_index` (2026-08-13) | Covered by `GetBalanceParams` |
| Block trade indicator for WebSocket trades (2026-08-13) | Added `WsTrade.is_block_trade` |
| Exchange shard descriptions (2026-08-13) | Added `ExchangeIndexStatus.description` |
| Margin order groups bind to single `exchange_index` (2026-08-13) | Out of scope — margin order groups not in crate |
| Order group maximum increased to 100,000 (2026-08-13) | No code change — quota only |
| **Multivariate lookup endpoint and channel removed (2026-08-06)** | **Breaking** — removed the lookup REST methods/types, `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, and `WsMultivariate` |
| Richer combo-validation errors on FIX (2026-08-13) | No change needed — FIX not implemented |
| Intra-account transfer history endpoints (2026-08-13) | Added `get_intra_exchange_instance_transfers()` / `get_intra_exchange_instance_transfer()` and a pager |
| FIX execution reports identify source exchange index (2026-08-06) | No change needed — FIX not implemented |
| Sided leverage estimates on margin markets (2026-08-06) | Out of scope — margin market types not in crate |
| Order group limit updates support subaccounts (2026-08-06) | **Breaking** — `update_order_group_limit()` now takes `SubaccountQueryParams` |
| Multivariate event collections include `exchange_index` (2026-08-06) | Added `MultivariateEventCollection.exchange_index` |
| Richer combo-validation errors on MVE creation (2026-07-30) | No code change — `ErrorResponse.details` already carries the offending tickers |
| **`service` removed from error responses (2026-08-06)** | **Breaking** — removed `ErrorResponse.service` |
| `service` deprecated (2026-07-28) | Superseded by the 2026-08-06 removal |
| Lifecycle creation messages include `exchange_index` (2026-07-30) | Added `exchange_index` to `WsMarketLifecycleV2` and `WsEventLifecycle` |
| Series responses include `exchange_index` (2026-07-30) | Added `Series.exchange_index` |
| **New endpoint for event-keyed live data (2026-07-30)** | Added `get_event_live_data()`, `EventLiveData`, `GetEventLiveDataParams` |
| Restricted keys can read order queue positions (2026-07-30) | No code change — permission scoping only |
| Event `product_metadata` includes `cadence` (2026-07-30) | Added `EventMetadata.cadence` |
| Restricted keys can use batch order endpoints (2026-07-30) | No code change — permission scoping only |
| Subaccount on `quote_created` (2026-07-30) | Added `WsQuoteCreated.subaccount` (and the spec-required `rfq_creator_id`) |
| Restricted keys can manage order groups (2026-07-30) | No code change — permission scoping only |
| Order groups limited to 25,000 per user (2026-07-23) | No code change — quota only (later raised to 100,000) |
| Incentive programs on hidden events excluded (2026-07-22) | No code change — server-side filtering |
| **Historical positions endpoint (2026-07-23)** | Added `get_historical_positions()` + `GetHistoricalPositionsParams`, and the matching cutoff field |
| Restricted keys can open WebSocket sessions (2026-07-23) | No code change — "No new fields are introduced" |
| Restricted keys can quote on RFQ FIX sessions (2026-07-23) | No change needed — FIX not implemented |
| **Pyth value WebSocket channel (2026-07-23)** | Added `WsChannelV2::PythValue`, `WsPythValue`, `WsPythUnderlyingList`, `underlying_tickers`, and three `WsUpdateAction` variants |
| Support for FIX Tag 2446 (2026-07-09) | No change needed — FIX not implemented |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote()`; marked `get_quote()` deprecated in docs |
| **Exchange announcements endpoint removed (2026-07-04)** | **Breaking** — removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| **Deprecated Predictions REST schema fields removed (2026-07-09)** | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| Margin orders identify system order reasons (2026-07-09) | Out of scope — margin orders not in crate |
| New price level structures (2026-07-23) | No code change — no new fields; documented in `docs/spec-parity.md` |
| Multivariate lookup history fully deprecated (2026-07-02) | Superseded by the 2026-08-06 removal |
| Margin positions `is_portfolio` flag (2026-07-02) | Out of scope — margin positions not in crate |
| Trade-scoped API key permissions (2026-06-30) | No code change — scopes are `Vec<String>` |
| `price_ranges` added to `market_lifecycle_v2` (2026-07-02) | Added `WsMarketLifecycleV2.price_ranges` + `WsLifecyclePriceRange` |
| Margin `margin_used` omitted for portfolio positions (2026-06-29) | Out of scope — margin positions not in crate |
| Margin risk per-market metrics limited (2026-06-26) | Out of scope — margin risk not in crate |
| **Per-index exchange status (2026-07-02)** | Added `intra_exchange_transfers_active` and `exchange_index_statuses` + `ExchangeIndexStatus` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index` |
| AcceptQuote rejects carry a specific reason (2026-07-02) | No change needed — FIX not implemented |
| More specific FIX rejects (2026-07-02) | No change needed — FIX not implemented |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `delete_rfq_quote()`, `accept_rfq_quote()`, `confirm_rfq_quote()`; marked the quote-ID-only variants deprecated |
| API usage tier qualification halved (2026-06-25) | No code change — tier thresholds only |
| FIX exchange index routing (2026-06-25) | No change needed — FIX not implemented |
| RFQ quotes support post-only on FIX (2026-06-24) | No change needed — FIX not implemented |
| Get Quote rate-limit cost reduced (2026-06-23) | No code change — rate-limit accounting only |
| **RFQ quote market and event filters removed (2026-06-20)** | **Breaking** — removed `market_ticker` / `event_ticker` from `GetQuotesParams` |
| Communications retention window reduced (2026-06-19) | No code change — retention policy only |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount` / `fcm_subtrader_id` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| `settlement_sources` added to the events API (2026-06-18) | Added `EventData.settlement_sources` |
| Strike type and cap strike on `metadata_updated` (2026-06-18) | Added top-level `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2` |
| RFQ quote identity on FIX (2026-06-18) | No change needed — FIX not implemented |
| Trade entries in FIX market data (2026-06-18) | No change needed — FIX not implemented |
| Legacy order mutation endpoints deprecated (2026-06-18) | No code change — V2 methods already present since 0.6.0; legacy methods retained |
| Event tickers filter on `GET /events` (2026-06-18) | Added `GetEventsParams.tickers` |
| Subaccount on margin positions (2026-06-18) | Out of scope — margin positions not in crate |
| Block-trade accept API key permissions (2026-06-18) | No code change — scopes are `Vec<String>` |
| Sanity limits on orderbook subscriptions (2026-06-18) | No code change — server-side limits |
| Quote time filters and pagination fix (2026-06-18) | Added `min_ts` / `max_ts` to `GetQuotesParams` |
| **API usage volume progress endpoint (2026-06-11)** | Added `get_account_api_usage_level_volume_progress()` and its types |
| Perps mark prices on margin markets (2026-06-11) | Out of scope — margin market types not in crate |
| **Self-serve Advanced API tier upgrade (2026-06-11)** | Added `upgrade_account_api_usage_level()` |
| Margin fee-tier endpoint returns active rates (2026-06-11) | No code change — exchange-side fix |
| Perps volume/OI notional fields (2026-06-11) | Out of scope — margin market types not in crate |
| Tick size on `GET /margin/markets` (2026-06-11) | Out of scope — margin market types not in crate |
| Fractional quantities for RFQs (2026-06-11) | No code change — `contracts_fp` already present |

### Added

- [Rust API] Added `exchange_index` across the shard-aware surface Kalshi rolled out over
  2026-07/08: `Series`, `EventData`, `MultivariateEventCollection`, `SubaccountBalance`, `Fill`,
  `Settlement`, `MarketPosition` (REST and the borrowed `MarketPositionRef`), `Order`, `WsFill`,
  `WsUserOrder`, `WsMarketLifecycleV2`, and `WsEventLifecycle`. All are `Option` because the
  rollout was staged per endpoint. Matching `exchange_index` filters were added to
  `GetOrdersParams`, `GetPositionsParams`, and `GetFillsParams`.
- [Rust API] Added the target balance allocation endpoints: `get_target_balance_allocation()` and
  `set_target_balance_allocation()`, with `TargetBalanceAllocation`,
  `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest` (with `validate()`),
  and the `RestingMarginReservation` enum (`max` | `sum` | `Unknown`).
- [Rust API] Added the intra-exchange transfer surface: `intra_exchange_instance_transfer()`,
  `get_intra_exchange_instance_transfers()`, `get_intra_exchange_instance_transfer()`, an
  `intra_exchange_instance_transfers_pager()`, plus `IntraExchangeInstanceTransfer`,
  `IntraExchangeInstanceTransferStatus`, `IntraExchangeInstanceTransferRequest` (with
  `source_subaccount` / `destination_subaccount`) and their responses.
- [Rust API] Added `get_balance_scoped(GetBalanceParams)` for `exchange_index` / `subaccount`-scoped
  balance reads. `get_balance()` is unchanged and still returns all-index totals. Added
  `GetBalanceResponse.balance_breakdown` and
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`, both
  `Vec<IndexedBalance>`.
- [Rust API] Added `cancel_all_orders(SubaccountQueryParams)` for
  `DELETE /portfolio/events/orders`.
- [Rust API] Added `get_historical_positions()` and `GetHistoricalPositionsParams`, plus
  `GetHistoricalCutoffResponse.market_positions_last_updated_ts`.
- [Rust API] Added `min_ts` to `GetHistoricalFillsParams` and `GetHistoricalOrdersParams`.
- [Rust API] Added the Kalshi Weather Index endpoints: `get_weather_index()` and
  `get_weather_index_calibrations()`, with `GetWeatherIndexParams`, `WeatherIndexPoint`,
  `WeatherIndexStationReading`, `WeatherIndexCalibration`, `WeatherIndexCalibrationStation` and
  their responses. `WeatherIndexPoint.v` is `Option<f64>` — `incomplete` points have no value at
  all, not `0`.
- [Rust API] Added `get_event_live_data()` with `EventLiveData`, `GetEventLiveDataParams`, and
  `GetEventLiveDataResponse` for `GET /live_data/events/{event_ticker}`.
- [Rust API] Added the account API usage-tier endpoints:
  `get_account_api_usage_level_volume_progress()` and `upgrade_account_api_usage_level()`, with
  `AccountApiUsageLevelVolumeProgress` and `AccountApiUsageLevelVolumeGoal`.
- [Rust API] Added the RFQ-scoped quote endpoints `get_rfq_quote()`, `delete_rfq_quote()`,
  `accept_rfq_quote()`, and `confirm_rfq_quote()`. The quote-ID-only variants still work but are
  documented as deprecated upstream.
- [Rust API] Added `CreateRFQRequest.target_cost_excludes_fees` for principal-only target-cost
  sizing, and `min_ts` / `max_ts` / `user_filter` to `GetQuotesParams`.
- [Rust API] Added `WsChannelV2::CfbenchmarksValue5Hz` and `WsChannelV2::PythValue`, with
  `WsCfBenchmarksValue5Hz`, `WsPythValue`, `WsPythUnderlyingList` (plus borrowed forms) and the
  `cfbenchmarks_value_5hz`, `cfbenchmarks_value_5hz_indexlist`, `pyth_value`, and
  `pyth_value_underlying_list` message types routed through both the wire and envelope parse paths.
- [Rust API] Added `underlying_tickers` to `WsSubscriptionParamsV2` and
  `WsUpdateSubscriptionParamsV2`, and `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList`. The subscription tracker folds underlying
  add/remove updates into the resubscribe state, and `validate_update` enforces the AsyncAPI rules
  (no market targets, `underlying_tickers` required for add/remove).
- [Rust API] Added `Series.categories`, `EventData.settlement_sources`,
  `EventData.fee_type_override` / `fee_multiplier_override`, `EventMetadata.cadence`,
  `GetEventsParams.tickers`, `ExchangeIndexStatus` (with `description`),
  `GetExchangeStatusResponse.intra_exchange_transfers_active` / `exchange_index_statuses`,
  `GetApiKeysResponse.api_key_region_expiration_ts`, `ApiKey.subaccount` /
  `ApiKey.fcm_subtrader_id`, `subaccount` / `fcm_subtrader_id` on the API-key create/generate
  requests, `IncentiveProgram.incentive_description` / `max_reward_per_account`,
  `GetFcmOrdersParams.client_order_ids`, `WsTrade.is_block_trade`, `WsQuoteCreated.subaccount` /
  `rfq_creator_id`, `WsMarketLifecycleV2.price_ranges` (with `WsLifecyclePriceRange`), and
  top-level `strike_type` / `cap_strike` / `custom_strike` on `WsMarketLifecycleV2`.
- [Tests] Added deterministic coverage for every touched shape: new WebSocket channel routing
  (owned and borrowed), `is_block_trade`, `exchange_index` on fills/orders, lifecycle
  `price_ranges` and `metadata_updated` strike fields, `quote_created.subaccount`, Pyth
  subscription validation, the weather index `incomplete` / `receipt_basis` cases, target balance
  allocation round-tripping, intra-exchange transfers, and tolerance of every removed field.

### Changed

- [Rust API] `update_order_group_limit()` now takes a `SubaccountQueryParams` argument so
  subaccount-scoped limit updates are reachable.
- [Rust API] `GetFcmOrdersParams.subtrader_id` is now `Option<String>`; `GET /fcm/orders` requires
  at least one of `subtrader_id` or `client_order_ids`.
- [Docs] `docs/spec-parity.md` gained durable notes on the shard rollout, the deprecated vs
  RFQ-scoped quote endpoints, price-level-structure churn, `metadata_updated` top-level strikes,
  the weather index `incomplete` / `receipt_basis` semantics, the two new streaming channels,
  retired WebSocket error codes, and `cancel_all_orders` subaccount semantics.

### Deprecated

- [Upstream] The quote-ID-only communications endpoints (`get_quote`, `delete_quote`,
  `accept_quote`, `confirm_quote`) are deprecated upstream (2026-06-25 / 2026-07-09). They are
  retained and documented; prefer the `*_rfq_quote` variants, which Kalshi expects to become
  mandatory and which cost fewer rate-limit tokens on confirm.
- [Upstream] The legacy `/portfolio/orders` mutation endpoints are deprecated upstream
  (2026-06-18). The V2 `*_v2` methods added in 0.6.0 remain the preferred path.

### Removed

- [Rust API] Removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, and `AnnouncementStatus`
  (`GET /exchange/announcements` removed upstream 2026-07-04).
- [Rust API] Removed `ErrorResponse.service` (removed upstream 2026-08-06).
- [Rust API] Removed `EventData.available_on_brokers` (removed upstream 2026-09-10).
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` (removed upstream 2026-07-09), along with
  `WsMarketLifecycleV2.fractional_trading_enabled` and
  `WsMarketLifecycleEventType::FractionalTradingUpdated`, which are absent from the current
  AsyncAPI.
- [Rust API] Removed the multivariate lookup surface (removed upstream 2026-08-06):
  `get_multivariate_event_collection_lookup_history()`,
  `lookup_tickers_for_market_in_multivariate_event_collection()`, their request/response types and
  `LookupPoint`, plus `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`,
  `WsMsgType::MultivariateLookup`, `WsDataMessageV2::Multivariate` /
  `WsDataMessageRef::Multivariate`, and `WsMultivariate` / `WsMultivariateRef` /
  `WsMultivariateSelectedMarket`.
- [Rust API] Removed `market_ticker` and `event_ticker` from `GetQuotesParams` (removed upstream
  2026-06-20, effective immediately).
- [Rust API] Removed `IncentiveProgram.target_size`; the current OpenAPI exposes only
  `target_size_fp`.

### Fixed

- [Rust API] `WsError.message` / `WsErrorRef.message` now actually populate. The AsyncAPI names the
  human-readable error text `msg` inside the error object, but the field was only bound to
  `message`, so it silently deserialized as `None` against real exchange traffic on every
  WebSocket error frame. `msg` is now accepted as a serde alias on both the owned and borrowed
  paths, and `message` keeps working.
- [Tests] `cargo test --all-targets` compiles again. `tests`-only code in
  `src/ws/types/envelope.rs` still destructured `WsMessageV2::ListSubscriptions` /
  `WsMessageRef::ListSubscriptions` without the `sid` / `seq` fields added in 0.7.0, which broke
  the `lib test` target on this branch before this refresh.
- [Tests] `cargo check --all-targets --features live-tests` compiles again. The live suites had
  drifted since 0.6.0: `tests/rest_auth.rs` still read the flat `read_limit` / `write_limit`
  fields replaced by `BucketLimit` in 0.6.0, and `tests/rest_public.rs` exercised the
  `/exchange/announcements` and multivariate-lookup endpoints removed upstream.

### Breaking

- [Rust API] `ErrorResponse.service` is removed. Branch on `code`, which is present on every error
  response; `details` carries the offending market tickers on combo-validation failures.
- [Rust API] `EventData.available_on_brokers` is removed. It returned `false` unconditionally from
  August 2026 onward.
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` are removed. Use `Market.price_level_structure` /
  `Market.price_ranges` and the fixed-point count and dollar fields.
- [Rust API] `WsMarketLifecycleEventType` lost its `FractionalTradingUpdated` variant and
  `WsMarketLifecycleV2` lost `fractional_trading_enabled`. Exhaustive matches over the event-type
  enum must drop that arm.
- [Rust API] `WsChannelV2` lost `Multivariate`, `WsMsgType` lost `Multivariate` and
  `MultivariateLookup`, and `WsDataMessageV2` / `WsDataMessageRef` lost their `Multivariate`
  variants. Exhaustive matches must drop those arms; a stray `multivariate_lookup` frame now
  surfaces as `WsMessageV2::Unknown`.
- [Rust API] `WsChannelV2` gained `CfbenchmarksValue5Hz` and `PythValue`, `WsMsgType` gained four
  variants, and `WsDataMessageV2` / `WsDataMessageRef` gained four variants each. Exhaustive
  matches must handle them (or use `_`).
- [Rust API] `WsUpdateAction` gained `SubscribeUnderlyings`, `UnsubscribeUnderlyings`, and
  `UnderlyingList`, and both `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2` gained an
  `underlying_tickers` field. `WsUpdateSubscriptionParamsV2` has no `Default`, so struct-literal
  construction must add the field.
- [Rust API] `WsChannelV2::is_private()` now returns `true` for `CfbenchmarksValue`,
  `CfbenchmarksValue5Hz`, and `PythValue`; all three require an authenticated session per the
  AsyncAPI.
- [Rust API] `GetQuotesParams` lost `market_ticker` and `event_ticker`. Filter by `rfq_id`,
  `status`, `user_filter` / `rfq_user_filter`, or the new `min_ts` / `max_ts` window.
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from `String` to `Option<String>`, and
  `CreateRFQRequest` gained `target_cost_excludes_fees`. Struct-literal construction of either
  must be updated.
- [Rust API] `update_order_group_limit()` takes a new `SubaccountQueryParams` second argument.
- [Rust API] `get_exchange_announcements()` and the multivariate lookup methods are removed; there
  is no upstream endpoint to call.
- [Rust API] `IncentiveProgram.target_size` is removed; read `target_size_fp`.
- [Rust API] Several structs without `Default` gained fields and so break struct-literal
  construction: `Fill`, `Settlement`, `MarketPosition`, `Order`, `Series`, `EventData`,
  `IncentiveProgram`, `GetBalanceResponse`, `GetPortfolioRestingOrderTotalValueResponse`,
  `GetExchangeStatusResponse`, `GetApiKeysResponse`, `WsTrade`, `WsFill`, `WsUserOrder`,
  `WsMarketLifecycleV2`, `WsEventLifecycle`, and `WsQuoteCreated`. Deserialization is unaffected.


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
