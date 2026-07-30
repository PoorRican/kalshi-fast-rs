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

- `GET /exchange/announcements` was removed from the Predictions REST API (2026-07-04). Unlike the
  deprecated-but-present fields elsewhere in this document, this is a genuine removal (not a
  soft-deprecation), so `get_exchange_announcements` and its response types (`Announcement`,
  `AnnouncementType`, `AnnouncementStatus`, `GetExchangeAnnouncementsResponse`) were deleted rather
  than kept around returning a guaranteed error. This is a breaking Rust API change (0.6.0 → 0.7.0).

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema (2026-07-09). These are
  kept as `Option` fields (payloads simply omit them now) but are documented with plain doc comments
  rather than the `#[deprecated]` attribute, matching the existing `Order.side` / `Order.action` /
  `Fill.side` / `Fill.action` convention in this crate: `#[deprecated]` on a struct field only
  warns cleanly when the field is never touched by hand-written code, and `serde`-derive-generated
  (de)serialization does not itself trigger the lint (confirmed: a `#[derive(Deserialize,
  Serialize)]` struct with a `#[deprecated]` field builds warning-free). `MarketPosition` is the
  concrete case that would break this: `MarketPositionRef::into_owned()` in `src/ws/types/mod.rs`
  hand-writes `resting_orders_count: self.resting_orders_count` to convert the WebSocket
  `market_positions` channel's borrowed position into the REST `MarketPosition` type, and that
  hand-written access would warn under `#[deprecated]`. `ErrorResponse.service` (removed from error
  responses upstream 2026-08-06, deprecated 2026-07-28) follows the same doc-comment convention: it
  has a real, non-decorative reader in `rest/retry.rs`'s retry classification, so marking it
  `#[deprecated]` would warn on legitimate production code.

- `#[deprecated]` (the attribute) is used only on `KalshiRestClient` *methods* being replaced by a
  newer method, since method deprecation has no equivalent derive/conversion friction:
  - Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
    `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated upstream
    2026-06-18/25 in favor of the V2 event-order endpoints, and now return an error directing
    callers to V2. `tests/rest_orders.rs` intentionally keeps exercising the legacy surface for
    regression coverage and carries `#![allow(deprecated)]` for that reason.
  - The quote-ID-only endpoints (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) were
    deprecated upstream 2026-06-25 in favor of the RFQ-scoped equivalents
    (`get_quote_scoped`/`delete_quote_scoped`/`accept_quote_scoped`/`confirm_quote_scoped`); the RFQ
    ID is expected to become required for quote actions in a future migration.
  `tests/rest_communications.rs` carries the same `#![allow(deprecated)]` allowance.

- `update_order_group_limit` gained a `SubaccountQueryParams` argument (2026-07-30/08-06) so a
  subaccount's order group can be targeted, matching every sibling order-group method
  (`get_order_group`, `delete_order_group`, `reset_order_group`, `trigger_order_group`). This is a
  breaking Rust API change (0.6.0 → 0.7.0).

- `SubaccountBalance.exchange_index: i64` was added as a required field (2026-07-02): balances are
  now reported per exchange index, so a subaccount with funds on multiple indexes appears as
  multiple entries instead of one combined row. This is a breaking Rust API change (0.6.0 → 0.7.0).

- `GetQuotesParams.market_ticker` / `.event_ticker` are no longer functional upstream (2026-06-20);
  the server silently ignores them rather than rejecting the request, so they are kept as `Option`
  fields (doc-noted, not removed) instead of forcing a breaking change for a filter that can be
  replaced by user/RFQ/status/time filters.

- The `pyth_value` WebSocket channel (added 2026-07-23) mirrors the `cfbenchmarks_value` channel's
  shape: `underlying_tickers` (not `index_ids`) seeds/updates the subscription, `["all"]` subscribes
  to every underlying, and `subscribe_underlyings` / `unsubscribe_underlyings` / `underlying_list`
  are the update actions (vs. `subscribe_indices` / `unsubscribe_indices` / `indexlist`). Unlike
  `cfbenchmarks_value`, `pyth_value` requires authentication (`WsChannelV2::PythValue::is_private()`
  is `true`).

- `WsMarketLifecycleV2.exchange_index` and `.price_ranges` mirror the REST `Market.price_ranges`
  shape (`{ start, end, step }` fixed-point dollar bands) and are present only on `created` /
  `price_level_structure_updated` events per the AsyncAPI. The AsyncAPI marks `exchange_index`
  required on `event_lifecycle` messages, but — consistent with the existing best-effort treatment
  of `title`/`subtitle`/`collateral_return_type`/`series_ticker` on that same message — it is modeled
  as `Option` rather than trusted as always-present.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
