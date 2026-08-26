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

- **Exchange sharding / `exchange_index`.** Kalshi is rolling out multiple exchange shards
  (2026-06 through 2026-08). Every `exchange_index` field added to REST responses, REST request
  filters, and WebSocket messages in this refresh is modeled as `Option<u32>` even where the
  OpenAPI/AsyncAPI schema marks it required — the rollout is still in progress and "exchange index
  0" is currently the only production value, so treating it as optional avoids breaking on older
  payloads or environments that predate the field. Covered surfaces: `Fill`, `MarketPosition`
  (REST and the WS `market_positions`-channel zero-copy mirror in `ws::types::mod`), `Series`,
  `EventData`, `MultivariateEventCollection`, `SubaccountBalance`, `GetExchangeStatusResponse`
  (plus the new `ExchangeIndexStatus` per-shard breakdown), `WsFill`, `WsUserOrder`,
  `WsMarketLifecycleV2`
  (covers both `market_lifecycle_v2` and, since it reuses the same message type, the
  `multivariate_market_lifecycle` channel), and `WsEventLifecycle`. Filter parameters
  (`GetOrdersParams`, `GetPositionsParams`, `GetFillsParams`, `GetBalanceParams`) gained an
  `exchange_index` field for the same reason. **Not yet covered** (deferred — see below):
  `GET /portfolio/orders`/`/positions`/`/fills` response objects beyond `Fill`/`MarketPosition`
  (e.g. `Order`), and the many order-group / transfer endpoints whose `ExchangeIndexQuery`
  parameter is not named in any changelog entry through this refresh's watermark.

- **`update_order_group_limit` gained a `subaccount` parameter** (2026-08-06), matching the other
  order-group endpoints (`get_order_group`, `delete_order_group`, `reset_order_group`,
  `trigger_order_group`), which already took `SubaccountQueryParams`. Previously it was the only
  order-group method that didn't.

- **`market_lifecycle_v2` top-level fields.** `strike_type`, `cap_strike`, and `custom_strike` were
  already modeled inside `additional_metadata` (emitted on market creation) but are also emitted at
  the top level of `metadata_updated` events (added 2026-06-18/2026-06-25) — `WsMarketLifecycleV2`
  now carries both. `price_ranges` was added at the top level for `created` and
  `price_level_structure_updated` events (2026-07-02) and reuses `rest::markets::PriceRange`
  (fields are plain `String`s, so the same type serves both the owned and the zero-copy `Ref`
  struct without an extra lifetime-parameterized variant).

- **`fractional_trading_enabled` removed.** The field existed on `WsMarketLifecycleV2` (and the
  `fractional_trading_updated` `event_type` variant) in an older AsyncAPI revision, but neither
  appears in the current AsyncAPI schema — removed rather than kept as a compatibility shim, per
  this repo's policy of not preserving schema shapes the live spec no longer describes. The
  `WsMarketLifecycleEventType` enum still has a `#[serde(other)] Unknown` fallback, so an exchange
  that somehow still emits the old event type is tolerated rather than rejected.

- **Legacy `/portfolio/orders` mutation endpoints are deprecated, not removed.** Kalshi deprecated
  (but has not removed) `create_order`, `cancel_order`, `amend_order`, `decrease_order`,
  `batch_create_orders`, and `batch_cancel_orders` on 2026-06-18 in favor of the V2 event-order
  endpoints added in 0.6.0. They're marked `#[deprecated]` with a pointer to their V2 replacement
  rather than removed, since Kalshi still serves them. `get_order`/`get_orders` (read-only) were
  not deprecated upstream and are untouched.

- **RFQ-scoped quote actions supersede quote-ID-only actions.** `get_quote`, `delete_quote`,
  `accept_quote`, and `confirm_quote` (quote-ID-only) are marked `#[deprecated]` in favor of
  `get_quote_for_rfq`, `delete_quote_for_rfq`, `accept_quote_for_rfq`, and `confirm_quote_for_rfq`
  (RFQ-ID + quote-ID scoped), added 2026-06-25/2026-07-09. Kalshi still serves the old endpoints,
  so they were deprecated rather than removed. `GetQuotesParams.market_ticker` /
  `.event_ticker` were removed outright (not deprecated) because Kalshi stopped honoring them as
  of 2026-06-20; `min_ts`/`max_ts` were added (2026-06-18) as their replacement filtering
  mechanism.

- **Fields/endpoints removed to match the live schema** (per this repo's policy of not carrying
  stale shapes as compatibility shims): `Market.response_price_units`,
  `Market.fractional_trading_enabled`, and `MarketPosition.resting_orders_count` (removed from the
  OpenAPI schema 2026-07-09); `GET /exchange/announcements` and its `Announcement` /
  `AnnouncementType` / `AnnouncementStatus` / `GetExchangeAnnouncementsResponse` types (endpoint
  removed 2026-07-04); `PUT .../multivariate_event_collections/{ticker}/lookup` and
  `GET .../lookup` (history) and their request/response types, and the WebSocket `multivariate`
  channel (`multivariate_lookup` message type) (all removed 2026-08-06 — an unrecognized
  `multivariate`/`multivariate_lookup` message type now falls through to `WsMsgType::Unknown`,
  matching the documented "unknown-channel error" behavior).

- **`EventData.available_on_brokers`** is marked `#[deprecated]` (kept as `Option<bool>`, not
  removed) — Kalshi deprecated it 2026-08-27 ("no longer populated, always returns `false`") but
  has not removed it from the schema yet.

- **Deferred in this refresh** (new upstream surface identified but not yet implemented; tracked
  for a follow-up refresh rather than silently dropped): the `pyth_value` /
  `pyth_value_underlying_list` WebSocket channel (2026-07-23); `POST`/`GET
  /portfolio/target_balance_allocation` (2026-08-20); `GET
  /portfolio/intra_exchange_instance_transfers[/​{id}]` history endpoints (2026-08-13); `GET
  /historical/positions` (2026-07-23); `GET /live_data/weather/{city}` and `GET
  /live_data/events/{event_ticker}` (2026-08-20/2026-07-30); the
  `resting_order_value_breakdown` field on `GetPortfolioRestingOrderTotalValueResponse`
  (2026-08-20); and `source_subaccount`/`destination_subaccount` on the intra-exchange-instance
  transfer request (2026-08-20). None of these have Rust API surface in the crate today, so their
  absence is additive-only (no breaking change) — a future refresh should add them.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
