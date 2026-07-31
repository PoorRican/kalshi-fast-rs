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

- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were removed from the OpenAPI spec
  between 2026-06-18 and 2026-06-25 (changelog: "Legacy order mutation endpoints deprecated"). The
  exchange now responds with `Please switch to the V2 endpoints`. The crate methods are kept but
  marked `#[deprecated]` pointing at their `_v2` equivalents rather than removed, since the routes
  may still function as an error-passthrough and removing a public method is a harder break than
  necessary. `examples/place_order.rs` and `tests/rest_orders.rs` were migrated to the V2 endpoints.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST API schema
  (changelog, 2026-07-09) and are no longer present in the OpenAPI spec. The corresponding Rust
  fields were removed (breaking, 0.6.0 → 0.7.0) since they no longer carry any signal from the
  exchange; `Market.price_level_structure`, `Market.price_ranges`, and the fixed-point count/dollar
  fields are the canonical replacements. The `fractional_trading_enabled` field on the
  `market_lifecycle_v2` WebSocket message (a separate AsyncAPI schema) is unaffected by the REST
  removal but is no longer present in the current AsyncAPI schema either; it is kept as `Option` for
  backward compatibility rather than removed, since WS messages are not scoped by this changelog
  entry.

- `GET /exchange/announcements` was removed from the Predictions REST API (changelog, 2026-07-04).
  `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus` were removed from the crate (breaking). Exchange
  schedule remains available through `get_exchange_schedule`.

- `GET /communications/quotes` no longer supports `market_ticker` / `event_ticker` filters
  (changelog, 2026-06-20); these were removed from `GetQuotesParams` (breaking). `min_ts` / `max_ts`
  time-window filters were added the same window (2026-06-18) as their replacement, alongside a
  previously-missing `user_filter` field that was already present in the OpenAPI spec.

- RFQ-scoped quote action/lookup endpoints (`GET`/`DELETE`/`PUT .../rfqs/{rfq_id}/quotes/{quote_id}`)
  were added 2026-06-25 (accept/confirm/delete) and 2026-07-09 (lookup) as `get_rfq_quote`,
  `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`. The quote-ID-only equivalents
  (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) remain but are marked `#[deprecated]`
  per the OpenAPI spec's own `deprecated: true` annotation on those operations; quotes are no longer
  guaranteed queryable/actionable this way unless in a post-acceptance state.

- `SubaccountBalance.exchange_index` was added 2026-07-02: `GET /portfolio/subaccounts/balances` now
  returns one balance entry per exchange index rather than one combined row per subaccount.

- `ApiKey.subaccount`, `CreateApiKeyRequest.subaccount`, and `GenerateApiKeyRequest.subaccount` were
  added 2026-07-02 to support subaccount-restricted API keys (`POST /api_keys`,
  `POST /api_keys/generate`). Absent/`None` means an unrestricted key.

- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active` and `exchange_index_statuses`
  (2026-07-02); `Series.exchange_index`, `EventData.exchange_index`, `EventData.settlement_sources`,
  `Market.exchange_index` were added across 2026-06-18 – 2026-07-30 for the multi-exchange-index
  rollout. `EventMetadata.cadence` is modeled from the 2026-07-30 changelog entry even though
  `Event.product_metadata` remains an opaque `object` in the OpenAPI schema (no typed shape to grep).

- `update_order_group_limit` now takes a `SubaccountQueryParams` argument (breaking signature
  change) since `PUT .../order_groups/{id}/limit` gained a `subaccount` query parameter
  (changelog, 2026-08-06).

- `ErrorResponse.service` is marked `#[deprecated]` (Rust attribute) rather than removed: the
  upstream field was deprecated 2026-07-28 and removed from all REST error responses 2026-08-06.
  Kept as `Option<String>` (always `None` now) so older cached error bodies still parse; callers
  should branch on `code` instead, which was already the documented stable contract.

- The `pyth_value` WebSocket channel (added 2026-07-23, requires authentication) mirrors the
  `cfbenchmarks_value` channel's shape: `underlying_tickers` seeds the subscription (`["all"]` for
  every underlying), and `update_subscription` supports `subscribe_underlyings` /
  `unsubscribe_underlyings` / `underlying_list` actions via new `WsUpdateAction` variants. Modeled as
  `WsPythValue` / `WsPythUnderlyingList` in `ws::types::messages::pyth`, routed through
  `WsDataMessageV2` like every other typed channel.

- Seven new `price_level_structure` string values were introduced 2026-07-23
  (`center_{whole,half,quint}_edge_{half,quint,deci}_cent` combinations). No crate change was needed:
  the field is already modeled as a loose `Option<String>` (never an enum) specifically to tolerate
  new values without a release.

- `GET /historical/positions` (added 2026-07-23) reuses the existing `GetPositionsResponse` shape and
  is modeled with a dedicated `GetHistoricalPositionsParams` (`ticker`, `event_ticker`, `limit`,
  `cursor`) rather than the broader `GetPositionsParams`, since the historical endpoint's query
  parameters are a strict subset (no `count_filter`, no `subaccount`).

- `GET /live_data/events/{event_ticker}` (added 2026-07-30) is modeled as `EventLiveData` /
  `GetEventLiveDataResponse` in `rest::live_data`. `is_historical` and `default_range` are
  `Option<bool>` / `Option<String>` since both are conditional per the OpenAPI description.

- Multivariate lookup history endpoints are fully deprecated (changelog, 2026-07-02) and no longer
  documented in the OpenAPI spec. `get_multivariate_event_collection_lookup_history` is kept but
  marked `#[deprecated]` rather than removed.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
