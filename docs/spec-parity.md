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

- **Exchange sharding rollout (2026-07/08).** Kalshi is provisioning dedicated exchange instances
  ("shards", identified by `exchange_index`) for high-volume categories (Crypto, Tennis, Baseball
  announced 2026-08-24). `ExchangeIndex` (`i64`) is a shared type alias. `exchange_index` was added
  to: `Market`, `Series`, `MultivariateEventCollection`, `MarketPosition`, `Fill`, `Settlement`,
  `Order`, `EventData`, `SubaccountBalance`, `ApiKey`-adjacent balance types (`IndexedBalance`), the
  WS `market_lifecycle_v2`/`multivariate_market_lifecycle` `created` event and `event_lifecycle`
  message, and the WS `fill` message. `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` and
  `GetBalanceResponse`/balance reads gained an optional `exchange_index` filter/scope (balance and
  portfolio value aggregate across all indexes when omitted). New endpoints:
  `get_target_balance_allocation` / `set_target_balance_allocation`
  (`/portfolio/target_balance_allocation`), `intra_exchange_instance_transfer` and
  `get_intra_exchange_instance_transfer(s)` (`/portfolio/intra_exchange_instance_transfer[s]`,
  supersedes/complements the existing same-index `transfer_subaccount`), and
  `GetExchangeStatusResponse.exchange_index_statuses` (per-shard status, with `description` added
  2026-08-13).

- **Legacy `/portfolio/orders` mutation endpoints** (`create_order`, `cancel_order`, `amend_order`,
  `decrease_order`, `batch_create_orders`, `batch_cancel_orders`) were announced deprecated
  2026-06-18 ("switch to the V2 endpoints"). They are absent from the current OpenAPI spec and its
  human-readable docs index, but — unlike the exchange-announcements endpoint, the multivariate
  lookup endpoint, and the `service` error field (all of which got explicit "removed" changelog
  entries) — no follow-up "removed" changelog entry has been published for them as of this refresh.
  They are kept, marked `#[deprecated]`, rather than deleted; use `create_order_v2` and friends for
  new code. If a future refresh finds an explicit removal announcement (or the endpoints start
  hard-failing), delete them outright per the standard removed-endpoint policy.

- **Quote-ID-only communications actions** (`get_quote`, `delete_quote`, `accept_quote`,
  `confirm_quote`) were deprecated 2026-06-25/07-09 in favor of RFQ-scoped equivalents
  (`get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`, `confirm_rfq_quote`, all taking
  `rfq_id` + `quote_id`). Unlike the legacy order endpoints above, Kalshi documents these as
  "remain supported for now" with no removal date, so both surfaces are kept; the quote-ID-only
  methods are marked `#[deprecated]`.

- **`GetQuotesParams`** dropped `market_ticker`/`event_ticker` (removed 2026-06-20, confirmed absent
  from the live OpenAPI parameter list); filter by `rfq_id`, status, or the new `min_ts`/`max_ts`
  window (added 2026-06-18) instead. This is a breaking Rust API change (struct fields removed).

- **The WebSocket `multivariate` channel** (message type `multivariate_lookup`) and the REST
  `PUT /multivariate_event_collections/{ticker}/lookup` + `GET .../lookup` (history) endpoints were
  removed 2026-08-06 — both are absent from the live AsyncAPI channel enum and OpenAPI paths, and
  the changelog carries an explicit "no longer exists" removal notice. `WsChannelV2::Multivariate`,
  `WsMsgType::Multivariate`/`MultivariateLookup`, `WsDataMessageV2::Multivariate`, `WsMultivariate`,
  `get_multivariate_event_collection_lookup_history`, and
  `lookup_tickers_for_market_in_multivariate_event_collection` were deleted (breaking). Use
  `create_market_in_multivariate_event_collection` or the communications (RFQ) APIs, and the
  `multivariate_market_lifecycle` channel for lifecycle state.

- **`ErrorResponse.service`** was deprecated 2026-07-28 and confirmed removed from the live schema
  2026-08-06; the field was deleted from the crate (breaking) per the standard removed-field policy.
  Branch on `code`, which is present on every error response.

- **`EventData.available_on_brokers`** was deprecated 2026-08-27 (no longer populated, always
  `false`; not yet removed). The field is kept as `Option<bool>` and marked `#[deprecated]`.

- **New `pyth_value` WebSocket channel** (2026-07-23) mirrors the `cfbenchmarks_value` channel's
  shape: `WsSubscriptionParamsV2::underlying_tickers` seeds the initial subscription,
  `WsUpdateAction::SubscribeUnderlyings`/`UnsubscribeUnderlyings`/`UnderlyingList` plus
  `WsUpdateSubscriptionParamsV2::underlying_tickers` manage it post-subscribe, and
  `WsPythValue`/`WsPythUnderlyingList` (routed through `WsDataMessageV2`) model the two message
  types (`pyth_value`, `pyth_value_underlying_list`).

- **New `price_level_structure` values** (seven `center_{center}_edge_{edge}_cent` variants added
  2026-07-23, plus `center_deci_edge_centi_cent` added 2026-08-13 and adopted for all combo markets
  2026-08-27) require no code change: `Market.price_level_structure` and the WS
  `market_lifecycle_v2` field are both plain `Option<String>`, so new label values round-trip
  losslessly. Always read valid prices from `price_ranges` (`{start, end, step}` in fixed-point
  dollars), never by keying logic off the structure label, per Kalshi's own guidance.

- **Deprecated Predictions REST schema fields removed 2026-07-09**: `Market.response_price_units`,
  `Market.fractional_trading_enabled`, and `MarketPosition.resting_orders_count` were deleted from
  the crate (breaking) — confirmed absent from the live OpenAPI schema and from the `required` set
  it replaced them with (`price_level_structure`, `price_ranges`, and the fixed-point fields).

- `GET /exchange/announcements` was removed 2026-07-04 (`GET /exchange/schedule` remains);
  `get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, and `AnnouncementStatus` were deleted from the crate (breaking).

- **Pre-existing gaps noticed but out of scope for this refresh** (not driven by a changelog entry
  since the 2026-06-08 watermark, so left for a future pass): `GET /portfolio/deposits` and
  `GET /portfolio/withdrawals` (added 2026-05-05); `GET /events/fee_changes`; the series/event-scoped
  candlestick paths (`/series/{series_ticker}/events/{ticker}/candlesticks`,
  `/series/{series_ticker}/markets/{ticker}/candlesticks`); and the block-trade-proposal endpoints
  (`GET /communications/block-trade-proposals`,
  `POST .../block-trade-proposals/{id}/accept` — the 2026-06-18 changelog entry for these was about
  new `read::block_trade_accept`/`write::block_trade_accept` API-key scopes, not new endpoints).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
