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

- Exchange sharding (2026-06 through 2026-09 changelog entries) introduced an `exchange_index`
  identifier across most REST and WebSocket surfaces. This refresh added `exchange_index` to
  `Series`, `SubaccountBalance`, `WsEventLifecycle`, `WsMarketLifecycleV2`, `WsFill`, `WsUserOrder`,
  plus `exchange_index` query filters on `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams`/
  `GetBalanceParams`, and per-index breakdown structs (`ExchangeIndexStatus`, `IndexedBalance`).
  A handful of exchange-sharding endpoints are **not yet implemented** (tracked below under
  "Known Gaps") because they introduce a new domain concept (cross-shard transfers, target balance
  allocation) rather than a field addition to an existing type.
- `GetQuotesParams::market_ticker` / `event_ticker` are `#[deprecated]`: Kalshi removed both filters
  from `GET /communications/quotes` on 2026-06-20 and the server now silently ignores them if sent.
  They are kept (rather than removed) because they are harmless no-ops, not a parsing hazard.
- `get_quote` / `delete_quote` / `accept_quote` / `confirm_quote` (quote-ID-only) are `#[deprecated]`
  in favor of the RFQ-scoped `get_rfq_quote` / `delete_rfq_quote` / `accept_rfq_quote` /
  `confirm_rfq_quote`, which the live docs now prefer (2026-06-25 / 2026-07-09) and which carry
  better rate limits for `confirm`. The quote-ID-only endpoints still function upstream.
- The `multivariate` WebSocket channel and its `multivariate_lookup` message type, and the REST
  `.../multivariate_event_collections/{collection_ticker}/lookup` endpoint (both GET history and PUT
  ticker-pair lookup), were removed upstream on 2026-08-06. The corresponding crate surface
  (`WsChannelV2::Multivariate`, `WsMsgType::Multivariate`/`MultivariateLookup`,
  `WsDataMessageV2::Multivariate`, `WsMultivariate`/`WsMultivariateRef`,
  `lookup_tickers_for_market_in_multivariate_event_collection`,
  `get_multivariate_event_collection_lookup_history`) was removed rather than kept as dead code.
  The `multivariate_market_lifecycle` channel and the base
  `POST /multivariate_event_collections/{collection_ticker}` endpoint are unaffected and remain
  fully supported.

## Known Gaps (Deferred)

The following upstream additions since the 2026-06-08 watermark are **not yet implemented**. Each
introduces a new domain concept rather than a field/param addition to an existing type, so they are
tracked here explicitly rather than rushed:

- **Ed25519 API keys** (2026-09-24). `src/auth.rs` currently hardcodes RSA-PSS/SHA-256 signing
  (`KalshiAuth` holds an `RsaPrivateKey`). Supporting Ed25519 requires a key-type-aware signer
  (`ed25519-dalek` or similar), a `key_type` field on `GenerateApiKeyRequest`, and careful signing
  correctness testing. Deferred to a dedicated follow-up given the security sensitivity of auth code.
- **Target balance allocation** (2026-08-20 / 2026-09-17 / 2026-09-24). `GET`/`POST
  /portfolio/target_balance_allocation`, including the `RestingMarginReservation` policy
  (`none`/`max`/`sum`) and the "rebalance without resting-order reservation" behavior. Not
  implemented at all; needs new request/response types.
- **Cross-shard subaccount transfers** (2026-08-20) and **intra-account transfer history**
  (2026-08-13). `POST /portfolio/intra_exchange_instance_transfer` and `GET
  /portfolio/intra_exchange_instance_transfers[/{id}]`. Not implemented; distinct from the existing
  same-shard `apply_subaccount_transfer`.
- **Kalshi Weather Index endpoints** (2026-08-20, calibration history 2026-08-31, `receipt_basis`
  2026-09-10). `GET /live_data/weather/{city}` and `.../calibrations`. Not implemented; a new
  endpoint family distinct from `get_event_live_data`.
- **`pyth_value` WebSocket channel** (2026-07-23) and **`cfbenchmarks_value_5hz` WebSocket channel**
  (2026-09-03). Both are new channels not yet wired into `WsChannelV2`/`WsMsgType`/the envelope
  parse paths. `cfbenchmarks_value` (non-5Hz) is already fully supported as a template to follow.
- **WebSocket `user_filter`** on the `communications` channel subscription (2026-10-01), to receive
  only RFQs/quotes created by the authenticated user. Not yet added to `WsSubscriptionParamsV2`.
- **Optional WebSocket permessage-deflate compression** (2026-09-24). `tokio-tungstenite` 0.24 (this
  crate's pinned version) has no built-in extension negotiation support, so this is blocked on a
  dependency upgrade or a custom extension-negotiation layer, not just a field addition.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
