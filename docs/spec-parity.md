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

- The multi-exchange-shard rollout (2026-06 through 2026-09) added `exchange_index` to many REST and
  WebSocket response shapes. The AsyncAPI marks it `required` on `fill`, `event_lifecycle`, and
  `user_order` messages, but it is kept `Option<u32>` everywhere in this crate (REST and WS alike)
  because it is a still-rolling-out field observed inconsistently across exchange instances during
  this refresh, consistent with this crate's existing defensive-parsing convention for fields the
  spec marks required but the exchange sometimes omits.
- `MarketPosition` is shared between the REST `GetPositionsResponse` and the zero-copy WS
  `market_positions` channel view (`MarketPositionRef` in `ws::types::mod`). `resting_orders_count`
  was removed from both call sites together when Kalshi removed it from the OpenAPI schema on
  2026-07-09, since both surfaces map to the same struct.
- The RFQ-scoped quote action endpoints (`get_quote_scoped`, `delete_quote_scoped`,
  `accept_quote_scoped`, `confirm_quote_scoped`) were added 2026-06-25 / 2026-07-09 alongside the
  quote-ID-only endpoints, which Kalshi deprecated but has not removed. The quote-ID-only methods
  are marked `#[deprecated]` rather than removed, since the underlying REST endpoints are still live;
  remove them in a future refresh once Kalshi actually retires the endpoints.
- `GetQuotesParams::market_ticker` / `event_ticker` are marked `#[deprecated]` rather than removed:
  `GET /communications/quotes` stopped honoring these filters on 2026-06-20, but sending them is a
  harmless no-op (the server ignores unrecognized/unsupported query params on this endpoint), so
  removing the fields outright was not required for correctness.
- `pyth_value` and `cfbenchmarks_value_5hz` are new WebSocket channels (2026-07-23 and 2026-09-03)
  implemented following the existing `cfbenchmarks_value` pattern. `pyth_value` uses its own
  `underlying_tickers` subscription field and `subscribe_underlyings` / `unsubscribe_underlyings` /
  `underlying_list` update actions (distinct namespace from `cfbenchmarks_value`'s `index_ids` /
  `subscribe_indices` / `unsubscribe_indices` / `indexlist`); `cfbenchmarks_value_5hz` reuses the
  `index_ids` namespace since the AsyncAPI documents its `update_subscription` command as shared
  with `cfbenchmarks_value`.
- The WebSocket `multivariate` channel and its `multivariate_lookup` message (predates RFQs; Kalshi
  removed it 2026-08-06) were removed entirely rather than kept as dead code, along with the REST
  `.../lookup` endpoints (`GET`/`PUT`) on multivariate event collections. A frame using the old
  `multivariate_lookup` type string now parses as `WsMessageV2::Unknown` — see
  `ws_envelope_into_message_removed_multivariate_lookup_is_unknown` in `ws::types::envelope`.

### Deferred (not implemented this refresh)

The following upstream additions from the 2026-06-08 to 2026-09-10 changelog window are genuinely
new REST surface — not shape drift on an already-modeled endpoint — and were left out of this
refresh to keep it bounded. Each is a candidate for a future refresh:

- Kalshi Weather Index endpoints: `GET /live_data/weather/{city}` (2026-08-20),
  `GET /live_data/weather/{city}/calibrations` (2026-08-31), and the `receipt_basis` field on
  weather index points (2026-09-10). New domain, not previously modeled.
- `GET /trade-api/v2/live_data/events/{event_ticker}` (event-keyed live data, 2026-07-30).
- `POST`/`GET /portfolio/target_balance_allocation` (2026-08-20), including the
  `resting_margin_reservation` parameter added 2026-09-03.
- `POST /portfolio/intra_exchange_instance_transfer` and
  `GET /portfolio/intra_exchange_instance_transfers(/{transfer_id})` (2026-08-13 / 2026-08-20)
  for cross-exchange-shard transfers. Distinct from the already-modeled
  `POST /portfolio/subaccounts/transfer` (single-shard subaccount transfers).
  `PUT /portfolio/order_groups/{order_group_id}/limit`'s `subaccount` / `exchange_index` query-param
  scoping (2026-08-06) is also deferred, since `update_order_group_limit` currently takes no query
  params and adding them is a breaking signature change best bundled with the transfer work above.
- `GET /account/api_usage_level/volume_progress` and
  `POST /account/api_usage_level/upgrade` (2026-06-11).
- `client_order_ids` filter on `GET /fcm/orders` (2026-09-03).
- CF Benchmarks REST passthrough (historical index values via `GET /trade-api/v2/cfbenchmarks/*`,
  documented 2026-08-27) — the live `cfbenchmarks_value(_5hz)` WebSocket channels are implemented,
  but the REST historical-lookback endpoint is not.
- `Accept-Language`-based localized market content (2026-08-27) — the REST client does not currently
  expose a way to set custom request headers per call.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
