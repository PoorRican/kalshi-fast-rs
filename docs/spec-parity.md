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

- `GET /exchange/announcements` was removed from the OpenAPI spec (2026-07-04). The crate's
  `get_exchange_announcements` method and its `Announcement`/`AnnouncementType`/`AnnouncementStatus`/
  `GetExchangeAnnouncementsResponse` types were removed accordingly.

- The multivariate lookup-history endpoint (`GET /multivariate_event_collections/{ticker}/lookup`)
  was fully removed from the OpenAPI spec (2026-07-02); only the `PUT` ticker-lookup endpoint at the
  same path remains. The crate's `get_multivariate_event_collection_lookup_history` method and its
  `GetMultivariateEventCollectionLookupHistoryParams`/`Response`/`LookupPoint` types were removed.

- `Market.response_price_units` and `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count`, were removed from the OpenAPI schema (2026-07-09) and
  dropped from the crate's `Market`/`MarketPosition` structs (breaking).

- The `fractional_trading_updated` event on the `market_lifecycle_v2` WebSocket channel, and the
  top-level `fractional_trading_enabled` field on `WsMarketLifecycleV2`, were removed from the
  AsyncAPI spec on 2026-04-17 (fractional trading is now unconditional) but were never previously
  reconciled in this crate. This refresh removes
  `WsMarketLifecycleEventType::FractionalTradingUpdated` and `WsMarketLifecycleV2.fractional_trading_enabled`
  to match the current spec (breaking). Found via spec grep, not the recent changelog window — the
  YAML is authoritative for shape even when the triggering changelog entry predates the tracked
  watermark.

- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were dropped from the OpenAPI spec
  entirely (2026-06-18 to 2026-06-25 deprecation window), while the Kalshi changelog describes them
  as merely deprecated — calls still succeed but the exchange nudges callers toward the V2 endpoints.
  Per the deprecated-but-still-present convention, these methods are kept and marked `#[deprecated]`
  (pointing at their `*_v2` replacements) rather than removed, since removal would sever a still-
  working path with no upstream-mandated removal date.

- Communications RFQ/quote endpoints were restructured to be RFQ-scoped (2026-06-25 / 2026-07-09):
  `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, and `confirm_rfq_quote` take both
  `rfq_id` and `quote_id`. The quote-ID-only endpoints (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) remain supported but are marked `#[deprecated]`. Quotes are no longer guaranteed
  queryable unless in a post-acceptance state (`accepted`/`confirmed`/`executed`); callers should not
  treat open/cancelled quotes as durable records.

- `GetQuotesParams` dropped the `market_ticker`/`event_ticker` filters (removed upstream 2026-06-20,
  breaking) and gained `min_ts`/`max_ts`/`user_filter` (added 2026-06-18).

- `GetEventsParams` gained `tickers` (comma-separated event tickers, added 2026-06-18) and
  `min_updated_ts` (added 2026-07, found via spec grep — poll for event metadata changes
  efficiently). Unlike `GetMarketsParams.min_updated_ts`, the OpenAPI spec states no mutual-exclusion
  constraint for the events variant, so no extra validation was added.

- `exchange_index` (defaults to `0`; only `0` is currently live) was added to `Market`, `EventData`,
  `SubaccountBalance` (required), and `GetExchangeStatusResponse` (via the new
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` array plus `intra_exchange_transfers_active`),
  as part of an in-progress multi-exchange-index rollout (2026-06/07). All are `Option` except
  `SubaccountBalance.exchange_index`, which the OpenAPI spec marks required for that response.

- API keys can now be restricted to a single sub-account (2026-07-02): `subaccount: Option<u32>` was
  added to `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest`.

- Two new account endpoints were added: `get_account_api_usage_level_volume_progress` (trailing
  30-day volume progress toward volume-based tiers) and `upgrade_account_api_usage_level`
  (self-serve Advanced-tier upgrade, added 2026-06-11).

- `pyth_value` is a new authenticated AsyncAPI channel (added 2026-07-23) delivering deduplicated
  Pyth price updates by underlying ticker, modeled the same way as `cfbenchmarks_value`:
  `underlying_tickers` seeds the initial subscribe (`["all"]` for every underlying), and
  `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` manage the
  subscription via `update_subscription_v2`. Unlike `cfbenchmarks_value`, this channel requires
  authentication (`WsChannelV2::PythValue.is_private()` is `true`).

- `market_lifecycle_v2` gained `price_ranges` (on `created`/`price_level_structure_updated` events,
  added 2026-07-02) and top-level `strike_type`/`cap_strike`/`custom_strike` on `metadata_updated`
  events (added 2026-06-18), alongside the pre-existing top-level `floor_strike`/`yes_sub_title`.
  `price_ranges` reuses the REST `PriceRange` type since the shapes are identical.

- Seven new `price_level_structure` values were introduced (2026-07-23, rolling out through early
  August). No crate change was needed: the crate already models `price_level_structure` as a plain
  `String` (both on `Market` and on `market_lifecycle_v2` messages) rather than a closed enum, so new
  values pass through without a release.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
