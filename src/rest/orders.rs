//! Order endpoints: orders listing, single-order lookups, V2 event-order
//! create/amend/decrease/cancel, batch operations, queue positions, order groups,
//! and FCM subtrader views.
//!
//! All endpoints require authentication.
//!
//! The legacy V1 order mutation endpoints (`POST /portfolio/orders`,
//! `DELETE /portfolio/orders/{order_id}`, `POST /portfolio/orders/{order_id}/amend`,
//! `POST /portfolio/orders/{order_id}/decrease`, and `POST`/`DELETE
//! /portfolio/orders/batched`) were announced as deprecated on 2026-06-18 and are no
//! longer present in the Kalshi OpenAPI spec. Use the V2 event-order methods
//! ([`KalshiRestClient::create_order_v2`] and friends) instead. `GET /portfolio/orders`
//! and `GET /portfolio/orders/{order_id}` are unaffected and remain available.

use crate::KalshiError;
use crate::rest::account::{EmptyResponse, SubaccountQueryParams};
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::rest::portfolio::GetPositionsResponse;
use crate::types::{
    BookSide, BuySell, ErrorResponse, FixedPointCount, FixedPointDollars, OrderStatus, OrderType,
    SelfTradePreventionType, TimeInForce, YesNo, deserialize_null_as_empty_vec, serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// GET /portfolio/orders query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// CSV max 10
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub event_ticker: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 1000

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,

    /// Filter results by exchange shard. Omit to return orders from all exchange
    /// shards. Added 2026-08-20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<u32>,
}

impl GetOrdersParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit
            && (limit == 0 || limit > 1000)
        {
            return Err(KalshiError::InvalidParams(
                "GET /portfolio/orders: limit must be 1..=1000".to_string(),
            ));
        }
        if let Some(evts) = &self.event_ticker
            && evts.len() > 10
        {
            return Err(KalshiError::InvalidParams(
                "GET /portfolio/orders: event_ticker supports up to 10 tickers".to_string(),
            ));
        }
        if let Some(sub) = self.subaccount
            && sub > 63
        {
            return Err(KalshiError::InvalidParams(
                "subaccount must be 0..=63".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    /// Deprecated upstream; still returned. Use [`Order::outcome_side`].
    #[deprecated(since = "0.8.0", note = "use outcome_side (or book_side) instead")]
    #[serde(default)]
    pub side: Option<YesNo>,
    /// Deprecated upstream; still returned. Use [`Order::book_side`].
    #[deprecated(since = "0.8.0", note = "use book_side (or outcome_side) instead")]
    #[serde(default)]
    pub action: Option<BuySell>,
    /// Normalized outcome side (yes | no).
    pub outcome_side: YesNo,
    /// Normalized book side (bid | ask).
    pub book_side: BookSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub yes_price_dollars: FixedPointDollars,
    pub no_price_dollars: FixedPointDollars,
    pub fill_count_fp: FixedPointCount,
    pub remaining_count_fp: FixedPointCount,
    pub initial_count_fp: FixedPointCount,
    pub taker_fill_cost_dollars: FixedPointDollars,
    pub maker_fill_cost_dollars: FixedPointDollars,
    pub taker_fees_dollars: FixedPointDollars,
    pub maker_fees_dollars: FixedPointDollars,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(default)]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(default, rename = "subaccount_number")]
    pub subaccount_number: Option<u32>,
    /// Exchange shard that owns this order. Added 2026-08-20.
    #[serde(default)]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrdersResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub orders: Vec<Order>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrderQueuePositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    /// Defaults to the primary account. Subaccount-restricted keys may omit this
    /// (their locked subaccount is inferred) but may not target another subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

impl GetOrderQueuePositionsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrderQueuePositionsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub queue_positions: Vec<OrderQueuePosition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderQueuePosition {
    pub order_id: String,
    pub market_ticker: String,
    /// Number of preceding shares before the order in the queue.
    pub queue_position_fp: FixedPointCount,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrderQueuePositionResponse {
    /// Number of preceding shares before the order in the queue.
    pub queue_position_fp: FixedPointCount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrderGroupsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub order_groups: Vec<OrderGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderGroup {
    pub id: String,
    /// Current maximum contracts allowed over a rolling 15-second window.
    #[serde(default)]
    pub contracts_limit_fp: Option<FixedPointCount>,
    pub is_auto_cancel_enabled: bool,
    /// Exchange shard this order group is bound to.
    #[serde(default)]
    pub exchange_index: Option<i32>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Query params shared by the order-group endpoints that accept both
/// `subaccount` and `exchange_index`
/// (`DELETE`/`PUT .../reset`/`PUT .../trigger`/`PUT .../limit`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderGroupParams {
    /// Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Exchange shard index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

impl From<SubaccountQueryParams> for OrderGroupParams {
    fn from(params: SubaccountQueryParams) -> Self {
        Self {
            subaccount: params.subaccount,
            exchange_index: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateOrderGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Whole contracts only. Provide `contracts_limit` or `contracts_limit_fp`;
    /// if both are provided they must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
    /// Exchange shard index. Defaults to 0 server-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOrderGroupResponse {
    pub order_group_id: String,
    /// 0 = primary account, 1–63 = subaccount.
    pub subaccount: u32,
    #[serde(default)]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetOrderGroupResponse {
    pub is_auto_cancel_enabled: bool,
    /// Current maximum contracts allowed over a rolling 15-second window.
    #[serde(default)]
    pub contracts_limit_fp: Option<FixedPointCount>,
    /// IDs of the orders that belong to this order group.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub orders: Vec<String>,
    #[serde(default)]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateOrderGroupLimitRequest {
    /// Whole contracts only. Provide `contracts_limit` or `contracts_limit_fp`;
    /// if both are provided they must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
}

pub type GetFcmOrdersResponse = GetOrdersResponse;
pub type GetFcmPositionsResponse = GetPositionsResponse;

// ---------------------------------------------------------------------------
// V2 event-order endpoints  (/portfolio/events/orders/*)
// ---------------------------------------------------------------------------

/// Create Order (V2) body. Uses `BookSide` + single fixed-point price.
///
/// Required: `ticker`, `side`, `count`, `price`, `time_in_force`, `self_trade_prevention_type`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderV2Request {
    pub ticker: String,
    pub side: BookSide,
    pub count: FixedPointCount,
    pub price: FixedPointDollars,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention_type: SelfTradePreventionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Unix seconds; combine with `time_in_force: good_till_canceled` for GTT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// 0 = primary; 1–63 = subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    /// Exchange shard index. Defaults to 0. Use `-1` to auto-route by market ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOrderV2Response {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub fill_count: FixedPointCount,
    pub remaining_count: FixedPointCount,
    #[serde(default)]
    pub average_fill_price: Option<FixedPointDollars>,
    #[serde(default)]
    pub average_fee_paid: Option<FixedPointDollars>,
    pub ts_ms: i64,
}

/// Query params for Cancel Order (V2).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelOrderV2Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Exchange shard index. Use `-1` to auto-route by `market_ticker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    /// Required when `exchange_index` is `-1` (auto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelOrderV2Response {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub reduced_by: FixedPointCount,
    pub ts_ms: i64,
}

/// Amend Order (V2) body. Uses `BookSide` + single fixed-point price.
///
/// Required: `ticker`, `side`, `price`, `count`.
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderV2Request {
    pub ticker: String,
    pub side: BookSide,
    pub price: FixedPointDollars,
    pub count: FixedPointCount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    /// Exchange shard index. Defaults to 0. Use `-1` to auto-route by market ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmendOrderV2Response {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub remaining_count: Option<FixedPointCount>,
    #[serde(default)]
    pub fill_count: Option<FixedPointCount>,
    #[serde(default)]
    pub average_fill_price: Option<FixedPointDollars>,
    #[serde(default)]
    pub average_fee_paid: Option<FixedPointDollars>,
    pub ts_ms: i64,
}

/// Decrease Order (V2) body. Fixed-point strings only; no integer variants.
///
/// Exactly one of `reduce_by` or `reduce_to` must be provided.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DecreaseOrderV2Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<FixedPointCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<FixedPointCount>,
    /// Exchange shard index. Defaults to 0. Use `-1` to auto-route by `market_ticker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    /// Required when `exchange_index` is `-1` (auto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecreaseOrderV2Response {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub remaining_count: FixedPointCount,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchCreateOrdersV2Request {
    pub orders: Vec<CreateOrderV2Request>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchCreateOrderV2OrderResponse {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub fill_count: Option<FixedPointCount>,
    #[serde(default)]
    pub remaining_count: Option<FixedPointCount>,
    #[serde(default)]
    pub average_fill_price: Option<FixedPointDollars>,
    #[serde(default)]
    pub average_fee_paid: Option<FixedPointDollars>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default)]
    pub error: Option<ErrorResponse>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchCreateOrdersV2Response {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub orders: Vec<BatchCreateOrderV2OrderResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCancelOrderV2RequestOrder {
    pub order_id: String,
    /// 0 = primary; 1–63 = subaccount. Subaccount-restricted API keys must omit
    /// this or pass their locked subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Exchange shard index. Defaults to 0. Use `-1` to auto-route by `market_ticker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    /// Required when `exchange_index` is `-1` (auto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCancelOrdersV2Request {
    pub orders: Vec<BatchCancelOrderV2RequestOrder>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchCancelOrderV2OrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub reduced_by: FixedPointCount,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default)]
    pub error: Option<ErrorResponse>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchCancelOrdersV2Response {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub orders: Vec<BatchCancelOrderV2OrderResponse>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmOrdersParams {
    pub subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmPositionsParams {
    pub subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl KalshiRestClient {
    /// List orders with optional filters. Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_orders(
        &self,
        params: GetOrdersParams,
    ) -> Result<GetOrdersResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/portfolio/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_order(&self, order_id: &str) -> Result<GetOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/orders/{order_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_order_queue_positions(
        &self,
        params: GetOrderQueuePositionsParams,
    ) -> Result<GetOrderQueuePositionsResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/portfolio/orders/queue_positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_order_queue_position(
        &self,
        order_id: &str,
    ) -> Result<GetOrderQueuePositionResponse, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/orders/{order_id}/queue_position"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_order_groups(
        &self,
        params: SubaccountQueryParams,
    ) -> Result<GetOrderGroupsResponse, KalshiError> {
        let path = Self::full_path("/portfolio/order_groups");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Create an order group with a contracts limit measured over a rolling
    /// 15-second window. Users can have up to 100,000 order groups at a time.
    ///
    /// **Requires auth.**
    pub async fn create_order_group(
        &self,
        body: CreateOrderGroupRequest,
    ) -> Result<CreateOrderGroupResponse, KalshiError> {
        let path = Self::full_path("/portfolio/order_groups/create");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn get_order_group(
        &self,
        order_group_id: &str,
        params: SubaccountQueryParams,
    ) -> Result<GetOrderGroupResponse, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/order_groups/{order_group_id}"));
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn delete_order_group(
        &self,
        order_group_id: &str,
        params: impl Into<OrderGroupParams>,
    ) -> Result<EmptyResponse, KalshiError> {
        let params = params.into();
        let path = Self::full_path(&format!("/portfolio/order_groups/{order_group_id}"));
        self.send(
            Method::DELETE,
            &path,
            Some(&params),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Update an order group's contracts limit.
    ///
    /// Accepts `subaccount` (added 2026-08-06) and `exchange_index` query params.
    ///
    /// **Requires auth.**
    pub async fn update_order_group_limit(
        &self,
        order_group_id: &str,
        params: impl Into<OrderGroupParams>,
        body: UpdateOrderGroupLimitRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let params = params.into();
        let path = Self::full_path(&format!("/portfolio/order_groups/{order_group_id}/limit"));
        self.send(Method::PUT, &path, Some(&params), Some(&body), true)
            .await
    }

    pub async fn reset_order_group(
        &self,
        order_group_id: &str,
        params: impl Into<OrderGroupParams>,
    ) -> Result<EmptyResponse, KalshiError> {
        let params = params.into();
        let path = Self::full_path(&format!("/portfolio/order_groups/{order_group_id}/reset"));
        let body = EmptyResponse::default();
        self.send(Method::PUT, &path, Some(&params), Some(&body), true)
            .await
    }

    pub async fn trigger_order_group(
        &self,
        order_group_id: &str,
        params: impl Into<OrderGroupParams>,
    ) -> Result<EmptyResponse, KalshiError> {
        let params = params.into();
        let path = Self::full_path(&format!("/portfolio/order_groups/{order_group_id}/trigger"));
        let body = EmptyResponse::default();
        self.send(Method::PUT, &path, Some(&params), Some(&body), true)
            .await
    }

    // --- V2 event-order endpoints ---

    /// Place a new order via the V2 event-order endpoint.
    ///
    /// Uses `BookSide` + single fixed-point `price`.
    ///
    /// **Requires auth.**
    pub async fn create_order_v2(
        &self,
        body: CreateOrderV2Request,
    ) -> Result<CreateOrderV2Response, KalshiError> {
        let path = Self::full_path("/portfolio/events/orders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Cancel an order via the V2 event-order endpoint.
    ///
    /// **Requires auth.**
    pub async fn cancel_order_v2(
        &self,
        order_id: &str,
        params: CancelOrderV2Params,
    ) -> Result<CancelOrderV2Response, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/events/orders/{order_id}"));
        self.send(
            Method::DELETE,
            &path,
            Some(&params),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Amend an order via the V2 event-order endpoint.
    ///
    /// **Requires auth.**
    pub async fn amend_order_v2(
        &self,
        order_id: &str,
        params: SubaccountQueryParams,
        body: AmendOrderV2Request,
    ) -> Result<AmendOrderV2Response, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/events/orders/{order_id}/amend"));
        self.send(Method::POST, &path, Some(&params), Some(&body), true)
            .await
    }

    /// Decrease an order via the V2 event-order endpoint.
    ///
    /// Provide exactly one of `reduce_by` or `reduce_to` in the body.
    ///
    /// **Requires auth.**
    pub async fn decrease_order_v2(
        &self,
        order_id: &str,
        params: SubaccountQueryParams,
        body: DecreaseOrderV2Request,
    ) -> Result<DecreaseOrderV2Response, KalshiError> {
        let path = Self::full_path(&format!("/portfolio/events/orders/{order_id}/decrease"));
        self.send(Method::POST, &path, Some(&params), Some(&body), true)
            .await
    }

    /// Submit a batch of orders via the V2 event-order endpoint.
    ///
    /// Subaccount-restricted API keys may use this endpoint; per-order
    /// `subaccount` values must match the key's locked subaccount or be omitted.
    ///
    /// **Requires auth.**
    pub async fn batch_create_orders_v2(
        &self,
        body: BatchCreateOrdersV2Request,
    ) -> Result<BatchCreateOrdersV2Response, KalshiError> {
        let path = Self::full_path("/portfolio/events/orders/batched");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Cancel a batch of orders via the V2 event-order endpoint.
    ///
    /// Subaccount-restricted API keys may use this endpoint; per-order
    /// `subaccount` values must match the key's locked subaccount or be omitted.
    ///
    /// **Requires auth.**
    pub async fn batch_cancel_orders_v2(
        &self,
        body: BatchCancelOrdersV2Request,
    ) -> Result<BatchCancelOrdersV2Response, KalshiError> {
        let path = Self::full_path("/portfolio/events/orders/batched");
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Some(&body),
            true,
        )
        .await
    }

    pub async fn get_fcm_orders(
        &self,
        params: GetFcmOrdersParams,
    ) -> Result<GetFcmOrdersResponse, KalshiError> {
        let path = Self::full_path("/fcm/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_fcm_positions(
        &self,
        params: GetFcmPositionsParams,
    ) -> Result<GetFcmPositionsResponse, KalshiError> {
        let path = Self::full_path("/fcm/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Create a pager for iterating over orders page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn orders_pager(&self, params: GetOrdersParams) -> CursorPager<Order> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_orders(page_params).await?;
                Ok((resp.orders, resp.cursor))
            })
        })
    }

    /// Stream orders one by one.
    ///
    /// **Requires auth.**
    pub fn stream_orders(
        &self,
        params: GetOrdersParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Order, KalshiError>> + Send {
        stream_items(self.orders_pager(params), max_items)
    }
}
