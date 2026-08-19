# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-19

### Compatibility

- Docs snapshot: 2026-08-19
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-19

**Version bump rationale:** minor (0.7.0 → 0.8.0). Per [`VERSIONING.md`](VERSIONING.md),
while the crate is below `1.0.0`, "Minor releases are for any intentional breaking change to the
public Rust API." This release removes the legacy V1 order-mutation surface, the `multivariate`
WebSocket channel, `ErrorResponse.service`, several upstream-removed struct fields, and changes
three method signatures — all breaking. No major bump: upstream Kalshi churn does not by itself
imply a major, and `1.0.0` is reserved for the future stable transition.

**Changelog entries since the 0.7.0 watermark (2026-06-08) through 2026-08-19 — all 72 live
entries mapped to a diff or an explicit no-change justification:**

| # | Date | Tags | Entry | Disposition |
|---|---|---|---|---|
| 1 | 06-11 | REST/Pred/Margin | API usage volume progress endpoint | Added `get_account_api_usage_level_volume_progress()` + response types |
| 2 | 06-11 | REST/Margin | Perps mark prices on margin markets | No change — margin market types not modeled in crate |
| 3 | 06-11 | REST/Pred/Margin | Self-serve Advanced API usage tier upgrade | Added `upgrade_account_api_usage_level()` |
| 4 | 06-11 | REST/Margin | Margin fee-tier endpoint returns active rates | No change — already handled in 0.6.0 (exchange-side fix) |
| 5 | 06-11 | REST/WS/Margin | Perps volume and OI notional fields | No change — margin market types not modeled |
| 6 | 06-11 | REST/Margin | Tick size added to GET Margin Markets | No change — margin market types not modeled |
| 7 | 06-11 | REST/FIX/Pred | Fractional quantities for RFQs | No change — `contracts_fp` / `*_contracts_offered_fp` already present |
| 8 | 06-18 | REST/Pred | `settlement_sources` added to events API | Added `EventData.settlement_sources` |
| 9 | 06-18 | WS/Pred | Strike type and cap strike on `metadata_updated` | Added `strike_type`, `cap_strike`, `custom_strike` to `WsMarketLifecycleV2(Ref)` |
| 10 | 06-18 | FIX/Pred | RFQ quote identity on FIX | No change — FIX not in crate scope |
| 11 | 06-18 | FIX/Pred/Margin | Trade entries in FIX market data | No change — FIX not in crate scope |
| 12 | 06-18 | REST/Pred | Legacy order mutation endpoints deprecated | **Breaking** — removed the V1 mutation surface (see #58 note; paths absent from live OpenAPI) |
| 13 | 06-18 | REST/Pred | Event tickers filter on `GET /events` | Added `GetEventsParams.tickers` |
| 14 | 06-18 | (untagged) | Subaccount on margin positions | No change — margin position types not modeled |
| 15 | 06-18 | REST/Pred/Margin | Block-trade accept API key permissions | No change — scopes stored as `Vec<String>` already |
| 16 | 06-18 | WS/Pred | Sanity limits on orderbook subscriptions | No change — server-side quota, no field/shape change |
| 17 | 06-18 | REST/Pred | Quote time filters and pagination fix | Added `GetQuotesParams.min_ts` / `.max_ts` |
| 18 | 06-19 | REST/Pred | RFQ/quote retention window reduced | No change — retention policy only; noted in `docs/spec-parity.md` |
| 19 | 06-20 | REST/Pred | RFQ quote market/event filters removed | **Breaking** — removed `GetQuotesParams.market_ticker` / `.event_ticker` |
| 20 | 06-23 | REST/Pred | Get Quote rate-limit cost reduced | No change — rate-limit cost only |
| 21 | 06-24 | FIX/Pred | RFQ quotes support post-only on FIX | No change for FIX; REST `Quote.post_only` added (present in OpenAPI) |
| 22 | 06-25 | REST/FIX/Pred | RFQ quote retention + RFQ-scoped quote actions | Added `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`; deprecated the quote-ID-only forms |
| 23 | 06-25 | REST/Pred | API usage tier qualification halved | No change — qualification thresholds only |
| 24 | 06-25 | FIX/Pred | FIX exchange index routing | No change — FIX not in crate scope |
| 25 | 06-26 | REST/Margin | Margin risk per-market metrics limited | No change — margin risk types not modeled |
| 26 | 06-29 | REST/Margin | Margin `margin_used` omitted for portfolio positions | No change — margin position types not modeled |
| 27 | 06-30 | REST/Pred/Margin | Trade-scoped API key permissions | No change — scopes stored as `Vec<String>` already |
| 28 | 07-02 | REST/Pred | Multivariate lookup history fully deprecated | **Breaking** — removed (see #58) |
| 29 | 07-02 | REST/Margin | Margin positions include `is_portfolio` | No change — margin position types not modeled |
| 30 | 07-02 | WS/Pred | `price_ranges` on `market_lifecycle_v2` | Added `WsMarketLifecycleV2(Ref).price_ranges` |
| 31 | 07-02 | REST/Pred | Per-index exchange status | Added `intra_exchange_transfers_active` + `exchange_index_statuses` / `ExchangeIndexStatus` |
| 32 | 07-02 | REST/Pred | Per-index subaccount balances | **Breaking** — added required `SubaccountBalance.exchange_index` |
| 33 | 07-02 | FIX/Pred | AcceptQuote reject reasons on FIX | No change — FIX not in crate scope |
| 34 | 07-02 | FIX/Pred | FIX rejects for cancel/replace failures | No change — FIX not in crate scope |
| 35 | 07-02 | REST/FIX/Pred | Sub-account-restricted API keys | Added `subaccount` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest` |
| 36 | 07-04 | REST/Pred | Exchange announcements endpoint removed | **Breaking** — removed `get_exchange_announcements()` and announcement types |
| 37 | 07-09 | FIX/Pred/Margin | FIX Tag 2446 on Incremental Refresh | No change — FIX not in crate scope |
| 38 | 07-09 | REST/Pred | RFQ-scoped quote lookup endpoint | Added `get_rfq_quote()`; deprecated `get_quote()` |
| 39 | 07-09 | REST/Pred | Deprecated REST schema fields removed | **Breaking** — removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| 40 | 07-09 | REST/Margin | Margin orders identify system order reasons | No change — margin order types not modeled |
| 41 | 07-22 | REST/Pred | Incentive programs on hidden events excluded | No change — server-side filtering, no shape change |
| 42 | 07-23 | REST/Pred/Margin | Order groups limited to 25,000 | No change — superseded by #69; server-side quota |
| 43 | 07-23 | REST/Pred | Historical positions endpoint | Added `get_historical_positions()` + `GetHistoricalPositionsParams`; added `market_positions_last_updated_ts` to cutoff response |
| 44 | 07-23 | WS/Pred/Margin | Subaccount-restricted keys can open WS sessions | No change — entry states "No new fields are introduced" |
| 45 | 07-23 | FIX/Pred | Subaccount-restricted keys quote on FIX | No change — FIX not in crate scope |
| 46 | 07-23 | WS/Pred | Pyth value WebSocket channel | Added full `pyth_value` channel support (types, msg types, wire/envelope routing, subscription actions) |
| 47 | 07-23 | REST/WS/Pred | New price level structures | No change — `price_level_structure` modeled as `Option<String>`; `price_ranges` is the source of truth and is already exposed |
| 48 | 07-28 | REST/Pred/Margin | `service` field deprecated | Superseded by #63 |
| 49 | 07-30 | REST/Pred | Richer combo-validation errors | No change — `ErrorResponse.message` / `.details` already modeled |
| 50 | 07-30 | WS/Pred | Lifecycle creation messages include `exchange_index` | Added `exchange_index` to `WsMarketLifecycleV2(Ref)` and `WsEventLifecycle(Ref)` |
| 51 | 07-30 | REST/Pred | Series responses include `exchange_index` | Added `Series.exchange_index` (and `EventData.exchange_index` per OpenAPI) |
| 52 | 07-30 | REST/Pred | New endpoint for event-keyed live data | Added `get_event_live_data()` + `EventLiveData` / `GetEventLiveDataResponse` / `GetEventLiveDataParams` |
| 53 | 07-30 | REST/Pred | Subaccount keys can read order queue positions | No change — `GetOrderQueuePositionsParams.subaccount` already present |
| 54 | 07-30 | REST/Pred | Event `product_metadata` includes `cadence` | Added `EventMetadata.cadence` |
| 55 | 07-30 | REST/Pred | Subaccount keys can use batch order endpoints | No change — V2 batch request orders already carry `subaccount` |
| 56 | 07-30 | WS/Pred | Subaccount on `quote_created` | Added `subaccount` (and `rfq_creator_id`) to `WsQuoteCreated(Ref)`; `subaccount` also added to `WsQuoteAccepted(Ref)` / `WsQuoteExecuted(Ref)` per AsyncAPI |
| 57 | 07-30 | REST/Pred/Margin | Subaccount keys can manage order groups | No change — order-group endpoints already accept `SubaccountQueryParams` |
| 58 | 08-06 | REST/WS/Pred | Multivariate lookup endpoint and channel removed | **Breaking** — removed `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate*` types, and the REST `PUT/GET .../lookup` methods and types |
| 59 | 08-06 | FIX/Pred/Margin | FIX execution reports identify exchange index | No change — FIX not in crate scope |
| 60 | 08-06 | REST/Margin | Sided leverage estimates on margin markets | No change — margin market types not modeled |
| 61 | 08-06 | REST/Pred | Order group limit updates support subaccounts | **Breaking** — `update_order_group_limit()` gained a `UpdateOrderGroupLimitParams` query argument |
| 62 | 08-06 | REST/Pred | Multivariate collections include `exchange_index` | Added `MultivariateEventCollection.exchange_index` |
| 63 | 08-06 | REST/Pred/Margin | `service` removed from error responses | **Breaking** — removed `ErrorResponse.service` |
| 64 | 08-13 | REST/WS/FIX/Pred | New `center_deci_edge_centi_cent` structure | No change — see #47; no new fields or message formats |
| 65 | 08-13 | REST/Pred | Balance reads scoped by `exchange_index` | **Breaking** — `get_balance()` now takes `GetBalanceParams`; added `GetBalanceResponse.balance_breakdown` / `IndexedBalance` |
| 66 | 08-13 | WS/Pred | Block trade indicator for WebSocket trades | Added `WsTrade(Ref).is_block_trade` |
| 67 | 08-13 | REST/Pred | Exchange shard descriptions | Included as `ExchangeIndexStatus.description` (see #31) |
| 68 | 08-13 | REST/Margin | Margin order groups bind to single `exchange_index` | No change — margin order-group types not modeled |
| 69 | 08-13 | REST/Pred | Order group maximum increased to 100,000 | No change — server-side quota only |
| 70 | 08-13 | FIX/Pred | Richer combo-validation errors on FIX RFQ | No change — FIX not in crate scope |
| 71 | 08-13 | REST/Pred | Intra-account transfer history endpoints | Added `get_intra_exchange_instance_transfers()`, `get_intra_exchange_instance_transfer()`, `create_intra_exchange_instance_transfer()`, pager/stream helpers, and the transfer types |
| 72 | 08-16 | REST/Pred | API key location attestation expiry | Added `GetApiKeysResponse.api_key_region_expiration_ts` |

Thirteen further changelog entries are dated after 2026-08-19 (2026-08-20 through 2026-08-27) and
are **not** covered by this watermark: Kalshi Weather Index endpoint, tapered sub-cent pricing on
combo markets, exchange sharding, combo RFQ fee assignment, FIX entry timestamps, cross-shard
subaccount transfers, target balance allocation endpoints, resting-order value breakdown by
exchange index, exchange index on portfolio records, exchange index filters for portfolio lists,
RFQs/combo creation for sub-account-restricted keys, optional balance reads by `exchange_index`,
and exit triggers on margin positions. Two of these are already partly reflected because the live
OpenAPI ships their shapes today: `IntraExchangeInstanceTransferRequest.source_subaccount` /
`.destination_subaccount` (cross-shard subaccount transfers) and `Order.exchange_index`.

### Added

- [Rust API] Added `get_account_api_usage_level_volume_progress()` and
  `GetAccountApiUsageLevelVolumeProgressResponse` / `AccountApiUsageLevelVolumeProgress` /
  `AccountApiUsageLevelVolumeGoal` for `GET /account/api_usage_level/volume_progress`.
- [Rust API] Added `upgrade_account_api_usage_level()` for `POST /account/api_usage_level/upgrade`.
- [Rust API] Added intra-exchange-instance transfer support:
  `create_intra_exchange_instance_transfer()`, `get_intra_exchange_instance_transfers()`,
  `get_intra_exchange_instance_transfer()`, plus `intra_exchange_instance_transfers_pager()` /
  `stream_intra_exchange_instance_transfers()` and the types
  `IntraExchangeInstanceTransferRequest`, `IntraExchangeInstanceTransferResponse`,
  `IntraExchangeInstanceTransfer`, `IntraExchangeInstanceTransferStatus`,
  `GetIntraExchangeInstanceTransfersParams`, `GetIntraExchangeInstanceTransfersResponse`,
  `GetIntraExchangeInstanceTransferResponse`.
- [Rust API] Added `get_historical_positions()` and `GetHistoricalPositionsParams` for
  `GET /historical/positions`, plus `GetHistoricalCutoffResponse.market_positions_last_updated_ts`.
- [Rust API] Added `get_event_live_data()` with `EventLiveData`, `GetEventLiveDataResponse`, and
  `GetEventLiveDataParams` for `GET /live_data/events/{event_ticker}`.
- [Rust API] Added the RFQ-scoped quote endpoints `get_rfq_quote()`, `delete_rfq_quote()`,
  `accept_rfq_quote()`, and `confirm_rfq_quote()`.
- [Rust API] Added full `pyth_value` WebSocket channel support: `WsChannelV2::PythValue`,
  `WsMsgType::PythValue` / `PythValueUnderlyingList`, the `WsPythValue` / `WsPythValueRef` /
  `WsPythUnderlyingList` / `WsPythUnderlyingListRef` message types, `WsDataMessageV2::PythValue` /
  `PythValueUnderlyingList` (and the borrowed equivalents) routed through both the wire and
  envelope parse paths, `underlying_tickers` on `WsSubscriptionParamsV2` /
  `WsUpdateSubscriptionParamsV2`, and the `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` actions with matching `validate_update` rules and
  resubscribe tracking.
- [Rust API] Added `ExchangeIndexStatus` plus `GetExchangeStatusResponse.intra_exchange_transfers_active`
  and `.exchange_index_statuses`.
- [Rust API] Added `IndexedBalance` and `GetBalanceResponse.balance_breakdown`.
- [Rust API] Added `CreateSubaccountRequest` so subaccounts can be created on a chosen exchange index.
- [Rust API] Added `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`).
- [Rust API] New struct fields: `Series.exchange_index`; `EventData.settlement_sources`,
  `.fee_type_override`, `.fee_multiplier_override`, `.exchange_index`; `EventMetadata.cadence`;
  `GetEventsParams.tickers`; `MultivariateEventCollection.exchange_index`;
  `MarketPosition.exchange_index`; `Order.exchange_index`; `GetOrdersParams.exchange_index`;
  `ApiKey.subaccount`; `CreateApiKeyRequest.subaccount`; `GenerateApiKeyRequest.subaccount`;
  `GetApiKeysResponse.api_key_region_expiration_ts`; `Quote.post_only`, `.creator_subaccount`,
  `.rfq_creator_subaccount`; `RFQ.creator_subaccount`; `GetQuotesParams.min_ts`, `.max_ts`,
  `.user_filter`; `GetRFQsParams.user_filter`.
- [Rust API] New WebSocket fields: `WsMarketLifecycleV2(Ref).exchange_index`, `.price_ranges`,
  `.strike_type`, `.cap_strike`, `.custom_strike`; `WsEventLifecycle(Ref).exchange_index`;
  `WsTrade(Ref).is_block_trade`; `WsQuoteCreated(Ref).subaccount` and `.rfq_creator_id`;
  `WsQuoteAccepted(Ref).subaccount` and `.rfq_creator_id`; `WsQuoteExecuted(Ref).subaccount`.
- [Tests] Added deterministic coverage for the new exchange-status, series/event, WebSocket
  lifecycle strike/price-range, block-trade, `quote_created` subaccount, Pyth message, and Pyth
  subscription-validation behavior.

### Fixed

- [Rust API] `SeriesFeeChange.fee_multiplier` was `i64` but the OpenAPI types it
  `number`/`double`. `serde_json` rejects any float token for an integer field, so
  `GET /series/fee_changes` failed to parse for any fractional multiplier (and even for `1.0`
  written with a decimal point). Now `f64`, matching `Series.fee_multiplier`.
- [Rust API] `SeriesFeeChange.scheduled_ts` was `i64` but the OpenAPI types it
  `string`/`date-time`, so an ISO-8601 value could never deserialize. Now a `String` read
  leniently from string or number.
- [Rust API] Added the missing `Fill.exchange_index`. The OpenAPI marks it required on `Fill`, and
  `Fill` has no `#[serde(flatten)] extra`, so the value was being silently discarded on every
  deserialize.
- [Rust API] `WsMarketLifecycleAdditionalMetadata.custom_strike` (and the `Ref` equivalent) was
  `Option<BTreeMap<String, String>>`, which would fail to parse any non-string value. The
  AsyncAPI types it as an unconstrained `object`, so it is now `Option<Map<String, Value>>`,
  matching the (already correct) top-level `WsMarketLifecycleV2.custom_strike`.

### Changed

- [Rust API] `SeriesFeeChange.id` is now a `String` (deserialized leniently from string or number)
  to match the OpenAPI `type: string`.
- [Rust API] `GetQuotesParams.quote_creator_user_id` / `.rfq_creator_user_id` and
  `GetRFQsParams.creator_user_id` are marked `#[deprecated]`, matching `deprecated: true` in the
  OpenAPI. Use `user_filter` / `rfq_user_filter` instead.
- [Rust API] `EventData.category` is marked `#[deprecated]` (use the series-level category).
- [Rust API] The quote-ID-only quote actions (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) are marked `#[deprecated]` in favor of their RFQ-scoped counterparts.

### Removed

- [Rust API] Removed the legacy V1 order-mutation surface: `create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`, and the
  `CreateOrderRequest` / `CreateOrderResponse` / `CancelOrderParams` / `CancelOrderResponse` /
  `AmendOrderRequest` / `AmendOrderResponse` / `DecreaseOrderRequest` / `DecreaseOrderResponse` /
  `BatchCreateOrders*` / `BatchCancelOrders*` types. These paths no longer exist in the live
  OpenAPI; use the V2 event-order methods added in 0.6.0.
- [Rust API] Removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, and `AnnouncementStatus`.
- [Rust API] Removed the `multivariate` WebSocket channel: `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`, `WsMsgType::MultivariateLookup`, `WsDataMessageV2::Multivariate`
  (and the borrowed equivalent), and the `WsMultivariate` / `WsMultivariateRef` /
  `WsMultivariateSelectedMarket` / `WsMultivariateSelectedMarketRef` types. Use
  `multivariate_market_lifecycle` instead.
- [Rust API] Removed the multivariate lookup REST surface:
  `get_multivariate_event_collection_lookup_history()`,
  `lookup_tickers_for_market_in_multivariate_event_collection()`, and their request/response types.
- [Rust API] Removed `ErrorResponse.service`.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count`.
- [Rust API] Removed `WsMarketLifecycleV2.fractional_trading_enabled` (and the `Ref` equivalent)
  plus `WsMarketLifecycleEventType::FractionalTradingUpdated`. Neither `fractional_trading` field
  nor the `fractional_trading_updated` event value appears anywhere in the live AsyncAPI. The
  enum's `#[serde(other)] Unknown` catch-all means parsing is unaffected.
- [Rust API] Removed `GetQuotesParams.market_ticker` and `GetQuotesParams.event_ticker`.
- [Rust API] Removed the unused `MarketPositionRef` / `EventPositionRef` borrowed views, which
  mirrored REST position shapes not carried on any WebSocket channel.

### Breaking

- [Rust API] `get_balance()` now takes a `GetBalanceParams` argument. Pass
  `GetBalanceParams::default()` to preserve the previous behavior.
- [Rust API] `update_order_group_limit()` now takes a `UpdateOrderGroupLimitParams` query argument
  before the body. Pass `UpdateOrderGroupLimitParams::default()` to preserve previous behavior.
- [Rust API] `create_subaccount()` now takes a `CreateSubaccountRequest`. Pass
  `CreateSubaccountRequest::default()` to preserve previous behavior.
- [Rust API] `SubaccountBalance.exchange_index` is a new required field; payloads without it no
  longer deserialize, and struct-literal construction must supply it.
- [Rust API] `SeriesFeeChange.id` changed from `i64` to `String`, `.scheduled_ts` from `i64` to
  `String`, and `.fee_multiplier` from `i64` to `f64`.
- [Rust API] `Fill` gained an `exchange_index` field; struct-literal construction must supply it.
- [Rust API] `WsMarketLifecycleAdditionalMetadata.custom_strike` (and the `Ref` equivalent) changed
  from `Option<BTreeMap<String, String>>` to `Option<serde_json::Map<String, serde_json::Value>>`.
- [Rust API] `WsUpdateAction` gained `SubscribeUnderlyings`, `UnsubscribeUnderlyings`, and
  `UnderlyingList`; `WsChannelV2` gained `PythValue`; `WsMsgType` gained `PythValue` and
  `PythValueUnderlyingList`; `WsDataMessageV2` / `WsDataMessageRef` gained `PythValue` and
  `PythValueUnderlyingList`. Downstream exhaustive matches must handle the new variants.
- [Rust API] `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2` gained an
  `underlying_tickers` field; struct-literal construction must supply it (or use `..Default::default()`).
- [Rust API] `WsTrade` / `WsTradeRef` gained a non-`Option` `is_block_trade: bool` field
  (`#[serde(default)]`, so parsing is unaffected); struct-literal construction must supply it.
- [Rust API] `WsFill.purchased_side` / `WsFillRef.purchased_side` changed from `YesNo` to
  `Option<YesNo>`. The AsyncAPI still marks it required, but it carries the same deprecation
  notice as `side` / `action` (which the crate already models as `Option`), so leaving it
  non-`Option` would make the whole `fill` message unparseable the day Kalshi drops it.
  Use `outcome_side` / `book_side`.
- [Rust API] All items listed under **Removed** above are breaking for downstream code that
  referenced them.


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
