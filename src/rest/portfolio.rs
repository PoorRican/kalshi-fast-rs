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
    BookSide, BuySell, FixedPointCount, FixedPointDollars, PositionCountFilter, YesNo,
    deserialize_null_as_empty_vec, serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// A fixed-point dollar balance scoped to one exchange shard.
///
/// Used by the per-exchange-index breakdowns added 2026-08-20.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexedBalance {
    pub exchange_index: i32,
    pub balance: FixedPointDollars,
}

/// GET /portfolio/balance query params.
///
/// `balance` and `portfolio_value` cover every exchange index unless
/// `exchange_index` is supplied (2026-08-20). Passing `subaccount` explicitly
/// (including `0`) scopes the balance to that subaccount (2026-08-13).
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBalanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBalanceResponse {
    pub balance: i64,
    pub portfolio_value: i64,
    pub updated_ts: i64,
    /// Centi-cent precision dollar balance (direct members only). Added 2026-05-28.
    #[serde(default)]
    pub balance_dollars: Option<FixedPointDollars>,
    /// Per-exchange-index balance breakdown. Added 2026-08-20. Omitted for
    /// subaccount-restricted API keys.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub balance_breakdown: Vec<IndexedBalance>,
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

    /// Restrict results to one exchange shard. Added 2026-08-20. Omitting it
    /// returns positions from every exchange index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
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
    pub total_traded_dollars: FixedPointDollars,
    pub position_fp: FixedPointCount,
    pub market_exposure_dollars: FixedPointDollars,
    pub realized_pnl_dollars: FixedPointDollars,
    pub fees_paid_dollars: FixedPointDollars,
    pub last_updated_ts: String,
    /// Exchange shard the position lives on. Added 2026-08-20.
    #[serde(default)]
    pub exchange_index: Option<i32>,
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
    /// Exchange shard the settled market lived on. Added 2026-08-20.
    #[serde(default)]
    pub exchange_index: Option<i32>,
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
    /// Exchange shard the fill occurred on. Added 2026-08-20.
    #[serde(default)]
    pub exchange_index: Option<i32>,
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
    /// Restrict results to one exchange shard. Added 2026-08-20. Omitting it
    /// returns fills from every exchange index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
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
    /// Per-exchange-index resting order value, each balance a fixed-point
    /// dollar string. Added 2026-08-20.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub resting_order_value_breakdown: Vec<IndexedBalance>,
}

/// Collateral an automatic shard rebalance leaves behind for resting orders.
///
/// Added 2026-09-03 (`POST`) / 2026-09-17 (`GET`). `Sum` is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestingMarginReservation {
    /// Reserve the largest single market-side commitment.
    Max,
    /// Reserve the summed margin of every resting order (default).
    Sum,
    #[serde(other)]
    Unknown,
}

impl RestingMarginReservation {
    pub fn as_str(self) -> &'static str {
        match self {
            RestingMarginReservation::Max => "max",
            RestingMarginReservation::Sum => "sum",
            RestingMarginReservation::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for RestingMarginReservation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for RestingMarginReservation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// One exchange-index slice of the sweepable-balance target allocation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetBalanceAllocation {
    pub exchange_index: i32,
    /// Target percentage of sweepable balance, 0..=100.
    pub percent: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTargetBalanceAllocationResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub allocations: Vec<TargetBalanceAllocation>,
    /// Reservation policy in force. Returned even when `allocations` is empty
    /// so callers can preserve it when updating. Added 2026-09-17.
    #[serde(default)]
    pub resting_margin_reservation: Option<RestingMarginReservation>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetTargetBalanceAllocationRequest {
    /// Up to 101 entries. An empty list stops automatic rebalancing.
    pub allocations: Vec<TargetBalanceAllocation>,
    /// Defaults to `sum` upstream when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting_margin_reservation: Option<RestingMarginReservation>,
}

impl SetTargetBalanceAllocationRequest {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if self.allocations.len() > 101 {
            return Err(KalshiError::InvalidParams(
                "POST /portfolio/target_balance_allocation: at most 101 allocations".to_string(),
            ));
        }
        for allocation in &self.allocations {
            if allocation.exchange_index < 0 {
                return Err(KalshiError::InvalidParams(
                    "POST /portfolio/target_balance_allocation: exchange_index must be >= 0"
                        .to_string(),
                ));
            }
            if !(0..=100).contains(&allocation.percent) {
                return Err(KalshiError::InvalidParams(
                    "POST /portfolio/target_balance_allocation: percent must be 0..=100"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntraExchangeInstanceTransferStatus {
    Pending,
    Complete,
    #[serde(other)]
    Unknown,
}

/// A completed or in-flight intra-account transfer between exchange instances
/// or shards. Added 2026-08-13.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransfer {
    pub transfer_id: String,
    /// Source exchange instance (`event_contract` | `margined`). Kept as a raw
    /// string so future instances round-trip without a crate update.
    pub source: String,
    pub destination: String,
    pub source_exchange_shard: i32,
    pub destination_exchange_shard: i32,
    /// Transfer amount as a fixed-point dollar string.
    pub amount: FixedPointDollars,
    pub status: IntraExchangeInstanceTransferStatus,
    pub created_ts: i64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIntraExchangeInstanceTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransfersResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub transfers: Vec<IntraExchangeInstanceTransfer>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransferResponse {
    pub transfer: IntraExchangeInstanceTransfer,
}

impl KalshiRestClient {
    /// Get the account balance across every exchange index.
    ///
    /// Use [`KalshiRestClient::get_balance_scoped`] to scope the response to a
    /// single `exchange_index` or `subaccount`.
    ///
    /// **Requires auth.**
    pub async fn get_balance(&self) -> Result<GetBalanceResponse, KalshiError> {
        self.get_balance_scoped(GetBalanceParams::default()).await
    }

    /// Get the account balance, optionally scoped to one exchange index and/or
    /// subaccount.
    ///
    /// Omitting `exchange_index` returns `balance` and `portfolio_value` summed
    /// across all exchange indexes (2026-08-20).
    ///
    /// **Requires auth.**
    pub async fn get_balance_scoped(
        &self,
        params: GetBalanceParams,
    ) -> Result<GetBalanceResponse, KalshiError> {
        let path = Self::full_path("/portfolio/balance");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Read the target balance allocation across exchange shards.
    ///
    /// **Requires auth.** Added 2026-08-20.
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

    /// Set the target balance allocation across exchange shards.
    ///
    /// Pass an empty `allocations` list to stop automatic rebalancing. When
    /// `resting_margin_reservation` is omitted the exchange defaults to `sum`,
    /// so read the current value first if you need to preserve it.
    ///
    /// **Requires auth.** Added 2026-08-20.
    pub async fn set_target_balance_allocation(
        &self,
        body: SetTargetBalanceAllocationRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        body.validate()?;
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// List intra-exchange-instance transfer history. Supports cursor pagination.
    ///
    /// **Requires auth.** Added 2026-08-13.
    pub async fn get_intra_exchange_instance_transfers(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> Result<GetIntraExchangeInstanceTransfersResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfers");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get one intra-exchange-instance transfer by id.
    ///
    /// **Requires auth.** Added 2026-08-13.
    pub async fn get_intra_exchange_instance_transfer(
        &self,
        transfer_id: &str,
    ) -> Result<GetIntraExchangeInstanceTransferResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/portfolio/intra_exchange_instance_transfers/{transfer_id}"
        ));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Create a pager for iterating over intra-exchange transfers page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn intra_exchange_instance_transfers_pager(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> CursorPager<IntraExchangeInstanceTransfer> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client
                    .get_intra_exchange_instance_transfers(page_params)
                    .await?;
                Ok((resp.transfers, resp.cursor))
            })
        })
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
