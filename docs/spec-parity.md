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

- `Trade.is_block_trade: bool` was added as a required field by Kalshi on 2026-06-01. It is
  `true` for trades matched off-book via RFQ or block proposals, `false` for standard order-book
  fills. It is NOT present on WebSocket `WsTrade` messages (AsyncAPI 2.0.0 does not include it).

- `Trade.taker_outcome_side` and `Trade.taker_book_side` are marked required in OpenAPI 3.20.0
  and required in the AsyncAPI `tradePayload` schema. However, the canonical AsyncAPI example
  payload omits both fields, indicating the exchange may send legacy messages without them.
  Both fields are kept as `Option` on `WsTrade`/`WsTradeRef` to avoid parse failures. On the
  REST `Trade` struct they are also `Option` for the same reason.

- `CancelOrderResponse.reduced_by` (integer centi-count) and
  `BatchCancelOrdersIndividualResponse.reduced_by` are no longer present in the OpenAPI 3.20.0
  schema. Both fields are modeled as `Option<i64>` so existing responses that still include the
  field continue to parse. Use `reduced_by_fp` (always required) for cancellation quantities.

- The `PostOnlyCrossCancel` cancel reason (added 2026-06-04) is a string value reported in
  exchange notifications for post-only orders that would have crossed the book. It is not a
  separate schema field in the OpenAPI Order object and requires no struct change; it appears
  as a string in any cancel-reason context.

- The `write::transfer` scope (added 2026-06-03) is a new `ApiKeyScope` enum value in the
  OpenAPI spec. The crate stores API key scopes as `Vec<String>`, which already accepts this
  value without change.

- `event_fee_update` is an AsyncAPI message delivered on the `market_lifecycle_v2` channel (it is
  not a separately-subscribable channel). It is modeled by `WsEventFeeUpdate`. `fee_type_override`
  is kept as `Option<String>` rather than reusing the `FeeType` enum, because the spec includes a
  `quadratic_with_maker_fees` variant not present in `FeeType` and the field must stay lossless for
  fee math. Both override fields are nullable (`None` when the override is cleared).
- The AsyncAPI marks several timestamp/required fields that the exchange may omit in practice
  (`ts_ms` on ticker/trade/order-group messages, the legacy direction fields). These are modeled as
  `Option` so parsing never fails on their absence.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
