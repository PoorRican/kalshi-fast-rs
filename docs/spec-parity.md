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

- The legacy `/portfolio/orders` mutation endpoints (`POST`/`DELETE /portfolio/orders`, `.../amend`,
  `.../decrease`, `POST`/`DELETE /portfolio/orders/batched`) were removed from the OpenAPI spec
  entirely (deprecated 2026-06-18, gone by this refresh). `create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` and their
  request/response types were removed from the crate. Use the V2 event-order methods
  (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead. `GET /portfolio/orders` and
  `GET /portfolio/orders/{order_id}` (read paths) are unaffected.

- The `multivariate` WebSocket channel and its `multivariate`/`multivariate_lookup` message type
  were removed from the AsyncAPI spec, along with the REST
  `PUT /multivariate_event_collections/{ticker}/lookup` and
  `GET /multivariate_event_collections/{ticker}/lookup` (history) endpoints. `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate*` types, and the corresponding REST
  lookup types/methods were removed. `multivariate_market_lifecycle` (a separate channel) is
  unaffected and still reuses `WsMarketLifecycleV2`.

- `GET /portfolio/balance` gained `exchange_index` and `subaccount` query parameters, so
  `get_balance` now takes a `GetBalanceParams` argument instead of none.

- `ErrorResponse.service` was removed from the OpenAPI error schema (only `code`/`message`/`details`
  remain) and dropped from the crate.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema and dropped from the
  crate (they predate the fixed-point/`price_ranges` migration). `WsMarketLifecycleEventType::FractionalTradingUpdated`
  was likewise dropped: the AsyncAPI `market_lifecycle_v2` `event_type` enum no longer lists it.

- `EventData.available_on_brokers` was deprecated 2026-08-27 (stopped populating, always `false`)
  and removed from the OpenAPI schema 2026-09-10; dropped from the crate.

- Exchange sharding: many REST response types (`Order`, `Fill`, `Settlement`, `MarketPosition`,
  `Event`, `Series`, `MultivariateEventCollection`) and WebSocket messages (`WsMarketLifecycleV2`,
  `WsEventLifecycle`, `WsFill`, `WsUserOrder`, `WsTrade`) gained an `exchange_index: Option<i64>`
  field identifying which exchange shard the record lives on. `GetOrdersParams`,
  `GetPositionsParams`, and `GetFillsParams` gained a matching `exchange_index` filter.

- `RestingMarginReservation` (target balance allocation's collateral-reservation policy, `"max"` |
  `"sum"`) is kept as a `String` type alias rather than an enum, matching the crate's convention for
  server-controlled string enums that may grow new variants.

- `GET /account/endpoint_costs` documents a `GET /trade-api/v2/cfbenchmarks/*` REST passthrough for
  historical CF Benchmarks values (changelog 2026-08-27), but no `/cfbenchmarks/*` path or schema
  exists in the OpenAPI document, so it is not modeled. Revisit once it appears in the published spec.

- `Accept-Language` (Spanish/Portuguese localized market content, added 2026-08-27) has no dedicated
  parameter; set it via `KalshiRestClientBuilder::with_default_headers`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
