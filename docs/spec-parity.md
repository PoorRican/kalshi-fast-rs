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
  `MarketPosition.resting_orders_count` were removed outright from the OpenAPI schema on 2026-07-09
  (not merely deprecated), so the corresponding Rust fields were removed rather than kept as dead
  `Option`s. Downstream code that read these fields will not compile against 0.7.0.
- The `market_lifecycle_v2` AsyncAPI message dropped the `fractional_trading_enabled` field and the
  `fractional_trading_updated` `event_type` value entirely (no longer present in the 2026-07 AsyncAPI
  snapshot). `WsMarketLifecycleV2::fractional_trading_enabled` and
  `WsMarketLifecycleEventType::FractionalTradingUpdated` were removed for the same reason as above.
- `market_lifecycle_v2` gained a `price_ranges` array (reusing the REST `PriceRange` shape) emitted
  alongside `price_level_structure` on `created` and `price_level_structure_updated` events
  (2026-07-02), and top-level `strike_type` / `cap_strike` / `custom_strike` fields on
  `metadata_updated` events alongside the existing top-level `floor_strike` (2026-06-18). All are
  modeled as `Option` on `WsMarketLifecycleV2` since they only appear on specific event types.
- Seven new `price_level_structure` enum values were added upstream in 2026-07 (finer tick sizes,
  pilot rollout starting the week of 2026-07-27). No crate change was needed: `price_level_structure`
  is already modeled as a raw `String` on both the REST `Market` and the `market_lifecycle_v2` message,
  so new values round-trip losslessly.
- `GET /communications/quotes` (`GetQuotesParams`) dropped the `market_ticker` and `event_ticker`
  query parameters on 2026-06-20 with no replacement; they were removed from the crate (breaking).
  `min_ts` / `max_ts` (2026-06-18) and `user_filter` were added to match the current parameter set.
  `quote_creator_user_id` and `rfq_creator_user_id` remain present upstream but are marked
  `deprecated: true` in the OpenAPI spec with no replacement, so the corresponding struct fields carry
  `#[deprecated]`.
- RFQ-scoped quote lookup/action endpoints (`GET`/`DELETE`/`PUT .../rfqs/{rfq_id}/quotes/{quote_id}`)
  were added 2026-06-25 (actions) and 2026-07-09 (lookup) as the preferred replacements for the
  quote-ID-only endpoints. `get_quote`, `delete_quote`, `accept_quote`, and `confirm_quote` remain
  functional but are marked `#[deprecated]` per the OpenAPI spec's own `deprecated: true` flag; use
  `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote` instead.
- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated by Kalshi between
  2026-06-18 and 2026-06-25 and no longer appear in the OpenAPI spec at all (the paths were removed,
  not merely flagged). The corresponding Rust methods are marked `#[deprecated]` rather than removed,
  to preserve source compatibility for callers who have not yet migrated; calling them now returns a
  404. Use the `*_v2` event-order methods instead (added in 0.6.0).
- `GET /exchange/announcements` was removed from the OpenAPI spec on 2026-07-04.
  `get_exchange_announcements` is marked `#[deprecated]` (it now always 404s) rather than removed, for
  the same source-compatibility reason as the legacy order endpoints. Use `get_exchange_schedule` for
  operating hours.
- Multivariate lookup history endpoints were fully deprecated on 2026-07-02. The `GET
  /multivariate_event_collections/{collection_ticker}/lookup` route (lookup history) was removed
  entirely from the spec; `get_multivariate_event_collection_lookup_history` is marked `#[deprecated]`
  (always 404s). The `PUT` variant at the same path (`lookup_tickers_for_market_in_multivariate_event_collection`)
  remains present upstream but is now marked `deprecated: true` in the spec ("predates RFQs"), so it
  is marked `#[deprecated]` too, without removal.
- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (2026-07-02), reflecting a per-exchange-index
  breakdown; the existing top-level fields continue to reflect the default index (0).
- `SubaccountBalance` was missing `voluntarily_locked`, `settlement_advance_state`, and
  `settlement_advance` (all required or optional in the current OpenAPI `SubaccountBalance` schema)
  even before this refresh; they were added alongside the new `exchange_index: i64` field
  (2026-07-02, "one balance per exchange index") while reconciling the struct against the full
  `required` list.
- `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest` gained `subaccount: Option<u32>`
  (2026-07-02) for sub-account-restricted API keys (0-63). Absent/null means the key is unrestricted.
- `GET /historical/positions` (`get_historical_positions`, 2026-07-23) returns the same
  `GetPositionsResponse` shape as `GET /portfolio/positions`; positions are archived per whole event
  and only settled positions older than the new `market_positions_last_updated_ts` cutoff on
  `GET /historical/cutoff` are available through it.
- `GET /events` gained a `tickers` query parameter (2026-06-18) and `EventData` gained
  `settlement_sources: Option<Vec<SettlementSource>>` (2026-06-18), mirroring the field already
  available on `Series`.
- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade`
  (2026-06-11) are modeled as `get_account_api_usage_level_volume_progress` and
  `upgrade_account_api_usage_level`; the upgrade endpoint returns an empty 201 body, modeled as
  `EmptyResponse`.
- The `pyth_value` WebSocket channel (2026-07-13, authenticated) is modeled the same way as
  `cfbenchmarks_value`: `WsChannelV2::PythValue` (added to `is_private()`), an
  `underlying_tickers: Option<Vec<String>>` field on `WsSubscriptionParamsV2` (use `["all"]` for every
  underlying, or omit and discover via `underlying_list`), `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` variants plus a matching `underlying_tickers` field on
  `WsUpdateSubscriptionParamsV2`, and `WsPythValue` / `WsPythUnderlyingList` message types routed
  through both the wire and envelope parse paths. `validate_update` rejects mixing underlying actions
  with market targets and requires `underlying_tickers` for the add/remove actions, matching the
  AsyncAPI error semantics for `cfbenchmarks_value`.
- Margin/perps endpoints and fields (fee tiers, perps mark prices/volume/OI notionals, margin
  positions `is_portfolio` / `subaccount` / jointly-margined `margin_used` omission, margin risk
  per-market metric restrictions, margin order `order_reason`) are intentionally not modeled: this
  crate does not implement the Margin exchange market types. Each such changelog entry is a
  documented no-op rather than an oversight.
- FIX protocol changes (FIX Tag 2446, RFQ quote identity/post-only/exchange-index routing, FIX
  reject-reason improvements, subaccount-restricted FIX sessions) are intentionally not modeled: this
  crate is a REST/WebSocket adapter only and does not implement FIX.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
