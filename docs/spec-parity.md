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

- `Market.liquidity_dollars` was removed upstream (2026-09-24; OpenAPI 3.31.0 no longer lists it), so
  the crate field was removed rather than `#[deprecated]` (0.10.0). `settlement_bounds_type` is
  `required` in the spec but modeled as `Option<SettlementBoundsType>` (with `#[serde(other)] Unknown`)
  so partial/older payloads parse; `settlement_floor_dollars` is nullable and only set when the type is
  `floor`. The `Market` struct intentionally keeps legacy integer/cents fields the spec no longer lists.

- `AmendOrderV2Request.expiration_time`: `None` preserves expiry, `Some(0)` clears it, a future Unix
  second sets it. The v1 `AmendOrderRequest` has no such field in the spec. Margin amend already had it.

- Ed25519 API keys: `KalshiAuth` detects RSA vs Ed25519 (PKCS#8) from the PEM; the pre-sign text and
  headers are identical. `CreateApiKeyRequest` has no `key_type` in the spec (inferred from the public
  key); only `GenerateApiKeyRequest/Response` carry it. Not modeled yet: `subaccount` /
  `fcm_subtrader_id` on the generate request and `warning` on the responses (Generate response
  `warning` and `ApiKey` extras land in `extra` where a flatten exists).

- `communications` `user_filter` is `""` or `"self"` (`WsUserFilter`); it is a subscribe-time param only.
  `obscure_creator_id` RFQs show creator id `"0"` to other users, so treat `creator_id` as opaque.

- `user_orders.last_update_reason` is not in the spec's `required` list, so it is `Option`.

- Not in the crate (pre-existing gaps, not part of this refresh): margin isolated-position exit-trigger
  endpoints (`/margin/isolated/positions/{ticker}/exit_trigger`; the 2026-09-28 "prices must be positive"
  rule would apply), `Market.exchange_index`, and FIX-only changes.

## Changelog Reconciliation (validated through 2026-09-29)

| Changelog entry | Disposition |
| --- | --- |
| Amend order expiry | `AmendOrderV2Request.expiration_time` (margin already had it); FIX n/a |
| Exit trigger prices must be positive | No change: exit-trigger endpoints not implemented (see gaps above) |
| Settlement bounds on markets | `Market.settlement_bounds_type` / `settlement_floor_dollars` |
| SETTLEMENT_BOUNDS_CANCEL (FIX) | No change: FIX only |
| Reduce-only orders over FIX | No change: FIX only (REST `reduce_only` already modeled) |
| RFQ creators may obscure pre-trade id | `CreateRFQRequest.obscure_creator_id` (+ `target_cost_excludes_fees`); ids are plain strings |
| ClearingBusinessDate on Margin FIX | No change: FIX only |
| user_orders include last_update_reason | `WsUserOrder.last_update_reason`; margin variant already present |
| Filter communications RFQs to own user | `WsSubscriptionParamsV2.user_filter` |
| liquidity_dollars removed | Field removed (breaking) |
| RFQ/quote creation timestamps over FIX | No change: FIX only |
| Ticker reference_price for Pyth perps | No change: `reference_price` already `Option<TickerPrice>` on margin ticker |
| Ed25519 API keys | `KalshiAuth` Ed25519 support, `ApiKeyType`, `key_type` on generate |
| 20% higher rate limits | No change: docs-only tier values; limits are fetched from `/account/limits` |
| Optional WebSocket compression | No change: opt-in by the client; the crate does not offer permessage-deflate |

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
