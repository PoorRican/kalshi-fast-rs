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

- `service` on `ErrorResponse` was deprecated by Kalshi on 2026-07-28 and removed from all REST
  error responses on 2026-08-06. The field is kept (as `Option<String>`, unchanged) but marked
  `#[deprecated]` so downstream code branching on it gets a compiler warning; branch on `code`
  instead, which is present on every error response.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST API schema
  (2026-07-09) and are no longer present in the OpenAPI spec at all. The corresponding Rust struct
  fields were removed (a breaking change — see `VERSIONING.md`). The WebSocket `market_positions`
  channel mirrors `MarketPosition`, so its (dead-code) `resting_orders_count` field was removed too.
  `GET /exchange/announcements` was removed the same window (2026-07-04); `get_exchange_announcements`
  and its supporting types (`Announcement`, `AnnouncementType`, `AnnouncementStatus`,
  `GetExchangeAnnouncementsResponse`) were deleted from the crate.

- `WsMarketLifecycleV2::fractional_trading_enabled` and the `FractionalTradingUpdated` event-type
  variant are marked `#[deprecated]` rather than removed: the AsyncAPI schema no longer documents a
  `fractional_trading_updated` event (fractional trading has fully rolled out), but removing a public
  enum variant/field outright would be a harsher break than necessary for a field that simply stops
  being emitted. `#[deprecated]` gives downstream code a compiler warning without breaking builds.

- `GetQuotesParams` (`GET /communications/quotes`) no longer supports `market_ticker` or
  `event_ticker` filters (removed 2026-06-20); those fields were removed from the Rust struct. The
  endpoint gained `min_ts`/`max_ts` (last-updated time window) and `user_filter` (2026-06-18/2026-06-23).

- RFQ-scoped quote endpoints (`GET`/`DELETE /communications/rfqs/{rfq_id}/quotes/{quote_id}`,
  `PUT .../accept`, `PUT .../confirm`) were added 2026-06-25 as the preferred replacement for the
  quote-ID-only endpoints (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`), which are
  now `#[deprecated]` but still call the still-live legacy paths. Likewise, the legacy
  `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated 2026-06-18 in favor
  of the V2 event-order endpoints already in the crate, and
  `get_multivariate_event_collection_lookup_history` / `lookup_tickers_for_market_in_multivariate_event_collection`
  are deprecated per the OpenAPI spec (the GET lookup-history operation was removed entirely; the PUT
  lookup operation predates RFQs and is marked `deprecated: true`). All of these are `#[deprecated]`
  attributes, not removals, so existing callers keep compiling with a warning.

- `exchange_index` (identifies an exchange shard; defaults to 0) was added across the OpenAPI/AsyncAPI
  specs in 2026-07 for multi-exchange-index rollout: `Market`, `EventData`, `Series`,
  `WsMarketLifecycleV2` (market creation only), `WsEventLifecycle`, `SubaccountBalance`,
  `GetExchangeStatusResponse` (via new `exchange_index_statuses: Vec<ExchangeIndexStatus>` plus
  top-level `intra_exchange_transfers_active`), and the order-group delete/reset/trigger/limit query
  params (via a new `exchange_index` field on `SubaccountQueryParams`). All modeled as `Option<i64>`
  (or `Option<u32>` for the query-param cases, matching the existing `subaccount` convention) even
  where the spec marks them required, since these are newly-added fields and older cached payloads
  may predate them.

- `PUT /portfolio/order_groups/{order_group_id}/limit` gained `subaccount`/`exchange_index` **query**
  parameters (2026-08-06), not body fields. `update_order_group_limit` now takes a
  `SubaccountQueryParams` argument in addition to the `UpdateOrderGroupLimitRequest` body — a
  breaking signature change.

- `Event product_metadata now includes cadence` (changelog, 2026-07-30) has no corresponding field in
  the OpenAPI `GetEventMetadataResponse` schema as of this reconciliation. No dedicated `cadence`
  field was added to `EventMetadata`; it is still captured losslessly via the existing
  `extra: Map<String, Value>` flatten field if the exchange sends it. Revisit once the OpenAPI schema
  catches up.

- `EventData.settlement_sources` (mirroring the field already on `Series`), `fee_type_override`, and
  `fee_multiplier_override` were added to match the OpenAPI `EventData` schema (2026-06-18 / ongoing).

- New `pyth_value` WebSocket channel (2026-07-23) follows the same pattern as `cfbenchmarks_value`:
  `WsChannelV2::PythValue`, `underlying_tickers: Option<Vec<String>>` subscription parameter (use
  `["all"]` for every underlying), and `WsUpdateAction::SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList` update actions (mutually exclusive with market targets,
  same validation shape as the CF Benchmarks index actions). Messages are `WsPythValue`
  (`pyth_value`) and `WsPythUnderlyingList` (`pyth_value_underlying_list`), routed through
  `WsDataMessageV2::PythValue` / `PythValueUnderlyingList`.

- New endpoints added: `GET /live_data/events/{event_ticker}` (`get_event_live_data`, 2026-07-30),
  `GET /historical/positions` (`get_historical_positions`, reuses `GetPositionsResponse`,
  2026-07-23), `POST /account/api_usage_level/upgrade` (`upgrade_account_api_usage_level`,
  2026-06-11), and `GET /account/api_usage_level/volume_progress`
  (`get_account_api_usage_level_volume_progress`, 2026-06-11).

- `ApiKey.subaccount`, and a `subaccount: Option<u32>` field on `CreateApiKeyRequest` /
  `GenerateApiKeyRequest`, were added 2026-07-02 for subaccount-restricted API keys. `WsQuoteCreated`
  gained a `subaccount: Option<u32>` field the same window (present only when your side of the quote
  used a subaccount).

- The seven new `price_level_structure` values introduced 2026-07-23 (plus
  `center_centi_edge_centi_cent` for combo markets, 2026-08-17) require no crate change:
  `Market.price_level_structure` and `WsMarketLifecycleV2.price_level_structure` are already plain
  `String`, and the source of truth for valid prices is the `price_ranges` array, already modeled.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
