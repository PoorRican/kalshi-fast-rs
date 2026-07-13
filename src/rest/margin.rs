use crate::error::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::BookSide;
use reqwest::Method;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Query parameter helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginTradesParams {
    pub ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginMarketCandlesticksParams {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginMarketOrderbookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_tick_size: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginFundingHistoryParams {
    pub start_date: String,
    pub end_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginHistoricalFundingRatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginOrderRequest {
    pub ticker: String,
    pub client_order_id: String,
    pub side: BookSide,
    pub count: String,
    pub price: String,
    pub time_in_force: MarginTimeInForce,
    pub self_trade_prevention_type: MarginSelfTradePreventionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecreaseMarginOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AmendMarginOrderRequest {
    pub ticker: String,
    pub side: BookSide,
    pub price: String,
    pub count: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginOrderGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMarginOrderGroupLimitRequest {
    pub contracts_limit: Option<i64>,
    pub contracts_limit_fp: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyMarginSubaccountTransferRequest {
    pub client_transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginTimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginSelfTradePreventionType {
    TakerAtCross,
    Maker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginMarketStatus {
    Inactive,
    Active,
    Closed,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginExchangeStatusResponse {
    pub exchange_active: bool,
    pub trading_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginEnabledResponse {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketResponse {
    pub market: MarginMarket,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginMarketsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub markets: Vec<MarginMarket>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarket {
    pub ticker: String,
    pub status: MarginMarketStatus,
    pub title: String,
    pub contract_size: String,
    pub tick_size: String,
    pub fractional_trading_enabled: bool,
    #[serde(default)]
    pub leverage_estimate: Option<f64>,
    #[serde(default)]
    pub leverage_estimates: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub volume_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub open_interest: Option<String>,
    #[serde(default)]
    pub open_interest_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub volume_24h: Option<String>,
    #[serde(default)]
    pub volume_24h_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub bid: Option<String>,
    #[serde(default)]
    pub ask: Option<String>,
    #[serde(default)]
    pub settlement_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub liquidation_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub reference_price: Option<TickerPrice>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerPrice {
    pub price: String,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrderbookResponse {
    pub orderbook: MarginOrderbookCount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrderbookCount {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub bids: Vec<Vec<String>>,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginMarketCandlesticksResponse {
    pub ticker: String,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub candlesticks: Vec<MarginMarketCandlestick>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketCandlestick {
    pub end_period_ts: i64,
    pub bid: MarginBidAskDistribution,
    pub ask: MarginBidAskDistribution,
    pub price: MarginPriceDistribution,
    pub volume: String,
    pub volume_notional_value_dollars: String,
    pub open_interest: String,
    pub open_interest_notional_value_dollars: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginBidAskDistribution {
    pub open: String,
    pub low: String,
    pub high: String,
    pub close: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginPriceDistribution {
    #[serde(default)]
    pub open: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub close: Option<String>,
    #[serde(default)]
    pub mean: Option<String>,
    #[serde(default)]
    pub previous: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub fill_count: String,
    pub remaining_count: String,
    #[serde(default)]
    pub average_fill_price: Option<String>,
    #[serde(default)]
    pub average_fee_paid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrderResponse {
    pub order: MarginOrder,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrdersResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub orders: Vec<MarginOrder>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrder {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub side: BookSide,
    pub price: String,
    pub fill_count: String,
    pub remaining_count: String,
    pub last_update_reason: String,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[serde(default)]
    pub self_trade_prevention_type: Option<MarginSelfTradePreventionType>,
    #[serde(default)]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub order_source: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<MarginTimeInForce>,
    #[serde(default)]
    pub post_only: Option<bool>,
    #[serde(default)]
    pub reduce_only: Option<bool>,
    #[serde(default)]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub reduced_by: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecreaseMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub remaining_count: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmendMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub remaining_count: Option<String>,
    #[serde(default)]
    pub fill_count: Option<String>,
    #[serde(default)]
    pub average_fill_price: Option<String>,
    #[serde(default)]
    pub average_fee_paid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFillsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub fills: Vec<MarginFill>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFill {
    pub fill_id: String,
    pub order_id: String,
    pub is_taker: bool,
    pub side: BookSide,
    pub count: String,
    pub created_time: String,
    pub ticker: String,
    pub price: String,
    pub entry_price: String,
    pub fees: String,
    pub realized_pnl: String,
    #[serde(default)]
    pub order_source: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginPositionsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub positions: Vec<MarginPosition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginPosition {
    pub subaccount: u32,
    pub market_ticker: String,
    pub position: String,
    pub entry_price: String,
    pub unrealized_pnl: String,
    pub margin_used: String,
    pub fees: String,
    #[serde(default)]
    pub roe: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginTradesResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub trades: Vec<MarginTrade>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginTrade {
    pub trade_id: String,
    pub ticker: String,
    pub count: String,
    pub price: String,
    pub created_time: String,
    pub taker_side: BookSide,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionalRiskLimitResponse {
    pub default_notional_value_risk_limit: String,
    #[serde(default)]
    pub notional_value_risk_limits_by_market_ticker:
        Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginBalanceResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub subaccount_balances: Vec<MarginSubaccountBalance>,
    pub settled_funds: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginSubaccountBalance {
    pub subaccount: u32,
    pub position_value: String,
    pub account_equity: String,
    pub maintenance_margin: String,
    pub initial_margin: String,
    #[serde(default)]
    pub resting_orders_margin: Option<String>,
    #[serde(default)]
    pub available_balance: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginRiskResponse {
    pub total_position_notional: String,
    pub total_maintenance_margin: String,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub positions: Vec<MarginRiskPosition>,
    #[serde(default)]
    pub account_leverage: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginRiskPosition {
    pub subaccount: u32,
    pub market_ticker: String,
    pub position: String,
    pub mark_price: String,
    pub position_notional: String,
    #[serde(default)]
    pub maintenance_margin_required: Option<String>,
    #[serde(default)]
    pub position_leverage: Option<f64>,
    #[serde(default)]
    pub estimated_liquidation_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginRiskParametersResponse {
    pub liquidation_margin_ratio_threshold: f64,
    pub queue_entry_margin_ratio_threshold: f64,
    #[serde(default)]
    pub initial_margin_multiplier: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFundingHistoryResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub funding_history: Vec<MarginFundingHistoryEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFundingHistoryEntry {
    pub market_ticker: String,
    pub funding_time: String,
    pub funding_rate: f64,
    pub mark_price: String,
    pub funding_amount: String,
    pub quantity: String,
    #[serde(default)]
    pub subaccount_number: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginHistoricalFundingRatesResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub funding_rates: Vec<MarginFundingRate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFundingRate {
    pub market_ticker: String,
    pub funding_time: String,
    pub funding_rate: f64,
    pub mark_price: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFundingRateEstimateResponse {
    pub next_funding_time: String,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub computed_time: Option<String>,
    #[serde(default)]
    pub funding_rate: Option<f64>,
    #[serde(default)]
    pub mark_price: Option<String>,
}

use crate::rest::account::EmptyResponse;
use crate::rest::orders::{
    CreateOrderGroupResponse, GetOrderGroupResponse, GetOrderGroupsResponse,
    UpdateOrderGroupLimitRequest,
};

// ---------------------------------------------------------------------------
// Endpoints
// ---------------------------------------------------------------------------

impl KalshiRestClient {
    /// Check if margin trading is enabled for the authenticated user.
    pub async fn get_margin_enabled(&self) -> Result<MarginEnabledResponse, KalshiError> {
        let path = Self::full_path("/margin/enabled");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_markets(
        &self,
        status: Option<MarginMarketStatus>,
    ) -> Result<GetMarginMarketsResponse, KalshiError> {
        let path = Self::full_path("/margin/markets");
        #[derive(Serialize)]
        struct Q<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<&'a MarginMarketStatus>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q {
                status: status.as_ref(),
            }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market(
        &self,
        ticker: &str,
    ) -> Result<MarginMarketResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market_orderbook(
        &self,
        ticker: &str,
        params: GetMarginMarketOrderbookParams,
    ) -> Result<MarginOrderbookResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}/orderbook"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market_candlesticks(
        &self,
        ticker: &str,
        params: GetMarginMarketCandlesticksParams,
    ) -> Result<GetMarginMarketCandlesticksResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}/candlesticks"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_orders(
        &self,
        params: GetMarginOrdersParams,
    ) -> Result<GetMarginOrdersResponse, KalshiError> {
        let path = Self::full_path("/margin/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn create_margin_order(
        &self,
        body: &CreateMarginOrderRequest,
    ) -> Result<CreateMarginOrderResponse, KalshiError> {
        let path = Self::full_path("/margin/orders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_margin_order(
        &self,
        order_id: &str,
    ) -> Result<GetMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn cancel_margin_order(
        &self,
        order_id: &str,
        subaccount: Option<u32>,
    ) -> Result<CancelMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::DELETE,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn decrease_margin_order(
        &self,
        order_id: &str,
        body: &DecreaseMarginOrderRequest,
        subaccount: Option<u32>,
    ) -> Result<DecreaseMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}/decrease"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::POST,
            &path,
            Some(&Q { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    pub async fn amend_margin_order(
        &self,
        order_id: &str,
        body: &AmendMarginOrderRequest,
    ) -> Result<AmendMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}/amend"));
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_margin_fills(
        &self,
        params: GetMarginFillsParams,
    ) -> Result<GetMarginFillsResponse, KalshiError> {
        let path = Self::full_path("/margin/fills");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_positions(
        &self,
        params: GetMarginPositionsParams,
    ) -> Result<GetMarginPositionsResponse, KalshiError> {
        let path = Self::full_path("/margin/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_trades(
        &self,
        params: GetMarginTradesParams,
    ) -> Result<GetMarginTradesResponse, KalshiError> {
        let path = Self::full_path("/margin/trades");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_balance(
        &self,
        compute_available_balance: Option<bool>,
    ) -> Result<GetMarginBalanceResponse, KalshiError> {
        let path = Self::full_path("/margin/balance");
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            compute_available_balance: Option<bool>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q {
                compute_available_balance,
            }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_risk(&self) -> Result<GetMarginRiskResponse, KalshiError> {
        let path = Self::full_path("/margin/risk");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_risk_parameters(
        &self,
    ) -> Result<GetMarginRiskParametersResponse, KalshiError> {
        let path = Self::full_path("/margin/risk_parameters");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_notional_risk_limit(
        &self,
    ) -> Result<NotionalRiskLimitResponse, KalshiError> {
        let path = Self::full_path("/margin/notional_risk_limit");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_exchange_status(
        &self,
    ) -> Result<GetMarginExchangeStatusResponse, KalshiError> {
        let path = Self::full_path("/margin/exchange/status");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_perps_account_api_limits(
        &self,
    ) -> Result<crate::rest::account::GetAccountApiLimitsResponse, KalshiError> {
        let path = Self::full_path("/account/limits/perps");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_funding_rate_estimate(
        &self,
        ticker: &str,
    ) -> Result<GetMarginFundingRateEstimateResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_rates/estimate");
        #[derive(Serialize)]
        struct Q<'a> {
            ticker: &'a str,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { ticker }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_historical_funding_rates(
        &self,
        params: GetMarginHistoricalFundingRatesParams,
    ) -> Result<GetMarginHistoricalFundingRatesResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_rates/historical");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_funding_history(
        &self,
        params: GetMarginFundingHistoryParams,
    ) -> Result<GetMarginFundingHistoryResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_history");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_order_groups(
        &self,
        subaccount: Option<u32>,
    ) -> Result<GetOrderGroupsResponse, KalshiError> {
        let path = Self::full_path("/margin/order_groups");
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn create_margin_order_group(
        &self,
        body: &CreateMarginOrderGroupRequest,
    ) -> Result<CreateOrderGroupResponse, KalshiError> {
        let path = Self::full_path("/margin/order_groups/create");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<GetOrderGroupResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn delete_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::DELETE,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn reset_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/reset"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn trigger_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/trigger"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn update_margin_order_group_limit(
        &self,
        order_group_id: &str,
        body: &UpdateOrderGroupLimitRequest,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/limit"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    pub async fn create_margin_subaccount(
        &self,
    ) -> Result<crate::rest::account::CreateSubaccountResponse, KalshiError> {
        let path = Self::full_path("/portfolio/margin/subaccounts");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn apply_margin_subaccount_transfer(
        &self,
        body: &ApplyMarginSubaccountTransferRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/margin/subaccounts/transfer");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }
}
