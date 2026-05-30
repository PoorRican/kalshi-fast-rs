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

## Deprecated-but-present Fields

The upstream Kalshi API is migrating order-direction vocabulary from legacy
`side`/`action`/`taker_side` to the new `outcome_side`/`book_side`/`taker_outcome_side`
and `taker_book_side` fields. The spec marks the old fields as deprecated but they
remain present in server responses during the transition.

The crate models this as follows:

| Struct | Deprecated field(s) | Replacement(s) | Modeling |
|---|---|---|---|
| `Order` | `side: Option<YesNo>`, `action: Option<BuySell>` | `outcome_side: YesNo`, `book_side: BookSide` | Old fields `#[deprecated]` + `Option`; new fields required |
| `Fill` | `side: Option<YesNo>`, `action: Option<BuySell>` | `outcome_side: YesNo`, `book_side: BookSide` | Old fields `#[deprecated]` + `Option`; new fields required |
| `Trade` | `taker_side: Option<TradeTakerSide>` | `taker_outcome_side: TradeTakerSide`, `taker_book_side: BookSide` | Old field `#[deprecated]` + `Option`; new fields required |
| `WsFill` | `side: Option<YesNo>`, `action: Option<BuySell>`, `purchased_side: Option<YesNo>` | `outcome_side: YesNo`, `book_side: BookSide` | Old fields `#[deprecated]` + `Option`; new fields required |
| `WsTrade` | `taker_side: Option<TradeTakerSide>` | `taker_outcome_side: TradeTakerSide`, `taker_book_side: BookSide` | Old field `#[deprecated]` + `Option`; new fields required |
| `WsUserOrder` | `side: Option<YesNo>`, `is_yes: Option<bool>` | `outcome_side: Option<YesNo>`, `book_side: Option<BookSide>` | Old fields `#[deprecated]` + `Option`; new fields `Option` (conditionally present) |

Downstream consumers should read from the replacement fields. The deprecated fields
are preserved only to avoid silent deserialization failures when the server still
sends them; they will be removed once Kalshi drops them from responses.

## External API Hosts

Kalshi provides a dedicated external API host for direct/non-FCM members that
bypasses FCM routing. The crate exposes these via `KalshiEnvironment::external()`
and `KalshiEnvironment::external_demo()`:

| Environment | REST | WebSocket |
|---|---|---|
| Production (FCM) | `https://api.elections.kalshi.com` | `wss://api.elections.kalshi.com/trade-api/ws/v2` |
| Production (External) | `https://external-api.kalshi.com` | `wss://external-api-ws.kalshi.com/trade-api/ws/v2` |
| Demo (FCM) | `https://demo-api.kalshi.co` | `wss://demo-api.kalshi.co/trade-api/ws/v2` |
| Demo (External) | `https://external-api.demo.kalshi.co` | `wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2` |

New integrations should prefer the external host unless they are FCM members.

## Conditional Fields on `WsMarketLifecycleV2`

`floor_strike` and `yes_sub_title` are only present on `metadata_updated` events.
Both are modeled as `Option` on `WsMarketLifecycleV2` and its borrowed counterpart.
The `WsMarketLifecycleAdditionalMetadata` struct similarly carries `floor_strike`
and other fields that only appear for specific event types.

## Test Strategy

- Deterministic parsing and behavior checks: `tests/parsing.rs`,
  `tests/ws_parsing.rs`, `tests/ws_command_behavior.rs`
- Live contract checks: `tests/rest_public.rs`, `tests/rest_auth.rs`,
  `tests/ws_public.rs`, `tests/ws_auth.rs`
