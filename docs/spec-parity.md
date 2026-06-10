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
  is kept as `Option<String>` rather than reusing the `FeeType` enum, because the spec includes a
  `quadratic_with_maker_fees` variant not present in `FeeType` and the field must stay lossless for
  fee math. Both override fields are nullable (`None` when the override is cleared).
- The AsyncAPI marks several timestamp/required fields that the exchange may omit in practice
  (`ts_ms` on ticker/trade/order-group messages, the legacy direction fields). These are modeled as
  `Option` so parsing never fails on their absence.

- `Trade.is_block_trade` is marked `required` in the OpenAPI (added 2026-06-01). It is modeled
  with `#[serde(default)]` so pre-cutoff historical trade records that lack the field still parse.
  Deserializes to `false` when absent.

- `GET /account/limits` was restructured on 2026-06-09 (live 2026-06-11). The previous integer
  fields `read_limit` / `write_limit` were replaced by `read: BucketLimit` and `write: BucketLimit`
  objects, and a `grants: Vec<ApiUsageLevelGrant>` array was added. The crate was updated to match
  the new OpenAPI shape; code accessing the old integer fields must migrate to `read.refill_rate` /
  `write.refill_rate` or `read.bucket_capacity` / `write.bucket_capacity`.

- `GET /trade-api/v2/account/limits/perps` is mentioned in the 2026-06-09 changelog entry but is
  not present in the published OpenAPI spec. Per the authoritative YAML policy, this endpoint has
  not been added to the crate. Add it when it appears in the spec.

- The `last_update_reason` field on orders (values: `PostOnlyCrossCancel`, `Decrease`, etc.) is
  referenced in the 2026-06-04 changelog entry but is not present in the published OpenAPI or
  AsyncAPI specs. It is not modeled in the crate. Add it when it appears in the spec.

- Perps margin market data fields (`lifetime_volume_notional_dollars`, `open_interest_notional_dollars`,
  `volume_24h_notional_dollars`) were added on 2026-06-05 to `GET /margin/markets` and the
  `margin_ticker` WebSocket channel. The crate does not yet have margin market endpoint structs;
  these fields are not modeled. Add margin market types when scoping that feature.

- RFQ fractional contract quantities were announced 2026-05-26, effective 2026-06-11. Fields
  `contracts_fp` on RFQ create/list and `yes_contracts_offered_fp` / `no_contracts_offered_fp` on
  quotes are already represented as `FixedPointCount` strings in the existing `WsRfqCreated` /
  `WsQuoteCreated` types. No crate changes are required; callers should accept fractional values.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
