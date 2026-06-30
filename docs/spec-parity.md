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

- `GET /exchange/status` (`get_exchange_status`) now returns two additional optional fields
  (`intra_exchange_transfers_active: bool` and `exchange_index_statuses: array`) added in 2026-06.
  Both are `Option` because they are not in the OpenAPI `required` list and the field
  `exchange_index_statuses` is explicitly described as "absent when the per-index breakdown is
  unavailable". The new `ExchangeIndexStatus` struct carries the four required per-index fields.

- `GET /portfolio/subaccounts/balances` responses now include `exchange_index: i64` on each
  `SubaccountBalance` entry (required per current OpenAPI, added 2026-06-24). This is a structural
  change — the field is non-optional in the struct. Payloads predating the field will fail to
  deserialize, but the exchange began populating it immediately; old records are not expected.

- `EventData` (returned by `GET /events` and `GET /events/{ticker}`) now exposes
  `settlement_sources: Option<Vec<SettlementSource>>` as a typed field rather than capturing it in
  the `extra` flatten. The OpenAPI marks this field `required` but also `nullable`; it is modeled
  as `Option` so payloads where the field is absent or `null` still parse. The fields
  `fee_type_override: Option<String>`, `fee_multiplier_override: Option<f64>`, and
  `exchange_index: Option<i64>` are also now explicit (previously fell into `extra`).

- `WsMarketLifecycleV2` (and `WsMarketLifecycleV2Ref`) now expose `strike_type: Option<String>`
  and `cap_strike: Option<f64>` at the **top level** for `metadata_updated` events, matching the
  AsyncAPI update from 2026-06. These join the existing top-level `floor_strike` and `yes_sub_title`.
  They also remain in `WsMarketLifecycleAdditionalMetadata` for market-creation events.

- `GetQuotesParams` no longer includes `market_ticker` or `event_ticker`. These parameters were
  removed from `GET /trade-api/v2/communications/quotes` effective 2026-06-20 per the changelog and
  are absent from the current OpenAPI spec. The new `user_filter: Option<String>` (filter for quotes
  created by the authenticated user) and timestamp filters `min_ts`/`max_ts` were added instead.

- RFQ-scoped quote actions (`DELETE/PUT /communications/rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`)
  are now available via `delete_rfq_quote`, `accept_rfq_quote`, and `confirm_rfq_quote`. The older
  quote-ID-only endpoints remain supported by the exchange and the crate; the scoped variants are
  preferred because they tolerate the reduced quote-retention window (7 days post-close) more
  gracefully.

- `SeriesFeeChange` field types updated to match the current OpenAPI: `id` changed from `i64` to
  `String`; `fee_multiplier` changed from `i64` to `f64`; `scheduled_ts` changed from `i64` to
  `String` (ISO 8601 date-time). This was a pre-existing divergence from the OpenAPI schema.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
