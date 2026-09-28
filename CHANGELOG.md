# Changelog

This file records release history for `kalshi-fast-rs`.

Release entries may include a `Compatibility` block summarizing the upstream
Kalshi docs snapshot tracked by that release.

For crate versioning policy and bump rules, see [`VERSIONING.md`](VERSIONING.md).


## [0.8.0] - 2026-09-28

### Compatibility

- Docs snapshot: 2026-09-28
- OpenAPI: 3.31.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-10-01

**Changelog entries since the 0.7.0 watermark (2026-06-08) and disposition:**

| Date | Entry | Action |
|---|---|---|
| 2026-06-11 | API usage volume progress endpoint | No code change — new `GET /account/api_usage_level/volume_progress` endpoint not yet modeled. Tracked as a gap. |
| 2026-06-11 | Perps mark prices on margin markets | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-11 | Self-serve Advanced API usage tier upgrade | No code change — new `POST /account/api_usage_level/upgrade` endpoint not yet modeled. Tracked as a gap. |
| 2026-06-11 | Margin fee-tier endpoint returns active rates | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-11 | Perps volume and open interest notional fields | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-11 | Tick size added to GET Margin Markets | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-11 | Fractional quantities for RFQs | No code change — `contracts_fp` already present in `CreateRfqRequest` (carried over from the 0.6.0 refresh). |
| 2026-06-18 | settlement_sources added to the events API | No code change — `EventData` has a flatten `extra` catch-all; `settlement_sources` round-trips losslessly without a typed field. |
| 2026-06-18 | Strike type and cap strike on market_lifecycle_v2 metadata_updated | **Added** — top-level `strike_type`/`cap_strike`/`custom_strike` on `WsMarketLifecycleV2`/`Ref`. |
| 2026-06-18 | RFQ quote identity on FIX | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-06-18 | Trade entries in FIX market data | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-06-18 | Legacy order mutation endpoints deprecated | **Removed** — legacy `/portfolio/orders` write endpoints (create/cancel/amend/decrease/batch) confirmed gone from `openapi.yaml`; removed `create_order`/`cancel_order`/`amend_order`/`decrease_order`/`batch_create_orders`/`batch_cancel_orders` and their request/response types. Use the V2 equivalents. |
| 2026-06-18 | Event tickers filter on GET /trade-api/v2/events | No code change — `GetEventsParams` not exhaustively modeled for every filter; tracked as a gap, not a correctness issue (extra filters are additive). |
| 2026-06-18 | Subaccount on margin positions | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only (tags omitted in the source changelog entry itself). |
| 2026-06-18 | Block-trade accept API key permissions | No code change — `ApiKeyScope` not modeled as a closed enum in this crate (scopes are `Vec<String>`), so new scope strings need no update. |
| 2026-06-18 | Sanity limits enforced on orderbook subscriptions | No code change — operational rate limit, not a schema change. |
| 2026-06-18 | Quote time filters and pagination fix | No code change — `GetQuotesParams` filter surface not exhaustively modeled; pagination fix is server-side behavior only. |
| 2026-06-19 | Communications RFQ and quote retention window reduced | No code change — operational retention-window change only. |
| 2026-06-20 | RFQ quote market and event filters removed | No code change — `market_ticker`/`event_ticker` quote filters were not modeled as typed fields on `GetQuotesParams`. |
| 2026-06-23 | Get Quote rate-limit cost reduced to 2 tokens | No code change — rate-limit accounting is server-side. |
| 2026-06-24 | RFQ quotes support post-only on FIX | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-06-25 | RFQ quote retention and RFQ-scoped quote actions | No code change — RFQ-scoped (`rfq_id`-in-path) quote action endpoints not yet added; existing quote-ID-only actions remain supported per the entry. Tracked as a gap. |
| 2026-06-25 | API usage tier qualification requirements halved | No code change — account-tier qualification is server-side accounting. |
| 2026-06-25 | FIX exchange index routing | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-06-26 | Margin risk per-market metrics limited to single-position subaccounts and gross margin markets | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-29 | Margin positions margin_used omitted for jointly-margined portfolio positions | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-06-30 | Trade-scoped API key permissions | No code change — `ApiKeyScope` is an unconstrained `Vec<String>`; new scope strings need no update. |
| 2026-07-02 | Multivariate lookup history endpoints are fully deprecated | **Removed** — `PUT/GET .../lookup` confirmed gone from `openapi.yaml`; `get_multivariate_event_collection_lookup_history` / `lookup_tickers_for_market_in_multivariate_event_collection` and their request/response types removed. |
| 2026-07-02 | Margin positions now include an is_portfolio flag | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-07-02 | price_ranges added to market_lifecycle_v2 events | **Added** — `price_ranges: Option<Vec<PriceRange>>` on `WsMarketLifecycleV2`/`Ref`. |
| 2026-07-02 | Per-index exchange status | No code change — `GetExchangeStatusResponse` uses a flatten `extra` catch-all; new `exchange_index_statuses` fields round-trip losslessly. Not promoted to typed fields this pass. |
| 2026-07-02 | Per-index subaccount balances | No code change — `GetSubaccountBalancesResponse` shape change (one row per exchange index) tolerated by existing structure; not re-verified field-by-field this pass. |
| 2026-07-02 | AcceptQuote rejects carry a specific reason on FIX | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-07-02 | More specific FIX rejects for cancel/replace failures | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-07-02 | Sub-account-restricted API keys | No code change — `POST /api_keys` already accepts arbitrary request bodies via typed struct; `subaccount` param not yet added to `CreateApiKeyRequest`. Tracked as a gap. |
| 2026-07-04 | Exchange announcements endpoint removed | **Confirmed still present in `openapi.yaml`** — `GET /exchange/announcements` still exists as of this snapshot; `get_exchange_announcements` left in place. (Upstream may have reverted or the changelog entry may describe a later rollback; re-check on next refresh.) |
| 2026-07-09 | Support for FIX Tag 2446 on Incremental Refresh | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-07-09 | RFQ-scoped quote lookup endpoint | No code change — RFQ-scoped lookup not added; existing quote-ID-only `get_quote` remains supported per the entry. |
| 2026-07-09 | Deprecated Predictions REST schema fields removed | **Removed** — `Market.response_price_units`, `Market.fractional_trading_enabled`, `MarketPosition.resting_orders_count`. |
| 2026-07-09 | Margin orders now identify system order reasons | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-07-22 | Incentive programs on hidden events excluded from listing | No code change — server-side filtering behavior only. |
| 2026-07-23 | Order groups limited to 25,000 per user | No code change — server-side limit, not a schema change. |
| 2026-07-23 | Historical positions endpoint | No code change — `GET /historical/positions` not yet modeled. Tracked as a gap. |
| 2026-07-23 | Subaccount-restricted API keys can open WebSocket sessions | No code change — behavior/authorization change only; private channels already modeled. |
| 2026-07-23 | Subaccount-restricted API keys can quote on RFQ FIX sessions | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-07-23 | Pyth value WebSocket channel | No code change — new `pyth_value` channel not yet modeled. Tracked as a gap (see `WsChannelV2`). |
| 2026-07-23 | New price level structures | No code change — `price_level_structure` is modeled as `Option<String>` (not a closed enum), so new structure values round-trip without a crate update. Snap-to-`price_ranges` guidance already documented. |
| 2026-07-28 | The service field on error responses is deprecated | Superseded by the 2026-08-06 removal entry below. |
| 2026-07-30 | Richer combo-validation errors on multivariate market creation | No code change — `ErrorResponse.details: Option<String>` already carries the richer message/details text losslessly. |
| 2026-07-30 | Lifecycle creation messages now include exchange_index | **Added** — `exchange_index: Option<i64>` on `WsMarketLifecycleV2`/`Ref` and `WsEventLifecycle`/`Ref`. |
| 2026-07-30 | Series responses include exchange_index | No code change — not added to `Series` this pass; tracked as a follow-up (see spec-parity.md). |
| 2026-07-30 | New endpoint for event-keyed live data | No code change — `GET /live_data/events/{event_ticker}` not yet modeled. Tracked as a gap. |
| 2026-07-30 | Subaccount-restricted API keys can read order queue positions | No code change — behavior/authorization change only; endpoints already modeled. |
| 2026-07-30 | Event product_metadata now includes cadence | No code change — `EventMetadata` (product_metadata) not verified field-by-field this pass; likely tolerated via existing structure. |
| 2026-07-30 | Subaccount-restricted API keys can use batch order endpoints | No code change — behavior/authorization change only; batch endpoints already modeled (now via V2 batch methods). |
| 2026-07-30 | Subaccount on quote_created | No code change — WS `quote_created` message shape not re-verified field-by-field this pass. |
| 2026-07-30 | Subaccount-restricted API keys can manage order groups | No code change — behavior/authorization change only; order-group endpoints already modeled. |
| 2026-08-06 | Multivariate lookup endpoint and channel removed | **Removed** — WS `multivariate` channel / `multivariate_lookup` message type and `WsMultivariate`/`WsMultivariateRef` deleted; REST `PUT .../lookup` confirmed gone from `openapi.yaml` and removed (see the 2026-07-02 entry above). |
| 2026-08-06 | FIX execution reports identify the source exchange index | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-08-06 | Sided leverage estimates on margin markets | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-08-06 | Order group limit updates support subaccounts | No code change — `UpdateOrderGroupLimitRequest`/params not re-verified field-by-field this pass. |
| 2026-08-06 | Multivariate event collections include exchange_index | No code change — not added to `MultivariateEventCollection` this pass; tracked as a follow-up. |
| 2026-08-06 | The service field has been removed from error responses | **Removed** — `ErrorResponse.service` field and its use in `build_http_error`'s emptiness check. |
| 2026-08-13 | New center_deci_edge_centi_cent price level structure | No code change — `price_level_structure` is `Option<String>`, not a closed enum; new values round-trip without a crate update. |
| 2026-08-13 | Balance reads scoped by exchange_index | No code change — `exchange_index` query param not added to `get_balance`. Tracked as a gap. |
| 2026-08-13 | Block trade indicator for WebSocket trades | **Added** — `is_block_trade: bool` on `WsTrade`/`WsTradeRef`. |
| 2026-08-13 | Exchange shard descriptions | No code change — `exchange_index_statuses` uses a flatten `extra` catch-all on `GetExchangeStatusResponse`. |
| 2026-08-13 | Margin order groups bind to single exchange_index | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-08-13 | Order group maximum increased to 100,000 per user | No code change — server-side limit only. |
| 2026-08-13 | Richer combo-validation errors on FIX RFQ creation | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-08-13 | Intra-account transfer history endpoints | No code change — new endpoints not yet modeled. Tracked as a gap. |
| 2026-08-16 | API key location attestation expiry | No code change — `api_key_region_expiration_ts` not added to the `ApiKey` struct this pass. Tracked as a gap. |
| 2026-08-20 | VPC peering for Prime members | No code change — infrastructure/connectivity option, not an API schema change. |
| 2026-08-20 | Kalshi Weather Index endpoint | No code change — new `GET /live_data/weather/{city}` endpoint not yet modeled. Tracked as a gap. |
| 2026-08-20 | Maker fee exemption for independent NFL combo markets | No code change — fee computation is server-side; `FeeType`/fee fields unaffected. |
| 2026-08-20 | Entry timestamps for FIX market data | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-08-20 | Cross-shard subaccount transfers | No code change — `source_subaccount`/`destination_subaccount` params on the intra-exchange transfer endpoint not re-verified this pass. |
| 2026-08-20 | Target balance allocation endpoints | No code change — new endpoints not yet modeled. Tracked as a gap. |
| 2026-08-20 | Resting order value breakdown by exchange index | No code change — `GetPortfolioRestingOrderTotalValueResponse.resting_order_value_breakdown` not added this pass. |
| 2026-08-20 | Exchange index on portfolio and WebSocket fill records | **Added** (REST) — `exchange_index: Option<i64>` on `Order`, `MarketPosition`, `Fill`, `Settlement`. **Not added** to `WsFill` this pass — tracked as a follow-up. |
| 2026-08-20 | Exchange index filters for portfolio lists | No code change — `exchange_index` query filter not added to `GetOrdersParams`/`GetPositionsParams`/`GetFillsParams` this pass. Tracked as a follow-up. |
| 2026-08-20 | RFQs and combo-market creation for sub-account-restricted API keys | No code change — behavior/authorization change only; endpoints already modeled. |
| 2026-08-20 | Optional balance reads by exchange_index | No code change — `exchange_index` query param not added to `get_balance` this pass (duplicate of the 2026-08-13 entry above). |
| 2026-08-20 | Exit triggers on margin positions | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-08-22 | Post-only quotes preserved; crossing rate limits may apply | No code change — `post_only` already modeled on quote creation; rate-limit behavior is server-side. |
| 2026-08-22 | Combo RFQ fee assignment for briefly resting orders | No code change — fee computation is server-side. |
| 2026-08-24 | Upcoming exchange sharding | No code change — advance notice of a routing change, not a schema change. |
| 2026-08-27 | Localized market content in REST responses | No code change — opt-in via the `Accept-Language` request header, which callers can already set on the underlying `reqwest` client; no schema change. |
| 2026-08-27 | Trade type on FIX market data | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-08-27 | Exchange index on user order messages | No code change — `exchange_index` not added to `WsUserOrder` this pass. Tracked as a follow-up. |
| 2026-08-27 | Cancel-all-orders endpoints | No code change — new endpoints not yet modeled. Tracked as a gap. |
| 2026-08-27 | Historical CF Benchmarks values via the REST passthrough | No code change — documentation-only addition to an already-modeled endpoint (`get_live_data`/CF Benchmarks passthrough). |
| 2026-08-27 | The available_on_brokers field on event responses is deprecated | Superseded by the 2026-09-10 removal entry below. |
| 2026-08-27 | Exchange auto-routing enabled by default | No code change — server-side routing default; `exchange_index`/`market_ticker` params unaffected. |
| 2026-08-27 | Margin maker-volume incentive programs | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-08-29 | Structured target images in Trade API v2 | No code change — `details.image_url` carried losslessly via the structured-target response's existing flatten/Value handling. |
| 2026-08-31 | Weather index calibration history | No code change — new endpoint not yet modeled; weather-index surface out of scope for this crate. |
| 2026-09-03 | CF Benchmarks 5Hz value websocket channel | No code change — new `cfbenchmarks_value_5hz` channel not yet modeled. Tracked as a gap. |
| 2026-09-03 | Higher FIX market data session limit | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-09-03 | Order identity on FIX market data | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-09-03 | Margin fee tier rates | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-03 | Filter FCM orders by client order IDs | No code change — `client_order_ids` filter not added to `GetFcmOrdersParams` this pass. |
| 2026-09-03 | Filter historical positions by subaccount | No code change — `GET /historical/positions` not modeled at all (see 2026-07-23 entry). |
| 2026-09-03 | Correct remaining counts after crossing order amendments | No code change — behavior/bug fix on an already-modeled response field (`remaining_count`). |
| 2026-09-03 | Lower rate-limit cost for cancel all orders | No code change — rate-limit accounting is server-side. |
| 2026-09-03 | Shard rebalance margin reservation | No code change — `resting_margin_reservation` request field belongs to the not-yet-modeled `target_balance_allocation` endpoint. |
| 2026-09-03 | ClearingBusinessDate on FIX trade execution reports | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-09-03 | Tapered sub-cent pricing on multivariate (combo) markets | No code change — `price_level_structure` is `Option<String>`; prices already read from `*_dollars` fixed-point fields per existing guidance. |
| 2026-09-10 | Per-shard margin order rate limits | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-10 | The deprecated available_on_brokers field is removed from event responses | **Removed** — `Event.available_on_brokers` (already dropped in this pass; deprecation entry above is the same field). |
| 2026-09-10 | Principal-only sizing for target-cost RFQs | No code change — `target_cost_excludes_fees` not added to `CreateRfqRequest` this pass. Tracked as a follow-up. |
| 2026-09-10 | Margin taker-volume incentive programs | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-10 | Weather index points expose receipt_basis | No code change — weather-index surface out of scope for this crate. |
| 2026-09-10 | Margin markets expose asset_class | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-10 | Upcoming exchange sharding for commodities and basketball | No code change — advance notice of a routing change, not a schema change. |
| 2026-09-10 | The center_deci_edge_centi_cent price level structure is emitted again | No code change — bug fix on a value already tolerated by `Option<String>` modeling. |
| 2026-09-10 | WebSocket schemas corrected to match the messages the service sends | Reviewed — `seq` already present on the crate's wire types for the named channels; `market_id`/`market_ticker` were never modeled on `WsError` (nothing to remove); error codes 6/16/17 are not hardcoded in the crate (errors are `code: Option<i64>`, not a closed enum), so their retirement needs no change. |
| 2026-09-17 | Margin market important information | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-17 | Margin market responses return the configured tick size | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-09-17 | WebSocket schema corrections | No code change — `WsTicker.dollar_volume`/`dollar_open_interest` are already `i64` (signed), matching the corrected AsyncAPI. Margin-specific corrections out of scope. |
| 2026-09-17 | FIX EventResendRequest (35=U1) Gated | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-09-17 | Historical fills and orders support min_ts | No code change — `min_ts` filter not verified/added to the historical fills/orders params this pass. |
| 2026-09-17 | Reduced rate limit cost for QuoteConfirm when providing the RFQ ID. | No code change — rate-limit accounting is server-side. |
| 2026-09-17 | Target balance allocations include their reservation policy | No code change — belongs to the not-yet-modeled `target_balance_allocation` endpoint. |
| 2026-09-17 | Series responses include a categories list | No code change — `categories` not added to the `Series` struct this pass. Tracked as a follow-up. |
| 2026-09-17 | Returning to idiomatic MVE series | No code change — ticker/series naming convention change only, not a schema change. |
| 2026-09-17 | WebSocket subscriptions are ready when acknowledged | No code change — server-side ordering bug fix. |
| 2026-09-17 | RFQ and quote writes share the shard 1 rate-limit budget | No code change — rate-limit accounting is server-side. |
| 2026-09-24 | Ed25519 API keys | No code change — `auth.rs` remains RSA-PSS SHA256 only; Ed25519 signing/`key_type` request field not implemented this pass. This is a real capability gap, not a parsing nuance — documented in spec-parity.md and tracked as a follow-up feature, not attempted here given scope. |
| 2026-09-24 | 20% higher read and write rate limits | No code change — rate-limit accounting is server-side. |
| 2026-09-24 | Optional WebSocket compression | No code change — `permessage-deflate` negotiation is a `tokio-tungstenite`/transport-level concern, not a message-schema change; the crate does not currently offer it explicitly. |
| 2026-09-24 | Rebalancing without resting-order reservation | No code change — belongs to the not-yet-modeled `target_balance_allocation` endpoint. |
| 2026-09-24 | Subaccount-scoped historical fills and orders | No code change — historical fills/orders `subaccount` filter not verified this pass. |
| 2026-09-24 | Orders historical cutoff advances independently | No code change — behavior-only; `orders_updated_ts` already modeled on `GetHistoricalCutoffResponse`. |
| 2026-10-01 | Exit trigger prices must be positive | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-10-01 | Reduce-only orders over FIX | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-10-01 | ClearingBusinessDate on Margin FIX trade execution reports | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |
| 2026-10-01 | user_orders messages include last_update_reason | **Added** — `last_update_reason: Option<String>` on `WsUserOrder`. (REST `Order.last_update_reason` not confirmed in `openapi.yaml`; not added to REST.) |
| 2026-10-01 | Filter communications RFQs to your own user | No code change — `user_filter` subscription param not added to the communications channel subscription this pass. Tracked as a follow-up. |
| 2026-10-01 | The deprecated liquidity_dollars field is removed from market responses | **Removed** — `Market.liquidity_dollars`. |
| 2026-10-01 | RFQ and quote creation timestamps over FIX | No code change — FIX protocol is not implemented by this crate (REST/WebSocket only); this entry is FIX-only. |
| 2026-10-01 | Ticker reference_price for Pyth-indexed perps | No code change — Margin trading surface not modeled beyond `get_margin_fee_tiers`; this entry is Margin-only. |

### Breaking

- [Rust API] Removed the legacy `/portfolio/orders` order-mutation surface: Kalshi removed
  `POST`/`DELETE` on `/portfolio/orders`, `/portfolio/orders/{order_id}`,
  `/portfolio/orders/{order_id}/amend`, `/portfolio/orders/{order_id}/decrease`, and
  `/portfolio/orders/batched` from the OpenAPI spec (only `GET` remains). Removed
  `create_order`, `cancel_order`, `amend_order`, `decrease_order`, `batch_create_orders`,
  `batch_cancel_orders` and their request/response types (`CreateOrderRequest`,
  `CreateOrderResponse`, `CancelOrderParams`, `CancelOrderResponse`, `AmendOrderRequest`,
  `AmendOrderResponse`, `DecreaseOrderRequest`, `DecreaseOrderResponse`,
  `BatchCreateOrdersRequest`, `BatchCreateOrdersResponse`, `BatchCreateOrdersIndividualResponse`,
  `BatchCancelOrdersRequestOrder`, `BatchCancelOrdersRequest`, `BatchCancelOrdersResponse`,
  `BatchCancelOrdersIndividualResponse`). Use the V2 event-order endpoints instead
  (`create_order_v2`, `cancel_order_v2`, `amend_order_v2`, `decrease_order_v2`,
  `batch_create_orders_v2`, `batch_cancel_orders_v2`, already present since 0.6.0).
  `get_orders`/`get_order` (read-only) and order-group endpoints are unaffected.
- [Rust API] Removed `Market.response_price_units`, `Market.fractional_trading_enabled`,
  `Market.liquidity_dollars`, `MarketPosition.resting_orders_count`, and
  `Event.available_on_brokers` — all confirmed removed upstream by the changelog (see
  disposition table above; `docs/spec-parity.md` records the exact entries).
- [Rust API] Removed `ErrorResponse.service` (removed upstream 2026-08-06; branch on `code`
  instead).
- [Rust API] Removed the WebSocket `multivariate` channel and `multivariate_lookup` message:
  `WsChannelV2::Multivariate`, `WsMsgType::Multivariate`, `WsMsgType::MultivariateLookup`, and
  the `WsMultivariate`/`WsMultivariateRef` types no longer exist (subscriptions to `multivariate`
  now return an unknown-channel error upstream). `multivariate_market_lifecycle` is unaffected.
- [Rust API] Removed the REST multivariate lookup surface: `PUT/GET
  /multivariate_event_collections/{collection_ticker}/lookup` is gone from `openapi.yaml`.
  Removed `get_multivariate_event_collection_lookup_history`,
  `lookup_tickers_for_market_in_multivariate_event_collection`, and their request/response types
  (`GetMultivariateEventCollectionLookupHistoryParams`,
  `GetMultivariateEventCollectionLookupHistoryResponse`, `LookupPoint`,
  `LookupTickersForMarketInMultivariateEventCollectionRequest`,
  `LookupTickersForMarketInMultivariateEventCollectionResponse`). Use the communications (RFQ)
  APIs or `POST /multivariate_event_collections/{collection_ticker}` instead.
- [Rust API] Removed `WsMarketLifecycleV2::fractional_trading_enabled` and the
  `WsMarketLifecycleEventType::FractionalTradingUpdated` variant (no longer in the AsyncAPI
  `event_type` enum for `market_lifecycle_v2`).
- [Rust API] Removed the dead, unused `MarketPositionRef`/`EventPositionRef` types from
  `src/ws/types/mod.rs`. They duplicated the REST `MarketPosition`/`EventPosition` shape but were
  never wired into any WebSocket message parsing path and did not match any real message schema.

### Added

- [Rust API] `exchange_index: Option<i64>` on `Market`, `Order`, `MarketPosition`, `Fill`, and
  `Settlement` (REST), reflecting the exchange-sharding rollout.
- [Rust API] `WsMarketLifecycleV2`/`Ref` gained top-level `strike_type`, `cap_strike`,
  `custom_strike` (present on `metadata_updated` events), `price_ranges` (on `created` /
  `price_level_structure_updated` events), and `exchange_index` (on `created` events).
  `WsEventLifecycle`/`Ref` gained `exchange_index`.
- [Rust API] `WsTrade`/`WsTradeRef` gained `is_block_trade: bool`.
- [Rust API] `WsUserOrder` gained `last_update_reason: Option<String>`.

### Fixed

- [Tests] Fixed two pre-existing compile errors in the test suite (present before this refresh,
  unrelated to the changes above) that made `cargo test` fail to build entirely:
  `WsMessageV2::ListSubscriptions`/`WsMessageRef::ListSubscriptions` match patterns in
  `envelope.rs` tests didn't account for the `sid`/`seq` fields added in 0.7.0, and
  `tests/rest_auth.rs` still referenced the pre-0.6.0 `GetAccountApiLimitsResponse` shape
  (`read_limit`/`write_limit` instead of `read.bucket_capacity`/`write.bucket_capacity`).


## [0.7.0] - 2026-08-12

### Compatibility

- Docs snapshot: 2026-06-08
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-06-08

### Fixed

- [Rust API] Preserved Kalshi WebSocket subscription cursor metadata on unknown frames in both
  owned and borrowed parsing paths. `WsMessageV2::subscription_id()` / `.sequence()` and
  `WsMessageRef::subscription_id()` / `.sequence()` now return unknown-frame `sid` / `seq`.

### Breaking

- [Rust API] All public `WsMessageV2` and `WsMessageRef` control and `Unknown` variants carry
  `sid` and `seq`. Downstream constructors and exhaustive matches must supply or handle the new
  fields (or use `..`). Consumers accounting for the per-subscription cursor MUST use
  `subscription_id()` and `sequence()` rather than matching message variants.


## [0.6.0] - 2026-06-08

### Compatibility

- Docs snapshot: 2026-06-08
- OpenAPI: 3.20.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-06-08

**Changelog entries since 0.5.0 watermark (2026-06-04) and disposition:**

| Entry | Action |
|---|---|
| Margin fee-tier returns active rates (2026-06-03/11) | No code change — exchange bug fix only |
| Perps volume/OI notional fields on margin markets (2026-06-05/11) | No code change — margin market types not in crate |
| Tick size on `GET /margin/markets` (2026-06-03/11) | No code change — margin market types not in crate |
| Automated API rate-limit tiers / grants (2026-06-06) | **Breaking** — replaced `GetAccountApiLimitsResponse`; added `BucketLimit`, `ApiUsageLevelGrant`; added `GET /account/endpoint_costs` (`get_account_endpoint_costs`, `GetAccountEndpointCostsResponse`, `EndpointTokenCost`) |
| Fractional contract quantities for RFQs (2026-05-26/2026-06-11) | No code change — `contracts_fp` already present in `CreateRfqRequest` |
| Legacy order endpoints cost 10× rate-limit tokens (2026-06-04) | No code change — operational rate-limit change only |
| Post Only Cross Cancel `last_update_reason` value (2026-06-04) | No code change — `last_update_reason` not modeled in `Order`; tolerated by existing `extra` flatten if present |
| Transfer-scoped API key permissions (2026-06-03) | No code change — scopes stored as `Vec<String>` already |
| Block trade indicators on public trade endpoints (2026-05-29/2026-06-01) | Added `is_block_trade` to `Trade` and `GetTradesParams` |
| V2 event-order endpoints (`/portfolio/events/orders/*`) | Added all V2 types and six new `KalshiRestClient` methods |
| `cfbenchmarks_value` AsyncAPI channel | Added full channel, subscription, and message support |
| `FeeType::quadratic_with_maker_fees` | Added `QuadraticWithMakerFees` variant to `FeeType` enum |

### Added

- [Rust API] Added `is_block_trade: bool` (with `#[serde(default)]`) to the public REST `Trade`
  struct (2026-05-29). Defaults to `false` for payloads predating the flag.
- [Rust API] Added `is_block_trade: Option<bool>` filter to `GetTradesParams` so callers can filter
  by block-trade status on `GET /markets/trades` and `GET /historical/trades`.
- [Rust API] Added all V2 event-order types and six new `KalshiRestClient` methods for the lower-cost
  `/portfolio/events/orders/*` endpoints: `create_order_v2`, `cancel_order_v2`, `amend_order_v2`,
  `decrease_order_v2`, `batch_create_orders_v2`, `batch_cancel_orders_v2`. These endpoints use a
  single price + `BookSide` instead of separate yes/no prices.
  New request/response types: `CreateOrderV2Request`, `CreateOrderV2Response`,
  `CancelOrderV2Params`, `CancelOrderV2Response`, `AmendOrderV2Request`, `AmendOrderV2Response`,
  `DecreaseOrderV2Request`, `DecreaseOrderV2Response`, `BatchCreateOrdersV2Request`,
  `BatchCreateOrderV2OrderResponse`, `BatchCreateOrdersV2Response`,
  `BatchCancelOrderV2RequestOrder`, `BatchCancelOrdersV2Request`,
  `BatchCancelOrderV2OrderResponse`, `BatchCancelOrdersV2Response`.
- [Rust API] Added `BucketLimit` and `ApiUsageLevelGrant` structs (2026-06-06). `BucketLimit` holds
  `refill_rate: i64` and `bucket_capacity: i64`. `ApiUsageLevelGrant` holds `exchange_instance`,
  `level`, `source: String`, and `expires_ts: Option<i64>` (absent for non-expiring grants).
- [Rust API] Added `get_account_endpoint_costs()` method and `GetAccountEndpointCostsResponse` /
  `EndpointTokenCost` structs for the new public `GET /account/endpoint_costs` endpoint, which lists
  API v2 endpoints whose token cost differs from the default cost.
- [Rust API] Added CF Benchmarks subscription-update support so the documented post-subscribe
  workflow is reachable: `WsUpdateAction::SubscribeIndices` / `UnsubscribeIndices` / `Indexlist`
  variants and an `index_ids: Option<Vec<String>>` field on `WsUpdateSubscriptionParamsV2`. The
  subscription tracker now folds index add/remove updates into the resubscribe state, and
  `validate_update` enforces that index actions carry no market targets and that
  `subscribe_indices` / `unsubscribe_indices` include `index_ids`.
- [Rust API] Added `FeeType::QuadraticWithMakerFees` variant (serialized
  `quadratic_with_maker_fees`). `FeeType` now also carries an `#[serde(other)] Unknown` catch-all
  so unknown future variants never panic.
- [Rust API] Added full `cfbenchmarks_value` channel support:
  - `WsChannelV2::CfbenchmarksValue` variant
  - `index_ids: Option<Vec<String>>` parameter on `WsSubscriptionParamsV2` (use `["all"]` for all
    indices)
  - `WsMsgType::CfbenchmarksValue` and `WsMsgType::CfbenchmarksValueIndexlist` variants
  - New types `WsCfBenchmarksValue`, `WsCfBenchmarksValueRef`, `WsCfBenchmarksAvgData`,
    `WsCfBenchmarksIndexList`, `WsCfBenchmarksIndexListRef` in `ws::types::messages::cfbenchmarks`
  - `WsDataMessageV2::CfbenchmarksValue` and `WsDataMessageV2::CfbenchmarksValueIndexlist` variants
    routed through both the wire and envelope parse paths


### Changed

- [Rust API] `GetAccountApiLimitsResponse` now reflects the current OpenAPI shape: nested
  `read: BucketLimit` and `write: BucketLimit` objects plus `grants: Vec<ApiUsageLevelGrant>`.
  The old flat `read_limit: i64` / `write_limit: i64` fields are removed.

### Breaking

- [Rust API] `GetAccountApiLimitsResponse` field layout changed (automated API rate-limit tiers,
  2026-06-06). Replace `resp.read_limit` → `resp.read.refill_rate` (or `.bucket_capacity`) and
  `resp.write_limit` → `resp.write.refill_rate`. The `grants` field is new; downstream exhaustive
  struct destructuring must add it.
- [Rust API] `WsUpdateAction` gained `SubscribeIndices`, `UnsubscribeIndices`, and `Indexlist`
  variants, and `WsUpdateSubscriptionParamsV2` gained an `index_ids` field. Downstream code with
  exhaustive matches over `WsUpdateAction` or struct-literal construction of
  `WsUpdateSubscriptionParamsV2` must be updated.



## [0.5.0] - 2026-05-29

### Compatibility

- Docs snapshot: 2026-05-29
- Validated through changelog: 2026-06-04

### Added

- [Rust API] Added `BookSide` enum (`Bid` | `Ask` | `Unknown`) to `types.rs` for the normalized
  `book_side` field added to order/fill responses on 2026-05-07.
- [Rust API] Added `outcome_side: Option<YesNo>` and `book_side: Option<BookSide>` fields to
  `Order`, `Fill`, `WsFill`, `WsFillRef`, and `WsUserOrder`. These are the normalized direction
  fields Kalshi added on 2026-05-07 (`bid` ≡ `yes`, `ask` ≡ `no`).
- [Rust API] Added `taker_outcome_side: Option<TradeTakerSide>` and `taker_book_side:
  Option<BookSide>` to the public `Trade` (REST) and `WsTrade` / `WsTradeRef` (WebSocket) objects,
  matching the normalized taker-direction fields added to trade responses on 2026-05-07.
- [Rust API] Added `balance_dollars: Option<FixedPointDollars>` to `GetBalanceResponse` for the
  centi-cent precision balance field added on 2026-05-28 (direct members only).
- [Rust API] Added `subaccount: Option<u32>` to `CreateOrderGroupResponse` for the field added on
  2026-05-07 (0 = primary, 1–32 = subaccount).
- [Rust API] Added `rfq_user_filter: Option<String>` to `GetQuotesParams` for the filter parameter
  added on 2026-05-07. Pass `"self"` to restrict to quotes on the authenticated user's RFQs.
- [Rust API] Added `WsMarketLifecycleEventType::MetadataUpdated` variant for the new lifecycle event
  type added on 2026-05-11, fired when market metadata (name, title, subtitles) changes.
- [Rust API] Surfaced the top-level `metadata_updated` payload values on `WsMarketLifecycleV2` /
  `WsMarketLifecycleV2Ref`: added `floor_strike: Option<f64>` and `yes_sub_title: Option<String>`
  (per AsyncAPI these appear at the top level only on `metadata_updated`, distinct from the
  `additional_metadata.*` copies emitted on creation), plus a top-level flatten `extra` map so other
  conditional lifecycle keys are no longer silently discarded.
- [Rust API] Added the `event_fee_update` WebSocket message: new `WsEventFeeUpdate` /
  `WsEventFeeUpdateRef` types, a `WsMsgType::EventFeeUpdate` variant, and
  `WsDataMessageV2::EventFeeUpdate` / `WsDataMessageRef::EventFeeUpdate` variants. This message is
  delivered on the existing `market_lifecycle_v2` channel and carries `event_ticker`,
  `fee_type_override`, and `fee_multiplier_override` (both overrides `null` when cleared).
  Previously these messages surfaced as `WsMessageV2::Unknown`.
- [Rust API] Added the spec-required `ts_ms` (matching-engine timestamp, ms) to `WsOrderGroupUpdate`
  and `WsOrderGroupUpdateRef`, which were previously dropping the field.
- [Rust API] Added `get_margin_fee_tiers()` method and `GetMarginFeeTiersResponse` struct for the
  `GET /margin/fee_tiers` endpoint. The response uses `maker_fee_rates` / `taker_fee_rates` (market
  ticker → decimal fee rate maps, fee = `notional * rate`).
- [Tests] Added `ws_fill_normalized_fields_parse` test covering the new `outcome_side` / `book_side`
  fields on `WsFill`.

### Changed

- [Rust API] Updated `KalshiEnvironment::demo()` and `KalshiEnvironment::production()` to use the
  dedicated external API hosts introduced on 2026-05-07. REST hosts: `external-api.demo.kalshi.co` /
  `external-api.kalshi.com`. WS hosts: `external-api-ws.demo.kalshi.co` /
  `external-api-ws.kalshi.com`. The old hosts (`demo-api.kalshi.co`, `api.elections.kalshi.com`)
  are no longer used.

### Breaking

- [Rust API] `Order.side` changed from `YesNo` to `Option<YesNo>`. The `side` field was deprecated
  by Kalshi on 2026-05-07 and removed ~2026-05-28. Downstream code must use `outcome_side` (or
  handle `None`).
- [Rust API] `Order.action` changed from `BuySell` to `Option<BuySell>`. Same deprecation/removal
  timeline as `Order.side`. Use `book_side` instead.
- [Rust API] `Fill.side` changed from `YesNo` to `Option<YesNo>` for the same reason.
- [Rust API] `Fill.action` changed from `BuySell` to `Option<BuySell>` for the same reason.
- [Rust API] `WsFill.side` changed from `YesNo` to `Option<YesNo>` for the same reason.
- [Rust API] `WsFill.action` changed from `BuySell` to `Option<BuySell>` for the same reason.
- [Rust API] `Trade.taker_side` and `WsTrade.taker_side` changed from `TradeTakerSide` to
  `Option<TradeTakerSide>`. The `taker_side` field was deprecated on 2026-05-07 in favor of
  `taker_outcome_side` / `taker_book_side`. Downstream code must handle `None`.
- [Rust API] `KalshiEnvironment::demo()` and `KalshiEnvironment::production()` now point to the new
  dedicated external API hostnames. Code that hard-coded the old host strings must update.
- [Upstream] `GET /margin/fee_tiers` response no longer returns `maker_fee_tiers` /
  `taker_fee_tiers` tier-name maps; it now returns `maker_fee_rates` / `taker_fee_rates` decimal
  maps. `GetMarginFeeTiersResponse` was added with the new shape (no old shape existed in this
  crate).


## [0.4.0] - 2026-04-18

### Compatibility

- Docs snapshot: 2026-04-18
- OpenAPI: 3.13.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-04-16

### Added

- [Rust API] Added REST helpers for current Kalshi endpoints and aliases, including `get_market_orderbooks`, `get_trades_historical`, `get_fills_historical`, `get_live_data_by_milestone`, `get_game_stats`, and `get_market_candlesticks_historical`.
- [Rust API] Added current OpenAPI fields used by the refreshed docs, including `occurrence_datetime` on event and market payloads, `series_ticker` on historical market filters, and fixed-point quote contract fields.
- [Docs] Added `VERSIONING.md` plus repo guidance that points refresh work at the live Kalshi docs, changelog RSS, OpenAPI, and AsyncAPI documents instead of checked-in spec snapshots.

### Changed

- [Rust API] Restored `GetOrderQueuePositionsParams` to the current OpenAPI behavior by allowing unfiltered queue-position requests.
- [Rust API] Migrated the WebSocket public surface to the current V2 contract, including `WsChannelV2`, `WsMessageV2`, `WsDataMessageV2`, `WsSubscriptionParamsV2`, and the `subscribe_v2` / `unsubscribe_v2` / `update_subscription_v2` / `start_reader_v2` / `next_event_v2` methods.
- [Rust API] Aligned authenticated REST response structs with the current OpenAPI fixed-point contract for `Order`, `Trade`, `Fill`, `Settlement`, `MarketPosition`, and `EventPosition`.
- [Rust API] Aligned communications REST and WebSocket quote/RFQ payloads with the current fixed-point-only docs by removing stale integer compatibility fields and relying on `*_dollars` and `*_fp` fields.
- [Upstream] Validated the current Kalshi docs snapshot against the changelog items covering historical `series_ticker` filtering, fixed-point response cleanup, millisecond WebSocket timestamps, and `occurrence_datetime` on market responses.
- [Tests] Refreshed parsing fixtures to the current OpenAPI/AsyncAPI field sets, added coverage for `occurrence_datetime`, and added deterministic V2 WebSocket command-behavior coverage.
- [Tests] Updated live integration coverage to use the filters and account-scope assumptions required by the current communications, queue-position, and FCM-only portfolio endpoints.
- [Upstream] Updated docs, examples, and tests for Kalshi's current WebSocket handshake behavior, which now requires authenticated connections even when subscribing only to public channels.
- [Docs] Tightened the refresh workflow to remove upstream-removed schema fields and response shapes from the public Rust API instead of preserving compatibility shims by default.

### Removed

- [Docs] Removed vendored OpenAPI/AsyncAPI snapshots, spec manifest artifacts, the parity generation script, and raw spec contract tests in favor of live upstream docs plus concise `docs/spec-parity.md` notes.
- [Rust API] Removed stale REST compatibility fields and aliases that are no longer present in the current OpenAPI, including legacy fill/settlement fixed-point aliases.
- [Rust API] Removed stale WebSocket fill aliases for `yes_price_fixed` and `no_price_fixed` so parsing follows the current AsyncAPI names.
- [Rust API] Removed stale quote and RFQ integer compatibility fields from REST and WebSocket communications payloads.
- [Rust API] Removed stale WebSocket compatibility fields and shapes from `WsTicker`, `WsTrade`, `WsOrderbookSnapshot`, `WsOrderbookDelta`, and `WsFill`; downstream consumers must use the current `*_dollars` and `*_fp` fields from the live AsyncAPI contract.
- [Rust API] Removed the stale `GetMarketOrderbookResponse.orderbook` compatibility view and its synthesized integer orderbook shape; the current OpenAPI response is `orderbook_fp` only.

### Breaking

- [Rust API] Downstream WebSocket code must migrate from the pre-V2 types and methods such as `WsChannel`, `WsMessage`, `WsDataMessage`, `subscribe`, `unsubscribe`, `update_subscription`, `start_reader`, and `next_event` to the V2 names and `*_v2` methods.
- [Rust API] `KalshiWsClient::connect` and `KalshiWsLowLevelClient::connect` no longer provide an unauthenticated public-channel path; downstream code must use `connect_authenticated`, even for public subscriptions.
- [Rust API] V2 subscription validation is stricter: `orderbook_delta` requires `market_ticker` or `market_tickers`, rejects `market_id` and `market_ids`, and enforces exclusive market-target fields on subscribe and update commands.
- [Rust API] Downstream code must update authenticated REST response field access to the current spec names such as `fill_count_fp`, `remaining_count_fp`, `initial_count_fp`, `last_update_time`, `subaccount_number`, `total_traded_dollars`, `market_exposure_dollars`, `total_cost_dollars`, and `total_cost_shares_fp`.
- [Rust API] Legacy integer/count response fields and compatibility aliases previously accepted by `Order`, `Trade`, `Fill`, `Settlement`, `MarketPosition`, and `EventPosition` are no longer exposed by the public Rust types.
- [Rust API] Downstream WebSocket code can no longer access removed compatibility fields such as `price`, `yes_bid`, `yes_ask`, `volume`, `open_interest`, `count`, `yes_price`, `no_price`, `delta`, `no_price_dollars`, or the legacy integer orderbook snapshot levels on current V2 message types.
- [Rust API] Downstream REST code must read `GetMarketOrderbookResponse.orderbook_fp` directly; the legacy `orderbook` field has been removed.

## [0.3.0] - 2026-03-05

### Compatibility

- Not recorded for this historical release.

### Added

- [Rust API] Added `MarketStatusConversionError` for strict lifecycle/query status conversions.
- [Rust API] Added best-effort `From` conversions between lifecycle `MarketStatus` and query `MarketStatusQuery`.
- [Rust API] Added strict `TryFrom<&...>` conversions for exact one-to-one status mapping.
- [Tests] Added and expanded parsing tests for status serialization and conversion behavior.
- [Rust API] Added `KalshiError::Parse` with parse context, human-readable reason, raw payload bytes, and optional serde source error.
- [Rust API] Added public parse accessors on `KalshiError`: `parse_context()`, `parse_error_reason()`, and `parse_raw_bytes()`.
- [Tests] Added regression tests covering REST and WebSocket parse failures to verify reason text and raw-byte preservation.

### Changed

- [Rust API] Renamed query enum `MarketStatus` to `MarketStatusQuery`.
- [Rust API] Renamed REST market lifecycle enum `MarketState` to `MarketStatus`.
- [Rust API] Updated `GetMarketsParams.status` to use `Option<MarketStatusQuery>`.
- [Rust API] Updated `Market.status` to use `Option<MarketStatus>`.
- [Docs] Updated examples, tests, and REST module docs to use the new names.
- [Rust API] REST success-response decoding now returns `KalshiError::Parse` with raw bytes instead of a plain serde JSON error.
- [Rust API] WebSocket envelope and message parsing now returns `KalshiError::Parse` with clearer parse-failure context and preserved raw payload bytes.

### Removed

- [Rust API] Removed old `MarketState` and old query `MarketStatus` names without aliases.

### Breaking

- [Rust API] Downstream consumers must update imports and enum references to the new names.
- [Rust API] Downstream exhaustive `match` statements over `KalshiError` must handle the new `Parse` variant.
