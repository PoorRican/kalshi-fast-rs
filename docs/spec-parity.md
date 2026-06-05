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

- `Trade.is_block_trade` is marked required in the OpenAPI spec (added 2026-05-29). It is modeled
  as `bool` (non-optional) following the spec. The companion `GetTradesParams.is_block_trade: Option<bool>`
  filter applies to both `GET /markets/trades` and `GET /historical/trades`.

- The OpenAPI spec marks `GetBalanceResponse.balance_dollars` as required, but this field is only
  populated for direct exchange members (broker-routed accounts may not receive it). It is modeled
  as `Option<FixedPointDollars>` so non-direct-member responses parse without error.
  `GetBalanceResponse.balance_breakdown` (per-exchange-shard balances) is optional per spec and
  modeled as `Option<Vec<IndexedBalance>>`.

- The `/margin/fee_tiers` response was restructured on 2026-05-11. The previous tier-name maps
  (`maker_fee_tiers`, `taker_fee_tiers`) were replaced by per-ticker decimal-rate maps
  (`maker_fee_rates`, `taker_fee_rates`). Fee is computed as `notional * rate`.
  This endpoint is documented in the Kalshi changelog but is not present in the published OpenAPI
  spec; see `exchange.rs::GetMarginFeeTiersResponse`.

- `event_fee_update` is an AsyncAPI message delivered on the `market_lifecycle_v2` channel (it is
  not a separately-subscribable channel). It is modeled by `WsEventFeeUpdate`. `fee_type_override`
  uses `Option<String>` rather than `FeeType` for lossless encoding (the field can carry any
  string override value). Both override fields are nullable (`None` when the override is cleared).
- The AsyncAPI marks several timestamp/required fields that the exchange may omit in practice
  (`ts_ms` on ticker/trade/order-group messages, the legacy direction fields). These are modeled as
  `Option` so parsing never fails on their absence.

- The changelog (2026-06-04) introduces `PostOnlyCrossCancel` as a new `last_update_reason` value
  for post-only orders that cross the book. `last_update_reason` is not present in the published
  OpenAPI or AsyncAPI YAML as of 2026-06-05. Per the reconciliation policy the YAML is
  authoritative for shape; this field is not modeled until it appears in the published spec.

- The changelog (2026-06-05) adds `tick_size` to margin market responses and notional dollar fields
  to perps market data and `margin_ticker` WebSocket messages. These endpoints
  (`GET /trade-api/v2/margin/markets*`, WebSocket `margin_ticker`) are not present in the
  published OpenAPI or AsyncAPI YAML as of 2026-06-05; no change needed until the spec is updated.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
