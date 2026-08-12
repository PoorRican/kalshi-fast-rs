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

- **Legacy V1 order mutation endpoints were removed from the OpenAPI spec (0.7.0).** As of this
  refresh, `POST /portfolio/orders`, `DELETE /portfolio/orders/{order_id}`,
  `POST /portfolio/orders/{order_id}/amend`, `POST /portfolio/orders/{order_id}/decrease`,
  `POST /portfolio/orders/batched`, and `DELETE /portfolio/orders/batched` no longer appear in the
  spec at all (Kalshi announced deprecation 2026-06-18; the paths are now gone, not merely marked
  `deprecated: true`). `create_order`, `cancel_order`, `amend_order`, `decrease_order`,
  `batch_create_orders`, `batch_cancel_orders`, and their request/response types were removed from
  the crate (breaking, 0.6.0 → 0.7.0). Use the V2 equivalents (`create_order_v2`, `cancel_order_v2`,
  `amend_order_v2`, `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`), which
  use `BookSide` + a single fixed-point `price` instead of separate yes/no price + side + action.
  `GET /portfolio/orders`, `GET /portfolio/orders/{order_id}`, and the queue-position endpoints are
  unaffected and remain modeled as before.

- **`GET /exchange/announcements` was removed from the OpenAPI spec (0.7.0).** `get_exchange_announcements`,
  `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, and `AnnouncementStatus`
  were removed from the crate (breaking). `GetExchangeStatusResponse` gained
  `intra_exchange_transfers_active: Option<bool>` and `exchange_index_statuses:
  Option<Vec<ExchangeIndexStatus>>` for per-shard status (added to the spec 2026-07-02, `description`
  field added 2026-08-07).

- **The multivariate ticker-lookup surface was removed from both specs (0.7.0).** The
  `PUT/GET .../multivariate_event_collections/{collection_ticker}/lookup` REST endpoints and the
  `multivariate` WebSocket channel (`multivariate_lookup` message type) were removed by Kalshi on
  2026-08-06 (lookup history was already fully deprecated 2026-07-02). `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, their request/response types, `WsChannelV2::Multivariate`,
  `WsMultivariate`/`WsMultivariateRef`, and the `multivariate_lookup` message-type variants were
  removed from the crate (breaking). The `multivariate_market_lifecycle` channel and
  `create_market_in_multivariate_event_collection` (create/resolve) are unaffected.

- `Market.response_price_units`, `Market.fractional_trading_enabled` (REST and WS
  `market_lifecycle_v2`), and `MarketPosition.resting_orders_count` (REST and WS) were removed from
  both specs 2026-07-09 ("Deprecated Predictions REST schema fields removed") and are no longer
  modeled (breaking). `price_level_structure`/`price_ranges`/the fixed-point fields remain the
  canonical replacements.

- `ErrorResponse.service` was deprecated 2026-07-28 and removed from all error responses 2026-08-06.
  It is kept as a `#[deprecated]` `Option<String>` field (rather than removed outright) since it is
  harmless to retain and some cached/historical payloads may still carry it; branch on `code`
  instead, which is present on every error response.

- `exchange_index` (`Option<u32>`, defaulting to 0) was added across most REST response/request
  shapes during 2026-06/07/08 as Kalshi rolled out multi-shard exchange support: `Series`, `EventData`,
  `MultivariateEventCollection`, `Order`, `OrderGroup` and its create/get responses, `SubaccountBalance`,
  `SubaccountTransfer`, `SubaccountNettingConfig`, and the WS `market_lifecycle_v2` /
  `event_lifecycle` creation payloads (`WsMarketLifecycleV2`, `WsEventLifecycle`). Request-side
  `exchange_index` fields that route by market ticker (`CreateOrderRequest` in the V2 create/amend/
  decrease/cancel/batch-cancel order bodies) are modeled as `Option<i32>` instead, because the API
  accepts `-1` there as an "auto-route by ticker" sentinel; the sibling `market_ticker` field becomes
  required when `-1` is used. `GetBalanceResponse` gained a `balance_breakdown:
  Option<Vec<IndexedBalance>>` per-instance breakdown (2026-08-13), and `get_balance` now takes a
  `GetBalanceParams { subaccount, exchange_index }`.

- `event product_metadata.cadence` (added 2026-07-30) is not a distinct field on `EventMetadata`;
  it round-trips through the existing `#[serde(flatten)] extra: Map<String, Value>` catch-all, so no
  code change was needed for it.

- New `price_level_structure` string values (seven added 2026-07-23, `center_deci_edge_centi_cent`
  added 2026-08-13, `center_centi_edge_centi_cent` for combo markets scheduled 2026-08-17) require no
  code change: the field is modeled as `Option<String>` (not an enum) precisely so new structure
  names round-trip without a crate update. Consumers should key off `price_ranges`
  (`{start, end, step}` bands), not the structure name.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
