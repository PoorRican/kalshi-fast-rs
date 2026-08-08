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

- `GET /exchange/announcements` was removed from the Predictions REST API (2026-07-04). The endpoint,
  `get_exchange_announcements()`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus` were removed from the public Rust API to match.
  `GET /exchange/schedule` remains the source for exchange scheduling.
- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (2026-07-02). Each
  `ExchangeIndexStatus` entry is required-complete per the OpenAPI schema (`exchange_index`,
  `description`, `exchange_active`, `trading_active`, `intra_exchange_transfers_active`); `description`
  itself was added later (2026-08-13) but is required on the current schema, so it is modeled as
  non-`Option` `String`.
- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the OpenAPI schema (2026-07-09) and dropped
  from the public Rust API. `Market.price_level_structure` / `Market.price_ranges` remain the canonical
  replacements. `resting_orders_count` was also removed from `MarketPositionRef` (the WS-borrowed
  mirror of the REST `MarketPosition` type) for the same reason; it is unrelated to the actually
  distinct `WsMarketPosition` (`market_position` channel message), which never had this field.
- `fractional_trading_enabled` and the `WsMarketLifecycleEventType::FractionalTradingUpdated` variant
  were removed from `WsMarketLifecycleV2`/`Ref`: the current AsyncAPI `market_lifecycle_v2` payload no
  longer documents either the field or the `fractional_trading_updated` event type. The enum's
  `#[serde(other)] Unknown` catch-all absorbs any legacy `fractional_trading_updated` payload without
  panicking.
- `WsMarketLifecycleV2`/`Ref` gained `exchange_index` (only on `created` events), `price_ranges`
  (mirrors REST `Market.price_ranges`/`PriceRange`, emitted on `created` and
  `price_level_structure_updated`), and top-level `strike_type` / `cap_strike` / `custom_strike`
  (only on `metadata_updated`, alongside the pre-existing `floor_strike` / `yes_sub_title`).
  `WsEventLifecycle`/`Ref` gained `exchange_index` too. All are `Option`.
- `price_level_structure` (REST `Market`, WS lifecycle) has always been an untyped `String`, never a
  closed Rust enum, so the 7 new `center_*` values and the `center_centi_edge_centi_cent` (centicent
  pricing on combo markets) value need no crate change — they round-trip losslessly. Consumers should
  read `price_ranges` for valid order prices rather than branching on the structure name.
- The multivariate lookup surface was fully removed 2026-08-06: `PUT
  /multivariate_event_collections/{collection_ticker}/lookup`, `GET
  .../{collection_ticker}/lookup` (lookup history), and the WS `multivariate` channel
  (`multivariate_lookup` message) no longer exist upstream. Removed from the public Rust API:
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, their request/response types, `WsMultivariate*`,
  `WsChannelV2::Multivariate`, and `WsMsgType::Multivariate` / `MultivariateLookup`. The
  `multivariate_market_lifecycle` channel (a distinct, still-valid channel for multivariate market
  state changes) is unaffected. `MultivariateEventCollection` gained `exchange_index: Option<u32>`
  (2026-08-06).
- `is_block_trade: bool` (with `#[serde(default)]`) was added to `WsTrade`/`WsTradeRef` (2026-08-13),
  matching the REST `Trade.is_block_trade` precedent from 0.6.0.
- RFQ-scoped quote lookup/action endpoints were added under `/communications/rfqs/{rfq_id}/quotes/
  {quote_id}` (lookup added 2026-07-09; delete/accept/confirm added 2026-06-25):
  `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`. The quote-ID-only
  endpoints (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`) remain supported upstream
  but are marked `#[deprecated]` at the Rust level.
- `GET /communications/quotes` dropped the `market_ticker` / `event_ticker` query filters effective
  2026-06-20 (removed from `GetQuotesParams`, a breaking field removal) and gained `min_ts` / `max_ts`
  (2026-06-18). Filter by RFQ, status, user, or update time instead.
- Legacy (pre-V2) order mutation methods — `create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders` — are marked `#[deprecated]` at the
  Rust level (upstream deprecation 2026-06-18). Use the `_v2` methods added in 0.6.0. `get_order` /
  `get_orders` / `get_order_queue_position(s)` are read endpoints and were not deprecated upstream.
- API keys can now be restricted to a single subaccount (2026-07-02): `CreateApiKeyRequest` and
  `GenerateApiKeyRequest` gained `subaccount: Option<u32>` (0-63); `ApiKey` gained the same field on
  responses (absent/`None` means unrestricted).
- `update_order_group_limit` gained a `params: SubaccountQueryParams` argument (2026-08-06, `subaccount`
  is a query parameter, not a body field) — a breaking method-signature change, matching the sibling
  `get_order_group` / `delete_order_group` / `reset_order_group` / `trigger_order_group` pattern.
- `pyth_value` is a new authenticated-only AsyncAPI channel (2026-07-23) that mirrors the
  `cfbenchmarks_value` pattern: it uses `underlying_tickers` (not market tickers) for subscription
  seeding (`["all"]` for every underlying), emits `pyth_value` (per-underlying deduplicated price) and
  `pyth_value_underlying_list` (recently streamed underlyings) messages, and supports
  `subscribe_underlyings` / `unsubscribe_underlyings` / `underlying_list` via
  `WsUpdateAction` + `WsUpdateSubscriptionParamsV2.underlying_tickers`, following the same
  `validate_update` / `SubscriptionTracker` folding logic as the CF Benchmarks index actions.
- `ErrorResponse.service` was deprecated 2026-07-28 and fully removed from all error responses
  2026-08-06. It remains modeled as `Option<String>` (an explicit exception to the field-removal rule,
  since it already tolerated absence and a hard removal would be a needless breaking change) and will
  simply always deserialize to `None` going forward. Branch on `code` instead.
- `EventMetadata` (the crate's typed view of `Event.product_metadata`) gained `cadence: Option<String>`
  (2026-07-30). The manually-defined OpenAPI leaves `product_metadata` itself untyped
  (`type: object`, no `properties`), so this field is not formally documented in the spec; it is typed
  here for parity with the changelog and with the other already-typed `product_metadata` fields
  (`competition`, `image_url`, etc.), consistent with prior practice in this crate.
- `EventData` (REST `Event`) gained `settlement_sources: Vec<SettlementSource>` (2026-06-18, mirrors
  `Series.settlement_sources`) and `exchange_index: Option<u32>` (present in the current OpenAPI schema
  but not explicitly called out in the changelog entries reconciled by this refresh).
- `GET /historical/positions` (`get_historical_positions`) was added (2026-07-23) for settled positions
  archived per whole event; it reuses `GetPositionsResponse`. `GetHistoricalCutoffResponse` gained the
  matching `market_positions_last_updated_ts: Option<String>` cutoff field.
- `GET /account/api_usage_level/volume_progress` and `POST /account/api_usage_level/upgrade` were added
  (2026-06-11). The upgrade endpoint has no documented response schema (bare 201), so it is modeled as
  returning `EmptyResponse` like other action endpoints without a JSON body.
- `GET /portfolio/intra_exchange_instance_transfers(/{transfer_id})` were added (2026-08-13).
  `IntraExchangeInstanceTransfer.source` / `.destination` / `.status` are kept as raw `String` (not
  closed enums), matching the `ApiUsageLevelGrant.exchange_instance` precedent, so future values
  round-trip losslessly.
- `SubaccountBalance` gained a required `exchange_index: u32` (2026-07-02): a subaccount with funds on
  multiple exchange indexes now appears as multiple entries instead of one combined row.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
