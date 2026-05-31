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

- The OpenAPI spec marks `GetBalanceResponse.balance_dollars` as `required`, but the changelog
  entry (2026-05-28) notes it is provided for "direct members only". Modeled as `Option` to survive
  responses from non-direct-member accounts where the field may be absent. If Kalshi confirms
  universal presence, this should be promoted to a non-optional field.

- `GetBalanceResponse.balance_breakdown` is an optional array of `IndexedBalance` (per-exchange-shard
  balance). Not in the required list; deserialized with `deserialize_null_as_empty_vec` so null and
  absent both parse as an empty `Vec`.

- `FeeType::QuadraticWithMakerFees` is a valid enum value per the OpenAPI spec (`fee_type` enum:
  `[quadratic, quadratic_with_maker_fees, flat]`). Previously decoded as `FeeType::Unknown`.
  Adding this variant is a minor-breaking change per VERSIONING.md (see 0.6.0 release notes).
  The `WsEventFeeUpdate.fee_type_override` field continues to use `Option<String>` to remain
  lossless for any future unknown fee-type overrides.

- `Market.fractional_trading_enabled` is deprecated by Kalshi ("always true, carries no information")
  and marked `#[deprecated]` in Rust. The WS lifecycle struct `WsMarketLifecycleV2` also carries
  this field but was not marked deprecated (it mirrors the REST Market shape; callers should prefer
  the REST behavior note).

- V2 REST order endpoints (`/portfolio/events/orders`) use `BookSide` (`bid`|`ask`) exclusively for
  order direction, replacing the legacy `YesNo`+`BuySell` pair from the V1 endpoints. Response
  shapes are lightweight (order ID, fill counts, timestamp) rather than full `Order` objects.
  The `exchange_index` field present on V2 request types defaults to `0`; currently only `0` is
  supported.

- `Quote.post_only` is only visible to the quote creator (not the RFQ creator). Similarly,
  `Quote.creator_subaccount` is only visible to the quote creator, and `Quote.rfq_creator_subaccount`
  is only visible to the RFQ creator. All three are modeled as `Option` since they're conditionally
  present in the response.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
