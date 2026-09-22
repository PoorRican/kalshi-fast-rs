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

- **Exchange sharding (2026-07 through 2026-09).** Kalshi began provisioning some
  categories (crypto, tennis, baseball, commodities, basketball) on dedicated exchange
  instances/shards, identified by `exchange_index` (a plain integer, `-1` meaning "auto-route").
  `exchange_index` was added across many REST and WebSocket response shapes: `Market`, `Order`,
  `Fill`, `Settlement`, `MarketPosition`, `Series`, `EventData`, `MultivariateEventCollection`,
  `SubaccountBalance`, `GetExchangeStatusResponse` (via `ExchangeIndexStatus`), and the WS
  `market_lifecycle_v2`, `fill`, and `user_order` messages. Where the live schema marks
  `exchange_index` required (`Fill`, `Settlement`, `MarketPosition`), the crate still models it as
  `Option<i64>` defensively — consistent with this crate's general policy of tolerating rollout gaps
  in newly-required fields — and documents the nuance here rather than trusting `required:` blindly.
  `GET /portfolio/balance`, `GET /portfolio/orders`, `GET /portfolio/positions`, and
  `GET /portfolio/fills` gained an optional `exchange_index` filter. `GetBalanceResponse` gained
  `balance_breakdown: Option<Vec<IndexedBalance>>` (per-exchange-index balances; omitted only for
  subaccount-restricted keys), and `get_balance` now takes a `GetBalanceParams` (`subaccount`,
  `exchange_index`) — a breaking method-signature change.

- **Market legacy compatibility fields removed (this refresh).** The live OpenAPI `Market` schema
  has never included a large set of fields this crate carried since inception: `market_id`,
  `series_ticker`, `series_id`, `event_id`, `response_price_units`, `floor_price`, `cap_price`,
  `yes_bid`/`yes_ask`/`no_bid`/`no_ask`/`price`/`last_price` (integer cent fields),
  `volume`/`volume_24h`/`open_interest`/`notional_value`/`liquidity` (integer count fields),
  `fractional_trading_enabled`, `previous_yes_bid`/`previous_yes_ask`/`previous_price` (integer),
  `tick_size`, `settlement_value` (integer), `created_ts`/`updated_ts`/`open_ts`/`close_ts`/
  `settled_ts`/`expiration_ts` (integer timestamps), `resolution_source`, `event_title`,
  `can_trade`, `can_settle`. Per the refresh workflow's policy against carrying dead compatibility
  fields, these were removed from `Market` in this release; use the `*_dollars`, `*_fp`, and
  `*_time` (RFC3339 string) fields instead. The same cleanup removed
  `MarketPosition.resting_orders_count` (confirmed absent from the live `MarketPosition` schema,
  matching the 2026-07-09 changelog entry) and the equivalent dead `resting_orders_count` field on
  the (previously unused, now-removed) WS `MarketPositionRef`/`EventPositionRef` compatibility
  shim in `ws::types`. The real WS `market_position` message shape has always been
  `WsMarketPosition` (`user_id`, `market_ticker`, `position_cost_dollars`,
  `position_fee_cost_dollars`, `volume_fp`, ...), not the REST `MarketPosition` shape — the
  now-removed `MarketPositionRef`/`EventPositionRef` types were dead code that aliased the wrong
  (REST) shape and were never wired into any parser.

- `GET /exchange/announcements` was removed from the Predictions REST API 2026-07-04.
  `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus` were removed from the public Rust API accordingly.
  Exchange schedule remains available through `get_exchange_schedule()`.

- The `multivariate` WebSocket channel (message type `multivariate_lookup`) and the
  `PUT /multivariate_event_collections/{collection_ticker}/lookup` REST endpoint were removed
  2026-08-06 after being deprecated 2026-07-02. `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate`/`WsMultivariateRef`, and the
  REST lookup-history types/method were removed from the public Rust API. Use
  `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes and
  `POST /multivariate_event_collections/{collection_ticker}` (`create_market_in_multivariate_event_collection`)
  to create/resolve combo markets.

- `service` on REST error response bodies was deprecated 2026-07-28 and removed 2026-08-06 (the
  exchange no longer populates it on any response). `ErrorResponse.service` is kept as
  `Option<String>` — it will always deserialize to `None` against current payloads, but the field
  costs nothing to keep and removing it would be a needless breaking change for a field that no
  longer round-trips any data. Branch on `code` instead, per Kalshi's guidance.

- `GET /communications/quotes` no longer supports `market_ticker`/`event_ticker` filters (removed
  2026-06-20); `GetQuotesParams` no longer has these fields. It gained `min_ts`/`max_ts`
  (2026-06-18) and `user_filter` (parallels the existing `rfq_user_filter`). RFQ-scoped quote
  action endpoints (`.../rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`) were added 2026-06-25
  and are preferred going forward — the quote-ID-only endpoints (`get_quote`, `delete_quote`,
  `accept_quote`, `confirm_quote`) are deprecated but still fully supported, per Kalshi's own
  migration guidance (no removal date announced). The only existing `#[deprecated]` usage in this
  crate is on a single field with an announced fallback (`WsOrderbookDelta.ts`); applying the same
  hard compiler-warning attribute to four still-current, indefinitely-supported public methods
  would be noisier than warranted, so these carry a `Deprecated: ...` doc-comment note instead,
  pointing at the RFQ-scoped replacement.

- `pyth_value` (2026-07-23) and `cfbenchmarks_value_5hz` (2026-09-03) are new AsyncAPI channels
  following the same pattern as `cfbenchmarks_value`: `pyth_value` uses `underlying_tickers`
  (not market tickers or index IDs) for subscription parameters and supports
  `subscribe_underlyings`/`unsubscribe_underlyings`/`underlying_list` on `update_subscription`;
  `cfbenchmarks_value_5hz` reuses `index_ids` and the `subscribe_indices`/`unsubscribe_indices`/
  `indexlist` actions already used by `cfbenchmarks_value`, but delivers raw ticks (no rolling
  averages) at up to 5 updates/second.

- `market_lifecycle_v2` gained additional top-level keys that, like the pre-existing
  `floor_strike`/`yes_sub_title` pair, are documented only in changelog prose and not in the
  formal AsyncAPI `msg` property list (the spec's `metadata_updated` handling remains
  under-specified as of the 2026-09-10/09-17 "WebSocket schema corrections" passes): `strike_type`,
  `cap_strike`, and `custom_strike` (2026-06-18, alongside the pre-existing `floor_strike`), and
  `price_ranges` (2026-07-02, alongside `price_level_structure` on `created` and
  `price_level_structure_updated` events). `exchange_index` (present only on `created`) is
  documented in the schema. All are modeled as `Option` on `WsMarketLifecycleV2`.

- Seven new `price_level_structure` values (`center_whole_edge_half_cent`, etc., 2026-07-23) and
  `center_deci_edge_centi_cent` (2026-08-13, briefly serialized empty 2026-09-10 due to a
  service-side bug, fixed same day) required no code change: `Market.price_level_structure` and
  the WS equivalent are already untyped `String`/`Option<String>`. Consumers should read valid
  order prices from `price_ranges` rather than keying logic off the structure name, per Kalshi's
  own guidance.

- `FeeType` gained `QuadraticWithComboMakerFees` (serialized `quadratic_with_combo_maker_fees`):
  the combo/RFQ maker-fee structure with a 0.5 (rather than 0.25) maker multiplier. The existing
  `#[serde(other)] Unknown` catch-all already tolerated this value before the variant was added;
  it is now surfaced explicitly for callers that want to branch on it.

- New REST endpoints added this refresh, all following existing crate conventions:
  `GET /historical/positions` (`get_historical_positions`, 2026-07-23, reuses `GetPositionsResponse`);
  `DELETE /portfolio/events/orders` (`cancel_all_orders_v2`, 2026-08-27); `GET`/`POST
  /portfolio/target_balance_allocation` (`get_target_balance_allocation` /
  `set_target_balance_allocation`, 2026-08-20); `POST /portfolio/intra_exchange_instance_transfer`
  plus its history endpoints (`intra_exchange_instance_transfer`,
  `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer`,
  2026-08-13/08-20); `GET /live_data/events/{event_ticker}` (`get_event_live_data`, 2026-07-30,
  modeled with a distinct `EventLiveData` type — not the milestone-keyed `LiveData` shape, since
  the OpenAPI schema differs: no `milestone_id`, plus `is_historical`/`default_range`/
  `range_options`).
  Deliberately **not** implemented this refresh (out of scope for a trading-focused adapter, or
  low value relative to effort): `GET/POST /account/api_usage_level/*` (tier bookkeeping),
  `GET /live_data/weather/{city}` and `/calibrations` (a new, large weather-index domain),
  `GET /account/api_usage_level/volume_progress`. These are genuine gaps versus full OpenAPI
  coverage; flagged here rather than silently omitted.

- `RestingMarginReservation` (`"none"` | `"max"` | `"sum"`, used by target balance allocation) and
  `IntraExchangeInstanceTransferRequest.source`/`.destination` (`ExchangeInstance`: `"event_contract"`
  | `"margined"`) are kept as raw `String`/`type alias = String` rather than enums, matching the
  existing `ApiUsageLevelGrant.exchange_instance` convention: future values round-trip losslessly
  without a crate update.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
