# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-20

Spec reconciliation against OpenAPI 3.28.0 / AsyncAPI 2.0.0, covering the 87
changelog entries published between the previous watermark (2026-06-08) and
2026-08-20.

**Version bump rationale.** `VERSIONING.md` (pre-1.0): *"Minor releases are for
any intentional breaking change to the public Rust API"*, and *"Bump `minor` if
deprecated or removed upstream fields/endpoints force a breaking Rust API
change."* Upstream removed six order-mutation endpoints, the exchange
announcements endpoint, the multivariate lookup endpoint and WebSocket channel,
and several response fields — all of which force source-breaking Rust changes.
Minor bump: **0.7.0 → 0.8.0**.

### Compatibility

- Docs snapshot: 2026-08-20
- OpenAPI: 3.28.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-20

### Upstream changelog disposition

All 87 entries after the 2026-06-08 watermark. Entries the crate does not model
are grouped; every other entry maps to a concrete diff or an explicit
justification.

**Out of scope — no change needed (22 entries).** The crate models the
Predictions REST and WebSocket surface only.

- *Margin exchange (11)*: margin fee-tier active rates; perps mark prices; perps
  volume/OI notional fields; tick size on margin markets; subaccount on margin
  positions; margin risk per-market metric limits; `margin_used` omitted for
  jointly-margined positions; `is_portfolio` flag; margin order `order_reason`;
  sided leverage estimates; margin order groups bound to one `exchange_index`;
  exit triggers on margin positions. The crate has no `/margin/*` market,
  position, risk, or order types.
- *FIX protocol (11)*: RFQ quote identity; trade entries in FIX market data;
  post-only on FIX quotes; FIX exchange index routing; AcceptQuote reject
  reasons; cancel/replace reject reasons; Tag 2446; subaccount-restricted RFQ
  sessions; `LastMkt<30>` on execution reports; combo-validation rejects;
  `MDEntryDate`/`MDEntryTime`. The crate has no FIX transport.

**Operational / server-side only — no schema change (15 entries).**

| Entry | Justification |
|---|---|
| 2026-06-18 Sanity limits on orderbook subscriptions | Session limits enforced server-side; no message or field change |
| 2026-06-18 Block-trade accept API key permissions | New scope strings only; scopes modeled as `Vec<String>` |
| 2026-06-19 RFQ/quote retention reduced to 7 days | Retention policy; no schema change |
| 2026-06-23 Get Quote cost reduced to 2 tokens | Token cost only; surfaced via `get_account_endpoint_costs` |
| 2026-06-25 API usage tier qualification halved | Server-side thresholds |
| 2026-06-30 Trade-scoped API key permissions | New `write::trade` scope string only |
| 2026-07-22 Incentive programs on hidden events excluded | Server-side filtering of an existing response |
| 2026-07-23 Order groups limited to 25,000 per user | Server-side limit |
| 2026-07-23 Subaccount-restricted keys can open WebSocket sessions | Changelog states "No new fields are introduced" |
| 2026-07-30 Subaccount-restricted keys: order queue positions, batch orders, order groups | Authorization scope only; `subaccount` params already modeled |
| 2026-08-13 Order group maximum raised to 100,000 | Server-side limit |
| 2026-08-20 VPC peering for Prime members | Connectivity/infrastructure |
| 2026-08-20 Maker fee exemption for independent NFL combo markets | Fee policy; no schema change |
| 2026-08-24 Upcoming exchange sharding | Announcement; the `exchange_index` fields it implies are covered by their own entries |
| 2026-08-27 Tapered sub-cent pricing on combo markets | Changelog: "No API fields or message formats change" — consume `price_ranges` |

**Mapped to concrete diffs (50 entries).** See the sections below; each bullet
names the originating changelog date.

### Added

- [Rust API] `GET /exchange/status` gained `intra_exchange_transfers_active` and
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` (2026-07-02), each entry
  carrying `exchange_index`, `description` (2026-08-13), `exchange_active`,
  `trading_active`, and `intra_exchange_transfers_active`.
- [Rust API] `Series::exchange_index` (2026-07-30).
- [Rust API] `get_event_live_data()` for `GET /live_data/events/{event_ticker}`
  with an optional `range` filter, plus `EventLiveData` /
  `GetEventLiveDataResponse` / `GetEventLiveDataParams` (2026-07-30).
- [Rust API] `get_weather_index()` for `GET /live_data/weather/{city}`, plus
  `GetWeatherIndexParams`, `GetWeatherIndexResponse`, `WeatherIndexPoint`, and
  `WeatherIndexStationReading` (2026-08-20). `WeatherIndexPoint::v` is
  `Option<f64>` — absent, not `0.0`, on `incomplete` points.
- [Rust API] `EventData::settlement_sources` (2026-06-18) and
  `GetEventsParams::tickers` (2026-06-18).
- [Rust API] `EventProductMetadata` with `cadence` (2026-07-30);
  `EventData::product_metadata` now uses it.
- [Rust API] `FeeType::QuadraticWithComboMakerFees` (2026-08-22).
- [Upstream] `GetSeriesListParams::min_updated_ts`,
  `GetMilestonesParams::min_updated_ts`, `GetEventsParams::min_updated_ts`, and
  `GetIncentiveProgramsParams::incentive_description` — spec'd filters the crate
  was missing.
- [Rust API] `IncentiveProgram::incentive_description` (spec-required).
- [Rust API] `pyth_value` WebSocket channel (2026-07-23): `WsChannelV2::PythValue`,
  `WsMsgType::{PythValue, PythUnderlyingList}`, `WsPythValue` / `WsPythUnderlyingList`
  (and borrowed twins), `underlying_tickers` on both subscription param structs, and
  `WsUpdateAction::{SubscribeUnderlyings, UnsubscribeUnderlyings, UnderlyingList}`.
  `validate_update` rejects mixing underlying actions with market targets and
  requires `underlying_tickers` for the add/remove actions.
- [Rust API] `market_lifecycle_v2` gained `strike_type`, `cap_strike`, and
  `custom_strike` (2026-06-18), `price_ranges` with the new `WsPriceRange` type
  (2026-07-02), and `exchange_index` (2026-07-30, also on `event_lifecycle`).
- [Rust API] `WsTrade::is_block_trade` (2026-08-13) and `WsFill::exchange_index`
  (2026-08-20).
- [Rust API] `subaccount` on `WsQuoteCreated` (2026-07-30) — and on
  `WsQuoteAccepted` / `WsQuoteExecuted`, plus `rfq_creator_id`, which the specs
  declare and the crate had been silently dropping.
- [Rust API] RFQ-scoped quote methods `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, and `confirm_rfq_quote` (2026-06-25, 2026-07-09).
- [Rust API] `GetQuotesParams::{min_ts, max_ts}` (2026-06-18) and `user_filter`
  on `GetQuotesParams` / `GetRFQsParams`; `post_only` on `Quote` and
  `CreateQuoteRequest`; `creator_subaccount` on `RFQ` and `Quote`, plus
  `Quote::rfq_creator_subaccount` (2026-08-20).
- [Rust API] `exchange_index` on `MarketPosition`, `Settlement`, `Fill`, `Order`,
  `OrderGroup`, `MultivariateEventCollection`, and as an optional filter on
  `GetOrdersParams` / `GetPositionsParams` / `GetFillsParams` (2026-08-20).
- [Rust API] `GetBalanceParams` (`subaccount`, `exchange_index`) plus
  `GetBalanceResponse::balance_breakdown` and
  `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown`,
  both over the new shared `IndexedBalance` type (2026-08-13, 2026-08-20).
- [Rust API] `get_historical_positions()` with the full pagination family
  (2026-07-23), and `GetHistoricalCutoffResponse::market_positions_last_updated_ts`.
- [Rust API] Intra-exchange-instance transfer support (2026-08-13, 2026-08-20):
  `intra_exchange_instance_transfer()` with `source_subaccount` /
  `destination_subaccount`, the paginated history endpoints, and
  `IntraExchangeInstanceTransfer` / `IntraExchangeInstanceTransferStatus`.
- [Rust API] `get_target_balance_allocation()` / `set_target_balance_allocation()`
  (2026-08-20), validating that allocation percents total exactly 100.
- [Rust API] `ApiKey::subaccount` and `subaccount` on the API-key create/generate
  requests (2026-07-02), plus `GetApiKeysResponse::api_key_region_expiration_ts`
  (2026-08-16).
- [Rust API] `get_account_api_usage_level_volume_progress()` and
  `upgrade_account_api_usage_level()` (2026-06-11).
- [Rust API] `SubaccountBalance::exchange_index` (2026-07-02).
- [Rust API] `OrderGroupParams` (`subaccount`, `exchange_index`) for the
  order-group endpoints (2026-08-06).

### Changed

- [Rust API] `PriceRange::{start,end,step}` retyped `String` →
  `FixedPointDollars` (type alias; source compatible).
- [Rust API] `Series::volume_fp` retyped to `Option<FixedPointCount>` (type
  alias; source compatible).
- [Rust API] The normalized direction fields are now `required` in the specs and
  are modeled as plain (non-`Option`) types: `Order::{outcome_side, book_side}`,
  `Fill::{outcome_side, book_side}`, and
  `Trade::{taker_outcome_side, taker_book_side}`.
- [Rust API] `GetOrderGroupResponse::orders` retyped `Vec<Order>` → `Vec<String>`
  — the schema declares an array of order IDs, not order objects.
- [Rust API] `exchange_index` is `i32` on request bodies and queries that accept
  `-1` (auto-route by market ticker) and `u32` where the schema sets
  `minimum: 0`.
- [Rust API] `WsChannelV2::CfbenchmarksValue::is_private()` now returns `true`;
  both value feeds are documented as requiring authentication, so subscribing on
  an unauthenticated client fails client-side instead of at the server.
- [Upstream] `GetOrdersParams` now accepts `limit` up to 1000 (was 200), matching
  the shared `LimitQuery` maximum.
- [Upstream] Subaccount validation now enforces `0..=63` (was `0..=32`), matching
  the range the OpenAPI states throughout. Subaccounts 33–63 were previously
  rejected client-side.

### Deprecated

- [Rust API] The quote-ID-only communications endpoints (`get_quote`,
  `delete_quote`, `accept_quote`, `confirm_quote`) are marked `#[deprecated]`
  (2026-06-25, 2026-07-09). They remain `deprecated: true` but present in the
  OpenAPI, so they are kept. Use the RFQ-scoped replacements.
- [Rust API] The legacy direction fields `Order::{side, action}`,
  `Fill::{side, action}`, and `Trade::taker_side` are marked `#[deprecated]`.
  They are still present in the specs, so they remain `Option` rather than being
  removed. Use `outcome_side` / `book_side` (`taker_*` on `Trade`).
- [Rust API] `CreateRFQRequest::target_cost_centi_cents` is marked
  `#[deprecated]` (`deprecated: true` upstream). Use `target_cost_dollars`.

### Removed

- [Rust API] `get_exchange_announcements()` and the `Announcement`,
  `AnnouncementType`, `AnnouncementStatus`, and
  `GetExchangeAnnouncementsResponse` types — `GET /exchange/announcements` was
  removed upstream on 2026-07-04. Use `get_exchange_schedule()`.
- [Rust API] `Market::response_price_units` and
  `Market::fractional_trading_enabled`, and `MarketPosition::resting_orders_count`
  — removed from the schema on 2026-07-09.
- [Rust API] `ErrorResponse::service` — deprecated 2026-07-28, removed
  2026-08-06. Branch on `code`, which is present on every error response.
- [Rust API] `GetQuotesParams::{market_ticker, event_ticker}` — these filters
  were removed from `GET /communications/quotes` on 2026-06-20.
- [Rust API] The six legacy V1 order-mutation methods — `create_order`,
  `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders` — and their 15 request/response types. The 2026-06-18
  changelog announced these as *deprecated*, but OpenAPI 3.28.0 no longer
  declares the paths at all (`/portfolio/orders` and
  `/portfolio/orders/{order_id}` retain only `GET`). Use the `*_v2` event-order
  methods.
- [Rust API] The multivariate lookup surface, removed upstream on 2026-08-06:
  `lookup_tickers_for_market_in_multivariate_event_collection()`,
  `get_multivariate_event_collection_lookup_history()`, and their types; plus
  the `multivariate` WebSocket channel — `WsChannelV2::Multivariate`,
  `WsMsgType::{Multivariate, MultivariateLookup}`, the `WsDataMessage`
  variants, and the `WsMultivariate*` types. Use
  `multivariate_market_lifecycle` and the RFQ APIs.
- [Rust API] `WsMarketLifecycleV2::fractional_trading_enabled` and
  `WsMarketLifecycleEventType::FractionalTradingUpdated` — absent from both the
  OpenAPI and the AsyncAPI as of the 2026-07-09 field removal.
- [Rust API] `OrderQueuePosition::queue_position`,
  `GetOrderQueuePositionResponse::queue_position`, and `contracts_limit` on
  `OrderGroup` / `GetOrderGroupResponse` — the integer twins were dropped from
  the schemas in favor of the fixed-point fields.

### Fixed

- [Rust API] `SeriesFeeChange` field types corrected to match the OpenAPI
  (`id: i64 → String`, `fee_multiplier: i64 → f64`,
  `scheduled_ts: i64 → String`). The previous shape could not decode a live
  `/series/fee_changes` response.
- [Rust API] `GetLiveDatasParams::milestone_ids` uses OpenAPI `form` /
  `explode: true` repeated query parameters, which `serde_urlencoded` cannot
  emit from a struct field — every `get_live_data_batch()` call previously
  failed to serialize. The params struct now flattens to key/value pairs at call
  time and additionally accepts `include_player_stats`.

### Breaking

- [Rust API] Removed items listed under **Removed** above.
- [Rust API] `EventData::product_metadata` retyped `Option<EventMetadata>` →
  `Option<EventProductMetadata>`. Field-for-field source compatible, but code
  naming `EventMetadata` in that position must change.
- [Rust API] `SeriesFeeChange` field retypes (see **Fixed**).
- [Rust API] `GetLiveDatasParams` no longer derives `Serialize` and gained a
  field; construct it with `..Default::default()`.
- [Rust API] `IncentiveProgram` gained a required `incentive_description` field.
- [Rust API] `get_balance()` now takes a `GetBalanceParams` argument; pass
  `GetBalanceParams::default()` to preserve the previous behavior.
  `GetBalanceResponse::balance_dollars` is promoted from `Option` to required.
- [Rust API] `update_order_group_limit()` takes a params argument, and
  `delete_order_group()` / `reset_order_group()` / `trigger_order_group()` take
  `impl Into<OrderGroupParams>` — existing `SubaccountQueryParams` call sites
  still compile via the provided `From` conversion.
- [Rust API] `MarketPosition`, `Settlement`, and `Fill` gained a required
  `exchange_index`, so payloads omitting it no longer deserialize. Likewise
  `SubaccountBalance::exchange_index`, which additionally means
  `subaccount_number` is no longer unique within a subaccount-balances response.
- [Rust API] `WsUpdateSubscriptionParamsV2::underlying_tickers` has no `Default`,
  so every struct literal must supply it.
- [Rust API] Removed enum variants (`WsChannelV2`, `WsMsgType`,
  `WsDataMessageV2` / `WsDataMessageRef`, `WsMarketLifecycleEventType`) and
  added variants both break exhaustive `match` arms.
- [Rust API] Structs that gained fields break exhaustive struct literals; all
  are `Default`-constructible unless noted above.


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
