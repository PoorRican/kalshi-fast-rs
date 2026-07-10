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

- `GET /exchange/announcements` was removed from the OpenAPI spec (2026-07-04). The crate no longer
  exposes `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, or `AnnouncementStatus`.
- `Market.response_price_units` and `Market.fractional_trading_enabled` were removed from the
  Predictions REST schema (2026-07-03); `MarketPosition.resting_orders_count` was removed at the
  same time. All three are removed from the crate's public types rather than kept as stale
  `Option` fields, since they no longer round-trip any upstream value. The WebSocket
  `market_lifecycle_v2` payload's `fractional_trading_enabled` field and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant were removed for the same reason
  (the `fractional_trading_updated` event type is no longer in the AsyncAPI `event_type` enum).
- A multi-index exchange rollout added `exchange_index` (integer, defaults to `0`) to several
  response shapes: `Market`, `SubaccountBalance`, and the new `ExchangeIndexStatus` (part of
  `GetExchangeStatusResponse.exchange_index_statuses`). `GetExchangeStatusResponse` also gained
  `intra_exchange_transfers_active`. All are modeled as `Option` since the per-index breakdown is
  absent on older/unaffected responses.
- The `market_lifecycle_v2` `created` and `price_level_structure_updated` events now carry a
  `price_ranges` array (start/end/step, fixed-point dollars) alongside `price_level_structure`.
  Modeled as `Option<Vec<PriceRange>>` reusing the REST `PriceRange` type (owned path) and a
  zero-copy `WsPriceRangeRef` (borrowed path), consistent with the WebSocket module's zero-copy
  parsing goal. `price_level_structure` itself stays a raw `String`/`Cow<str>` so the seven new
  price-level-structure values introduced 2026-07-07 round-trip with no crate change.
- `GET /multivariate_event_collections/{collection_ticker}/lookup` (the lookup-history feed) was
  fully removed from the OpenAPI spec (2026-07-02, "fully deprecated"). The crate no longer exposes
  `get_multivariate_event_collection_lookup_history` or its `Params`/`Response`/`LookupPoint`
  types. The sibling `PUT` endpoint (`lookup_tickers_for_market_in_multivariate_event_collection`)
  is still present upstream but now marked `deprecated: true` ("predates RFQs, do not use for new
  integrations", no replacement) — kept in the crate but annotated `#[deprecated]`.
- The quote-ID-only endpoints (`GET`/`DELETE`/`PUT .../accept`/`PUT .../confirm` under
  `/communications/quotes/{quote_id}`) were all marked `deprecated: true` upstream (2026-07-07) in
  favor of RFQ-scoped equivalents under `/communications/rfqs/{rfq_id}/quotes/{quote_id}`. The
  crate keeps `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote` (annotated `#[deprecated]`)
  and adds `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote` as their
  replacements; both sets return the same response types.
- Margin endpoints beyond `/margin/fee_tiers` (`/margin/risk`, `/margin/positions`,
  `/margin/orders`) remain unmodeled, so upstream margin-only changes (per-market risk metric
  gating, `margin_used` omission for jointly-margined positions, `is_portfolio` flag,
  `order_reason` on system orders) require no crate change.
- FIX protocol behavior (`AcceptQuote` reject reasons, FIX tag 2446 on Incremental Refresh) is out
  of scope: this crate only implements the REST and WebSocket JSON APIs, not FIX.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
