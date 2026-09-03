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

## 2026-09 Refresh (Exchange Sharding, Order V2 Migration)

This refresh reconciled the crate against Kalshi's live OpenAPI (3.29.0) and AsyncAPI (2.0.0,
AsyncAPI spec format 3.0.0) documents and the full changelog history since the prior
`Validated through changelog: 2026-06-08` watermark. The single largest theme is **exchange
sharding**: Kalshi is rolling out multiple exchange indexes (shards) per exchange instance, and has
added an `exchange_index` field across most portfolio/market schemas.

### Exchange sharding (`exchange_index`)

- Added `exchange_index` to: `Market`, `Series`, `EventData`, `MultivariateEventCollection`,
  `Order`, `OrderGroup`/`GetOrderGroupResponse`/`CreateOrderGroupResponse`, `Fill` (REST + WS),
  `MarketPosition`, `Settlement`, `WsUserOrder`, `WsMarketLifecycleV2` (top-level, present only on
  `created` events), `SubaccountBalance`, `SubaccountTransfer`, `SubaccountNettingConfig`.
- Where the OpenAPI schema lists `exchange_index` as **required** (`Fill`, `MarketPosition`,
  `Settlement`, and the three `Subaccount*` schemas), the crate still models it as a bare
  `#[serde(default)] i64` rather than `Option<i64>`. Rationale: the documented default shard is `0`,
  serde's numeric default is also `0`, so a payload from a not-yet-sharded environment or an older
  cached fixture still parses correctly and produces the semantically correct value. This is a
  deliberate deviation from the "required-but-conditional → `Option`" rule for fields whose zero
  value **is** the documented default, to avoid forcing every caller to unwrap a field that is
  effectively always present.
- Where `exchange_index` is present but **not** in the schema's `required` list (`Order`,
  `OrderGroup`, `GetOrderGroupResponse`, `CreateOrderGroupResponse`, `WsMarketLifecycleV2`,
  `WsUserOrder`), it is modeled the same way (`Order`/`OrderGroup`/`*Response` as bare `i64`
  default-0; the two WS message types as `Option<i64>` since the AsyncAPI explicitly scopes them to
  specific event types, i.e. genuinely conditional, not just newly-added).
- `GetBalanceResponse` gained `balance_breakdown: Option<Vec<IndexedBalance>>` and `get_balance` now
  takes a `GetBalanceParams { subaccount, exchange_index }` (was previously argument-less) to scope
  the read to one shard.
- `GetPositionsParams`, `GetFillsParams`, `GetOrdersParams` gained an `exchange_index: Option<i64>`
  filter (`ExchangeIndexFilterQuery` in the spec).
- `SubaccountQueryParams` (shared by the four order-group mutation endpoints — delete/reset/
  trigger/update-limit — and reused harmlessly elsewhere since it's omitted when `None`) gained
  `exchange_index: Option<i64>` (`ExchangeIndexQuery`, default 0).
- New account-level surface: `intra_exchange_instance_transfer` / `get_intra_exchange_instance_transfers`
  / `get_intra_exchange_instance_transfer` (`POST/GET /portfolio/intra_exchange_instance_transfer(s)`)
  and `get_target_balance_allocation` / `set_target_balance_allocation`
  (`GET/POST /portfolio/target_balance_allocation`), both added 2026-08-13/2026-08-20.
- `GetExchangeStatusResponse` gained `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` and
  `intra_exchange_transfers_active` (2026-07-02, "per-index exchange status").
- `GetPortfolioRestingOrderTotalValueResponse` gained `resting_order_value_breakdown:
  Vec<IndexedBalance>` (2026-08-20).

### Legacy `/portfolio/orders` mutation endpoints deprecated (2026-06-18/25)

Kalshi deprecated the legacy order-mutation endpoints in favor of the V2 event-order endpoints;
calls to the legacy endpoints now return an error asking callers to migrate. The legacy operation
IDs (`CreateOrder`, `CancelOrder`, `AmendOrder`, `DecreaseOrder`, `BatchCreateOrders`,
`BatchCancelOrders`) no longer appear in the live OpenAPI document at all. Rather than delete
`create_order` / `cancel_order` / `amend_order` / `decrease_order` / `batch_create_orders` /
`batch_cancel_orders` outright (a large, disruptive break for a shape that still round-trips an
`Err(KalshiError::Http{..})` the caller can act on), they are kept but marked `#[deprecated]`
pointing at the V2 equivalents (`create_order_v2`, etc., already added in 0.6.0). The
`examples/place_order.rs` flagship example was updated to use `create_order_v2`. Added
`cancel_all_orders_v2` (`DELETE /portfolio/events/orders`, 2026-08-27).

### Multivariate lookup surface removed (2026-08-06)

Kalshi fully removed the deprecated multivariate lookup surface:
`PUT /multivariate_event_collections/{collection_ticker}/lookup`,
`GET .../lookup` (lookup history), and the WebSocket `multivariate` channel
(message type `multivariate_lookup`) no longer exist — subscribing to the channel now returns an
unknown-channel error. Per the crate's refresh policy (do not preserve removed upstream surface as
a compatibility shim), these were deleted outright:
`lookup_tickers_for_market_in_multivariate_event_collection`,
`get_multivariate_event_collection_lookup_history`, their request/response types (`LookupPoint`,
`GetMultivariateEventCollectionLookupHistoryParams/Response`,
`LookupTickersForMarketInMultivariateEventCollectionRequest/Response`), `WsChannelV2::Multivariate`,
`WsMsgType::Multivariate`/`MultivariateLookup`, and `WsMultivariate`/`WsMultivariateRef`
(`src/ws/types/messages/multivariate.rs` removed). Use
`POST /multivariate_event_collections/{collection_ticker}` to create/resolve a combo market, or the
communications (RFQ) APIs for quoting workflows. Use `WsChannelV2::MultivariateMarketLifecycle` for
multivariate market state — that channel is unaffected and still exists.

### `GET /exchange/announcements` removed (2026-07-04)

The endpoint, `get_exchange_announcements`, and the `Announcement`/`AnnouncementType`/
`AnnouncementStatus`/`GetExchangeAnnouncementsResponse` types were removed; the operation no longer
appears in the live OpenAPI document.

### New WebSocket channels

- `cfbenchmarks_value_5hz` (2026-09-02): CF Benchmarks index values at up to 5 updates/sec, raw tick
  data only (no windowed averages, unlike the once/sec `cfbenchmarks_value` channel). Reuses the same
  `index_ids` / `subscribe_indices` / `unsubscribe_indices` / `indexlist` subscription-update
  mechanism as `cfbenchmarks_value`.
- `pyth_value` (2026-07-23): real-time Pyth prices for configured underlying tickers. Requires
  authentication (`WsChannelV2::PythValue.is_private() == true`). New subscription field
  `underlying_tickers: Option<Vec<String>>` and update actions `SubscribeUnderlyings` /
  `UnsubscribeUnderlyings` / `UnderlyingList`, mirroring the CF Benchmarks index-ID pattern.

### Other field/type additions

- `WsTrade`/`WsTradeRef` gained `is_block_trade: bool` (2026-08-13), mirroring the REST `Trade`
  field added in 0.6.0.
- `WsFill`/`WsFillRef` gained `exchange_index: i64`. `purchased_side` is now marked
  `deprecated: true` in the AsyncAPI (past its original "not removed before May 14, 2026" date, same
  pattern as `side`/`action` elsewhere in this crate) — kept as a required field (spec still requires
  it) but annotated `#[deprecated]` pointing at `outcome_side`/`book_side`.
- `WsUserOrder` gained `exchange_index: Option<i64>` (2026-08-27).
- `WsMarketLifecycleV2`/`Ref` gained top-level `exchange_index` (created events only), `price_ranges`
  (created + `price_level_structure_updated` events), and `strike_type`/`cap_strike`/`custom_strike`
  (metadata_updated events only) — same "top-level key present only for specific event types"
  pattern already used for `floor_strike`/`yes_sub_title`.
- `WsQuoteCreated`/`Ref` gained `rfq_creator_id: Option<String>` and `subaccount: Option<i64>`
  (2026-07-30; present only when the caller's side of the quote used a subaccount).
- `EventData` gained `settlement_sources: Vec<SettlementSource>` (distinct from the nested
  `product_metadata.settlement_sources`), `fee_type_override`/`fee_multiplier_override`
  (2026-07-30), and `exchange_index`. `available_on_brokers` is now `deprecated: true` in the OpenAPI
  ("no longer populated and always returns false"; 2026-08-27) — kept `Option<bool>` (shape
  unchanged) but annotated `#[deprecated]`.
- `ErrorResponse.service` was removed from the OpenAPI schema (deprecated 2026-07-28, removed
  2026-08-06). No shape change needed (`service` was already `Option<String>`); annotated
  `#[deprecated]`. `rest::retry::build_http_error`'s `error.service.is_some()` check becomes a
  permanently-false OR branch — harmless (the other three fields still detect a populated error
  body) and left as-is rather than touched as an unrelated cleanup.
- `price_level_structure` (REST `Market`, WS lifecycle) is a raw `Option<String>`, not an enum, so
  the new `center_deci_edge_centi_cent` variant (2026-08-13) round-trips with **no code change**.
- `GetQuotesParams`: removed `market_ticker`/`event_ticker` (removed from the live OpenAPI
  2026-06-20 — "RFQ quote market and event filters removed"), added `min_ts`/`max_ts` and
  `user_filter` (2026-06-18/2026-05-01), marked `quote_creator_user_id`/`rfq_creator_user_id` as
  Kalshi-deprecated in a doc comment (still present, spec still allows them).
- Added RFQ-scoped quote endpoints (2026-07-09): `get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote` (`/communications/rfqs/{rfq_id}/quotes/{quote_id}[...]`),
  mirroring the existing non-RFQ-scoped quote endpoints.
- `GetFcmOrdersParams.subtrader_id` changed from required `String` to `Option<String>`, and gained
  `client_order_ids: Option<String>` — at least one of the two is now required by the API
  (2026-09-03).
- `GetEventsParams` gained `event_tickers: Option<Vec<String>>` (serialized as `tickers`, CSV;
  2026-06-18) and `min_updated_ts: Option<i64>`.
- New endpoints: `get_historical_positions` (`GET /historical/positions`, reuses
  `GetPositionsResponse`; 2026-07-23), `get_event_live_data` (`GET /live_data/events/{event_ticker}`;
  2026-07-30), `get_weather_index` / `get_weather_index_calibrations`
  (`GET /live_data/weather/{city}[/calibrations]`; 2026-08-20/31 — per-station audit detail is kept
  as generic `Vec<serde_json::Value>` since it is diagnostic detail, not central to trading),
  `get_account_api_usage_level_volume_progress` / `upgrade_account_api_usage_level`
  (`GET /account/api_usage_level/volume_progress`, `POST /account/api_usage_level/upgrade`;
  2026-06-11), `create_subaccount` now takes a `CreateSubaccountRequest { exchange_index }` body
  (was argument-less).

### No code change needed (verified against the live spec, not merely assumed)

- `cadence` on event `product_metadata`, "localized market content", and "structured target images"
  do not appear as distinct schema properties in the live OpenAPI — `product_metadata` and
  `StructuredTarget.details` are already generic `Map<String, Value>` in this crate, so any new keys
  Kalshi adds there round-trip losslessly with no crate change.
- The CF Benchmarks "REST passthrough" (`/cfbenchmarks/rest-passthrough`) is a docs-only page with no
  entry in the structured OpenAPI document; nothing to model type-safely.
- Margin-exchange and FIX-only changelog entries (perps mark prices, margin fee tiers, margin order
  groups, FIX tag/session changes, etc.): margin market types and FIX are not modeled by this crate.
- Pure rate-limit/operational changelog entries (token-cost changes, tier-qualification threshold
  changes, RFQ/quote retention-window changes, order-group count limits): these don't change any
  request/response shape.
- Sub-account-restricted API key permission-scoping entries: scopes are already stored as
  `Vec<String>`; no crate-visible shape change.

## Version Bump

Applied per `VERSIONING.md`: **minor** (0.7.0 → 0.8.0). Multiple intentional breaking Rust API
changes are included in this refresh: `get_balance` and `create_subaccount` gained required
parameters, `update_order_group_limit` gained a parameter, `GetFcmOrdersParams.subtrader_id` changed
type, `GetQuotesParams` lost two fields, `WsChannelV2`/`WsMsgType`/`WsDataMessageV2`/`WsDataMessageRef`
lost the `Multivariate`/`MultivariateLookup` variants, and several REST/WS structs gained new
non-`Option` fields that break exhaustive struct-literal construction. Per VERSIONING.md, "any
intentional breaking change to the public Rust API" while pre-1.0 is a minor bump, not a patch.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
