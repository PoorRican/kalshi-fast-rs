# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.7.0] - 2026-07-14

### Compatibility

- Docs snapshot: 2026-07-14
- OpenAPI: 3.24.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-07-14

**Changelog entries since 0.6.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| `GET /account/api_usage_level/volume_progress` (2026-06-11) | **Added** — missed by the 0.6.0 refresh; added `get_account_api_usage_level_volume_progress` |
| `POST /account/api_usage_level/upgrade` (2026-06-11) | **Added** — missed by the 0.6.0 refresh; added `upgrade_account_api_usage_level` |
| Perps market mark prices / dollar-notional fields (2026-06-11) | No code change — margin market types not in crate |
| `settlement_sources` on top-level event payload (2026-06-18) | Added to `EventData` |
| `strike_type`/`cap_strike`/`custom_strike` on `metadata_updated` (2026-06-18) | Added to `WsMarketLifecycleV2` |
| Legacy `/portfolio/orders` mutation endpoints deprecation notice (2026-06-18/06-25) | **Deprecated** — `#[deprecated]` on `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/`batch_cancel_orders` (not removed; live-removal status unconfirmed) |
| `GET /events` gains `tickers` filter (2026-06-18) | Added to `GetEventsParams` |
| `GET /events` `min_updated_ts` (found via spec grep, not separately changelogged) | Added to `GetEventsParams` |
| Narrow `block_trade_accept`/`portfolio_balance` API key scopes (2026-06-18) | No code change — scopes stored as `Vec<String>` already |
| WebSocket orderbook subscription sanity limits (2026-06-18) | No code change — operational rate limit only |
| `GET /communications/quotes` `min_ts`/`max_ts` + cursor pagination fix (2026-06-18) | Added `min_ts`/`max_ts`; pagination fix is server-side only |
| RFQ/quote retention window reduced 14d → 7d (2026-06-19) | No code change — operational |
| `GET /communications/quotes` drops `market_ticker`/`event_ticker` filters (2026-06-20) | **Breaking** — those `GetQuotesParams` fields marked `#[deprecated]` (no longer honored by the server) |
| `GET /communications/quotes/{quote_id}` now costs 2 tokens (2026-06-23) | No code change — rate-limit only |
| RFQ-scoped quote action endpoints + quote durability semantics (2026-06-25) | Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`; deprecated quote-ID-only equivalents |
| Predictions qualification requirements halved (2026-06-25) | No code change — no API shape change |
| Margin risk/positions per-market metric restrictions, `is_portfolio` flag (2026-06-26/07-02) | No code change — margin market types not in crate |
| `write::trade` API key scope (2026-06-30) | No code change — scopes stored as `Vec<String>` already |
| Multivariate lookup-history endpoint fully deprecated (2026-07-02) | **Removed** — endpoint dropped entirely from OpenAPI |
| `market_lifecycle_v2` gains `price_ranges` on `created`/`price_level_structure_updated` (2026-07-02) | Added to `WsMarketLifecycleV2` |
| `GET /exchange/status` per-index breakdown (2026-07-02) | Added `exchange_index_statuses`/`intra_exchange_transfers_active` |
| `GET /portfolio/subaccounts/balances` per-index rows (2026-07-02) | **Breaking** — `SubaccountBalance` gained required `exchange_index: i64` |
| API key single-subaccount restriction (2026-07-02) | Added `subaccount` to `ApiKey`/`CreateApiKeyRequest`/`GenerateApiKeyRequest` |
| `GET /exchange/announcements` removed (2026-07-04) | **Removed** — endpoint dropped entirely from OpenAPI |
| FIX-only changelog entries (Tag 2446, ExDestination, AcceptQuoteStatus/OrderCancelReject reasons, quoter party ID, margin trade refreshes, post-only RFQ quotes) | No code change — FIX protocol not supported by this crate |
| RFQ-scoped quote lookup endpoint, deprecating quote-ID-only lookup (2026-07-09) | Added `get_rfq_quote`; deprecated `get_quote` |
| Deprecated Predictions schema fields removed: `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (2026-07-09) | **Removed** from `Market`/`MarketPosition`/`WsMarketLifecycleV2` |
| Margin orders `order_reason` (2026-07-09) | No code change — margin market types not in crate |
| New `pyth_value` WebSocket channel (2026-07-13) | Added full channel, subscription, and message support |
| Seven new `price_level_structure` values (2026-07-23, phased rollout) | No code change — `price_level_structure` is an untyped string, not an enum |

### Added

- [Rust API] Added `get_account_api_usage_level_volume_progress()` and `GetAccountApiUsageLevelVolumeProgressResponse` / `AccountApiUsageLevelVolumeProgress` / `AccountApiUsageLevelVolumeGoal` for `GET /account/api_usage_level/volume_progress` (2026-06-11).
- [Rust API] Added `upgrade_account_api_usage_level()` for `POST /account/api_usage_level/upgrade` (2026-06-11).
- [Rust API] Added `settlement_sources: Vec<SettlementSource>` to `EventData` (2026-06-18).
- [Rust API] Added `tickers: Option<String>` and `min_updated_ts: Option<i64>` query parameters to `GetEventsParams`.
- [Rust API] Added `strike_type: Option<String>`, `cap_strike: Option<f64>`, and `custom_strike: Option<Map<String, Value>>` to `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref`, surfaced on `metadata_updated` events (2026-06-18).
- [Rust API] Added `price_ranges: Option<Vec<PriceRange>>` (reusing the REST `PriceRange` type) to `WsMarketLifecycleV2`/`WsMarketLifecycleV2Ref`, emitted on `created`/`price_level_structure_updated` events (2026-07-02).
- [Rust API] Added `min_ts`/`max_ts`/`user_filter` to `GetQuotesParams` and `user_filter` to `GetRFQsParams`.
- [Rust API] Added RFQ-scoped quote endpoints: `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` (2026-06-25/2026-07-09).
- [Rust API] Added `ExchangeIndexStatus` and `intra_exchange_transfers_active: Option<bool>` / `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` on `GetExchangeStatusResponse` (2026-07-02).
- [Rust API] Added `subaccount: Option<u32>` to `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest` for single-subaccount-restricted API keys (2026-07-02).
- [Rust API] Added full `pyth_value` channel support: `WsChannelV2::PythValue` (private/authenticated), `underlying_tickers` on `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2`, `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`, `WsMsgType::PythValue`/`PythValueUnderlyingList`, new types `WsPythValue`/`WsPythValueRef`/`WsPythUnderlyingList`/`WsPythUnderlyingListRef`, and `WsDataMessageV2`/`WsDataMessageRef::PythValue`/`PythValueUnderlyingList` variants routed through both wire and envelope parse paths (2026-07-13).
- [Tests] Added coverage for the new lifecycle fields (`strike_type`/`cap_strike`/`custom_strike`/`price_ranges`), the `pyth_value`/`pyth_value_underlying_list` messages, the underlying-ticker subscription tracker/validation behavior, and the per-index exchange status fields.

### Changed

- [Rust API] `GetQuotesParams::market_ticker` and `::event_ticker` are `#[deprecated]`: the server no longer honors them as of 2026-06-20 (kept for source compatibility, but have no effect).
- [Rust API] `GetQuotesParams::quote_creator_user_id`/`::rfq_creator_user_id` and `GetRFQsParams::creator_user_id` are `#[deprecated]` in favor of `user_filter`/`rfq_user_filter` (still functional per the current spec).
- [Rust API] The legacy v1 order mutation methods (`create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) are `#[deprecated]` in favor of the `_v2` equivalents; the OpenAPI spec no longer documents the underlying `/portfolio/orders` mutation paths (only `GET` remains).
- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` (quote-ID-only) are `#[deprecated]` in favor of the RFQ-scoped equivalents.
- [Rust API] `lookup_tickers_for_market_in_multivariate_event_collection` is `#[deprecated]` (the OpenAPI spec marks it deprecated; it predates RFQs).

### Removed

- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, and `AnnouncementStatus` — `GET /exchange/announcements` was removed from the OpenAPI spec (2026-07-04).
- [Rust API] Removed `get_multivariate_event_collection_lookup_history`, `GetMultivariateEventCollectionLookupHistoryParams`, `GetMultivariateEventCollectionLookupHistoryResponse`, and `LookupPoint` — the lookup-history endpoint was fully removed from the OpenAPI spec (2026-07-02).
- [Rust API] Removed `Market.response_price_units` and `Market.fractional_trading_enabled` — removed from the OpenAPI schema (2026-07-09).
- [Rust API] Removed `MarketPosition.resting_orders_count` (REST and the WebSocket `MarketPositionRef`) — removed from the OpenAPI schema (2026-07-09).
- [Rust API] Removed `WsMarketLifecycleV2.fractional_trading_enabled`/`WsMarketLifecycleV2Ref.fractional_trading_enabled` and the `WsMarketLifecycleEventType::FractionalTradingUpdated` variant — no longer present in the AsyncAPI `market_lifecycle_v2` schema.

### Breaking

- [Rust API] `SubaccountBalance` gained a required `exchange_index: i64` field (2026-07-02); downstream exhaustive struct construction must add it.
- [Rust API] `GetQuotesParams::market_ticker`/`::event_ticker` no longer filter server-side (2026-06-20); code relying on that filtering behavior must migrate to filtering client-side or by RFQ/status/update-time.
- [Rust API] `WsChannelV2` gained `PythValue`; `WsUpdateAction` gained `SubscribeUnderlyings`, `UnsubscribeUnderlyings`, `UnderlyingList`; `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2` gained an `underlying_tickers` field. Downstream exhaustive matches/struct-literal construction must be updated.
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count`, `WsMarketLifecycleV2.fractional_trading_enabled`, and `WsMarketLifecycleEventType::FractionalTradingUpdated` were removed; downstream code referencing them will not compile.
- [Rust API] `get_exchange_announcements` (and its response/type family) and `get_multivariate_event_collection_lookup_history` (and its param/response/type family) were removed entirely; downstream code calling them will not compile.


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
