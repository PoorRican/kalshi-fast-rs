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

- `exchange_index` identifies the exchange shard a record lives on. Kalshi rolled it out across
  REST and WebSocket payloads through 2026-07/08 (series 07-30, events and lifecycle creation
  messages 07-30, multivariate collections 08-06, fills/settlements/market positions and WS fills
  08-20, `user_orders` 08-27). Every occurrence is modeled as `Option` even where the OpenAPI marks
  it required: the rollout was staged per endpoint, and the AsyncAPI documents it as "present ONLY
  when the market is created" on `market_lifecycle_v2`. Matching `exchange_index` filters exist on
  `GET /portfolio/orders`, `/positions`, and `/fills` (`Option<i32>`, omit for all shards).

- `GET /portfolio/balance` returns totals across every exchange index by default; passing
  `exchange_index` scopes both `balance` and `portfolio_value` (2026-08-20). `get_balance()` keeps
  the all-index default and `get_balance_scoped(GetBalanceParams { .. })` exposes the scoped form,
  so adding the parameters did not change the existing method's signature. `balance_breakdown` and
  `resting_order_value_breakdown` are `Vec<IndexedBalance>` defaulting to empty, because they are
  omitted entirely for subaccount-restricted API keys.

- `resting_margin_reservation` (`max` | `sum`) is the collateral policy an automatic shard rebalance
  leaves for resting orders. `GET /portfolio/target_balance_allocation` returns it even when
  `allocations` is empty, and `POST` defaults it to `sum` when omitted — so read the current value
  first if you intend to preserve it rather than reset it.

- The quote-ID-only communications endpoints (`GET`/`DELETE /communications/quotes/{quote_id}` and
  its `/accept`, `/confirm` variants) were deprecated on 2026-06-25 / 2026-07-09 in favour of the
  RFQ-scoped paths. Both families are kept: the deprecated methods (`get_quote`, `delete_quote`,
  `accept_quote`, `confirm_quote`) still work and are documented as deprecated, while
  `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote` take `rfq_id` and
  are preferred. Kalshi has said `rfq_id` will become required in a future migration, and supplying
  it already lowers the rate-limit cost of quote confirmation (2026-09-17). Note also that since
  2026-06-25 only `accepted` / `confirmed` / `executed` quotes are durably queryable; open and
  cancelled quotes are best-effort and may 404 after a server roll.

- `GET /communications/quotes` dropped the `market_ticker` and `event_ticker` filters on 2026-06-20
  (effective immediately, no deprecation window), so they are removed from `GetQuotesParams` rather
  than kept as no-ops. Filter by `rfq_id`, `status`, `user_filter` / `rfq_user_filter`, or the
  `min_ts` / `max_ts` last-update window added on 2026-06-18.

- `target_cost_excludes_fees` on `CreateRFQRequest` (2026-09-10) flips target-cost sizing from
  fee-inclusive (the default: derived contract counts shrink so principal *plus* Kalshi fees stay
  inside the cash amount) to principal-only (contracts = target cost / price, taker fee charged on
  top). It is `Option<bool>` and omitted when unset so the default behaviour is unchanged, and the
  exchange rejects it when no target cost is supplied.

- `GET /exchange/announcements` was removed from the Predictions REST API on 2026-07-04. The
  endpoint, `GetExchangeAnnouncementsResponse`, `Announcement`, `AnnouncementType`, and
  `AnnouncementStatus` are removed rather than kept as a dead surface. Maintenance windows remain
  available through `GET /exchange/schedule`.

- The `service` field was removed from REST error bodies on 2026-08-06 (deprecated 2026-07-28).
  `ErrorResponse.service` is removed; branch on `code`, which is present on every error response.
  `details` now also carries the offending market tickers as a comma-separated string on
  multivariate combo-validation failures (2026-07-30).

- The multivariate *lookup* surface was removed on 2026-08-06:
  `PUT`/`GET /multivariate_event_collections/{collection_ticker}/lookup` no longer exists, and the
  `multivariate` WebSocket channel (message type `multivariate_lookup`) now returns an
  unknown-channel error. `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`,
  `WsMsgType::MultivariateLookup`, `WsMultivariate`, and the lookup REST types are removed. Use
  `POST /multivariate_event_collections/{collection_ticker}` to create or resolve a combo market,
  the communications (RFQ) APIs for quoting, and `multivariate_market_lifecycle` for state changes.
  A stray `multivariate_lookup` frame now surfaces as `WsMessageV2::Unknown` with its `sid`/`seq`
  preserved.

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions schema on 2026-07-09 and
  are removed from the Rust types. `fractional_trading_enabled` and the corresponding
  `WsMarketLifecycleEventType::FractionalTradingUpdated` lifecycle variant are likewise gone from
  the AsyncAPI and from `WsMarketLifecycleV2`. `Market.price_level_structure` and
  `Market.price_ranges` are the canonical replacements for tick-size logic.

- `available_on_brokers` was deprecated on event responses on 2026-08-27 and removed on 2026-09-10
  (it had returned `false` unconditionally since August). `EventData.available_on_brokers` is
  removed; payloads that still carry the key fall through to the `extra` flatten map.

- Price-level structures keep expanding (`center_whole_edge_half_cent` and six siblings on
  2026-07-23, `center_deci_edge_centi_cent` on 2026-08-13, all combo markets migrated to the latter
  on 2026-09-03). None of these add fields. `price_level_structure` therefore stays a raw `String`
  on both `Market` and `WsMarketLifecycleV2`, and callers must snap order and RFQ quote prices to
  the `step` of the band containing the price in `price_ranges` rather than keying off the structure
  name. Sub-cent prices need all four decimals of the `*_dollars` fields; the integer-cent fields
  cannot represent them. `price_ranges` is now also pushed inline on `market_lifecycle_v2` `created`
  and `price_level_structure_updated` events (2026-07-02) as `WsLifecyclePriceRange`, so no
  follow-up REST call is needed when a market's tick grid changes.

- `metadata_updated` lifecycle events carry the updated strike information at the **top level** of
  the payload, not under `additional_metadata`. Since 2026-06-18 that includes `strike_type`,
  `cap_strike`, and `custom_strike` alongside the existing `floor_strike` and `yes_sub_title`, and
  the event now also fires on `cap_strike` / `strike_type` changes. All five are surfaced as
  top-level `Option` fields on `WsMarketLifecycleV2` (distinct from the `additional_metadata.*`
  copies emitted on creation) so a full strike range can be reconstructed from the push alone.

- `Series.categories` (2026-09-17) is the list of discovery categories; `Series.category` remains
  the *primary* category. The `category` filter on `GET /series` matches any entry in `categories`,
  so a series returned for `category=Commodities` may report a different primary `category`. The
  field is `Vec<String>` defaulting to empty for payloads predating it.

- `orders_updated_ts` on `GET /historical/cutoff` advances independently of the other cutoffs since
  2026-09-24 and sits roughly two weeks behind the present. Read it before querying orders rather
  than inferring it from `trades_created_ts`. `market_positions_last_updated_ts` (2026-07-23) is the
  matching cutoff for `GET /historical/positions`; it is `Option` because it postdates the other
  three.

- Weather index points (`GET /live_data/weather/{city}`) model `v` as `Option<f64>`: on
  `incomplete` points the value is genuinely **absent, not `0`**, and minutes that failed the index
  quorum are omitted entirely, so gaps in `timeseries` are real gaps. `receipt_basis`
  (2026-09-10) is present only on historical-backfill points whose receipt deadline was judged
  against `observation_time + Synoptic ingest latency`; **those points are not settlement-eligible**.
  `GET /live_data/weather/{city}/calibrations` (2026-08-31) returns the configuration timeline, in
  Celsius, needed to reproduce a value under the `config_version` that computed it.

- `cfbenchmarks_value_5hz` (2026-09-03) is a sibling of `cfbenchmarks_value`, not a replacement. It
  streams lean raw ticks (`index_id`, `value_usd`, `source_ts_ms`, `received_at`, `data`) at up to
  five updates per second on the indices the vendor publishes at 200ms granularity, and carries
  **no** rolling averages — `avg_60s_data` and `last_60s_windowed_average_15min` stay on the
  once-per-second channel. Its enum variant needs an explicit `#[serde(rename)]` because
  `snake_case` would render `CfbenchmarksValue5Hz` as `cfbenchmarks_value5_hz`.

- `pyth_value` (2026-07-23) streams deduplicated Pyth prices keyed by underlying ticker. Unlike
  every other channel it targets `underlying_tickers` (use `["all"]` for everything), managed
  through `update_subscription_v2` with the `subscribe_underlyings` / `unsubscribe_underlyings` /
  `underlying_list` actions. `validate_update` mirrors the CF Benchmarks index rules: underlying
  actions reject market targets, and the add/remove actions require `underlying_tickers`
  (AsyncAPI error code 28).

- `cfbenchmarks_value`, `cfbenchmarks_value_5hz`, and `pyth_value` all require an authenticated
  session per the AsyncAPI, so `WsChannelV2::is_private()` now reports `true` for all three. The
  Pyth feed additionally requires a paid Kalshi data subscription.

- WebSocket error frames carry `sid` and `seq` when the error is scoped to a subscription
  (documented 2026-09-10). The crate already surfaced these from the envelope, so no shape change
  was needed. Error codes 6, 16, and 17 are retired — the service never emits them and the numbers
  stay reserved — while codes 23–28 are live. `WsError.code` stays a plain `i64` rather than an
  enum so retired and future codes round-trip without a crate update.

- `is_block_trade` reached WebSocket trade messages on 2026-08-13, roughly two months after the
  REST `Trade` object (2026-05-29). `WsTrade.is_block_trade` is `bool` with `#[serde(default)]`,
  matching the REST field, so pre-flag payloads parse as `false`.

- `GET /fcm/orders` requires at least one of `subtrader_id` or `client_order_ids` since 2026-09-03,
  so `GetFcmOrdersParams.subtrader_id` is now `Option<String>`. A `client_order_ids` lookup searches
  only orders created in the last 24 hours and silently raises an earlier `min_ts` to that bound;
  client order IDs are unique only within a subtrader among live and recent orders, so one ID can
  match orders across subtraders or across time.

- `DELETE /portfolio/events/orders` (`cancel_all_orders`, 2026-08-27) cancels every resting
  event-market order across all shards. Omitting `subaccount` makes orders from *any* subaccount
  eligible — it does not mean "primary only". Newly placed orders may also be cancelled during the
  minute after the request, so it is not safe to place-then-assume immediately afterwards.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
