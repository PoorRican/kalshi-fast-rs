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

- **`openapi.yaml` is a partial spec, not the full legacy surface.** Its `info.title` is
  "Kalshi Trade API Manual Endpoints" and its description states it covers "endpoints being
  migrated to spec-first approach." A field's *absence* from this document is therefore not on
  its own proof that the exchange stopped sending it — only the changelog (or a live response)
  confirms an actual removal. `docs/spec-parity.md` and `CHANGELOG.md` only record a field/endpoint
  as removed when the changelog explicitly says so; `asyncapi.yaml` carries no such caveat and is
  treated as the complete, authoritative WebSocket contract.
- Legacy `/portfolio/orders` order-mutation endpoints (`POST` create, `DELETE` cancel, `POST`
  `.../amend`, `.../decrease`, `.../batched`) were removed from `openapi.yaml` between the 2026-06-08
  and 2026-09-28 docs snapshots, consistent with the 2026-06-18 changelog entry announcing their
  deprecation ("calls to these endpoints will return `Please switch to the V2 endpoints`"). Only
  `GET /portfolio/orders` and `GET /portfolio/orders/{order_id}` remain. The crate now only exposes
  order mutation through the V2 event-order endpoints (`create_order_v2`, `cancel_order_v2`,
  `amend_order_v2`, `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`);
  `CreateOrderRequest`/`CancelOrderParams`/`AmendOrderRequest`/`DecreaseOrderRequest`/
  `BatchCreateOrdersRequest`/`BatchCancelOrdersRequest` and their responses were removed along with
  the corresponding legacy client methods. `GetOrderResponse`, `GetOrdersParams`, and `get_orders`/
  `get_order` (read-only) are unaffected.
- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed per the 2026-07-09 changelog entry
  ("Deprecated Predictions REST schema fields removed"). `Market.liquidity_dollars` was removed per
  the 2026-10-01 entry (deprecated 2026-02, "always returned `\"0.0000\"`" since then; use
  `yes_bid_size_fp` / `yes_ask_size_fp` for top-of-book size instead). `Event.available_on_brokers`
  was deprecated 2026-08-27 and removed 2026-09-10 ("always returns `false`"). All four fields are
  removed from the crate's structs. `Market.liquidity` / `liquidity_fp` (the non-`_dollars`
  siblings) are **not** removed — the changelog only named `liquidity_dollars`, and per the caveat
  above their absence from the partial `openapi.yaml` document is not itself evidence of removal.
- The WebSocket `multivariate` channel and its `multivariate_lookup` message were fully removed
  (2026-08-06 changelog: "Subscriptions to it now return an unknown-channel error"). `WsChannelV2`
  no longer has a `Multivariate` variant, `WsMsgType` no longer has `Multivariate` /
  `MultivariateLookup`, and `WsMultivariate` / `WsMultivariateRef` were deleted. The
  `multivariate_market_lifecycle` channel is unrelated and unaffected — it reuses
  `WsMarketLifecycleV2`, same as `market_lifecycle_v2`. The matching REST surface
  (`PUT`/`GET /multivariate_event_collections/{collection_ticker}/lookup`) is also gone from
  `openapi.yaml` (2026-07-02 changelog: "fully deprecated"); `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types
  were removed. `POST /multivariate_event_collections/{collection_ticker}` (create/resolve a combo
  market) and the communications (RFQ) endpoints are unaffected.
- `market_lifecycle_v2` (and, since they share a struct, `multivariate_market_lifecycle`)
  `metadata_updated` events now surface top-level `strike_type`, `cap_strike`, and `custom_strike`
  alongside the existing top-level `floor_strike` / `yes_sub_title` (2026-06-18). `created` and
  `price_level_structure_updated` events carry a top-level `price_ranges` array (2026-07-02), and
  `created` events carry `exchange_index` (2026-07-30, exchange-sharding rollout); `WsEventLifecycle`
  gained the same `exchange_index` field. `fractional_trading_enabled` and the
  `FractionalTradingUpdated` event-type variant were removed from `market_lifecycle_v2` — the
  AsyncAPI `event_type` enum for this payload no longer lists it.
- `WsTrade.is_block_trade` (2026-08-13) and `WsUserOrder.last_update_reason` (2026-10-01,
  `Option<String>` to tolerate future reason values losslessly) were added to match current
  AsyncAPI-required fields.
- `ErrorResponse.service` was removed (deprecated 2026-07-28, removed 2026-08-06: "no longer
  returned by any REST endpoint... Branch on `code` instead"). `build_http_error`'s emptiness check
  no longer references it.
- The exchange-sharding rollout (2026-07 through 2026-09) adds `exchange_index` to many REST
  objects. This pass added it to `Market`, `Order`, `MarketPosition`, `Fill`, and `Settlement` only
  (the objects most central to placing and tracking trades). It was **not** added to `Series`,
  `MultivariateEventCollection`, the `exchange_index` query filters on `GET /portfolio/{orders,
  positions,fills}`, or the various new sharding/transfer/rebalancing endpoints
  (`target_balance_allocation`, `intra_exchange_instance_transfer`, cancel-all-orders,
  `api_usage_level/*`, the Kalshi Weather Index and CF Benchmarks 5Hz/Pyth-value channels) — these
  remain tracked as follow-up work, not modeled by this crate yet.
- Two dead public types, `MarketPositionRef` / `EventPositionRef` (in `src/ws/types/mod.rs`), were
  removed. They duplicated the REST `MarketPosition`/`EventPosition` shape but were never wired into
  any WebSocket message dispatch — the real `market_positions` channel payload is `WsMarketPosition`
  (`src/ws/types/messages/positions.rs`), which has an unrelated, already-correct shape
  (`user_id`, `market_ticker`, `position_fp`, `position_cost_dollars`, ...).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
