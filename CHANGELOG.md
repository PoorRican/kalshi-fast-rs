# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-26

### Compatibility

- Docs snapshot: 2026-09-26
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-10-01

Per [`VERSIONING.md`](VERSIONING.md): this release removes/renames public Rust
types and changes a public method signature, so it is a **minor** bump
(0.7.0 → 0.8.0), not a patch, even though most of the upstream churn is
additive.

**Changelog entries since the 0.6.0 watermark (2026-06-08) and disposition.**
Kalshi's changelog carries roughly 140 entries in this window; FIX-only and
pure-Margin entries are grouped since this crate implements neither surface
(existing policy — margin market types and FIX are out of scope; the one
exception, `get_margin_fee_tiers`, is unaffected).

| Entries | Action |
|---|---|
| ~20 FIX-only protocol entries (2026-06-11 → 2026-10-01: FIX tags, rejects, rate limits, session limits, order identity, trade type, exchange routing) | No code change — this crate does not implement FIX |
| ~45 Margin-exchange-only entries (fee tiers/rates, margin markets, positions, risk, order groups, exit triggers, incentive programs, perps mark prices/leverage/asset_class/tick_size, `margin_ticker` WS notional fields, per-shard margin rate limits, `is_portfolio`/`subaccount` on margin positions) | No code change — margin market types are out of scope for this crate; `get_margin_fee_tiers` (`/margin/fee_tiers`) is the only margin surface modeled and its shape is unaffected (the new sibling `/margin/fee_tier_rates` endpoint is new margin surface, also out of scope) |
| Operational/behavioral-only entries (rate-limit tier increases, exchange sharding rollouts, fee-timing rollouts, RFQ/quote retention windows, token-cost changes, "exchange auto-routing enabled by default", cancel-all rate-limit cost, crossing-amendment `remaining_count` fix, WebSocket compression, sanity limits, subscription-ready-on-ack race fix, order-group limit increases, API tier qualification halved) | No code change — operational/behavioral only, no Rust type or endpoint shape affected |
| Deprecation notices later finalized in this window (`service` on error responses deprecated 07-28 then removed 08-06; `available_on_brokers` deprecated 08-27, already removed by 09-10; multivariate lookup deprecated pre-watermark, fully removed 08-06) | See Removed/Breaking below — handled at the removal entry, deprecation notices themselves needed no interim change |
| Ed25519 API keys (09-24) | `key_type` surfaced as a passthrough string on `GenerateApiKeyRequest`/`GenerateApiKeyResponse`; RSA-PSS remains the only signing implementation in `auth.rs` (Ed25519 signing is a larger feature, tracked as a known gap in `docs/spec-parity.md`) |
| WebSocket schema-correction entries (09-17, 09-10: nullable fields/enum values, `seq` "documented" on channels that already carried it, `sid`/`seq` "documented" on subscription-scoped errors, retired error codes 6/16/17, `market_id`/`market_ticker` confirmed absent from error schema, ticker `dollar_volume`/`dollar_open_interest` confirmed signed) | No code change — this crate already modeled the corrected (accurate) behavior in every case; verified field-by-field against the live AsyncAPI |
| All other entries | Mapped to a concrete diff below |

### Added

- [Rust API] `exchange_index` field, reflecting Kalshi's exchange-sharding rollout, added to: `MarketPosition`, `Fill` (REST `Fill` and WebSocket `WsFill`/`WsFillRef`), `Settlement`, `Order`, `Series`, `EventData`, `MultivariateEventCollection`, `SubaccountBalance`, `SubaccountTransfer`, `WsUserOrder`, `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref` (present only on `created` events), `WsEventLifecycle`/`WsEventLifecycleRef`, and the zero-copy `MarketPositionRef` REST mirror. Added `exchange_index` filters to `GetPositionsParams`, `GetOrdersParams`, `GetFillsParams`. Added `GetBalanceParams` (new required parameter to `get_balance()`) with `subaccount`/`exchange_index` scoping, and `GetBalanceResponse.balance_breakdown: Option<Vec<IndexedBalance>>`. Added `ApplySubaccountTransferRequest.exchange_index`.
- [Rust API] `GetExchangeStatusResponse.intra_exchange_transfers_active` and `.exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (new `ExchangeIndexStatus` type, including its `description` field added 2026-08-13).
- [Rust API] `Series.categories: Vec<String>` (discovery categories, added 2026-09-17).
- [Rust API] `EventData.settlement_sources: Vec<SettlementSource>` and `EventMetadata.cadence: Option<String>`.
- [Rust API] `GetEventsParams.tickers` filter.
- [Rust API] `GetQuotesParams`/`GetRFQsParams.user_filter`, and `GetQuotesParams.min_ts`/`.max_ts`.
- [Rust API] `CreateRFQRequest.target_cost_excludes_fees`.
- [Rust API] RFQ-scoped quote endpoints, preferred over the now-deprecated quote-ID-only endpoints: `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`.
- [Rust API] `ApiKey.subaccount`/`.fcm_subtrader_id`, `GetApiKeysResponse.api_key_region_expiration_ts`, `CreateApiKeyRequest`/`GenerateApiKeyRequest.subaccount`/`.fcm_subtrader_id`, `GenerateApiKeyRequest.key_type`, `CreateApiKeyResponse.warning`, `GenerateApiKeyResponse.key_type`.
- [Rust API] `GetHistoricalFillsParams`/`GetHistoricalOrdersParams.min_ts`/`.subaccount`. New endpoint `get_historical_positions` (`GetHistoricalPositionsParams`, reuses `GetPositionsResponse`).
- [Rust API] `GetFcmOrdersParams.client_order_ids` filter.
- [Rust API] New target-balance-allocation endpoints `get_target_balance_allocation` / `set_target_balance_allocation`, with `TargetBalanceAllocation`, `RestingMarginReservation`, `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest`.
- [Rust API] New intra-exchange-instance transfer endpoints: `intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfers`, `get_intra_exchange_instance_transfer`, with `IntraExchangeInstanceTransferRequest`/`Response`, `IntraExchangeInstanceTransfer`, `GetIntraExchangeInstanceTransfersParams`/`Response`, `GetIntraExchangeInstanceTransferResponse`.
- [Rust API] New account API-usage-level endpoints: `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level`, with `AccountApiUsageLevelVolumeProgress`, `AccountApiUsageLevelVolumeGoal`, `GetAccountApiUsageLevelVolumeProgressResponse`.
- [Rust API] New endpoint `cancel_all_orders` (`CancelAllOrdersParams`) for `DELETE /portfolio/events/orders`.
- [Rust API] New public live-data endpoints: `get_event_live_data` (`GetEventLiveDataParams`, `EventLiveData`, `GetEventLiveDataResponse`), `get_weather_index` (`GetWeatherIndexParams`, `WeatherIndexPoint`, `WeatherIndexStationReading`, `GetWeatherIndexResponse`), `get_weather_index_calibrations` (`WeatherIndexCalibration`, `WeatherIndexCalibrationStation`, `GetWeatherIndexCalibrationsResponse`).
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Vec<IndexedBalance>`.
- [Rust API] WebSocket: `WsUserOrder.last_update_reason`; `WsTrade`/`WsTradeRef.is_block_trade`; `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref` top-level `price_ranges`, `cap_strike`, `strike_type`, `custom_strike` (present only on `created` / `price_level_structure_updated` / `metadata_updated` events, matching the existing `floor_strike`/`yes_sub_title` treatment); `WsQuoteCreated`/`WsQuoteAccepted` (+ Ref) gained `.subaccount` and the previously-missing required `.rfq_creator_id` field.
- [Rust API] New WebSocket channels `cfbenchmarks_value_5hz` (`WsCfBenchmarksValue5Hz`, `WsCfBenchmarksValue5HzIndexList` + Ref variants) and `pyth_value` (`WsPythValue`, `WsPythUnderlyingList` + Ref variants), following the existing `cfbenchmarks_value` pattern. Added `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` and `underlying_tickers` on `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2`, with validation and subscription-tracker support mirroring the CF Benchmarks index-action handling.
- [Rust API] `WsSubscriptionParamsV2.user_filter` (`communications` channel: `"self"` scopes `rfq_created`/`rfq_deleted` to the caller's own RFQs) and `.use_yes_price` (`orderbook_delta` channel: unifies no-side pricing onto the yes-leg scale).
- [Tests] Added deterministic parsing tests for `pyth_value`, `pyth_value_underlying_list`, `cfbenchmarks_value_5hz`, and `cfbenchmarks_value_5hz_indexlist` (owned and borrowed paths), plus subscription-validation and subscription-tracker tests for the new Pyth underlying actions.

### Changed

- [Rust API] `get_balance()` now takes a `GetBalanceParams` argument instead of no arguments.
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from `String` to `Option<String>`: `GET /fcm/orders` now accepts `client_order_ids` as an alternative filter, and `subtrader_id` is optional for API keys bound to a single FCM subtrader.
- [Rust API] `WsChannelV2::is_private()` now correctly includes `CfbenchmarksValue` (previously miscategorized as public, even though the live AsyncAPI has always required authentication for it), alongside the two new channels.

### Removed

- [Rust API] `Market.liquidity_dollars` (deprecated since Feb 2026, always `"0.0000"`; use `yes_bid_size_fp`/`yes_ask_size_fp`), `Market.response_price_units`, `Market.fractional_trading_enabled`, `EventData.available_on_brokers`, `MarketPosition.resting_orders_count` — all confirmed absent from the live OpenAPI schema and explicitly called out as removed in the changelog.
- [Rust API] `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed from the Predictions REST API (2026-07-04); exchange schedule remains available via `get_exchange_schedule()`.
- [Rust API] `GetQuotesParams.market_ticker`/`.event_ticker` — `GET /communications/quotes` no longer supports these filters (removed 2026-06-20); filter by RFQ, status, or update time instead.
- [Rust API] `ErrorResponse.service` — removed from error response bodies upstream (2026-08-06); branch on `code` instead, which is present on every error response.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate`/`WsMultivariateRef`/`WsMultivariateSelectedMarket`/`WsMultivariateSelectedMarketRef` (the whole `messages::multivariate` module), and the corresponding `WsDataMessageV2`/`WsDataMessageRef`/`WsWireMessage`/`WsWireMessageRef::Multivariate` variants — the `multivariate` WebSocket channel (message type `multivariate_lookup`) was removed upstream (2026-08-06); subscribing to it now returns an unknown-channel error. Use `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes.

### Breaking

- [Rust API] `get_balance()` signature changed; callers must pass `GetBalanceParams::default()` or scope with `subaccount`/`exchange_index`.
- [Rust API] `GetFcmOrdersParams.subtrader_id` is now `Option<String>`; struct-literal construction without `..Default::default()` must wrap the value in `Some(..)`.
- [Rust API] The five removed struct fields above are gone from public structs; downstream code reading them will not compile.
- [Rust API] `get_exchange_announcements()` and its associated types no longer exist.
- [Rust API] `GetQuotesParams` no longer has `market_ticker`/`event_ticker` fields.
- [Rust API] `ErrorResponse` no longer has a `service` field.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, and all `WsMultivariate*` types are gone; downstream exhaustive matches over `WsChannelV2`, `WsMsgType`, `WsDataMessageV2`, or `WsDataMessageRef` must drop these arms. Downstream code subscribing to the `multivariate` channel must migrate to `multivariate_market_lifecycle`.
- [Rust API] `WsUpdateAction` gained `SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`, and `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2` gained `underlying_tickers`/`user_filter`/`use_yes_price` fields; downstream exhaustive matches or struct-literal construction must account for them.

### Fixed

- [Rust API] `WsMessageV2::ListSubscriptions`/`WsMessageRef::ListSubscriptions` match arms in an inline unit test were missing the `sid`/`seq` fields added in 0.7.0, which meant `cargo test` (not just `cargo build`) failed to compile on `master` before this release.
- [Tests] `tests/rest_auth.rs::test_get_account_api_limits` (behind `--features live-tests`) still referenced `resp.read_limit`/`resp.write_limit`, removed by the 0.6.0 `GetAccountApiLimitsResponse` restructuring; updated to `resp.read.refill_rate`/`resp.write.refill_rate`.


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
