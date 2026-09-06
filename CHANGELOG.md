# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-06

### Compatibility

- Docs snapshot: 2026-09-06
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-10

**Version bump: minor (0.7.0 → 0.8.0).** Per `VERSIONING.md`, "Minor releases are for any
intentional breaking change to the public Rust API, or any change likely to require downstream
code changes even if it originates from upstream Kalshi churn." This refresh removes several
upstream-removed fields/endpoints/channels and changes `get_balance`'s signature, `GetFcmOrdersParams.subtrader_id`'s
type, and adds required fields to `CreateApiKeyRequest`/`GenerateApiKeyRequest` — see Breaking below.

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition.** This
refresh reconciles roughly three months of upstream changelog against the
live OpenAPI/AsyncAPI. Entries are grouped by disposition rather than listed
individually; see `docs/spec-parity.md` for the durable behavioral notes.

| Category | Disposition |
|---|---|
| Exchange sharding: `exchange_index` on markets, series, events, orders, order groups, fills, settlements, positions, multivariate event collections, balance, resting-order-value, WS fill/user_orders/lifecycle messages | **Added** — modeled as `Option<i64>`/`Option<u32>` throughout (see Added) |
| `Market.response_price_units`, `Market.fractional_trading_enabled` (+ WS `market_lifecycle_v2`), `MarketPosition.resting_orders_count` (REST + WS) | **Removed** — gone from the live OpenAPI/AsyncAPI schemas |
| `EventData.available_on_brokers` (deprecated 2026-08-27, removed 2026-09-10) | **Removed** |
| `GET /exchange/announcements` (removed 2026-07-04) | **Removed** — `get_exchange_announcements` and its types deleted |
| Multivariate lookup REST endpoint + WS `multivariate`/`multivariate_lookup` channel (removed 2026-08-06) | **Removed** — `lookup_tickers_for_market_in_multivariate_event_collection`, `get_multivariate_event_collection_lookup_history`, `WsChannelV2::Multivariate`, `WsMultivariate` and related wire types deleted |
| `service` field on REST error responses (deprecated 2026-07-28, removed 2026-08-06) | **Removed** from `ErrorResponse` |
| New `price_level_structure` values (7 new July 23, `center_deci_edge_centi_cent` Aug 13) | **No code change** — `price_level_structure` is already a plain `String` in this crate, not a closed enum |
| `pyth_value` WebSocket channel (2026-07-23), `cfbenchmarks_value_5hz` channel (2026-09-03) | **Added** — new channels, message types; `cfbenchmarks_value_5hz` reuses the existing `index_ids` / `SubscribeIndices` update-action machinery |
| RFQ-scoped quote actions (2026-06-25), RFQ-scoped quote lookup (2026-07-09) | **Added** — `get_quote_for_rfq`, `delete_quote_for_rfq`, `accept_quote_for_rfq`, `confirm_quote_for_rfq`; quote-ID-only variants kept and documented as deprecated |
| `GET /communications/quotes` `market_ticker`/`event_ticker` filters (removed 2026-06-20); `min_ts`/`max_ts`/`user_filter` (added 2026-06-18) | **Changed** — `GetQuotesParams` updated |
| `GET /historical/positions` (added 2026-07-23; `subaccount` filter added 2026-09-03) | **Added** — `get_historical_positions` |
| `GET /live_data/events/{event_ticker}` (added 2026-07-30) | **Added** — `get_event_live_data` |
| `POST /portfolio/intra_exchange_instance_transfer` + history endpoints (added 2026-08-13/20) | **Added** |
| `POST`/`GET /portfolio/target_balance_allocation` (added 2026-08-20; `resting_margin_reservation` added 2026-09-03) | **Added** |
| `DELETE /portfolio/events/orders` (cancel all orders, added 2026-08-27) | **Added** — `cancel_all_orders` |
| `POST /account/api_usage_level/upgrade`, `GET /account/api_usage_level/volume_progress` (added 2026-06-11) | **Added** |
| `GET /fcm/orders` `client_order_ids` filter; `subtrader_id` became optional (added 2026-09-03) | **Changed (breaking)** — `GetFcmOrdersParams.subtrader_id` is now `Option<String>` |
| `GET /portfolio/balance` `exchange_index`/`subaccount` scoping (added 2026-07-02/08-13/08-20) | **Changed (breaking)** — `get_balance` now takes `GetBalanceParams` |
| `POST /api_keys`, `POST /api_keys/generate` `subaccount` restriction (added 2026-07-02); `GET /api_keys` returns `subaccount` and `api_key_region_expiration_ts` (2026-08-16) | **Changed (breaking)** — new required struct-literal field on `CreateApiKeyRequest`/`GenerateApiKeyRequest` |
| `PUT /portfolio/order_groups/{id}/limit` `subaccount` param (2026-08-06); order group max raised 25k→100k (2026-07-23/08-13) | **Changed** / no code change (limit is a server-side quota) |
| `GET /events` `tickers` filter (2026-06-18); `settlement_sources` on events (2026-06-18); `product_metadata.cadence` (announced 2026-07-30) | **Added** `tickers` filter and `settlement_sources`; **no code change** for `cadence` — not present in the live OpenAPI schema despite the changelog announcement |
| `market_lifecycle_v2` gains `price_ranges` (2026-07-02), `strike_type`/`cap_strike`/`custom_strike` on `metadata_updated` (2026-06-18), `exchange_index` on `created` (2026-07-30); `event_lifecycle` gains `exchange_index` (2026-07-30) | **Added** |
| `quote_created`/`quote_accepted`/`quote_executed` WS messages gain `subaccount` (2026-07-30) | **Added** |
| WS trade message gains `is_block_trade` (2026-08-13) | **Added** to `WsTrade` |
| FIX-only changes (order entry, market data, RFQ, execution reports) | **No code change** — this crate does not implement FIX |
| Margin-exchange-only changes (margin markets, margin risk/positions, margin order groups, margin fee tiers/rates, perps) | **No code change** — margin trading surfaces are out of this crate's scope, matching the existing `get_margin_fee_tiers` exception |
| Weather index endpoints (`GET /live_data/weather/{city}`, calibration history) | **No code change** — out of scope; weather markets are not separately modeled |
| New `GET /margin/fee_tier_rates` endpoint (2026-09-03) | **No code change** — margin-only and, unlike the legacy `/margin/fee_tiers` exception, not present in the published OpenAPI spec either |
| Rate-limit/cost/tier notices, RFQ retention windows, sanity limits, exchange sharding rollout notices, fee-schedule timing announcements, FIX-session limits | **No code change** — operational/business rules with no new fields |
| WS schema corrections (`seq` on more channels, `order_source` on margin messages, `sid`/`seq` on subscription errors, retired error codes 6/16/17, `market_id`/`market_ticker` removed from the error schema, narrowed `multivariate_market_lifecycle` schema) | **No code change** — this crate's `WsEnvelope`/`WsError` already carry `sid`/`seq` generically and never modeled the removed fields |
| `FeeType::QuadraticWithComboMakerFees` (combo RFQ maker-fee multiplier) | **Added** enum variant |

### Added

- [Rust API] `exchange_index: Option<i64>` added to `Market`, `Series`, `EventData`, `MarketPosition`
  (REST + WS), `Fill` (REST + WS), `Settlement`, `Order`, `OrderGroup`, `CreateOrderGroupRequest`/
  `CreateOrderGroupResponse`/`GetOrderGroupResponse`, `MultivariateEventCollection`,
  `WsUserOrder`, and the WS `market_lifecycle_v2`/`event_lifecycle` messages. `exchange_index`
  filters added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] `Series.exchange_index`, `EventData.settlement_sources` (mirrors the existing `Series`
  field), `EventData.exchange_index`.
- [Rust API] `GetExchangeStatusResponse` gains `intra_exchange_transfers_active` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (new `ExchangeIndexStatus` type).
- [Rust API] `GetBalanceResponse.balance_breakdown: Option<Vec<IndexedBalance>>`;
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Option<Vec<IndexedBalance>>`;
  new `IndexedBalance` type shared by both.
- [Rust API] `SubaccountBalance.exchange_index` — a subaccount with funds on multiple exchange
  indexes now appears as multiple entries.
- [Rust API] New portfolio endpoints: `intra_exchange_instance_transfer`,
  `get_intra_exchange_instance_transfers`, `get_intra_exchange_instance_transfer`,
  `get_target_balance_allocation`, `set_target_balance_allocation`, `cancel_all_orders`. New types:
  `ExchangeInstance`, `IntraExchangeInstanceTransferRequest/Response`,
  `IntraExchangeInstanceTransfer`, `IntraExchangeInstanceTransferStatus`,
  `GetIntraExchangeInstanceTransfersParams/Response`, `GetIntraExchangeInstanceTransferResponse`,
  `TargetBalanceAllocation`, `GetTargetBalanceAllocationResponse`,
  `SetTargetBalanceAllocationRequest`, `RestingMarginReservation`.
- [Rust API] New account endpoints: `upgrade_account_api_usage_level`,
  `get_account_api_usage_level_volume_progress`. New types:
  `AccountApiUsageLevelVolumeGoal`, `AccountApiUsageLevelVolumeProgress`,
  `GetAccountApiUsageLevelVolumeProgressResponse`.
  `ApiKey.subaccount`, `ApiKey.api_key_region_expiration_ts`;
  `CreateApiKeyRequest.subaccount`, `GenerateApiKeyRequest.subaccount`.
- [Rust API] New endpoint `get_historical_positions` (`GetHistoricalPositionsParams`, reuses
  `GetPositionsResponse`) for `GET /historical/positions`.
- [Rust API] New endpoint `get_event_live_data` (`GetEventLiveDataParams`,
  `GetEventLiveDataResponse`, `EventLiveData`) for `GET /live_data/events/{event_ticker}`.
- [Rust API] New RFQ-scoped quote action methods: `get_quote_for_rfq`, `delete_quote_for_rfq`,
  `accept_quote_for_rfq`, `confirm_quote_for_rfq`. `GetQuotesParams` gains `min_ts`, `max_ts`,
  `user_filter`.
- [Rust API] `GetEventsParams.tickers` (CSV event-ticker filter).
- [Rust API] `UpdateOrderGroupLimitRequest.subaccount`.
- [Rust API] `GetFcmOrdersParams.client_order_ids` (CSV filter, max 100; validated alongside
  `subtrader_id`).
- [Rust API] New WebSocket channels `WsChannelV2::PythValue` and `WsChannelV2::CfbenchmarksValue5hz`,
  with message types `WsPythValue`, `WsPythUnderlyingList`, `WsCfBenchmarksValue5Hz` (+ borrowed
  `*Ref` variants). `WsSubscriptionParamsV2.underlying_tickers` seeds a `pyth_value` subscription.
  `cfbenchmarks_value_5hz` shares the existing `index_ids` / `SubscribeIndices` /
  `UnsubscribeIndices` / `Indexlist` update-action machinery with `cfbenchmarks_value`.
- [Rust API] WS `market_lifecycle_v2` gains top-level `price_ranges`, `strike_type`, `cap_strike`,
  `custom_strike` (on `metadata_updated`/`created`/`price_level_structure_updated` events);
  WS `event_lifecycle` gains `exchange_index`.
- [Rust API] WS `quote_created`/`quote_accepted`/`quote_executed` gain `subaccount`.
- [Rust API] WS `WsTrade`/`WsTradeRef` gain `is_block_trade`, matching the REST `Trade` field
  added in 0.6.0.
- [Rust API] `FeeType::QuadraticWithComboMakerFees` variant.

### Changed

- [Rust API] `GetQuotesParams` dropped `market_ticker`/`event_ticker` (removed upstream 2026-06-20).

### Removed

- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled` (+ the same field
  on WS `market_lifecycle_v2` messages), `MarketPosition.resting_orders_count` (REST + WS) — all
  gone from the live OpenAPI/AsyncAPI schemas.
- [Rust API] `EventData.available_on_brokers` — deprecated 2026-08-27, removed upstream 2026-09-10.
- [Rust API] `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was removed upstream
  2026-07-04.
- [Rust API] `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types — the
  multivariate lookup REST surface was removed upstream 2026-08-06.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsMultivariate`/`WsMultivariateRef`, `WsDataMessageV2::Multivariate`/`WsDataMessageRef::Multivariate`
  — the `multivariate` WebSocket channel (message type `multivariate_lookup`) was removed upstream
  2026-08-06. Use `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes.
- [Rust API] `ErrorResponse.service` — deprecated upstream 2026-07-28, removed 2026-08-06. Branch
  on `code` instead.

### Breaking

- [Rust API] `get_balance()` now takes a `GetBalanceParams` (`subaccount`, `exchange_index`)
  argument: `client.get_balance(GetBalanceParams::default())`.
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from `String` to `Option<String>` (at least
  one of `subtrader_id` or `client_order_ids` is now required upstream).
- [Rust API] `CreateApiKeyRequest` and `GenerateApiKeyRequest` gained a `subaccount` field;
  exhaustive struct literals must be updated (or use `..Default::default()` where available).
- [Rust API] All removals listed above are breaking for any downstream code referencing the
  removed items.

### Fixed

- [Docs] `docs/spec-parity.md` updated with the durable distinctions from this refresh (see file
  for details): `pyth_value` lacks update-action parity with `cfbenchmarks_value`'s `index_ids`
  workflow, `price_level_structure` is intentionally an open `String`, and `cadence` was announced
  but is not present in the live OpenAPI schema.


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
