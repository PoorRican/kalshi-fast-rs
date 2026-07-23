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

- `Market` and `EventData` had accumulated a large amount of legacy REST cruft (integer cent
  fields, `market_id`/`series_id`/`event_id`, non-fixed-point counts, per-event lifecycle
  timestamps, etc.) that predates this refresh and was never removed even though the current
  OpenAPI schema had already dropped it. This refresh removed all of it in one pass rather than
  carrying it forward as a silent compatibility shim, alongside the changelog-flagged removals
  (`Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`, 2026-07-09). `EventData.milestones` was also removed —
  `with_milestones=true` returns milestones as a response-level sibling array
  (`GetEventsResponse.milestones`), never nested per event.
- Fields the OpenAPI spec still marks `deprecated: true` but keeps present (`Market.title`,
  `Market.subtitle`, `Market.expiration_time`, `Market.liquidity_dollars` — always `"0.0000"` —
  and `EventData.category`) are kept as `Option` fields annotated with Rust's `#[deprecated]`
  attribute, so `cargo build` surfaces a compiler warning at every read site without breaking
  parsing of payloads that still send them.
- `WsMarketLifecycleV2` / `WsMarketLifecycleV2Ref`: `fractional_trading_enabled` and the
  `FractionalTradingUpdated` event-type variant were removed — neither appears in the AsyncAPI
  `market_lifecycle_v2` schema anymore. The `metadata_updated` event gained top-level
  `strike_type` / `cap_strike` / `custom_strike` (2026-06-18), and `created` /
  `price_level_structure_updated` gained `price_ranges` (2026-07-02, same `PriceRange` shape as
  the REST `Market.price_ranges`).
- `price_level_structure` (REST `Market` and the WS `market_lifecycle_v2` payload) is kept as an
  untyped `String` rather than an enum, so the seven new `center_*_edge_*_cent` values added
  2026-07-23 need no crate change — the source of truth for valid prices is always `price_ranges`.
- `GET /communications/quotes`: `market_ticker` / `event_ticker` filters were removed 2026-06-20
  (breaking); `min_ts` / `max_ts` were added 2026-06-18. RFQ-scoped quote action/lookup endpoints
  (`.../rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`) were added 2026-06-25 / 2026-07-09 as
  `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`. The original
  quote-ID-only methods (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) remain but
  are documented as deprecated: quotes that haven't reached a post-acceptance state are no longer
  guaranteed queryable through them.
- `GET /exchange/announcements` was removed from the OpenAPI spec 2026-07-04. `get_exchange_announcements`
  and the `Announcement`/`AnnouncementType`/`AnnouncementStatus`/`GetExchangeAnnouncementsResponse`
  types were removed rather than kept as a dead code path.
- The multivariate lookup-history feed (`GET /multivariate_event_collections/{ticker}/lookup`) was
  fully removed from the OpenAPI spec 2026-07-02 (only the deprecated `PUT` ticker-pair lookup
  remains). `get_multivariate_event_collection_lookup_history` and its
  `GetMultivariateEventCollectionLookupHistoryParams` / `...Response` / `LookupPoint` types were removed.
- `exchange_index` (defaults to `0`, multi-index rollout) was added as an `Option<i64>` /
  `i64` field on `Market`, `EventData`, `GetExchangeStatusResponse` (plus the new
  `ExchangeIndexStatus.exchange_index`), and `SubaccountBalance` — the latter is now `i64`
  (required per spec) since `GET /portfolio/subaccounts/balances` returns one entry per
  exchange index per subaccount as of 2026-07-02, instead of one entry per subaccount.
- `pyth_value` is a new authenticated-only AsyncAPI channel (2026-07-23) that mirrors the
  `cfbenchmarks_value` pattern: seed `underlying_tickers` on subscribe (`["all"]` for every
  underlying), and use `update_subscription` with `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` (mirroring `SubscribeIndices` / `UnsubscribeIndices`
  / `Indexlist`) to manage it afterward. Modeled as `WsPythValue` / `WsPythUnderlyingList`, routed
  through `WsDataMessageV2`. Unlike `cfbenchmarks_value`, `pyth_value` requires authentication, so
  `WsChannelV2::PythValue` is included in `is_private()`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
