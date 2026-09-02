# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).

## [0.8.0] - 2026-09-02

### Compatibility

- Docs snapshot: 2026-09-02
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-03

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| Higher FIX market data session limit (2026-09-03) | No code change — FIX API not implemented in crate |
| Order identity on FIX market data (2026-09-03) | No code change — FIX API not implemented in crate |
| Public margin fee tier rates (2026-09-03) | No code change — margin market types beyond `/margin/fee_tiers` not in crate scope |
| Filter historical positions by subaccount (2026-09-03) | Added `get_historical_positions` / `GetHistoricalPositionsParams` (also covers "Historical positions endpoint" below) |
| Correct remaining counts after crossing order amendments (2026-09-03) | No code change — exchange-side value-correctness fix only |
| Lower rate-limit cost for cancel all orders (2026-09-03) | No code change — operational rate-limit only |
| Shard rebalance margin reservation (2026-09-03) | Added `resting_margin_reservation` / `RestingMarginReservation` to `SetTargetBalanceAllocationRequest` |
| ClearingBusinessDate on FIX trade execution reports (2026-09-03) | No code change — FIX API not implemented in crate |
| Weather index calibration history (2026-08-31) | Added `get_weather_index_calibrations` and calibration types |
| Structured target images in Trade API v2 (2026-08-29) | No code change — `StructuredTarget.details` is an untyped map that already carries `image_url` |
| Localized market content in REST responses (2026-08-27) | No code change — opt-in via `Accept-Language` request header, no response shape change |
| Trade type on FIX market data (2026-08-27) | No code change — FIX API not implemented in crate |
| Exchange index on user order messages (2026-08-27) | Added `exchange_index` to `WsUserOrder` |
| Cancel-all-orders endpoints (2026-08-27) | Added `cancel_all_orders` / `CancelAllOrdersParams` for `DELETE /portfolio/events/orders` |
| Historical CF Benchmarks values via the REST passthrough (2026-08-27) | No code change — generic CF Benchmarks passthrough endpoint not modeled by crate |
| The `available_on_brokers` field on event responses is deprecated (2026-08-27) | Documented deprecation on `EventData.available_on_brokers` (already `Option<bool>`) |
| Exchange auto-routing enabled by default (2026-08-27) | No code change — behavior default; covered by existing `exchange_index`/`market_ticker` auto-routing fields |
| VPC peering for Prime members (2026-08-20) | No code change — informational/infrastructure |
| Margin maker-volume incentive programs (2026-08-27) | Added `IncentiveProgram.max_reward_per_account` and `incentive_description` (+ params filter) |
| Kalshi Weather Index endpoint (2026-08-20) | Added `get_weather_index` and weather index types |
| Tapered sub-cent pricing on multivariate markets (2026-09-03) | No code change — `price_level_structure` already modeled as raw `String`; prices already read from `*_dollars` fields |
| Upcoming exchange sharding (2026-08-24) | No code change — informational; addressed by `exchange_index` additions throughout this release |
| Post-only quotes preserved; crossing rate limits may apply (2026-08-22) | No code change — informational, no field changes |
| Combo RFQ fee assignment for briefly resting orders (2026-08-22) | No code change — fee mechanics, not modeled by crate |
| Maker fee exemption for independent NFL combo markets (2026-08-20) | No code change — fee mechanics, not modeled by crate |
| Entry timestamps for FIX market data (2026-08-20) | No code change — FIX API not implemented in crate |
| Cross-shard subaccount transfers (2026-08-20) | Added `source_subaccount` / `destination_subaccount` to `IntraExchangeInstanceTransferRequest` |
| Target balance allocation endpoints (2026-08-20) | Added `get_target_balance_allocation` / `set_target_balance_allocation` and types |
| Resting order value breakdown by exchange index (2026-08-20) | Added `resting_order_value_breakdown: Vec<IndexedBalance>` to `GetPortfolioRestingOrderTotalValueResponse` |
| Exchange index on portfolio and WebSocket fill records (2026-08-20) | Added `exchange_index` to `Fill`, `Settlement`, `WsFill`/`WsFillRef` |
| Exchange index filters for portfolio lists (2026-08-20) | Added `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| RFQs and combo-market creation for sub-account-restricted API keys (2026-08-20) | No code change — permission-only |
| Optional balance reads by exchange_index (2026-08-20) | Added `GetBalanceParams { subaccount, exchange_index }`; `get_balance` now takes params (breaking) |
| API key location attestation expiry (2026-08-16) | Added `api_key_region_expiration_ts` to `GetApiKeysResponse` |
| New `center_deci_edge_centi_cent` price level structure (2026-08-13) | No code change — `price_level_structure` modeled as raw `String` |
| Balance reads scoped by exchange_index (2026-08-13) | Same as "Optional balance reads by exchange_index" above |
| Block trade indicator for WebSocket trades (2026-08-13) | Added `is_block_trade: bool` to `WsTrade`/`WsTradeRef` |
| Exchange shard descriptions (2026-08-13) | Added `ExchangeIndexStatus.description` |
| Margin order groups bind to single exchange_index (2026-08-13) | No code change — margin order groups not in crate scope |
| Order group maximum increased to 100,000 per user (2026-08-13) | No code change — operational limit only |
| Multivariate lookup endpoint and channel removed (2026-08-06) | **Removed** REST lookup endpoints/types and the `multivariate` WebSocket channel/message type entirely |
| Richer combo-validation errors on FIX RFQ creation (2026-08-13) | No code change — FIX API not implemented in crate |
| Intra-account transfer history endpoints (2026-08-13) | Added `get_intra_exchange_instance_transfer(s)` and types |
| FIX execution reports identify the source exchange index (2026-08-06) | No code change — FIX API not implemented in crate |
| Sided leverage estimates on margin markets (2026-08-06) | No code change — margin market types not in crate |
| Order group limit updates support subaccounts (2026-08-06) | `update_order_group_limit` now takes a `SubaccountQueryParams` (breaking) |
| Multivariate event collections include exchange_index (2026-08-06) | Added `exchange_index` to `MultivariateEventCollection` |
| Richer combo-validation errors on multivariate market creation (2026-07-30) | No code change — `ErrorResponse.message`/`.details` already `Option<String>`; `code` unchanged |
| The `service` field has been removed from error responses (2026-08-06) | **Removed** `ErrorResponse.service` entirely (breaking) |
| The `service` field on error responses is deprecated (2026-07-28) | Superseded by removal above |
| Lifecycle creation messages now include exchange_index (2026-07-30) | Added `exchange_index` to `WsMarketLifecycleV2`/`Ref` and `WsEventLifecycle`/`Ref` |
| Series responses include exchange_index (2026-07-30) | Added `exchange_index` to `Series` |
| New endpoint for event-keyed live data (2026-07-30) | Added `get_event_live_data` and types |
| Subaccount-restricted API keys can read order queue positions (2026-07-30) | No code change — permission-only; endpoints already modeled |
| Event `product_metadata` now includes cadence (2026-07-30) | Added `cadence` to `EventMetadata` |
| Subaccount-restricted API keys can use batch order endpoints (2026-07-30) | No code change — permission-only |
| Subaccount on `quote_created` (2026-07-30) | Added `subaccount` to `WsQuoteCreated`/`Ref`, and (already-documented but previously missing) `WsQuoteAccepted`/`Ref` and `WsQuoteExecuted`/`Ref` |
| Subaccount-restricted API keys can manage order groups (2026-07-30) | No code change — permission-only |
| Order groups limited to 25,000 per user (2026-07-23) | No code change — operational limit (superseded by the 100,000 increase above) |
| Incentive programs on hidden events excluded from listing (2026-07-22) | No code change — visibility/behavior only |
| Historical positions endpoint (2026-07-23) | See "Filter historical positions by subaccount" above |
| Subaccount-restricted API keys can open WebSocket sessions (2026-07-23) | No code change — permission-only |
| Subaccount-restricted API keys can quote on RFQ FIX sessions (2026-07-23) | No code change — FIX API not implemented in crate |
| Pyth value WebSocket channel (2026-07-23) | Added full `pyth_value` channel: `WsChannelV2::PythValue`, `WsPythValue(Ref)`, `WsPythUnderlyingList(Ref)`, `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`, `underlying_tickers` fields |
| Support for FIX Tag 2446 on Incremental Refresh (2026-07-09) | No code change — FIX API not implemented in crate |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_rfq_quote`; see RFQ quote retention entry below for the full RFQ-scoped action set |
| Exchange announcements endpoint removed (2026-07-04) | **Removed** `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` entirely |
| Deprecated Predictions REST schema fields removed (2026-07-09) | **Removed** `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` entirely |
| Margin orders now identify system order reasons (2026-07-09) | No code change — margin orders not in crate scope |
| New price level structures (2026-07-23) | No code change — `price_level_structure` modeled as raw `String`; `price_ranges` is the source of truth and was already/now consumed |
| Multivariate lookup history endpoints are fully deprecated (2026-07-02) | See "Multivariate lookup endpoint and channel removed" above |
| Margin positions now include an `is_portfolio` flag (2026-07-02) | No code change — margin positions not in crate scope |
| Trade-scoped API key permissions (2026-06-30) | No code change — scopes already modeled as `Vec<String>` |
| `price_ranges` added to `market_lifecycle_v2` events (2026-07-02) | Added `price_ranges: Vec<WsPriceRange>` to `WsMarketLifecycleV2`/`Ref` (previously missing) |
| Margin positions `margin_used` omitted for jointly-margined portfolio positions (2026-06-29) | No code change — margin positions not in crate scope |
| Margin risk per-market metrics limited (2026-06-26) | No code change — margin risk not in crate scope |
| Per-index exchange status (2026-07-02) | Added `intra_exchange_transfers_active` and `exchange_index_statuses: Vec<ExchangeIndexStatus>` to `GetExchangeStatusResponse` |
| Per-index subaccount balances (2026-07-02) | Added `exchange_index` to `SubaccountBalance` |
| AcceptQuote rejects carry a specific reason on FIX (2026-07-02) | No code change — FIX API not implemented in crate |
| More specific FIX rejects for cancel/replace failures (2026-07-02) | No code change — FIX API not implemented in crate |
| RFQ quote retention and RFQ-scoped quote actions (2026-06-25) | Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; deprecated (`#[deprecated]`) the quote-ID-only equivalents |
| API usage tier qualification requirements halved (2026-06-25) | No code change — no shape change |
| FIX exchange index routing (2026-06-25) | No code change — FIX API not implemented in crate |
| RFQ quotes support post-only on FIX (2026-06-24) | No code change — FIX API not implemented in crate; REST `CreateQuoteRequest.post_only` added independently below |
| Get Quote rate-limit cost reduced to 2 tokens (2026-06-23) | No code change — operational |
| RFQ quote market and event filters removed (2026-06-20) | **Removed** `market_ticker`/`event_ticker` from `GetQuotesParams` (breaking) |
| Communications RFQ and quote retention window reduced (2026-06-19) | No code change — operational retention policy |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| `settlement_sources` added to the events API (2026-06-18) | Added `settlement_sources: Vec<SettlementSource>` to `EventData` |
| Strike type and cap strike on `market_lifecycle_v2` `metadata_updated` (2026-06-18) | Added `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2`/`Ref` |
| RFQ quote identity on FIX (2026-06-18) | No code change — FIX API not implemented in crate |
| Trade entries in FIX market data (2026-06-18) | No code change — FIX API not implemented in crate |
| Legacy order mutation endpoints deprecated (2026-06-18) | **Removed** `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` and exclusive types entirely; use the V2 equivalents |
| Event tickers filter on `GET /trade-api/v2/events` (2026-06-18) | Added `tickers` (CSV) filter to `GetEventsParams` |
| Block-trade accept API key permissions (2026-06-18) | No code change — scopes already modeled as `Vec<String>` |
| Sanity limits enforced on orderbook subscriptions (2026-06-18) | No code change — server-side limits only |
| Quote time filters and pagination fix (2026-06-18) | Added `min_ts`/`max_ts` to `GetQuotesParams`; pagination fix is server-side only |
| API usage volume progress endpoint (2026-06-11) | Added `get_api_usage_level_volume_progress` and types |
| Perps mark prices on margin markets (2026-06-11) | No code change — margin markets not in crate scope |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Added `upgrade_api_usage_level` |
| Margin fee-tier endpoint returns active rates (2026-06-11) | No code change — value-only fix on an already-modeled endpoint |
| Perps volume and open interest notional fields (2026-06-11) | No code change — margin/perps markets not in crate scope |
| Tick size added to `GET Margin Markets` (2026-06-11) | No code change — margin markets not in crate scope |
| Fractional quantities for RFQs (2026-06-11) | No code change — `contracts_fp` already present |

### Added

- [Rust API] `cancel_all_orders` / `CancelAllOrdersParams` for the new `DELETE /portfolio/events/orders`
  endpoint. `CancelOrderV2Params`, `DecreaseOrderV2Request`, and `BatchCancelOrderV2RequestOrder` gained
  a `market_ticker` field for exchange-shard auto-routing.
- [Rust API] `get_historical_positions` / `GetHistoricalPositionsParams` for `GET /historical/positions`.
- [Rust API] `get_target_balance_allocation`, `set_target_balance_allocation`,
  `SetTargetBalanceAllocationRequest`, `TargetBalanceAllocation`, `RestingMarginReservation` for
  `/portfolio/target_balance_allocation`.
- [Rust API] `intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfer(s)`,
  `intra_exchange_instance_transfers_pager`, `stream_intra_exchange_instance_transfers`, and the
  supporting types for `/portfolio/intra_exchange_instance_transfer(s)`.
- [Rust API] `get_event_live_data` / `GetEventLiveDataParams` / `EventLiveData` for
  `GET /live_data/events/{event_ticker}`.
- [Rust API] `get_weather_index` and `get_weather_index_calibrations` plus `GetWeatherIndexParams`,
  `WeatherIndexPoint`, `WeatherIndexStationReading`, `WeatherIndexCalibration`,
  `WeatherIndexCalibrationStation` for the new Kalshi Weather Index endpoints.
- [Rust API] `get_api_usage_level_volume_progress` and `upgrade_api_usage_level` for
  `/account/api_usage_level/volume_progress` and `/account/api_usage_level/upgrade`.
- [Rust API] `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` for the
  RFQ-scoped quote action endpoints.
- [Rust API] Full `pyth_value` WebSocket channel support: `WsChannelV2::PythValue`, `WsMsgType::PythValue`
  / `PythValueUnderlyingList`, `WsPythValue(Ref)`, `WsPythUnderlyingList(Ref)`,
  `WsDataMessageV2::PythValue` / `PythValueUnderlyingList` (+ `Ref` variants),
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList`, and an
  `underlying_tickers` field on `WsSubscriptionParamsV2` / `WsUpdateSubscriptionParamsV2`.
- [Rust API] `exchange_index` field added to: `Market`, `MarketPosition`, `Fill`, `Settlement`, `Series`,
  `EventData`, `MultivariateEventCollection`, `SubaccountBalance`, `ApiKey`/`CreateApiKeyRequest`/
  `GenerateApiKeyRequest` (as `subaccount`), `WsUserOrder`, `WsFill`/`Ref`, `WsMarketLifecycleV2`/`Ref`,
  `WsEventLifecycle`/`Ref`. `ExchangeIndexStatus` (with `description`) added to
  `GetExchangeStatusResponse.exchange_index_statuses`, alongside `intra_exchange_transfers_active`.
- [Rust API] `exchange_index` filter added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
  `GetBalanceParams { subaccount, exchange_index }` and `GetBalanceResponse.balance_breakdown:
  Vec<IndexedBalance>` for per-exchange-index balance reads.
- [Rust API] `WsMarketLifecycleV2`/`Ref` gained `price_ranges: Vec<WsPriceRange>`, `strike_type`,
  `cap_strike`, `custom_strike` (all present only on the relevant lifecycle events).
- [Rust API] `WsTrade`/`Ref` gained `is_block_trade: bool`. `WsQuoteCreated`/`Ref`,
  `WsQuoteAccepted`/`Ref`, and `WsQuoteExecuted`/`Ref` gained `subaccount: Option<u32>`.
- [Rust API] `EventData` gained `settlement_sources: Vec<SettlementSource>`. `EventMetadata` gained
  `cadence: Option<String>`. `GetEventsParams` gained `tickers: Option<Vec<String>>`.
- [Rust API] `GetQuotesParams` gained `min_ts`, `max_ts`, `user_filter`. `CreateQuoteRequest` gained
  `post_only: Option<bool>`. `RFQ` gained `creator_subaccount`; `Quote` gained `post_only`,
  `creator_subaccount`, `rfq_creator_subaccount`.
- [Rust API] `GetApiKeysResponse` gained `api_key_region_expiration_ts`. `IncentiveProgram` gained
  `incentive_description` and `max_reward_per_account`; `GetIncentiveProgramsParams` gained
  `incentive_description`.
- [Rust API] `FeeType` gained `QuadraticWithComboMakerFees`.

### Changed

- [Rust API] `get_balance` now takes `GetBalanceParams` instead of no arguments.
- [Rust API] `update_order_group_limit` now takes an additional `SubaccountQueryParams` argument.
  `SubaccountQueryParams` gained an `exchange_index` field (also usable on
  `delete_order_group`/`reset_order_group`/`trigger_order_group`).
- [Rust API] `create_subaccount` now takes a `CreateSubaccountRequest` (`exchange_index`) instead of no
  arguments. `ApplySubaccountTransferRequest` gained an `exchange_index` field.

### Deprecated

- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` are `#[deprecated]`; use the
  RFQ-scoped equivalents (`get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`).
  The exchange still serves the legacy endpoints (`deprecated: true` in the OpenAPI spec) but recommends
  migrating.

### Removed

- [Rust API] `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders` and their exclusive request/response types (`CreateOrderRequest`,
  `CreateOrderResponse`, `CancelOrderParams`, `CancelOrderResponse`, `AmendOrderRequest`,
  `AmendOrderResponse`, `DecreaseOrderRequest`, `DecreaseOrderResponse`, `BatchCreateOrdersRequest`,
  `BatchCreateOrdersResponse`, `BatchCreateOrdersIndividualResponse`, `BatchCancelOrdersRequestOrder`,
  `BatchCancelOrdersRequest`, `BatchCancelOrdersResponse`, `BatchCancelOrdersIndividualResponse`) — the
  legacy `/portfolio/orders` mutation surface was removed from the API. Use the V2 event-order
  endpoints.
- [Rust API] `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their exclusive types
  (`GetMultivariateEventCollectionLookupHistoryParams`, `GetMultivariateEventCollectionLookupHistoryResponse`,
  `LookupPoint`, `LookupTickersForMarketInMultivariateEventCollectionRequest`,
  `LookupTickersForMarketInMultivariateEventCollectionResponse`) — the underlying REST endpoints were
  removed from the API.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsMultivariate`/`Ref`, `WsMultivariateSelectedMarket`/`Ref`, and `WsDataMessageV2::Multivariate`/`Ref`
  — the `multivariate` WebSocket channel was removed from the API.
- [Rust API] `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed from the API.
- [Rust API] `ErrorResponse.service` — removed from the API; branch on `code` instead.
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` — removed from the API schema.
- [Rust API] `GetQuotesParams.market_ticker` and `GetQuotesParams.event_ticker` — removed from the API;
  filter by `rfq_id`, `status`, `user_filter`, `rfq_user_filter`, or `min_ts`/`max_ts` instead.
- [Rust API] Also removed the internal `ws::types::MarketPositionRef`/`EventPositionRef` zero-copy
  mirrors of the REST `MarketPosition`/`EventPosition` types: they were unused by the crate's own
  WebSocket parsing (the real `market_positions` channel payload is unrelated and already modeled by
  `WsMarketPositionRef`) and had drifted from the REST shape.

### Breaking

- [Rust API] See Removed above: several public methods and types were deleted outright rather than
  deprecated, because the underlying API surface no longer exists. Downstream code depending on any of
  the removed items must migrate to the listed replacements.
- [Rust API] `get_balance()` → `get_balance(GetBalanceParams)`. `create_subaccount()` →
  `create_subaccount(CreateSubaccountRequest)`. `update_order_group_limit(id, body)` →
  `update_order_group_limit(id, SubaccountQueryParams, body)`.
- [Rust API] `MarketPosition`, `Fill`, `Settlement`, `ApiKey`, `CreateApiKeyRequest`,
  `GenerateApiKeyRequest`, `GetApiKeysResponse`, `GetQuotesParams`, `CreateQuoteRequest`,
  `ApplySubaccountTransferRequest`, `SubaccountBalance`, `IncentiveProgram`, `GetBalanceResponse`,
  `GetExchangeStatusResponse`, `GetPortfolioRestingOrderTotalValueResponse`, `EventData`,
  `EventMetadata`, `Series`, `MultivariateEventCollection`, `SubaccountQueryParams`, `FeeType`, and
  several `Ws*`/`Ws*Ref` message and subscription types gained new public fields/variants. Exhaustive
  struct-literal construction or `match` over these types must be updated (or use `..`/`..Default::default()`
  and wildcard arms).
- [Rust API] `CreateOrderV2Request.exchange_index`, `AmendOrderV2Request.exchange_index`,
  `DecreaseOrderV2Request.exchange_index`, `CancelOrderV2Params.exchange_index`, and
  `BatchCancelOrderV2RequestOrder.exchange_index` changed from `Option<u32>` to `Option<i32>` to allow
  the documented `-1` auto-routing sentinel.

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
