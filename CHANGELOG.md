# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-09

### Compatibility

- Docs snapshot: 2026-09-09
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-10

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition:**

| Date | Entry | Disposition |
|---|---|---|
| June 11, 2026 | Fractional quantities for RFQs | No code — `contracts_fp` already supported on `CreateRFQRequest`/quotes (0.6.0) |
| June 11, 2026 | Margin fee-tier endpoint returns active rates | No code — exchange-side bug fix, `GetMarginFeeTiersResponse` already untyped rate maps |
| June 11, 2026 | Perps mark prices on margin markets | No code — margin market types not in crate |
| June 11, 2026 | Tick size added to GET Margin Markets | No code — margin market types not in crate |
| June 11, 2026 | API usage volume progress endpoint | No code — new endpoint (`/account/api_usage_level/volume_progress`) not modeled; deferred, see spec-parity.md |
| June 11, 2026 | Self-serve Advanced API usage tier upgrade | No code — new endpoint not modeled; deferred, see spec-parity.md |
| June 11, 2026 | Perps volume and open interest notional fields | No code — margin market types not in crate |
| June 18, 2026 | RFQ quote identity on FIX | No code — FIX not modeled in this crate |
| June 18, 2026 | Trade entries in FIX market data | No code — FIX not modeled |
| June 18, 2026 | Event tickers filter on GET /trade-api/v2/events | Added `GetEventsParams::tickers` |
| June 18, 2026 | Legacy order mutation endpoints deprecated | No code — deprecation notice only; legacy endpoints remain live and modeled |
| June 18, 2026 | Quote time filters and pagination fix | Added `GetQuotesParams::min_ts`/`max_ts`; pagination bug fix is exchange-side only |
| June 18, 2026 | settlement_sources added to the events API | Added `EventData::settlement_sources` |
| June 18, 2026 | Block-trade accept API key permissions | No code — scopes already modeled as `Vec<String>` |
| June 18, 2026 | Sanity limits enforced on orderbook subscriptions | No code — operational limit only |
| June 18, 2026 | Strike type and cap strike on market_lifecycle_v2 metadata_updated | Added `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| June 19, 2026 | Communications RFQ and quote retention window reduced | No code — retention window is operational only |
| June 20, 2026 | RFQ quote market and event filters removed | Deprecated `GetQuotesParams::market_ticker`/`event_ticker` (`#[deprecated]`) |
| June 23, 2026 | Get Quote rate-limit cost reduced to 2 tokens | No code — rate-limit accounting only |
| June 24, 2026 | RFQ quotes support post-only on FIX | No code — FIX not modeled |
| June 25, 2026 | FIX exchange index routing | No code — FIX not modeled |
| June 25, 2026 | RFQ quote retention and RFQ-scoped quote actions | Added `get_quote_scoped`/`delete_quote_scoped`/`accept_quote_scoped`/`confirm_quote_scoped`; deprecated the quote-ID-only methods |
| June 25, 2026 | API usage tier qualification requirements halved | No code — business-rule change only |
| June 26, 2026 | Margin risk per-market metrics limited to single-position subaccounts and gross margin markets | No code — margin risk metrics not in crate |
| June 29, 2026 | Margin positions margin_used omitted for jointly-margined portfolio positions | No code — margin positions not in crate |
| June 30, 2026 | Trade-scoped API key permissions | No code — scopes already modeled as `Vec<String>` |
| July 2, 2026 | AcceptQuote rejects carry a specific reason on FIX | No code — FIX not modeled |
| July 2, 2026 | More specific FIX rejects for cancel/replace failures | No code — FIX not modeled |
| July 2, 2026 | Sub-account-restricted API keys | Added `subaccount`/`fcm_subtrader_id` to `CreateApiKeyRequest`/`GenerateApiKeyRequest` |
| July 2, 2026 | Margin positions now include an is_portfolio flag | No code — margin positions not in crate |
| July 2, 2026 | Multivariate lookup history endpoints are fully deprecated | Removed with the Aug 6 full removal (see below) |
| July 2, 2026 | Per-index exchange status | Added `GetExchangeStatusResponse::intra_exchange_transfers_active`/`exchange_index_statuses` (+ `ExchangeIndexStatus`) |
| July 2, 2026 | Per-index subaccount balances | Added `SubaccountBalance::exchange_index` |
| July 2, 2026 | price_ranges added to market_lifecycle_v2 events | Added `WsMarketLifecycleV2::price_ranges` |
| July 4, 2026 | Exchange announcements endpoint removed | Removed `get_exchange_announcements` and `GetExchangeAnnouncementsResponse`/`Announcement*` types |
| July 9, 2026 | Support for FIX Tag 2446 on Incremental Refresh | No code — FIX not modeled |
| July 9, 2026 | Margin orders now identify system order reasons | No code — margin orders not in crate |
| July 9, 2026 | Deprecated Predictions REST schema fields removed | Removed `Market::response_price_units`/`fractional_trading_enabled`, `MarketPosition::resting_orders_count` |
| July 9, 2026 | RFQ-scoped quote lookup endpoint | Added `get_quote_scoped` |
| July 22, 2026 | Incentive programs on hidden events excluded from listing | No code — server-side filtering only |
| July 23, 2026 | Subaccount-restricted API keys can quote on RFQ FIX sessions | No code — FIX not modeled |
| July 23, 2026 | Historical positions endpoint | Added `get_historical_positions`/`GetHistoricalPositionsParams` |
| July 23, 2026 | Order groups limited to 25,000 per user | No code — informational limit only |
| July 23, 2026 | New price level structures | No code — `price_level_structure` already modeled as raw `String` |
| July 23, 2026 | Pyth value WebSocket channel | Added `WsChannelV2::PythValue` + `pyth_value`/`pyth_value_underlying_list` messages and `subscribe_underlyings`/`unsubscribe_underlyings`/`underlying_list` update actions |
| July 23, 2026 | Subaccount-restricted API keys can open WebSocket sessions | No code — server-side auth/scoping behavior only |
| July 28, 2026 | The service field on error responses is deprecated | Removed with the Aug 6 full removal (see below) |
| July 30, 2026 | Event product_metadata now includes cadence | Added `EventMetadata::cadence` |
| July 30, 2026 | New endpoint for event-keyed live data | No code — new `live_data` endpoint out of current scope; deferred, see spec-parity.md |
| July 30, 2026 | Richer combo-validation errors on multivariate market creation | No code — richer `message`/`details` text, no shape change |
| July 30, 2026 | Series responses include exchange_index | Added `Series::exchange_index` |
| July 30, 2026 | Subaccount-restricted API keys can read order queue positions | No code — server-side auth/scoping behavior only |
| July 30, 2026 | Subaccount-restricted API keys can use batch order endpoints | No code — server-side auth/scoping behavior only |
| July 30, 2026 | Subaccount-restricted API keys can manage order groups | No code — server-side auth/scoping behavior only |
| July 30, 2026 | Lifecycle creation messages now include exchange_index | Added `exchange_index` to `WsMarketLifecycleV2` and `WsEventLifecycle` |
| July 30, 2026 | Subaccount on quote_created | Added `WsQuoteCreated::subaccount` |
| August 6, 2026 | FIX execution reports identify the source exchange index | No code — FIX not modeled |
| August 6, 2026 | Sided leverage estimates on margin markets | No code — margin market types not in crate |
| August 6, 2026 | Multivariate event collections include exchange_index | Added `MultivariateEventCollection::exchange_index` |
| August 6, 2026 | Order group limit updates support subaccounts | No code — `subaccount`/`exchange_index` query scoping for `update_order_group_limit` not yet exposed; deferred, see spec-parity.md |
| August 6, 2026 | The service field has been removed from error responses | Removed `ErrorResponse::service` |
| August 6, 2026 | Multivariate lookup endpoint and channel removed | Removed `get_multivariate_event_collection_lookup_history`/`lookup_tickers_for_market_in_multivariate_event_collection` and the WS `multivariate` channel/`multivariate_lookup` message |
| August 13, 2026 | Richer combo-validation errors on FIX RFQ creation | No code — FIX not modeled |
| August 13, 2026 | Margin order groups bind to single exchange_index | No code — margin order groups not in crate |
| August 13, 2026 | Balance reads scoped by exchange_index | Added `GetBalanceParams` (`subaccount`/`exchange_index`) and `GetBalanceResponse::balance_breakdown` |
| August 13, 2026 | Exchange shard descriptions | Added `ExchangeIndexStatus::description` |
| August 13, 2026 | Intra-account transfer history endpoints | No code — `/portfolio/intra_exchange_instance_transfer(s)` endpoints out of current scope; deferred, see spec-parity.md |
| August 13, 2026 | Order group maximum increased to 100,000 per user | No code — informational limit only |
| August 13, 2026 | New center_deci_edge_centi_cent price level structure | No code — `price_level_structure` already modeled as raw `String` |
| August 13, 2026 | Block trade indicator for WebSocket trades | Added `WsTrade::is_block_trade` |
| August 16, 2026 | API key location attestation expiry | No code — `api_key_region_expiration_ts` round-trips through `ApiKey::extra` |
| August 20, 2026 | Entry timestamps for FIX market data | No code — FIX not modeled |
| August 20, 2026 | Exit triggers on margin positions | No code — margin positions not in crate |
| August 20, 2026 | Cross-shard subaccount transfers | No code — `/portfolio/intra_exchange_instance_transfer` endpoint out of current scope; deferred, see spec-parity.md |
| August 20, 2026 | Exchange index filters for portfolio lists | Added `exchange_index` to `GetOrdersParams`/`GetFillsParams`/`GetPositionsParams` |
| August 20, 2026 | Kalshi Weather Index endpoint | No code — new weather-index domain out of current scope; deferred, see spec-parity.md |
| August 20, 2026 | Optional balance reads by exchange_index | Covered by `GetBalanceParams::exchange_index` (see Aug 13 entry above) |
| August 20, 2026 | RFQs and combo-market creation for sub-account-restricted API keys | No code — server-side auth/scoping behavior only |
| August 20, 2026 | Resting order value breakdown by exchange index | Added `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown` |
| August 20, 2026 | Target balance allocation endpoints | No code — new `/portfolio/target_balance_allocation` endpoints out of current scope; deferred, see spec-parity.md |
| August 20, 2026 | Maker fee exemption for independent NFL combo markets | No code — fee-schedule change only |
| August 20, 2026 | Exchange index on portfolio and WebSocket fill records | Added `exchange_index` to `Fill`, `Settlement`, `MarketPosition`, and `WsFill` |
| August 20, 2026 | VPC peering for Prime members | No code — connectivity/docs only |
| August 22, 2026 | Combo RFQ fee assignment for briefly resting orders | No code — fee-schedule change only |
| August 22, 2026 | Post-only quotes preserved; crossing rate limits may apply | No code — rate-limit/fee behavior only |
| August 24, 2026 | Upcoming exchange sharding | No code — informational rollout notice |
| August 27, 2026 | Trade type on FIX market data | No code — FIX not modeled |
| August 27, 2026 | Margin maker-volume incentive programs | No code — margin incentive programs not in crate |
| August 27, 2026 | Exchange auto-routing enabled by default | No code — server-side default routing behavior only |
| August 27, 2026 | Historical CF Benchmarks values via the REST passthrough | No code — CF Benchmarks REST passthrough endpoint out of current scope; deferred, see spec-parity.md |
| August 27, 2026 | Localized market content in REST responses | No code — `Accept-Language` request header not exposed as a typed client parameter; deferred, see spec-parity.md |
| August 27, 2026 | The available_on_brokers field on event responses is deprecated | Superseded by the Sep 10 full removal (see below) |
| August 27, 2026 | Cancel-all-orders endpoints | Added `cancel_all_orders` (Predictions `DELETE /portfolio/events/orders`); margin cancel-all out of scope (margin orders not modeled) |
| August 27, 2026 | Exchange index on user order messages | Added `WsUserOrder::exchange_index` |
| August 29, 2026 | Structured target images in Trade API v2 | No code — `StructuredTarget::details` is already untyped `Map<String, Value>` |
| August 31, 2026 | Weather index calibration history | No code — new weather-index domain out of current scope; deferred, see spec-parity.md |
| September 3, 2026 | ClearingBusinessDate on FIX trade execution reports | No code — FIX not modeled |
| September 3, 2026 | Higher FIX market data session limit | No code — FIX not modeled |
| September 3, 2026 | Order identity on FIX market data | No code — FIX not modeled |
| September 3, 2026 | Margin fee tier rates | No code — margin fee tiers already modeled with untyped rate maps tolerant of exchange-side changes |
| September 3, 2026 | Filter FCM orders by client order IDs | No code — `client_order_ids` FCM filter out of current scope; deferred, see spec-parity.md |
| September 3, 2026 | Filter historical positions by subaccount | Added `GetHistoricalPositionsParams::subaccount` |
| September 3, 2026 | Shard rebalance margin reservation | No code — margin-specific `target_balance_allocation` parameter out of current scope |
| September 3, 2026 | Correct remaining counts after crossing order amendments | No code — exchange-side bug fix only, response shape unchanged |
| September 3, 2026 | Lower rate-limit cost for cancel all orders | No code — rate-limit accounting only |
| September 3, 2026 | Tapered sub-cent pricing on multivariate (combo) markets | No code — `price_level_structure` already modeled as raw `String` |
| September 3, 2026 | CF Benchmarks 5Hz value websocket channel | Added `WsChannelV2::CfbenchmarksValue5Hz` + `cfbenchmarks_value_5hz`/`cfbenchmarks_value_5hz_indexlist` messages |
| September 10, 2026 | Principal-only sizing for target-cost RFQs | Added `CreateRFQRequest::target_cost_excludes_fees` |
| September 10, 2026 | Margin markets expose asset_class | No code — margin market types not in crate |
| September 10, 2026 | Margin taker-volume incentive programs | No code — margin incentive programs not in crate |
| September 10, 2026 | The deprecated available_on_brokers field is removed from event responses | Removed `EventData::available_on_brokers` |
| September 10, 2026 | Weather index points expose receipt_basis | No code — new weather-index domain out of current scope; deferred, see spec-parity.md |
| September 10, 2026 | Upcoming exchange sharding for commodities and basketball | No code — informational rollout notice |
| September 10, 2026 | The center_deci_edge_centi_cent price level structure is emitted again | No code — exchange-side bug fix only, already modeled as raw `String` |
| September 10, 2026 | WebSocket schemas corrected to match the messages the service sends | No code — documentation-only corrections (`sid`/`seq` already modeled; `market_id`/`market_ticker` never present in `WsError`; no hardcoded error-code enum to update) |

### Added

- [Rust API] Two new WebSocket channels, following the existing `cfbenchmarks_value` pattern:
  - `WsChannelV2::PythValue` (`pyth_value`, added 2026-07-23): real-time Pyth prices for configured
    underlying tickers. New types `WsPythValue`/`WsPythValueRef`, `WsPythUnderlyingList`/Ref, and
    `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` plus an
    `underlying_tickers` field on `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2`.
    `SubscriptionTracker` folds underlying add/remove updates into resubscribe state, mirroring the
    CF Benchmarks index-tracking logic.
  - `WsChannelV2::CfbenchmarksValue5Hz` (`cfbenchmarks_value_5hz`, added 2026-09-03): up to 5
    updates/sec on the subset of indices CF Benchmarks publishes at 200ms granularity. New types
    `WsCfBenchmarksValue5Hz`/Ref, `WsCfBenchmarksIndexList5Hz`/Ref. Reuses the existing
    `index_ids` / `SubscribeIndices` / `UnsubscribeIndices` / `Indexlist` update mechanics shared
    with `cfbenchmarks_value`.
- [Rust API] Added `cancel_all_orders(subaccount: Option<u32>)` (`DELETE /portfolio/events/orders`,
  2026-08-27) to cancel all resting Predictions event-market orders.
- [Rust API] Added `get_historical_positions()` / `GetHistoricalPositionsParams`
  (`GET /historical/positions`, 2026-07-23) for settled positions archived to the historical
  database, reusing `GetPositionsResponse`.
- [Rust API] Added RFQ-scoped quote action endpoints (2026-06-25 / 2026-07-09), mirroring the
  RFQ-scoped path Kalshi now recommends over the quote-ID-only endpoints:
  `get_quote_scoped`, `delete_quote_scoped`, `accept_quote_scoped`, `confirm_quote_scoped`.
  The quote-ID-only methods (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) are
  marked `#[deprecated]` (still functional; Kalshi has not removed the underlying endpoints).
- [Rust API] Added `exchange_index: Option<u32>` (multi-exchange-shard rollout, 2026-07-02 through
  2026-08-27) to: `WsMarketLifecycleV2` (created events only), `WsEventLifecycle`, `Series`,
  `MultivariateEventCollection`, `Fill`, `Settlement`, `MarketPosition` (REST and the WS
  `market_positions` zero-copy view), `WsFill`, `WsUserOrder`, `SubaccountBalance`, and as a new
  optional filter on `GetOrdersParams`, `GetFillsParams`, and `GetPositionsParams`. Kept `Option`
  even where the AsyncAPI marks it required, per this crate's established defensive-parsing
  convention for fields not yet observed on every exchange rollout stage.
- [Rust API] Added `GetExchangeStatusResponse::intra_exchange_transfers_active` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (new struct, with `description` added
  2026-08-13) for the per-exchange-index status breakdown (2026-07-02).
- [Rust API] Added `GetBalanceParams` (`subaccount`, `exchange_index`) and changed `get_balance` to
  take it; added `GetBalanceResponse::balance_breakdown: Option<Vec<IndexedBalance>>` (new struct)
  for the per-exchange-index balance breakdown (2026-08-13).
- [Rust API] Added `GetPortfolioRestingOrderTotalValueResponse::resting_order_value_breakdown:
  Option<Vec<IndexedBalance>>` (2026-08-20).
- [Rust API] Added `EventData::settlement_sources: Vec<SettlementSource>` (2026-06-18), mirroring
  the field already present on `Series`.
- [Rust API] Added `EventMetadata::cadence: Option<String>` (2026-07-30; documented in the Kalshi
  changelog but not yet in the published OpenAPI spec, so kept as a raw string) and
  `GetEventsParams::tickers`/`min_updated_ts` (event-ticker and updated-since filters).
- [Rust API] Added `WsMarketLifecycleV2::strike_type`/`cap_strike`/`custom_strike` (top-level,
  `metadata_updated` events only, 2026-06-18) and `price_ranges: Option<Vec<PriceRange>>` (`created`
  / `price_level_structure_updated` events, 2026-07-02).
- [Rust API] Added `WsTrade::is_block_trade: Option<bool>` (2026-08-13), matching the REST `Trade`
  field added in 0.6.0.
- [Rust API] Added `WsQuoteCreated::subaccount: Option<u32>` (2026-07-30).
- [Rust API] Added `CreateRFQRequest::target_cost_excludes_fees: Option<bool>` (2026-09-10).
- [Rust API] Added `GetQuotesParams::min_ts`/`max_ts` (2026-06-18).
- [Rust API] Added `subaccount`/`fcm_subtrader_id` to `CreateApiKeyRequest` and
  `GenerateApiKeyRequest` (2026-07-02) for sub-account-restricted and FCM-subtrader-bound API keys.
- [Rust API] Added `MultivariateEventCollection::exchange_index` (2026-08-06).
- [Tests] Added deterministic coverage for every new field/endpoint/channel above, including
  WS envelope routing tests for `pyth_value`, `pyth_value_underlying_list`, `cfbenchmarks_value_5hz`,
  and `cfbenchmarks_value_5hz_indexlist`, and a regression test confirming the retired
  `multivariate_lookup` message type now parses as `WsMessageV2::Unknown` instead of panicking.

### Changed

- [Rust API] `get_balance` now takes a `GetBalanceParams` argument instead of no arguments.

### Removed

- [Rust API] Removed `Market::response_price_units` and `Market::fractional_trading_enabled`,
  `MarketPosition::resting_orders_count`, and `EventData::available_on_brokers` — all removed from
  the OpenAPI schema (2026-07-09 and 2026-09-10 respectively). `MarketPosition` is shared between
  the REST `GetPositionsResponse` and the zero-copy WS `market_positions` view, so both surfaces
  lost the field together.
- [Rust API] Removed `WsMarketLifecycleV2::fractional_trading_enabled` (and its `Ref` mirror) — no
  longer present in the AsyncAPI schema.
- [Rust API] Removed `ErrorResponse::service` — removed from every REST error response
  (deprecated 2026-07-28, removed 2026-08-06).
- [Rust API] Removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, and `AnnouncementStatus` — `GET /exchange/announcements` was
  removed from the Predictions REST API on 2026-07-04. Use `get_exchange_schedule` instead.
- [Rust API] Removed `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types
  (`GetMultivariateEventCollectionLookupHistoryParams/Response`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest/Response`) — the
  `/multivariate_event_collections/{collection_ticker}/lookup` endpoint predated RFQs and was
  removed by Kalshi on 2026-08-06 (deprecated since 2026-07-02).
- [Rust API] Removed the WebSocket `multivariate` channel (`WsChannelV2::Multivariate`) and its
  `multivariate_lookup` message type (`WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsDataMessageV2::Multivariate`/`Ref`, and `WsMultivariate`/`WsMultivariateRef` in
  `ws::types::messages`) — removed by Kalshi on 2026-08-06. Subscribing to it now returns an
  unknown-channel error server-side; a frame using the old message-type string now parses as
  `WsMessageV2::Unknown` instead of the removed variant. Use `multivariate_market_lifecycle` for
  multivariate market state changes.

### Breaking

- [Rust API] All removals above are breaking. Downstream code reading `Market::response_price_units`
  / `fractional_trading_enabled`, `MarketPosition::resting_orders_count`, `EventData
  ::available_on_brokers`, `WsMarketLifecycleV2::fractional_trading_enabled`, or
  `ErrorResponse::service` must stop; none of these fields are sent by the exchange anymore.
- [Rust API] `get_exchange_announcements` and its response/announcement types no longer exist.
  Downstream code must remove all calls and matches against them.
- [Rust API] `get_multivariate_event_collection_lookup_history` and
  `lookup_tickers_for_market_in_multivariate_event_collection` (plus their request/response types)
  no longer exist.
- [Rust API] `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsDataMessageV2::Multivariate`, `WsDataMessageRef::Multivariate`, `WsMultivariate`, and
  `WsMultivariateRef` no longer exist. Downstream exhaustive matches over `WsChannelV2`,
  `WsMsgType`, `WsDataMessageV2`, or `WsDataMessageRef` must drop these arms.
- [Rust API] `get_balance` now requires a `GetBalanceParams` argument
  (`client.get_balance(GetBalanceParams::default())` preserves the old all-exchange-indexes,
  primary-account behavior).


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
