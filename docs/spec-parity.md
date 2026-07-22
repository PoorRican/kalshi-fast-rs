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

- `Market.response_price_units` and `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` (REST and the WS `market_positions` zero-copy view) were
  removed from the OpenAPI schema on 2026-07-09/03. They are removed from the public Rust API
  rather than kept as `Option` — the crate had already modeled `fractional_trading_enabled` as
  deprecated-and-ignorable, and Kalshi confirmed on 2026-04-17 that fractional trading state can no
  longer change, so there is no residual signal in these fields worth preserving.
- `WsMarketLifecycleV2.fractional_trading_enabled` is also removed: the current AsyncAPI
  `marketLifecycleV2Payload.msg` schema never lists it, and the corresponding
  `fractional_trading_updated` lifecycle event was removed from the AsyncAPI back on 2026-04-17. The
  `WsMarketLifecycleEventType::FractionalTradingUpdated` enum variant is left in place (harmless dead
  code covered by `#[serde(other)] Unknown` semantics) rather than removed, since no upstream source
  currently forces a variant-level break.
- `GET /exchange/announcements` was removed from the OpenAPI on 2026-07-04. `Announcement`,
  `AnnouncementType`, `AnnouncementStatus`, `GetExchangeAnnouncementsResponse`, and
  `get_exchange_announcements()` are removed from the crate; exchange schedule remains available via
  `get_exchange_schedule()`.
- `GET /multivariate_event_collections/{ticker}/lookup` (the *history* variant, not the ticker-pair
  lookup `PUT` of the same path) was removed from the OpenAPI on 2026-07-02 ("Multivariate lookup
  history endpoints are fully deprecated"). `GetMultivariateEventCollectionLookupHistoryParams`,
  `GetMultivariateEventCollectionLookupHistoryResponse`, `LookupPoint`, and
  `get_multivariate_event_collection_lookup_history()` are removed. The `PUT .../lookup` endpoint
  (`lookup_tickers_for_market_in_multivariate_event_collection`) is unaffected — it is marked
  `deprecated` in the OpenAPI but still present, so the crate keeps it.
- `GET /communications/quotes` dropped its `market_ticker` / `event_ticker` filters on 2026-06-20;
  those fields are removed from `GetQuotesParams`. `min_ts`, `max_ts`, and `user_filter` were added
  to match the current parameter list.
- RFQ quote actions (get/delete/accept/confirm) are now available RFQ-scoped
  (`/communications/rfqs/{rfq_id}/quotes/{quote_id}...`, added 2026-06-25/07-09) alongside the
  legacy quote-ID-only endpoints, which the OpenAPI marks `deprecated: true`. The crate mirrors this
  with `#[deprecated]` on `get_quote`/`delete_quote`/`accept_quote`/`confirm_quote`, pointing callers
  at the new `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote` methods. Kalshi
  has not announced a removal date for the legacy endpoints, so they are deprecated, not removed.
- `SubaccountBalance` gained a required `exchange_index: i64` field (2026-07-02): a subaccount with
  balances on multiple exchange indexes now returns one row per index instead of one row per
  subaccount. Modeled as non-optional since the field is unconditionally required in the schema.
- `POST /api_keys`, `POST /api_keys/generate`, and `GET /api_keys` gained an optional 0-63
  `subaccount` restriction field (2026-07-02): `CreateApiKeyRequest.subaccount`,
  `GenerateApiKeyRequest.subaccount` (write side), and `ApiKey.subaccount` (nullable read side).
- `price_level_structure` is modeled as a raw `Option<String>` everywhere it appears (`Market`,
  `WsMarketLifecycleV2`), not a typed enum, so the seven new tick-size structures added 2026-07-07
  round-trip with no crate change. `PriceRange` (`start`/`end`/`step`, all strings) already matches
  the OpenAPI/AsyncAPI shape used by both the REST `Market.price_ranges` field and the new
  `WsMarketLifecycleV2.price_ranges` field (added 2026-06-30, emitted on `created` and
  `price_level_structure_updated` events).
- `WsMarketLifecycleV2` gained top-level `strike_type`, `cap_strike`, and `custom_strike` fields
  (2026-06-18), present only on `metadata_updated` events, distinct from the
  `additional_metadata.*` copies emitted on market creation — following the same pattern already
  used for `floor_strike` / `yes_sub_title`.
- The `pyth_value` WebSocket channel (added 2026-07-13) is modeled end-to-end following the same
  pattern as `cfbenchmarks_value`: `WsChannelV2::PythValue`, `WsMsgType::PythValue` /
  `PythValueUnderlyingList`, `WsPythValue` / `WsPythUnderlyingList` (+ zero-copy `*Ref` variants),
  and `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` plus an
  `underlying_tickers` field on `WsSubscriptionParamsV2` / `WsUpdateSubscriptionParamsV2`. Like
  `cfbenchmarks_value`, it is *not* included in `WsChannelV2::is_private()` even though the AsyncAPI
  channel description says "Requires authentication" — that reflects the WS session needing to be
  authenticated overall (per the 2026-04 handshake change), not that the channel is a private
  per-account data feed.
- `GET /events` gained a `tickers` filter parameter (comma-separated event tickers, added
  2026-06-18): `GetEventsParams.tickers`.
- `EventData` gained `settlement_sources: Vec<SettlementSource>` (required-but-nullable, added
  2026-06-18), reusing the `SettlementSource` type already defined for `Series` / `EventMetadata`.
- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (added 2026-07-02); the latter is a
  new `ExchangeIndexStatus` struct with all-required fields (`exchange_index`, `exchange_active`,
  `trading_active`, `intra_exchange_transfers_active`).
- `GetHistoricalCutoffResponse` gained `market_positions_last_updated_ts: Option<String>` (added
  2026-07-23; optional because it is not in the schema's `required` list, unlike the three original
  cutoff fields). The new `GET /historical/positions` endpoint
  (`get_historical_positions`/`GetHistoricalPositionsParams`) reuses the existing
  `GetPositionsResponse` shape from `GET /portfolio/positions`, since both return
  `market_positions` / `event_positions` / `cursor`.
- New account endpoints added 2026-06-11: `GET /account/api_usage_level/volume_progress`
  (`get_account_api_usage_level_volume_progress`, returns trailing-30d volume progress toward
  volume-based tiers) and `POST /account/api_usage_level/upgrade`
  (`upgrade_account_api_usage_level`, self-promotes to a permanent Advanced grant; 201 with no body,
  modeled as `EmptyResponse`).
- Several 2026-06/07 changelog entries required no crate change because the affected surface is not
  modeled here: margin-market types (mark prices, notional fields, `tick_size`, `is_portfolio`,
  `order_reason`, per-position risk omission) are out of scope per the "margin market types not in
  crate" precedent from the 0.6.0 refresh, and FIX-protocol changes (Tag 2446, `ExDestination`,
  `ExecInst`, reject-reason text) are out of scope since this crate is REST/WebSocket only. API-key
  scope additions (`write::trade`, `read`/`write::block_trade_accept`) required no change because
  scopes are already stored as `Vec<String>`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
