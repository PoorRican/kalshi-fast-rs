# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.5.0] - 2026-05-30

### Compatibility

- Docs snapshot: 2026-05-30
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-05-26

### Changelog entries since 2026-04-16 watermark → code mapping

| Entry (approx date) | Disposition |
|---|---|
| External API hosts (`external-api.kalshi.com`, `external-api-ws.kalshi.com`, demo equivalents) | Added `KalshiEnvironment::external()` and `external_demo()` in `env.rs` |
| `BookSide` enum (`bid`/`ask`) added to V2 order vocabulary | Added `BookSide` enum in `types.rs` |
| `outcome_side` + `book_side` required on `Order` response | Added as required fields; deprecated `side`/`action` as `Option` |
| `outcome_side` + `book_side` required on `Fill` response | Added as required fields; deprecated `side`/`action` as `Option` |
| `taker_outcome_side` + `taker_book_side` required on `Trade` response | Added as required fields; deprecated `taker_side` as `Option` |
| `balance_dollars` required on `GetBalanceResponse` | Added `balance_dollars: FixedPointDollars` |
| `subaccount` required on `CreateOrderGroupResponse` | Added `subaccount: u32` |
| V2 single order endpoints (`POST /portfolio/orders/v2`, `DELETE`, `PUT`, `PATCH /decrease`) | Added `CreateOrderV2Request/Response`, `CancelOrderV2{Params,Response}`, `AmendOrderV2Request/Response`, `DecreaseOrderV2Request/Response` and client methods |
| V2 batch endpoints (`POST /portfolio/batch_orders/v2`, `DELETE`) | Added `BatchCreateOrdersV2Request/Response`, `BatchCancelOrdersV2Request/Response` and client methods |
| `rfq_user_filter` param added to `GET /portfolio/communications/quotes` | Added `rfq_user_filter: Option<String>` to `GetQuotesParams` |
| WS `fill` channel: `outcome_side` + `book_side` required; `side`/`action`/`purchased_side` deprecated | Updated `WsFill` / `WsFillRef` |
| WS `trade` channel: `taker_outcome_side` + `taker_book_side` required; `taker_side` deprecated | Updated `WsTrade` / `WsTradeRef` |
| WS `user_order` channel: `outcome_side` + `book_side` added | Updated `WsUserOrder` |
| WS `market_lifecycle_v2` channel: `metadata_updated` event type; `floor_strike` + `yes_sub_title` optional fields | Added `MetadataUpdated` variant and new optional fields to `WsMarketLifecycleV2` |
| AsyncAPI version unchanged (2.0.0) | No version-level changes needed |

### Added

- [Rust API] `KalshiEnvironment::external()` — REST `https://external-api.kalshi.com` / WS `wss://external-api-ws.kalshi.com/trade-api/ws/v2`; for direct/non-FCM members.
- [Rust API] `KalshiEnvironment::external_demo()` — demo equivalent for direct members.
- [Rust API] `BookSide` enum (`Bid` | `Ask` | `Unknown`) in `types.rs`; serialises as `"bid"` / `"ask"`.
- [Rust API] `outcome_side: YesNo` and `book_side: BookSide` required fields on `Order`, `Fill`.
- [Rust API] `taker_outcome_side: TradeTakerSide` and `taker_book_side: BookSide` required fields on `Trade`.
- [Rust API] `balance_dollars: FixedPointDollars` required field on `GetBalanceResponse`.
- [Rust API] `subaccount: u32` required field on `CreateOrderGroupResponse`.
- [Rust API] Full V2 single-order surface: `CreateOrderV2Request`, `CreateOrderV2Response`, `CancelOrderV2Params`, `CancelOrderV2Response`, `AmendOrderV2Request`, `AmendOrderV2Response`, `DecreaseOrderV2Request`, `DecreaseOrderV2Response`.
- [Rust API] Full V2 batch-order surface: `BatchCreateOrdersV2Request`, `BatchCreateOrdersV2Response`, `BatchCreateOrdersV2IndividualResponse`, `BatchCancelOrdersV2Request`, `BatchCancelOrdersV2RequestOrder`, `BatchCancelOrdersV2Response`, `BatchCancelOrdersV2IndividualResponse`.
- [Rust API] `KalshiRestClient` methods: `create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`.
- [Rust API] `rfq_user_filter: Option<String>` on `GetQuotesParams`.
- [Rust API] `outcome_side: YesNo` and `book_side: BookSide` required fields on `WsFill` / `WsFillRef`.
- [Rust API] `taker_outcome_side` and `taker_book_side` required fields on `WsTrade` / `WsTradeRef`.
- [Rust API] `outcome_side: Option<YesNo>` and `book_side: Option<BookSide>` on `WsUserOrder`.
- [Rust API] `WsMarketLifecycleEventType::MetadataUpdated` variant.
- [Rust API] `floor_strike: Option<f64>` and `yes_sub_title: Option<String>` on `WsMarketLifecycleV2` and borrowed `WsMarketLifecycleV2Ref`.

### Changed

- [Rust API] `Order::side` and `Order::action` are now `Option<_>` and marked `#[deprecated]`; use `outcome_side` / `book_side`.
- [Rust API] `Fill::side` and `Fill::action` are now `Option<_>` and marked `#[deprecated]`; use `outcome_side` / `book_side`.
- [Rust API] `Trade::taker_side` is now `Option<TradeTakerSide>` and marked `#[deprecated]`; use `taker_outcome_side`.
- [Rust API] `WsFill::side`, `WsFill::action`, `WsFill::purchased_side` are now `Option<_>` and marked `#[deprecated]`.
- [Rust API] `WsTrade::taker_side` is now `Option<_>` and marked `#[deprecated]`.
- [Rust API] `WsUserOrder::side` and `WsUserOrder::is_yes` are now `Option<_>` and marked `#[deprecated]`.
- [Tests] Updated all parsing fixtures to include new required fields (`outcome_side`, `book_side`, `taker_outcome_side`, `taker_book_side`, `balance_dollars`).

### Breaking

- [Rust API] `Order::side` type changed from `YesNo` to `Option<YesNo>` — downstream `match` or direct field access must handle `Option`.
- [Rust API] `Order::action` type changed from `BuySell` to `Option<BuySell>`.
- [Rust API] `Fill::side` type changed from `YesNo` to `Option<YesNo>`.
- [Rust API] `Fill::action` type changed from `BuySell` to `Option<BuySell>`.
- [Rust API] `Trade::taker_side` type changed from `TradeTakerSide` to `Option<TradeTakerSide>`.
- [Rust API] `GetBalanceResponse` gains a required `balance_dollars` field — fixture structs in downstream tests must include it.
- [Rust API] `CreateOrderGroupResponse` gains a required `subaccount` field.

> **Version bump**: 0.4.1 → 0.5.0. Per VERSIONING.md §Pre-1.0: "any breaking Rust API change is a minor bump." Seven type changes from non-optional to `Option` qualify as breaking.

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
