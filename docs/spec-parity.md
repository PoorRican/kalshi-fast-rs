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

- The `service` field on `ErrorResponse` was deprecated by Kalshi 2026-07-28 and removed from
  error response bodies 2026-07-29. Branch on `code` instead. The field is marked
  `#[deprecated]` but kept as `Option<String>` so older cached/mocked payloads still parse.
- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Kalshi REST schema 2026-07-09 (the
  WebSocket `market_position` message never had `resting_orders_count`). These fields are marked
  `#[deprecated]` and always deserialize to `None` on current payloads. Use
  `Market.price_level_structure` / `Market.price_ranges` in place of the removed pricing fields.
- The legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were removed from the OpenAPI spec
  2026-06-18 in favor of the V2 event-order endpoints (`create_order_v2` etc, already present in the
  crate since 0.6.0). The legacy methods are marked `#[deprecated]` rather than deleted so existing
  call sites still compile; the exchange now returns an error directing callers to the V2 endpoint.
- `get_exchange_announcements` / `GET /exchange/announcements` was removed from the OpenAPI spec
  2026-07-04. The method is marked `#[deprecated]`; use `get_exchange_schedule` for exchange hours.
- `get_multivariate_event_collection_lookup_history` / `GET
  /multivariate_event_collections/{ticker}/lookup` (the history-lookup GET) was removed from the
  OpenAPI spec 2026-07-02 ("Multivariate lookup history endpoints are fully deprecated"). The method
  is marked `#[deprecated]`. The PUT lookup endpoint (`lookup_tickers_for_market_in_multivariate_event_collection`)
  is unaffected.
- `GetQuotesParams` no longer has `market_ticker` / `event_ticker` filters — Kalshi removed support
  for filtering `GET /communications/quotes` by market or event ticker 2026-06-20. `min_ts` /
  `max_ts` filters were added in their place (2026-06-18). RFQ-scoped quote actions
  (`get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`) were added
  2026-06-25/07-09 and are preferred over the quote-ID-only actions (`get_quote`, `delete_quote`,
  `accept_quote`, `confirm_quote`, now `#[deprecated]`), which Kalshi still supports but has marked
  deprecated in the OpenAPI spec.
- `update_order_group_limit` now takes a `SubaccountQueryParams` argument (the `subaccount` is a
  query parameter, not a request-body field) matching the sibling `reset_order_group` /
  `trigger_order_group` methods. Added 2026-07-30.
- `WsChannelV2::is_private()` was missing `CfbenchmarksValue` and now also `PythValue` — both
  channels require authentication per their AsyncAPI channel descriptions ("Requires
  authentication"), so subscribing to either without an authenticated connection previously slipped
  past the client's auth gate. Fixed as part of adding `pyth_value` channel support.
- The `pyth_value` WebSocket channel (added 2026-07-23) delivers deduplicated Pyth price updates by
  underlying ticker, mirroring the `cfbenchmarks_value` channel's shape: seed `underlying_tickers` on
  subscribe (or `["all"]`), and use `update_subscription_v2` with `WsUpdateAction::SubscribeUnderlyings`
  / `UnsubscribeUnderlyings` / `UnderlyingList` plus the new `underlying_tickers` field on
  `WsUpdateSubscriptionParamsV2`. `validate_update` rejects mixing underlying actions with market
  targets and requires `underlying_tickers` for the add/remove actions, matching the AsyncAPI error
  semantics (error code 28).
- `WsMarketLifecycleV2` (`market_lifecycle_v2` / `multivariate_market_lifecycle`) gained
  `exchange_index` (present only on `created` events), `price_ranges` (present on `created` and
  `price_level_structure_updated` events, alongside `price_level_structure`), and `strike_type` /
  `cap_strike` / `custom_strike` (present only on `metadata_updated` events, alongside the existing
  top-level `floor_strike` / `yes_sub_title`). `WsEventLifecycle` (`event_lifecycle`) gained
  `exchange_index`. All are `Option` for the same top-level-conditional-field reasons as the
  existing `floor_strike` / `yes_sub_title` fields.
- `WsQuoteCreated`, `WsQuoteAccepted`, and `WsQuoteExecuted` (the `communications` channel's
  `quote_created` / `quote_accepted` / `quote_executed` messages) gained `subaccount: Option<i64>`,
  present only when your side of the quote used a subaccount.
- `EventData` (`GET /events`, `GET /events/{event_ticker}`) gained `settlement_sources` (mirroring
  the field already on `Series`, added 2026-06-18) and `exchange_index` (added 2026-07-30).
  `EventMetadata` (`product_metadata`) gained `cadence: Option<String>` (added 2026-07-28).
  `Series` gained `exchange_index` (added 2026-07-30). `GetEventsParams` gained `tickers` (CSV
  filter, added 2026-06-18) and `min_updated_ts`.
- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` (per-exchange-index status breakdown), both
  added 2026-07-02.
- `SubaccountBalance` (`GET /portfolio/subaccounts/balances`) gained `exchange_index: Option<i64>` —
  Kalshi now returns one balance entry per exchange index per subaccount (added 2026-07-02).
- `ApiKey`, `CreateApiKeyRequest`, and `GenerateApiKeyRequest` gained `subaccount: Option<u32>` for
  subaccount-restricted API keys (added 2026-07-02).
- Added `get_account_api_usage_level_volume_progress` (`GET
  /account/api_usage_level/volume_progress`) and `upgrade_account_api_usage_level` (`POST
  /account/api_usage_level/upgrade`), both added 2026-06-11.
- Added `get_historical_positions` (`GET /historical/positions`, added 2026-07-22) and
  `GetHistoricalCutoffResponse.market_positions_last_updated_ts` (the cutoff separating live from
  historical positions).
- Added `get_event_live_data` (`GET /live_data/events/{event_ticker}`, added 2026-07-28) returning
  event-keyed live data (crypto price charts, commodity timeseries, weather observations); the
  `EventLiveData.live_data_type` field (JSON `type`) names the schema of `details`.
- `price_level_structure` on `Market` and the WebSocket lifecycle payload remain plain `String`
  (not a fixed enum) — the seven new tapered/centered structure values introduced 2026-07-23 require
  no crate change; always read the market's live `price_ranges` for valid order prices rather than
  branching on the structure label.
- Richer combo-validation error bodies on multivariate market creation (added 2026-07-29) require no
  crate change: `ErrorResponse.message` / `.details` are already untyped `Option<String>` and the
  `code` values are unchanged.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
