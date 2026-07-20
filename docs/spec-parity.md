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
  `MarketPosition.resting_orders_count` (and its WS mirror `MarketPositionRef.resting_orders_count`)
  were removed from the OpenAPI schema on 2026-07-09. They are kept as `#[deprecated]` `Option`
  fields — the exchange no longer populates them, but older cached payloads and downstream code that
  still reads them continue to compile and parse without error.

- `GetQuotesParams.market_ticker`/`.event_ticker` are `#[deprecated]` and no longer filter results:
  the server stopped honoring them on 2026-06-20 (`GET /communications/quotes` requests should
  filter by user, RFQ, status, or update time instead). The fields are kept, not removed, so
  existing call sites still compile.

- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) are `#[deprecated]`. Their
  operationIds were removed from the OpenAPI spec between 2026-06-18 and 2026-06-25 in favor of the
  V2 event-order endpoints (`create_order_v2`, `cancel_order_v2`, etc., under
  `/portfolio/events/orders/*`). The read-only `get_orders`/`get_order` methods are unaffected — the
  OpenAPI spec still documents `GetOrders`/`GetOrder` on `/portfolio/orders`.

- The quote-ID-only communications endpoints (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) are `#[deprecated]` in favor of the RFQ-scoped variants (`get_rfq_quote`,
  `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`), added 2026-06-25. The OpenAPI spec
  marks the quote-ID-only paths deprecated but still routable.

- `lookup_tickers_for_market_in_multivariate_event_collection` (`PUT
  /multivariate_event_collections/{ticker}/lookup`) is `#[deprecated]` — the OpenAPI spec marks it
  deprecated as predating RFQs. Its sibling GET lookup-history endpoint was fully removed from the
  spec (2026-07-02) and the corresponding Rust method/types
  (`get_multivariate_event_collection_lookup_history`,
  `GetMultivariateEventCollectionLookupHistoryParams`/`Response`, `LookupPoint`) were deleted
  outright rather than deprecated, since calling a nonexistent endpoint has no useful fallback
  behavior.

- `GET /exchange/announcements` was removed from the Predictions REST API on 2026-07-04.
  `get_exchange_announcements()` and its response types (`GetExchangeAnnouncementsResponse`,
  `Announcement`, `AnnouncementType`, `AnnouncementStatus`) were deleted rather than deprecated, for
  the same reason as the multivariate lookup-history removal above.

- `SubaccountBalance.exchange_index` is `Option<i32>` even though the OpenAPI spec marks it
  required (2026-07-02: a subaccount with funds on multiple exchange indexes now returns one row
  per index instead of one combined row). `Option` tolerates payloads captured before the field
  existed.

- `price_level_structure` (on both the REST `Market` object and the WS `market_lifecycle_v2`
  `created`/`price_level_structure_updated` events) is modeled as an untyped `String` rather than an
  enum. The AsyncAPI spec added seven new values in 2026-07 (`center_*_edge_*_cent` variants
  describing finer tick-size bands); the OpenAPI spec does not constrain the field to an enum at
  all. Consumers should treat `price_ranges` (the `{start, end, step}` bands) as the source of truth
  for a market's valid prices rather than branching on the `price_level_structure` label.

- The `pyth_value` WebSocket channel requires an authenticated connection (`WsChannelV2::PythValue`
  returns `true` from `is_private()`), unlike the public `cfbenchmarks_value` channel it otherwise
  mirrors. Subscription/update parameters use `underlying_tickers` (not `index_ids`), and the
  update-subscription actions are `subscribe_underlyings` / `unsubscribe_underlyings` /
  `underlying_list` (not `indexlist`) — modeled as separate `WsUpdateAction` variants with their own
  `is_underlying_action()` validation, parallel to but independent from the CF Benchmarks index
  actions.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
