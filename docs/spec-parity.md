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

- **2026-07 refresh — `Market` and `EventData` no longer carry fields absent from the live OpenAPI
  schema.** A field-by-field diff against the current `Market` and `EventData` schemas (confirmed
  empirically against live `GET /markets`, `GET /historical/markets`, and `GET /events` payloads)
  found a large set of fields that are not, and likely never were, part of the published contract:
  on `Market` — `market_id`, `resolution_source`, `can_trade`, `can_settle`, `series_ticker`,
  `series_id`, `event_id`, the legacy integer/timestamp fields superseded by `*_dollars`/`*_fp`/
  `*_time` equivalents (`open_ts`, `close_ts`, `settled_ts`, `expiration_ts`, `created_ts`,
  `updated_ts`, `floor_price`, `cap_price`, `yes_bid`, `yes_ask`, `no_bid`, `no_ask`, `price`,
  `last_price`, `price_dollars`, `volume`, `volume_24h`, `open_interest`, `notional_value`,
  `previous_yes_bid`, `previous_yes_ask`, `previous_price`, `liquidity`, `liquidity_fp`,
  `tick_size`, `settlement_value`); on `EventData` — `description`, `status`, `can_trade`,
  `can_settle`, `start_ts`, `close_ts`, `settled_ts`, `series_id`, `mutual_exclusive_group_id(s)`,
  `event_delta`, `volume`, `volume_fp`, `occurrence_datetime`. All were removed rather than kept as
  `Option` per the refresh workflow's default (no documented exception existed for them). Both
  structs keep an `extra` flatten map, so any of these keys still round-trip losslessly if the
  exchange ever sends them — they are just no longer named fields on the public struct. This is an
  intentional breaking change (0.6.0 → 0.7.0); see `CHANGELOG.md`.
- `Market` gained `exchange_index: Option<i64>` and `EventData` gained `settlement_sources:
  Option<Vec<SettlementSource>>`, `fee_type_override: Option<String>`,
  `fee_multiplier_override: Option<f64>`, and `exchange_index: Option<i64>`, all added to the
  OpenAPI spec 2026-06/2026-07. `GetEventsParams` gained `tickers: Option<Vec<String>>` (comma-
  separated event ticker filter) and `min_updated_ts: Option<i64>`.
- `Market.title`, `Market.subtitle`, `Market.expiration_time`, and `Market.liquidity_dollars` are
  marked `#[deprecated]` (Rust attribute, following the existing precedent on
  `WsOrderbookDelta::ts`) because the OpenAPI spec flags them `deprecated: true` while still
  including them in the schema. Likewise `EventData.category` (spec says to prefer the
  series-level category).

- **`response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count`** — the Kalshi changelog (2026-07-03/09) announces these
  Predictions REST fields as removed, and the live OpenAPI spec already omits them, but live
  `GET /markets` payloads as of this refresh (2026-07-05) still return
  `response_price_units`/`fractional_trading_enabled`. Kept as `Option` (already were) rather than
  removed; safe either way once the exchange actually stops sending them.

- **`GET /exchange/announcements` (`get_exchange_announcements`)** was removed from the OpenAPI
  spec (changelog, effective 2026-07-09) in favor of `GET /exchange/schedule`, but the endpoint
  still responds live as of this refresh. The method is marked `#[deprecated]` rather than removed,
  so existing callers keep working (with a warning) until Kalshi actually retires the route.
- **`get_multivariate_event_collection_lookup_history`** is fully deprecated per the Kalshi
  changelog (2026-07-02) and the path is no longer in the OpenAPI spec, but the endpoint still
  responds live (400 for an invalid ticker, not 404). Marked `#[deprecated]` for the same reason as
  `get_exchange_announcements`.

- `GET /communications/quotes` no longer accepts `event_ticker`/`market_ticker` filters (changelog,
  2026-06-20); the OpenAPI spec confirms both are gone. `GetQuotesParams` had them removed (a
  breaking change) and gained `min_ts`/`max_ts`/`user_filter` to match the current parameter set.
  `GetRFQsParams` (whose `event_ticker`/`market_ticker` filters are still valid per the spec) gained
  `user_filter` for parity. RFQ-scoped quote action endpoints
  (`DELETE/PUT /communications/rfqs/{rfq_id}/quotes/{quote_id}/...`) were added as
  `delete_rfq_quote` / `accept_rfq_quote` / `confirm_rfq_quote`, preferred over the quote-ID-only
  variants so a quote cleared by a server roll/restart can't be confused with one belonging to a
  different RFQ (changelog, 2026-06-25).

- `market_lifecycle_v2` gained top-level `price_ranges` (on `created`/`price_level_structure_updated`
  events, alongside `price_level_structure`) and top-level `strike_type` / `cap_strike` /
  `custom_strike` (on `metadata_updated` events, alongside the existing `floor_strike` /
  `yes_sub_title`). Added to `WsMarketLifecycleV2` / `WsMarketLifecycleV2Ref` as `Option` fields
  per the AsyncAPI spec 2026-06/2026-07.

- `GetExchangeStatusResponse` gained `intra_exchange_transfers_active: Option<bool>` and
  `exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>` (per-shard status breakdown), added
  to the OpenAPI spec 2026-07. Both are optional even though the live exchange currently always
  returns them, since the spec itself doesn't mark them required.

- Subaccounts gained exchange-shard awareness (OpenAPI spec 2026-07): `SubaccountBalance` gained a
  required `exchange_index: i64` (a subaccount with funds on multiple indexes now appears as
  multiple balance rows), `SubaccountTransfer` gained `exchange_index`/`transfer_type` (`cash` |
  `position`) plus position-only fields (`market_ticker`, `side`, `count`, `price_cents`), and
  `ApplySubaccountTransferRequest` gained an optional `exchange_index`. A new
  `POST /portfolio/subaccounts/positions/transfer` endpoint (`transfer_position_subaccount`) moves
  a position between subaccounts, distinct from the existing cash `transfer_subaccount`.

- API keys can now be restricted to a single sub-account (OpenAPI spec 2026-07):
  `CreateApiKeyRequest`, `GenerateApiKeyRequest`, and the `ApiKey` response type all gained an
  optional `subaccount: Option<u32>`.

- Added `get_account_api_usage_level_volume_progress()` and
  `upgrade_account_api_usage_level()` for the new `GET /account/api_usage_level/volume_progress`
  and `POST /account/api_usage_level/upgrade` endpoints (OpenAPI spec 2026-06).

- Margin-exchange changelog entries since the 2026-06-08 watermark (per-market risk-metric
  limitations, `is_portfolio`/`margin_used` omission rules, margin order `order_reason`, margin
  market mark prices/tick size/notional fields, margin fee-tier active rates) required no crate
  changes: margin market/order types are not modeled by this crate (see `CHANGELOG.md` for the
  full disposition table).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
