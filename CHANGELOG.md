# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-27

### Compatibility

- Docs snapshot: 2026-09-27
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-10-01

**Changelog entries since 0.7.0 watermark (2026-06-08) and disposition (140 entries):**

| Entry | Action |
|---|---|
| June 11, 2026 [REST, Predictions, Margin] New endpoint: `GET /trade-api/v2/account/api_usage_level/volume_progress` reports your ... | Added `get_account_api_usage_level_volume_progress` |
| June 11, 2026 [REST, Margin] Perps margin market responses now include mark prices and their timestamps | No code change — margin market types not in crate |
| June 11, 2026 [REST, Predictions, Margin] Users can now self-promote to the Advanced API tier by calling | Added `upgrade_account_api_usage_level` |
| June 11, 2026 [REST, Margin] `GET /trade-api/v2/margin/fee_tiers` now returns active maker and taker | No code change — exchange bug fix only (margin fee tiers modeled, shape unchanged) |
| June 11, 2026 [REST, WebSocket, Margin] Perps market data now includes dollar notional companions for lifetime volume, | No code change — margin market types not in crate |
| June 11, 2026 [REST, Margin] Margin market responses now report `tick_size` | No code change — margin market types not in crate |
| June 11, 2026 [REST, FIX, Predictions] RFQs will support fractional contract quantities beginning with the June 11, | No code change — `contracts_fp` already present in `CreateRFQRequest` |
| June 18, 2026 [REST, Predictions] The events API now returns `settlement_sources` on each event, mirroring the | Added `EventData.settlement_sources` |
| June 18, 2026 [WebSocket, Predictions] `metadata_updated` events on the `market_lifecycle_v2` channel now include | Added `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| June 18, 2026 [FIX, Predictions] FIX RFQ `Quote (35=S)` notifications sent to RFQ creators now include the | No code change — FIX API, not modeled |
| June 18, 2026 [FIX, Predictions, Margin] FIX market data incremental refreshes now include trades as `MDEntryType<269>=2` | No code change — FIX API, not modeled |
| June 18, 2026 [REST, Predictions] Legacy `/portfolio/orders` mutation endpoints will be deprecated sometime | Documented — legacy `/portfolio/orders` mutation endpoints not yet removed upstream |
| June 18, 2026 [REST, Predictions] `GET /trade-api/v2/events` now supports a `tickers` query parameter to | Added `GetEventsParams.tickers` |
| June 18, 2026 [REST, Predictions, Margin] API keys can use `read::block_trade_accept` and | No code change — scopes stored as `Vec<String>` already |
| June 18, 2026 [WebSocket, Predictions] Sanity limits enforced on orderbook subscriptions: | No code change — operational WS limits, no schema impact |
| June 18, 2026 [REST, Predictions] `GET /trade-api/v2/communications/quotes` now supports `min_ts` and `max_ts` | Added `min_ts`/`max_ts` to `GetQuotesParams` |
| June 19, 2026 [REST, Predictions] Closed RFQs and cancelled quotes returned by the communications APIs will be | No code change — operational retention-window change only |
| June 20, 2026 [REST, Predictions] `GET /trade-api/v2/communications/quotes` no longer supports filtering by | **Breaking** — removed `market_ticker`/`event_ticker` from `GetQuotesParams` |
| June 23, 2026 [REST, Predictions] `GET /trade-api/v2/communications/quotes/{quote_id}` will cost 2 tokens per | No code change — rate-limit token cost only |
| June 24, 2026 [FIX, Predictions] FIX RFQ `Quote (35=S)` creation now supports `ExecInst<18>=6` | No code change — FIX API, not modeled |
| June 25, 2026 [REST, FIX, Predictions] Effective immediately, RFQ quotes are no longer guaranteed to remain queryable | Added `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote` |
| June 25, 2026 [REST, Predictions] Qualification requirements for all tiers has been halved | No code change — operational tier-qualification change |
| June 25, 2026 [FIX, Predictions] FIX order entry now supports `ExDestination<100>` for exchange index | No code change — FIX API, not modeled |
| June 26, 2026 [REST, Margin] Effective immediately, `GET /trade-api/v2/margin/risk` no longer populates per-market m... | No code change — margin risk not modeled |
| June 29, 2026 [REST, Margin] `GET /trade-api/v2/margin/positions` now omits `margin_used` (and the derived | No code change — margin positions not modeled |
| June 30, 2026 [REST, Predictions, Margin] API keys can use `write::trade` to grant access to order, order-group, and | No code change — scopes stored as `Vec<String>` already |
| July 2, 2026 [REST, Predictions] Multivariate lookup history endpoints are fully deprecated | No code change at time of entry (see #57 for actual removal) |
| July 2, 2026 [REST, Margin] Each position returned by `GET /trade-api/v2/margin/risk` and | No code change — margin risk/positions not modeled |
| July 2, 2026 [WebSocket, Predictions] The `market_lifecycle_v2` channel now emits an optional `price_ranges` array | Added `WsMarketLifecycleV2.price_ranges` |
| July 2, 2026 [REST, Predictions] `GET /trade-api/v2/exchange/status` now returns two additional fields: | Added `GetExchangeStatusResponse.intra_exchange_transfers_active`/`exchange_index_statuses` |
| July 2, 2026 [REST, Predictions] `GET /trade-api/v2/portfolio/subaccounts/balances` now returns one balance per | Added `SubaccountBalance.exchange_index` |
| July 2, 2026 [FIX, Predictions] On `AcceptQuote (35=UA)`, when a quote can no longer be accepted the | No code change — FIX API, not modeled |
| July 2, 2026 [FIX, Predictions] and CxlRejReason<102> instead of INTERNAL_ERROR." | No code change — FIX API, not modeled |
| July 2, 2026 [REST, FIX, Predictions] You can now restrict an API key to a single sub-account when you create it | Added `ApiKey.subaccount`/`CreateApiKeyRequest.subaccount` |
| July 4, 2026 [REST, Predictions] `GET /trade-api/v2/exchange/announcements` has been removed from the Predictions | **Breaking** — removed `get_exchange_announcements` and `Announcement*` types |
| July 9, 2026 [FIX, Predictions, Margin] FIX Tag 2446 (`AggressorSide`) is now supported on `35=X` (Incremental Refresh) | No code change — FIX API, not modeled |
| July 9, 2026 [REST, Predictions] REST now supports looking up a quote within a specific RFQ by passing both the | Added `get_rfq_quote` (RFQ-scoped lookup; pairs with #21) |
| July 9, 2026 [REST, Predictions] The following deprecated fields have been removed from the Predictions REST API schema: | **Breaking** — removed `Market.response_price_units`/`fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| July 9, 2026 [REST, Margin] `GET /trade-api/v2/margin/orders` now includes an `order_reason` field when | No code change — margin orders not modeled |
| July 22, 2026 [REST, Predictions] `GET /incentive_programs` now excludes incentive programs whose market | No code change — visibility behavior only |
| July 23, 2026 [REST, Predictions, Margin] Attempts to create an order group after reaching the 25,000-group limit will | No code change — operational order-group limit |
| July 23, 2026 [REST, Predictions] Added `GET /historical/positions` — an authenticated endpoint for querying settled posi... | Added `get_historical_positions` |
| July 23, 2026 [WebSocket, Predictions, Margin] Subaccount-restricted API keys, previously denied at session start, can now | No code change — authorization/access behavior only |
| July 23, 2026 [FIX, Predictions] An API key restricted to a single subaccount (created via `POST /api_keys` | No code change — FIX API, not modeled |
| July 23, 2026 [WebSocket, Predictions] Authenticated WebSocket clients can subscribe to the new `pyth_value` channel | Deferred — `pyth_value` WebSocket channel not implemented this refresh (see spec-parity.md) |
| July 23, 2026 [REST, WebSocket, Predictions] Seven new `price_level_structure` values are being introduced: | No code change — `price_level_structure` modeled as raw `String`, not a closed enum |
| July 28, 2026 [REST, Predictions, Margin] The `service` field on error response bodies is deprecated and will be removed | Documented — `ErrorResponse.service` deprecated (see #62 for removal) |
| July 30, 2026 [REST, Predictions] `POST /trade-api/v2/multivariate_event_collections/{collection_ticker}` now | No code change — `ErrorResponse` already generic `code`/`message`/`details` |
| July 30, 2026 [WebSocket, Predictions] Market created messages on the `market_lifecycle_v2` and | Added `WsMarketLifecycleV2.exchange_index` |
| July 30, 2026 [REST, Predictions] `GET /series` now exposes `exchange_index`, identifying the target exchange instance fo... | Added `Series.exchange_index` |
| July 30, 2026 [REST, Predictions] A new endpoint, `GET /trade-api/v2/live_data/events/{event_ticker}`, returns | Deferred — `GET /live_data/events/{event_ticker}` not implemented this refresh |
| July 30, 2026 [REST, Predictions] API keys restricted to a single subaccount, previously rejected with a 403 | No code change — authorization/access behavior only |
| July 30, 2026 [REST, Predictions] Event objects returned by the REST API now include a `cadence` value inside | No code change — `product_metadata` is an opaque object, absorbed generically |
| July 30, 2026 [REST, Predictions] API keys restricted to a single subaccount, previously rejected with a 403 | No code change — authorization/access behavior only |
| July 30, 2026 [WebSocket, Predictions] The `quote_created` message on the `communications` channel now includes a | Added `WsQuoteCreated.subaccount` |
| July 30, 2026 [REST, Predictions, Margin] API keys restricted to a single subaccount can now use every REST order-group | No code change — authorization/access behavior only |
| August 6, 2026 [REST, WebSocket, Predictions] The deprecated multivariate lookup surface has been removed: | **Breaking** — removed multivariate lookup REST endpoints and the `multivariate` WS channel |
| August 6, 2026 [FIX, Predictions, Margin] Exchange-generated order and trade `ExecutionReport (35=8)` messages now include `LastM... | No code change — FIX API, not modeled |
| August 6, 2026 [REST, Margin] `GET /trade-api/v2/margin/markets` and `GET /trade-api/v2/margin/markets/{ticker}` | No code change — margin market types not in crate |
| August 6, 2026 [REST, Predictions] `PUT /portfolio/order_groups/{order_group_id}/limit` now supports the `subaccount` para... | Added `UpdateOrderGroupLimitRequest.subaccount` |
| August 6, 2026 [REST, Predictions] Multivariate event collection responses now include `exchange_index` | Added `MultivariateEventCollection.exchange_index` |
| August 6, 2026 [REST, Predictions, Margin] The `service` field announced as deprecated on July 28 has been removed from | **Breaking** — removed `ErrorResponse.service` |
| August 13, 2026 [REST, WebSocket, FIX, Predictions] A new `price_level_structure`, `center_deci_edge_centi_cent`, is available: | No code change — `price_level_structure` modeled as raw `String` |
| August 13, 2026 [REST, Predictions] `GET /trade-api/v2/portfolio/balance` now scopes `portfolio_value` to the | **Breaking** — `get_balance` now takes `GetBalanceParams`; added `balance_breakdown` |
| August 13, 2026 [WebSocket, Predictions] Predictions trade WebSocket messages now include `is_block_trade`, indicating | Added `WsTrade.is_block_trade` |
| August 13, 2026 [REST, Predictions] Each `exchange_index_statuses` entry now includes its shard `description` | Added `ExchangeIndexStatus.description` |
| August 13, 2026 [REST, Margin] Margin order groups are now bound to a single `exchange_index` | No code change — margin order groups not modeled |
| August 13, 2026 [REST, Predictions] The maximum number of order groups a user can have at a time is increasing | No code change — operational order-group limit |
| August 13, 2026 [FIX, Predictions] When an RFQ (`35=R`) selects legs that form an invalid multivariate | No code change — FIX API, not modeled |
| August 13, 2026 [REST, Predictions] Adding APIs to track intra-exchange account transfers: | Deferred — intra-exchange transfer endpoints not implemented this refresh |
| August 16, 2026 [REST, Predictions] `GET /trade-api/v2/api_keys` now returns `api_key_region_expiration_ts`, the | Added `GetApiKeysResponse.api_key_region_expiration_ts` |
| August 20, 2026 [WebSocket, FIX, Predictions, Margin] Members on the Prime tier or above can contact | No code change — connectivity/infra documentation only |
| August 20, 2026 [REST, Predictions] New endpoint: `GET /trade-api/v2/live_data/weather/{city}` serves the | Deferred — `GET /live_data/weather/{city}` not implemented this refresh |
| August 20, 2026 [REST, WebSocket, FIX, Predictions] Combo markets created after 11:59 PM ET on August 19, 2026 that are composed | No code change — fee-schedule/operational behavior only |
| August 20, 2026 [FIX, Predictions] and MDEntryTime<273>." | No code change — FIX API, not modeled |
| August 20, 2026 [REST, Predictions] `POST /trade-api/v2/portfolio/intra_exchange_instance_transfer` accepts | Deferred — pairs with #70 (intra-exchange transfers) |
| August 20, 2026 [REST, Predictions] New endpoints available for managing a target balance allocation across exchange shards | Deferred — `target_balance_allocation` endpoints not implemented this refresh |
| August 20, 2026 [REST, Predictions] `GET /trade-api/v2/portfolio/summary/total_resting_order_value` now returns | Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` |
| August 20, 2026 [REST, WebSocket, Predictions] Exchange index provided on REST fill, settlement, and market position responses and Web... | Added `exchange_index` to `Fill`/`Settlement`/`MarketPosition`/`WsFill` |
| August 20, 2026 [REST, Predictions] `GET /portfolio/orders`, `GET /portfolio/positions`, and `GET /portfolio/fills` now acc... | Added `exchange_index` filter to `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` |
| August 20, 2026 [REST, Predictions] Sub-account-restricted API keys can now use the | No code change — authorization/access behavior only |
| August 20, 2026 [REST, Predictions] `GET /trade-api/v2/portfolio/balance` returns `balance` and `portfolio_value` | Same as #64 (`GetBalanceParams`/`GetBalanceResponse`) |
| August 20, 2026 [REST, Margin] Stop-loss, take-profit, and trailing-stop triggers on margin positions are now available | No code change — margin positions not modeled |
| August 22, 2026 [REST, WebSocket, FIX, Predictions] As a result of trader feedback, the formerly announced planned change to | No code change — announcement only (post-only preserved) |
| August 22, 2026 [REST, WebSocket, FIX, Predictions] **Rollout timing:** | No code change — fee-schedule rollout timing only |
| August 24, 2026 [REST, WebSocket, FIX, Predictions] Upcoming exchange sharding: Crypto, Tennis, and Baseball will be provisioned | No code change — announcement only (exchange_index already modeled) |
| August 27, 2026 [REST, Predictions] Trade API v2 market responses can now return available Spanish or Portuguese | No code change — `Accept-Language` header, no schema change |
| August 27, 2026 [FIX, Predictions, Margin] ." | No code change — FIX API, not modeled |
| August 27, 2026 [WebSocket, Predictions] The `user_orders` WebSocket channel now includes `exchange_index` on each | Added `WsUserOrder.exchange_index` |
| August 27, 2026 [REST, Predictions, Margin] New endpoints cancel all resting Predictions or margin orders across every | Deferred — cancel-all-resting-orders endpoints not implemented this refresh |
| August 27, 2026 [REST, Predictions] The [CF Benchmarks REST Passthrough](/cfbenchmarks/rest-passthrough) page | No code change — documentation only |
| August 27, 2026 [REST, Predictions] The `available_on_brokers` field on the individual and batch `GET` events | Documented — `EventData.available_on_brokers` deprecated (see #109 for removal) |
| August 27, 2026 [REST, Predictions] Exchange auto-routing enabled by default when providing `market_ticker` and excluding `... | No code change — auto-routing default behavior only |
| August 27, 2026 [REST, Margin] `GET /trade-api/v2/incentive_programs` now accepts | No code change — `incentive_type` modeled as raw `String`; margin-only value |
| August 29, 2026 [REST, Predictions] Trade API v2 structured target responses now include the target's public | No code change — `StructuredTarget.details` is an opaque map, absorbed generically |
| August 31, 2026 [REST, Predictions] New endpoint `GET /live_data/weather/{city}/calibrations` returns the | Deferred — `GET /live_data/weather/{city}/calibrations` not implemented this refresh |
| September 3, 2026 [WebSocket, Predictions, Margin] The new `cfbenchmarks_value_5hz` websocket channel streams CF Benchmarks | Deferred — `cfbenchmarks_value_5hz` WebSocket channel not implemented this refresh |
| September 3, 2026 [FIX, Predictions, Margin] FIX market data sessions now support up to 100,000 active subscriptions. See | No code change — FIX API, not modeled |
| September 3, 2026 [FIX, Predictions, Margin] Book entries on `MarketDataIncrementalRefresh<35=X>` now include | No code change — FIX API, not modeled |
| September 3, 2026 [REST, Margin] New authenticated endpoint `GET /trade-api/v2/margin/fee_tier_rates` returns | No code change — new margin endpoint beyond modeled margin surface (fee_tiers/account limits only) |
| September 3, 2026 [REST, Predictions] `GET /fcm/orders` now accepts the `client_order_ids` query parameter, a | Added `GetFcmOrdersParams.client_order_ids` + validation (`subtrader_id` now optional) |
| September 3, 2026 [REST, Predictions] `GET /historical/positions` now accepts the `subaccount` query parameter to | Added `GetHistoricalPositionsParams.subaccount` |
| September 3, 2026 [REST, Predictions, Margin] `POST /trade-api/v2/portfolio/events/orders/{order_id}/amend` and | No code change — behavior/bug fix only |
| September 3, 2026 [REST, Predictions, Margin] Cancel-all requests now consume the same number of write tokens as cancelling | No code change — operational rate-limit token cost |
| September 3, 2026 [REST, Predictions] `POST /portfolio/target_balance_allocation` now accepts | Deferred — pairs with #77 (`target_balance_allocation`) |
| September 3, 2026 [FIX, Predictions] Predictions FIX Execution Reports with `ExecType=Trade` now include | No code change — FIX API, not modeled |
| September 3, 2026 [REST, WebSocket, FIX, Predictions] Multivariate (combo) markets are moving from `deci_cent` (a uniform \$0.001 | No code change — `price_level_structure` modeled as raw `String`, no new fields |
| September 10, 2026 [REST, FIX, Margin] Margin creates and amends now use per-shard Write budgets, shared across the API | No code change — margin/FIX write-budget routing only |
| September 10, 2026 [REST, Predictions] Breaking Change: the deprecated `available_on_brokers` field is removed | **Breaking** — removed `EventData.available_on_brokers` |
| September 10, 2026 [REST, FIX, Predictions] Target-cost RFQs can now opt out of fee-inclusive sizing. By default the | Added `target_cost_excludes_fees` to `CreateRFQRequest`/`Quote`/`RFQ` |
| September 10, 2026 [REST, Margin] `GET /trade-api/v2/incentive_programs` now accepts | No code change — `incentive_type` modeled as raw `String`; margin-only value |
| September 10, 2026 [REST, Predictions] `GetWeatherIndex` points now include an optional `receipt_basis` field | No code change — weather endpoint deferred (see #73/#96) |
| September 10, 2026 [REST, Margin] `GetMarginMarkets` and `GetMarginMarket` responses now include an | No code change — margin market types not in crate |
| September 10, 2026 [REST, WebSocket, FIX, Predictions] Starting at 12:00 PM ET on September 10, 2026, new commodities markets will | No code change — announcement only |
| September 10, 2026 [WebSocket, Predictions] Market lifecycle messages now report the `center_deci_edge_centi_cent` price | No code change — upstream bug fix; crate already tolerant (raw `String`) |
| September 10, 2026 [WebSocket, Predictions, Margin] The AsyncAPI specs described some messages inaccurately. The corrections below | No code change — AsyncAPI doc corrections describing existing behavior (verified: `seq`, error-message shape, error codes already match) |
| September 17, 2026 [REST, Margin] `GET /trade-api/v2/margin/markets` and `GET /trade-api/v2/margin/markets/{ticker}` | No code change — margin market types not in crate |
| September 17, 2026 [REST, Margin] Margin market GET responses now return the market's configured price increment in | No code change — margin market types not in crate |
| September 17, 2026 [WebSocket, Predictions, Margin] Updated the [Predictions](/asyncapi.yaml) and [Margin](/perps_asyncapi.yaml) | No code change — `WsTicker.dollar_volume`/`dollar_open_interest` already modeled as signed `i64` |
| September 17, 2026 [FIX, Predictions, Margin] `EventResendRequest` (`35=U1`) on `KalshiNR` and `KalshiDC` will require account | No code change — FIX API, not modeled |
| September 17, 2026 [REST, Predictions] `GET /trade-api/v2/historical/fills` and | Added `min_ts` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| September 17, 2026 [REST, FIX, Predictions] Reduced rate limit cost for QuoteConfirm when providing the RFQ ID | No code change — operational rate-limit cost |
| September 17, 2026 [REST, Predictions] `GET /trade-api/v2/portfolio/target_balance_allocation` now returns | Deferred — pairs with #77 (`target_balance_allocation`) |
| September 17, 2026 [REST, Predictions] Series objects now include `categories`, the list of discovery categories | Added `Series.categories` |
| September 17, 2026 [REST, WebSocket, FIX, Predictions] As of the September 17th maintenance window, new combos will be created on the | No code change — ticker/series-naming change only |
| September 17, 2026 [WebSocket, Predictions, Margin] Fixed a race that could skip events arriving immediately after a `subscribed` | No code change — upstream bug fix only |
| September 17, 2026 [REST, FIX, Predictions] RFQ and quote creation/cancellation, quote acceptance/confirmation, and | No code change — FIX/operational write-budget routing |
| September 24, 2026 [REST, WebSocket, FIX, Predictions, Margin] API keys may now be Ed25519 or RSA. Register an Ed25519 public key with | Added `key_type` field pass-through on API key request/response; Ed25519 signing itself not implemented (see spec-parity.md) |
| September 24, 2026 [REST, FIX, Predictions] Predictions read and write rate limits are increasing 20% for Premier, | No code change — operational rate-limit increase, no hardcoded values in crate |
| September 24, 2026 [WebSocket, Predictions, Margin] Public WebSocket connections support RFC 7692 `permessage-deflate` | No code change — transport-level (handled by `tokio-tungstenite`) |
| September 24, 2026 [REST, Predictions] `POST /trade-api/v2/portfolio/target_balance_allocation` now accepts | Deferred — pairs with #77 (`target_balance_allocation`) |
| September 24, 2026 [REST, Predictions] `GET /trade-api/v2/historical/fills` and `GET /trade-api/v2/historical/orders` | Added `subaccount` to `GetHistoricalFillsParams`/`GetHistoricalOrdersParams` |
| September 24, 2026 [REST, Predictions] `orders_updated_ts` in `GET /trade-api/v2/historical/cutoff` now advances | No code change — behavior only; `GetHistoricalCutoffResponse.orders_updated_ts` already modeled |
| October 1, 2026 [FIX, Predictions, Margin] `NewOrderSingle` (`35=D`) accepts `ExecInst` (`18`) `E` for reduce-only orders | No code change — FIX API, not modeled |
| October 1, 2026 [FIX, Margin] Margin FIX Execution Reports with `ExecType=Trade` now include | No code change — FIX API, not modeled |
| October 1, 2026 [REST, WebSocket, Predictions, Margin] `user_orders` messages now include `last_update_reason`. Reduce-only orders with | Added `WsUserOrder.last_update_reason` |
| October 1, 2026 [WebSocket, Predictions] Subscribe to [communications](/websockets/communications) with | Added `WsSubscriptionParamsV2.user_filter` |
| October 1, 2026 [REST, Predictions] Breaking Change: the deprecated `liquidity_dollars` field is removed from | **Breaking** — removed `Market.liquidity_dollars` (and stale `liquidity`/`liquidity_fp`) |
| October 1, 2026 [FIX, Predictions] FIX [RFQ and quote messages](/fix/rfq-messages) now include the creation time in `Trans... | No code change — FIX API, not modeled |
| October 1, 2026 [WebSocket, Margin] The Margin `ticker` WebSocket channel now includes `reference_price` for | No code change — margin market types not in crate |

### Added

- [Rust API] Added `get_account_api_usage_level_volume_progress()` / `GetAccountApiUsageLevelVolumeProgressResponse`
  for `GET /account/api_usage_level/volume_progress`, and `upgrade_account_api_usage_level()` for
  `POST /account/api_usage_level/upgrade` (2026-06-11).
- [Rust API] Added `EventData.settlement_sources: Vec<SettlementSource>` and `EventData.exchange_index`
  (2026-06-18 / 2026-07-30). Added `GetEventsParams.tickers` (2026-06-18).
- [Rust API] Added `strike_type`, `cap_strike`, `custom_strike`, `price_ranges: Vec<WsPriceRange>`, and
  `exchange_index` to `WsMarketLifecycleV2` / `WsMarketLifecycleV2Ref` (also covers
  `multivariate_market_lifecycle`, which reuses the same struct), and `exchange_index` to
  `WsEventLifecycle` / `WsEventLifecycleRef` (2026-06-18, 2026-07-02, 2026-07-30).
- [Rust API] Added `min_ts` / `max_ts` / `user_filter` to `GetQuotesParams` (2026-06-18); added new
  RFQ-scoped quote action endpoints `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`,
  `confirm_rfq_quote` (2026-06-25 / 2026-07-09), which upstream prefers over the still-supported but
  deprecated quote-ID-only endpoints.
- [Rust API] Added `GetExchangeStatusResponse.intra_exchange_transfers_active` and
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` (with `description`, added 2026-08-13)
  (2026-07-02).
- [Rust API] Added `SubaccountBalance.exchange_index` (2026-07-02); `ApiKey.subaccount` /
  `fcm_subtrader_id`, matching request fields on `CreateApiKeyRequest` / `GenerateApiKeyRequest`
  (2026-07-02); `GetApiKeysResponse.api_key_region_expiration_ts` (2026-08-16); and
  `key_type: Option<String>` on `GenerateApiKeyRequest` / `GenerateApiKeyResponse` for Ed25519 API
  keys (2026-09-24 — field pass-through only, see Deprecated/Known-gaps note below).
- [Rust API] Added `get_historical_positions()` / `GetHistoricalPositionsParams` for
  `GET /historical/positions` (2026-07-23), with `subaccount` (2026-09-03).
- [Rust API] Added `UpdateOrderGroupLimitRequest.subaccount` (2026-08-06) and
  `MultivariateEventCollection.exchange_index` (2026-08-06).
- [Rust API] Added `WsTrade.is_block_trade` (2026-08-13).
- [Rust API] Added `GetBalanceParams` (`subaccount`, `exchange_index` fields, now required by
  `get_balance`), `GetBalanceResponse.balance_breakdown: Option<Vec<IndexedBalance>>`
  (2026-08-13 / 2026-08-20).
- [Rust API] Added `exchange_index` filters to `GetOrdersParams`, `GetPositionsParams`,
  `GetFillsParams`, and `exchange_index` fields to `Order`, `Fill`, `Settlement`, `MarketPosition`,
  `WsFill`, `WsUserOrder` (2026-08-20 / 2026-08-27).
- [Rust API] Added `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`
  (2026-08-20).
- [Rust API] Added `WsUserOrder.exchange_index` (2026-08-27) and `WsUserOrder.last_update_reason`
  (2026-10-01).
- [Rust API] Added `FeeType::QuadraticWithComboMakerFees` (found directly in the live OpenAPI
  schema during this refresh, not explicitly called out in the changelog text).
- [Rust API] Added `Quote.post_only` / `target_cost_excludes_fees` / `creator_subaccount` /
  `rfq_creator_subaccount`; `RFQ.target_cost_excludes_fees` / `creator_subaccount`;
  `CreateRFQRequest.target_cost_excludes_fees` (2026-09-10). Added `subaccount` to
  `WsQuoteCreated`, `WsQuoteAccepted`, and `WsQuoteExecuted` (2026-07-30 — only `WsQuoteCreated` had
  actually been wired despite the 0.6.0 changelog claiming parity across all three), and
  `rfq_creator_id: Option<String>` to `WsQuoteCreated` / `WsQuoteAccepted` (a spec-required field
  found missing from both while auditing this channel; `WsQuoteExecuted` already had it).
- [Rust API] Added `WsSubscriptionParamsV2.user_filter` for the `communications` channel
  (2026-10-01).
- [Rust API] Added `Series.exchange_index` (2026-07-30) and `Series.categories: Vec<String>`
  (2026-09-17).
- [Rust API] Added `GetFcmOrdersParams.client_order_ids` (CSV, max 100) and a `validate()` check
  requiring at least one of `subtrader_id` / `client_order_ids` (2026-09-03).
- [Rust API] Added `min_ts` (2026-09-17) and `subaccount` (2026-09-24) to
  `GetHistoricalFillsParams` / `GetHistoricalOrdersParams`.

### Changed

- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from required `String` to `Option<String>`
  (2026-09-03, paired with `client_order_ids`).
- [Rust API] `WsFill.purchased_side` changed from `YesNo` to `Option<YesNo>`. The live AsyncAPI now
  marks this field `deprecated` in favor of `outcome_side` / `book_side` (the same migration
  already applied to `side` / `action`) while still listing it as required; made `Option`
  proactively rather than waiting for a future breaking removal.
- [Rust API] `get_balance()` now takes a `GetBalanceParams` argument instead of no arguments.

### Deprecated

- [Upstream] `ErrorResponse.service` was deprecated 2026-07-28 and removed 2026-08-06 (see Removed).
- [Upstream] `EventData.available_on_brokers` stopped being populated (always `false`) 2026-08-27
  and was removed from the schema 2026-09-10 (see Removed).
- [Upstream] The legacy quote-ID-only quote action endpoints (`delete_quote`, `accept_quote`,
  `confirm_quote`, `get_quote`) and the legacy `/portfolio/orders` mutation endpoints remain
  supported upstream but are deprecated in favor of the RFQ-scoped quote actions and the V2
  event-order endpoints, respectively. No Rust API change; documented for future removal.

### Removed

- [Rust API] Removed `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus` — `GET /exchange/announcements` was
  removed from the Predictions REST API 2026-07-04.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` — removed from the OpenAPI schema 2026-07-09.
- [Rust API] Removed `Market.liquidity_dollars` (2026-10-01 removal) along with the already-stale
  `Market.liquidity` / `Market.liquidity_fp` fields, which the live OpenAPI schema no longer
  documents either.
- [Rust API] Removed `EventData.available_on_brokers` (2026-09-10 removal).
- [Rust API] Removed `ErrorResponse.service` (2026-08-06 removal). Branch on `code` instead.
- [Rust API] Removed the multivariate ticker-pair lookup surface: REST
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`,
  `GetMultivariateEventCollectionLookupHistoryParams/Response`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest/Response`; and the `multivariate`
  WebSocket channel (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `MultivariateLookup`, `WsMultivariate`, `WsMultivariateRef`,
  `WsDataMessageV2::Multivariate` / `WsDataMessageRef::Multivariate`) — removed 2026-08-06. Use
  `create_market_in_multivariate_event_collection` / the RFQ communications APIs, and
  `multivariate_market_lifecycle` for multivariate market lifecycle state.
- [Rust API] Removed `GetQuotesParams.market_ticker` / `event_ticker` — `GET
  /communications/quotes` stopped supporting these filters 2026-06-20.

### Fixed

- [Tests] Fixed a pre-existing compile break in `src/ws/types/envelope.rs` test code
  (`WsMessageV2::ListSubscriptions` / `WsMessageRef::ListSubscriptions` match arms did not account
  for the `sid`/`seq` fields added in 0.7.0); unrelated to this refresh's upstream changes but
  needed for `cargo test --all-targets` to pass.

### Breaking

- [Rust API] `get_balance()` signature changed (now requires `GetBalanceParams`). Update call sites
  to `client.get_balance(GetBalanceParams::default())` or supply `subaccount` / `exchange_index`.
- [Rust API] `GetFcmOrdersParams.subtrader_id` changed from `String` to `Option<String>`; at least
  one of `subtrader_id` / `client_order_ids` is now required (enforced by `validate()`).
- [Rust API] `WsFill.purchased_side` changed from `YesNo` to `Option<YesNo>`.
- [Rust API] All of the removals listed above are breaking: downstream code referencing
  `get_exchange_announcements`, `Market.response_price_units` / `.fractional_trading_enabled` /
  `.liquidity_dollars`, `MarketPosition.resting_orders_count`, `EventData.available_on_brokers`,
  `ErrorResponse.service`, the multivariate lookup REST/WS surface, or
  `GetQuotesParams.market_ticker` / `.event_ticker` will not compile and must be updated per the
  Removed section above.

### Known gaps / deferred (documented, not implemented this refresh)

See `docs/spec-parity.md` for full detail. Deferred as additive, non-breaking work for a future
patch/minor release: the `pyth_value` and `cfbenchmarks_value_5hz` WebSocket channels; `GET
/live_data/events/{event_ticker}` and `GET /live_data/weather/{city}[/calibrations]`;
`POST`/`GET /portfolio/target_balance_allocation` and `resting_margin_reservation`; the
intra-exchange-instance-transfer endpoints; the cancel-all-resting-orders endpoints; and full
Ed25519 request signing in `auth.rs` (only the `key_type` API-key field was added).


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
