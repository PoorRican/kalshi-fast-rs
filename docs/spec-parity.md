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

- `market.response_price_units`, `market.fractional_trading_enabled`, and
  `market_positions.resting_orders_count` were removed outright from the Predictions REST schema on
  2026-07-03 (not deprecated-in-place — gone). Per the crate's removal policy these fields were
  deleted from `Market` (REST and `WsMarketLifecycleV2`/`Ref`) and `MarketPosition` (REST and
  `MarketPositionRef`) rather than kept as stale `Option` fields. The
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant was removed for the same reason —
  the AsyncAPI's `market_lifecycle_v2` `event_type` enum no longer lists `fractional_trading_updated`.
  This is an intentional minor-version break (0.6.0 → 0.7.0).

- `GET /exchange/announcements` was removed from the OpenAPI spec on 2026-07-04. The crate's
  `get_exchange_announcements` method and `GetExchangeAnnouncementsResponse` / `Announcement` /
  `AnnouncementType` / `AnnouncementStatus` types were deleted rather than kept as dead code
  (intentional minor-version break, 0.6.0 → 0.7.0).

- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (2026-06, per-index exchange status).
  Both are `Option` because the top-level fields reflect only the default exchange index (0) and the
  per-index breakdown is documented as "absent when unavailable."

- `SubaccountBalance.exchange_index: i64` and `SubaccountTransfer.exchange_index: i64` /
  `SubaccountTransfer.transfer_type: SubaccountTransferType` are newly *required* fields in the
  OpenAPI spec (2026-06, per-index subaccount balances/transfers) that were entirely missing from
  the crate's structs — not a case of upstream relaxing a requirement, but a gap this refresh closed.
  `SubaccountTransfer` also gained the position-transfer-only fields `market_ticker`, `side`,
  `count`, and `price`, modeled as `Option` since the spec marks them present only for
  `transfer_type: position` rows.

- The multivariate lookup-history endpoint (`GET
  /multivariate_event_collections/{collection_ticker}/lookup`, exposed by the crate as
  `get_multivariate_event_collection_lookup_history`) no longer exists anywhere in the OpenAPI spec
  (changelog: "fully deprecated," 2026-07-02). The method and its `GetMultivariateEventCollection
  LookupHistoryParams` / `Response` / `LookupPoint` types were removed rather than kept as dead code
  (intentional minor-version break, 0.6.0 → 0.7.0). The sibling `PUT` lookup endpoint
  (`lookup_tickers_for_market_in_multivariate_event_collection`) still exists but is now marked
  `deprecated: true` in the OpenAPI spec ("predates RFQs"); it is kept and annotated
  `#[deprecated(since = "0.7.0", ...)]` rather than removed, since it is still live upstream.

- RFQ-scoped quote lookup: `GET /communications/rfqs/{rfq_id}/quotes/{quote_id}` (and the sibling
  `DELETE`/`accept`/`confirm` endpoints) are the current, non-deprecated way to operate on a quote.
  The unscoped `/communications/quotes/{quote_id}` family (`GetQuote`, `DeleteQuote`, `AcceptQuote`,
  `ConfirmQuote`) is marked `deprecated: true` in the OpenAPI spec as of 2026-07-07. The crate adds
  `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote` and marks the
  existing unscoped `get_quote` / `delete_quote` / `accept_quote` / `confirm_quote` methods
  `#[deprecated(since = "0.7.0", ...)]` rather than removing them, since they are still live
  upstream (deprecated, not removed).

- `price_level_structure` (`Market.price_level_structure`, `WsMarketLifecycleV2.price_level_structure`)
  remains a raw `Option<String>` rather than a closed enum. Seven new values were added to the
  OpenAPI/AsyncAPI enum in 2026-07 ("New price level structures," pilot rollout starting the week of
  2026-07-27), and modeling this as a string means no crate update is needed as the rollout expands.
  `WsMarketLifecycleV2`/`Ref` gained a typed `price_ranges: Option<Vec<WsPriceRange>>` /
  `Option<Vec<WsPriceRangeRef>>` field (previously only reachable via the `extra` catch-all),
  mirroring the REST `Market.price_ranges: Option<Vec<PriceRange>>` field added in the prior release.
  It is `Option` because the AsyncAPI marks it present only on `created` and
  `price_level_structure_updated` events.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
