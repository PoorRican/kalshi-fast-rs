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

- `price_level_structure` (REST `Market`, WebSocket `market_lifecycle_v2`) is kept as a raw
  `String`, not an enum. Kalshi has added ten variants since this crate started tracking it
  (`center_deci_edge_centi_cent`, seven `center_*_edge_*_cent` structures, etc.) with no
  accompanying API/message-format change each time — clients are expected to read the market's
  `price_ranges` array (`{start, end, step}` fixed-point-dollar bands) to determine valid order
  prices rather than branch on the structure name. Keeping it a raw string means new structure
  names round-trip without a crate update.

- Exchange sharding (multiple `exchange_index` values) began rolling out 2026-07 through 2026-08.
  Every `exchange_index` field added during this rollout is modeled as `#[serde(default)]` — either
  `i64` defaulting to `0` (the primary/only shard in production today) on already-shipping message
  types (`WsFill`, `WsUserOrder`), or `Option<i64>` where the field is explicitly conditional per
  the spec (`WsMarketLifecycleV2`, `WsEventLifecycle`) — rather than a hard-required field, so
  parsing never breaks if a payload lags the rollout on some code path. `GET /portfolio/balance`,
  `PUT /portfolio/order_groups/{id}/limit`, `GET/POST /portfolio/target_balance_allocation`, and the
  `/portfolio/intra_exchange_instance_transfer*` endpoints all take an `exchange_index` parameter
  scoped to "defaults to 0" per the OpenAPI spec; only exchange index `0` is available in production
  as of this writing.

- `pyth_value` is a new authenticated AsyncAPI channel (introduced 2026-07-23) mirroring
  `cfbenchmarks_value`'s shape: `underlying_tickers` (not market tickers) seeds the initial
  subscription (`["all"]` for every underlying), and `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` manage it afterward via `update_subscription_v2`,
  validated the same way as the CF Benchmarks index actions (mutually exclusive with market
  targets, `underlying_tickers` required for subscribe/unsubscribe). Two message types:
  `WsPythValue` (deduplicated per-underlying price) and `WsPythUnderlyingList` (discovery of
  recently streamed underlyings).

- The `multivariate` WebSocket channel (message type `multivariate_lookup`) and the REST
  `PUT /multivariate_event_collections/{ticker}/lookup` endpoint were removed by Kalshi on
  2026-08-06 (they predated RFQs). Both were removed from the public Rust API rather than kept as
  dead code — use `multivariate_market_lifecycle` for market state changes and the communications
  (RFQ) APIs for quoting.

- `GetPositionsParams.event_ticker` was previously modeled as a CSV `Option<Vec<String>>`, but
  `GET /portfolio/positions` actually takes `SingleEventTickerQuery` — one event ticker, not a
  list. Corrected to `Option<String>` (breaking). `GetOrdersParams.event_ticker` is a genuinely
  different, CSV-based parameter on a different endpoint and is unaffected.

- Subaccount numbers range `0-63` (0 = primary, 1-63 = subaccounts) per every `Subaccount*` query
  parameter in the OpenAPI spec. Client-side validation in `GetPositionsParams`, `GetOrdersParams`,
  and `CreateOrderRequest` previously capped this at `32` (stale from an earlier, narrower
  subaccount range) and rejected valid requests for subaccounts 33-63; corrected to `0..=63`.

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) and the quote-ID-only
  communications methods (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) were
  deprecated upstream (2026-06-18 and 2026-06-25 respectively) in favor of cheaper V2 order
  endpoints and RFQ-scoped quote actions. Both are still fully functional upstream, so this crate
  keeps them and only adds doc-comment deprecation notices pointing at the replacements — no
  removal until Kalshi actually retires them.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
