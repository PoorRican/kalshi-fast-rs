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

- **Breaking removals (0.6.0 → 0.7.0).** Three deprecated `Market`/`MarketPosition` fields were
  removed from the OpenAPI schema on 2026-07-09 and are now removed from the crate:
  `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count` (both the REST type and the WS `MarketPositionRef`/
  `market_positions` mirror). `GET /exchange/announcements` was removed from the REST API on
  2026-07-04; `get_exchange_announcements()` and its types (`Announcement`, `AnnouncementType`,
  `AnnouncementStatus`, `GetExchangeAnnouncementsResponse`) were deleted. The multivariate lookup
  surface was removed on 2026-08-06: `PUT /multivariate_event_collections/{ticker}/lookup` and
  `GET .../lookup` (history) are gone from the OpenAPI paths, so
  `lookup_tickers_for_market_in_multivariate_event_collection` and
  `get_multivariate_event_collection_lookup_history` (plus their request/response types) were
  deleted; the `multivariate` WebSocket channel and `multivariate_lookup` message type were
  likewise removed from the AsyncAPI spec and deleted from the crate (`WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`/`MultivariateLookup`, `WsMultivariate*` types). `GET
  /communications/quotes` no longer accepts `market_ticker`/`event_ticker` (removed 2026-06-20);
  those fields were removed from `GetQuotesParams`. `PUT /portfolio/order_groups/{id}/limit` gained
  `subaccount`/`exchange_index` query params (2026-08-06), so
  `update_order_group_limit` now takes a `SubaccountQueryParams` argument.

- **New account/API-key endpoints.** `GET /account/api_usage_level/volume_progress` (2026-06-11)
  and `POST /account/api_usage_level/upgrade` (2026-06-11) are modeled as
  `get_account_api_usage_level_volume_progress()` / `upgrade_account_api_usage_level()`.
  `CreateApiKeyRequest`, `GenerateApiKeyRequest`, and `ApiKey` gained a `subaccount: Option<u32>`
  field (2026-07-02) for sub-account-restricted API keys. `SubaccountBalance` gained a required
  `exchange_index: i64` field (2026-07-02): balances are now reported per exchange index instead of
  one combined row per subaccount.

- **`ErrorResponse.service` is deprecated.** Kalshi removed the `service` field from error response
  bodies on 2026-08-06 (deprecated 2026-07-28). The field is `#[deprecated]` and stays
  `Option<String>` (always `None` on current responses) so older cached/logged payloads that still
  have it continue to parse without a breaking removal.

- **`GetEventsParams.tickers`**, **`EventData.settlement_sources`** (mirrors the field already on
  `Series`), **`Series.exchange_index`**, and **`MultivariateEventCollection.exchange_index`** were
  added as new optional REST fields (2026-06-18 / 2026-07-30 / 2026-08-06). `GetExchangeStatusResponse`
  gained `intra_exchange_transfers_active: Option<bool>` and `exchange_index_statuses:
  Vec<ExchangeIndexStatus>` (2026-07-02) for the per-index exchange-status breakdown.

- **RFQ-scoped quote actions.** Kalshi added `rfq_id`-scoped quote endpoints
  (`GET`/`DELETE /communications/rfqs/{rfq_id}/quotes/{quote_id}`, `PUT .../accept`, `PUT
  .../confirm`) between 2026-06-25 and 2026-07-09, deprecating the quote-ID-only equivalents. The
  crate adds `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote` and marks
  `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote` `#[deprecated]` (still functional).
  `GetQuotesParams` also gained `min_ts`/`max_ts`/`user_filter` (2026-06-18) quote time filters.

- **Legacy V1 order mutation endpoints.** `POST/DELETE/PUT /portfolio/orders*` mutation operations
  (`create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders`) were removed from the OpenAPI paths entirely (deprecation announced
  2026-06-18; the paths are simply absent from the current spec). The crate marks all six
  `#[deprecated]` pointing at their V2 (`_v2`) equivalents rather than deleting them outright, since
  removal is a larger breaking change than deprecation; downstream users should migrate to the V2
  event-market order endpoints. `examples/place_order.rs` was updated to use `create_order_v2`.

- **New endpoints.** `GET /historical/positions` (2026-07-23, `get_historical_positions`, reuses
  `portfolio::GetPositionsResponse`) and `GET /live_data/events/{event_ticker}` (2026-07-30,
  `get_event_live_data`, models `EventLiveData` with a `serde_json::Value`-style `details` map per
  this crate's flexible-payload convention) were added. `POST
  /portfolio/intra_exchange_instance_transfer` plus its `GET` history/detail endpoints
  (`intra_exchange_instance_transfer`, `get_intra_exchange_instance_transfers[_all]`,
  `get_intra_exchange_instance_transfer`) were added even though the changelog dates them
  2026-08-13 (after this refresh's "today"), because the endpoints and schemas are already fully
  present in the live OpenAPI spec.

- **`pyth_value` WebSocket channel** (added 2026-07-23) delivers real-time Pyth price updates,
  modeled the same way as `cfbenchmarks_value`: `WsChannelV2::PythValue` (private/auth-required),
  `WsPythValue`/`WsPythUnderlyingList` message types, and `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` update actions gated by a new `underlying_tickers`
  field on `WsSubscriptionParamsV2`/`WsUpdateSubscriptionParamsV2`.

- **`market_lifecycle_v2` / `event_lifecycle` additions.** `WsMarketLifecycleV2` gained top-level
  `strike_type`/`cap_strike`/`custom_strike` (present only on `metadata_updated` events,
  2026-06-18), `price_ranges: Option<Vec<WsPriceRange>>` (present alongside `price_level_structure`
  on `created`/`price_level_structure_updated` events, 2026-07-02), and `exchange_index` (present
  only on market creation, 2026-07-30). `WsEventLifecycle` gained `exchange_index` (required by the
  spec on every `event_lifecycle` message, but kept `Option` per this crate's established
  tolerate-anything convention). `WsQuoteCreated` gained `subaccount: Option<i64>` (2026-07-30).
  `price_level_structure` remains an untyped `String` on both REST `Market` and the WS payload (no
  Rust enum), so the seven new structure values added 2026-07-23 plus the future
  `center_centi_edge_centi_cent` value (announced 2026-08-17, not yet effective) require no code
  change.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
