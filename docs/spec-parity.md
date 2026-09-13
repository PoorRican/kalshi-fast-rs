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

- The following fields were removed from the live OpenAPI/AsyncAPI schemas and are no longer
  modeled (removed, not converted to `Option`, per the refresh policy of not preserving removed
  upstream fields): `Market.response_price_units`, `Market.fractional_trading_enabled` and the
  matching `market_lifecycle_v2` field, `MarketPosition.resting_orders_count`,
  `EventData.available_on_brokers` (deprecated 2026-08-27, removed 2026-09-10), and
  `ErrorResponse.service` (deprecated 2026-07-28, removed 2026-08-06). Downstream code accessing
  these fields will no longer compile (0.7.0 → 0.8.0, breaking).

- `GET /trade-api/v2/exchange/announcements` was removed 2026-07-04 with no replacement; the crate
  dropped `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus`.

- `PUT .../multivariate_event_collections/{ticker}/lookup` and the `GET .../lookup` history feed
  were removed 2026-08-06 (predated RFQs). The crate dropped
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types. Use
  `create_market_in_multivariate_event_collection` or the communications (RFQ) APIs instead.

- The WebSocket `multivariate` channel (message type `multivariate_lookup`) was removed 2026-08-06;
  subscriptions to it now return an unknown-channel error. The crate dropped
  `WsChannelV2::Multivariate`, the `multivariate`/`multivariate_lookup` `WsMsgType` variants, and
  `WsMultivariate`/`WsMultivariateRef`. This is distinct from the `multivariate_market_lifecycle`
  channel, which is unaffected and continues to reuse `WsMarketLifecycleV2` (it shares the same
  `event_type`/`market_ticker`/`additional_metadata` shape as `market_lifecycle_v2`, minus
  `metadata_updated` support).

- `exchange_index` (identifying an exchange shard under Kalshi's 2026-07/09 sharding rollout) was
  added across the REST and WebSocket surface: `Market`, `EventData`, `Series`,
  `MultivariateEventCollection`, `MarketPosition`, `Settlement`, `Fill`, `WsFill`,
  `SubaccountBalance`, `ApiKey.subaccount`, `WsMarketLifecycleV2` (`market_lifecycle_v2` /
  `multivariate_market_lifecycle`), `WsEventLifecycle`, and `WsUserOrder`. All are modeled as
  `Option<i64>` even where the spec marks them required, matching the crate's general resilience
  policy. `GET /portfolio/balance` gained `GetBalanceParams { exchange_index, subaccount }` and
  `GetBalanceResponse.balance_breakdown: Option<Vec<IndexedBalance>>`;
  `GetPortfolioRestingOrderTotalValueResponse` gained the matching
  `resting_order_value_breakdown`. `GetPositionsParams`, `GetFillsParams`, `GetSettlementsParams`,
  and `GetOrdersParams` all gained an `exchange_index` filter. `GetExchangeStatusResponse` gained
  `intra_exchange_transfers_active` and `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>`.

- `WsMarketLifecycleV2` gained top-level `strike_type`, `cap_strike`, and `custom_strike` fields
  (2026-06-18), alongside the pre-existing `floor_strike` / `yes_sub_title`: all five are present
  only on `metadata_updated` events per the AsyncAPI description, distinct from the same-named
  fields nested under `additional_metadata` (emitted only on `created`). It also gained
  `price_ranges: Option<Vec<PriceRange>>` (2026-07-02, reusing `crate::rest::PriceRange`), emitted
  alongside `price_level_structure` on `created` and `price_level_structure_updated` events.

- Seven new `price_level_structure` values (`center_whole_edge_half_cent`, …,
  `center_centi_edge_centi_cent`, `center_deci_edge_centi_cent`) were introduced between
  2026-07-23 and 2026-09-03 for sub-cent tick sizes on standard and combo markets. No crate change
  was needed: `Market.price_level_structure` and the WebSocket equivalent are already modeled as
  raw `Option<String>`, not a closed enum, so new values pass through losslessly. Per Kalshi's own
  guidance, consumers should key off `price_ranges` rather than the structure name.

- `FeeType` gained `QuadraticWithComboMakerFees` (serialized `quadratic_with_combo_maker_fees`),
  the combo-market maker-fee structure using a 0.5 multiplier instead of 0.25. The existing
  `#[serde(other)] Unknown` catch-all already tolerated this value before the explicit variant was
  added; the explicit variant is for ergonomics/parity with `QuadraticWithMakerFees`.

- `GET /communications/quotes` dropped its `market_ticker` and `event_ticker` filters 2026-06-20
  (breaking: removed from `GetQuotesParams`); filter by `rfq_id`, user, status, or update time
  instead. It gained `min_ts`/`max_ts` (2026-06-18) and `user_filter` (present alongside the older
  `rfq_user_filter`).

- RFQ-scoped quote action endpoints (`GET`/`DELETE`/`PUT .../rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`)
  were added 2026-06-25 as `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, and
  `confirm_rfq_quote`. The quote-ID-only equivalents (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) remain for now but are marked `#[deprecated]`, matching Kalshi's own
  deprecation of those endpoints; quotes not yet in a post-acceptance state are no longer
  guaranteed queryable without the RFQ ID.

- `CreateRFQRequest` gained `target_cost_excludes_fees: Option<bool>` (2026-09-10): when `true`,
  quote sizing treats the target cost as principal-only rather than fee-inclusive. Only valid
  together with a target cost.

- `cancel_all_orders` (`DELETE /portfolio/events/orders`, added 2026-08-27) cancels every resting
  V2 order, optionally scoped to one subaccount via the existing `SubaccountQueryParams`.

- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade`
  (added 2026-06-11) are modeled as `get_account_api_usage_level_volume_progress` and
  `upgrade_account_api_usage_level`.

- `EventMetadata.cadence: Option<String>` (2026-07-30) is not present in the currently-published
  manually-maintained OpenAPI schema — `EventData.product_metadata` there is typed as an opaque
  `object` rather than `$ref`-ing `GetEventMetadataResponse` — but is documented with a concrete
  shape in the Kalshi changelog, so it is modeled per the changelog per the task's own
  "changelog for timing/intent, YAML for shape when present" policy.

## Known Gaps (Not Modeled This Refresh)

The following upstream additions since the prior watermark are real, documented capabilities that
are not yet modeled in this crate. They were deliberately deferred rather than rushed; each is a
plausible target for a future refresh:

- `cfbenchmarks_value_5hz` WebSocket channel (5Hz CF Benchmarks feed, 2026-09-03).
- `pyth_value` WebSocket channel (deduplicated Pyth prices by underlying, 2026-07-23).
- `GET /historical/positions` (archived settled positions, 2026-07-23).
- `GET`/`POST /portfolio/target_balance_allocation` (2026-08-20, extended 2026-09-03/17).
- `GET /portfolio/intra_exchange_instance_transfers[/{id}]` (2026-08-13).
- `GET /live_data/events/{event_ticker}`, `GET /live_data/weather/{city}`, and
  `GET /live_data/weather/{city}/calibrations` (2026-07-30 / 2026-08-20 / 2026-08-31).
- FCM `GET /fcm/orders` `client_order_ids` filter (2026-09-03) — FCM subtrader endpoints are not
  modeled in this crate.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
