# Spec Notes

This repository follows Kalshi's published OpenAPI and AsyncAPI documents
directly.

Those upstream specs are the baseline for contract review, but they do not
fully define every public behavior in the crate. The most important behavior
checks live in tests, especially where the YAML specs are underspecified or
examples are ambiguous.

## Known Distinctions

- `MarketStatusQuery` is the query/filter enum used by list endpoints.
- `MarketStatus` is the lifecycle/status enum returned on market objects.
- They overlap, but they are not one-to-one. Lifecycle states such as
  `determined`, `disputed`, and `amended` collapse differently when converted
  into query status. The conversion behavior is covered in `tests/parsing.rs`.

- The AsyncAPI examples imply both singular and plural market ticker fields for
  websocket subscriptions.
- The crate accepts `market_ticker` or `market_tickers`, but not both.
- `orderbook_delta` requires market tickers and rejects `market_id` and
  `market_ids`.
- `skip_ticker_ack` is supported on subscription updates.
- These behaviors are covered by `tests/ws_command_behavior.rs` and
  `tests/ws_parsing.rs`.

- The AsyncAPI spec marks `ts_ms` as required on both the `trade` and
  `ticker` channel messages (`WsTrade`, `WsTicker`).
- In practice the field is occasionally omitted by the exchange. Consumers
  should treat `ts_ms` as best-effort and fall back to `ts` (seconds) when
  precise millisecond timing matters.

- The `side` and `action` fields on `Order`, `Fill`, and `WsFill` were deprecated by Kalshi on
  2026-05-07. The new normalized fields are `outcome_side` (`yes` | `no`) and `book_side`
  (`bid` | `ask`), where `bid` ≡ `yes` and `ask` ≡ `no`. The OpenAPI/AsyncAPI specs still mark the
  legacy fields required ("not removed before May 14, 2026"), but the changelog scheduled removal
  for 2026-05-28. To survive either state, the legacy fields are modeled as `Option`, and the new
  normalized fields are also `Option` so older payloads (lacking them) still parse.
- The public `Trade` object (REST `Trade`, WebSocket `WsTrade`) uses the taker-prefixed variants:
  `taker_side` (deprecated) plus `taker_outcome_side` / `taker_book_side`. These follow the same
  `Option` treatment for the same reasons.

- The `/margin/fee_tiers` response was restructured on 2026-05-11. The previous tier-name maps
  (`maker_fee_tiers`, `taker_fee_tiers`) were replaced by per-ticker decimal-rate maps
  (`maker_fee_rates`, `taker_fee_rates`). Fee is computed as `notional * rate`.

- `event_fee_update` is an AsyncAPI message delivered on the `market_lifecycle_v2` channel (it is
  not a separately-subscribable channel). It is modeled by `WsEventFeeUpdate`. `fee_type_override`
  is kept as `Option<String>` rather than reusing the `FeeType` enum so the raw string survives any
  future fee-type additions without a crate update. Both override fields are nullable (`None` when
  the override is cleared).

- `FeeType` enum now includes `QuadraticWithMakerFees` (serialized `quadratic_with_maker_fees`),
  added to the OpenAPI spec in 2026. An `#[serde(other)] Unknown` catch-all is also present so
  unrecognised future variants never panic during deserialization. `fee_type_override` on
  `WsEventFeeUpdate` remains `Option<String>` for lossless round-trip regardless.

- `is_block_trade: bool` was added to the public REST `Trade` struct (2026-05-29). The field is
  `#[serde(default)]` (defaults to `false`) so payloads predating the flag still parse. The query
  filter `GetTradesParams::is_block_trade: Option<bool>` lets callers filter by block-trade status.

- `GET /account/limits` (`get_account_api_limits`) response was restructured in 2026-06 (automated
  API rate-limit tiers). The old flat shape (`read_limit: i64, write_limit: i64`) was replaced by
  nested `BucketLimit` objects (`read: BucketLimit, write: BucketLimit`) plus a `grants:
  Vec<ApiUsageLevelGrant>` array. The `GetAccountApiLimitsResponse` struct was updated accordingly;
  old field access will not compile (intentional minor-version break, 0.5.0 → 0.6.0).
  `ApiUsageLevelGrant.expires_ts` is `Option<i64>` because the field is absent for non-expiring
  grants.

- `cfbenchmarks_value` is a new AsyncAPI channel (introduced 2026-06) that delivers CF Benchmarks
  index values. It uses `index_ids` (not market tickers) for subscription parameters; pass
  `["all"]` to receive all available indices. The channel emits two message types:
  `cfbenchmarks_value` (per-index value + 60-second windowed average) and
  `cfbenchmarks_value_indexlist` (the full set of available index IDs). Both are modeled as
  `WsCfBenchmarksValue` / `WsCfBenchmarksIndexList` and routed through the standard
  `WsDataMessageV2` enum. `last_60s_windowed_average_15min` on `WsCfBenchmarksValue` is `Option`
  because the spec marks it conditional. The documented post-subscribe workflow (discover indices
  via `indexlist`, then add/remove with `subscribe_indices` / `unsubscribe_indices`) is supported
  through `update_subscription_v2` using the `WsUpdateAction::SubscribeIndices` /
  `UnsubscribeIndices` / `Indexlist` actions plus the `index_ids` field on
  `WsUpdateSubscriptionParamsV2`. `validate_update` rejects mixing index actions with market targets
  and requires `index_ids` for the add/remove actions, matching the AsyncAPI error semantics.

- `GET /account/endpoint_costs` (`get_account_endpoint_costs`) is modeled as a public (unauthed)
  endpoint because the OpenAPI operation declares no `security` requirement, unlike `/account/limits`.
  `ApiUsageLevelGrant.exchange_instance` is kept as `String` rather than an `ExchangeInstance` enum
  (`event_contract` | `margined`); the raw string round-trips losslessly and tolerates any future
  exchange-instance values without a crate update.
- The AsyncAPI marks several timestamp/required fields that the exchange may omit in practice
  (`ts_ms` on ticker/trade/order-group messages, the legacy direction fields). These are modeled as
  `Option` so parsing never fails on their absence.

## Exchange Sharding (2026-06 → 2026-09 refresh)

Kalshi is progressively splitting trading across multiple "exchange indexes" (shards). The rollout
adds an `exchange_index` field to many REST and WebSocket response types, sometimes marked
`required` in the spec even though the field is being introduced gradually. To avoid parse failures
during the rollout, `exchange_index` is modeled as `Option<i64>` everywhere it was added in this
refresh: `Order`, `Fill`, `Settlement`, `MarketPosition`, `EventData`, `Series`,
`MultivariateEventCollection`, `SubaccountBalance`, `WsFill`, `WsUserOrder`, `WsMarketLifecycleV2`,
`WsEventLifecycle` (the last one is spec-required but still modeled `Option` for the same reason).
`GetExchangeStatusResponse` gained `intra_exchange_transfers_active` and a new
`exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` breakdown. `GetOrdersParams`,
`GetPositionsParams`, and `GetFillsParams` gained an `exchange_index` filter; `GetBalanceResponse`
gained `balance_breakdown: Option<Vec<IndexedBalance>>` and `get_balance` now takes a
`GetBalanceParams { subaccount, exchange_index }` argument (previously no params).

This refresh does not implement the newer sharding-adjacent endpoints (target balance allocation,
cross-shard/intra-exchange transfers, cancel-all-orders), see "Known Gaps" below.

## Market Lifecycle WebSocket Additions (2026-06 → 2026-07)

`WsMarketLifecycleV2` (`market_lifecycle_v2` channel) gained fields that only appear on specific
`event_type` values, per the AsyncAPI's `oneOf`-style payload split that this crate flattens into one
struct:
- `strike_type`, `cap_strike`, `custom_strike` (alongside the existing `floor_strike` /
  `yes_sub_title`) appear only on `metadata_updated` events.
- `price_ranges: Option<Vec<PriceRange>>` (reusing the REST `PriceRange` type) appears alongside
  `price_level_structure` only on `created` and `price_level_structure_updated` events.

The `fractional_trading_enabled` field and its corresponding
`WsMarketLifecycleEventType::FractionalTradingUpdated` event type were removed from the AsyncAPI
spec (matching the REST `Market.fractional_trading_enabled` removal, see "Removed Fields" below) and
have been removed from the crate.

## Removed Fields And Endpoints (2026-07 → 2026-09 refresh)

Per `VERSIONING.md`, fields and endpoints confirmed absent from the live OpenAPI/AsyncAPI were
removed from the public Rust API rather than kept as compatibility shims:

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` — removed from the OpenAPI schema 2026-07-09.
- `EventData.available_on_brokers` — deprecated 2026-08-27 (stopped being populated, always
  returned `false`), removed from the schema 2026-09-10.
- `ErrorResponse.service` — deprecated 2026-07-28, removed from the schema 2026-08-06. Branch on
  `code` instead.
- `GET /exchange/announcements` (`get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus`) — removed from the OpenAPI spec
  2026-07-04.
- The multivariate ticker-pair lookup surface — removed from the OpenAPI spec 2026-08-06:
  `PUT /multivariate_event_collections/{collection_ticker}/lookup`
  (`lookup_tickers_for_market_in_multivariate_event_collection`) and the lookup-history endpoint
  (`get_multivariate_event_collection_lookup_history`), plus their request/response types
  (`LookupTickersForMarketInMultivariateEventCollection{Request,Response}`,
  `GetMultivariateEventCollectionLookupHistory{Params,Response}`, `LookupPoint`). The WebSocket
  `multivariate` channel and `multivariate_lookup` message type were removed the same day; use
  `multivariate_market_lifecycle` for multivariate market state changes instead.
- `GetQuotesParams.market_ticker` / `.event_ticker` — removed from `GET /communications/quotes`
  2026-06-20. Filter by `rfq_id`, `status`, or the new `min_ts` / `max_ts` / `user_filter` /
  `rfq_user_filter` instead.

## RFQ-Scoped Quote Actions (2026-06-25)

RFQ quotes are no longer guaranteed queryable/actionable without an `rfq_id` scope (a quote cleared
by a server roll may 404 on the ID-only lookup). New RFQ-scoped methods were added —
`get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` — and the ID-only
methods (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) are marked
`#[deprecated]` (still functional; the upstream ID-only endpoints remain supported for now).

## Known Gaps (Deferred This Refresh)

The following upstream additions were confirmed present in the live OpenAPI/AsyncAPI during this
refresh but are not yet implemented, given the scope of the 2026-06-08 → 2026-09-17 changelog
window. They do not represent contract drift in what the crate already models — no existing type or
endpoint is wrong — just newer surface not yet wired up:

- New endpoints: `GET /historical/positions`, `GET /portfolio/orders/cancel_all` (and the margin
  equivalent), `POST`/`GET /portfolio/target_balance_allocation`,
  `GET /account/api_usage_level/volume_progress`, `POST /account/api_usage_level/upgrade`,
  `GET /live_data/weather/{city}` (+ `/calibrations`), `GET /live_data/events/{event_ticker}`,
  `GET /portfolio/intra_exchange_instance_transfers` (+ `/{transfer_id}`).
- New WebSocket channels: `pyth_value`, `cfbenchmarks_value_5hz`.
- Cross-shard transfer fields (`source_subaccount` / `destination_subaccount` on
  `POST /portfolio/intra_exchange_instance_transfer`) and the `resting_margin_reservation` field on
  target balance allocation.

Margin-exchange and FIX-only changelog entries are out of scope entirely: this crate models the
Predictions REST/WebSocket surface only (see `CLAUDE.md`).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
