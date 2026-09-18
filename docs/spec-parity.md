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

- **Legacy `/portfolio/orders` mutation endpoints (V1).** Kalshi began deprecating
  `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/
  `batch_cancel_orders` between 2026-06-18 and 2026-06-25, and by the 2026-09-18 refresh their
  routes (`POST /portfolio/orders`, `DELETE /portfolio/orders/{order_id}`,
  `POST /portfolio/orders/{order_id}/amend`, `POST /portfolio/orders/{order_id}/decrease`,
  `POST`/`DELETE /portfolio/orders/batched`) are no longer present in the OpenAPI spec at all
  (only `GET /portfolio/orders` and `GET /portfolio/orders/{order_id}` remain). Rather than
  removing the methods outright — which would break any caller still migrating — they are marked
  `#[deprecated]` and continue to call the same paths; use the `_v2` event-order methods
  (`create_order_v2` etc., `/portfolio/events/orders/*`) instead. If Kalshi confirms the legacy
  routes now hard-404, these should be removed in a future minor release.

- **Multivariate lookup surface removed.** The ticker-pair lookup endpoint
  (`PUT /multivariate_event_collections/{collection_ticker}/lookup`), the lookup-history endpoint
  (`GET .../lookup`), and the `multivariate`/`multivariate_lookup` WebSocket channel and message
  type were fully removed by Kalshi (deprecated 2026-07-02, removed 2026-08-06) and are absent
  from both the current OpenAPI and AsyncAPI specs (the AsyncAPI's channel-name enum no longer
  lists `multivariate`). The corresponding crate methods, request/response types, and
  `WsMultivariate`/`WsMultivariateRef` message types were removed in 0.8.0. Use
  `create_market_in_multivariate_event_collection` to create or resolve a combo market, and the
  `multivariate_market_lifecycle` channel for lifecycle state.

- **`exchange_index` is `Option<i64>` everywhere**, even on schemas where the OpenAPI/AsyncAPI
  mark it `required`. Kalshi is mid-rollout on exchange sharding (only index `0` is live in
  production as of this refresh; margin markets are all `exchange_index=0` for now), so treating
  the field defensively avoids parse failures against older/cached payloads or non-Predictions
  exchanges the crate doesn't model.

- **`EventMetadata.cadence`** lives on two different response shapes that the crate models with
  one struct: the dedicated `GET /events/{event_ticker}/metadata` response (whose OpenAPI schema
  is `EventMetadata`: `image_url`, `settlement_sources`, `market_details`, ...) and
  `EventData.product_metadata`, which the OpenAPI spec types as a free-form `object` with no
  defined shape. `cadence` (added 2026-07-30) is documented only as living inside
  `product_metadata`; it is added to the shared `EventMetadata` struct for ergonomic access on
  both call sites, tolerated via `#[serde(default)]`.

- **Deferred: new endpoints/channels not yet modeled.** The following upstream additions from the
  2026-09-18 refresh are new API surface (not shape changes to anything already modeled), so
  leaving them out does not desync existing behavior — they are tracked here rather than in
  `CHANGELOG.md`'s per-release table beyond a pointer:
  - `pyth_value` and `cfbenchmarks_value_5hz` WebSocket channels (new subscribable channels
    alongside the already-modeled `cfbenchmarks_value`).
  - `GET /live_data/events/{event_ticker}` (event-keyed live data), `GET /live_data/weather/{city}`
    and `GET /live_data/weather/{city}/calibrations` (Kalshi Weather Index).
  - Target balance allocation (`POST`/`GET /portfolio/target_balance_allocation`).
  - Intra-exchange-instance transfer history (`GET /portfolio/intra_exchange_instance_transfers[/…]`,
    `POST /portfolio/intra_exchange_instance_transfer`).
  - Cancel-all-orders endpoints (new `/portfolio/*` and `/margin/*` cancel-all routes).
  - `exchange_index`/`subaccount` scoping query params on `GET /portfolio/balance` — the crate's
    `get_balance()` currently takes no params at all; adding scoping requires a signature change
    and is deferred to avoid bundling an unrelated breaking change into this refresh.
  - `resting_order_value_breakdown` (per-exchange-index breakdown) on
    `GetPortfolioRestingOrderTotalValueResponse`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
