# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-22

### Compatibility

- Docs snapshot: 2026-09-22
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-24

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

97 relevant entries (tagged `Predictions` + `REST`/`WebSocket`) were triaged below. An
additional 34 entries pre-filtered as FIX-only or Margin-only were spot-checked; none touch
`GET /margin/fee_tiers` or `GET /account/endpoint_costs` (the only Margin/account-adjacent
surface this crate models) — e.g. the new `GET /margin/fee_tier_rates` (Sept 3) is a distinct
path from this crate's `/margin/fee_tiers`, so none were promoted into the table below.

| Entry (date) | Action |
|---|---|
| API usage volume progress endpoint (Jun 11) | **Deferred** — new `GET /account/api_usage_level/volume_progress` endpoint not implemented; account-tier bookkeeping, low priority for a trading adapter (see `docs/spec-parity.md`) |
| Self-serve Advanced API usage tier upgrade (Jun 11) | **Deferred** — new `POST /account/api_usage_level/upgrade` endpoint not implemented, same rationale |
| Fractional quantities for RFQs (Jun 11) | No code change — `contracts_fp`/`yes_contracts_offered_fp`/`no_contracts_offered_fp` already fixed-point strings |
| settlement_sources added to the events API (Jun 18) | Added `EventData.settlement_sources: Option<Vec<SettlementSource>>` |
| Strike type and cap strike on market_lifecycle_v2 metadata_updated (Jun 18) | Added `WsMarketLifecycleV2`/`Ref` top-level `strike_type`, `cap_strike`, `custom_strike` |
| Legacy order mutation endpoints deprecated (Jun 18) | No code change — deprecation notice only; V2 event-order endpoints already implemented (0.6.0) |
| Event tickers filter on GET /events (Jun 18) | Added `GetEventsParams.tickers` |
| Block-trade accept API key permissions (Jun 18) | No code change — API-key scope semantics; block-trade endpoints not modeled |
| Sanity limits enforced on orderbook subscriptions (Jun 18) | No code change — server-side rate/volume limit, no shape change |
| Quote time filters and pagination fix (Jun 18) | Added `GetQuotesParams.min_ts`/`max_ts`; pagination fix is server-side only |
| Communications RFQ and quote retention window reduced (Jun 19) | No code change — retention-window/operational change only |
| RFQ quote market and event filters removed (Jun 20) | **Removed** `GetQuotesParams.market_ticker`/`event_ticker` |
| Get Quote rate-limit cost reduced to 2 tokens (Jun 23) | No code change — rate-limit-only |
| RFQ quote retention and RFQ-scoped quote actions (Jun 25) | Added `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`; documented quote-ID-only methods as deprecated in doc comments |
| API usage tier qualification requirements halved (Jun 25) | No code change — account-tier threshold only |
| Trade-scoped API key permissions (Jun 30) | No code change — API-key scope semantics only |
| Multivariate lookup history endpoints are fully deprecated (Jul 2) | No code change — deprecation notice (removal handled under Aug 6 entry below) |
| price_ranges added to market_lifecycle_v2 events (Jul 2) | Added `WsMarketLifecycleV2`/`Ref` `price_ranges: Option<Vec<WsLifecyclePriceRange>>` |
| Per-index exchange status (Jul 2) | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses` + `ExchangeIndexStatus` |
| Per-index subaccount balances (Jul 2) | **Breaking** — added required `SubaccountBalance.exchange_index` |
| Sub-account-restricted API keys (Jul 2) | No code change — `ApiKey`/`CreateApiKeyResponse` already tolerate the new `subaccount` field via `extra` flatten |
| Exchange announcements endpoint removed (Jul 4) | **Removed** `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus` |
| RFQ-scoped quote lookup endpoint (Jul 9) | Added `get_rfq_quote` (see Jun 25 entry) |
| Deprecated Predictions REST schema fields removed (Jul 9) | **Removed** `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` (part of the larger `Market` field-reconciliation cleanup this refresh — see Added/Removed) |
| Incentive programs on hidden events excluded from listing (Jul 22) | No code change — filtering behavior only |
| Order groups limited to 25,000 per user (Jul 23) | No code change — account limit only |
| Historical positions endpoint (Jul 23) | Added `get_historical_positions`, `GetHistoricalPositionsParams`, `GetHistoricalCutoffResponse.market_positions_last_updated_ts` |
| Subaccount-restricted API keys can open WebSocket sessions (Jul 23) | No code change — server-side authorization only |
| Pyth value WebSocket channel (Jul 23) | Added `WsChannelV2::PythValue`, `WsPythValue[Ref]`, `WsPythUnderlyingList[Ref]`, `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList`, `underlying_tickers` fields |
| New price level structures (Jul 23) | No code change — `price_level_structure` already untyped `String` |
| The service field on error responses is deprecated (Jul 28) | Documented `ErrorResponse.service` as deprecated (doc comment); see Aug 6 for removal |
| Richer combo-validation errors on multivariate market creation (Jul 30) | No code change — `message`/`details` already handled generically by `KalshiError::Http` |
| Lifecycle creation messages now include exchange_index (Jul 30) | Added `WsMarketLifecycleV2`/`Ref` `exchange_index` |
| Series responses include exchange_index (Jul 30) | Added `Series.exchange_index` |
| New endpoint for event-keyed live data (Jul 30) | Added `get_event_live_data`, `EventLiveData`, `GetEventLiveDataResponse`, `GetEventLiveDataParams` |
| Subaccount-restricted API keys can read order queue positions (Jul 30) | No code change — authorization only; `subaccount` param already present |
| Event product_metadata now includes cadence (Jul 30) | No code change — `product_metadata` already untyped `Map`/`Option<Map>` |
| Subaccount-restricted API keys can use batch order endpoints (Jul 30) | No code change — authorization only |
| Subaccount on quote_created (Jul 30) | Added `subaccount` to `WsQuoteCreated`, `WsQuoteAccepted`, `WsQuoteExecuted` (grepped AsyncAPI shows the field on all three; none were previously modeled) |
| Subaccount-restricted API keys can manage order groups (Jul 30) | No code change — authorization only |
| Multivariate lookup endpoint and channel removed (Aug 6) | **Removed** REST lookup-history types/method (`GetMultivariateEventCollectionLookupHistoryParams/Response`, `LookupPoint`, `LookupTickersForMarketInMultivariateEventCollectionRequest/Response`) and WS `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate[Ref]` |
| Order group limit updates support subaccounts (Aug 6) | **Breaking** — `update_order_group_limit` now takes a new `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) |
| Multivariate event collections include exchange_index (Aug 6) | Added `MultivariateEventCollection.exchange_index` |
| The service field has been removed from error responses (Aug 6) | `ErrorResponse.service` kept as always-`None` `Option<String>` (see Jul 28; documented exception in `docs/spec-parity.md`) |
| New center_deci_edge_centi_cent price level structure (Aug 13) | No code change (see Jul 23) |
| Balance reads scoped by exchange_index (Aug 13) | **Breaking** — `get_balance` now takes `GetBalanceParams` (`subaccount`, `exchange_index`); added `GetBalanceResponse.balance_breakdown` |
| Block trade indicator for WebSocket trades (Aug 13) | Added `WsTrade`/`Ref` `is_block_trade: bool` |
| Exchange shard descriptions (Aug 13) | Added `ExchangeIndexStatus.description` |
| Order group maximum increased to 100,000 per user (Aug 13) | No code change — account limit only |
| Intra-account transfer history endpoints (Aug 13) | Added `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer`, `IntraExchangeInstanceTransfer[Response]` |
| API key location attestation expiry (Aug 16) | No code change — `ApiKey.extra` flatten already tolerates `api_key_region_expiration_ts` |
| VPC peering for Prime members (Aug 20) | No code change — infra/connectivity only |
| Kalshi Weather Index endpoint (Aug 20) | **Deferred** — new large domain (`GET /live_data/weather/{city}`), not implemented (see `docs/spec-parity.md`) |
| Maker fee exemption for independent NFL combo markets (Aug 20) | No code change — fee business logic only |
| Cross-shard subaccount transfers (Aug 20) | Added `source_subaccount`/`destination_subaccount` to `IntraExchangeInstanceTransferRequest` |
| Target balance allocation endpoints (Aug 20) | Added `get_target_balance_allocation`, `set_target_balance_allocation`, `TargetBalanceAllocation[Input]`, `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest` |
| Resting order value breakdown by exchange index (Aug 20) | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| Exchange index on portfolio and WebSocket fill records (Aug 20) | Added `exchange_index` to REST `Fill`, `Settlement`, `MarketPosition`, `Order` and WS `WsFill`/`Ref` |
| Exchange index filters for portfolio lists (Aug 20) | Added `exchange_index` filter to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| RFQs and combo-market creation for sub-account-restricted API keys (Aug 20) | No code change — authorization only |
| Optional balance reads by exchange_index (Aug 20) | See Aug 13 `GetBalanceParams` entry |
| Post-only quotes preserved; crossing rate limits may apply (Aug 22) | No code change from this entry (policy reversal); `post_only` on `CreateQuoteRequest`/`Quote` added this refresh as a pre-existing schema gap found while touching these structs for Jun 11/Jun 25 changes |
| Combo RFQ fee assignment for briefly resting orders (Aug 22) | No code change — fee computation only; `FeeType::QuadraticWithComboMakerFees` added separately via full-schema reconciliation of `FeeType` |
| Upcoming exchange sharding (Aug 24) | No code change — operational rollout notice; `exchange_index` support added under other entries |
| Localized market content in REST responses (Aug 27) | No code change — `KalshiRestClient::with_default_headers()` already supports setting `Accept-Language`; no response shape change |
| Exchange index on user order messages (Aug 27) | Added `WsUserOrder.exchange_index` |
| Cancel-all-orders endpoints (Aug 27) | Added `cancel_all_orders_v2` (`DELETE /portfolio/events/orders`) |
| Historical CF Benchmarks values via the REST passthrough (Aug 27) | No code change — documents an existing `/cfbenchmarks/*` passthrough not wrapped by this crate (out of scope) |
| The available_on_brokers field on event responses is deprecated (Aug 27) | No code change from deprecation itself (removed under Sep 10 entry below) |
| Exchange auto-routing enabled by default (Aug 27) | No code change — server-side routing default; `exchange_index` request fields already `Option` |
| Structured target images in Trade API v2 (Aug 29) | No code change — `StructuredTarget.details` already untyped `Map<String, Value>` |
| Weather index calibration history (Aug 31) | **Deferred** — see Aug 20 Weather Index entry |
| CF Benchmarks 5Hz value websocket channel (Sep 3) | Added `WsChannelV2::CfbenchmarksValue5hz`, `WsCfBenchmarksValue5Hz[Ref]`, `WsCfBenchmarksIndexList5Hz[Ref]` |
| Filter FCM orders by client order IDs (Sep 3) | Added `GetFcmOrdersParams.client_order_ids` |
| Filter historical positions by subaccount (Sep 3) | Added `GetHistoricalPositionsParams.subaccount` (part of Jul 23 entry) |
| Correct remaining counts after crossing order amendments (Sep 3) | No code change — server-side computed-value bug fix, no shape change |
| Lower rate-limit cost for cancel all orders (Sep 3) | No code change — rate-limit-only |
| Shard rebalance margin reservation (Sep 3) | Added `SetTargetBalanceAllocationRequest.resting_margin_reservation` (part of Aug 20 entry) |
| Tapered sub-cent pricing on multivariate (combo) markets (Sep 3) | No code change — explicitly "no API fields or message formats change" |
| The deprecated available_on_brokers field is removed from event responses (Sep 10) | **Removed** `EventData.available_on_brokers` |
| Principal-only sizing for target-cost RFQs (Sep 10) | Added `CreateRFQRequest.target_cost_excludes_fees`, `RFQ.target_cost_excludes_fees`, `Quote.target_cost_excludes_fees` |
| Weather index points expose receipt_basis (Sep 10) | **Deferred** — see Aug 20 Weather Index entry |
| Upcoming exchange sharding for commodities and basketball (Sep 10) | No code change — operational rollout notice |
| The center_deci_edge_centi_cent price level structure is emitted again (Sep 10) | No code change — server-side bug fix (briefly serialized empty string) |
| WebSocket schemas corrected to match the messages the service sends (Sep 10) | No code change — `seq` already surfaced on every `WsDataMessageV2`/`Ref` variant; `order_source` is Margin-only (out of scope); `WsError`/`Ref` already model only `code`/`message` |
| WebSocket schema corrections (Sep 17) | No code change — ticker `dollar_volume`/`dollar_open_interest` already signed `i64` |
| Historical fills and orders support min_ts (Sep 17) | Added `min_ts` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| Reduced rate limit cost for QuoteConfirm when providing the RFQ ID (Sep 17) | No code change — rate-limit-only; `confirm_rfq_quote` already supports the RFQ ID |
| Target balance allocations include their reservation policy (Sep 17) | Added `GetTargetBalanceAllocationResponse.resting_margin_reservation` (part of Aug 20 entry) |
| Series responses include a categories list (Sep 17) | Added `Series.categories` |
| Returning to idiomatic MVE series (Sep 17) | No code change — ticker/series-naming convention only |
| WebSocket subscriptions are ready when acknowledged (Sep 17) | No code change — server-side race-condition fix |
| RFQ and quote writes share the shard 1 rate-limit budget (Sep 17) | No code change — rate-limit-only |
| Optional WebSocket compression (Sep 24) | No code change — transport-level `permessage-deflate` negotiation, no application-level shape change |
| Rebalancing without resting-order reservation (Sep 24) | No code change — `RestingMarginReservation` already a raw `String`, so `"none"` round-trips without a crate update |
| Subaccount-scoped historical fills and orders (Sep 24) | Added `subaccount` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| Orders historical cutoff advances independently (Sep 24) | No code change — cutoff semantics only, `orders_updated_ts` already modeled |

### Added

- [Rust API] `exchange_index: Option<i64>` (exchange-sharding shard identifier) added to REST
  `Market`, `Order`, `Fill`, `Settlement`, `MarketPosition`, `Series`, `EventData`,
  `MultivariateEventCollection`, and WS `WsMarketLifecycleV2`/`Ref`, `WsFill`/`Ref`, `WsUserOrder`.
  `SubaccountBalance.exchange_index: i64` (non-optional; new field, always present).
- [Rust API] `ExchangeIndexStatus` struct and `GetExchangeStatusResponse.intra_exchange_transfers_active`
  / `.exchange_index_statuses` for the per-exchange-index status breakdown.
- [Rust API] `GetBalanceParams` (`subaccount`, `exchange_index`) and `GetBalanceResponse.balance_breakdown:
  Option<Vec<IndexedBalance>>`.
- [Rust API] `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`.
- [Rust API] `exchange_index` filter added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] New account/portfolio endpoints and types: `intra_exchange_instance_transfer`,
  `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer` (+
  `IntraExchangeInstanceTransfer[Request/Response]`); `get_target_balance_allocation`,
  `set_target_balance_allocation` (+ `TargetBalanceAllocation[Input]`,
  `GetTargetBalanceAllocationResponse`, `SetTargetBalanceAllocationRequest`,
  `RestingMarginReservation`); `cancel_all_orders_v2`; `get_historical_positions` (+
  `GetHistoricalPositionsParams`); `get_event_live_data` (+ `EventLiveData`,
  `GetEventLiveDataResponse`, `GetEventLiveDataParams`).
- [Rust API] `GetHistoricalCutoffResponse.market_positions_last_updated_ts`.
- [Rust API] `min_ts` and `subaccount` added to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams`.
- [Rust API] `GetFcmOrdersParams.client_order_ids`.
- [Rust API] `GetEventsParams.tickers` (comma-separated event ticker filter).
- [Rust API] `EventData.settlement_sources: Option<Vec<SettlementSource>>`.
- [Rust API] `Series.categories: Vec<String>`.
- [Rust API] RFQ-scoped quote action endpoints: `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`. `GetQuotesParams.min_ts`/`max_ts`/`user_filter`.
  `CreateRFQRequest`/`RFQ`/`Quote.target_cost_excludes_fees`. `RFQ.creator_subaccount`;
  `Quote.creator_subaccount`/`.rfq_creator_subaccount`/`.post_only`.
  `CreateQuoteRequest.post_only`.
- [Rust API] `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` (+ `Ref` variants) gained
  `subaccount: Option<i64>`.
- [Rust API] `WsMarketLifecycleV2`/`Ref` gained `exchange_index`, `strike_type`, `cap_strike`,
  `custom_strike`, and `price_ranges: Option<Vec<WsLifecyclePriceRange[Ref]>>`.
- [Rust API] `WsTrade`/`Ref` gained `is_block_trade: bool`.
- [Rust API] `WsUserOrder.exchange_index`.
- [Rust API] New WebSocket channels and message types: `WsChannelV2::PythValue` (+
  `WsPythValue[Ref]`, `WsPythUnderlyingList[Ref]`, `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList`, `underlying_tickers` on
  `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2`) and
  `WsChannelV2::CfbenchmarksValue5hz` (+ `WsCfBenchmarksValue5Hz[Ref]`,
  `WsCfBenchmarksIndexList5Hz[Ref]`).
- [Rust API] `FeeType::QuadraticWithComboMakerFees` variant (serialized
  `quadratic_with_combo_maker_fees`); already tolerated by the existing `#[serde(other)] Unknown`
  catch-all, now surfaced explicitly.
- [Rust API] `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) query params for
  `update_order_group_limit`.
- [Docs] `docs/spec-parity.md`: new notes on exchange sharding, the `Market` legacy-field cleanup,
  removed endpoints/channels, the `service` field's documented retention exception, the new
  `pyth_value`/`cfbenchmarks_value_5hz` channels, and the deliberately deferred entries.
- [Tests] Added/updated coverage for `GetExchangeStatusResponse` per-index breakdown,
  `SubaccountBalance.exchange_index`, historical fills/orders `min_ts`/`subaccount`, and removed
  stale assertions on deleted `Market`/`MarketPosition` fields.

### Changed

- [Rust API] `get_balance` now takes a `GetBalanceParams` argument (was zero-argument).
- [Rust API] `update_order_group_limit` now takes an additional `UpdateOrderGroupLimitParams`
  argument.

### Removed

- [Rust API] `Market` legacy compatibility fields not present in the live OpenAPI schema:
  `market_id`, `series_ticker`, `series_id`, `event_id`, `response_price_units`, `floor_price`,
  `cap_price`, `yes_bid`, `yes_ask`, `no_bid`, `no_ask`, `price`, `last_price`, `volume`,
  `volume_24h`, `open_interest`, `fractional_trading_enabled`, `notional_value`,
  `previous_yes_bid`, `previous_yes_ask`, `previous_price`, `liquidity`, `liquidity_fp`,
  `tick_size`, `settlement_value`, `created_ts`, `updated_ts`, `open_ts`, `close_ts`, `settled_ts`,
  `expiration_ts`, `resolution_source`, `event_title`, `can_trade`, `can_settle`. Use the
  `*_dollars`/`*_fp`/`*_time` fields instead.
- [Rust API] `MarketPosition.resting_orders_count` (confirmed removed from the live schema,
  2026-07-09). The dead, unused `MarketPositionRef`/`EventPositionRef` compatibility shim in
  `ws::types` (which aliased the wrong — REST — shape and was never wired into any WS parser) was
  also removed.
- [Rust API] `EventData.available_on_brokers` (removed from the live schema, 2026-09-10; deprecated
  2026-08-27).
- [Rust API] `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` (`GET /exchange/announcements` removed upstream
  2026-07-04).
- [Rust API] `GetQuotesParams.market_ticker`/`.event_ticker` (removed upstream 2026-06-20).
- [Rust API] `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`/`Response`, and the
  `get_multivariate_event_collection_lookup_history` /
  `lookup_tickers_for_market_in_multivariate_event_collection` methods (`PUT
  /multivariate_event_collections/{collection_ticker}/lookup` removed upstream 2026-08-06).
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsMultivariate`/`WsMultivariateRef` (the `multivariate` WS channel / `multivariate_lookup`
  message type removed upstream 2026-08-06).

### Fixed

- [Tests] Fixed a pre-existing compile error in `ws::types::envelope`'s
  `ws_message_from_bytes_list_subscriptions_accepts_numeric_shard_key` test (missing `sid`/`seq`
  fields in a `ListSubscriptions` match pattern), found while running the full test suite for this
  refresh; unrelated to the changelog reconciliation itself.
- [Docs] Corrected a stale example filename in `README.md`
  (`ws_user_orders_v2.rs` → `ws_user_orders.rs`).

### Breaking

- [Rust API] `get_balance()` signature changed to `get_balance(GetBalanceParams)`.
- [Rust API] `update_order_group_limit(order_group_id, body)` signature changed to
  `update_order_group_limit(order_group_id, params, body)`.
- [Rust API] `SubaccountBalance` gained a required `exchange_index: i64` field; downstream
  exhaustive struct construction must add it.
- [Rust API] All `Market` legacy compatibility fields listed under Removed are gone; downstream
  code reading them will not compile.
- [Rust API] `MarketPosition.resting_orders_count`, `EventData.available_on_brokers`,
  `GetQuotesParams.market_ticker`/`.event_ticker` are gone; downstream code reading/constructing
  them will not compile.
- [Rust API] `get_exchange_announcements` and its types, the multivariate lookup-history REST
  types/method, and the WS `Multivariate`/`MultivariateLookup` channel/message types are gone.
- [Rust API] `WsChannelV2`, `WsMsgType`, `WsUpdateAction`, `WsDataMessageV2`, and
  `WsDataMessageRef` gained new variants (`PythValue`, `CfbenchmarksValue5hz`, `SubscribeUnderlyings`,
  etc.); downstream exhaustive matches over these enums must add arms (or use `..`/wildcards).
  `FeeType` gained `QuadraticWithComboMakerFees`; same caveat.


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
