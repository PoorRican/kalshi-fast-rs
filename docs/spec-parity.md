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

- **Legacy (non-V2) order mutation endpoints are no longer documented in OpenAPI 3.28.0.** As of this
  refresh, `POST /portfolio/orders`, `DELETE /portfolio/orders/{order_id}`, `POST
  /portfolio/orders/{order_id}/amend`, `POST /portfolio/orders/{order_id}/decrease`, and `POST`/`DELETE
  /portfolio/orders/batched` no longer appear as paths in the published spec (only the `GET` order
  endpoints remain). No changelog entry corroborates an intentional removal date — this may be a docs
  generator change rather than an actual endpoint sunset. Given the risk of silently breaking working
  trading code, `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  and `batch_cancel_orders` are kept functional but marked `#[deprecated]`, pointing callers at the `_v2`
  equivalents. Re-verify on the next refresh and remove outright once removal is corroborated by the
  changelog (or confirmed still-working/still-gone against the live API).

- **`exchange_index` (multi-shard rollout).** Kalshi is rolling out exchange sharding across the API
  surface; the field is a plain, unconstrained `integer` (`ExchangeIndex` schema) that defaults to `0`.
  On the V2 order endpoints (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`,
  `decrease_order_v2`, batch V2 endpoints) it additionally accepts the sentinel `-1` to mean
  "auto-route by market ticker" — those fields are modeled as `Option<i64>` (not `u32`) specifically to
  represent `-1`, and a companion `market_ticker: Option<String>` field (required when `exchange_index`
  is `-1`) was added to `CancelOrderV2Params`, `DecreaseOrderV2Request`, and
  `BatchCancelOrderV2RequestOrder`. Elsewhere (order groups, balance, transfers, `Market`,
  `MultivariateEventCollection`, exchange status) `exchange_index` is always non-negative per the spec,
  but is still modeled as `i64`/`Option<i64>` for consistency rather than mixing signed/unsigned types
  across the crate. The changelog described one slice of this rollout as "margin order groups bind to
  single exchange_index," but the OpenAPI schema shows `exchange_index` on the generic `OrderGroup` /
  `CreateOrderGroupRequest` / `CreateOrderGroupResponse` / `GetOrderGroupResponse` schemas — it is not
  margin-specific, so it was added to all order-group types.

- `GetOrderGroupResponse.orders` is `Vec<String>` (order IDs), not `Vec<Order>` — this was a pre-existing
  crate bug (the field was mistyped as full `Order` objects) that would have failed to deserialize
  against the real API. Fixed as part of this refresh. `OrderGroup.contracts_limit` and
  `GetOrderGroupResponse.contracts_limit` (plain integer fields) were removed — the OpenAPI schemas for
  these types only define `contracts_limit_fp`; the plain-integer variants were never real.

- `delete_order_group`, `reset_order_group`, `trigger_order_group`, and `update_order_group_limit` now
  take an `OrderGroupActionParams { subaccount, exchange_index }` query-params argument (both fields
  documented as query parameters, not body fields, on those endpoints) instead of the shared
  `SubaccountQueryParams`. `get_order_group` / `get_order_groups` are unaffected — the OpenAPI confirms
  those two endpoints only accept `subaccount`, not `exchange_index`.

- `SubaccountTransfer.exchange_index` and `SubaccountBalance.exchange_index` are spec-required fields
  that were missing entirely from the crate (a pre-existing gap, not tied to a specific changelog
  entry). Modeled as plain `i64` with `#[serde(default)]` so existing test fixtures / older payloads
  without the field still parse (defaulting to shard `0`).

- `WsEventLifecycle.exchange_index` is marked `required` by the AsyncAPI spec but is modeled as
  `Option<i64>` here: the struct previously had no catch-all `extra` field, so the exchange sending an
  `event_lifecycle` message without it would otherwise fail to parse. Treated with the same leniency as
  other "required per spec, sometimes absent in practice" fields documented above.

- `WsMarketLifecycleV2` / `WsMarketLifecycleV2Ref` no longer carry `fractional_trading_enabled`, and
  `WsMarketLifecycleEventType` no longer has a `FractionalTradingUpdated` variant. Both were added in a
  prior release based on an upstream field that is no longer present anywhere in the current AsyncAPI
  spec (verified via a full-file grep for "fractional"). Removed as stale/dead code per the refresh
  workflow's "don't carry forward removed upstream fields" rule.

- `WsQuoteCreated`, `WsQuoteAccepted`, and `WsQuoteExecuted` gained an optional `subaccount: Option<i64>`
  field, present only when the authenticated user's side of the RFQ/quote used a subaccount.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
