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
  `taker_side` (deprecated) plus `taker_outcome_side` / `taker_book_side`.
- **Updated for OpenAPI 3.28.0:** the normalized fields have since become `required` and
  non-nullable on the `Trade`, `Fill`, and `Order` schemas, so `Trade::taker_outcome_side` /
  `taker_book_side`, `Fill::outcome_side` / `book_side`, and `Order::outcome_side` / `book_side`
  are now plain (non-`Option`) types. Only the legacy `side` / `action` / `taker_side` fields
  remain `Option`, and they are marked `#[deprecated]` — the specs still carry them with
  `deprecated: true`, so they are kept rather than removed.

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

- `is_block_trade: bool` was added to the public REST `Trade` struct (2026-05-29) and to the
  WebSocket `trade` message (2026-08-13). Both specs now mark it `required`, so the REST `Trade`
  field is plain `bool` with no `serde` default. `WsTrade::is_block_trade` keeps `#[serde(default)]`
  so frames predating the 2026-08-13 WebSocket rollout still parse. The query filter
  `GetTradesParams::is_block_trade: Option<bool>` lets callers filter by block-trade status.

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

## OpenAPI 3.28.0 / AsyncAPI 2.0.0 Refresh (0.8.0)

- `ErrorResponse.service` was deprecated on 2026-07-28 and removed on 2026-08-06. OpenAPI 3.28.0
  declares `ErrorResponse` with only `code`, `message`, and `details` (and no `required` set), so
  the field was removed from the Rust struct rather than retained as an `Option`. Branch on `code`,
  which is the field Kalshi documents as present on every error response.

- `FeeType` gained `quadratic_with_combo_maker_fees` (combo RFQ maker/taker fee swap, 2026-08-22).
  The `#[serde(other)] Unknown` catch-all still absorbs any future variant.

- **Changelog and YAML disagree on the legacy order-mutation endpoints, and the YAML wins.** The
  2026-06-18 changelog entry says the legacy `/portfolio/orders` mutation endpoints would be
  *deprecated* between June 18 and June 25. OpenAPI 3.28.0 no longer declares them at all —
  `/portfolio/orders` exposes only `GET`, and `POST /portfolio/orders`,
  `DELETE /portfolio/orders/{order_id}`, `POST /portfolio/orders/{order_id}/decrease`,
  `POST /portfolio/orders/{order_id}/amend`, `POST /portfolio/orders/batched`, and
  `DELETE /portfolio/orders/batched` are absent. Per the repo rule that a shape removed from the
  live schema is removed from the public Rust API, the corresponding V1 methods were dropped.
  Callers migrate to the V2 event-order methods (`create_order_v2`, `cancel_order_v2`,
  `amend_order_v2`, `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`),
  which use a single price plus `BookSide` instead of separate yes/no prices.

- The quote-ID-only communications endpoints are the opposite case: **deprecated but still present**
  in the YAML (`GetQuote`, `DeleteQuote`, `AcceptQuote`, `ConfirmQuote` all carry
  `deprecated: true`). They are kept and marked `#[deprecated]` in Rust rather than removed. The
  RFQ-scoped replacements (`GET|DELETE /communications/rfqs/{rfq_id}/quotes/{quote_id}` and the
  `/accept` and `/confirm` sub-paths) are the supported path. Kalshi has stated that `rfq_id` will
  become *required* for quote actions in a future migration, so callers should store the RFQ ID
  returned at RFQ creation and use the scoped methods now.

- `GET /communications/quotes` lost its `market_ticker` and `event_ticker` filters outright on
  2026-06-20 (confirmed absent from the YAML) and gained `min_ts` / `max_ts`. Filter quotes by
  user, RFQ, status, or update time instead.

- The `multivariate` WebSocket channel and its `multivariate_lookup` message were removed on
  2026-08-06, along with `PUT /multivariate_event_collections/{collection_ticker}/lookup` and the
  lookup-history endpoint. AsyncAPI 2.0.0 no longer lists the channel, so the `WsChannelV2` and
  `WsMsgType` variants were removed rather than left as dead wire names. Unknown message types
  still decode to `WsMsgType::Unknown`, so a stale server sending `multivariate_lookup` is
  tolerated rather than fatal. Use `multivariate_market_lifecycle` for multivariate market state.

- `pyth_value` is a new authenticated AsyncAPI channel (2026-07-23). It mirrors the
  `cfbenchmarks_value` design but keys on `underlying_tickers` rather than `index_ids`, with
  `["all"]` tracking every available underlying. It emits `pyth_value` (per-underlying price) and
  `pyth_value_underlying_list` (discovery). Subscription updates use the
  `subscribe_underlyings` / `unsubscribe_underlyings` / `underlying_list` actions, and
  `validate_update` applies the same guards as the index actions: underlying actions may not be
  mixed with market targets, and the add/remove actions require `underlying_tickers`.

- `Market.price_level_structure` is intentionally `Option<String>` and **not** an enum. The OpenAPI
  schema types it as an open `string` with no `enum`, and Kalshi shipped eight new values in three
  weeks (seven on 2026-07-23, plus `center_deci_edge_centi_cent` on 2026-08-13). Kalshi's own
  guidance is to derive valid prices from the market's `price_ranges` array
  (`{start, end, step}` fixed-point dollar bands) rather than branching on the structure label.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the schema on 2026-07-09 and are removed
  from the Rust structs. Payloads from a stale server that still carry them decode into the
  struct's flattened `extra` map rather than failing.

- The crate still declares the pre-fixed-point-migration cent/integer fields on `Market` and
  `MarketCandlestick` (`yes_bid`, `volume`, `open_interest`, `tick_size`, and friends) even though
  OpenAPI 3.28.0 no longer declares them. This is a deliberate, documented exception to the
  "remove what the schema removed" rule: the fields are the compatibility surface for payloads
  predating the fixed-point migration, they are already modeled as `Option`, and removing them is a
  large break unrelated to this refresh. The fixed-point `*_dollars` / `*_fp` fields are the
  canonical source. Prices below $0.01 and above $0.99 (combo markets, from 2026-08-27) cannot be
  represented in the integer-cent fields at all, so read prices from the `*_dollars` fields.

- The subaccount range is **0–63** (0 = primary), stated throughout the OpenAPI. The crate's
  `validate()` methods previously rejected anything above 32, which wrongly refused valid
  subaccounts 33–63; they now enforce `0..=63`.

- `api_key_region_expiration_ts` (2026-08-16) sits on `GetApiKeysResponse`, not on each `ApiKey` —
  it is an account-level attestation expiry, which the changelog prose does not make obvious. It is
  `Option` because it is absent when the account has never attested.

- `ApiKeyScope` is a closed enum upstream (`read`, `write`, `read::block_trade_accept`,
  `read::portfolio_balance`, `write::trade`, `write::transfer`, `write::block_trade_accept`), but
  API key scopes are modeled as `Vec<String>`. Four of those seven scopes were minted inside a
  single three-month window; a `String` round-trips a new scope losslessly with no crate release,
  whereas the crate's `#[serde(other)] Unknown` enum convention would *lose* the scope name on a
  value it does not know — which matters because scopes are echoed back on writes.

- `GetSubaccountBalancesResponse` is now keyed by `(subaccount_number, exchange_index)` rather than
  by subaccount alone (2026-07-02). A subaccount holding funds on several exchange indexes appears
  as several entries, so `subaccount_number` is no longer unique within the response.

- `WsFill::exchange_index` and `WsEventLifecycle::exchange_index` are marked required in the
  AsyncAPI but modeled as `Option`, because the fields shipped on 2026-08-20 and frames predating
  the rollout must still parse. The same reasoning applies to `ts_ms` elsewhere.

- `GetFillsResponse::cursor` is marked `required` in the OpenAPI while the sibling
  `GetPositionsResponse::cursor` and `GetSettlementsResponse::cursor` are optional. All three are
  modeled as `Option<String>` (absent or empty = last page): the pagination helpers depend on it,
  and the asymmetry reads as a spec artifact rather than a semantic difference.

- **Changelog and YAML disagree on the fractional RFQ quote field names** (2026-06-11). The
  changelog names them `yes_contracts_offered_fp` / `no_contracts_offered_fp`; the OpenAPI `Quote`
  schema has no such properties. The real names are `yes_contracts_fp` / `no_contracts_fp`, which
  the crate already used. YAML followed.

- RFQ quotes are only guaranteed queryable once they reach `accepted`, `confirmed`, or `executed`
  (2026-06-25). `open` and `cancelled` quotes are best-effort; a quote cleared by a server roll
  should be treated as effectively cancelled even when no cancelled record exists, and a later
  lookup may return `404`. Callers should not treat a `404` from `get_rfq_quote` as a client bug.

- Two `serde` shapes in the crate were unusable and are fixed rather than preserved:
  `SeriesFeeChange` declared `id` / `fee_multiplier` / `scheduled_ts` as integers where the
  OpenAPI declares `string` / `number` / `string(date-time)`, and `GetLiveDatasParams` derived
  `Serialize` with a `Vec<String>` field even though `milestone_ids` uses OpenAPI
  `form` / `explode: true` repeated parameters, which `serde_urlencoded` cannot emit from a struct
  field. Both would have failed against the live API.

### Known coverage gaps

These endpoints exist in OpenAPI 3.28.0 and are not yet modeled. All predate the 2026-06-08
watermark except where noted, so none is drift introduced by this refresh:

- `GET|POST /communications/block-trade-proposals` and
  `POST /communications/block-trade-proposals/{id}/accept`
- `GET /portfolio/deposits`, `GET /portfolio/withdrawals`
- `GET /events/fee_changes`
- `use_yes_price` on WebSocket orderbook subscription params
- `include_volume` on `GET /series/{series_ticker}` and `include_player_stats` on
  `GET /live_data/{type}/milestone/{milestone_id}` (both would change existing method signatures)

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
