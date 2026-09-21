# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-21

### Compatibility

- Docs snapshot: 2026-09-21
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-24

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition.** Entries
tagged `Margin` or `FIX` are out of scope (this crate models the Predictions
REST/WebSocket surface only; FIX and Margin market/order/position endpoints are
not implemented) unless noted otherwise. Several new Predictions endpoint
families are also **not yet implemented** and are called out explicitly below
as known gaps rather than silently skipped.

| Entry | Action |
|---|---|
| `available_on_brokers` removed from event responses (2026-09-10) | **Breaking** — removed `EventData::available_on_brokers` |
| Legacy `/portfolio/orders` mutation endpoints deprecated (2026-06-18), now fully removed upstream | **Breaking** — removed `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/`batch_cancel_orders` and their request/response types; use the V2 `*_v2` equivalents (already present since 0.6.0) |
| `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` removed (2026-07-09) | **Breaking** — removed all three fields; also removed the now-unused, already-incorrect `MarketPositionRef`/`EventPositionRef` dead code in `ws::types` that referenced the removed field |
| `service` field deprecated (2026-07-28) then removed (2026-08-06) from error response bodies | **Breaking** — removed `ErrorResponse::service` |
| `GET /exchange/announcements` removed (2026-07-04) | **Breaking** — removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| Multivariate lookup surface removed: `PUT .../lookup`, its GET history feed, and the WS `multivariate` channel (message `multivariate_lookup`) (2026-08-06) | **Breaking** — removed `get_multivariate_event_collection_lookup_history`, `lookup_tickers_for_market_in_multivariate_event_collection` and related types; removed `WsChannelV2::Multivariate`, `WsMsgType::Multivariate(Lookup)`, and the `WsMultivariate`/`WsMultivariateRef` message types |
| `GET /communications/quotes` drops `market_ticker`/`event_ticker` filters (2026-06-20); gains `min_ts`/`max_ts` (2026-06-18) and `user_filter` | **Breaking** — removed the two fields from `GetQuotesParams`; added `min_ts`, `max_ts`, `user_filter` |
| RFQ quotes no longer durably queryable by quote ID alone; RFQ-scoped quote endpoints added (2026-06-25); RFQ-scoped quote lookup added (2026-07-09) | Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; deprecated (`#[deprecated]`) `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote` |
| `target_cost_excludes_fees` on `CreateRFQ` (2026-09-10) | Added to `CreateRFQRequest` |
| `GET /historical/fills`, `GET /historical/orders` accept `min_ts` (2026-09-17) | Added to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| `GET /historical/positions` (new endpoint); `market_positions_last_updated_ts` on `GET /historical/cutoff` | Added `get_historical_positions`, `GetHistoricalPositionsParams`; added the cutoff field |
| `GET /fcm/orders` accepts `client_order_ids`; `subtrader_id` now optional (2026-09-03) | Updated `GetFcmOrdersParams` |
| Series objects include `categories` (2026-09-17); `GET /series` exposes `exchange_index` (2026-07-30) | Added both to `Series` |
| Events gain top-level `settlement_sources` (2026-06-18); `product_metadata.cadence` (2026-07-30); `exchange_index`, `fee_type_override`, `fee_multiplier_override` on the event schema | Added all to `EventData`/`EventMetadata` |
| `GET /events` gains `tickers` filter (2026-06-18) and `min_updated_ts` | Added to `GetEventsParams` |
| `exchange_index` added across the multi-shard rollout: `Fill`, `Settlement`, `MarketPosition`, `Order` (V1 shape kept for `GET /portfolio/orders`/`GET /portfolio/orders/{id}`), `MultivariateEventCollection`, `SubaccountBalance`, `ApiKey`; `GET /portfolio/{orders,positions,fills}` gain an `exchange_index` filter (2026-07-02 through 2026-08-20) | Added fields/params throughout `orders.rs`, `portfolio.rs`, `multivariate.rs`, `account.rs` |
| `GET /portfolio/balance` scopes to `exchange_index` / defaults to all, gains `balance_breakdown` (2026-08-13, 2026-08-20) | **Breaking** — `get_balance` now takes `GetBalanceParams`; added `IndexedBalance`, `GetBalanceResponse::balance_breakdown` |
| `GET /portfolio/summary/total_resting_order_value` gains `resting_order_value_breakdown` (2026-08-20) | Added to `GetPortfolioRestingOrderTotalValueResponse` |
| `GET /exchange/status` gains `intra_exchange_transfers_active`, `exchange_index_statuses` (2026-07-02); shard `description` (2026-09-10) | Added `ExchangeIndexStatus`, extended `GetExchangeStatusResponse` |
| `PUT /portfolio/order_groups/{id}/limit` gains `subaccount`/`exchange_index` query params (2026-08-06) | **Breaking** — `update_order_group_limit` now takes an added `UpdateOrderGroupLimitParams` |
| `POST /api_keys`, `POST /api_keys/generate` accept `subaccount` (0-63); `GET /api_keys` returns it; `api_key_region_expiration_ts` on `GET /api_keys` (2026-07-02, 2026-08-16) | Added `subaccount` to `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey`; added `api_key_region_expiration_ts` |
| `GET /portfolio/subaccounts/balances` returns one entry per exchange index (2026-07-02) | Added `SubaccountBalance::exchange_index` |
| `GET /account/api_usage_level/volume_progress`, `POST /account/api_usage_level/upgrade` (new endpoints, 2026-06-11) | Added `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level` and response types |
| `GET /live_data/weather/{city}` (2026-08-20), `GET /live_data/weather/{city}/calibrations` (2026-08-31), `GET /live_data/events/{event_ticker}` (2026-07-30) (new endpoints) | Added `get_weather_index`, `get_weather_index_calibrations`, `get_event_live_data` and all response types |
| WS `market_lifecycle_v2` `metadata_updated` gains top-level `strike_type`/`cap_strike`/`custom_strike` (2026-06-18); `created`/`price_level_structure_updated` gain `price_ranges` (2026-07-02) and `exchange_index` (2026-07-30) | Added all to `WsMarketLifecycleV2`/`Ref` |
| WS `event_lifecycle` gains `exchange_index` (2026-07-30) | Added to `WsEventLifecycle`/`Ref` |
| WS `fill` requires `exchange_index` (already-shipped field, confirmed against AsyncAPI); WS `user_orders` gains `exchange_index` (2026-08-27) | Added to `WsFill`/`Ref` and `WsUserOrder` |
| WS trade messages gain `is_block_trade` (2026-08-13) | Added to `WsTrade`/`Ref` |
| WS `quote_created` gains `subaccount` (2026-07-30); also added the previously-missing required `rfq_creator_id` field (pre-existing drift found while touching this struct) | Added both to `WsQuoteCreated`/`Ref` |
| AsyncAPI doc corrections: `fractional_trading_enabled` / the `fractional_trading_updated` event type removed from the spec; ticker `dollar_volume`/`dollar_open_interest` documented as signed (already `i64`); `sid`/`seq` on subscription-scoped errors (already modeled since 0.7.0); error codes 6/16/17 retired; `market_id`/`market_ticker` confirmed absent from the error schema (2026-09-10, 2026-09-17) | **Breaking** — removed `fractional_trading_enabled` and `WsMarketLifecycleEventType::FractionalTradingUpdated`; everything else already matched, no change needed |
| New `price_level_structure` values: 7 added 2026-07-23, `center_deci_edge_centi_cent` added 2026-08-13/09-10 | No code change — modeled as a raw `String`, not an enum |
| `available_on_brokers` field deprecated (2026-08-27, superseded by removal above) | Superseded, see removal entry |
| `Market.price_level_structure`/`price_ranges` remain the canonical replacements (2026-07-09 clarification) | No code change — already modeled |
| Structured target `details.image_url` (2026-08-29) | No code change — `StructuredTarget::details` is already a free-form `Map<String, Value>` per this crate's decouple-from-schema design |
| `Accept-Language` header for localized market responses (2026-08-27) | No code change — caller-supplied request header, no response shape change |
| RFQ fractional contracts via `contracts_fp` (2026-06-11) | No code change — already present in `CreateRFQRequest` (confirmed in 0.6.0) |
| Exchange sharding rollout announcements, per-shard rate-limit/token-cost changes, qualification/retention-window/rate-limit-cost tweaks, post-only/maker-fee rollout timing, order-group limit increase (25k→100k), cancel-all token cost, amend `remaining_count` reporting fix, WS orderbook subscription sanity limits, WS subscribed/data race fix, new combo series tickers (2026-06 through 2026-09) | No code change — operational/behavioral only, no schema impact |
| API key scopes: `write::trade`, `read`/`write::block_trade_accept`, `read::portfolio_balance` (2026-06-18, 2026-06-30) | No code change — scopes are caller-supplied strings, not enumerated by this crate |
| Subaccount-restricted API keys can now use WS, FIX RFQ sessions, order-group/batch/queue-position REST endpoints (2026-07-23, 2026-07-30) | No code change — server-side authorization behavior only |
| `GET /incentive_programs` excludes hidden-event programs (2026-07-22) | No code change — behavioral filter only |
| Multivariate collection creation richer REST/FIX error bodies (2026-07-30, 2026-08-13) | No code change — `message`/`details` already generic on `ErrorResponse` |
| FIX-only entries (order entry tags, market data tags, reject reasons, sharding, connectivity) | No code change — FIX is not implemented by this crate |
| Margin-only entries (fee tiers/rates, market fields, order groups, positions, risk, exit triggers, rate limits, mark prices, notional fields, `asset_class`, `important_info`) | No code change — Margin market/order/position endpoints are not implemented by this crate (only `GET /margin/fee_tiers` is, unaffected by these) |

**Known gaps (not implemented in this release — new endpoint/channel families, tracked for a future refresh):**

- `POST`/`GET /portfolio/target_balance_allocation` (2026-08-20, 2026-09-03, 2026-09-17)
- `POST /portfolio/intra_exchange_instance_transfer` and `GET /portfolio/intra_exchange_instance_transfers[/{id}]` (2026-08-13, 2026-08-20)
- New "cancel all resting orders across subaccounts" endpoints (2026-08-27)
- WS `pyth_value` channel (2026-07-23)
- WS `cfbenchmarks_value_5hz` channel (2026-09-03)

### Breaking

- [Rust API] Removed the legacy `/portfolio/orders` mutation surface: `create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`, and their
  `CreateOrderRequest`/`CreateOrderResponse`/`CancelOrderParams`/`CancelOrderResponse`/
  `AmendOrderRequest`/`AmendOrderResponse`/`DecreaseOrderRequest`/`DecreaseOrderResponse`/
  `BatchCreateOrdersRequest`/`BatchCreateOrdersResponse`/`BatchCreateOrdersIndividualResponse`/
  `BatchCancelOrdersRequestOrder`/`BatchCancelOrdersRequest`/`BatchCancelOrdersResponse`/
  `BatchCancelOrdersIndividualResponse` types. Upstream removed these endpoints entirely. Use the
  V2 event-order endpoints (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`,
  `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`), present since 0.6.0.
- [Rust API] `get_balance` now takes a `GetBalanceParams { subaccount, exchange_index }` argument.
- [Rust API] `update_order_group_limit` now takes an added `UpdateOrderGroupLimitParams { subaccount, exchange_index }` argument before the request body.
- [Rust API] Removed `EventData::available_on_brokers`, `Market::response_price_units`,
  `Market::fractional_trading_enabled`, `MarketPosition::resting_orders_count`,
  `ErrorResponse::service`. All were removed from the live OpenAPI/AsyncAPI schemas.
- [Rust API] Removed `WsMarketLifecycleEventType::FractionalTradingUpdated` and the
  `fractional_trading_enabled` field from `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref`. Neither
  appears in the current AsyncAPI spec.
- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` (`GET /exchange/announcements` removed
  upstream; use `GET /exchange/schedule`).
- [Rust API] Removed the multivariate lookup surface: `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `GetMultivariateEventCollectionLookupHistoryParams/Response`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest/Response`; and the WS `multivariate`
  channel (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsMultivariate`/`WsMultivariateRef`). Also removed the dead, already-mismatched
  `MarketPositionRef`/`EventPositionRef` structs from `ws::types` (unused; superseded by the correct
  `WsMarketPosition`/`WsMarketPositionRef` in `ws::types::messages::positions`).
- [Rust API] `GetQuotesParams` no longer has `event_ticker`/`market_ticker` (removed upstream); gained
  `min_ts`, `max_ts`, `user_filter`.
- [Rust API] `GetFcmOrdersParams::subtrader_id` changed from `String` to `Option<String>` (now
  optional when `client_order_ids` is supplied).

### Added

- [Rust API] RFQ-scoped quote endpoints: `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`,
  `confirm_rfq_quote`. Deprecated the quote-ID-only `get_quote`/`delete_quote`/`accept_quote`/
  `confirm_quote` (`#[deprecated]`); Kalshi may remove them in a future release.
- [Rust API] `CreateRFQRequest::target_cost_excludes_fees`.
- [Rust API] `GetHistoricalFillsParams::min_ts`, `GetHistoricalOrdersParams::min_ts`.
- [Rust API] `get_historical_positions` / `GetHistoricalPositionsParams` for `GET /historical/positions`;
  `GetHistoricalCutoffResponse::market_positions_last_updated_ts`.
- [Rust API] `GetFcmOrdersParams::client_order_ids`.
- [Rust API] `Series::categories`, `Series::exchange_index`.
- [Rust API] `EventData::settlement_sources`, `EventData::exchange_index`,
  `EventData::fee_type_override`, `EventData::fee_multiplier_override`, `EventMetadata::cadence`.
- [Rust API] `GetEventsParams::tickers`, `GetEventsParams::min_updated_ts`.
- [Rust API] `exchange_index` on `Fill`, `Settlement`, `MarketPosition`, `Order`,
  `MultivariateEventCollection`, `SubaccountBalance`, `ApiKey`; `exchange_index` filter on
  `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams`.
- [Rust API] `GetBalanceParams`, `GetBalanceResponse::balance_breakdown`, `IndexedBalance`.
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown`.
- [Rust API] `ExchangeIndexStatus`, `GetExchangeStatusResponse::intra_exchange_transfers_active`,
  `GetExchangeStatusResponse::exchange_index_statuses`.
- [Rust API] `CreateApiKeyRequest::subaccount`, `GenerateApiKeyRequest::subaccount`,
  `ApiKey::subaccount`, `ApiKey::api_key_region_expiration_ts`.
- [Rust API] `get_account_api_usage_level_volume_progress` / `GetAccountApiUsageLevelVolumeProgressResponse`,
  `upgrade_account_api_usage_level` for the new `/account/api_usage_level/*` endpoints.
- [Rust API] `get_weather_index` / `GetWeatherIndexResponse`, `get_weather_index_calibrations` /
  `GetWeatherIndexCalibrationsResponse`, `get_event_live_data` / `GetEventLiveDataResponse` for the
  new `/live_data/weather/*` and `/live_data/events/{event_ticker}` endpoints.
- [Rust API] `strike_type`, `cap_strike`, `custom_strike`, `price_ranges`, `exchange_index` on WS
  `market_lifecycle_v2`; `exchange_index` on WS `event_lifecycle`, `fill`, `user_orders`;
  `is_block_trade` on WS `trade`; `subaccount` and the previously-missing `rfq_creator_id` on WS
  `quote_created`.

### Removed

- [Rust API] See Breaking section — all removals correspond to endpoints/fields removed from the
  live OpenAPI/AsyncAPI specs, not preserved as compatibility shims per this repo's refresh policy.

### Fixed

- [Tests] Fixed a pre-existing compile break in `ws::types::envelope`'s own test module
  (`WsMessageV2/Ref::ListSubscriptions` match arms hadn't been updated for the `sid`/`seq` fields
  added in 0.7.0), and in `tests/rest_auth.rs` (`GetAccountApiLimitsResponse` field names hadn't
  been updated for the 0.6.0 restructuring). Both were only reachable via `cargo test --all-targets`
  / `--features live-tests` and were not caught by default `cargo test`.


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
