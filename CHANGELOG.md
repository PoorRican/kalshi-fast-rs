# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-10

### Compatibility

- Docs snapshot: 2026-09-10
- OpenAPI: 3.30.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-09-10

### Version Bump

**Minor** (0.7.0 → 0.8.0). Per `VERSIONING.md` "Version Bump Rules For Refreshes": deprecated or
removed upstream fields/endpoints forced multiple breaking Rust API changes this refresh
(struct fields removed, a method signature changed, WebSocket enum variants removed), so this
is a minor release, not a patch.

**Changelog entries since the 2026-06-08 watermark and disposition** (Margin-exchange-only and
FIX-only entries are out of scope for this crate, which models the Predictions REST/WebSocket
surface only, per `CLAUDE.md`):

| Date | Entry | Disposition |
|---|---|---|
| 06-11 | API usage volume progress endpoint | Added `get_account_api_usage_level_volume_progress` |
| 06-11 | Perps mark prices on margin markets | No change — margin market types not in crate |
| 06-11 | Self-serve Advanced API usage tier upgrade | Added `upgrade_account_api_usage_level` |
| 06-11 | Margin fee-tier endpoint returns active rates | No change — exchange behavior only, margin |
| 06-11 | Perps volume/OI notional fields | No change — margin market types not in crate |
| 06-11 | Tick size on `GET /margin/markets` | No change — margin market types not in crate |
| 06-11 | Fractional quantities for RFQs | No change — `contracts_fp` already present |
| 06-18 | `settlement_sources` on events API | Added to `EventData` |
| 06-18 | Strike type/cap strike on `metadata_updated` | Added `strike_type`/`cap_strike`/`custom_strike` to `WsMarketLifecycleV2` |
| 06-18 | RFQ quote identity on FIX | No change — FIX not modeled |
| 06-18 | Trade entries in FIX market data | No change — FIX not modeled, margin |
| 06-18 | Legacy order mutation endpoints deprecated | **Deprecated** `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/`batch_cancel_orders` (confirmed fully removed from the live API — see Known Issues) |
| 06-18 | `tickers` filter on `GET /events` | Added to `GetEventsParams` |
| 06-18 | Block-trade accept API key permissions | No change — scopes stored as `Vec<String>` already |
| 06-18 | Sanity limits on orderbook subscriptions | No change — server-side limit only |
| 06-18 | Quote time filters + pagination fix | Added `min_ts`/`max_ts` to `GetQuotesParams`; pagination fix is server-side |
| 06-19 | RFQ/quote retention window reduced | No change — server-side retention policy only |
| 06-20 | RFQ quote market/event filters removed | **Removed** `market_ticker`/`event_ticker` from `GetQuotesParams` |
| 06-23 | Get Quote rate-limit cost reduced | No change — billing only |
| 06-24 | RFQ quotes support post-only on FIX | No change — FIX not modeled |
| 06-25 | RFQ quote retention + RFQ-scoped quote actions | Added RFQ-scoped quote action endpoints; FIX part out of scope |
| 06-25 | API usage tier qualification halved | No change — business policy only |
| 06-25 | FIX exchange index routing | No change — FIX not modeled |
| 06-26 | Margin risk per-market metrics limited | No change — margin not modeled |
| 06-29 | Margin `margin_used` omitted for joint positions | No change — margin not modeled |
| 06-30 | Trade-scoped API key permissions | No change — scopes stored as `Vec<String>` already |
| 07-02 | Multivariate lookup history endpoints deprecated | Superseded by 08-06 full removal |
| 07-02 | Margin `is_portfolio` flag | No change — margin not modeled |
| 07-02 | `price_ranges` on `market_lifecycle_v2` events | Added to `WsMarketLifecycleV2`/`Ref` |
| 07-02 | Per-index exchange status | Added `exchange_index_statuses`/`intra_exchange_transfers_active` to `GetExchangeStatusResponse` |
| 07-02 | Per-index subaccount balances | Added `exchange_index` to `SubaccountBalance` (breaking) |
| 07-02 | AcceptQuote FIX reject reasons | No change — FIX not modeled |
| 07-02 | FIX cancel/replace reject reasons | No change — FIX not modeled |
| 07-02 | Sub-account-restricted API keys | Added `subaccount` to `CreateApiKeyRequest`/`GenerateApiKeyRequest`/`ApiKey` |
| 07-04 | Exchange announcements endpoint removed | **Removed** `get_exchange_announcements` and `Announcement`/`AnnouncementType`/`AnnouncementStatus`/`GetExchangeAnnouncementsResponse` |
| 07-09 | FIX Tag 2446 on Incremental Refresh | No change — FIX not modeled |
| 07-09 | RFQ-scoped quote lookup endpoint | Added `get_quote_scoped` (and scoped delete/accept/confirm); old quote-ID-only methods deprecated |
| 07-09 | Deprecated Predictions REST schema fields removed | **Removed** `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count` |
| 07-09 | Margin order `order_reason` | No change — margin not modeled |
| 07-22 | Incentive programs on hidden events excluded | No change — server-side filtering only |
| 07-23 | Order groups limited to 25,000 | No change — limit only |
| 07-23 | Historical positions endpoint | Added `get_historical_positions` (`GET /historical/positions`) |
| 07-23 | Subaccount-restricted keys can open WS sessions | No change — auth semantics only, no new fields |
| 07-23 | Subaccount-restricted keys can quote on RFQ FIX | No change — FIX not modeled |
| 07-23 | `pyth_value` WebSocket channel | Added: `WsChannelV2::PythValue`, new `ws::types::messages::pyth` module, `WsUpdateAction::{Subscribe,Unsubscribe}Underlyings`/`UnderlyingList` |
| 07-23 | New `price_level_structure` values | No change — modeled as plain `String`, not a closed enum |
| 07-28 | `service` field on errors deprecated | Superseded by 08-06 removal |
| 07-30 | Richer combo-validation errors | No change — already carried in `ErrorResponse.message`/`.details` (plain strings) |
| 07-30 | `exchange_index` on lifecycle creation messages | Added to `WsMarketLifecycleV2`/`WsEventLifecycle` |
| 07-30 | `exchange_index` on series responses | Added to `Series` |
| 07-30 | Event-keyed live data endpoint | Added `get_event_live_data` (`GET /live_data/events/{event_ticker}`) |
| 07-30 | Subaccount-restricted keys read queue positions | No change — auth semantics only |
| 07-30 | Event `product_metadata.cadence` | No change — `product_metadata` is an opaque object in the live schema; passes through `EventMetadata.extra` |
| 07-30 | Subaccount-restricted keys use batch order endpoints | No change — auth semantics only |
| 07-30 | `subaccount` on `quote_created` | Added to `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted` (live schema has it on all three) |
| 07-30 | Subaccount-restricted keys manage order groups | No change — auth semantics only |
| 08-06 | Multivariate lookup endpoint + channel removed | **Removed** REST lookup/history methods and types; **removed** WS `Multivariate` channel/message family (deleted `ws/types/messages/multivariate.rs`) |
| 08-06 | FIX execution reports `LastMkt` | No change — FIX not modeled |
| 08-06 | Sided leverage estimates on margin markets | No change — margin not modeled |
| 08-06 | Order group limit updates support subaccounts | Added `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) |
| 08-06 | `exchange_index` on multivariate event collections | Added to `MultivariateEventCollection` |
| 08-06 | `service` field removed from errors | **Removed** `ErrorResponse.service` |
| 08-13 | `center_deci_edge_centi_cent` price level structure | No change — plain `String` |
| 08-13 | Balance reads scoped by `exchange_index` | **Changed** `get_balance()` now takes `GetBalanceParams { subaccount, exchange_index }` (breaking) |
| 08-13 | Block trade indicator for WS trades | Added `is_block_trade` to `WsTrade`/`WsTradeRef` |
| 08-13 | Exchange shard descriptions | Added `description` to `ExchangeIndexStatus` |
| 08-13 | Margin order groups bind to `exchange_index` | No change — margin not modeled |
| 08-13 | Order group maximum increased to 100,000 | No change — limit only |
| 08-13 | Richer combo-validation errors on FIX RFQ | No change — FIX not modeled |
| 08-13 | Intra-account transfer history endpoints | Added `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer` |
| 08-16 | API key location attestation expiry | Added `api_key_region_expiration_ts` to `GetApiKeysResponse` |
| 08-20 | VPC peering for Prime members | No change — connectivity/infra only |
| 08-20 | Kalshi Weather Index endpoint | Added `get_weather_index` (`GET /live_data/weather/{city}`) |
| 08-20 | Maker fee exemption, independent NFL combo markets | No change — fee business rule only |
| 08-20 | Entry timestamps for FIX market data | No change — FIX not modeled |
| 08-20 | Cross-shard subaccount transfers | Added `source_subaccount`/`destination_subaccount` to `IntraExchangeInstanceTransferRequest` |
| 08-20 | Target balance allocation endpoints | Added `get_target_balance_allocation`/`set_target_balance_allocation` |
| 08-20 | Resting order value breakdown by exchange index | Added `resting_order_value_breakdown` to `GetPortfolioRestingOrderTotalValueResponse` |
| 08-20 | `exchange_index` on portfolio/WS fill records | Added to `Fill`, `Settlement`, `MarketPosition` (REST) and `WsFill`/`WsFillRef` |
| 08-20 | `exchange_index` filters for portfolio lists | Added to `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams` |
| 08-20 | RFQs/combo creation for sub-account-restricted keys | No change — auth semantics only |
| 08-20 | Optional balance reads by `exchange_index` | Covered by the 08-13 `GetBalanceParams` change above |
| 08-20 | Exit triggers on margin positions | No change — margin not modeled |
| 08-22 | Post-only quotes preserved; crossing rate limits | No change — policy/rate-limit only, no schema impact |
| 08-22 | Combo RFQ fee assignment for briefly resting orders | No change — fee calculation only |
| 08-24 | Upcoming exchange sharding (crypto/tennis/baseball) | No change — announcement only |
| 08-27 | Localized market content (`Accept-Language`) | No change — already supported via `KalshiRestClientBuilder::with_default_headers` |
| 08-27 | Trade type on FIX market data | No change — FIX not modeled |
| 08-27 | `exchange_index` on `user_orders` WS channel | Added to `WsUserOrder` |
| 08-27 | Cancel-all-orders endpoints | Added `cancel_all_orders_v2` (`DELETE /portfolio/events/orders`) |
| 08-27 | Historical CF Benchmarks via REST passthrough | No change — the passthrough has no fixed OpenAPI schema to bind to (documentation-only change) |
| 08-27 | `available_on_brokers` deprecated | Superseded by 09-10 removal |
| 08-27 | Exchange auto-routing enabled by default | No change — server routing behavior only |
| 08-27 | Margin maker-volume incentive programs | No change — margin not modeled |
| 08-29 | Structured target `details.image_url` | No change — `details` is an opaque object in the live schema; passes through unchanged |
| 08-31 | Weather index calibration history | Added `get_weather_index_calibrations` |
| 09-03 | `cfbenchmarks_value_5hz` WebSocket channel | Added `WsChannelV2::CfbenchmarksValue5hz` and 5Hz message types |
| 09-03 | Higher FIX market data session limit | No change — FIX not modeled |
| 09-03 | Order identity on FIX market data | No change — FIX not modeled |
| 09-03 | Margin fee tier rates endpoint | No change — margin not modeled |
| 09-03 | Filter FCM orders by client order IDs | Added `client_order_ids` to `GetFcmOrdersParams` |
| 09-03 | Filter historical positions by subaccount | Added `subaccount` to `GetHistoricalPositionsParams` |
| 09-03 | Correct remaining counts after crossing amendments | No change — server-side bug fix, no schema impact |
| 09-03 | Lower rate-limit cost for cancel all orders | No change — billing only |
| 09-03 | Shard rebalance margin reservation | Added `resting_margin_reservation` to target balance allocation |
| 09-03 | `ClearingBusinessDate` on FIX trade reports | No change — FIX not modeled |
| 09-03 | Tapered sub-cent pricing on multivariate markets | No change — plain `String` structure name, no new fields |
| 09-10 | `available_on_brokers` removed | **Removed** `EventData.available_on_brokers` |
| 09-10 | Principal-only sizing for target-cost RFQs | Added `target_cost_excludes_fees` to `CreateRFQRequest` |
| 09-10 | Margin taker-volume incentive programs | No change — margin not modeled |
| 09-10 | Weather index `receipt_basis` | Added to weather index point type |
| 09-10 | Margin markets expose `asset_class` | No change — margin not modeled |
| 09-10 | Upcoming exchange sharding (commodities/basketball) | No change — announcement only |
| 09-10 | `center_deci_edge_centi_cent` emitted correctly | No change — server-side bug fix; already a plain `String` |
| 09-10 | WebSocket schemas corrected | No schema change needed (`seq`/`sid` already generic on the envelope, error codes already a plain integer, `market_id`/`market_ticker` never modeled on errors) — but this verification pass found and fixed a real, unrelated bug: see Fixed |

### Added

- [Rust API] `get_account_api_usage_level_volume_progress()` / `upgrade_account_api_usage_level()`
  and `GetAccountApiUsageLevelVolumeProgressResponse` for the new volume-tier endpoints.
- [Rust API] `exchange_index_statuses: Vec<ExchangeIndexStatus>` and `intra_exchange_transfers_active`
  on `GetExchangeStatusResponse`; `ExchangeIndexStatus.description`.
- [Rust API] `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer`,
  and `create_intra_exchange_instance_transfer` (with `source_subaccount`/`destination_subaccount`)
  for the new intra-exchange-instance transfer family, plus `IntraExchangeInstanceTransfer` and
  friends.
- [Rust API] `api_key_region_expiration_ts` on `GetApiKeysResponse`; `subaccount` on `ApiKey`,
  `CreateApiKeyRequest`, `GenerateApiKeyRequest`.
- [Rust API] `settlement_sources` on `EventData`; `tickers` filter on `GetEventsParams`.
- [Rust API] `exchange_index` on `Series` and on `MultivariateEventCollection`.
- [Rust API] `get_historical_positions()` (`GET /historical/positions`, with `ticker`/`event_ticker`/
  `subaccount` filters).
- [Rust API] `get_event_live_data()` (`GET /live_data/events/{event_ticker}`) with `EventLiveData`'s
  flexible `type`/`details` shape, matching the crate's existing live-data pattern.
- [Rust API] Full Kalshi Weather Index support: `get_weather_index()`, `get_weather_index_calibrations()`,
  `WeatherIndexPoint` (canonical/incomplete/detailed shapes, `receipt_basis`), `WeatherIndexStationReading`,
  `WeatherIndexCalibration`.
- [Rust API] `get_target_balance_allocation()` / `set_target_balance_allocation()` and
  `RestingMarginReservation`.
- [Rust API] `resting_order_value_breakdown` on `GetPortfolioRestingOrderTotalValueResponse`;
  `balance_breakdown` on `GetBalanceResponse` (both `Vec<IndexedBalance>`).
- [Rust API] `exchange_index` on `Fill`, `Settlement`, `MarketPosition` (REST), and on `WsFill`/
  `WsFillRef`, `WsUserOrder`, `WsMarketLifecycleV2`/`Ref`, `WsEventLifecycle`/`Ref`.
- [Rust API] `exchange_index` query filter on `GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`.
- [Rust API] `cancel_all_orders_v2()` (`DELETE /portfolio/events/orders`).
- [Rust API] `client_order_ids` filter on `GetFcmOrdersParams`.
- [Rust API] `get_quote_scoped`/`delete_quote_scoped`/`accept_quote_scoped`/`confirm_quote_scoped`
  (RFQ-scoped quote actions); `min_ts`/`max_ts` on `GetQuotesParams`; `target_cost_excludes_fees`
  on `CreateRFQRequest`.
- [Rust API] `UpdateOrderGroupLimitParams` (`subaccount`, `exchange_index`) on `update_order_group_limit`.
- [Rust API] `strike_type`/`cap_strike`/`custom_strike` on `WsMarketLifecycleV2`/`Ref` `metadata_updated`
  events; `price_ranges` on `created`/`price_level_structure_updated` events.
- [Rust API] `is_block_trade` on `WsTrade`/`WsTradeRef`.
- [Rust API] `subaccount` on `WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted`.
- [Rust API] New `pyth_value` WebSocket channel: `WsChannelV2::PythValue`, `ws::types::messages::pyth`
  (`WsPythValue`/`Ref`, `WsPythUnderlyingList`/`Ref`), `WsUpdateAction::SubscribeUnderlyings`/
  `UnsubscribeUnderlyings`/`UnderlyingList`, `underlying_tickers` on `WsSubscriptionParamsV2`/
  `WsUpdateSubscriptionParamsV2`.
- [Rust API] New `cfbenchmarks_value_5hz` WebSocket channel: `WsChannelV2::CfbenchmarksValue5hz`,
  `WsCfBenchmarks5HzValue`/`Ref`, reusing `WsCfBenchmarksIndexList` for its indexlist response.
- [Rust API] `FeeType::QuadraticWithComboMakerFees` variant.
- [Tests] Deterministic parsing/behavior coverage added across all of the above.

### Changed

- [Rust API] `get_balance()` now takes `GetBalanceParams { subaccount, exchange_index }`. Passing
  `subaccount: Some(0)` is now distinct from omitting it (both previously meant "primary").
- [Rust API] `update_order_group_limit()` now takes an additional `UpdateOrderGroupLimitParams`
  argument.

### Deprecated

- [Rust API] `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders` (the legacy non-V2 `/portfolio/orders` mutation endpoints) are marked
  `#[deprecated(since = "0.8.0")]` in favor of `create_order_v2` / `cancel_order_v2` / etc. **The
  legacy REST paths themselves are already gone from the live API** (confirmed directly against
  the OpenAPI spec: `/portfolio/orders` and `/portfolio/orders/{order_id}` now expose only `GET`).
  These methods are kept as deprecated stubs for one release, rather than removed outright, to
  avoid an unreviewed hard compile break; see `docs/spec-parity.md` for the documented exception
  and plan to remove them in a following minor release.
- [Rust API] `get_quote`, `delete_quote`, `accept_quote`, `confirm_quote` (quote-ID-only actions)
  in favor of the RFQ-scoped equivalents; the unscoped REST paths remain live and deprecated
  upstream (unlike the legacy order endpoints above), so no functional exception applies here.

### Removed

- [Rust API] `EventData.available_on_brokers` (removed from the live schema).
- [Rust API] `Market.response_price_units`, `Market.fractional_trading_enabled` (removed from the
  live schema; `price_level_structure` / `price_ranges` / fixed-point fields remain canonical).
- [Rust API] `MarketPosition.resting_orders_count` (removed from the live schema).
- [Rust API] `GetQuotesParams.market_ticker`, `GetQuotesParams.event_ticker` (query params removed
  upstream 2026-06-20).
- [Rust API] `ErrorResponse.service` (removed from the live error schema 2026-08-06).
- [Rust API] The multivariate lookup REST surface: the `PUT .../lookup` method and its lookup-history
  types/method in `multivariate.rs` (both the pre-RFQ lookup endpoint and its history endpoint share
  the now-gone path).
- [Rust API] The multivariate WebSocket lookup surface: `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`/`MultivariateLookup`, and the `WsMultivariate`/`WsMultivariateRef`/
  `WsMultivariateSelectedMarket(Ref)` type family (`ws/types/messages/multivariate.rs` deleted).
  `WsChannelV2::MultivariateMarketLifecycle` is a distinct, still-current channel and is unaffected.
- [Rust API] `get_exchange_announcements()` and `Announcement`/`AnnouncementType`/`AnnouncementStatus`/
  `GetExchangeAnnouncementsResponse` (`GET /exchange/announcements` removed from the live API;
  `GET /exchange/schedule` remains).

### Fixed

- [Rust API] `WsError`/`WsErrorRef.message` deserialized from a field literally named `message`,
  but the live AsyncAPI schema nests the error text under `msg.msg` — the crate was silently
  dropping every WebSocket error's human-readable text. Found while verifying the 2026-09-10
  "WebSocket schemas corrected" changelog entry against the live spec directly; fixed with
  `#[serde(rename = "msg")]` and covered by a new assertion in
  `tests/ws_control_frame_sequence.rs`.
- [Rust API] Fixed a pre-existing compile break in `src/ws/types/envelope.rs`'s own unit tests
  (`WsMessageV2`/`WsMessageRef::ListSubscriptions` match arms not updated for the `sid`/`seq`
  fields added in 0.7.0), unrelated to this refresh but blocking `cargo test --lib` on `main`.

### Breaking

- [Rust API] `get_balance()` signature change (see Changed).
- [Rust API] `update_order_group_limit()` signature change (see Changed).
- [Rust API] `SubaccountBalance` gained a new required `exchange_index: i64` field.
- [Rust API] All Removed items above.
- [Rust API] `WsMultivariate`/`WsMultivariateRef`/`WsMultivariateSelectedMarket(Ref)` and the
  `Multivariate`/`MultivariateLookup` enum variants no longer exist; downstream exhaustive matches
  over `WsChannelV2`/`WsMsgType` must drop them.

### Known Issues

- The legacy (non-V2) order-mutation REST methods are deprecated-but-present per the Deprecated
  section above; calling them against the live API will fail, since the underlying REST paths no
  longer accept those methods. Plan to remove them outright in the next minor release once
  downstream consumers have had a release to migrate to `_v2`.

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
