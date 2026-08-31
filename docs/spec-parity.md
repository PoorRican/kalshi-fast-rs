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

- Kalshi is mid-rollout on **exchange sharding** (announced 2026-08-24; Crypto, Tennis, and Baseball
  move to dedicated exchange instances). The live specs now attach `exchange_index` to dozens of
  request/response shapes. This refresh added explicit `exchange_index: Option<i64>` fields to the
  structs most load-bearing for a trading adapter — `Order`, `Fill`, `Settlement`, `MarketPosition`,
  `Market`, `Series`, `SubaccountBalance`, WS `WsFill`/`WsUserOrder`/`WsMarketLifecycleV2`/
  `WsEventLifecycle`, and the new `ExchangeIndexStatus` breakdown on `GetExchangeStatusResponse` —
  plus an `exchange_index` filter on `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` and on
  `GetBalanceParams`. Other exchange_index-bearing surfaces (e.g. multivariate event collections,
  API-key location attestation edge cases) still round-trip the field losslessly through each
  struct's `extra` flatten map rather than a typed field; give those explicit fields in a follow-up
  refresh once the sharding rollout finishes and the shape stabilizes.
- `GET /portfolio/balance` gained `subaccount` / `exchange_index` query parameters (2026-08).
  `get_balance` now takes a `GetBalanceParams` (breaking signature change, 0.7.0 → 0.8.0). Passing
  `subaccount: Some(0)` explicitly now scopes to the primary account specifically, distinct from
  omitting it (aggregate). `GetBalanceResponse.balance_breakdown` is the new per-exchange-index view.
- `price_level_structure` is modeled as a plain `String` (not an enum) on `Market` and the
  `market_lifecycle_v2` WS payload, so the several new structure values added since 2026-07-23
  (`center_whole_edge_half_cent`, `center_deci_edge_centi_cent`, etc.) round-trip with no crate
  change. Always read valid prices from the `price_ranges` array rather than the structure name.
- Legacy `/portfolio/orders` mutation endpoints (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were deprecated by Kalshi on
  2026-06-18 in favor of the V2 event-order endpoints (`*_v2`). They still exist in the live OpenAPI
  spec (not removed), so the crate keeps them with a deprecation doc-comment rather than removing
  them.
- The `multivariate` WS channel (message type `multivariate_lookup`) and the REST
  `PUT/GET .../multivariate_event_collections/{ticker}/lookup` endpoints were removed by Kalshi on
  2026-08-06 (deprecated since before this crate's tracked history, fully deprecated 2026-07-02).
  They have been removed from the crate: `WsChannelV2::Multivariate`, `WsMsgType::Multivariate` /
  `MultivariateLookup`, `WsDataMessageV2::Multivariate` (+ `Ref` variants), and
  `get_multivariate_event_collection_lookup_history` /
  `lookup_tickers_for_market_in_multivariate_event_collection` are all gone (breaking, 0.7.0 → 0.8.0).
- `GET /communications/quotes` (`GetQuotesParams`) lost its `market_ticker` / `event_ticker` filters
  on 2026-06-20 (breaking) and gained `min_ts`, `max_ts`, and `user_filter` on 2026-06-18. RFQ-scoped
  quote action endpoints (`get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`,
  `confirm_rfq_quote`) were added 2026-06-25; the quote-ID-only equivalents (`get_quote`,
  `delete_quote`, `accept_quote`, `confirm_quote`) are kept but documented as deprecated, since RFQ
  quotes are no longer guaranteed queryable by ID alone after a server roll.
- `GET /exchange/announcements` was removed from the live OpenAPI spec (last seen before this
  refresh's watermark); `get_exchange_announcements` and the `Announcement`/`AnnouncementType`/
  `AnnouncementStatus`/`GetExchangeAnnouncementsResponse` types have been removed (breaking).
- `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`, and the WS `market_lifecycle_v2`
  `fractional_trading_enabled` field were removed from the live OpenAPI/AsyncAPI schemas
  (2026-07-09) and have been removed from the crate (breaking).
- `pyth_value` is a new AsyncAPI channel (subscription-updatable Pyth price feed by underlying
  ticker) that is not yet modeled in this crate — tracked as a gap for the next refresh rather than
  rushed in this one.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
