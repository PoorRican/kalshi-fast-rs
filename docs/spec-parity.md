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

### 2026-09-27 refresh (validated through changelog 2026-10-01, OpenAPI 3.31.0, AsyncAPI 2.0.0)

- **Removed** (matching upstream removals — these are breaking Rust API changes, not carried
  forward as `Option`, per the refresh workflow's "don't preserve removed fields" rule):
  - `GET /exchange/announcements` (`get_exchange_announcements`, `GetExchangeAnnouncementsResponse`,
    `Announcement`, `AnnouncementType`, `AnnouncementStatus`) — removed from the Predictions REST API
    2026-07-04. Exchange schedule remains available via `get_exchange_schedule`.
  - `Market.response_price_units`, `Market.fractional_trading_enabled`,
    `MarketPosition.resting_orders_count` — removed from the OpenAPI schema 2026-07-09. The
    fixed-point/price-range fields are the canonical replacements.
  - `Market.liquidity_dollars` (and the already-stale `liquidity` / `liquidity_fp` companions, which
    the live OpenAPI schema no longer documents either) — `liquidity_dollars` removed 2026-10-01.
    Use `yes_bid_size_fp` / `yes_ask_size_fp` for top-of-book size.
  - `EventData.available_on_brokers` — stopped being populated (always `false`) 2026-08-27, formally
    removed from the schema 2026-09-10.
  - `ErrorResponse.service` — deprecated 2026-07-28, removed 2026-08-06. Branch on `code` instead.
  - The multivariate ticker-pair lookup surface: `PUT
    /multivariate_event_collections/{collection_ticker}/lookup` (and its GET lookup-history sibling)
    plus the `multivariate` WebSocket channel (message type `multivariate_lookup`,
    `WsMultivariate`/`WsMultivariateRef`) — removed 2026-08-06. Use
    `create_market_in_multivariate_event_collection` / the RFQ communications APIs, and
    `multivariate_market_lifecycle` for multivariate market state changes.
  - `GetQuotesParams::market_ticker` / `event_ticker` — `GET /communications/quotes` stopped
    supporting these filters 2026-06-20. Filter by user, RFQ, status, or update time instead.

- **`FeeType`** gained `QuadraticWithComboMakerFees` (serialized `quadratic_with_combo_maker_fees`;
  found directly in the live OpenAPI spec, not called out in the changelog text) alongside the
  already-modeled `QuadraticWithMakerFees`. The `#[serde(other)] Unknown` catch-all still protects
  against further additions.

- **Exchange sharding** (`exchange_index`, rolled out across Predictions through 2026 Q3) is now
  surfaced, as `Option<ExchangeIndex>` (`ExchangeIndex = i64`), on: `GetExchangeStatusResponse`
  (plus the new `exchange_index_statuses: Vec<ExchangeIndexStatus>` per-shard breakdown, and
  `intra_exchange_transfers_active`), `Series`, `EventData`, `SubaccountBalance`,
  `MultivariateEventCollection`, `MarketPosition`, `Fill`, `Settlement`, `Order`, `WsFill`,
  `WsUserOrder`, and the lifecycle messages (`WsMarketLifecycleV2`, `WsEventLifecycle`). Several of
  these (`MarketPosition.exchange_index`, `Fill.exchange_index`, `event_lifecycle.exchange_index`)
  are spec-`required`, but are kept `Option` here since the rollout is gradual and older/cached
  payloads may still lack the field. `GetOrdersParams`, `GetPositionsParams`, and `GetFillsParams`
  each gained an optional `exchange_index` filter (2026-08-20).

- **`GET /portfolio/balance`** now takes a `GetBalanceParams { subaccount, exchange_index }`
  argument (previously no-arg) — both `balance` and `portfolio_value` scope to one exchange index
  when `exchange_index` is set, and cover all indexes otherwise (2026-08-13/2026-08-20).
  `GetBalanceResponse` gained `balance_breakdown: Option<Vec<IndexedBalance>>` (per-exchange-index
  balances; omitted only for subaccount-restricted keys). `balance_dollars` became spec-`required`
  around the same time but is kept `Option` here for tolerance.

- **`market_lifecycle_v2`** gained, all as `Option`/tolerant fields on the existing
  `WsMarketLifecycleV2` superset struct (which is also reused for `multivariate_market_lifecycle`):
  `strike_type`, `cap_strike`, `custom_strike` on `metadata_updated` events (2026-06-18); `exchange_index`
  on `created` events (2026-07-30, also added to `event_lifecycle`); and `price_ranges:
  Vec<WsPriceRange>` (`{start, end, step}` dollar bands) alongside `price_level_structure` on
  `created` / `price_level_structure_updated` events (2026-07-02).

- **New `price_level_structure` string values** (seven new `center_*_edge_*_cent` variants added
  2026-07-23/2026-08-13, plus `center_deci_edge_centi_cent` for combo markets 2026-08-13/2026-09-03)
  need **no crate change**: `Market.price_level_structure` and the WS equivalent are already modeled
  as raw `String`/`Option<String>`, not a closed enum, exactly so new structure names round-trip
  without a release. Consumers should always read valid prices from `price_ranges` rather than
  keying logic off the structure name (matches upstream guidance).

- **Communications (RFQ/Quote)** gained several fields, matching gaps found by diffing the live
  AsyncAPI/OpenAPI against the existing structs (not all were called out explicitly in the
  changelog text): `Quote`/`RFQ`/`CreateRFQRequest` gained `target_cost_excludes_fees` (2026-09-10);
  `Quote` gained `post_only`, `creator_subaccount`, `rfq_creator_subaccount`; `RFQ` gained
  `creator_subaccount`. The WebSocket `quote_created`, `quote_accepted`, and `quote_executed`
  messages all gained an optional `subaccount` field (2026-07-30) — only `quote_created` was
  previously wired despite the 0.6.0 changelog claiming parity across all three. `WsQuoteCreated`
  and `WsQuoteAccepted` also gained `rfq_creator_id: Option<String>`, a spec-required field that was
  simply missing from the original models (`WsQuoteExecuted` already had it, non-`Option`, since it
  matches the taker/creator identity, not the anonymized counterparty field).
  `GetQuotesParams` gained `min_ts`, `max_ts` (2026-06-18) and `user_filter` (mirrors
  `rfq_user_filter`). New RFQ-scoped quote action endpoints were added —
  `get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote` — alongside the existing
  quote-ID-only endpoints, which upstream deprecated (2026-06-25/2026-07-09) but still serves.

- **`ApiKey`** gained `subaccount: Option<u32>` and `fcm_subtrader_id: Option<String>`
  (2026-07-02); `CreateApiKeyRequest`/`GenerateApiKeyRequest` gained the same restriction
  parameters. `GetApiKeysResponse` gained `api_key_region_expiration_ts: Option<i64>` (2026-08-16).
  `GenerateApiKeyRequest`/`GenerateApiKeyResponse` gained `key_type: Option<String>` (2026-09-24,
  Ed25519 API keys) as a raw pass-through string (`"rsa"` | `"ed25519"`) — **the crate's signing
  implementation (`auth.rs`) is RSA-PSS only and was not extended to sign with Ed25519 in this
  refresh**; only the request/response field is modeled so callers can observe/request the key type
  via the plain HTTP API. Actually performing Ed25519-signed requests is not yet supported.

- **`GetFcmOrdersParams`**: `subtrader_id` changed from required `String` to `Option<String>`, and a
  new `client_order_ids: Option<Vec<String>>` (CSV, max 100) was added; `GET /fcm/orders` requires at
  least one of the two (2026-09-03), enforced by `GetFcmOrdersParams::validate()`.

- **`WsFill.purchased_side`** changed from `YesNo` to `Option<YesNo>`. The live AsyncAPI marks it
  `deprecated: true` in favor of `outcome_side`/`book_side` (the same migration already applied to
  `side`/`action`) while still listing it as required; the field is kept `Option` proactively rather
  than waiting for a future breaking removal, consistent with how `side`/`action` were already
  handled here.

- **New endpoints added**: `get_account_api_usage_level_volume_progress` (`GET
  /account/api_usage_level/volume_progress`, 2026-06-11), `upgrade_account_api_usage_level` (`POST
  /account/api_usage_level/upgrade`, 2026-06-11), `get_historical_positions` (`GET
  /historical/positions`, 2026-07-23, with the `subaccount` param added 2026-09-03).

- **`UpdateOrderGroupLimitRequest`** gained `subaccount: Option<u32>` (2026-08-06).

- **`GetPortfolioRestingOrderTotalValueResponse`** gained `resting_order_value_breakdown:
  Vec<IndexedBalance>` (2026-08-20).

- **`GetHistoricalFillsParams`/`GetHistoricalOrdersParams`** gained `min_ts` (2026-09-17) and
  `subaccount` (2026-09-24).

- **`WsSubscriptionParamsV2`** gained `user_filter: Option<String>` (`communications` channel only;
  pass `"self"` to receive `rfq_created`/`rfq_deleted` only for RFQs the caller created, 2026-10-01).

- **Known pre-existing distinction, not changed in this refresh**: the WebSocket `market_positions`
  channel (`MarketPositionRef` in `ws/types/mod.rs`) reuses the REST `MarketPosition` struct's shape,
  but the live AsyncAPI's `marketPositionPayload` schema for that channel documents a materially
  different field set (`user_id`, `position_cost_dollars`, `position_fee_cost_dollars`, `volume_fp`,
  etc., not `total_traded_dollars` / `market_exposure_dollars`). This mismatch predates the
  2026-06-08 watermark and is out of scope for this refresh (not raised by any of the 140 changelog
  entries covered here), but is flagged here for a future pass. In practice the borrowed
  `MarketPositionRef` only shares fields with the REST shape (`ticker`, `total_traded_dollars`,
  `position_fp`, `market_exposure_dollars`, `realized_pnl_dollars`, `fees_paid_dollars`,
  `last_updated_ts`) and would silently fail to populate the WS-specific fields (`user_id`,
  `position_cost_dollars`, `position_fee_cost_dollars`, `volume_fp`, `subaccount`) that the exchange
  actually sends on this channel.

- **Deferred (not implemented in this refresh)**, documented here rather than silently dropped:
  the `pyth_value` and `cfbenchmarks_value_5hz` WebSocket channels (2026-07-23, 2026-09-03); the
  `GET /live_data/events/{event_ticker}` and `GET /live_data/weather/{city}` (+
  `/calibrations`) REST endpoints (2026-07-30, 2026-08-20, 2026-08-31); `POST`/`GET
  /portfolio/target_balance_allocation` and the `resting_margin_reservation` setting
  (2026-08-20/2026-09/2026-09-24); `POST /portfolio/intra_exchange_instance_transfer` and the
  `GET .../intra_exchange_instance_transfers[/​{id}]` history endpoints (2026-08-13/2026-08-20); the
  new cancel-all-resting-orders endpoints (2026-08-27); and full Ed25519 request signing (see the
  `ApiKey` note above). These are all additive, non-breaking surface to add in a future patch/minor
  release and do not affect any currently-modeled behavior.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
