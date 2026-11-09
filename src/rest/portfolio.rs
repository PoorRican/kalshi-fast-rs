//! Portfolio endpoints: balance, positions, fills, settlements, resting order value.
//!
//! All endpoints here require authentication. Positions and orders share the
//! same taxonomy split (market vs event), and fills/settlements provide
//! post-trade accounting feeds.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{
    BookSide, BuySell, FixedPointCount, FixedPointDollars, PositionCountFilter, YesNo,
    serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Filter balance by subaccount and/or exchange index; omitted indexes are aggregated.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBalanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexedBalance {
    pub exchange_index: i32,
    pub balance: FixedPointDollars,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBalanceResponse {
    pub balance: i64,
    pub portfolio_value: i64,
    pub updated_ts: i64,
    pub balance_dollars: FixedPointDollars,
    /// Omitted for subaccount-restricted API keys.
    #[serde(default)]
    pub balance_breakdown: Option<Vec<IndexedBalance>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatusFilter {
    Unsettled,
    Settled,
    All,
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

    /// A single event ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<SettlementStatusFilter>,
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
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketPosition {
    pub ticker: String,
    pub exchange_index: i32,
    pub total_traded_dollars: FixedPointDollars,
    pub position_fp: FixedPointCount,
    pub market_exposure_dollars: FixedPointDollars,
    pub realized_pnl_dollars: FixedPointDollars,
    pub fees_paid_dollars: FixedPointDollars,
    pub last_updated_ts: String,
    /// Whether netting applies to this position's event; set at the subaccount's first order in the event.
    pub netting_enabled: bool,
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
    pub market_positions: Vec<MarketPosition>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementResult {
    Yes,
    No,
    Scalar,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settlement {
    pub ticker: String,
    pub exchange_index: i32,
    pub event_ticker: String,
    pub market_result: SettlementResult,
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
    pub settlements: Vec<Settlement>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fill {
    pub fill_id: String,
    pub exchange_index: i32,
    pub order_id: String,
    pub trade_id: String,
    pub ticker: String,
    pub market_ticker: String,
    /// Deprecated; use `outcome_side`.
    #[serde(default)]
    pub side: Option<YesNo>,
    /// Deprecated; use `book_side`.
    #[serde(default)]
    pub action: Option<BuySell>,
    pub outcome_side: YesNo,
    pub book_side: BookSide,
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
    /// Up to 100 market tickers, serialized as a comma-separated list.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub ticker: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}
impl GetFillsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if self
            .ticker
            .as_ref()
            .is_some_and(|tickers| tickers.len() > 100)
        {
            return Err(KalshiError::InvalidParams(
                "GET /portfolio/fills: ticker supports up to 100 tickers".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFillsResponse {
    pub fills: Vec<Fill>,
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPortfolioRestingOrderTotalValueResponse {
    pub total_resting_order_value: i64,
    pub resting_order_value_breakdown: Vec<IndexedBalance>,
}
/// Exchange instance between which funds may be transferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeInstance {
    EventContract,
    Margined,
}

/// An intra-account transfer. The request amount is in centicents.
///
/// Numbered subaccounts are supported only for event-contract to event-contract
/// transfers. For a margined leg its exchange shard must be omitted or zero;
/// an event-contract leg may select an exchange shard.
#[derive(Debug, Clone, Serialize)]
pub struct IntraExchangeInstanceTransferRequest {
    /// Primary-account transfers only. Reuse solely for retrying the same
    /// request: an already accepted ID on the same source instance and shard
    /// returns 409 without creating another transfer. Omission creates a new
    /// transfer for each request; Kalshi always generates the returned ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_transfer_id: Option<String>,
    pub source: ExchangeInstance,
    pub destination: ExchangeInstance,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_exchange_shard: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_exchange_shard: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransferResponse {
    pub transfer_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntraExchangeInstanceTransferStatus {
    Pending,
    Complete,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransfer {
    pub transfer_id: String,
    #[serde(default)]
    pub client_transfer_id: Option<String>,
    pub source: ExchangeInstance,
    pub destination: ExchangeInstance,
    pub source_exchange_shard: i32,
    pub destination_exchange_shard: i32,
    /// Fixed-point dollars, unlike the centicent creation request.
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
    pub transfers: Vec<IntraExchangeInstanceTransfer>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransferResponse {
    pub transfer: IntraExchangeInstanceTransfer,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCreditHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Inclusive creation time, in Unix seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    /// Exclusive creation time, in Unix seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditStatus {
    Unknown,
    Pending,
    Failed,
    Applied,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreditHistory {
    pub credit_id: String,
    #[serde(rename = "type")]
    pub credit_type: String,
    pub status: CreditStatus,
    pub amount_dollars: FixedPointDollars,
    pub reason: String,
    pub created_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetCreditHistoryResponse {
    pub credits: Vec<CreditHistory>,
    pub cursor: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDepositsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWithdrawalsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Applied,
    Failed,
    Returned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Ach,
    Wire,
    Crypto,
    Debit,
    Apm,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deposit {
    pub id: String,
    pub status: PaymentStatus,
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    pub amount_cents: i64,
    pub fee_cents: i64,
    pub created_ts: i64,
    #[serde(default)]
    pub finalized_ts: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Withdrawal {
    pub id: String,
    pub status: PaymentStatus,
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    pub amount_cents: i64,
    pub fee_cents: i64,
    pub created_ts: i64,
    #[serde(default)]
    pub finalized_ts: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetDepositsResponse {
    pub deposits: Vec<Deposit>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWithdrawalsResponse {
    pub withdrawals: Vec<Withdrawal>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestingMarginReservation {
    None,
    Max,
    Sum,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetBalanceAllocation {
    pub exchange_index: i32,
    pub percent: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTargetBalanceAllocationResponse {
    pub allocations: Vec<TargetBalanceAllocation>,
    pub resting_margin_reservation: RestingMarginReservation,
}

/// Replaces the allocation; percentages must sum to 100. An empty array
/// disables automatic rebalancing. Reservation defaults to `sum` if omitted.
#[derive(Debug, Clone, Serialize)]
pub struct SetTargetBalanceAllocationRequest {
    pub allocations: Vec<TargetBalanceAllocation>,
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

    /// Transfer funds within the account. Requires authentication.
    ///
    /// Cross-exchange-index subaccount transfers may involve up to three
    /// non-atomic steps: completed steps are not undone if a later step fails,
    /// and funds may remain in a primary account on either index. Same-shard
    /// transfers are treated as subaccount transfers. Restricted keys may only
    /// transfer between event-contract indexes with both subaccounts set to
    /// the key's subaccount.
    pub async fn intra_exchange_instance_transfer(
        &self,
        request: IntraExchangeInstanceTransferRequest,
    ) -> Result<IntraExchangeInstanceTransferResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfer");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Some(&request),
            true,
        )
        .await
    }

    /// Get intra-account transfer history.
    pub async fn get_intra_exchange_instance_transfers(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> Result<GetIntraExchangeInstanceTransfersResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfers");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get one intra-account transfer by its Kalshi-generated ID.
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

    /// Get credit history, newest first.
    pub async fn get_credit_history(
        &self,
        params: GetCreditHistoryParams,
    ) -> Result<GetCreditHistoryResponse, KalshiError> {
        let path = Self::full_path("/portfolio/credit_history");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get deposit history.
    pub async fn get_deposits(
        &self,
        params: GetDepositsParams,
    ) -> Result<GetDepositsResponse, KalshiError> {
        let path = Self::full_path("/portfolio/deposits");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get withdrawal history.
    pub async fn get_withdrawals(
        &self,
        params: GetWithdrawalsParams,
    ) -> Result<GetWithdrawalsResponse, KalshiError> {
        let path = Self::full_path("/portfolio/withdrawals");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get the target allocation across exchange indexes.
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

    /// Replace the target allocation across exchange indexes.
    pub async fn set_target_balance_allocation(
        &self,
        request: SetTargetBalanceAllocationRequest,
    ) -> Result<super::account::EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Some(&request),
            true,
        )
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
        params.validate()?;
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

    /// Page through intra-account transfers.
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

    /// Page through credit history.
    pub fn credit_history_pager(
        &self,
        params: GetCreditHistoryParams,
    ) -> CursorPager<CreditHistory> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_credit_history(page_params).await?;
                Ok((resp.credits, Some(resp.cursor)))
            })
        })
    }

    /// Page through deposits.
    pub fn deposits_pager(&self, params: GetDepositsParams) -> CursorPager<Deposit> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_deposits(page_params).await?;
                Ok((resp.deposits, resp.cursor))
            })
        })
    }

    /// Page through withdrawals.
    pub fn withdrawals_pager(&self, params: GetWithdrawalsParams) -> CursorPager<Withdrawal> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_withdrawals(page_params).await?;
                Ok((resp.withdrawals, resp.cursor))
            })
        })
    }

    /// Stream intra-account transfers.
    pub fn stream_intra_exchange_instance_transfers(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<IntraExchangeInstanceTransfer, KalshiError>> + Send {
        stream_items(
            self.intra_exchange_instance_transfers_pager(params),
            max_items,
        )
    }

    /// Stream credit history.
    pub fn stream_credit_history(
        &self,
        params: GetCreditHistoryParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<CreditHistory, KalshiError>> + Send {
        stream_items(self.credit_history_pager(params), max_items)
    }

    /// Stream deposits.
    pub fn stream_deposits(
        &self,
        params: GetDepositsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Deposit, KalshiError>> + Send {
        stream_items(self.deposits_pager(params), max_items)
    }

    /// Stream withdrawals.
    pub fn stream_withdrawals(
        &self,
        params: GetWithdrawalsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Withdrawal, KalshiError>> + Send {
        stream_items(self.withdrawals_pager(params), max_items)
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
                Ok((resp.fills, Some(resp.cursor)))
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn fills_require_direction_and_preserve_fractional_precision() {
        let value = json!({
            "fill_id": "f", "exchange_index": 7, "trade_id": "f",
            "order_id": "o", "ticker": "T", "market_ticker": "T",
            "outcome_side": "no", "book_side": "ask",
            "count_fp": "1.25", "yes_price_dollars": "0.123456",
            "no_price_dollars": "0.876544", "is_taker": false,
            "fee_cost": "0.000001", "subaccount_number": null
        });
        let fill: Fill = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(fill.exchange_index, 7);
        assert_eq!(fill.count_fp, "1.25");
        assert_eq!(fill.fee_cost, "0.000001");
        assert_eq!(fill.yes_price_dollars, "0.123456");
        assert_eq!(fill.subaccount_number, None);
        let mut missing = value;
        missing.as_object_mut().unwrap().remove("book_side");
        assert!(serde_json::from_value::<Fill>(missing).is_err());
    }

    #[test]
    fn fill_ticker_limit_and_csv_encoding() {
        let mut params = GetFillsParams {
            ticker: Some((0..100).map(|i| format!("T{i}")).collect()),
            exchange_index: Some(4),
            ..Default::default()
        };
        assert!(params.validate().is_ok());
        let encoded = serde_json::to_value(&params).unwrap();
        assert_eq!(encoded["ticker"].as_str().unwrap().split(',').count(), 100);
        assert_eq!(encoded["exchange_index"], 4);
        params.ticker.as_mut().unwrap().push("T100".into());
        assert!(params.validate().is_err());
    }

    #[test]
    fn settlement_filter_and_nullable_payment_finalization() {
        assert_eq!(
            serde_json::to_value(SettlementStatusFilter::All).unwrap(),
            "all"
        );
        let deposit: Deposit = serde_json::from_value(json!({
            "id": "d", "status": "pending", "type": "ach",
            "amount_cents": 100, "fee_cents": 1, "created_ts": 10,
            "finalized_ts": null
        }))
        .unwrap();
        assert_eq!(deposit.status, PaymentStatus::Pending);
        assert_eq!(deposit.payment_type, PaymentType::Ach);
        assert_eq!(deposit.finalized_ts, None);
        assert!(serde_json::from_value::<PaymentStatus>(json!("unknown")).is_err());
    }
    #[test]
    fn settlement_value_is_nullable_but_result_and_lists_are_not() {
        let settlement: Settlement = serde_json::from_value(json!({
            "ticker": "T", "exchange_index": 1, "event_ticker": "E",
            "market_result": "scalar", "yes_count_fp": "1.25",
            "yes_total_cost_dollars": "0.123456", "no_count_fp": "0.00",
            "no_total_cost_dollars": "0.000000", "revenue": 31,
            "settled_time": "2026-01-01T00:00:00Z", "fee_cost": "0.000001",
            "value": null
        }))
        .unwrap();
        assert_eq!(settlement.market_result, SettlementResult::Scalar);
        assert_eq!(settlement.value, None);
        assert!(
            serde_json::from_value::<GetFillsResponse>(json!({
                "fills": null, "cursor": ""
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<GetFillsResponse>(json!({
                "fills": []
            }))
            .is_err()
        );
    }

    #[test]
    fn transfer_history_uses_dollar_strings_and_known_status() {
        let value = json!({
            "transfer_id": "t", "source": "event_contract",
            "destination": "margined", "source_exchange_shard": 3,
            "destination_exchange_shard": 0, "amount": "12.340001",
            "status": "complete", "created_ts": 42
        });
        let transfer: IntraExchangeInstanceTransfer =
            serde_json::from_value(value.clone()).unwrap();
        assert_eq!(transfer.amount, "12.340001");
        assert_eq!(transfer.client_transfer_id, None);
        assert_eq!(
            transfer.status,
            IntraExchangeInstanceTransferStatus::Complete
        );
        let mut bad = value;
        bad["status"] = json!("failed");
        assert!(serde_json::from_value::<IntraExchangeInstanceTransfer>(bad).is_err());
    }
}
