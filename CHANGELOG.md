# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-05

### Compatibility

- Docs snapshot: 2026-09-05
- OpenAPI: 3.29.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-10

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Entry | Action |
|---|---|
| API usage volume progress endpoint (2026-06-11) | Deferred — new endpoint, tracked as a gap in `docs/spec-parity.md` |
| Perps mark prices on margin markets (2026-06-11) | No code change — margin market types not in crate |
| Self-serve Advanced API usage tier upgrade (2026-06-11) | Deferred — new endpoint, tracked as a gap |
| Margin fee-tier endpoint returns active rates (2026-06-11) | No code change — exchange bug fix only, shape unchanged |
| Perps volume/OI notional fields on margin markets (2026-06-11) | No code change — margin market types not in crate |
| Tick size on GET Margin Markets (2026-06-11) | No code change — margin market types not in crate |
| Fractional quantities for RFQs (2026-06-11) | No code change — `contracts_fp` / fixed-point quote fields already present |
| `settlement_sources` added to the events API (2026-06-18) | Added `EventData.settlement_sources` |
| Strike type and cap strike on `metadata_updated` (2026-06-18) | Added `WsMarketLifecycleV2`/`Ref` top-level `strike_type`, `cap_strike`, `custom_strike` |
| RFQ quote identity on FIX (2026-06-18) | No code change — FIX not modeled |
| Trade entries in FIX market data (2026-06-18) | No code change — FIX not modeled |
| Legacy order mutation endpoints deprecated (2026-06-18) | `#[deprecated]` on `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` |
| Event tickers filter on `GET /events` (2026-06-18) | Added `GetEventsParams::tickers` (and `::min_updated_ts`) |
| Subaccount on margin positions (2026-06-18) | No code change — `/margin/positions` not modeled |
| Block-trade accept API key permissions (2026-06-18) | No code change — `ApiKeyScope` already carries these scopes |
| Sanity limits on orderbook subscriptions (2026-06-18) | No code change — operational limit, no shape change |
| Quote time filters + pagination fix (2026-06-18) | Added `min_ts`/`max_ts` to `GetQuotesParams`; pagination fix is exchange-side |
| Communications retention window reduced (2026-06-19) | No code change — operational retention policy |
| RFQ quote market/event filters removed (2026-06-20) | Removed `market_ticker`/`event_ticker` from `GetQuotesParams` |
| Get Quote rate-limit cost reduced (2026-06-23) | No code change — operational rate-limit change |
| RFQ quotes support post-only on FIX (2026-06-24) | No code change — FIX not modeled |
| RFQ quote retention + RFQ-scoped quote actions (2026-06-25) | Added `get_quote_for_rfq`, `delete_quote_for_rfq`, `accept_quote_for_rfq`, `confirm_quote_for_rfq` |
| API usage tier qualification halved (2026-06-25) | No code change — operational |
| FIX exchange index routing (2026-06-25) | No code change — FIX not modeled |
| Margin risk per-market metrics restricted (2026-06-26) | No code change — margin not modeled |
| Margin `margin_used` omitted for jointly-margined positions (2026-06-29) | No code change — margin not modeled |
| Trade-scoped API key permissions (2026-06-30) | No code change — `ApiKeyScope::WriteTrade` already present |
| Multivariate lookup history endpoints deprecated (2026-07-02) | Superseded by the 2026-08-06 removal (below) |
| Margin positions include `is_portfolio` (2026-07-02) | No code change — margin not modeled |
| `price_ranges` on `market_lifecycle_v2` events (2026-07-02) | Added `WsMarketLifecycleV2`/`Ref.price_ranges: Vec<WsPriceRange>` |
| Per-index exchange status (2026-07-02) | Added `GetExchangeStatusResponse.intra_exchange_transfers_active` / `.exchange_index_statuses: Vec<ExchangeIndexStatus>` |
| Per-index subaccount balances (2026-07-02) | Added `SubaccountBalance.exchange_index` |
| AcceptQuote / cancel-replace FIX reject reasons (2026-07-02, x2) | No code change — FIX not modeled |
| Sub-account-restricted API keys (2026-07-02) | Added `subaccount`/`fcm_subtrader_id` to `ApiKey`, `CreateApiKeyRequest`, `GenerateApiKeyRequest`; `warning` to `CreateApiKeyResponse`; `CreateSubaccountRequest{exchange_index}` body on `create_subaccount` |
| Exchange announcements endpoint removed (2026-07-04) | Removed `get_exchange_announcements`, `Announcement*`, `GetExchangeAnnouncementsResponse` |
| FIX Tag 2446 on Incremental Refresh (2026-07-09) | No code change — FIX not modeled |
| RFQ-scoped quote lookup endpoint (2026-07-09) | Added `get_quote_for_rfq` (see 2026-06-25 row) |
| Deprecated Predictions REST schema fields removed (2026-07-09) | Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| Margin orders identify system order reasons (2026-07-09) | No code change — margin not modeled |
| Incentive programs on hidden events excluded (2026-07-22) | No code change — listing-visibility behavior only |
| Order groups limited to 25,000/user (2026-07-23) | No code change — operational limit |
| Historical positions endpoint (2026-07-23) | Deferred — new endpoint, tracked as a gap |
| Subaccount-restricted keys can open WS sessions (2026-07-23) | No code change — access-control behavior, no new fields |
| Subaccount-restricted keys can quote on RFQ FIX (2026-07-23) | No code change — FIX not modeled |
| Pyth value WebSocket channel (2026-07-23) | Deferred — new channel, tracked as a gap |
| New price level structures (2026-07-23) | No code change — `price_level_structure` is already a raw `String` |
| `service` field on errors deprecated (2026-07-28) | Superseded by the 2026-08-06 removal (below) |
| Richer combo-validation errors on MVE creation (2026-07-30) | No code change — `ErrorResponse.message`/`.details` already `Option<String>` |
| Lifecycle creation messages include `exchange_index` (2026-07-30) | Added `WsMarketLifecycleV2`/`WsEventLifecycle` top-level `exchange_index` |
| Series responses include `exchange_index` (2026-07-30) | Added `Series.exchange_index` |
| New endpoint for event-keyed live data (2026-07-30) | Deferred — new endpoint, tracked as a gap |
| Subaccount-restricted keys can read queue positions (2026-07-30) | No code change — access-control behavior only |
| Event `product_metadata` includes `cadence` (2026-07-30) | Added `EventMetadata.cadence` |
| Subaccount-restricted keys can use batch order endpoints (2026-07-30) | No code change — access-control behavior only |
| Subaccount on `quote_created` (2026-07-30) | Added `subaccount` to `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` (+ Ref variants) |
| Subaccount-restricted keys can manage order groups (2026-07-30) | No code change — access-control behavior only |
| Multivariate lookup endpoint + WS channel removed (2026-08-06) | **Removed**: `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsDataMessageV2/Ref::Multivariate`, `WsMultivariate`/`Ref`; `get_multivariate_event_collection_lookup_history`, `lookup_tickers_for_market_in_multivariate_event_collection` + types |
| FIX execution reports carry exchange index (2026-08-06) | No code change — FIX not modeled |
| Sided leverage estimates on margin markets (2026-08-06) | No code change — margin not modeled |
| Order group limit updates support subaccounts (2026-08-06) | Added `SubaccountQueryParams` param to `update_order_group_limit`; `SubaccountQueryParams.exchange_index` |
| Multivariate event collections include `exchange_index` (2026-08-06) | Added `MultivariateEventCollection.exchange_index` |
| `service` field removed from error responses (2026-08-06) | **Removed** `ErrorResponse.service` |
| New `center_deci_edge_centi_cent` structure (2026-08-13) | No code change — raw `String` |
| Balance reads scoped by `exchange_index` (2026-08-13) | Added `GetBalanceParams{subaccount, exchange_index}` argument to `get_balance` (was argument-less) |
| Block trade indicator for WS trades (2026-08-13) | Added `WsTrade`/`Ref.is_block_trade` |
| Exchange shard descriptions (2026-08-13) | Added `ExchangeIndexStatus.description` |
| Margin order groups bind to single exchange index (2026-08-13) | No code change — margin not modeled |
| Order group max increased to 100,000 (2026-08-13) | No code change — operational limit |
| Richer combo-validation errors on FIX RFQ creation (2026-08-13) | No code change — FIX not modeled |
| Intra-account transfer history endpoints (2026-08-13) | Deferred — new endpoints, tracked as a gap |
| API key location attestation expiry (2026-08-16) | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| VPC peering for Prime members (2026-08-20) | No code change — connectivity/infra, not an API shape |
| Kalshi Weather Index endpoint (2026-08-20) | Deferred — new endpoint, tracked as a gap |
| Maker fee exemption for independent NFL combos (2026-08-20) | No code change — fee computation behavior, not shape |
| Entry timestamps for FIX market data (2026-08-20) | No code change — FIX not modeled |
| Cross-shard subaccount transfers (2026-08-20) | Deferred — part of the `intra_exchange_instance_transfer` gap |
| Target balance allocation endpoints (2026-08-20) | Deferred — new endpoints, tracked as a gap |
| Resting order value breakdown by exchange index (2026-08-20) | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown: Vec<IndexedBalance>` |
| Exchange index on fill/settlement/position/WS-fill records (2026-08-20) | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition`, `WsFill`/`Ref` |
| Exchange index filters for portfolio lists (2026-08-20) | Added `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| RFQs/combo creation for sub-account-restricted keys (2026-08-20) | No code change — access-control behavior only |
| Optional balance reads by `exchange_index` (2026-08-20) | Covered by `GetBalanceParams` (see 2026-08-13 row) |
| Exit triggers on margin positions (2026-08-20) | No code change — margin not modeled |
| Post-only quotes preserved (2026-08-22) | No code change — policy/behavior only |
| Combo RFQ fee assignment for briefly resting orders (2026-08-22) | No code change — fee computation behavior |
| Upcoming exchange sharding (2026-08-24) | No code change — informational advance notice |
| Localized market content via `Accept-Language` (2026-08-27) | No code change — caller can set arbitrary request headers already |
| Trade type on FIX market data (2026-08-27) | No code change — FIX not modeled |
| Exchange index on `user_orders` messages (2026-08-27) | Added `WsUserOrder.exchange_index` |
| Cancel-all-orders endpoints (2026-08-27) | Deferred — new endpoints, tracked as a gap |
| Historical CF Benchmarks via REST passthrough (2026-08-27) | No code change — docs-only; passthrough endpoint already generic |
| `available_on_brokers` deprecated (2026-08-27) | No code change — kept as `Option<bool>`; still observed live (see spec-parity note) |
| Exchange auto-routing enabled by default (2026-08-27) | No code change — server-side default routing behavior |
| Margin maker-volume incentive programs (2026-08-27) | No code change — margin not modeled |
| Structured target images (2026-08-29) | No code change — `StructuredTarget.details` is already an untyped passthrough map |
| Weather index calibration history (2026-08-31) | Deferred — part of the weather-index gap |
| CF Benchmarks 5Hz WS channel (2026-09-03) | Deferred — new channel, tracked as a gap |
| Higher FIX market data session limit (2026-09-03) | No code change — FIX not modeled |
| Order identity on FIX market data (2026-09-03) | No code change — FIX not modeled |
| Margin fee tier rates endpoint (2026-09-03) | No code change — new margin-only endpoint, out of crate's modeled scope |
| Filter FCM orders by `client_order_ids` (2026-09-03) | Deferred — FCM orders endpoint not modeled at all |
| Filter historical positions by subaccount (2026-09-03) | Deferred — part of the historical-positions gap |
| Correct remaining counts after crossing amendments (2026-09-03) | No code change — exchange-side bug fix, response field semantics only |
| Lower rate-limit cost for cancel-all (2026-09-03) | No code change — operational; endpoint itself is a deferred gap |
| Shard rebalance margin reservation (2026-09-03) | Deferred — part of the `target_balance_allocation` gap |
| `ClearingBusinessDate` on FIX (2026-09-03) | No code change — FIX not modeled |
| Tapered sub-cent pricing on combo markets (2026-09-03) | No code change — `price_level_structure`/`price_ranges` already generic; verified live |
| `available_on_brokers` removed from events (2026-09-10) | No code change — field still observed live as of this refresh; see spec-parity note |
| Weather index `receipt_basis` (2026-09-10) | Deferred — part of the weather-index gap |
| Margin markets expose `asset_class` (2026-09-10) | No code change — margin market types not in crate |
| Sharding for commodities/basketball (2026-09-10) | No code change — informational |
| `center_deci_edge_centi_cent` emitted again (2026-09-10, bug fix) | No code change — raw `String`, unaffected either way |
| AsyncAPI schema corrections (`seq`, `sid`, retired error codes, error-schema field removal) (2026-09-10) | No code change — envelope already carries generic `sid`/`seq`; `WsError` never modeled `market_id`/`market_ticker`; no hardcoded error-code list exists |

### Added

- [Rust API] Added `exchange_index` (identifying the exchange shard a resource lives on) across the
  surface Kalshi has rolled multi-shard support into so far: `Market`, `EventData`, `Series`,
  `MultivariateEventCollection`, `Fill`, `Settlement`, `MarketPosition`, `SubaccountBalance`,
  `ApiKey`, `WsMarketLifecycleV2`/`WsEventLifecycle` (top-level, `created`/`event_lifecycle` only),
  `WsFill`, `WsUserOrder`. Added a shared `IndexedBalance` type (`exchange_index` + `balance`) used
  by `GetBalanceResponse.balance_breakdown` and
  `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`. Added
  `GetExchangeStatusResponse.intra_exchange_transfers_active` and
  `.exchange_index_statuses: Vec<ExchangeIndexStatus>` (with `description`, added 2026-08-13).
  Added an `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] Added `WsMarketLifecycleV2`/`Ref` top-level `strike_type`, `cap_strike`,
  `custom_strike` (present on `metadata_updated` events, alongside the pre-existing `floor_strike` /
  `yes_sub_title`), and `price_ranges: Option<Vec<WsPriceRange>>` (present on `created` and
  `price_level_structure_updated` events).
- [Rust API] Added `WsTrade`/`WsTradeRef.is_block_trade: Option<bool>`.
- [Rust API] Added `EventData.settlement_sources: Option<Vec<SettlementSource>>`,
  `EventData.fee_type_override` / `.fee_multiplier_override`, and `EventMetadata.cadence`.
- [Rust API] Added `GetEventsParams::tickers` (comma-separated event ticker filter) and
  `::min_updated_ts`.
- [Rust API] Added RFQ-scoped quote action methods: `get_quote_for_rfq`, `delete_quote_for_rfq`,
  `accept_quote_for_rfq`, `confirm_quote_for_rfq` (path-scoped by `rfq_id`, matching Kalshi's
  preferred replacement for the quote-ID-only methods, which are now `#[deprecated]` in favor of
  these). Added `GetQuotesParams::min_ts` / `::max_ts` / `::user_filter` and
  `GetRFQsParams::user_filter`. Added `Quote::post_only` / `::creator_subaccount` /
  `::rfq_creator_subaccount` and `RFQ::creator_subaccount`.
- [Rust API] Added `subaccount: Option<i32>` to `WsQuoteCreated`, `WsQuoteAccepted`,
  `WsQuoteExecuted` (and their `Ref` variants) — present when your side of the quote used a
  subaccount.
- [Rust API] Added sub-account and FCM-subtrader-restricted API key support: `ApiKey::subaccount` /
  `::fcm_subtrader_id`, `CreateApiKeyRequest`/`GenerateApiKeyRequest::subaccount` /
  `::fcm_subtrader_id`, `CreateApiKeyResponse::warning`, and
  `GetApiKeysResponse::api_key_region_expiration_ts`.
- [Rust API] Added a `CreateSubaccountRequest` (with `exchange_index`) as the (previously absent)
  body parameter for `create_subaccount`, and `exchange_index` to
  `ApplySubaccountTransferRequest`.
- [Rust API] Added `SubaccountQueryParams::exchange_index`, and a `SubaccountQueryParams` argument
  to `update_order_group_limit` (previously took no query params at all).
- [Rust API] Added `GetBalanceParams` (with `subaccount`, `exchange_index`) as the (previously
  absent) argument to `get_balance`.

### Changed

- [Rust API] `GetQuotesParams` no longer supports `market_ticker` / `event_ticker` filters (Kalshi
  removed them 2026-06-20); use RFQ ID, user, status, or the new `min_ts`/`max_ts` window instead.
- [Rust API] `orders.rs`'s V2 request/param `exchange_index` fields (`CreateOrderV2Request`,
  `CancelOrderV2Params`, `AmendOrderV2Request`, `DecreaseOrderV2Request`,
  `BatchCancelOrderV2RequestOrder`) changed from `Option<u32>` to `Option<i32>`: the documented
  auto-routing sentinel is `-1`, which `u32` cannot represent. This is a correctness fix, not a
  behavior change for callers already using a non-negative shard index.
- [Docs] `docs/spec-parity.md` gained a durable summary of the exchange-sharding rollout, the
  `metadata_updated` top-level field set, the `price_level_structure`-stays-a-string design
  rationale, the `multivariate` channel/endpoint removal, and an explicit list of upstream additions
  not yet implemented (see "Known gaps" below).

### Removed

- [Rust API] Removed the `multivariate` WebSocket channel and its `multivariate_lookup` message,
  which Kalshi removed 2026-08-06: `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `MultivariateLookup`, `WsDataMessageV2::Multivariate` / `WsDataMessageRef::Multivariate`, and the
  `WsMultivariate` / `WsMultivariateRef` message types.
- [Rust API] Removed the corresponding REST lookup surface, also removed by Kalshi:
  `KalshiRestClient::get_multivariate_event_collection_lookup_history` and
  `::lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types
  (`GetMultivariateEventCollectionLookupHistoryParams/Response`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest/Response`). Use
  `multivariate_market_lifecycle` and `POST /multivariate_event_collections/{collection_ticker}`
  instead.
- [Rust API] Removed `ErrorResponse.service` (Kalshi removed the field from the OpenAPI schema
  2026-08-06 after deprecating it 2026-07-28); branch on `code` instead.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` (removed from the OpenAPI schema and no longer returned
  live, 2026-07-09).
- [Rust API] Removed `get_exchange_announcements` and its types (`Announcement`,
  `AnnouncementType`, `AnnouncementStatus`, `GetExchangeAnnouncementsResponse`); Kalshi removed
  `GET /exchange/announcements` 2026-07-04. Use `get_exchange_schedule` instead.
- [Rust API] Removed dead code in `ws::types::mod` (`MarketPositionRef`/`EventPositionRef`) that
  modeled a stale, pre-rewrite `market_positions` shape and did not compile against the current
  `MarketPosition` struct; the live shape has been correctly modeled by
  `messages::positions::WsMarketPositionRef` all along.

### Fixed

- [Tests] Fixed two pre-existing compile failures on this branch unrelated to this refresh's
  changelog window: `ws::types::envelope`'s `ListSubscriptions` test match arms were missing the
  `sid`/`seq` fields added in 0.7.0, and `tests/rest_auth.rs::test_get_account_api_limits` still
  read the flat `read_limit`/`write_limit` fields removed in 0.6.0. Both are now fixed and
  `cargo test --all-targets --features live-tests` compiles again.

### Breaking

- [Rust API] `get_balance()` now requires a `GetBalanceParams` argument (was argument-less).
- [Rust API] `create_subaccount()` now requires a `CreateSubaccountRequest` argument (was
  argument-less).
- [Rust API] `update_order_group_limit()` now takes an additional `SubaccountQueryParams` argument
  before the request body.
- [Rust API] `ErrorResponse` no longer has a `service` field; exhaustive struct destructuring must
  drop it.
- [Rust API] `Market` no longer has `response_price_units` or `fractional_trading_enabled`;
  `MarketPosition` no longer has `resting_orders_count`.
- [Rust API] `get_exchange_announcements`, `Announcement`, `AnnouncementType`,
  `AnnouncementStatus`, and `GetExchangeAnnouncementsResponse` no longer exist.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsDataMessageV2::Multivariate`/`WsDataMessageRef::Multivariate`, `WsMultivariate`/
  `WsMultivariateRef`, and the two REST multivariate-lookup methods no longer exist. Exhaustive
  matches over `WsChannelV2`, `WsMsgType`, `WsDataMessageV2`, or `WsDataMessageRef` must drop the
  removed variants.
- [Rust API] `CreateOrderV2Request`, `CancelOrderV2Params`, `AmendOrderV2Request`,
  `DecreaseOrderV2Request`, and `BatchCancelOrderV2RequestOrder`'s `exchange_index` field changed
  type from `Option<u32>` to `Option<i32>`.
- [Rust API] `MarketPositionRef`/`EventPositionRef` (in `ws::types`, distinct from
  `messages::positions::WsMarketPositionRef`) no longer exist; they were dead code that did not
  compile.

Per `VERSIONING.md`, this release makes multiple intentional breaking changes to the public Rust
API (signature changes on `get_balance`/`create_subaccount`/`update_order_group_limit`, and several
type/field removals), so this is a **minor** version bump (0.7.0 → 0.8.0), not a patch.


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
