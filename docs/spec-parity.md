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

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST API schema on
  2026-07-09 (and are absent from the AsyncAPI `market_lifecycle_v2` message too). These fields are
  removed from the crate entirely rather than kept as `Option`, since the changelog confirms the
  exchange no longer sends them at all (not merely "deprecated but present").

- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated by Kalshi on
  2026-06-18 in favor of the lower-cost V2 event-order endpoints. Rather than removing them (the
  OpenAPI spec-first migration doesn't document a removal date), they're marked `#[deprecated]` so
  downstream code gets a compile-time nudge to migrate to `*_order_v2` while the legacy endpoints
  remain callable.

- Multivariate lookup history (`get_multivariate_event_collection_lookup_history` and
  `lookup_tickers_for_market_in_multivariate_event_collection`) is "fully deprecated" per the
  2026-07-02 changelog entry and the OpenAPI `deprecated: true` flag; predates RFQs. Both methods
  are marked `#[deprecated]` rather than removed, since the endpoints remain live.

- `GET /communications/quotes` no longer accepts `market_ticker` or `event_ticker` query filters as
  of 2026-06-20 (removed from the OpenAPI parameter list entirely). These fields are removed from
  `GetQuotesParams` rather than kept as inert `Option`s. `min_ts` / `max_ts` filters on last-update
  time were added 2026-06-18.

- RFQ-scoped quote action endpoints (`delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`,
  taking both `rfq_id` and `quote_id`) were added 2026-06-25. The quote-ID-only endpoints
  (`delete_quote`, `accept_quote`, `confirm_quote`) remain supported per the changelog and are kept
  as-is. RFQ quotes are no longer guaranteed durably queryable except in post-acceptance states
  (`accepted`, `confirmed`, `executed`); callers should track the RFQ ID alongside the quote ID and
  tolerate `404` on stale open/cancelled quotes.

- `WsMarketLifecycleV2` gained four more top-level fields that (per the AsyncAPI) only appear on
  specific event types: `price_ranges` (on `created` / `price_level_structure_updated`, added
  2026-07-02) and `strike_type` / `cap_strike` / `custom_strike` (on `metadata_updated`, added
  2026-06-18, alongside the pre-existing `floor_strike` / `yes_sub_title`). `price_ranges` reuses
  the REST `PriceRange` type.

- `EventData` (REST `GET /events`) gained `settlement_sources: Vec<SettlementSource>` (2026-06-18),
  mirroring the field already modeled on `Series` and `EventMetadata`. `GetEventsParams` gained
  `tickers` (comma-separated event ticker filter, 2026-06-18) and `min_updated_ts` (found via a spot
  check of the live OpenAPI parameter list for `GET /events`, not tied to a specific changelog
  entry).

- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (2026-07-02, per-index exchange
  status). Both are optional since the OpenAPI schema doesn't mark them required on
  `ExchangeStatus`, only on the nested `ExchangeIndexStatus` entries.

- `SubaccountBalance.exchange_index: i64` is a new **required** field (2026-07-02, per-index
  subaccount balances); a subaccount with funds on multiple exchange indexes now appears as
  multiple entries instead of one combined row. Modeled as a plain (non-`Option`) field since the
  OpenAPI marks it required and this is a genuinely new, non-conditional field.

- `POST /portfolio/subaccounts/positions/transfer` (`apply_subaccount_position_transfer`) was added
  2026-07-09 to move a position between subaccounts. `SubaccountTransfer` gained `exchange_index`,
  `transfer_type` (`cash` | `position`), and position-only fields (`market_ticker`, `side`, `count`,
  `price_cents`). These are modeled as `Option` even though the OpenAPI marks them required, because
  historical transfer rows recorded before this rollout may lack them.

- API keys can now be restricted to a single sub-account (2026-07-02): `subaccount: Option<u32>` was
  added to `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest`.

- Several 2026-06/07 changelog entries require no crate change and are intentionally not modeled:
  - Margin-market entries (order reasons, `is_portfolio`, per-market risk metric gating, subaccount
    on margin positions) — margin market types are out of scope for this crate, consistent with
    prior refreshes.
  - FIX protocol entries (exchange index routing, post-only quotes, reject reason codes, quote
    identity, trade entries in market data) — this crate only implements the REST and WebSocket
    surfaces, not FIX.
  - Pure rate-limit/policy changes (API usage tier qualification halved, Get Quote rate-limit cost,
    quote/RFQ retention window) — operational changes with no schema impact.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
