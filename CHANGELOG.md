# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-10

### Compatibility

- Docs snapshot: 2026-09-10
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-10

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition.** Margin-only entries
(`/margin/*` fields the crate doesn't model beyond `/margin/fee_tiers`) and FIX-only entries (the
crate is REST + WebSocket only) are grouped; every other entry gets its own row.

| Entry | Action |
|---|---|
| API usage volume progress endpoint (2026-06-11) | Added `get_account_api_usage_level_volume_progress`, `AccountApiUsageLevelVolumeProgress`/`Goal` |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Added `upgrade_account_api_usage_level` |
| Fractional contract quantities for RFQs (2026-06-11) | No code change — `contracts_fp`/`yes_contracts_fp`/`no_contracts_fp` already present |
| `settlement_sources` added to the events API (2026-06-18) | Added `EventData.settlement_sources` |
| Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` (2026-06-18) | Added top-level `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| Legacy order mutation endpoints deprecated (2026-06-18) | No code change — endpoints not yet removed from spec; still modeled |
| Event tickers filter on `GET /events` (2026-06-18) | Added `GetEventsParams.tickers`; also added `min_updated_ts` seen live in spec |
| Subaccount on margin positions; sanity limits on orderbook subscriptions; block-trade-accept API key scopes (2026-06-18) | No code change — Margin, operational, or scopes already `Vec<String>` |
| RFQ quote identity on FIX; trade entries in FIX market data (2026-06-18) | No code change — FIX |
| Quote time filters and pagination fix (2026-06-18) | Added `GetQuotesParams.min_ts`/`max_ts`; pagination fix is server-side |
| Communications RFQ/quote retention window reduced (2026-06-19) | No code change — operational |
| RFQ quote market and event filters removed (2026-06-20) | **Removed** `GetQuotesParams.market_ticker`/`event_ticker` |
| Get Quote rate-limit cost reduced (2026-06-23) | No code change — operational |
| RFQ quotes support post-only on FIX (2026-06-24) | No code change — FIX |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`; deprecated the quote-ID-only equivalents |
| API usage tier qualification halved; FIX exchange index routing (2026-06-25) | No code change — operational / FIX |
| Margin risk per-market metrics limited; margin positions `margin_used` omitted (2026-06-26, 06-29) | No code change — Margin |
| Trade-scoped API key permissions (2026-06-30) | No code change — scopes already `Vec<String>` |
| Multivariate lookup history endpoints fully deprecated (2026-07-02) | Deprecated (removed 2026-08-06, see below) |
| Margin positions `is_portfolio` flag (2026-07-02) | No code change — Margin |
| `price_ranges` added to `market_lifecycle_v2` events (2026-07-02) | Added `WsMarketLifecycleV2.price_ranges` |
| Per-index exchange status (2026-07-02) | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses`, `ExchangeIndexStatus` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index` |
| AcceptQuote FIX rejects; more specific FIX cancel/replace rejects (2026-07-02) | No code change — FIX |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount`/`fcm_subtrader_id` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| Exchange announcements endpoint removed (2026-07-04) | **Removed** `get_exchange_announcements`, `Announcement*`, `GetExchangeAnnouncementsResponse` |
| FIX Tag 2446 on Incremental Refresh (2026-07-09) | No code change — FIX |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote`; deprecated `get_quote` |
| Deprecated Predictions REST schema fields removed (2026-07-09) | **Removed** `Market.response_price_units`/`fractional_trading_enabled`, `MarketPosition.resting_orders_count`; also removed from `WsMarketLifecycleV2` (`fractional_trading_enabled`, `FractionalTradingUpdated`), confirmed gone from AsyncAPI |
| Margin orders identify system order reasons (2026-07-09) | No code change — Margin |
| Incentive programs on hidden events excluded (2026-07-22) | No code change — behavior only |
| Order groups limited to 25,000/user (2026-07-23) | No code change — server-enforced limit, not modeled |
| Historical positions endpoint (2026-07-23) | Added `get_historical_positions`, `GetHistoricalPositionsParams`, `GetHistoricalCutoffResponse.market_positions_last_updated_ts` |
| Subaccount-restricted API keys can open WebSocket sessions (2026-07-23) | No code change — private channels already scoped server-side |
| Subaccount-restricted keys can quote on RFQ FIX sessions (2026-07-23) | No code change — FIX |
| Pyth value WebSocket channel (2026-07-23) | Added `pyth_value` channel: `WsPythValue`, `WsPythUnderlyingList`, `underlying_tickers` subscription field, `SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` actions |
| New price level structures ×7 (2026-07-23) | No code change — `price_level_structure` already an untyped `String` |
| `service` field on error responses deprecated (2026-07-28) | No code change yet — already `Option<String>` (removed below, 2026-08-06) |
| Richer combo-validation errors on multivariate market creation (2026-07-30) | No code change — `ErrorResponse.details` already `String` |
| Lifecycle creation messages include `exchange_index` (2026-07-30) | Added `WsMarketLifecycleV2.exchange_index`, `WsEventLifecycle.exchange_index` |
| Series responses include `exchange_index` (2026-07-30) | Added `Series.exchange_index` |
| New endpoint for event-keyed live data (2026-07-30) | Added `get_event_live_data`, `EventLiveData`, `GetEventLiveDataParams` |
| Subaccount-restricted keys: queue positions, batch orders, order groups (2026-07-30) | No code change — behavior only |
| Event `product_metadata` includes `cadence` (2026-07-30) | Added `EventMetadata.cadence` |
| Subaccount on `quote_created` (2026-07-30) | Added `WsQuoteCreated.subaccount` (and `rfq_creator_id`, seen live in spec) |
| Multivariate lookup endpoint and channel removed (2026-08-06) | **Removed** `lookup_tickers_for_market_in_multivariate_event_collection`, `get_multivariate_event_collection_lookup_history`, and the WS `multivariate` channel / `WsMultivariate*` types |
| FIX execution reports identify source exchange index (2026-08-06) | No code change — FIX |
| Sided leverage estimates on margin markets (2026-08-06) | No code change — Margin |
| Order group limit updates support subaccounts (2026-08-06) | Added `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) to `update_order_group_limit` |
| Multivariate event collections include `exchange_index` (2026-08-06) | Added `MultivariateEventCollection.exchange_index` |
| The `service` field removed from error responses (2026-08-06) | **Removed** `ErrorResponse.service` |
| New `center_deci_edge_centi_cent` price level structure (2026-08-13) | No code change — untyped `String` |
| Balance reads scoped by `exchange_index` (2026-08-13, revised 2026-08-20) | Added `GetBalanceParams` (`subaccount`, `exchange_index`) to `get_balance`; final shape read directly from current spec |
| Block trade indicator for WebSocket trades (2026-08-13) | Added `WsTrade.is_block_trade` |
| Exchange shard descriptions (2026-08-13) | Added `ExchangeIndexStatus.description` |
| Margin order groups bind to single `exchange_index`; order group max raised to 100,000; richer FIX combo errors (2026-08-13) | No code change — Margin, operational limit, or FIX |
| Intra-account transfer history endpoints (2026-08-13) | Added `create_intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfers[_pager]`, `get_intra_exchange_instance_transfer` |
| API key location attestation expiry (2026-08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| VPC peering for Prime members (2026-08-20) | No code change — connectivity/docs only |
| Kalshi Weather Index endpoint (2026-08-20) | Added `get_weather_index`, `GetWeatherIndexResponse`, `WeatherIndexPoint`, `WeatherIndexStationReading` |
| Maker fee exemption for independent NFL combo markets (2026-08-20) | No code change — business rule, not modeled |
| Entry timestamps for FIX market data (2026-08-20) | No code change — FIX |
| Cross-shard subaccount transfers (2026-08-20) | Added `source_subaccount`/`destination_subaccount` to `IntraExchangeInstanceTransferRequest` |
| Target balance allocation endpoints (2026-08-20) | Added `get_target_balance_allocation`, `set_target_balance_allocation`, `TargetBalanceAllocation*` |
| Resting order value breakdown by exchange index (2026-08-20) | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`, `IndexedBalance` |
| Exchange index on portfolio and WebSocket fill records (2026-08-20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition`, `WsFill` |
| Exchange index filters for portfolio lists (2026-08-20) | Added `exchange_index` to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| RFQs/combo-market creation for sub-account-restricted keys (2026-08-20) | No code change — behavior only |
| Exit triggers on margin positions (2026-08-20) | No code change — Margin |
| Post-only quotes preserved; combo RFQ fee assignment; upcoming sharding (2026-08-22, 08-24) | No code change — business rules / docs |
| Localized market content in REST responses (2026-08-27) | No code change — `Accept-Language` request header, no response shape change |
| Trade type on FIX market data (2026-08-27) | No code change — FIX |
| Exchange index on user order messages (2026-08-27) | Added `WsUserOrder.exchange_index` |
| Cancel-all-orders endpoints (2026-08-27) | Added `cancel_all_orders_v2` (`DELETE /portfolio/events/orders`) |
| Historical CF Benchmarks via REST passthrough; `available_on_brokers` deprecated; exchange auto-routing default (2026-08-27) | No code change — docs-only, already `Option<bool>`, or behavior only |
| Margin maker-volume incentive programs (2026-08-27) | No code change — `incentive_type` already an untyped `String` |
| Structured target images in Trade API v2 (2026-08-29) | No code change — `StructuredTarget.details` already an untyped `Map` |
| Weather index calibration history (2026-08-31) | Added `get_weather_index_calibrations`, `WeatherIndexCalibration`, `WeatherIndexCalibrationStation` |
| CF Benchmarks 5Hz value websocket channel (2026-09-03) | Added `cfbenchmarks_value_5hz` channel: `WsCfBenchmarksValue5Hz`, reuses `WsCfBenchmarksIndexList` and the existing index-action mechanic |
| Higher FIX market data session limit; order identity on FIX; ClearingBusinessDate (2026-09-03) | No code change — FIX |
| Margin fee tier rates (2026-09-03) | No code change — Margin, account-level endpoint out of crate scope |
| Filter FCM orders by client order IDs (2026-09-03) | Added `GetFcmOrdersParams.client_order_ids`; `subtrader_id` is now `Option<String>` |
| Filter historical positions by subaccount (2026-09-03) | Covered by `GetHistoricalPositionsParams.subaccount` (added above) |
| Correct remaining counts after crossing amendments; lower cancel-all rate-limit cost (2026-09-03) | No code change — bug fix / operational |
| Shard rebalance margin reservation (2026-09-03) | Added `SetTargetBalanceAllocationRequest.resting_margin_reservation`, `RestingMarginReservation` |
| Tapered sub-cent pricing on multivariate markets (2026-09-03) | No code change — `price_level_structure` already an untyped `String` |
| Margin markets expose `asset_class`; upcoming sharding for commodities/basketball (2026-09-10) | No code change — Margin / docs |

### Added

- [Rust API] `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level` on
  `KalshiRestClient`, plus `AccountApiUsageLevelVolumeProgress`/`AccountApiUsageLevelVolumeGoal`/
  `GetAccountApiUsageLevelVolumeProgressResponse`.
- [Rust API] `EventData.settlement_sources`, `EventMetadata.cadence`, `GetEventsParams.tickers`/
  `min_updated_ts`, `Series.exchange_index`.
- [Rust API] `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref` gained top-level `exchange_index`,
  `price_ranges`, `strike_type`, `cap_strike`, `custom_strike`. `WsEventLifecycle`/`Ref` gained
  `exchange_index`.
- [Rust API] `GetQuotesParams.min_ts`/`max_ts`. New RFQ-scoped quote endpoints:
  `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`.
- [Rust API] `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses`
  plus `ExchangeIndexStatus`.
- [Rust API] `ApiKey`/`CreateApiKeyRequest`/`GenerateApiKeyRequest` gained `subaccount`/
  `fcm_subtrader_id`; `GetApiKeysResponse.api_key_region_expiration_ts`; `CreateSubaccountRequest`
  (optional `exchange_index`) is now required by `create_subaccount`.
- [Rust API] `get_historical_positions` (`GET /historical/positions`), `GetHistoricalPositionsParams`,
  `GetHistoricalCutoffResponse.market_positions_last_updated_ts`.
- [Rust API] `pyth_value` WebSocket channel: `WsChannelV2::PythValue`, `WsPythValue`/`Ref`,
  `WsPythUnderlyingList`/`Ref`, `WsSubscriptionParamsV2.underlying_tickers`,
  `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`,
  `WsUpdateSubscriptionParamsV2.underlying_tickers`.
- [Rust API] `cfbenchmarks_value_5hz` WebSocket channel: `WsChannelV2::CfbenchmarksValue5hz`,
  `WsCfBenchmarksValue5Hz`/`Ref` (reuses `WsCfBenchmarksIndexList` for its indexlist response and the
  existing `SubscribeIndices`/`UnsubscribeIndices`/`Indexlist` actions).
- [Rust API] `MultivariateEventCollection.exchange_index`.
- [Rust API] `get_event_live_data` (`GET /live_data/events/{event_ticker}`), `EventLiveData`,
  `GetEventLiveDataParams`.
- [Rust API] `get_weather_index` / `get_weather_index_calibrations`, `GetWeatherIndexResponse`,
  `WeatherIndexPoint`, `WeatherIndexStationReading`, `GetWeatherIndexCalibrationsResponse`,
  `WeatherIndexCalibration`, `WeatherIndexCalibrationStation`.
- [Rust API] `create_intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfers`
  (+ `_pager`/`stream_`), `get_intra_exchange_instance_transfer`, `IntraExchangeInstanceTransfer*`;
  `IntraExchangeInstanceTransferRequest` gained `source_subaccount`/`destination_subaccount`.
- [Rust API] `get_target_balance_allocation`, `set_target_balance_allocation`,
  `TargetBalanceAllocation`/`TargetBalanceAllocationInput`, `RestingMarginReservation`,
  `SetTargetBalanceAllocationRequest.resting_margin_reservation`.
- [Rust API] `GetBalanceParams` (`subaccount`, `exchange_index`) is now required by `get_balance`;
  `GetBalanceResponse.balance_breakdown`, `IndexedBalance`.
- [Rust API] `exchange_index` added to `Fill`, `Settlement`, `MarketPosition`, `WsFill`,
  `WsUserOrder`; `exchange_index` filter added to `GetOrdersParams`, `GetPositionsParams`,
  `GetFillsParams`.
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`.
- [Rust API] `cancel_all_orders_v2` (`DELETE /portfolio/events/orders`).
- [Rust API] `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) is now required by
  `update_order_group_limit`.
- [Rust API] `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` gained `subaccount`;
  `WsQuoteCreated`/`WsQuoteAccepted` gained `rfq_creator_id`. `WsTrade.is_block_trade`.
- [Rust API] `GetFcmOrdersParams.client_order_ids`.
- [Rust API] `Quote`/`RFQ` gained `post_only`/`creator_subaccount`/`rfq_creator_subaccount` fields
  seen live in the current spec.

### Deprecated

- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` are `#[deprecated]` in
  favor of the RFQ-scoped equivalents (`get_rfq_quote`, etc.); per the 2026-06-25 changelog entry, a
  quote is no longer guaranteed queryable by ID alone unless in a post-acceptance state.

### Removed

- [Rust API] `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed upstream.
- [Rust API] `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types — the
  multivariate lookup endpoint was removed upstream. The WebSocket `multivariate` channel
  (`WsChannelV2::Multivariate`, `WsMultivariate`/`Ref`, `WsMultivariateSelectedMarket`/`Ref`) was
  removed for the same reason.
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` — removed upstream. `WsMarketLifecycleV2.
  fractional_trading_enabled` and `WsMarketLifecycleEventType::FractionalTradingUpdated` were also
  removed (confirmed gone from the current AsyncAPI schema).
- [Rust API] `ErrorResponse.service` — removed from every REST error response upstream; branch on
  `code` instead.
- [Rust API] `GetQuotesParams.market_ticker`/`event_ticker` — `GET /communications/quotes` no longer
  accepts these filters upstream.
- [Rust API] `GetFillsParams.event_ticker` — `GET /portfolio/fills` has never accepted this query
  parameter; it was a stale, ineffective field on the client type.
- [Rust API] Dead, incorrectly-shaped `ws::types::MarketPositionRef`/`EventPositionRef` (mirrored the
  REST `MarketPosition` shape rather than any real WebSocket payload; unused anywhere in the crate).
  `ws::types::WsMarketPositionRef` — already correct — is unaffected.

### Fixed

- [Rust API] `GetPositionsParams.event_ticker` changed from `Option<Vec<String>>` to `Option<String>`:
  `GET /portfolio/positions` has only ever accepted a single event ticker.
- [Rust API] The client-side `subaccount` upper-bound validation in `GetPositionsParams`,
  `GetOrdersParams`, and `CreateOrderRequest` was corrected from 32 to 63, matching the documented
  0–63 subaccount range; the old bound rejected valid subaccounts 33–63.
- [Tests] `tests/parsing.rs` had two latent compile bugs unrelated to this refresh — a
  `WsMessageV2::ListSubscriptions`/`WsMessageRef::ListSubscriptions` match missing the `sid`/`seq`
  fields added in 0.7.0, and `GetAccountApiLimitsResponse` field access using the flat
  `read_limit`/`write_limit` shape replaced by nested `BucketLimit`s in 0.6.0 — both only surfaced
  when building with `--features live-tests`. Fixed alongside this refresh.

### Breaking

- [Rust API] `get_balance()` now takes `GetBalanceParams`; `create_subaccount()` now takes
  `CreateSubaccountRequest`; `update_order_group_limit()` now takes an additional
  `UpdateOrderGroupLimitParams` argument.
- [Rust API] `GetQuotesParams` lost `market_ticker`/`event_ticker`. `GetFillsParams` lost
  `event_ticker`. `GetPositionsParams.event_ticker` changed type from `Option<Vec<String>>` to
  `Option<String>`.
- [Rust API] `ErrorResponse` lost `service`. `Market` lost `response_price_units` and
  `fractional_trading_enabled`. `MarketPosition` lost `resting_orders_count`.
  `WsMarketLifecycleV2`/`Ref` lost `fractional_trading_enabled`; `WsMarketLifecycleEventType` lost
  `FractionalTradingUpdated`.
- [Rust API] Removed `get_exchange_announcements`, `Announcement*`, `GetExchangeAnnouncementsResponse`,
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types, plus
  `WsChannelV2::Multivariate`, `WsMultivariate*`, and `ws::types::MarketPositionRef`/
  `EventPositionRef` (see Removed above for the full list and rationale).
- [Rust API] `Fill`, `Settlement`, `MarketPosition`, `WsFill`, and `SubaccountBalance` each gained a
  new required (non-`Option`) `exchange_index` field, so exhaustive struct literals must supply it.


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
