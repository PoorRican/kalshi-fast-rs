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

- Exchange sharding (2026-07/08) added an `exchange_index` field to most portfolio, order, order-group,
  market, event, series, and multivariate-collection response shapes. Even where the OpenAPI/AsyncAPI
  now mark it `required`, it is modeled as `Option<i64>` across the crate — consistent with how every
  other field added to an *existing* struct is modeled here — so older/cached payloads without the
  field still parse. Plain `integer` fields typed like this (not an enum) are represented directly as
  `i64`; there is no dedicated `ExchangeIndex` newtype.
- `SubaccountQueryParams` (shared by the single-order-group endpoints: get/delete/reset/trigger/limit)
  gained an `exchange_index` query field to match the OpenAPI `ExchangeIndexQuery` parameter added to
  those endpoints. `GET /portfolio/order_groups` (list) does not accept `exchange_index` upstream; the
  field is harmlessly unused there when left `None`.
- `price_level_structure` (REST `Market.price_level_structure`, WS `market_lifecycle_v2`) is modeled as
  a raw `String`, not an enum, so new structure names Kalshi has introduced since the last refresh
  (`center_whole_edge_half_cent` and siblings, `center_deci_edge_centi_cent`) round-trip with no crate
  changes. Consult the live OpenAPI/AsyncAPI for the current enum values; do not hardcode logic against
  the label — read `Market.price_ranges` / the WS `price_ranges` field for actual tick bands instead.
- `FeeType::QuadraticWithComboMakerFees` (serialized `quadratic_with_maker_fees` sibling
  `quadratic_with_combo_maker_fees`) was added to the OpenAPI enum; the existing `#[serde(other)]
  Unknown` catch-all means this was never a hard-failure gap, but the variant is now named explicitly.
- The following fields were fully removed from the live OpenAPI schema (not deprecated-and-present) and
  were removed from the crate rather than kept as optional compatibility fields, per the refresh
  workflow's default: `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` (removed 2026-07-09), and `ErrorResponse.service` (deprecated
  2026-07-28, removed 2026-08-06 — use `code` instead, which is present on every error response).
- `GET /exchange/announcements` and the entire "multivariate lookup" surface — the REST endpoint
  `PUT /multivariate_event_collections/{ticker}/lookup` plus the `multivariate` WebSocket channel
  (message type `multivariate_lookup`) — were removed from the live docs (2026-07-04 and 2026-08-06
  respectively) and have been deleted from the crate's public API, including the `WsChannelV2::
  Multivariate` / `WsMsgType::Multivariate(Lookup)` enum variants. Use `MultivariateMarketLifecycle`
  for multivariate market state changes, and `POST /multivariate_event_collections/{ticker}` for
  create/resolve.
- `GET /communications/quotes` dropped its `market_ticker` / `event_ticker` filters (2026-06-20) in
  favor of RFQ-scoped lookup/action endpoints (`.../rfqs/{rfq_id}/quotes/{quote_id}...`, added
  2026-06-25/2026-07-09). The crate exposes both the RFQ-scoped methods (`get_quote_scoped`,
  `delete_quote_scoped`, `accept_quote_scoped`, `confirm_quote_scoped`) and the still-supported but
  upstream-deprecated quote-ID-only methods; prefer the scoped ones for new code.
- `WsChannelV2::CfbenchmarksValue` requires authentication per the AsyncAPI description, but was
  missing from `WsChannelV2::is_private()` (which gates the client-side `AuthRequired` pre-check on
  `subscribe_v2`). Fixed as part of this refresh; `PythValue` (added 2026-07-23, also authenticated)
  was added to the same list from the start.
- The `pyth_value` channel mirrors `cfbenchmarks_value`'s shape: `WsUpdateAction` gained
  `SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList`, and
  `WsSubscriptionParamsV2` / `WsUpdateSubscriptionParamsV2` gained `underlying_tickers`. The
  subscription tracker folds underlying add/remove updates into the resubscribe state the same way it
  already did for CF Benchmarks index IDs.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
