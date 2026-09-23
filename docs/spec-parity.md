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

## 2026-06-08 → 2026-09-24 refresh

- **Legacy order-mutation endpoints removed.** `POST /portfolio/orders`, `DELETE
  /portfolio/orders/{id}`, `.../amend`, `.../decrease`, and `/portfolio/orders/batched` were
  deprecated 2026-06-18 and are gone from the current OpenAPI spec entirely (only `GET
  /portfolio/orders` and `GET /portfolio/orders/{id}` remain). `create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders`, and their
  request/response types were removed from the crate. Use the V2 event-order endpoints
  (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead — this is a breaking change
  (minor bump, 0.7.0 → 0.8.0 per `VERSIONING.md`).
- **`multivariate` WebSocket channel and `multivariate_lookup` message type removed** (2026-08-06),
  along with the REST `PUT .../multivariate_event_collections/{ticker}/lookup` and `GET
  .../lookup` (lookup-history) endpoints. `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `MultivariateLookup`, `WsMultivariate(Ref)`, `WsMultivariateSelectedMarket(Ref)`,
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their request/response types were all
  removed. `multivariate_market_lifecycle` is unaffected — it is a distinct channel that already
  reused `WsMarketLifecycleV2`.
- **`service` field removed from REST error bodies** (deprecated 2026-07-28, removed 2026-08-06).
  `ErrorResponse.service` was removed; branch on `code` instead (present on every error response
  already). The internal "was this a structured API error" heuristic in `rest/retry.rs` no longer
  checks for it.
- **Schema fields removed 2026-07-09** (confirmed absent from the current OpenAPI): `Market
  .response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition
  .resting_orders_count`. All three were already `Option` in the crate, so removing them is a
  field-removal (not a parse break) — payloads that still send them simply have the value land
  nowhere instead of a typed field.
- **`EventData.available_on_brokers` removed** (deprecated 2026-08-27, removed 2026-09-10, stopped
  being populated even earlier). The field was removed from `EventData`; any legacy value present
  in a payload is captured by the existing `extra` flatten catch-all instead.
- **`GetQuotesParams` reshaped for `GET /communications/quotes`** (2026-06-18 – 2026-06-20):
  `market_ticker` and `event_ticker` filters were removed from the live endpoint (breaking removal
  from the crate's params struct); `min_ts`, `max_ts`, and `user_filter` were added to match the
  current parameter list.
- **`GET /exchange/announcements` removed** (2026-07-04). `get_exchange_announcements` and the
  `GetExchangeAnnouncementsResponse` / `Announcement` / `AnnouncementType` / `AnnouncementStatus`
  types were removed. `GET /exchange/schedule` remains the source for exchange hours.
- **Exchange sharding rollout** (ongoing July–September 2026) added an optional `exchange_index`
  (`ExchangeIndex` = `integer`, shard id) across many REST and WebSocket objects: `EventData`,
  `Series`, `SubaccountBalance`, `MultivariateEventCollection`, REST `Fill` / `Settlement` /
  `MarketPosition`, and the WS `market_lifecycle_v2` (`created` events only), `event_lifecycle`,
  `fill`, and `user_order` messages. The OpenAPI/AsyncAPI mark several of these `required` once the
  shard is known (e.g. `Fill.exchange_index`, `eventLifecyclePayload.msg.exchange_index`,
  `user_order.msg.exchange_index`), but since the rollout was staged over months and older/replayed
  messages may predate it, every `exchange_index` addition in this pass is modeled as `Option<i64>`
  rather than a required field, consistent with this file's existing policy for phased-rollout
  fields. `GetExchangeStatusResponse` gained `intra_exchange_transfers_active` and
  `exchange_index_statuses: Vec<ExchangeIndexStatus>` (each with `exchange_index`, `description`
  added 2026-08-13, `exchange_active`, `trading_active`, `intra_exchange_transfers_active`).
  `GetOrdersParams`, `GetPositionsParams`, and `GetFillsParams` gained an `exchange_index` filter.
  `GetPortfolioRestingOrderTotalValueResponse` gained `resting_order_value_breakdown:
  Vec<IndexedBalance>`.
- **`market_lifecycle_v2` `metadata_updated` events** gained top-level `strike_type`, `cap_strike`,
  and `custom_strike` (2026-06-18), alongside the pre-existing `floor_strike` / `yes_sub_title`
  top-level fields. `created` and `price_level_structure_updated` events gained an optional
  `price_ranges: Vec<PriceRange>` (2026-07-02), reusing the same `{start, end, step}` shape as the
  REST `Market.price_ranges` field.
- **New `price_level_structure` values** (`center_whole_edge_half_cent`,
  `center_deci_edge_centi_cent`, etc., added 2026-07-23 through 2026-09-03) require no crate change:
  `Market.price_level_structure` and the WS equivalent are modeled as raw `String`/`Option<String>`,
  not a closed enum, specifically to tolerate new structure names without a release. Consumers
  should key off `price_ranges` for valid prices, not the structure label — this is the documented
  behavior for the new structures too.
- **`WsTrade.is_block_trade: bool`** added 2026-08-13, mirroring the REST `Trade.is_block_trade`
  added 2026-05-29. Modeled with `#[serde(default)]` despite being `required` in the AsyncAPI, for
  the same forward/backward-compat reason as the REST field.
- **`WsQuoteCreated.subaccount: Option<u32>`** added 2026-07-30 (mirrors `quote_accepted` /
  `quote_executed`, which already carried it).
- **Sub-account-restricted API keys** (2026-07-02 onward): `ApiKey`, `CreateApiKeyRequest`, and
  `GenerateApiKeyRequest` gained `subaccount: Option<u32>` (0–63). `GetApiKeysResponse` gained
  `api_key_region_expiration_ts: Option<i64>` (2026-08-16) — this lives on the response envelope,
  not per-key, per the OpenAPI schema.
- **New account endpoints** (2026-06-11): `get_account_api_usage_level_volume_progress` (`GET
  /account/api_usage_level/volume_progress`) and `upgrade_account_api_usage_level` (`POST
  /account/api_usage_level/upgrade`, empty response body).
- **`update_order_group_limit` gained a `subaccount` query parameter** (2026-08-06); the method
  signature now takes a `SubaccountQueryParams` argument (breaking).
- **Principal-only RFQ sizing**: `CreateRFQRequest`, `RFQ`, and `Quote` gained
  `target_cost_excludes_fees: Option<bool>` (2026-09-10).
- **`GetFcmOrdersParams`**: `subtrader_id` changed from required `String` to `Option<String>`, and
  `client_order_ids: Option<Vec<String>>` (CSV) was added (2026-09-03) — the endpoint now requires
  at least one of `subtrader_id` or `client_order_ids`, so `subtrader_id` can no longer be modeled
  as always-required.
- **Historical fills/orders**: `GetHistoricalFillsParams` / `GetHistoricalOrdersParams` gained
  `min_ts` (2026-09-17) and `subaccount` (2026-09-24).
- **`GET /events` gained `tickers` (CSV, 2026-06-18) and `min_updated_ts` filters** on
  `GetEventsParams`.
- **`Series.categories: Vec<String>`** added 2026-09-17 (the discovery-category list; the existing
  `category` field remains the series' single primary category and the two are not kept in sync by
  the exchange).
- Dead code removed: `MarketPositionRef` / `EventPositionRef` in `ws/types/mod.rs` were unused,
  unreferenced stubs that did not match any real WebSocket payload shape (the actual
  `market_position` message — `WsMarketPosition` / `WsMarketPositionRef` in
  `ws/types/messages/positions.rs` — already modeled the correct fields: `user_id`, `market_ticker`,
  `position_cost_dollars`, `position_fee_cost_dollars`, `volume_fp`, etc.). Removed rather than
  patched, since nothing in the crate or its tests constructed them.
- Not implemented in this pass (tracked as follow-up, not required for parity with the crate's
  existing surface): the `pyth_value` and `cfbenchmarks_value_5hz` WebSocket channels; new REST
  endpoints for target-balance allocation, cancel-all-orders, historical positions, intra-exchange
  transfer history, the Kalshi Weather Index and its calibration history, and event-keyed live data;
  balance/`GET /portfolio/balance` scoping by `exchange_index`; WebSocket `permessage-deflate`
  compression negotiation; per-request `Accept-Language` header support for localized market
  content. None of these change the shape of any type the crate currently models, so leaving them
  out is a coverage gap, not a correctness issue.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
