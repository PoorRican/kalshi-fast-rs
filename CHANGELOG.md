# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-08-30

### Compatibility

- Docs snapshot: 2026-08-30
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-08-27

**Version bump rationale (see [`VERSIONING.md`](VERSIONING.md)):** this release removes public
Rust API surface (fields, an enum variant, an endpoint method, and a whole message-type family) and
changes an existing method's signature. Per "Version Bump Rules For Refreshes" — *"Bump minor if
deprecated or removed upstream fields/endpoints force a breaking Rust API change, if public types or
method names change, or if downstream consumers are likely to need code changes"* — this is a minor
bump: 0.7.0 → 0.8.0.

**Changelog entries since the 0.7.0 watermark (2026-06-08) and disposition.** Entries are grouped by
disposition; FIX-only and Margin-only entries are collapsed since this crate does not model FIX or
the Margin exchange (`get_margin_fee_tiers` excepted).

| Entry (date) | Action |
|---|---|
| **FIX-only** (crate has no FIX support): Trade type on FIX market data (8/27); Entry timestamps for FIX market data (8/20); Richer combo-validation errors on FIX RFQ creation (8/13); FIX execution reports identify source exchange index (8/6); FIX Tag 2446 AggressorSide (7/9); Subaccount-restricted keys can quote on RFQ FIX sessions (7/23); FIX exchange index routing (6/25); RFQ quotes support post-only on FIX (6/24); RFQ quote identity on FIX (6/18); Trade entries in FIX market data (6/18); AcceptQuote reject reasons on FIX (7/2); More specific FIX cancel/replace rejects (7/2) | No code change — FIX not modeled |
| **Margin-only** (crate does not model the Margin exchange beyond `get_margin_fee_tiers`): Margin maker-volume incentive programs (8/27); Exit triggers on margin positions (8/20); Margin order groups bind to single exchange_index (8/13); Sided leverage estimates on margin markets (8/6); Margin positions `is_portfolio` flag (7/2); Margin positions `margin_used` omitted for portfolio-margin (6/29); Margin risk per-market metrics limited (6/26); Perps volume/OI notional fields (6/11); Tick size on margin markets (6/11); Perps mark prices on margin markets (6/11); Margin orders `order_reason` (7/9) | No code change — margin market/position/risk/order-group types not in crate |
| Margin fee-tier endpoint returns active rates instead of zeros (6/11) | No code change — values only; `GetMarginFeeTiersResponse` shape unchanged |
| Exchange auto-routing enabled by default (8/27); Post-only preserved on quotes, crossing rate limits (8/22); Combo RFQ fee assignment for briefly-resting orders (8/22); Maker fee exemption for independent NFL combo markets (8/20); Upcoming exchange sharding announcement (8/24); VPC peering for Prime members (8/20); Order-group max raised to 100k (8/13) / limited to 25k (7/23); API usage tier qualification halved (6/25); `GET /communications/quotes/{id}` rate-limit cost reduced (6/23); RFQ/quote retention window reduced to 7d (6/19); Orderbook subscription sanity limits (6/18); Incentive programs on hidden events excluded from listing (7/22) | No code change — operational/fee/rate-limit behavior only, no schema impact |
| Tapered sub-cent pricing on combo markets, `center_deci_edge_centi_cent` (8/27, 8/13); seven new `center_*_edge_*_cent` price level structures (7/23) | No code change — `price_level_structure` is an untyped `String` and `price_ranges` is already the documented source of truth for valid prices; see `docs/spec-parity.md` |
| Multivariate lookup history "fully deprecated" (7/2) | Superseded by the 8/6 removal below |
| `service` field on error responses deprecated (7/28) | Superseded by the 8/6 removal below |
| RFQs/combo-market creation for sub-account-restricted keys (8/20); queue-position reads (7/30); batch order endpoints (7/30); order-group management (7/30); WebSocket sessions (7/23) for sub-account-restricted keys; block-trade-accept scopes (6/18); `write::trade` scope (6/30) | No code change — permission/scope enforcement is server-side; the crate passes API keys through unmodified |
| Fractional quantities for RFQs (6/11) | No code change — `contracts_fp` / `yes_contracts_offered_fp` / `no_contracts_offered_fp` already modeled |
| `EventData.available_on_brokers` deprecated, always `false` (8/27) | **[Rust API]** Marked `#[deprecated]`; kept as `Option<bool>` (field still present upstream) |
| `settlement_sources` added to events (6/18) | **[Rust API]** Added `EventData.settlement_sources: Vec<SettlementSource>` |
| `Event.product_metadata` gains `cadence` (7/30) | **[Rust API]** Added `EventMetadata.cadence: Option<String>` |
| `tickers` filter on `GET /events` (6/18) | **[Rust API]** Added `GetEventsParams.tickers: Option<Vec<String>>` |
| `FeeType::quadratic_with_combo_maker_fees` (OpenAPI, undated) | **[Rust API]** Added `FeeType::QuadraticWithComboMakerFees` |
| `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` removed (7/9) | **[Rust API, Breaking]** Fields removed from `Market` / `MarketPosition` |
| `service` field removed from error responses (8/6) | **[Rust API, Breaking]** Removed `ErrorResponse.service` |
| `GET /exchange/announcements` removed (7/4) | **[Rust API, Breaking]** Removed `get_exchange_announcements` and its response types |
| Multivariate lookup endpoint + WebSocket channel removed (8/6) | **[Rust API, Breaking]** Removed `lookup_tickers_for_market_in_multivariate_event_collection`, `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`, and the WebSocket `multivariate` channel / `multivariate_lookup` message (`WsMultivariate`, `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`) |
| RFQ quote `market_ticker`/`event_ticker` filters removed from `GET /communications/quotes` (6/20) | **[Rust API, Breaking]** Removed `GetQuotesParams.market_ticker`/`.event_ticker` |
| `min_ts`/`max_ts` + `user_filter` on `GET /communications/quotes`, pagination fix (6/18, 6/18) | **[Rust API]** Added `GetQuotesParams.min_ts`/`.max_ts`/`.user_filter` |
| RFQ-scoped quote actions (6/25); RFQ-scoped quote lookup (7/9) | **[Rust API]** Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`; deprecated `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote` |
| Sub-account-restricted API keys at creation (7/2); API key location attestation expiry (8/16) | **[Rust API]** Added `ApiKey.subaccount`/`.api_key_region_expiration_ts`; `CreateApiKeyRequest.subaccount`, `GenerateApiKeyRequest.subaccount` |
| API usage volume progress endpoint (6/11); self-serve Advanced tier upgrade (6/11) | **[Rust API]** Added `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level` |
| Cancel-all-orders endpoints (8/27) | **[Rust API]** Added `cancel_all_orders` (`DELETE /portfolio/events/orders`) |
| Historical positions endpoint (7/23) | **[Rust API]** Added `get_historical_positions` (`GET /historical/positions`) |
| Optional/scoped balance reads by `exchange_index` (8/20, 8/13) | **[Rust API, Breaking]** `get_balance` now takes a `GetBalanceParams { subaccount, exchange_index }` argument (previously took none) |
| Resting order value breakdown by exchange index (8/20) | **[Rust API]** Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Option<Vec<IndexedBalance>>` |
| Exchange index filters for portfolio lists (8/20) | **[Rust API]** Added `exchange_index` filter to `GetPositionsParams`/`GetFillsParams`/`GetOrdersParams` |
| Exchange index on portfolio and WebSocket fill records (8/20) | **[Rust API]** Added `exchange_index: Option<i64>` to `MarketPosition`, `Fill`, `Settlement`, `WsFill` |
| Order group limit updates support subaccounts (8/6) | **[Rust API, Breaking]** `update_order_group_limit` now takes a `UpdateOrderGroupLimitParams { subaccount, exchange_index }` argument |
| Multivariate event collections include `exchange_index` (8/6) | **[Rust API]** Added `MultivariateEventCollection.exchange_index` |
| Series responses include `exchange_index` (7/30) | **[Rust API]** Added `Series.exchange_index` |
| Per-index exchange status (7/2) | **[Rust API]** Added `GetExchangeStatusResponse.exchange_index_statuses`/`.intra_exchange_transfers_active`, new `ExchangeIndexStatus` |
| Exchange index on user order messages (8/27) | **[Rust API]** Added `WsUserOrder.exchange_index` |
| Lifecycle creation messages include `exchange_index` (7/30) | **[Rust API]** Added `exchange_index` to `WsMarketLifecycleV2` (covers `market_lifecycle_v2` and `multivariate_market_lifecycle`) and `WsEventLifecycle` |
| `price_ranges` added to `market_lifecycle_v2` events (7/2) | **[Rust API]** Added `WsMarketLifecycleV2.price_ranges: Option<Vec<WsPriceRange>>` |
| Strike type and cap strike on `metadata_updated` (6/18) | **[Rust API]** Added top-level `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| Block trade indicator for WebSocket trades (8/13) | **[Rust API]** Added `WsTrade.is_block_trade: Option<bool>` |
| Subaccount on `quote_created`, matching `quote_accepted`/`quote_executed` (7/30) | **[Rust API]** Added `subaccount: Option<i64>` to `WsQuoteCreated`, `WsQuoteAccepted`, `WsQuoteExecuted` (the latter two were missing it entirely; not previously modeled) |
| Per-index subaccount balances (7/2) | No code change needed beyond the general exchange-index modeling above — `GetSubaccountBalancesResponse` already returns one entry per balance via `SubaccountBalance`; a dedicated `exchange_index` field was not added to `SubaccountBalance` this refresh (tracked as a follow-up) |
| Cross-shard subaccount transfers (8/20); Intra-account transfer history endpoints (8/13) | Deferred — the base "Intra Account Transfer" endpoint family is not implemented in the crate (predates this watermark); see `docs/spec-parity.md` Known Gaps |
| Kalshi Weather Index endpoint (8/20); event-keyed live data endpoint (7/30); target balance allocation endpoints (8/20); Pyth value WebSocket channel (7/23) | Deferred — new endpoints/channel out of scope for this refresh; see `docs/spec-parity.md` Known Gaps |

### Fixed

- **[Rust API]** The WebSocket `market_lifecycle_v2` `fractional_trading_updated` event type and the
  `fractional_trading_enabled` field on `WsMarketLifecycleV2` were removed: they are no longer
  present in the live AsyncAPI spec (found via this refresh's required-field grep against the
  `market_lifecycle_v2` payload schema; the removal predates the 2026-06-08 watermark and was never
  caught by an earlier refresh).
- **[Docs]** `ws::types::MarketPositionRef`/`EventPositionRef` mirror the REST `MarketPosition`/
  `EventPosition` shape and are not wired to the actual `market_positions` WebSocket channel (that
  channel is correctly modeled by `WsMarketPosition`/`WsMarketPositionRef`). Left in place for
  compatibility but now kept in sync with `MarketPosition`'s fields; see `docs/spec-parity.md`.

### Added

See the disposition table above for the full list of additive fields, new enum variants, and new
`KalshiRestClient` methods (`cancel_all_orders`, `get_historical_positions`,
`get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level`,
`get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`).

### Deprecated

- **[Rust API]** `EventData.available_on_brokers` — no longer populated by the exchange.
- **[Rust API]** `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` — use the RFQ-scoped
  `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` instead.

### Removed

- **[Rust API]** `ErrorResponse.service`.
- **[Rust API]** `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`.
- **[Rust API]** `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` (`GET /exchange/announcements` removed upstream).
- **[Rust API]** `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, `GetMultivariateEventCollectionLookupHistoryParams`,
  `GetMultivariateEventCollectionLookupHistoryResponse`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response`.
- **[Rust API]** `WsMultivariate`, `WsMultivariateRef`, `WsMultivariateSelectedMarket(Ref)`,
  `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`, `WsMsgType::MultivariateLookup`,
  `WsMarketLifecycleEventType::FractionalTradingUpdated`,
  `WsMarketLifecycleV2.fractional_trading_enabled`.
- **[Rust API]** `GetQuotesParams.market_ticker`, `GetQuotesParams.event_ticker`.

### Breaking

- **[Rust API]** `get_balance` now takes a `GetBalanceParams` argument (previously took none).
- **[Rust API]** `update_order_group_limit` now takes an additional `UpdateOrderGroupLimitParams`
  argument.
- **[Rust API]** All field/type/method removals listed above under Removed.
- Downstream consumers matching exhaustively on `WsChannelV2`, `WsMsgType`,
  `WsMarketLifecycleEventType`, or constructing `Market`/`MarketPosition`/`ErrorResponse` with a
  struct literal (rather than `..Default::default()` / field access) will need updates.


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
