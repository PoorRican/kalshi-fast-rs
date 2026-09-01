//! Portfolio endpoints: balance, positions, fills, settlements, resting order value.
//!
//! All endpoints here require authentication. Positions and orders share the
//! same taxonomy split (market vs event), and fills/settlements provide
//! post-trade accounting feeds.

use crate::KalshiError;
use crate::rest::account::EmptyResponse;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{
    BookSide, BuySell, ExchangeIndex, FixedPointCount, FixedPointDollars, IndexedBalance,
    PositionCountFilter, YesNo, deserialize_null_as_empty_vec, serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// GET /portfolio/balance query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBalanceParams {
    /// 0..=32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,

    /// Scope the balance and portfolio value to a single exchange index. If
    /// omitted, both include all exchange indexes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBalanceResponse {
    pub balance: i64,
    pub portfolio_value: i64,
    pub updated_ts: i64,
    /// Centi-cent precision dollar balance (direct members only). Added 2026-05-28.
    #[serde(default)]
    pub balance_dollars: Option<FixedPointDollars>,
    /// User balance breakdown per exchange instance, omitted only when using
    /// a subaccount-restricted API key.
    #[serde(default)]
    pub balance_breakdown: Option<Vec<IndexedBalance>>,
}

/// GET /portfolio/positions query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 1000

    /// CSV of non-zero filters (position,total_traded)
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub count_filter: Option<Vec<PositionCountFilter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// CSV max 10
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub event_ticker: Option<Vec<String>>,

    /// 0..=32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,

    /// Filter results by exchange shard. Omit to return results from all
    /// exchange shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

impl GetPositionsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit
            && (limit == 0 || limit > 1000)
        {
            return Err(KalshiError::InvalidParams(
                "GET /portfolio/positions: limit must be 1..=1000".to_string(),
            ));
        }
        if let Some(evts) = &self.event_ticker
            && evts.len() > 10
        {
            return Err(KalshiError::InvalidParams(
                "GET /portfolio/positions: event_ticker supports up to 10 tickers".to_string(),
            ));
        }
        if let Some(sub) = self.subaccount
            && sub > 32
        {
            return Err(KalshiError::InvalidParams(
                "subaccount must be 0..=32".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketPosition {
    pub ticker: String,
    pub exchange_index: ExchangeIndex,
    pub total_traded_dollars: FixedPointDollars,
    pub position_fp: FixedPointCount,
    pub market_exposure_dollars: FixedPointDollars,
    pub realized_pnl_dollars: FixedPointDollars,
    #[serde(default)]
    pub resting_orders_count: Option<i32>,
    pub fees_paid_dollars: FixedPointDollars,
    pub last_updated_ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventPosition {
    pub event_ticker: String,
    pub total_cost_dollars: FixedPointDollars,
    pub total_cost_shares_fp: FixedPointCount,
    pub event_exposure_dollars: FixedPointDollars,
    pub realized_pnl_dollars: FixedPointDollars,
    pub fees_paid_dollars: FixedPointDollars,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPositionsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub market_positions: Vec<MarketPosition>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub event_positions: Vec<EventPosition>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PositionsPage {
    pub market_positions: Vec<MarketPosition>,
    pub event_positions: Vec<EventPosition>,
}

impl From<GetPositionsResponse> for PositionsPage {
    fn from(resp: GetPositionsResponse) -> Self {
        Self {
            market_positions: resp.market_positions,
            event_positions: resp.event_positions,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settlement {
    pub ticker: String,
    pub exchange_index: ExchangeIndex,
    pub event_ticker: String,
    pub market_result: String,
    pub yes_count_fp: FixedPointCount,
    pub yes_total_cost_dollars: FixedPointDollars,
    pub no_count_fp: FixedPointCount,
    pub no_total_cost_dollars: FixedPointDollars,
    pub revenue: i64,
    pub settled_time: String,
    pub fee_cost: FixedPointDollars,
    #[serde(default)]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSettlementsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSettlementsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub settlements: Vec<Settlement>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fill {
    pub fill_id: String,
    pub exchange_index: ExchangeIndex,
    pub order_id: String,
    pub trade_id: String,
    pub ticker: String,
    pub market_ticker: String,
    /// Deprecated 2026-05-07; removed ~2026-05-28. Use `outcome_side`.
    #[serde(default)]
    pub side: Option<YesNo>,
    /// Deprecated 2026-05-07; removed ~2026-05-28. Use `book_side`.
    #[serde(default)]
    pub action: Option<BuySell>,
    /// Normalized outcome side (yes | no). Added 2026-05-07.
    #[serde(default)]
    pub outcome_side: Option<YesNo>,
    /// Normalized book side (bid | ask). Added 2026-05-07.
    #[serde(default)]
    pub book_side: Option<BookSide>,
    pub count_fp: FixedPointCount,
    pub yes_price_dollars: FixedPointDollars,
    pub no_price_dollars: FixedPointDollars,
    pub is_taker: bool,
    pub fee_cost: FixedPointDollars,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub subaccount_number: Option<u32>,
    #[serde(default)]
    pub ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    /// Filter results by exchange shard. Omit to return results from all
    /// exchange shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFillsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub fills: Vec<Fill>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPortfolioRestingOrderTotalValueResponse {
    pub total_resting_order_value: i64,
    /// Total value of resting orders broken down by exchange index, with
    /// each balance expressed as a fixed-point dollar string.
    pub resting_order_value_breakdown: Vec<IndexedBalance>,
}

/// One exchange index's target share of sweepable balance, as returned by
/// `GET /portfolio/target_balance_allocation`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetBalanceAllocation {
    pub exchange_index: ExchangeIndex,
    pub percent: i32,
}

/// One exchange index's target share of sweepable balance, as submitted to
/// `POST /portfolio/target_balance_allocation`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetBalanceAllocationInput {
    pub exchange_index: ExchangeIndex,
    pub percent: i32,
}

/// Collateral an automatic rebalance leaves behind for resting orders. `Max`
/// reserves the largest single market-side commitment; `Sum` reserves the
/// summed margin of every resting order. Defaults to `Sum` when omitted.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RestingMarginReservation {
    Max,
    Sum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTargetBalanceAllocationResponse {
    pub allocations: Vec<TargetBalanceAllocation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetTargetBalanceAllocationRequest {
    pub allocations: Vec<TargetBalanceAllocationInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting_margin_reservation: Option<RestingMarginReservation>,
}

impl KalshiRestClient {
    /// Get the account balance.
    ///
    /// **Requires auth.**
    pub async fn get_balance(
        &self,
        params: GetBalanceParams,
    ) -> Result<GetBalanceResponse, KalshiError> {
        let path = Self::full_path("/portfolio/balance");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List open positions. Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_positions(
        &self,
        params: GetPositionsParams,
    ) -> Result<GetPositionsResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/portfolio/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List fills (executed trades). Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_fills(&self, params: GetFillsParams) -> Result<GetFillsResponse, KalshiError> {
        let path = Self::full_path("/portfolio/fills");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List settlements. Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_settlements(
        &self,
        params: GetSettlementsParams,
    ) -> Result<GetSettlementsResponse, KalshiError> {
        let path = Self::full_path("/portfolio/settlements");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_portfolio_total_resting_order_value(
        &self,
    ) -> Result<GetPortfolioRestingOrderTotalValueResponse, KalshiError> {
        let path = Self::full_path("/portfolio/summary/total_resting_order_value");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Get the caller's target balance allocation across exchange indexes.
    ///
    /// **Requires auth.**
    pub async fn get_target_balance_allocation(
        &self,
    ) -> Result<GetTargetBalanceAllocationResponse, KalshiError> {
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Replace the caller's target balance allocation across exchange
    /// indexes. Percentages must total 100. Passing an empty allocations
    /// array disables automatic rebalancing.
    ///
    /// **Requires auth.**
    pub async fn set_target_balance_allocation(
        &self,
        body: SetTargetBalanceAllocationRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Create a pager for iterating over positions page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn positions_pager(&self, params: GetPositionsParams) -> CursorPager<PositionsPage> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_positions(page_params).await?;
                let cursor = resp.cursor.clone();
                let page = PositionsPage::from(resp);
                Ok((vec![page], cursor))
            })
        })
    }

    /// Create a pager for iterating over fills page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn fills_pager(&self, params: GetFillsParams) -> CursorPager<Fill> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_fills(page_params).await?;
                Ok((resp.fills, resp.cursor))
            })
        })
    }

    /// Create a pager for iterating over settlements page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn settlements_pager(&self, params: GetSettlementsParams) -> CursorPager<Settlement> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_settlements(page_params).await?;
                Ok((resp.settlements, resp.cursor))
            })
        })
    }

    /// Stream positions one by one.
    ///
    /// **Requires auth.**
    pub fn stream_positions(
        &self,
        params: GetPositionsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<PositionsPage, KalshiError>> + Send {
        stream_items(self.positions_pager(params), max_items)
    }

    /// Stream fills one by one.
    ///
    /// **Requires auth.**
    pub fn stream_fills(
        &self,
        params: GetFillsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Fill, KalshiError>> + Send {
        stream_items(self.fills_pager(params), max_items)
    }

    /// Stream settlements one by one.
    ///
    /// **Requires auth.**
    pub fn stream_settlements(
        &self,
        params: GetSettlementsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Settlement, KalshiError>> + Send {
        stream_items(self.settlements_pager(params), max_items)
    }
}
