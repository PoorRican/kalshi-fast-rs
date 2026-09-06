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

- `price_level_structure` (REST `Market`, WS `market_lifecycle_v2`) is deliberately kept as a plain
  `String` rather than a closed Rust enum, even though the AsyncAPI documents an `enum` for it.
  Kalshi has added new structure names several times (`center_whole_edge_half_cent` and six
  siblings in 2026-07, `center_deci_edge_centi_cent` in 2026-08) without any shape change — callers
  must read the market's `price_ranges` array for valid prices/ticks rather than branching on the
  structure name, so a closed enum would only need updating for a value that downstream code
  shouldn't be keying off of anyway.

- The `exchange_index` sharding rollout (2026-06 through 2026-09) touches dozens of REST and
  WebSocket schemas. Every occurrence is modeled as `Option<i64>` (or `Option<u32>` for
  subaccount-shaped fields) even where the OpenAPI spec marks it required (e.g. `MarketPosition`,
  `Fill`, `Settlement`, `event_lifecycle`), because the feature is still rolling out ("for now, all
  exchange indexes are 0" appears repeatedly in the changelog) and older payloads predate the field
  entirely.

- `GET /account/api_usage_level/volume_progress`, `POST /account/api_usage_level/upgrade`,
  `POST /portfolio/intra_exchange_instance_transfer` (+ history endpoints), and
  `POST`/`GET /portfolio/target_balance_allocation` are new Predictions-exchange endpoints added
  during the sharding rollout. They are modeled following the same conventions as existing
  portfolio/account endpoints.

- `pyth_value` (2026-07-23) and `cfbenchmarks_value_5hz` (2026-09-03) are new WebSocket channels.
  `cfbenchmarks_value_5hz` fully reuses the existing `index_ids` seed field and the
  `SubscribeIndices` / `UnsubscribeIndices` / `Indexlist` `update_subscription` actions shared with
  `cfbenchmarks_value` (the AsyncAPI documents both channels under the same action). `pyth_value`
  instead uses `underlying_tickers` to seed an initial subscribe, but this crate does **not** yet
  implement update-subscription actions for it (`subscribe_underlyings` / `unsubscribe_underlyings`
  / `pyth_value_underlying_list`) — to change a `pyth_value` subscription's tracked underlyings,
  resubscribe with a new `underlying_tickers` list rather than mutating the existing one in place.

- `Event.product_metadata` does **not** carry a `cadence` field in the live OpenAPI schema, despite
  a 2026-07-30 changelog entry announcing one ("Event product_metadata now includes cadence"). No
  `cadence` property appears anywhere in the fetched `openapi.yaml`. Treated as not (yet) shipped;
  not modeled. Re-check on the next refresh.

- `GET /margin/fee_tier_rates` (announced 2026-09-03) is, like the existing `/margin/fee_tiers`
  exception, undocumented in the published OpenAPI spec. Unlike `/margin/fee_tiers`, this crate does
  not model it: margin trading endpoints remain out of scope, and there is no OpenAPI shape to
  validate a Rust type against.

- The `multivariate` WebSocket channel (message type `multivariate_lookup`) and the
  `PUT`/`GET .../multivariate_event_collections/{ticker}/lookup` REST endpoints were removed by
  Kalshi on 2026-08-06. They have been deleted from this crate rather than kept as compatibility
  shims. Use `WsChannelV2::MultivariateMarketLifecycle` for multivariate market state changes and
  `POST /multivariate_event_collections/{ticker}` to create/resolve combo markets.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
