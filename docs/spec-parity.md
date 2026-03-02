# Spec Parity Report

Generated from snapshot manifests in `docs/specs/kalshi/`.

## Snapshot Versions

- OpenAPI info version: `3.7.0`
- AsyncAPI info version: `2.0.0`

## REST Coverage

- Operation coverage: `77/77` path+method operations mapped

| Method | Path | Operation ID | Client Method |
|---|---|---|---|
| `GET` | `/account/limits` | `GetAccountApiLimits` | `get_account_api_limits` |
| `GET` | `/api_keys` | `GetApiKeys` | `get_api_keys` |
| `POST` | `/api_keys` | `CreateApiKey` | `create_api_key` |
| `POST` | `/api_keys/generate` | `GenerateApiKey` | `generate_api_key` |
| `DELETE` | `/api_keys/{api_key}` | `DeleteApiKey` | `delete_api_key` |
| `GET` | `/communications/id` | `GetCommunicationsID` | `get_communications_id` |
| `GET` | `/communications/quotes` | `GetQuotes` | `get_quotes` |
| `POST` | `/communications/quotes` | `CreateQuote` | `create_quote` |
| `DELETE` | `/communications/quotes/{quote_id}` | `DeleteQuote` | `delete_quote` |
| `GET` | `/communications/quotes/{quote_id}` | `GetQuote` | `get_quote` |
| `PUT` | `/communications/quotes/{quote_id}/accept` | `AcceptQuote` | `accept_quote` |
| `PUT` | `/communications/quotes/{quote_id}/confirm` | `ConfirmQuote` | `confirm_quote` |
| `GET` | `/communications/rfqs` | `GetRFQs` | `get_rfqs` |
| `POST` | `/communications/rfqs` | `CreateRFQ` | `create_rfq` |
| `DELETE` | `/communications/rfqs/{rfq_id}` | `DeleteRFQ` | `delete_rfq` |
| `GET` | `/communications/rfqs/{rfq_id}` | `GetRFQ` | `get_rfq` |
| `GET` | `/events` | `GetEvents` | `get_events` |
| `GET` | `/events/multivariate` | `GetMultivariateEvents` | `get_multivariate_events` |
| `GET` | `/events/{event_ticker}` | `GetEvent` | `get_event` |
| `GET` | `/events/{event_ticker}/metadata` | `GetEventMetadata` | `get_event_metadata` |
| `GET` | `/exchange/announcements` | `GetExchangeAnnouncements` | `get_exchange_announcements` |
| `GET` | `/exchange/schedule` | `GetExchangeSchedule` | `get_exchange_schedule` |
| `GET` | `/exchange/status` | `GetExchangeStatus` | `get_exchange_status` |
| `GET` | `/exchange/user_data_timestamp` | `GetUserDataTimestamp` | `get_user_data_timestamp` |
| `GET` | `/fcm/orders` | `GetFCMOrders` | `get_fcm_orders` |
| `GET` | `/fcm/positions` | `GetFCMPositions` | `get_fcm_positions` |
| `GET` | `/incentive_programs` | `GetIncentivePrograms` | `get_incentive_programs` |
| `GET` | `/live_data/batch` | `GetLiveDatas` | `get_live_data_batch` |
| `GET` | `/live_data/{type}/milestone/{milestone_id}` | `GetLiveData` | `get_live_data` |
| `GET` | `/markets` | `GetMarkets` | `get_markets` |
| `GET` | `/markets/candlesticks` | `BatchGetMarketCandlesticks` | `batch_get_market_candlesticks` |
| `GET` | `/markets/trades` | `GetTrades` | `get_trades` |
| `GET` | `/markets/{ticker}` | `GetMarket` | `get_market` |
| `GET` | `/markets/{ticker}/orderbook` | `GetMarketOrderbook` | `get_market_orderbook` |
| `GET` | `/milestones` | `GetMilestones` | `get_milestones` |
| `GET` | `/milestones/{milestone_id}` | `GetMilestone` | `get_milestone` |
| `GET` | `/multivariate_event_collections` | `GetMultivariateEventCollections` | `get_multivariate_event_collections` |
| `GET` | `/multivariate_event_collections/{collection_ticker}` | `GetMultivariateEventCollection` | `get_multivariate_event_collection` |
| `POST` | `/multivariate_event_collections/{collection_ticker}` | `CreateMarketInMultivariateEventCollection` | `create_market_in_multivariate_event_collection` |
| `GET` | `/multivariate_event_collections/{collection_ticker}/lookup` | `GetMultivariateEventCollectionLookupHistory` | `get_multivariate_event_collection_lookup_history` |
| `PUT` | `/multivariate_event_collections/{collection_ticker}/lookup` | `LookupTickersForMarketInMultivariateEventCollection` | `lookup_tickers_for_market_in_multivariate_event_collection` |
| `GET` | `/portfolio/balance` | `GetBalance` | `get_balance` |
| `GET` | `/portfolio/fills` | `GetFills` | `get_fills` |
| `GET` | `/portfolio/order_groups` | `GetOrderGroups` | `get_order_groups` |
| `POST` | `/portfolio/order_groups/create` | `CreateOrderGroup` | `create_order_group` |
| `DELETE` | `/portfolio/order_groups/{order_group_id}` | `DeleteOrderGroup` | `delete_order_group` |
| `GET` | `/portfolio/order_groups/{order_group_id}` | `GetOrderGroup` | `get_order_group` |
| `PUT` | `/portfolio/order_groups/{order_group_id}/limit` | `UpdateOrderGroupLimit` | `update_order_group_limit` |
| `PUT` | `/portfolio/order_groups/{order_group_id}/reset` | `ResetOrderGroup` | `reset_order_group` |
| `PUT` | `/portfolio/order_groups/{order_group_id}/trigger` | `TriggerOrderGroup` | `trigger_order_group` |
| `GET` | `/portfolio/orders` | `GetOrders` | `get_orders` |
| `POST` | `/portfolio/orders` | `CreateOrder` | `create_order` |
| `DELETE` | `/portfolio/orders/batched` | `BatchCancelOrders` | `batch_cancel_orders` |
| `POST` | `/portfolio/orders/batched` | `BatchCreateOrders` | `batch_create_orders` |
| `GET` | `/portfolio/orders/queue_positions` | `GetOrderQueuePositions` | `get_order_queue_positions` |
| `DELETE` | `/portfolio/orders/{order_id}` | `CancelOrder` | `cancel_order` |
| `GET` | `/portfolio/orders/{order_id}` | `GetOrder` | `get_order` |
| `POST` | `/portfolio/orders/{order_id}/amend` | `AmendOrder` | `amend_order` |
| `POST` | `/portfolio/orders/{order_id}/decrease` | `DecreaseOrder` | `decrease_order` |
| `GET` | `/portfolio/orders/{order_id}/queue_position` | `GetOrderQueuePosition` | `get_order_queue_position` |
| `GET` | `/portfolio/positions` | `GetPositions` | `get_positions` |
| `GET` | `/portfolio/settlements` | `GetSettlements` | `get_settlements` |
| `POST` | `/portfolio/subaccounts` | `CreateSubaccount` | `create_subaccount` |
| `GET` | `/portfolio/subaccounts/balances` | `GetSubaccountBalances` | `get_subaccount_balances` |
| `POST` | `/portfolio/subaccounts/transfer` | `ApplySubaccountTransfer` | `transfer_subaccount` |
| `GET` | `/portfolio/subaccounts/transfers` | `GetSubaccountTransfers` | `get_subaccount_transfers` |
| `GET` | `/portfolio/summary/total_resting_order_value` | `GetPortfolioRestingOrderTotalValue` | `get_portfolio_total_resting_order_value` |
| `GET` | `/search/filters_by_sport` | `GetFiltersForSports` | `get_filters_by_sport` |
| `GET` | `/search/tags_by_categories` | `GetTagsForSeriesCategories` | `get_tags_by_categories` |
| `GET` | `/series` | `GetSeriesList` | `get_series_list` |
| `GET` | `/series/fee_changes` | `GetSeriesFeeChanges` | `get_series_fee_changes` |
| `GET` | `/series/{series_ticker}` | `GetSeries` | `get_series` |
| `GET` | `/series/{series_ticker}/events/{ticker}/candlesticks` | `GetMarketCandlesticksByEvent` | `get_event_market_candlesticks` |
| `GET` | `/series/{series_ticker}/events/{ticker}/forecast_percentile_history` | `GetEventForecastPercentilesHistory` | `get_event_forecast_percentile_history` |
| `GET` | `/series/{series_ticker}/markets/{ticker}/candlesticks` | `GetMarketCandlesticks` | `get_market_candlesticks` |
| `GET` | `/structured_targets` | `GetStructuredTargets` | `get_structured_targets` |
| `GET` | `/structured_targets/{structured_target_id}` | `GetStructuredTarget` | `get_structured_target` |

## WebSocket Coverage

- AsyncAPI channels represented: `12` channels in snapshot
- Subscribe channel enum represented in V2 models: `10` channels
- Command payload families represented: `6` command messages

### Snapshot Channels

- `communications`
- `control_frames`
- `fill`
- `market_lifecycle_v2`
- `market_positions`
- `multivariate`
- `order_group_updates`
- `orderbook_delta`
- `root`
- `ticker`
- `trade`
- `user_orders`

### Required V2 Client Methods

- `subscribe_v2`
- `unsubscribe_v2`
- `update_subscription_v2`
- `next_message_v2`
- `next_event_v2`
- `start_reader_v2`
- `close`
- `shutdown_timeout`

### Required V2 Types

- `WsChannelV2`
- `WsSubscriptionParamsV2`
- `WsUnsubscribeParamsV2`
- `WsUpdateSubscriptionParamsV2`
- `WsUpdateAction`
- `WsMessageV2`
- `WsDataMessageV2`
- `WsUserOrder`
