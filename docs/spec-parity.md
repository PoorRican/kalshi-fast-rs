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

- `Market.response_price_units`, `Market.fractional_trading_enabled`, and
  `MarketPosition.resting_orders_count` were removed from the Predictions REST API schema on
  2026-07-03/09 and are no longer modeled (intentional minor-version break, 0.6.0 → 0.7.0). Fractional
  trading is now unconditional for all markets; `price_level_structure` / `price_ranges` are the
  canonical replacements for pricing-precision concerns.

- The `fractional_trading_updated` `market_lifecycle_v2` WebSocket event and its
  `fractional_trading_enabled` field were removed from the AsyncAPI schema on 2026-04-17 (the crate's
  0.6.0 refresh missed this; caught during a full grep-vs-YAML pass). Neither
  `WsMarketLifecycleEventType::FractionalTradingUpdated` nor `WsMarketLifecycleV2.fractional_trading_enabled`
  exist anymore.

- `GET /exchange/announcements` was removed from the Predictions REST API on 2026-07-04
  (`get_exchange_announcements`, `GetExchangeAnnouncementsResponse`, `Announcement`,
  `AnnouncementType`, `AnnouncementStatus` removed). `GET /exchange/schedule` remains the source for
  operational hours.

- `GET /exchange/status` gained a per-index breakdown on 2026-06-26: `intra_exchange_transfers_active`
  and `exchange_index_statuses: Vec<ExchangeIndexStatus>` (both `Option`, since only
  `exchange_active` / `trading_active` are `required` in the OpenAPI schema). The top-level fields
  continue to reflect the default exchange index (0).

- `GET /portfolio/subaccounts/balances` added a required `exchange_index` field to each
  `SubaccountBalance` element on 2026-06-24 (per-index subaccount balances; intentional
  minor-version break, 0.6.0 → 0.7.0).

- `GET /communications/rfqs/{rfq_id}/quotes/{quote_id}` (and its `accept`/`confirm`/`DELETE`
  siblings) scope quote lookups to their RFQ, added 2026-07-07. The quote-ID-only endpoints
  (`GET|DELETE /communications/quotes/{quote_id}`, `PUT .../accept`, `PUT .../confirm`) are still
  present upstream but deprecated, so the corresponding Rust methods (`get_quote`, `delete_quote`,
  `accept_quote`, `confirm_quote`) are kept working but marked `#[deprecated]` rather than removed.

- The multivariate ticker-pair lookup endpoints predate RFQs and are deprecated upstream:
  `PUT /multivariate_event_collections/{collection_ticker}/lookup` is marked `deprecated: true` in
  the live OpenAPI, and the lookup-history `GET` variant is "fully deprecated" (2026-07-02) and no
  longer documented in the OpenAPI at all. Both `lookup_tickers_for_market_in_multivariate_event_collection`
  and `get_multivariate_event_collection_lookup_history` are kept (the changelog says "deprecated,"
  not "removed") but marked `#[deprecated]`.

- `market_lifecycle_v2` `created` / `price_level_structure_updated` events carry a `price_ranges`
  array alongside `price_level_structure` (added 2026-06-30), mirroring the REST `Market.price_ranges`
  field. Modeled as `Option<Vec<PriceRange>>` on `WsMarketLifecycleV2` (reusing the REST `PriceRange`
  type), with a borrowed `WsPriceRangeRef` for the zero-copy path.

- `pyth_value` is a new authenticated AsyncAPI channel (added 2026-07-13) delivering deduplicated
  real-time Pyth price updates for configured underlying tickers. It follows the same shape as
  `cfbenchmarks_value`: seed `underlying_tickers` on subscribe (`["all"]` for every underlying), and
  use `WsUpdateAction::SubscribeUnderlyings` / `UnsubscribeUnderlyings` / `UnderlyingList` via
  `update_subscription_v2` to manage the set post-subscribe. `validate_update` enforces the same
  mutual-exclusivity and required-field rules as the CF Benchmarks index actions.

- `WsChannelV2::is_private()` now also returns `true` for `CfbenchmarksValue`: the AsyncAPI marks
  `cfbenchmarks_value` as requiring authentication, but this was missed when the channel was added in
  0.6.0. `PythValue` is authenticated from its initial implementation.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
