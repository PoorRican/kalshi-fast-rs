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

- Validated through the 2026-10-22 changelog (OpenAPI 3.34.0). Newly added response fields that the
  specs mark required are modeled as `Option` so payloads from before the rollout still parse:
  `Market.settlement_bounds_type` (`SettlementBoundsType`, `Unknown` catch-all) and
  `settlement_floor_dollars` (nullable, `floor` markets only);
  `MultivariateEventCollection.price_level_structure` / `price_ranges`; and on `MarginMarket`
  `market_version`, `funding_deadband`, `funding_cap`, `funding_limit_unit` and
  `funding_invalid_premium_handling` (the two unit/handling enums carry an `Unknown` catch-all).
  `MarginMarket.bid_size_fp` / `ask_size_fp` are omitted by the exchange when market stats are
  unavailable.
- `Market.liquidity_dollars` was removed upstream on 2026-10-01 (deprecated since February, always
  `"0.0000"`). The already-absent-from-spec `liquidity` and `liquidity_fp` were dropped with it.
  Use `yes_bid_size_fp` / `yes_ask_size_fp` for top-of-book size.
- `Market` and the other public market/event structs deliberately model spec-required fields as
  `Option`, so a field the exchange omits never fails a whole page. This predates the current refresh
  and is unchanged. `CreateMarginOrderRequest.ticker` is `String` although the spec also allows
  `market_id` instead.
- `sending_ts_ms` (added to every WebSocket message on 2026-10-01) is exposed on the margin
  `MarginEnvelope` only. The prediction-market `WsMessageV2` variants do not carry it: threading it
  through every owned and borrowed variant is a large breaking change on the hot path. Serde ignores
  the unknown key, so parsing is unaffected. Revisit if latency attribution is needed.
- `user_orders` `last_update_reason` is `WsLastUpdateReason` on the prediction channel (adds
  `SettlementBoundsCancel`). The margin enum (`MarginLastUpdateReason`) now has an `Unknown`
  catch-all, so a new reason string can no longer fail a whole message. The 2026-10-22 millisecond
  truncation of the legacy `created_time` / `last_update_time` / `expiration_time` strings needs no
  change: they stay RFC3339 `Option<String>`.
- Ed25519 API keys (2026-09-24, web-app default since 2026-10-01): `KalshiAuth::from_pem_str`
  detects RSA (PKCS#8/PKCS#1) or Ed25519 (PKCS#8) from the PEM. The pre-sign text is identical.
  `GenerateApiKeyRequest::key_type` / `GenerateApiKeyResponse::key_type` use `ApiKeyType`.
- Margin `corporate_actions` WebSocket channel (`MarginChannel::CorporateActions`) uses
  `MarginWsCorporateAction*` types, distinct from the REST `MarginCorporateAction*` types: the
  AsyncAPI split terms carry pre/post mark prices and the WS type set omits cash actions, whereas the
  REST notice has `status`, `updated_ts` and `cash` terms. The REST `cash` / `split` /
  `final_settlement` / `ticker_change` terms are `Option` because exactly one is present, and none
  are for `cancelled` notices.
- `GET /portfolio/balance` moved to its own server-side rate-limit bucket (2026-10-15). The
  client-side `RateLimitConfig` still counts balance calls against the read budget; this
  over-throttles slightly and never under-throttles, so no change was made.
- Known gaps that predate this refresh and are not part of the 2026-09-28..2026-10-22 delta:
  intra-exchange transfer endpoints (including `client_transfer_id`), `target_balance_allocation`
  (including `resting_margin_reservation`), margin exit triggers, `/margin/fee_tier_rates`, FCM
  subtrader endpoints, block-trade proposals, deposits/withdrawals, `exchange_index` on most request
  and response types, and the many legacy cent-denominated `Market` / `Order` fields that no longer
  appear in the spec. The external-feeds WebSocket host move (2026-10-22) is out of scope because the
  crate has no external-feeds client.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
