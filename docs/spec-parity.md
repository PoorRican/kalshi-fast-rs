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

- `FeeType` gained `QuadraticWithComboMakerFees` (`quadratic_with_combo_maker_fees`): the same
  maker-fee structure as `QuadraticWithMakerFees` but with a 0.5 maker multiplier instead of 0.25,
  used for combo-market fee schedules. The `#[serde(other)] Unknown` catch-all still protects
  against further additions.

- **Exchange sharding (2026-07/08).** Kalshi is splitting the exchange into multiple `exchange_index`
  shards (Crypto, Tennis, and Baseball are the first to move). The OpenAPI/AsyncAPI mark
  `exchange_index` as *required* on many objects introduced or touched by this rollout — `Market`,
  `EventData`, `Series`, `MultivariateEventCollection`, `MarketPosition`, `Fill`, `Settlement`, and
  the WebSocket `market_lifecycle_v2` / `event_lifecycle` / `user_orders` / `fill` payloads. Because
  the field did not exist before this rollout and production traffic is migrating gradually, every
  one of these is modeled as `Option<i64>` rather than a bare `i64`, so payloads from before the
  rollout (and any crate-side test fixtures that predate it) keep parsing. `GetExchangeStatusResponse`
  gained `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (per-shard breakdown) and
  `intra_exchange_transfers_active: Option<bool>`; the top-level `exchange_active`/`trading_active`
  fields continue to reflect shard 0. `GetPositionsParams`, `GetFillsParams`, `GetOrdersParams`, and
  `GetBalanceParams` (new — see below) gained an optional `exchange_index` filter/scope parameter.
  `GetPortfolioRestingOrderTotalValueResponse` gained `resting_order_value_breakdown: Option<Vec<IndexedBalance>>`.

- `GET /portfolio/balance` (`get_balance`) now takes a `GetBalanceParams { subaccount, exchange_index }`
  argument (previously took no parameters at all) — a breaking signature change. `exchange_index`
  scopes both `balance` and `portfolio_value` to one shard; omitting it aggregates across all shards.

- **`EventData.available_on_brokers` is deprecated** (2026-08-27): the exchange no longer populates
  it and it always returns `false`. It is marked `#[deprecated]` in Rust and kept as `Option<bool>`
  rather than removed, since the OpenAPI spec still lists the field (deprecated, not removed).

- **Removed from the live schema, and removed from the crate to match:**
  - `Market.response_price_units` and `Market.fractional_trading_enabled` (REST), and
    `MarketPosition.resting_orders_count` — removed from the OpenAPI schema 2026-07-09.
  - `ErrorResponse.service` — deprecated 2026-07-28, removed 2026-08-06. Branch on `code` instead.
  - `GET /exchange/announcements` (`get_exchange_announcements`) and its response types
    (`GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, `AnnouncementStatus`) —
    the endpoint was removed from the Predictions REST API 2026-07-04.
  - The multivariate lookup surface: `PUT /multivariate_event_collections/{ticker}/lookup`
    (`lookup_tickers_for_market_in_multivariate_event_collection`) and the WebSocket `multivariate`
    channel / `multivariate_lookup` message type (`WsMultivariate`, `WsChannelV2::Multivariate`,
    `WsMsgType::Multivariate`/`MultivariateLookup`) — removed 2026-08-06. The
    `multivariate_market_lifecycle` channel (`WsMarketLifecycleV2` re-used) is unaffected and remains
    the way to observe multivariate market lifecycle changes.
  - The WebSocket `market_lifecycle_v2` `fractional_trading_updated` event type and the
    `fractional_trading_enabled` field on `WsMarketLifecycleV2` are no longer present in the
    AsyncAPI spec (found during this refresh's required-field grep; not called out in the changelog
    by name, so the exact removal date is unconfirmed — treat as pre-2026-06-08).

- **`WsMarketLifecycleV2` `metadata_updated` events** now also carry top-level `strike_type`,
  `cap_strike`, and `custom_strike` (2026-06-18), alongside the existing top-level `floor_strike` /
  `yes_sub_title` pattern — distinct from the nested `additional_metadata.*` versions of the same
  field names, which are emitted only on `created`. `created` and `price_level_structure_updated`
  events also gained a top-level `price_ranges: Option<Vec<WsPriceRange>>` (2026-07-02), mirroring
  the REST `Market.price_ranges` shape.

- **Fixed:** the WebSocket `market_positions` channel is modeled by `WsMarketPosition`/
  `WsMarketPositionRef` (`user_id`, `position_cost_dollars`, `position_fee_cost_dollars`,
  `volume_fp`, `subaccount`, …), which matches the AsyncAPI `marketPositionPayload` schema. A
  second, unrelated pair of types — `MarketPositionRef`/`EventPositionRef` in `ws::types` — mirrored
  the *REST* `MarketPosition`/`EventPositionpayload` shape instead and were not wired to any actual
  WebSocket message path (dead code, found via this refresh's required-field grep). They are kept
  (some external consumers may already depend on them for zero-copy REST-shaped parsing) but are now
  kept in sync with the REST `MarketPosition` struct's fields rather than drifting from it.

- **New price level structures carry no new fields.** Kalshi introduced `center_deci_edge_centi_cent`
  (2026-08-13) and seven `center_*_edge_*_cent` variants (2026-07-23) on top of the existing
  `linear_cent` / `deci_cent` / `tapered_deci_cent` values. `Market.price_level_structure` and the
  WebSocket equivalent are untyped `String`/`Option<String>` in this crate specifically so new
  structure names never require a crate update — always read valid prices from the `price_ranges`
  array rather than branching on the structure name.

## Known Gaps (not implemented this refresh)

These upstream additions were reviewed against the live specs but are not yet implemented. They are
additive (no existing crate surface depends on them) and are tracked here rather than silently
dropped:

- `GET /live_data/weather/{city}` — Kalshi Weather Index endpoint (2026-08-20).
- `GET /live_data/events/{event_ticker}` — event-keyed live data (2026-07-30).
- `GET`/`POST /portfolio/target_balance_allocation` — target balance allocation across exchange
  indexes (2026-08-20).
- `POST /portfolio/intra_exchange_instance_transfer` and its history endpoints
  (`GET /portfolio/intra_exchange_instance_transfers[/{transfer_id}]`) — the base "Intra Account
  Transfer" endpoint family predates this refresh's watermark and was never implemented in the
  crate, so the 2026-06/08 refinements (subaccount fields, history endpoints) have nothing to attach
  to.
- `GET /historical/cutoff` — historical/live data boundary timestamps (referenced by
  `GET /historical/positions`, which *is* now implemented; the cutoff endpoint itself is not).
  `GET /historical/orders` is likewise not implemented.
  `GET /fcm/positions` — FCM subtrader positions (unrelated to this refresh; pre-existing gap).
- `pyth_value` WebSocket channel — deduplicated Pyth price updates by underlying ticker
  (2026-07-23). Unlike `cfbenchmarks_value`, this channel was not added to `WsChannelV2` this
  refresh.
- The Margin exchange (`/margin/*` beyond `get_margin_fee_tiers`) remains out of scope: margin
  markets, positions, risk, order groups, and exit triggers are not modeled. All Margin-tagged
  changelog entries since the last watermark were dispositioned as "no change" for this reason —
  see `CHANGELOG.md`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
