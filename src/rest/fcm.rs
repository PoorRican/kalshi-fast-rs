//! Event-contract FCM endpoints. All operations require authentication.
//!
//! FCM subtrader identifiers are full identifiers, not subaccount numbers. A
//! subtrader-bound key can omit the identifier on the orders, positions and
//! daily-cap read endpoints; the server uses the key's bound subtrader.

use crate::KalshiError;
use crate::rest::account::EmptyResponse;
use crate::rest::client::KalshiRestClient;
use crate::rest::orders::{GetOrdersResponse, Order};
use crate::rest::pagination::{CursorPager, stream_items};
use crate::rest::portfolio::{GetPositionsResponse, PositionsPage, SettlementStatusFilter};
use crate::types::{
    FixedPointCount, FixedPointDollars, OrderStatus, YesNo, deserialize_null_as_empty_vec,
    serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Filters for fills across the authenticated FCM's subtraders.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A fill as seen by an FCM. Maker and taker ownership information is present
/// only when the corresponding side belongs to the FCM; both can be present.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FcmFill {
    pub fill_id: String,
    pub exchange_index: i32,
    pub ticker: String,
    pub taker_outcome_side: YesNo,
    pub count_fp: FixedPointCount,
    pub yes_price_dollars: FixedPointDollars,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub maker_order_id: Option<String>,
    #[serde(default)]
    pub maker_subtrader_id: Option<String>,
    #[serde(default)]
    pub maker_fee_cost: Option<FixedPointDollars>,
    #[serde(default)]
    pub taker_order_id: Option<String>,
    #[serde(default)]
    pub taker_subtrader_id: Option<String>,
    #[serde(default)]
    pub taker_fee_cost: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFcmFillsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub fills: Vec<FcmFill>,
    #[serde(default)]
    pub cursor: Option<String>,
}

/// An unrestricted FCM key needs `subtrader_id` or `client_order_ids`. A
/// subtrader-bound key defaults an omitted identifier to its own subtrader.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    /// Comma-separated on the wire, at most 100 IDs. The server searches only
    /// the last 24 hours; IDs are not globally unique across subtraders.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub client_order_ids: Option<Vec<String>>,
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

/// Filters for FCM positions. Omit `subtrader_id` only for a bound key; the
/// server then uses the key's bound subtrader.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    /// Comma-separated nonzero-position field filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    /// The server defaults to unsettled positions when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<SettlementStatusFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FcmSubtrader {
    pub subtrader_id: String,
    pub exchange_indices: Vec<i32>,
    /// Effective block, including firm-wide and Kalshi restrictions.
    pub trading_blocked: bool,
    /// The FCM's own per-subtrader block.
    pub fcm_trading_blocked: bool,
    pub propagation_pending: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFcmSubtradersResponse {
    pub subtraders: Vec<FcmSubtrader>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateFcmSubtraderRequest {
    /// 1–16 case-sensitive ASCII alphanumeric characters. The server forms
    /// the full ID as `{account_id}_{subtrader_suffix}`.
    pub subtrader_suffix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateFcmSubtraderResponse {
    pub subtrader_id: String,
}

/// Query for a daily cap. A bound key defaults an omitted ID to its own ID.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmEventContractDailyCapParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
}

/// Required query identifier for an unrestricted FCM operation.
#[derive(Debug, Clone, Serialize)]
pub struct FcmSubtraderParams {
    pub subtrader_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFcmEventContractDailyCapResponse {
    pub subtrader_id: String,
    pub limit: FixedPointDollars,
    pub executed_utilization: FixedPointDollars,
    pub resting_order_utilization: FixedPointDollars,
    pub pending_order_utilization: FixedPointDollars,
    /// New York calendar date of executed utilization.
    pub cap_date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFcmEventContractDailyCapRequest {
    pub subtrader_id: String,
    pub limit: FixedPointDollars,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFcmSubtraderBlockedCategoriesResponse {
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFcmSubtraderBlockedCategoriesRequest {
    pub subtrader_id: String,
    /// One event category, 1–100 characters.
    pub category: String,
    /// `true` adds the category; `false` removes it.
    pub blocked: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateFcmSubtraderBlockedCategoriesResponse {
    pub categories: Vec<String>,
}

/// Move one market position into a subtrader of the FCM's configured error
/// account. Kalshi must enable the endpoint and supply the error account ID.
#[derive(Debug, Clone, Serialize)]
pub struct CreateFcmPositionTransferRequest {
    pub source_subtrader_id: String,
    pub target_subtrader_id: String,
    pub market_ticker: String,
    /// Signed whole contracts multiplied by 100: positive YES, negative NO.
    pub position_centicount: i64,
    /// YES price, even for NO positions, in centicents and whole cents.
    pub price_centicents: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateFcmPositionTransferResponse {
    /// Save this ID: the API does not provide a transfer-history endpoint.
    pub position_transfer_id: String,
}

impl KalshiRestClient {
    /// Get fills across an unrestricted FCM's subtraders.
    pub async fn get_fcm_fills(
        &self,
        params: GetFcmFillsParams,
    ) -> Result<GetFcmFillsResponse, KalshiError> {
        let path = Self::full_path("/fcm/fills");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get orders for FCM subtraders, including via a subtrader-bound key.
    pub async fn get_fcm_orders(
        &self,
        params: GetFcmOrdersParams,
    ) -> Result<GetOrdersResponse, KalshiError> {
        let path = Self::full_path("/fcm/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get positions for an FCM subtrader.
    pub async fn get_fcm_positions(
        &self,
        params: GetFcmPositionsParams,
    ) -> Result<GetPositionsResponse, KalshiError> {
        let path = Self::full_path("/fcm/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List event-contract subtraders. New accounts may not appear immediately.
    pub async fn list_fcm_subtraders(&self) -> Result<ListFcmSubtradersResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn create_fcm_subtrader(
        &self,
        body: CreateFcmSubtraderRequest,
    ) -> Result<CreateFcmSubtraderResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Read a subtrader's cap and utilization. No configured cap returns 404.
    pub async fn get_fcm_event_contract_daily_cap(
        &self,
        params: GetFcmEventContractDailyCapParams,
    ) -> Result<GetFcmEventContractDailyCapResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders/event_contract_daily_cap");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Set the cap. Existing resting orders are not cancelled by a cap change.
    pub async fn update_fcm_event_contract_daily_cap(
        &self,
        body: UpdateFcmEventContractDailyCapRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders/event_contract_daily_cap");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Remove the cap. Bound-key new orders are rejected until a cap is set.
    pub async fn delete_fcm_event_contract_daily_cap(
        &self,
        params: FcmSubtraderParams,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders/event_contract_daily_cap");
        self.send(
            Method::DELETE,
            &path,
            Some(&params),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_fcm_subtrader_blocked_categories(
        &self,
        params: FcmSubtraderParams,
    ) -> Result<GetFcmSubtraderBlockedCategoriesResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders/blocked_categories");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Update one category; blocks apply prospectively, not to resting orders.
    pub async fn update_fcm_subtrader_blocked_categories(
        &self,
        body: UpdateFcmSubtraderBlockedCategoriesRequest,
    ) -> Result<UpdateFcmSubtraderBlockedCategoriesResponse, KalshiError> {
        let path = Self::full_path("/fcm/subtraders/blocked_categories");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn create_fcm_position_transfer(
        &self,
        body: CreateFcmPositionTransferRequest,
    ) -> Result<CreateFcmPositionTransferResponse, KalshiError> {
        let path = Self::full_path("/fcm/positions/transfers");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub fn fcm_fills_pager(&self, params: GetFcmFillsParams) -> CursorPager<FcmFill> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let response = client.get_fcm_fills(page_params).await?;
                Ok((response.fills, response.cursor))
            })
        })
    }

    pub fn fcm_orders_pager(&self, params: GetFcmOrdersParams) -> CursorPager<Order> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let response = client.get_fcm_orders(page_params).await?;
                Ok((response.orders, response.cursor))
            })
        })
    }

    /// Each page contains both market and event positions.
    pub fn fcm_positions_pager(&self, params: GetFcmPositionsParams) -> CursorPager<PositionsPage> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let response = client.get_fcm_positions(page_params).await?;
                let cursor = response.cursor.clone();
                Ok((vec![PositionsPage::from(response)], cursor))
            })
        })
    }

    pub fn stream_fcm_fills(
        &self,
        params: GetFcmFillsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<FcmFill, KalshiError>> + Send {
        stream_items(self.fcm_fills_pager(params), max_items)
    }

    pub fn stream_fcm_orders(
        &self,
        params: GetFcmOrdersParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Order, KalshiError>> + Send {
        stream_items(self.fcm_orders_pager(params), max_items)
    }

    pub fn stream_fcm_positions(
        &self,
        params: GetFcmPositionsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<PositionsPage, KalshiError>> + Send {
        stream_items(self.fcm_positions_pager(params), max_items)
    }

    pub async fn get_fcm_fills_all(
        &self,
        params: GetFcmFillsParams,
    ) -> Result<Vec<FcmFill>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let response = self.get_fcm_fills(page_params).await?;
                Ok((response.fills, response.cursor))
            }
        })
        .await
    }

    pub async fn get_fcm_orders_all(
        &self,
        params: GetFcmOrdersParams,
    ) -> Result<Vec<Order>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let response = self.get_fcm_orders(page_params).await?;
                Ok((response.orders, response.cursor))
            }
        })
        .await
    }

    /// Collect both position categories, preserving page boundaries.
    pub async fn get_fcm_positions_all(
        &self,
        params: GetFcmPositionsParams,
    ) -> Result<Vec<PositionsPage>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let response = self.get_fcm_positions(page_params).await?;
                let cursor = response.cursor.clone();
                Ok((vec![PositionsPage::from(response)], cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fcm_fill_side_ownership_is_independent() {
        let fill: FcmFill = serde_json::from_str(
            r#"{
            "fill_id":"f", "exchange_index":2, "ticker":"T",
            "taker_outcome_side":"yes", "count_fp":"1.00", "yes_price_dollars":"0.4200",
            "maker_subtrader_id":"maker", "maker_fee_cost":"0.0100",
            "taker_subtrader_id":"taker", "taker_fee_cost":"0.0200"
        }"#,
        )
        .unwrap();
        assert_eq!(fill.maker_subtrader_id.as_deref(), Some("maker"));
        assert_eq!(fill.taker_subtrader_id.as_deref(), Some("taker"));
        assert!(fill.maker_order_id.is_none());
        assert!(fill.created_time.is_none());
    }

    #[test]
    fn client_order_ids_are_encoded_as_a_single_csv_filter() {
        let params = GetFcmOrdersParams {
            client_order_ids: Some(vec!["a".into(), "b".into()]),
            ..Default::default()
        };
        let value = serde_json::to_value(params).unwrap();
        assert_eq!(value["client_order_ids"], "a,b");
        assert!(value.get("subtrader_id").is_none());
    }

    #[test]
    fn bound_subtrader_queries_do_not_invent_an_identifier() {
        let orders = serde_json::to_value(GetFcmOrdersParams::default()).unwrap();
        let positions = serde_json::to_value(GetFcmPositionsParams::default()).unwrap();
        let cap = serde_json::to_value(GetFcmEventContractDailyCapParams::default()).unwrap();
        assert!(orders.get("subtrader_id").is_none());
        assert!(positions.get("subtrader_id").is_none());
        assert!(cap.get("subtrader_id").is_none());
    }
}
