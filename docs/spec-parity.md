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

## 2026-08-19 Reconciliation (0.8.0)

### Deprecated-but-still-present fields

- `GetQuotesParams.quote_creator_user_id` / `.rfq_creator_user_id` and
  `GetRFQsParams.creator_user_id` are marked `deprecated: true` in the OpenAPI but are still
  accepted query parameters. They are kept as `Option<String>` and carry `#[deprecated]` so the
  compiler surfaces the nuance at the call site without breaking existing code. The replacements
  are `user_filter` (filter by the authenticated user) and, for quotes, `rfq_user_filter` (filter
  by the RFQ's creator) — note these are *not* interchangeable: `user_filter` scopes by the quote's
  creator, `rfq_user_filter` by the RFQ's.
- `EventData.category` is marked `deprecated: true` upstream ("use series-level category instead")
  but is still returned. Modeled as `Option<String>` with `#[deprecated]`.
- The quote-ID-only quote actions (`GET`/`DELETE /communications/quotes/{quote_id}` and the
  `/accept` and `/confirm` variants) are explicitly marked `DEPRECATED` in their OpenAPI
  descriptions but still routed. The corresponding methods carry `#[deprecated]` pointing at the
  RFQ-scoped forms. Kalshi has signalled `rfq_id` will become required for quote actions in a
  future migration, so new code should use `get_rfq_quote` / `delete_rfq_quote` /
  `accept_rfq_quote` / `confirm_rfq_quote`.
- `Order.side` / `Order.action` (and the `Fill` / `WsFill` equivalents) are no longer in the
  OpenAPI `required` list — they are now `deprecated: true` optional properties. The crate already
  modeled them as `Option` (0.5.0); that treatment is now exactly what the spec says.
- `WsFill.purchased_side` is the outlier: the AsyncAPI *still* lists it in the fill payload's
  `required` array, yet its description carries the identical deprecation notice as `side` and
  `action` ("Deprecated. Use `outcome_side` (or `book_side`) instead. This field will not be
  removed before May 14, 2026."). It is modeled as `Option<YesNo>` for the same reason its
  siblings are: a required-and-deprecated field is a field scheduled to disappear, and leaving it
  non-`Option` would turn its removal into a total parse failure for every `fill` message rather
  than a single `None`.

### Required-but-conditional fields modeled as `Option`

- `GetBalanceResponse.balance_dollars` is marked required in the current OpenAPI, but has
  historically been absent for non-direct members, so it stays `Option<FixedPointDollars>`.
- `MarketPosition.exchange_index` and `Fill.exchange_index` are marked required, but are modeled as
  `Option<i64>` to tolerate older or partial payloads. By contrast
  `SubaccountBalance.exchange_index` is modeled as a plain required `i64`, because the per-index
  balance split (2026-07-02) means an entry without it is ambiguous rather than merely incomplete —
  a missing value there would silently misattribute a balance, so failing to parse is the safer
  behavior.
- `SeriesFeeChange.id` and `.scheduled_ts` are typed `string` in the OpenAPI (`scheduled_ts` as
  `format: date-time`) but are read through the lenient string-or-number deserializer. The crate's
  own historical fixture used raw epoch integers for `scheduled_ts` and a numeric `id`, so the
  encoding the exchange actually emits is not fully settled; accepting both avoids a hard parse
  failure either way. `.fee_multiplier` is `f64`, matching the spec's `number`/`double` — it was
  previously `i64`, which could never have parsed a fractional multiplier.
- `WsMarketLifecycleV2` top-level `strike_type` / `cap_strike` / `custom_strike` / `floor_strike` /
  `yes_sub_title` appear only on `metadata_updated` events, and `exchange_index` / `price_ranges`
  only on `created` (and, for `price_ranges`, `price_level_structure_updated`). All are `Option`.
- `WsQuoteCreated` / `WsQuoteAccepted` / `WsQuoteExecuted` carry `subaccount` only when your own
  side of the quote used a subaccount; the counterparty's is never shared. Modeled as
  `Option<u32>`.
- `WsCfBenchmarksValue.last_60s_windowed_average_15min` (unchanged from 0.6.0) remains `Option`.
- `WsTrade.is_block_trade` is marked required in the AsyncAPI trade payload, but is modeled as a
  plain `bool` with `#[serde(default)]` (defaulting to `false`) rather than `Option<bool>`. This
  mirrors the REST `Trade.is_block_trade` treatment added in 0.6.0: the field only appeared on the
  WebSocket trade message on 2026-08-13, so payloads captured or replayed from before then omit
  it, and "absent" unambiguously means "not a block trade". Consumers get a non-`Option` field
  without a parse failure on older data.

### Blanket policy: `required` in the spec, `Option` in the crate

Beyond the individually-justified cases above, several large response objects model most or all of
their spec-`required` fields as `Option` — notably `Market` (nearly all 32 required entries),
`Series`, `EventData`, `WsUserOrder`, `WsEventLifecycle`, and `WsMarketLifecycleV2.event_type`.
This is deliberate and predates this reconciliation. Kalshi's `required` lists have repeatedly
shifted underneath released versions of this crate (`side` / `action` / `taker_side` were required
until they weren't; `response_price_units` and `fractional_trading_enabled` were required until
they were deleted). Since these are *read* paths, a too-strict model turns an upstream field
change into a total parse failure for every message of that type, while a too-loose one degrades
to a single `None` that the caller can handle. For read models the crate therefore biases toward
`Option`, and pairs it with a `#[serde(flatten)] extra` map wherever practical so unmodeled keys
survive rather than being dropped.

The deliberate exceptions — where a missing value would be actively misleading rather than merely
absent — are called out individually above (`SubaccountBalance.exchange_index`) or are fields whose
absence has a single unambiguous meaning (`Trade.is_block_trade` / `WsTrade.is_block_trade`, where
absent means `false`).

Request/params types are the mirror image: they stay `Option` with
`skip_serializing_if = "Option::is_none"` so an unset filter is simply omitted from the query
string rather than sent as a default the server would act on.

### Removals confirmed against the live specs

The following were removed from the crate after confirming, by grepping the live YAML, that the
paths/fields are absent — not merely deprecated:

- `POST /portfolio/orders`, `DELETE /portfolio/orders/{order_id}`, and the `/amend`, `/decrease`,
  and `/batched` mutation paths. Only the `GET` list, `GET` single-order, and queue-position reads
  remain at those paths. Use the V2 event-order endpoints.
- `GET /exchange/announcements` (no `announcements` string anywhere in the OpenAPI).
- The `multivariate` WebSocket channel and its `multivariate_lookup` message type (the AsyncAPI
  channel enum lists only `multivariate_market_lifecycle`), plus the REST
  `PUT/GET /multivariate_event_collections/{collection_ticker}/lookup` pair.
- `ErrorResponse.service`, `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `MarketPosition.resting_orders_count`.
- `WsMarketLifecycleV2.fractional_trading_enabled` and the `fractional_trading_updated` lifecycle
  event value: the string `fractional_trading` does not appear anywhere in the live AsyncAPI, and
  the `event_type` enum lists only `created`, `deactivated`, `activated`, `close_date_updated`,
  `determined`, `settled`, `price_level_structure_updated`, and `metadata_updated`. The
  `WsMarketLifecycleEventType` enum keeps its `#[serde(other)] Unknown` catch-all, so an
  unexpected value from the exchange still parses rather than panicking.

`build_http_error` still parses error bodies that happen to include a stray `service` key — the
field is simply ignored rather than treated as a parse failure, so responses from any not-yet-
updated edge still deserialize.

### Price level structures

Ten new `price_level_structure` values landed between 2026-07-23 and 2026-08-13
(`center_whole_edge_half_cent` … `center_deci_edge_centi_cent`). The crate intentionally does
**not** model these as an enum: `Market.price_level_structure` and
`WsMarketLifecycleV2.price_level_structure` stay `Option<String>`. Kalshi's own guidance is to
snap prices to the `step` of the band containing the price in the market's `price_ranges` array
rather than keying logic off the structure name, and `price_ranges` is exposed on both the REST
market object and (since 2026-07-02) the lifecycle push. Keeping the label as a raw string means
future structures need no crate update.

### Not modeled (out of scope)

Margin-exchange types (margin markets, margin positions, margin risk, margin order groups) and the
FIX API are not modeled by this crate, so changelog entries tagged `Margin`-only or `FIX`-only are
recorded as no-change. Rate-limit costs, retention windows, and server-side quotas (order-group
maximums, orderbook subscription sanity limits, API usage tier thresholds) likewise carry no
schema change.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
