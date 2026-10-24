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

- **2026-10 refresh (changelog through 2026-10-29; OpenAPI 3.34.0, Perps OpenAPI 0.0.1, AsyncAPI 2.0.0).**
  - `liquidity_dollars` was removed upstream (always `"0.0000"` since Feb 2026) and is removed from
    `Market`, along with the legacy `liquidity` / `liquidity_fp` (not in the schema). Top-of-book size
    is `yes_bid_size_fp` / `yes_ask_size_fp`. `Market` still models every spec-required field as
    `Option` (and keeps some pre-fixed-point legacy fields) by design; new fields
    `settlement_bounds_type` (`default` | `floor`), `settlement_floor_dollars` and `exchange_index` follow that.
  - `clock` on `orderbook_snapshot` / `orderbook_delta` is required by the AsyncAPI but modeled as
    `Option<u64>`: captured pre-2026-10-21 frames lack it and dropping a book update over a missing
    sequencing aid is worse than a `None`. It is the cross-connection ordering key (unlike `seq`).
  - `sending_ts_ms` is envelope metadata on every message. It is exposed on `WsEnvelope` and
    `MarginEnvelope`; the typed Predictions `WsMessageV2` variants deliberately do not carry it
    (adding it would break every exhaustive match for a latency-only field). Read it from the raw frame.
  - `user_orders.last_update_reason` is `WsOrderUpdateReason` with an `Unknown` catch-all, since the
    spec's enum has grown twice already (`ReduceOnlyCancel`, `SettlementBoundsCancel`).
    `exchange_index` on `WsUserOrder` is spec-required but `Option` for older frames.
  - Newly spec-required REST fields are modeled as required where the endpoint is new or the field
    cannot reasonably be absent: `SubaccountBalance.exchange_index`, `MultivariateEventCollection`
    `price_level_structure` / `price_ranges`, `MarginMarket` `market_version` and
    `funding_{deadband,cap,limit_unit,invalid_premium_handling}`. `MarginMarket.schedule` is spec-required
    but kept `Option` (pre-existing).
  - `CreateRFQRequest.target_cost_centi_cents` is `#[deprecated]` (spec marks it deprecated, still
    accepted); the struct now derives `Default` so callers can omit it with `..Default::default()`.
  - Margin `bid_size_fp` / `ask_size_fp` are omitted when market stats are unavailable, hence `Option`.
  - `GET /portfolio/subaccounts/balances` returns one row per `(subaccount_number, exchange_index)`.
    Subaccount validation is `0..=63` (spec text "0 for primary, 1-63"; the schema only has `minimum: 0`).
  - `GET /portfolio/balance` now has its own rate-limit bucket (1 rps Basic/Advanced, 5 rps above).
    The client-side limiter still charges balance calls to the read bucket (conservative; no change).
  - The Predictions `GET /portfolio/positions` `event_ticker` description now says a single event
    ticker; the crate still sends a CSV list (up to 10) unchanged.

### Known gaps (not implemented; tracked, not silently dropped)

These upstream surfaces have no Rust counterpart. Several predate the previous watermark, so earlier
"validated through" dates covered the changelog text, not full endpoint coverage.

- Predictions REST: `POST /portfolio/intra_exchange_instance_transfer` and its list/get endpoints
  (so the 2026-10-22 `client_transfer_id` echo has nothing to attach to), `GET /portfolio/deposits`,
  `GET /portfolio/withdrawals`, RFQ batches and block-trade proposals (`/communications/rfq_batches*`,
  `/communications/block-trade-proposals*`), `/account/api_usage_level/*`, `/live_data/weather/*`,
  `/live_data/events/{event_ticker}`, `/fcm/subtraders*`, `/fcm/fills`, `/portfolio/target_balance_allocation`.
- Margin REST: margin exit triggers (`/margin/{isolated,cross}/positions/{ticker}/exit_trigger*`; the
  2026-10-01 `stop_loss_past_liquidation` and strictly-positive-price rules are server-side validations),
  `/margin/fee_tier_rates`, `/margin/fcm/*`, `/portfolio/intra_exchange_instance_transfer`.
- WebSocket: Predictions channels `batch_communications`, `cfbenchmarks_value_5hz`, `pyth_value`.
- Auth: Ed25519 request signing (`ApiKeyType::ed25519`). The web app now generates Ed25519 keys by
  default, but the crate signs RSA-PSS only, so new web-generated keys must be created as RSA.
- FIX: out of scope for this crate.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
