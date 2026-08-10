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

- The legacy `/portfolio/orders` mutation endpoints (create, cancel, amend, decrease, and the
  `/portfolio/orders/batched` batch endpoints) were removed from the OpenAPI spec entirely
  (deprecated 2026-06-18, confirmed removed by 2026-08-10). Only `GET /portfolio/orders` (list) and
  `GET /portfolio/orders/{order_id}` (single) remain. The corresponding Rust methods
  (`create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders`) and their request/response types were removed rather than deprecated,
  since calling them now fails against the live API. Use the V2 event-order methods
  (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead — these were already added in 0.6.0.
- `GET /exchange/announcements` was removed from the OpenAPI spec (2026-07-04); the exchange
  schedule (`get_exchange_schedule`) remains available. `get_exchange_announcements` and its
  `Announcement` / `AnnouncementType` / `AnnouncementStatus` types were removed.
- The `multivariate` WebSocket channel (message type `multivariate_lookup`) and the REST
  `/multivariate_event_collections/{collection_ticker}/lookup` endpoint (both `GET`, the lookup
  history feed, and `PUT`, ticker-pair resolution) were removed from the specs (2026-08-06). The
  corresponding Rust types and methods (`WsMultivariate`, `WsChannelV2::Multivariate`,
  `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types)
  were removed. Combo-market creation/resolution remains available via
  `create_market_in_multivariate_event_collection` (`POST
  /multivariate_event_collections/{collection_ticker}`); multivariate lifecycle notifications
  remain available via the still-present `multivariate_market_lifecycle` channel.
- `ErrorResponse.service` was removed (deprecated 2026-07-28, removed 2026-08-06); branch on `code`
  instead, which is present on every error response. A stray `service` key in an error body is still
  tolerated (ignored) rather than failing to parse, since `ErrorResponse` has no
  `deny_unknown_fields`.
- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema (2026-07-09). They were
  already `Option` in the crate, but since the fields are genuinely gone upstream (not just
  optional), they were removed from the public Rust API rather than kept as always-`None` fields.
  `Market.price_ranges` / the fixed-point count and dollar fields remain the canonical replacements.
- `EventData.category` is marked `deprecated: true` in the OpenAPI spec (2026-08-06, no removal
  date given yet); kept as `Option<String>` with a doc note. Prefer the series-level `category`.
- `MultivariateEventCollection.associated_event_tickers` is marked deprecated upstream in favor of
  `associated_events`; kept as `Vec<String>` with a doc note since the field is still populated.
- `pyth_value` is a new AsyncAPI channel (introduced 2026-07-23) that delivers deduplicated Pyth
  prices by underlying ticker. It mirrors the `cfbenchmarks_value` channel shape: `underlying_tickers`
  (not market tickers) seeds the initial subscription (`["all"]` for every underlying), and
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` manage it via
  `update_subscription_v2`. The channel emits `pyth_value` (`WsPythValue`: single-underlying tick)
  and `pyth_value_underlying_list` (`WsPythUnderlyingList`: recently-streamed underlyings, in
  response to the `underlying_list` action). `validate_update` mirrors the CF Benchmarks index-action
  validation for underlying actions.
- Several REST responses gained an `exchange_index` field identifying the exchange shard a resource
  lives on (`Series`, `EventData`, `MultivariateEventCollection`, `SubaccountBalance`) or shard-level
  status breakdown (`GetExchangeStatusResponse.exchange_index_statuses`, a new
  `ExchangeIndexStatus` struct). All exchange indices are currently `0` in production; the fields
  are modeled now so a future multi-shard rollout doesn't require another crate update.
  `WsMarketLifecycleV2.exchange_index` and `WsEventLifecycle.exchange_index` mirror this on the
  WebSocket side (2026-07-30). `WsEventLifecycle` also gained an `extra` flatten catch-all since it
  previously had none.
- `WsMarketLifecycleV2.price_ranges` (2026-07-02) reuses the REST `PriceRange` struct
  (`{start, end, step}` in fixed-point dollars) so consumers can read a market's valid-price grid
  directly from `created` / `price_level_structure_updated` events without a follow-up REST call.
- Multivariate (combo) markets are migrating to a new `center_centi_edge_centi_cent` price-level
  structure (2026-08-17, phased rollout) using the existing `*_dollars` fields at full precision; no
  crate change was needed since `Market.price_level_structure` / `WsMarketLifecycleV2.
  price_level_structure` are already untyped `String`, and the same is true of the seven
  `center_*_edge_*_cent` values added 2026-07-23.
- RFQ quote single-ID endpoints (`GET`/`DELETE`/`PUT .../accept`/`PUT .../confirm` under
  `/communications/quotes/{quote_id}`) are deprecated upstream in favor of RFQ-scoped equivalents
  under `/communications/rfqs/{rfq_id}/quotes/{quote_id}` (2026-06-25 for mutations, 2026-07-09 for
  the GET lookup). The crate keeps the old methods (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) marked `#[deprecated]` and adds the new ones (`get_rfq_quote`,
  `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`); RFQ quotes are not guaranteed
  queryable by quote ID alone once an RFQ rolls off a server, so new code should prefer the
  RFQ-scoped methods.
- `GET /communications/quotes` dropped its `market_ticker` / `event_ticker` filters (2026-06-20;
  filter by RFQ, status, or update time instead) and gained `min_ts` / `max_ts` (2026-06-18) plus a
  `user_filter` (self-only) alongside the existing `rfq_user_filter`. `GetQuotesParams` was updated
  to match; `quote_creator_user_id` and `rfq_creator_user_id` remain but are marked deprecated
  upstream (kept for compatibility, filtering by RFQ/status/time is preferred).

## Not Modeled

- Margin-exchange endpoints beyond `/margin/fee_tiers` (margin markets, positions, risk, order
  groups, perps market data) are out of scope for this crate; margin-tagged changelog entries are
  tracked in `CHANGELOG.md` release notes as "no code change" with a one-line justification rather
  than modeled here.
- FIX protocol behavior is entirely out of scope; this crate covers REST and WebSocket only per
  `CLAUDE.md`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
