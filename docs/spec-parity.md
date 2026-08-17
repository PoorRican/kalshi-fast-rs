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

- **Scope**: this crate models the Predictions exchange's REST and WebSocket surfaces only. The
  Margin exchange (`/margin/*` endpoints, `margin_ticker` WS channel) and FIX are out of scope,
  except `GET /margin/fee_tiers` (`get_margin_fee_tiers`), which predates this scoping decision and
  is kept for backward compatibility. Changelog entries tagged only `Margin` or `FIX` are treated as
  "no code change needed" for this reason unless they touch `/margin/fee_tiers`.

- `subaccount` validation bounds were corrected from `0..=32` to `0..=63` across
  `GetPositionsParams`, `GetOrdersParams`, and `CreateOrderRequest` (2026-08). The OpenAPI
  `SubaccountQuery` / `SubaccountQueryDefaultPrimary` parameters document 0 for primary and 1-63 for
  subaccounts (max 63 numbered subaccounts per user); the previous `> 32` checks were a pre-existing
  bug unrelated to any single changelog entry, caught while reconciling subaccount-related fields.

- `ErrorResponse.service` was deprecated 2026-07-28 and removed from all REST error bodies
  2026-08-06. It has been removed from the public `ErrorResponse` struct (breaking) rather than kept
  as `Option`, per the crate's policy of not preserving removed upstream fields — `code` is
  documented as present on every error response and is the intended branch target.

- The multivariate lookup surface was removed upstream on 2026-08-06: the `PUT
  .../multivariate_event_collections/{collection_ticker}/lookup` REST endpoint and the WebSocket
  `multivariate` channel (`multivariate_lookup` message type) no longer exist; subscriptions to the
  channel now return an unknown-channel error. The crate removed the corresponding
  `lookup_tickers_for_market_in_multivariate_event_collection` /
  `get_multivariate_event_collection_lookup_history` REST methods and their types, the
  `WsChannelV2::Multivariate` variant, and the `WsMultivariate` / `WsMultivariateRef` WS message
  types (breaking). Use `POST .../multivariate_event_collections/{collection_ticker}` to create or
  resolve a combo market, and the `multivariate_market_lifecycle` WS channel for market state
  changes.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema on 2026-07-09 and are
  removed from the crate (breaking) rather than kept as dead `Option` fields.
  `WsMarketLifecycleV2.fractional_trading_enabled` was independently absent from the current
  AsyncAPI `market_lifecycle_v2` payload and was removed for the same reason.

- `exchange_index` (an `Option<i64>`, default 0) was added across many REST and WebSocket response
  shapes in 2026-07/08 as Kalshi began provisioning sharded exchange instances: `Series`,
  `EventData`, `MultivariateEventCollection`, `SubaccountBalance`, `GetExchangeStatusResponse`
  (`exchange_index_statuses: Vec<ExchangeIndexStatus>`, with per-entry `description` added
  2026-08-13), `WsMarketLifecycleV2` (`created` events), and `WsEventLifecycle`. `GetBalanceParams`,
  `GetOrdersParams`, `GetPositionsParams`, and `GetFillsParams` all gained an `exchange_index` filter
  field. `GetBalance` additionally gained a `subaccount` parameter — `get_balance` now takes a
  `GetBalanceParams` argument instead of none (breaking).

- `GET /trade-api/v2/account/intra_exchange_instance_transfers` history endpoints (added
  2026-08-13) are **not yet implemented** — a known gap, tracked for a future refresh.

- The `pyth_value` WebSocket channel (added 2026-07-23, requires authentication) mirrors the
  `cfbenchmarks_value` pattern: `underlying_tickers` (not market tickers) for subscription
  parameters, `["all"]` to track every available underlying. It emits `pyth_value` (per-underlying
  USD value) and `pyth_value_underlying_list` (recently streamed underlyings) messages, modeled as
  `WsPythValue` / `WsPythUnderlyingList` and routed through the standard `WsDataMessageV2` enum. The
  `subscribe_underlyings` / `unsubscribe_underlyings` / `underlying_list` update-subscription
  workflow is supported through `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` /
  `UnderlyingList` plus `WsUpdateSubscriptionParamsV2.underlying_tickers`, validated the same way as
  the CF Benchmarks index actions.

- New `price_level_structure` enum values (seven `center_*_edge_*_cent` variants added 2026-07-23;
  `center_deci_edge_centi_cent` added 2026-08-13; `center_centi_edge_centi_cent` added 2026-08-17 for
  combo markets) require no crate change: `price_level_structure` is modeled as a raw `String`
  everywhere (`Market`, `WsMarketLifecycleV2`) specifically so new values pass through losslessly.
  Consumers should read valid order prices from the `price_ranges` array rather than branching on the
  structure name, per Kalshi's guidance.

- RFQ-scoped quote action endpoints (`GET`/`DELETE`/`PUT .../communications/rfqs/{rfq_id}/quotes/
  {quote_id}[/accept|/confirm]`, added 2026-06-25/07-09) are modeled as `get_rfq_quote` /
  `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`. The older quote-ID-only methods
  (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) remain for source compatibility but
  are marked `#[deprecated]`, matching the upstream OpenAPI `deprecated: true` markers. `GetQuotesParams`
  lost `market_ticker` / `event_ticker` (removed upstream 2026-06-20, breaking) and gained `min_ts` /
  `max_ts` (2026-06-18) and `user_filter`; `quote_creator_user_id` / `rfq_creator_user_id` are marked
  `#[deprecated]` to match the spec's `deprecated: true` on those filter parameters.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
