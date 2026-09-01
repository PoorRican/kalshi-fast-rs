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

## 2026-09-01 Refresh (OpenAPI 3.20.0 → 3.29.0, AsyncAPI 2.0.0 content refresh)

- **Exchange sharding / `exchange_index`.** Kalshi introduced multiple exchange shards
  (`exchange_index`) across most portfolio/order/market schemas. `types::ExchangeIndex` is a new
  `i64` alias (not an enum — the OpenAPI schema is a plain open `integer`, and request-side fields
  document `-1` as an "auto-route by ticker" sentinel, which rules out an unsigned type). Response
  fields that the spec marks `required` (`Fill.exchange_index`, `Settlement.exchange_index`,
  `MarketPosition.exchange_index`, `SubaccountBalance.exchange_index`, `SubaccountTransfer.exchange_index`,
  `WsFill` msg `exchange_index`) are modeled non-`Option`. Fields the spec leaves optional, or marks
  required-but-`x-omitempty:false` (an ambiguous combination this crate treats as "may be absent"),
  are modeled `Option<ExchangeIndex>` — this covers `Order`, `EventData`, `Series`,
  `MultivariateEventCollection`, `WsMarketLifecycleV2`, `WsEventLifecycle`, and `WsUserOrder`. New
  `ExchangeIndexStatus`/`GetExchangeStatusResponse.exchange_index_statuses` exposes per-shard status;
  new `IndexedBalance` (shared `exchange_index` + `balance`) backs `GetBalanceResponse.balance_breakdown`
  and `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown`.
- **`ErrorResponse.service` removed.** Kalshi deprecated (2026-07-28) then removed (2026-08-06) the
  `service` field from error bodies. It was already `Option<String>` with no internal readers, so it
  was deleted outright rather than kept as a permanently-`None` field.
- **Legacy (non-V2) order-mutation endpoints removed.** `POST /portfolio/orders`,
  `DELETE /portfolio/orders/{order_id}`, `.../amend`, `.../decrease`, and `/portfolio/orders/batched`
  no longer exist upstream (only `GET /portfolio/orders` and `GET /portfolio/orders/{order_id}`
  remain on that path). `create_order`, `cancel_order`, `amend_order`, `decrease_order`,
  `batch_create_orders`, `batch_cancel_orders` and their request/response types were removed from
  the crate; use the `*_v2` methods (`/portfolio/events/orders/*`) instead. **Breaking.**
- **`GET /exchange/announcements` removed.** `AnnouncementType`, `AnnouncementStatus`, `Announcement`,
  `GetExchangeAnnouncementsResponse`, and `get_exchange_announcements()` were removed. **Breaking.**
- **Multivariate lookup endpoints removed.** `PUT`/`GET .../multivariate_event_collections/{ticker}/lookup`
  no longer exist. `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`, and their types were removed. **Breaking.**
- **`GetQuotesParams`** dropped `market_ticker`/`event_ticker` (removed from the live `GET
  /communications/quotes` parameter list) and gained `min_ts`/`max_ts`. **Breaking** (field removal).
- **New RFQ-scoped quote endpoints**: `get_rfq_quote`, `delete_rfq_quote`, `accept_rfq_quote`,
  `confirm_rfq_quote` (`/communications/rfqs/{rfq_id}/quotes/{quote_id}[/accept|/confirm]`) sit
  alongside the pre-existing quote-id-only methods, which are unchanged.
- **`post_only`** added to `CreateQuoteRequest`/`Quote` (post-only quotes are now preserved rather
  than crossing immediately).
- **`EventData.settlement_sources`** is now spec-required; modeled the same way as the pre-existing
  `Series.settlement_sources` (`Vec<SettlementSource>` via `deserialize_null_as_empty_vec`).
  **`EventMetadata.cadence`** is changelog-only — `product_metadata` stays an untyped `object` in the
  OpenAPI schema with no sub-schema for `cadence` — so it is modeled `Option<String>` defensively,
  matching the crate's convention of surfacing a few known business fields on an otherwise-untyped
  parent. **`EventData.available_on_brokers`** is now `deprecated: true` upstream (always `false`);
  already modeled `Option<bool>`, no change needed.
- **`IncentiveProgram`** gained `max_reward_per_account: Option<i64>` and a new `margin_maker_volume`
  incentive type (the existing `incentive_type: String` already tolerates it); a pre-existing gap
  (`incentive_description`, spec-required) was also fixed since it's the same struct.
- **New endpoints**: `get_account_api_usage_level_volume_progress`, `upgrade_account_api_usage_level`,
  the `IntraExchangeInstanceTransfer*` group (`intra_exchange_instance_transfer`,
  `get_intra_exchange_instance_transfers[_all]`, `get_intra_exchange_instance_transfer`) with a new
  `ExchangeInstance` enum (`event_contract` | `margined`) distinct from the existing
  `ApiUsageLevelGrant.exchange_instance: String`, `get_target_balance_allocation` /
  `set_target_balance_allocation` (with a `RestingMarginReservation` enum), `get_historical_positions`
  (reuses `GetPositionsResponse`), `get_live_data_by_event`, and `get_weather_index` /
  `get_weather_index_calibrations`.
- **API keys**: `ApiKey`/`CreateApiKeyRequest`/`GenerateApiKeyRequest` gained `subaccount` (0-63,
  mutually exclusive with `fcm_subtrader_id`) and `fcm_subtrader_id`; `create_subaccount` now takes
  an `Option<ExchangeIndex>` instead of no arguments. **Breaking** (new required parameter).
- **New `pyth_value` WebSocket channel** (CF Benchmarks-style: per-underlying value push +
  underlying-list message + `subscribe_underlyings`/`unsubscribe_underlyings`/`underlying_list`
  update actions), modeled identically to `cfbenchmarks_value` in `src/ws/types/messages/pyth.rs`.
  Unlike `cfbenchmarks_value`, `pyth_value` requires authentication per the spec, so it is included
  in the channel's private-channel gating.
- **`WsTrade.is_block_trade`** added (`#[serde(default)] bool`), matching the existing REST
  `Trade.is_block_trade` convention. **`WsMarketLifecycleV2`** gained `strike_type`, `cap_strike`,
  `custom_strike` (top-level, `metadata_updated`-only — distinct from the same-named fields already
  nested under `additional_metadata`) and `price_ranges: Vec<WsMarketPriceRange>` (emitted alongside
  `price_level_structure`). **`WsQuoteCreated`/`WsQuoteAccepted`/`WsQuoteExecuted`** all gained
  `subaccount: Option<i64>` (the latter two had a pre-existing gap fixed alongside the former).
- **Known pre-existing gaps not touched by this refresh** (flagged during implementation, left as-is
  pending explicit upstream changelog coverage): `GetFillsParams.event_ticker` has no corresponding
  query parameter in the live `GET /portfolio/fills` spec; `GetPositionsParams.event_ticker` is
  modeled as a CSV-of-10 (`Vec<String>`) but the live `/portfolio/positions` and `/historical/positions`
  endpoints document a *singular* `event_ticker`; `GET /trade-api/v2/events` gained a `min_updated_ts`
  filter not yet modeled on `GetEventsParams`; `IncentiveProgram.target_size` (plain int) appears to
  no longer exist in the spec (only `target_size_fp` remains) but was left in place since it's
  harmless (`Option`, no compile impact).

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
