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

### 2026-07-25 reconciliation pass (0.6.0 → 0.7.0)

- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated by Kalshi between
  2026-06-18 and 2026-06-25 (calls now return "Please switch to the V2 endpoints") and are entirely
  absent from the current OpenAPI paths. The Rust methods are kept (not removed, to avoid an
  unnecessary breaking change) but marked `#[deprecated]` pointing at the `_v2` replacements.
- `GetExchangeAnnouncementsResponse`, `Announcement`, and `get_exchange_announcements()` are marked
  `#[deprecated]`: `GET /exchange/announcements` was removed from the OpenAPI spec on 2026-07-04 and
  calls will 404. Kept for source compatibility rather than deleted.
- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema on 2026-07-09. The
  fields were already `Option`, so no shape change was needed; they are now additionally marked
  `#[deprecated]` (the exchange will never populate them again). The WebSocket
  `market_lifecycle_v2` `fractional_trading_enabled` field and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant are deprecated for the same reason
  — `fractional_trading_updated` is no longer in the AsyncAPI `event_type` enum.
- `market_lifecycle_v2` `metadata_updated` events gained top-level `strike_type`, `cap_strike`, and
  `custom_strike` fields (2026-06-18), alongside the pre-existing top-level `floor_strike` /
  `yes_sub_title`. These are distinct from the same-named fields nested in `additional_metadata`
  (which are only emitted on `created` events). `created` and `price_level_structure_updated`
  events also gained a `price_ranges` array (2026-07-02), modeled by reusing
  `rest::markets::PriceRange` rather than duplicating the shape.
- Seven new `price_level_structure` values were added upstream (2026-07-23, pilot rollout the week
  of 2026-07-27). No crate change was needed: the field is modeled as a plain `Option<String>`
  everywhere, not an enum, so new values round-trip without a release.
- `GET /communications/quotes` (`GetQuotesParams`) had `market_ticker` / `event_ticker` filters
  **removed** (2026-06-20) — these fields were deleted from the struct (a breaking Rust API change).
  It gained `min_ts` / `max_ts` (2026-06-18) and a new `user_filter` (filters by quote creator,
  distinct from the pre-existing `rfq_user_filter` which filters by RFQ creator). Both
  `quote_creator_user_id` and `rfq_creator_user_id` are marked `#[deprecated]` (deprecated upstream
  in the OpenAPI spec, not removed). `GetRFQsParams.creator_user_id` is likewise deprecated, and
  `GetRFQsParams` gained a `user_filter` field, both from the same spec pass.
- Added RFQ-scoped quote endpoints (2026-06-25): `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`, all taking `(rfq_id, quote_id, ..)`. The prior
  quote-ID-only endpoints (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) are kept
  but marked `#[deprecated]` per the OpenAPI spec's own `deprecated: true` flag on those operations.
- `Quote` gained `post_only`, `creator_subaccount`, and `rfq_creator_subaccount` (all `Option`,
  visible only to the respective party) from the sub-account-restricted API key and post-only-quote
  changes rolled out across 2026-06-24 and 2026-07-02.
- `EventData` (`GetEventsResponse` / `GetEventResponse`) gained `settlement_sources` (mirroring the
  field already modeled on `Series`), plus `fee_type_override`, `fee_multiplier_override`, and
  `exchange_index`, all found while updating the struct's required-field set against the current
  schema. `GetEventsParams` gained `tickers` (comma-separated event ticker filter) and
  `min_updated_ts`; the latter is present in the OpenAPI spec (mirroring the existing
  `GetMarketsParams.min_updated_ts`) but is not called out in the changelog — modeled per the
  YAML-is-authoritative-for-shape rule.
- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active` and an optional
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` breakdown (2026-07-02). Both are `Option`
  since the OpenAPI spec does not mark them required.
- `SubaccountBalance` (`GetSubaccountBalancesResponse`, per-index subaccount balances, 2026-07-02)
  gained `exchange_index`, `voluntarily_locked`, `settlement_advance`, and
  `settlement_advance_state`. The first three are in the OpenAPI `required` list for this struct,
  but are modeled defensively (`#[serde(default)]`) since they are new; `settlement_advance_state`
  is genuinely optional (absent unless a settlement-advance state has been established). Related
  endpoints visible in the same schema area (`settlement-advance-lock`/`unlock`,
  `subaccounts/positions/transfer`) are **not** modeled — they aren't referenced by any changelog
  entry in this reconciliation window, so implementing them is deferred to a future pass.
- `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest` gained an optional `subaccount: u8`
  (0-63) for the subaccount-restricted API key feature (2026-07-02).
- Added `GET /account/api_usage_level/volume_progress`
  (`get_account_api_usage_level_volume_progress`) and `POST /account/api_usage_level/upgrade`
  (`upgrade_account_api_usage_level`), both added 2026-06-11.
- Added `GET /historical/positions` (`get_historical_positions`), added 2026-07-23. Reuses the
  existing `GetPositionsResponse` shape (the OpenAPI spec does the same). `GetHistoricalCutoffResponse`
  gained `market_positions_last_updated_ts: Option<String>` (not required in the spec).
- Added the `pyth_value` WebSocket channel in full (2026-07-23), mirroring the existing
  `cfbenchmarks_value` implementation: `WsChannelV2::PythValue`, `WsMsgType::PythValue` /
  `PythValueUnderlyingList`, `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` /
  `UnderlyingList`, an `underlying_tickers: Option<Vec<String>>` field on both
  `WsSubscriptionParamsV2` and `WsUpdateSubscriptionParamsV2`, and `WsPythValue` /
  `WsPythUnderlyingList` message types (plus `Ref` zero-copy variants) in
  `ws::types::messages::pyth`. `validate_update` and the subscription tracker treat underlying-ticker
  actions the same way index actions are treated for `cfbenchmarks_value`.
- Deprecated (not removed): `get_multivariate_event_collection_lookup_history` (GET
  `.../lookup`, entirely absent from the current OpenAPI paths — the changelog calls this "fully
  deprecated") and `lookup_tickers_for_market_in_multivariate_event_collection` (PUT `.../lookup`,
  marked `deprecated: true` in the spec, "predates RFQs").

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
