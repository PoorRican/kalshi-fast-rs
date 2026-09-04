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

- **2026-06-08 → 2026-09-10 refresh (0.8.0).** The margin exchange (all `/margin/*` endpoints and
  `margin_*` WebSocket fields other than `/margin/fee_tiers`) and the FIX API are explicitly out of
  scope for this crate (REST + WebSocket only, per `CLAUDE.md`); the large volume of Margin- and
  FIX-tagged changelog entries in this window required no code changes. See `CHANGELOG.md` for the
  full per-entry disposition table.

- `PUT /multivariate_event_collections/{ticker}/lookup`, the lookup-history endpoint, and the
  `multivariate` WebSocket channel (`multivariate_lookup` message) were removed 2026-08-06 after
  deprecation on 2026-07-02. All three are removed from the crate; use
  `create_market_in_multivariate_event_collection`, which already returns the resolved market
  ticker, instead of a separate lookup call.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the REST schema 2026-07-09 and are removed
  from the crate. The WebSocket `market_lifecycle_v2` message's `fractional_trading_enabled` field
  (and the corresponding `FractionalTradingUpdated` lifecycle event type) is also gone from the
  current AsyncAPI schema and was removed to match. Two previously-unused, incorrectly-shaped types
  (`ws::types::MarketPositionRef` / `EventPositionRef`, which mirrored the REST `MarketPosition`
  shape rather than the real `market_position` WebSocket payload) were deleted as dead code while
  making this change; the correct type for that channel has always been
  `ws::types::WsMarketPositionRef`.

- The `service` field on REST error bodies was deprecated 2026-07-28 and removed 2026-08-06.
  `ErrorResponse.service` is removed from the crate; branch on `code` instead, which is present on
  every error response.

- `GET /exchange/announcements` was removed from the REST API 2026-07-04. `get_exchange_announcements`
  and the `Announcement*` types are removed from the crate; use `get_exchange_schedule` for exchange
  hours.

- `GET /communications/quotes` no longer accepts `market_ticker` / `event_ticker` filters
  (removed 2026-06-20); `GetQuotesParams` no longer has these fields. `min_ts` / `max_ts` (added
  2026-06-18) replace them for narrowing by time. RFQ-scoped quote actions
  (`get_rfq_quote`/`delete_rfq_quote`/`accept_rfq_quote`/`confirm_rfq_quote`, added 2026-06-25) are
  preferred over the quote-ID-only equivalents, which are marked `#[deprecated]`: per the 2026-06-25
  changelog entry, a quote is no longer guaranteed queryable by ID alone unless it has reached a
  post-acceptance state (`accepted`, `confirmed`, `executed`).

- `exchange_index` was added across the REST and WebSocket surface as Kalshi's exchange sharding
  rolled out (`Fill`, `Settlement`, `MarketPosition`, `IndexedBalance`, `WsFill`, `WsUserOrder`, the
  WebSocket lifecycle messages, etc.). Where the field is required in the live spec on a struct whose
  sibling required fields are already modeled as non-`Option`, it is added as non-`Option`; on
  structs (like `WsUserOrder`) where sibling required fields are already `Option` for resilience, it
  follows that existing convention instead.

- `GetPositionsParams.event_ticker` was corrected from `Option<Vec<String>>` (a CSV of up to 10
  tickers) to `Option<String>`: `GET /portfolio/positions` has only ever accepted a single
  `event_ticker`, unlike `GET /portfolio/orders`, which does accept a CSV of up to 10 and correctly
  keeps `Option<Vec<String>>`. Historical positions (`get_historical_positions`) shares the
  single-ticker shape. `GetFillsParams.event_ticker` was removed outright: `GET /portfolio/fills` has
  never accepted an `event_ticker` query parameter.
- The `subaccount` upper bound used by client-side validation (`GetPositionsParams`,
  `GetOrdersParams`, `CreateOrderRequest`) was corrected from 32 to 63, matching the documented
  0–63 subaccount range everywhere else in the spec (`SubaccountQuery`, `POST /api_keys`, etc.); the
  32 bound was rejecting valid subaccounts 33–63.

- `EventData.product_metadata` and `GET /events/{event_ticker}/metadata` share the `EventMetadata`
  Rust type, but the OpenAPI spec types `product_metadata` as a free-form object with no fixed
  schema. Only fields confirmed to appear there (currently `cadence`, added 2026-07-30) are promoted
  to named fields on `EventMetadata`; anything else lands in `extra`.

- `cfbenchmarks_value_5hz` (added 2026-09-03) is the high-frequency, authenticated sibling of the
  public `cfbenchmarks_value` channel: raw ticks with no windowed-average metadata, up to 5/sec.
  It reuses the `index_ids` subscription mechanic and `WsUpdateAction::SubscribeIndices` /
  `UnsubscribeIndices` / `Indexlist` actions (both channels take the same `index_ids` shape and the
  `indexlist` response shape is identical), modeled by `WsCfBenchmarksValue5Hz` and the existing
  `WsCfBenchmarksIndexList`.
- `pyth_value` (added 2026-07-23) is a new authenticated channel for real-time Pyth prices by
  underlying ticker, using its own `underlying_tickers` subscription field and
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` actions
  (mirroring the CF Benchmarks index-action pattern), modeled by `WsPythValue` /
  `WsPythUnderlyingList`.

- `POST /portfolio/intra_exchange_instance_transfer` and the target-balance-allocation endpoints
  keep `source` / `destination` (`IntraExchangeInstanceTransferRequest` /
  `IntraExchangeInstanceTransfer`) as raw strings rather than a typed `event_contract` | `margined`
  enum, for the same forward-compatibility reason as `ApiUsageLevelGrant.exchange_instance`.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
