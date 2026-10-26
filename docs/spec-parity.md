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

- `Market::settlement_bounds_type` is spec-required but modeled as `Option<SettlementBoundsType>`
  like the rest of `Market`, so payloads from older snapshots still parse. `SettlementBoundsType`
  and the other new string enums (`MarginFundingLimitUnit`, `MarginFundingInvalidPremiumHandling`,
  `MarginCorporateActionType`/`Status`, `ExchangeInstance`) carry `#[serde(other)] Unknown`.
- `MarginMarket::halt_sources` is spec-required but may arrive `null`; it deserializes to an empty
  `Vec`. An empty list means "not halted or reason not observed", not proof the market is live.
  `MarginMarket::schedule` stays `Option` because the schema is nullable.
- Margin `GET /margin/exchange/status` shares the Predictions `ExchangeStatus` schema, but the margin
  exchange never returns `intra_exchange_transfers_active` or `exchange_index_statuses`, so only
  `exchange_estimated_resume_time` is modeled there. `POST /portfolio/margin/subaccounts/transfer`
  ignores `exchange_index`, so the request type omits it.
- `orderbook_snapshot` / `orderbook_delta` require `clock` (exchange clock, identical on every
  connection; `seq` is per subscription). Modeled as a required `i64`. Snapshots send `0` when the
  market has no recorded change.
- RFQ batch WebSocket payloads are low volume, so `WsCommunicationsRef` reuses the owned batch
  structs instead of duplicating every nested type with `Cow` fields.
- `sending_ts_ms` (optional on every AsyncAPI message since 2026-10-01) is intentionally not surfaced:
  parsers ignore it and the typed envelopes stay unchanged. Callers needing queue-to-receive latency
  should use the timed reader's receive stamps.
- The intra-account transfer endpoint exists in both the Predictions and Margin OpenAPI documents with
  the same shape (the margin copy requires all three API-key headers). One client method serves both.

## Changelog Reconciliation: 2026-09-28 -> 2026-10-26

Maps every upstream entry after the previous watermark (OpenAPI 3.20.0 -> 3.35.0) to the crate.

| Entry (date) | Outcome |
| --- | --- |
| Margin `market_lifecycle_v2` (10-29) | `MarginChannel::MarketLifecycleV2`, `MarginDataMessage::MarketLifecycleV2` |
| Restricted keys transfer between exchange indices (10-29) | `intra_exchange_instance_transfer`; no key-specific code (server-side scope) |
| RFQ batch WS messages (TBD) | seven `WsCommunications` variants; `quote_executed.rfq_batch_id` |
| Margin `halt_sources` (10-29) | `MarginMarket::halt_sources` |
| Subaccount balances for restricted keys (10-29) | `SubaccountBalance::exchange_index` (now one row per index) |
| FIX market-data selection fields (10-29), FIX logon / replace / mass-cancel / TradeID / SecurityStatus (10-01..10-22) | No change: the crate has no FIX client |
| `GET /portfolio/orders` up to 100 event tickers (10-29) | `GetOrdersParams` validation 10 -> 100 |
| Orderbook `clock` (10-29) | `clock` on snapshot/delta (owned and `Ref`) |
| `client_transfer_id` on transfer reads (10-22) | `IntraExchangeInstanceTransfer::client_transfer_id` |
| Post-only RFQ cancellations penalized (10-19) | No change: server-side policy |
| Milestone `details.status` lag (10-22) | No change: documentation clarification; `Milestone.details` is an untyped map |
| Redis auth / verified TLS for WS limits (10-22) | No change: server-side |
| `user_orders` timestamps truncated to ms (10-22) | No change: fields stay RFC3339 strings |
| REST schema corrections for shared definitions (10-22) | Margin `exchange_estimated_resume_time` added; the other documented fields are never returned by margin, so they are not modeled; Predictions `subaccount` `minimum: 0` is already `u32` |
| `GET /portfolio/balance` own rate-limit bucket (10-15) | No change: the opt-in client throttle stays conservative (balance draws on the read budget) |
| Funding averages clock-aligned bars (10-22) | `premium_index` / `premium_index_ts` on the funding estimate |
| Margin corporate actions: REST, WS channel, FIX (10-22) | `get_margin_corporate_actions`, `MarginChannel::CorporateActions`; FIX no change |
| `GET /margin/markets/orderbooks` (10-15) | `get_margin_market_orderbooks` |
| Multivariate `invalid_parameters` messages (10-15) | No change: error text only, status and code unchanged |
| Margin `funding_invalid_premium_handling`, `funding_deadband`/`funding_cap`/`funding_limit_unit`, `bid_size_fp`/`ask_size_fp`, `market_version` (10-15, 10-08) | `MarginMarket` fields; `CreateMarginOrderRequest::market_version` |
| `client_transfer_id` on transfer create (10-15) | `IntraExchangeInstanceTransferRequest::client_transfer_id` |
| `GET /margin/trades` optional ticker (10-15) | `GetMarginTradesParams::ticker: Option` |
| Restricted keys on perps orders/positions/fills (10-15) | No change: `subaccount` already optional on those requests |
| MCP portfolio grants; `exclude_markets` on `GET /events/{ticker}` (10-15) | `GetEventParams::exclude_markets`, `get_event_with_params`; MCP OAuth no change |
| FIX / RFQ closure notification / shard capital transfer (10-08) | No change: server-side or FIX |
| Multivariate collections `price_level_structure` / `price_ranges` (10-08) | `MultivariateEventCollection` fields |
| `GET /portfolio/fills` up to 100 tickers (10-08) | Doc comment only: `ticker` is already a free string |
| Exit-trigger validation (10-01) | No change: the crate has no exit-trigger endpoints (see drift) |
| `settlement_status` on positions (10-01) | No change: `GetPositionsParams::settlement_status` already exists |
| `sending_ts_ms` on WS messages (10-01) | No change: see note above |
| Amend `expiration_time` (10-01) | `AmendOrderV2Request::expiration_time`; margin amend already had it |
| `settlement_bounds_type` / `settlement_floor_dollars` (10-01) | `Market` fields |

## Unresolved Drift (not addressed in this refresh)

Found while auditing the YAML; none is tied to a changelog entry after the previous watermark.

- RFQ batch REST endpoints (`/communications/rfq_batches/...`) are not implemented.
- Margin exit-trigger endpoints (`.../exit_trigger`), `/margin/fee_tier_rates`, `/margin/fcm/*` and
  `/margin/funding_rates/premium_index` are not implemented.
- `GET /portfolio/positions` documents a single `event_ticker` and an `exchange_index` filter; the
  crate still sends a CSV `event_ticker` (max 10) and has no `exchange_index` filter.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
