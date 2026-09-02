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

- The legacy `/portfolio/orders` order-mutation surface (`POST` create, `DELETE` cancel, `.../amend`,
  `.../decrease`, `.../orders/batched` create and cancel) was removed from the OpenAPI spec (announced
  deprecated 2026-06-18, gone by this refresh). `GET /portfolio/orders` and `GET
  /portfolio/orders/{order_id}` remain. The crate removed `create_order`, `cancel_order`,
  `amend_order`, `decrease_order`, `batch_create_orders`, `batch_cancel_orders` and their exclusive
  request/response types entirely (0.8.0, breaking) rather than keeping them as dead code; use the V2
  event-order endpoints (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`) instead. A new `cancel_all_orders` /
  `CancelAllOrdersParams` covers the new `DELETE /portfolio/events/orders` endpoint.

- `exchange_index` (an integer shard identifier) was added across most REST response objects and
  several WebSocket messages as part of an ongoing exchange-sharding rollout (2026-07/2026-08):
  `Market`, `MarketPosition`, `Fill`, `Settlement`, `Series`, `EventData`, `MultivariateEventCollection`,
  `ExchangeIndexStatus`, `WsUserOrder`, `WsFill`, `WsMarketLifecycleV2`, `WsEventLifecycle`. It is
  modeled as `Option<i64>` everywhere even where the schema marks it required, because records/messages
  that predate sharding (or exist before a market/event is created) legitimately lack it; "exchange
  index 0" is the only index in production as of this writing. Several endpoints also gained an
  `exchange_index` request field for cross-shard auto-routing (`CreateOrderV2Request`,
  `AmendOrderV2Request`, `DecreaseOrderV2Request`, `CancelOrderV2Params`,
  `BatchCancelOrderV2RequestOrder`); these are `Option<i32>` (not `u32`) because `-1` is a valid
  "require auto-routing by ticker" sentinel value, and `DecreaseOrderV2Request` /
  `BatchCancelOrderV2RequestOrder` / `CancelOrderV2Params` additionally gained a `market_ticker` field
  used for that auto-routing when `exchange_index` is omitted or `-1`.

- `ErrorResponse.service` (deprecated 2026-07-28) was removed from the OpenAPI schema and all
  responses by 2026-08-06. The crate removed the field entirely (0.8.0, breaking) rather than keeping
  it `Option`; branch on `code` instead, which is present on every error response and was already the
  documented stable contract.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST schema 2026-07-09 and
  are removed from the crate entirely (0.8.0, breaking) rather than kept as `Option`, per the "don't
  preserve removed fields" refresh policy. `Market.price_level_structure`/`price_ranges` and the
  fixed-point count/dollar fields remain the canonical replacements.

- The multivariate lookup REST endpoints (`PUT`/`GET .../multivariate_event_collections/{ticker}/lookup`)
  and the `multivariate` WebSocket channel (message type `multivariate_lookup`) were removed from the
  API (2026-07-02 through 2026-08-06). The crate removed `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, their exclusive types, and the
  `WsChannelV2::Multivariate` / `WsMsgType::Multivariate(Lookup)` / `WsMultivariate*` types entirely
  (0.8.0, breaking). Use `create_market_in_multivariate_event_collection` to create or resolve a combo
  market, and `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes.

- `GET /exchange/announcements` was removed from the Predictions REST API (2026-07-04). The crate
  removed `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus` entirely (0.8.0, breaking). Use
  `get_exchange_schedule` for exchange hours.

- `GET /communications/quotes` no longer accepts `market_ticker` or `event_ticker` filters
  (removed 2026-06-20); the crate removed those two fields from `GetQuotesParams` (0.8.0, breaking).
  `min_ts`/`max_ts` filters and a `user_filter` (distinct from the existing `rfq_user_filter`) were
  added in their place. The RFQ-scoped quote actions (`get_rfq_quote`, `delete_rfq_quote`,
  `accept_rfq_quote`, `confirm_rfq_quote`) were added alongside the deprecated quote-ID-only methods
  (`get_quote`, `delete_quote`, `accept_quote`, `confirm_quote`, now `#[deprecated]`), which the
  OpenAPI spec still documents (`deprecated: true`) but has not removed.

- `pyth_value` is a new authenticated AsyncAPI channel (2026-07-23) that delivers deduplicated Pyth
  prices by underlying ticker. It mirrors the `cfbenchmarks_value` pattern: `underlying_tickers` (not
  market tickers) for subscription parameters, `["all"]` for every available underlying, and
  `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` actions on
  `update_subscription_v2` for post-subscribe add/remove/discovery, mirroring
  `SubscribeIndices`/`UnsubscribeIndices`/`Indexlist`.

- `GetBalanceResponse` gained `balance_breakdown: Vec<IndexedBalance>` (per-exchange-index balances,
  2026-07-02) and `get_balance` now takes `GetBalanceParams { subaccount, exchange_index }` (0.8.0,
  breaking) instead of no arguments, to scope the read to one exchange index (2026-08-13).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
