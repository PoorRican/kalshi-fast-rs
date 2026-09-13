# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-13

### Compatibility

- Docs snapshot: 2026-09-13
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-17

**Bump rationale (`VERSIONING.md`, pre-1.0 rules):** minor (0.7.0 → 0.8.0). Several upstream
field/endpoint removals force breaking Rust API changes (removed public struct fields, removed
methods and types, a changed method signature) — per `VERSIONING.md` "Bump minor if deprecated or
removed upstream fields/endpoints force a breaking Rust API change... or if public types or method
names change."

**Changelog entries since the 2026-06-08 watermark and disposition** (90 entries tagged
`Predictions` + `REST`/`WebSocket`; a further 31 entries tagged only `FIX` and/or `Margin` are
out of this crate's scope — this crate models the Predictions exchange's REST and WebSocket
surface only, per `CLAUDE.md` — and are omitted from the table below with no code change):

| Date | Entry | Disposition |
|---|---|---|
| 06-11 | API usage volume progress endpoint | Added `get_account_api_usage_level_volume_progress` |
| 06-11 | Self-serve Advanced API usage tier upgrade | Added `upgrade_account_api_usage_level` |
| 06-11 | Fractional quantities for RFQs | No change — `contracts_fp` already present |
| 06-18 | `settlement_sources` added to the events API | Added `EventData.settlement_sources` |
| 06-18 | Strike type and cap strike on `metadata_updated` | Added `WsMarketLifecycleV2.{strike_type,cap_strike,custom_strike}` |
| 06-18 | Legacy order mutation endpoints deprecated | No change — operational deprecation notice only |
| 06-18 | Event tickers filter on `GET /events` | Added `GetEventsParams.{tickers,min_updated_ts}` |
| 06-18 | Block-trade accept API key permissions | No change — scopes already raw `Vec<String>` |
| 06-18 | Sanity limits on orderbook subscriptions | No change — operational rate-limit only |
| 06-18 | Quote time filters and pagination fix | Added `GetQuotesParams.{min_ts,max_ts}` |
| 06-19 | Quote/RFQ retention window reduced | No change — operational retention window only |
| 06-20 | RFQ quote market/event filters removed | **Breaking** — removed `GetQuotesParams.{market_ticker,event_ticker}` |
| 06-23 | Get Quote rate-limit cost reduced | No change — operational rate-limit only |
| 06-25 | RFQ quote retention and RFQ-scoped quote actions | Added `{get,delete,accept,confirm}_rfq_quote`; deprecated the quote-ID-only equivalents |
| 06-25 | API usage tier qualification halved | No change — business rule only |
| 06-30 | Trade-scoped API key permissions | No change — scopes already raw `String` |
| 07-02 | Multivariate lookup history fully deprecated | No change at announcement; see 08-06 removal |
| 07-02 | `price_ranges` on `market_lifecycle_v2` | Added `WsMarketLifecycleV2.price_ranges` |
| 07-02 | Per-index exchange status | Added `GetExchangeStatusResponse.{intra_exchange_transfers_active,exchange_index_statuses}` |
| 07-02 | Per-index subaccount balances | Added `SubaccountBalance.exchange_index` |
| 07-02 | Sub-account-restricted API keys | Added `subaccount` to `{Create,Generate}ApiKeyRequest`/`ApiKey` |
| 07-04 | Exchange announcements endpoint removed | **Breaking** — removed `get_exchange_announcements` and `Announcement*` types |
| 07-09 | RFQ-scoped quote lookup endpoint | Added `get_rfq_quote` (see 06-25) |
| 07-09 | Deprecated Predictions REST fields removed | **Breaking** — removed `Market.{response_price_units,fractional_trading_enabled}`, `MarketPosition.resting_orders_count` |
| 07-22 | Hidden-event incentive programs excluded | No change — visibility rule only |
| 07-23 | Order groups limited to 25,000/user | No change — business rule only (superseded 08-13) |
| 07-23 | Historical positions endpoint | Deferred — new endpoint, not modeled (see `docs/spec-parity.md`) |
| 07-23 | Subaccount-restricted keys can open WS sessions | No change — authorization behavior only |
| 07-23 | `pyth_value` WebSocket channel | Deferred — new channel, not modeled |
| 07-23 | New price level structures | No change — `price_level_structure` already raw `String` |
| 07-28 | `service` error field deprecated | No change at announcement; see 08-06 removal |
| 07-30 | Richer combo-validation errors | No change — `message`/`details` already generic `String` |
| 07-30 | Lifecycle creation messages include `exchange_index` | Added to `WsMarketLifecycleV2`/`WsEventLifecycle` |
| 07-30 | Series responses include `exchange_index` | Added `Series.exchange_index` |
| 07-30 | New event-keyed live data endpoint | Deferred — new endpoint, not modeled |
| 07-30 | Subaccount-restricted keys can read queue positions | No change — authorization behavior only |
| 07-30 | `product_metadata.cadence` | Added `EventMetadata.cadence` |
| 07-30 | Subaccount-restricted keys can batch orders | No change — authorization behavior only |
| 07-30 | Subaccount on `quote_created` | Added `WsQuoteCreated.subaccount` |
| 07-30 | Subaccount-restricted keys can manage order groups | No change — authorization behavior only |
| 08-06 | Multivariate lookup endpoint + channel removed | **Breaking** — removed lookup REST endpoints, `WsChannelV2::Multivariate`, `WsMultivariate*` |
| 08-06 | Order group limit updates support subaccount | Added `UpdateOrderGroupLimitRequest.subaccount` |
| 08-06 | Multivariate collections include `exchange_index` | Added `MultivariateEventCollection.exchange_index` |
| 08-06 | `service` error field removed | **Breaking** — removed `ErrorResponse.service` |
| 08-13 | New `center_deci_edge_centi_cent` structure | No change — raw `String` |
| 08-13 | Balance reads scoped by `exchange_index` | Added `GetBalanceParams`, `GetBalanceResponse.balance_breakdown` |
| 08-13 | Block trade indicator on WS trades | Added `WsTrade.is_block_trade` |
| 08-13 | Exchange shard descriptions | Added `ExchangeIndexStatus.description` |
| 08-13 | Order group max raised to 100,000 | No change — business rule only |
| 08-13 | Intra-account transfer history endpoints | Deferred — new endpoints, not modeled |
| 08-16 | API key location attestation expiry | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| 08-20 | VPC peering for Prime members | No change — connectivity/infra, not modeled |
| 08-20 | Kalshi Weather Index endpoint | Deferred — new endpoint, not modeled |
| 08-20 | Maker fee exemption for NFL combos | No change — fee business rule only |
| 08-20 | Cross-shard subaccount transfers | Deferred — new endpoint, not modeled |
| 08-20 | Target balance allocation endpoints | Deferred — new endpoints, not modeled |
| 08-20 | Resting order value breakdown by index | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| 08-20 | `exchange_index` on fills | Added `Fill.exchange_index`, `WsFill.exchange_index` |
| 08-20 | `exchange_index` filters for portfolio lists | Added to `GetPositionsParams`/`GetFillsParams`/`GetSettlementsParams`/`GetOrdersParams` |
| 08-20 | RFQs/combos for sub-account-restricted keys | No change — authorization behavior only |
| 08-20 | Optional balance reads by `exchange_index` | Covered by 08-13 `GetBalanceParams` |
| 08-22 | Post-only quotes preserved | No change — planned removal cancelled |
| 08-22 | Combo RFQ fee assignment for resting orders | No change — fee business rule only |
| 08-24 | Upcoming exchange sharding (announcement) | No change — operational announcement |
| 08-27 | Localized market content (`Accept-Language`) | No change — same `Market` shape, translated content |
| 08-27 | `exchange_index` on `user_orders` | Added `WsUserOrder.exchange_index` |
| 08-27 | Cancel-all-orders endpoints | Added `cancel_all_orders` |
| 08-27 | CF Benchmarks REST passthrough history | No change — docs clarification of existing endpoint |
| 08-27 | `available_on_brokers` deprecated | No change at announcement; see 09-10 removal |
| 08-27 | Exchange auto-routing enabled by default | No change — server-side routing behavior only |
| 08-29 | Structured target image URLs | No change — captured by existing `extra` passthrough |
| 08-31 | Weather index calibration history endpoint | Deferred — new endpoint, not modeled |
| 09-03 | `cfbenchmarks_value_5hz` WebSocket channel | Deferred — new channel, not modeled |
| 09-03 | FCM orders `client_order_ids` filter | No change — FCM subtrader endpoints not modeled |
| 09-03 | Historical positions `subaccount` filter | No change — historical positions endpoint not modeled |
| 09-03 | Correct `remaining_count` after crossing amend | No change — behavior fix, field already modeled |
| 09-03 | Lower rate-limit cost for cancel-all | No change — operational rate-limit only |
| 09-03 | Shard rebalance margin reservation | No change — `target_balance_allocation` not modeled |
| 09-03 | Tapered sub-cent pricing on combos | No change — `price_level_structure` already raw `String` |
| 09-10 | `available_on_brokers` removed | **Breaking** — removed `EventData.available_on_brokers` |
| 09-10 | Principal-only sizing for target-cost RFQs | Added `CreateRFQRequest.target_cost_excludes_fees` |
| 09-10 | Weather index `receipt_basis` | No change — weather index endpoint not modeled |
| 09-10 | Sharding for commodities/basketball (announcement) | No change — operational announcement |
| 09-10 | `center_deci_edge_centi_cent` emitted again | No change — server-side bug fix, raw `String` |
| 09-10 | WebSocket schemas corrected (docs-only) | No change — documents existing wire-level `sid`/`seq` behavior already modeled |
| 09-17 | Target balance allocations include reservation policy | No change — `target_balance_allocation` not modeled |
| 09-17 | Series responses include `categories` | Added `Series.categories` |
| 09-17 | Returning to idiomatic MVE series (ticker naming) | No change — naming convention only |
| 09-17 | WS subscriptions ready when acknowledged | No change — server-side timing fix, no schema change |
| 09-17 | RFQ/quote writes share shard-1 budget | No change — operational rate-limit only |

### Breaking

- [Rust API] Removed fields (no longer present on the public structs): `Market.response_price_units`,
  `Market.fractional_trading_enabled`, the matching `market_lifecycle_v2` field,
  `MarketPosition.resting_orders_count`, `EventData.available_on_brokers`, `ErrorResponse.service`.
- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` (endpoint removed upstream 2026-07-04).
- [Rust API] Removed `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types
  (`PUT`/`GET .../lookup` removed upstream 2026-08-06).
- [Rust API] Removed `WsChannelV2::Multivariate`, the `multivariate`/`multivariate_lookup`
  `WsMsgType` variants, and `WsMultivariate`/`WsMultivariateRef` (WebSocket `multivariate` channel
  removed upstream 2026-08-06; distinct from `multivariate_market_lifecycle`, which is unaffected).
- [Rust API] Removed `GetQuotesParams.{market_ticker,event_ticker}` (filters removed upstream
  2026-06-20; filter by `rfq_id`, user, status, or update time instead).
- [Rust API] `KalshiRestClient::get_balance` now takes a `GetBalanceParams` argument
  (`exchange_index`, `subaccount`) instead of no arguments.

### Added

- [Rust API] `exchange_index: Option<i64>` added across the REST and WebSocket surface
  (`Market`, `EventData`, `Series`, `MultivariateEventCollection`, `MarketPosition`, `Settlement`,
  `Fill`, `WsFill`, `SubaccountBalance`, `WsMarketLifecycleV2`, `WsEventLifecycle`, `WsUserOrder`)
  tracking Kalshi's 2026-07/09 exchange sharding rollout. `GetPositionsParams`, `GetFillsParams`,
  `GetSettlementsParams`, and `GetOrdersParams` gained an `exchange_index` filter.
  `GetExchangeStatusResponse` gained `intra_exchange_transfers_active` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>`.
- [Rust API] `GetBalanceParams { exchange_index, subaccount }` for `get_balance`;
  `GetBalanceResponse.balance_breakdown` and
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`, both
  `Option<Vec<IndexedBalance>>`.
- [Rust API] `WsMarketLifecycleV2` gained top-level `strike_type`, `cap_strike`, `custom_strike`
  (present only on `metadata_updated` events, alongside the pre-existing `floor_strike` /
  `yes_sub_title`) and `price_ranges: Option<Vec<PriceRange>>` (present on `created` and
  `price_level_structure_updated` events).
- [Rust API] `WsTrade`/`WsTradeRef` gained `is_block_trade: Option<bool>`.
- [Rust API] `WsQuoteCreated`/`WsQuoteCreatedRef` gained `subaccount: Option<u32>`.
- [Rust API] `EventData` gained `settlement_sources: Vec<SettlementSource>`; `EventMetadata` gained
  `cadence: Option<String>`; `Series` gained `categories: Vec<String>`.
- [Rust API] `GetEventsParams` gained `tickers` and `min_updated_ts`. `GetQuotesParams` gained
  `min_ts`, `max_ts`, and `user_filter`. `CreateRFQRequest` gained
  `target_cost_excludes_fees: Option<bool>`. `UpdateOrderGroupLimitRequest` gained `subaccount`.
  `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey` gained `subaccount`.
  `GetApiKeysResponse` gained `api_key_region_expiration_ts`.
- [Rust API] New RFQ-scoped quote action methods: `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`. The quote-ID-only equivalents (`get_quote`,
  `delete_quote`, `accept_quote`, `confirm_quote`) are now `#[deprecated]`, matching Kalshi's own
  deprecation of those endpoints.
- [Rust API] New endpoints: `cancel_all_orders` (`DELETE /portfolio/events/orders`),
  `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level`.
- [Rust API] `FeeType` gained `QuadraticWithComboMakerFees`.
- [Tests] Added/extended coverage in `tests/parsing.rs` and inline WebSocket message tests for all
  of the above field additions and removals.
- [CI] Fixed a pre-existing test-only compile break in `envelope.rs`'s `ListSubscriptions` match
  arms (missing `sid`/`seq` destructuring), unrelated to this refresh's upstream changes.

### Deprecated

- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` — use the RFQ-scoped
  equivalents added above. Kalshi no longer guarantees quotes are queryable by quote ID alone
  outside a post-acceptance state.

### Docs

- [Docs] `docs/spec-parity.md` documents every distinction introduced by this refresh, including a
  "Known Gaps" section listing genuinely new upstream capabilities deliberately not modeled this
  pass (`cfbenchmarks_value_5hz`, `pyth_value`, historical positions, target balance allocation,
  intra-exchange transfers, live-data/weather endpoints, FCM order filtering).


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
