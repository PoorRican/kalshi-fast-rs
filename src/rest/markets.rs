//! Market endpoints: markets listing, individual market lookups, orderbooks,
//! and candlesticks (live and historical).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::rest::trades::GetHistoricalMarketsParams;
use crate::types::{
    FixedPointCount, FixedPointDollars, MarketStatusQuery, MveFilter,
    deserialize_null_as_empty_vec, serialize_csv_opt,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketStatus {
    Initialized,
    Inactive,
    Active,
    Closed,
    Determined,
    Disputed,
    Amended,
    Finalized,
    #[serde(other)]
    Unknown,
}

impl MarketStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MarketStatus::Initialized => "initialized",
            MarketStatus::Inactive => "inactive",
            MarketStatus::Active => "active",
            MarketStatus::Closed => "closed",
            MarketStatus::Determined => "determined",
            MarketStatus::Disputed => "disputed",
            MarketStatus::Amended => "amended",
            MarketStatus::Finalized => "finalized",
            MarketStatus::Unknown => "unknown",
        }
    }
}

/// Error returned by strict lifecycle/query status conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketStatusConversionError {
    LifecycleToQuery(MarketStatus),
    QueryToLifecycle(MarketStatusQuery),
}

impl fmt::Display for MarketStatusConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketStatusConversionError::LifecycleToQuery(status) => write!(
                f,
                "cannot strictly convert lifecycle market status `{}` to market status query",
                status.as_str()
            ),
            MarketStatusConversionError::QueryToLifecycle(status) => write!(
                f,
                "cannot strictly convert market status query `{}` to lifecycle market status",
                status.as_str()
            ),
        }
    }
}

impl std::error::Error for MarketStatusConversionError {}

/// Convert a lifecycle status to its best matching query status.
///
/// This mapping is lossy for determined, disputed and amended markets.
impl From<MarketStatus> for MarketStatusQuery {
    fn from(status: MarketStatus) -> Self {
        match status {
            MarketStatus::Initialized => MarketStatusQuery::Unopened,
            MarketStatus::Inactive => MarketStatusQuery::Paused,
            MarketStatus::Active => MarketStatusQuery::Open,
            MarketStatus::Closed
            | MarketStatus::Determined
            | MarketStatus::Disputed
            | MarketStatus::Amended => MarketStatusQuery::Closed,
            MarketStatus::Finalized => MarketStatusQuery::Settled,
            MarketStatus::Unknown => MarketStatusQuery::Unknown,
        }
    }
}

/// Convert query status filters to their response lifecycle status.
///
/// Prefer using the source enum directly when possible. This mapping is lossy.
impl From<MarketStatusQuery> for MarketStatus {
    fn from(status: MarketStatusQuery) -> Self {
        match status {
            MarketStatusQuery::Unopened => MarketStatus::Initialized,
            MarketStatusQuery::Open => MarketStatus::Active,
            MarketStatusQuery::Paused => MarketStatus::Inactive,
            MarketStatusQuery::Closed => MarketStatus::Closed,
            MarketStatusQuery::Settled => MarketStatus::Finalized,
            MarketStatusQuery::Unknown => MarketStatus::Unknown,
        }
    }
}

/// Strict lifecycle-to-query conversion helper.
///
/// Prefer direct enum usage whenever possible. This exists for forward
/// compatibility and only succeeds for one-to-one status values.
impl TryFrom<&MarketStatus> for MarketStatusQuery {
    type Error = MarketStatusConversionError;

    fn try_from(status: &MarketStatus) -> Result<Self, Self::Error> {
        match *status {
            MarketStatus::Closed => Ok(MarketStatusQuery::Closed),
            MarketStatus::Unknown => Ok(MarketStatusQuery::Unknown),
            _ => Err(MarketStatusConversionError::LifecycleToQuery(*status)),
        }
    }
}

/// Strict query-to-lifecycle conversion helper.
///
/// Prefer direct enum usage whenever possible. This exists for forward
/// compatibility and only succeeds for one-to-one status values.
impl TryFrom<&MarketStatusQuery> for MarketStatus {
    type Error = MarketStatusConversionError;

    fn try_from(status: &MarketStatusQuery) -> Result<Self, Self::Error> {
        match *status {
            MarketStatusQuery::Closed => Ok(MarketStatus::Closed),
            MarketStatusQuery::Unknown => Ok(MarketStatus::Unknown),
            _ => Err(MarketStatusConversionError::QueryToLifecycle(*status)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MveSelectedLeg {
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub yes_settlement_value_dollars: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Market {
    pub ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub status: Option<MarketStatus>,
    #[serde(default)]
    pub market_type: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    #[serde(default)]
    pub no_sub_title: Option<String>,
    #[serde(default)]
    pub rules_primary: Option<String>,
    #[serde(default)]
    pub rules_secondary: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub can_close_early: Option<bool>,
    #[serde(default)]
    pub settlement_timer_seconds: Option<i64>,
    #[serde(default)]
    pub price_level_structure: Option<String>,
    #[serde(default)]
    pub open_time: Option<String>,
    #[serde(default)]
    pub close_time: Option<String>,
    #[serde(default)]
    pub expected_expiration_time: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub latest_expiration_time: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub updated_time: Option<String>,
    #[serde(default)]
    pub yes_bid_size_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_ask_size_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_bid_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub yes_ask_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub no_bid_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub no_ask_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub last_price_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub volume_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub volume_24h_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub open_interest_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub notional_value_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub previous_yes_bid_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub previous_yes_ask_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub previous_price_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub expiration_value: Option<String>,
    #[serde(default)]
    pub occurrence_datetime: Option<String>,
    #[serde(default)]
    pub settlement_value_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub settlement_ts: Option<String>,
    /// Published settlement bound kind (`default` or `floor`); floor rollout
    /// is not implied by its presence in the schema.
    #[serde(default)]
    pub settlement_bounds_type: Option<String>,
    #[serde(default)]
    pub settlement_floor_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub fee_waiver_expiration_time: Option<String>,
    #[serde(default)]
    pub early_close_condition: Option<String>,
    #[serde(default)]
    pub strike_type: Option<String>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub functional_strike: Option<String>,
    #[serde(default)]
    pub custom_strike: Option<Map<String, Value>>,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    #[serde(default)]
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
    #[serde(default)]
    pub primary_participant_key: Option<String>,
    #[serde(default)]
    pub is_provisional: Option<bool>,
    #[serde(default)]
    pub price_ranges: Option<Vec<PriceRange>>,
    #[serde(default)]
    pub exchange_index: Option<i32>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// GET /markets query params and constraints
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarketsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Filter by one event ticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_created_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_created_ts: Option<i64>,

    /// Updated-time filters can be combined with each other and with
    /// `mve_filter=exclude`; `series_ticker` additionally requires that filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_updated_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_settled_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_settled_ts: Option<i64>,

    /// Only one status filter may be supplied at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MarketStatusQuery>,

    /// Market tickers comma-separated.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_csv_opt"
    )]
    pub tickers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<MveFilter>,
}

impl GetMarketsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit
            && limit > 1000
        {
            return Err(KalshiError::InvalidParams(
                "GET /markets: limit must be 0..=1000".to_string(),
            ));
        }

        let created = self.min_created_ts.is_some() || self.max_created_ts.is_some();
        let close = self.min_close_ts.is_some() || self.max_close_ts.is_some();
        let settled = self.min_settled_ts.is_some() || self.max_settled_ts.is_some();
        let updated = self.min_updated_ts.is_some() || self.max_updated_ts.is_some();

        let groups = [created, close, settled, updated]
            .iter()
            .filter(|x| **x)
            .count();
        if groups > 1 {
            return Err(KalshiError::InvalidParams(
                "GET /markets: timestamp filters are mutually exclusive (created vs close vs settled vs updated)"
                    .to_string(),
            ));
        }

        if updated {
            if self.status.is_some()
                || self.event_ticker.is_some()
                || self.tickers.is_some()
                || created
                || close
                || settled
            {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: updated timestamp filters cannot be combined with these filters"
                        .to_string(),
                ));
            }
            if matches!(self.mve_filter, Some(MveFilter::Only)) {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: with updated timestamps, only mve_filter=exclude is allowed"
                        .to_string(),
                ));
            }
            if self.series_ticker.is_some() && !matches!(self.mve_filter, Some(MveFilter::Exclude))
            {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: series_ticker with updated timestamps requires mve_filter=exclude"
                        .to_string(),
                ));
            }
        }

        if created
            && matches!(
                self.status,
                Some(
                    MarketStatusQuery::Closed
                        | MarketStatusQuery::Settled
                        | MarketStatusQuery::Paused
                )
            )
        {
            return Err(KalshiError::InvalidParams(
                    "GET /markets: created_ts filters are only compatible with status unopened/open or no status".to_string(),
                ));
        }
        if close
            && matches!(
                self.status,
                Some(
                    MarketStatusQuery::Unopened
                        | MarketStatusQuery::Open
                        | MarketStatusQuery::Settled
                        | MarketStatusQuery::Paused
                )
            )
        {
            return Err(KalshiError::InvalidParams(
                    "GET /markets: close_ts filters are only compatible with status closed or no status".to_string(),
                ));
        }
        if settled
            && matches!(
                self.status,
                Some(
                    MarketStatusQuery::Unopened
                        | MarketStatusQuery::Open
                        | MarketStatusQuery::Closed
                        | MarketStatusQuery::Paused
                )
            )
        {
            return Err(KalshiError::InvalidParams(
                    "GET /markets: settled_ts filters are only compatible with status settled or no status".to_string(),
                ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<Market>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketResponse {
    pub market: Market,
}

/// --- Orderbook ---

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderbookFp {
    /// Price levels: (price_dollars, quantity_fp)
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub yes_dollars: Vec<(FixedPointDollars, String)>,
    /// Price levels: (price_dollars, quantity_fp)
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub no_dollars: Vec<(FixedPointDollars, String)>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMarketOrderbookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketOrderbookResponse {
    pub orderbook_fp: OrderbookFp,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarketOrderbooksParams {
    pub tickers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketOrderbookFp {
    pub ticker: String,
    pub orderbook_fp: OrderbookFp,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketOrderbooksResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub orderbooks: Vec<MarketOrderbookFp>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketCandlesticksParams {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketCandlesticksHistoricalParams {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
}

fn validate_candlestick_interval(interval: u32) -> Result<(), KalshiError> {
    if matches!(interval, 1 | 60 | 1440) {
        Ok(())
    } else {
        Err(KalshiError::InvalidParams(
            "candlestick period_interval must be 1, 60, or 1440 minutes".to_string(),
        ))
    }
}

fn validate_batch_candlestick_interval(interval: u32) -> Result<(), KalshiError> {
    if (1..=i32::MAX as u32).contains(&interval) {
        Ok(())
    } else {
        Err(KalshiError::InvalidParams(
            "batch candlestick period_interval must be a positive 32-bit integer".to_string(),
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketCandlestick {
    pub end_period_ts: i64,
    pub yes_bid: BidAskDistribution,
    pub yes_ask: BidAskDistribution,
    pub price: PriceDistribution,
    pub volume_fp: FixedPointCount,
    pub open_interest_fp: FixedPointCount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketCandlesticksResponse {
    pub market_ticker: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub candlesticks: Vec<MarketCandlestick>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchGetMarketCandlesticksParams {
    pub market_tickers: String,
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchGetMarketCandlesticksResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<MarketCandlesticksResponse>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BidAskDistribution {
    #[serde(default)]
    pub open_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub low_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub high_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub close_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceDistribution {
    #[serde(default)]
    pub open_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub low_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub high_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub close_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub mean_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub previous_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub min_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub max_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BidAskDistributionHistorical {
    pub open: FixedPointDollars,
    pub low: FixedPointDollars,
    pub high: FixedPointDollars,
    pub close: FixedPointDollars,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceDistributionHistorical {
    #[serde(default)]
    pub open: Option<FixedPointDollars>,
    #[serde(default)]
    pub low: Option<FixedPointDollars>,
    #[serde(default)]
    pub high: Option<FixedPointDollars>,
    #[serde(default)]
    pub close: Option<FixedPointDollars>,
    #[serde(default)]
    pub mean: Option<FixedPointDollars>,
    #[serde(default)]
    pub previous: Option<FixedPointDollars>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketCandlestickHistorical {
    pub end_period_ts: i64,
    pub yes_bid: BidAskDistributionHistorical,
    pub yes_ask: BidAskDistributionHistorical,
    pub price: PriceDistributionHistorical,
    pub volume: FixedPointCount,
    pub open_interest: FixedPointCount,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketCandlesticksHistoricalResponse {
    pub ticker: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub candlesticks: Vec<MarketCandlestickHistorical>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarketCandlesticksResponse {
    pub ticker: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub candlesticks: Vec<MarketCandlestick>,
}

impl KalshiRestClient {
    /// List markets with optional filters. Supports cursor pagination.
    pub async fn get_markets(
        &self,
        params: GetMarketsParams,
    ) -> Result<GetMarketsResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/markets");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get a single market by ticker.
    pub async fn get_market(&self, market_ticker: &str) -> Result<GetMarketResponse, KalshiError> {
        let path = Self::full_path(&format!("/markets/{market_ticker}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get a single historical market by ticker.
    pub async fn get_historical_market(
        &self,
        market_ticker: &str,
    ) -> Result<GetMarketResponse, KalshiError> {
        let path = Self::full_path(&format!("/historical/markets/{market_ticker}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get the order book for a market, optionally limited to `depth` levels per side.
    pub async fn get_market_orderbook(
        &self,
        market_ticker: &str,
        depth: Option<u32>,
    ) -> Result<GetMarketOrderbookResponse, KalshiError> {
        let path = Self::full_path(&format!("/markets/{market_ticker}/orderbook"));
        let params = GetMarketOrderbookParams { depth };
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get order books for multiple markets in one request.
    pub async fn get_market_orderbooks(
        &self,
        params: GetMarketOrderbooksParams,
    ) -> Result<GetMarketOrderbooksResponse, KalshiError> {
        let path = Self::full_path("/markets/orderbooks");
        let query: Vec<(&str, &str)> = params
            .tickers
            .iter()
            .map(|ticker| ("tickers", ticker.as_str()))
            .collect();
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, false)
            .await
    }

    pub async fn batch_get_market_candlesticks(
        &self,
        params: BatchGetMarketCandlesticksParams,
    ) -> Result<BatchGetMarketCandlesticksResponse, KalshiError> {
        validate_batch_candlestick_interval(params.period_interval)?;
        let path = Self::full_path("/markets/candlesticks");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_historical_market_candlesticks(
        &self,
        ticker: &str,
        params: GetMarketCandlesticksHistoricalParams,
    ) -> Result<GetMarketCandlesticksHistoricalResponse, KalshiError> {
        validate_candlestick_interval(params.period_interval)?;
        let path = Self::full_path(&format!("/historical/markets/{ticker}/candlesticks"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_market_candlesticks_historical(
        &self,
        ticker: &str,
        params: GetMarketCandlesticksHistoricalParams,
    ) -> Result<GetMarketCandlesticksHistoricalResponse, KalshiError> {
        self.get_historical_market_candlesticks(ticker, params)
            .await
    }

    pub async fn get_market_candlesticks(
        &self,
        series_ticker: &str,
        ticker: &str,
        params: GetMarketCandlesticksParams,
    ) -> Result<GetMarketCandlesticksResponse, KalshiError> {
        validate_candlestick_interval(params.period_interval)?;
        let path = Self::full_path(&format!(
            "/series/{series_ticker}/markets/{ticker}/candlesticks"
        ));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// List historical markets.
    pub async fn get_historical_markets(
        &self,
        params: GetHistoricalMarketsParams,
    ) -> Result<GetMarketsResponse, KalshiError> {
        let path = Self::full_path("/historical/markets");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Create a pager for iterating over markets page by page. See [`CursorPager`].
    pub fn markets_pager(&self, params: GetMarketsParams) -> CursorPager<Market> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_markets(page_params).await?;
                Ok((resp.markets, resp.cursor))
            })
        })
    }

    /// Stream markets one by one.
    pub fn stream_markets(
        &self,
        params: GetMarketsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Market, KalshiError>> + Send {
        stream_items(self.markets_pager(params), max_items)
    }

    /// Fetch all pages for markets using cursor pagination.
    pub async fn get_markets_all(
        &self,
        params: GetMarketsParams,
    ) -> Result<Vec<Market>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_markets(page_params).await?;
                Ok((resp.markets, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn updated_time_filters_allow_series_only_with_mve_exclusion() {
        let mut params = GetMarketsParams {
            min_updated_ts: Some(100),
            max_updated_ts: Some(200),
            series_ticker: Some("SERIES".into()),
            ..Default::default()
        };
        assert!(params.validate().is_err());
        params.mve_filter = Some(MveFilter::Exclude);
        assert!(params.validate().is_ok());
        params.status = Some(MarketStatusQuery::Open);
        assert!(params.validate().is_err());
        params.status = None;
        params.min_updated_ts = None;
        assert!(params.validate().is_ok());
        params.mve_filter = Some(MveFilter::Only);
        assert!(params.validate().is_err());
    }

    #[test]
    fn candlestick_interval_constraints_match_each_operation() {
        for interval in [1, 60, 1440] {
            assert!(validate_candlestick_interval(interval).is_ok());
        }
        for interval in [0, 2, 59, 61, 1439, 1441] {
            assert!(validate_candlestick_interval(interval).is_err());
        }
        for interval in [1, 2, 60, 1440, i32::MAX as u32] {
            assert!(validate_batch_candlestick_interval(interval).is_ok());
        }
        for interval in [0, i32::MAX as u32 + 1, u32::MAX] {
            assert!(validate_batch_candlestick_interval(interval).is_err());
        }
    }

    #[test]
    fn market_fixed_point_metadata_and_price_ranges_are_lossless() {
        let market: Market = serde_json::from_value(json!({
            "ticker": "M",
            "settlement_bounds_type": "floor",
            "settlement_floor_dollars": "0.012345",
            "exchange_index": 4,
            "yes_bid_size_fp": "12.3456",
            "yes_ask_size_fp": "0.125",
            "occurrence_datetime": "2026-11-02T00:00:00Z",
            "strike_type": "structured",
            "price_ranges": [{"start": "0.0001", "end": "0.9999", "step": "0.0001"}]
        }))
        .unwrap();
        assert_eq!(market.settlement_floor_dollars.as_deref(), Some("0.012345"));
        assert_eq!(market.yes_bid_size_fp.as_deref(), Some("12.3456"));
        assert_eq!(market.yes_ask_size_fp.as_deref(), Some("0.125"));
        assert_eq!(market.exchange_index, Some(4));
        assert_eq!(market.strike_type.as_deref(), Some("structured"));
        assert_eq!(market.price_ranges.unwrap()[0].step, "0.0001");
        assert!(
            serde_json::from_value::<PriceRange>(json!({
                "min_price": "0.0001", "max_price": "0.9999", "increment": "0.0001"
            }))
            .is_err()
        );
    }

    #[test]
    fn orderbook_null_sides_and_fixed_point_levels() {
        let book: GetMarketOrderbookResponse = serde_json::from_value(json!({
            "orderbook_fp": {"yes_dollars": [["0.012345", "1.2345"]], "no_dollars": null}
        }))
        .unwrap();
        assert_eq!(
            book.orderbook_fp.yes_dollars[0],
            ("0.012345".into(), "1.2345".into())
        );
        assert!(book.orderbook_fp.no_dollars.is_empty());
        let batch: GetMarketOrderbooksResponse = serde_json::from_value(json!({
            "orderbooks": [{
                "ticker": "M",
                "orderbook_fp": {"yes_dollars": null, "no_dollars": [["0.987654", "0.125"]]}
            }]
        }))
        .unwrap();
        assert_eq!(batch.orderbooks[0].ticker, "M");
        assert!(batch.orderbooks[0].orderbook_fp.yes_dollars.is_empty());
        assert_eq!(
            batch.orderbooks[0].orderbook_fp.no_dollars[0],
            ("0.987654".into(), "0.125".into())
        );
    }

    #[test]
    fn historical_candles_use_unsuffixed_strings_and_nullable_trade_prices() {
        let candle: MarketCandlestickHistorical = serde_json::from_value(json!({
            "end_period_ts": 123,
            "yes_bid": {"open": "0.12345", "low": "0.12345", "high": "0.12345", "close": "0.12345"},
            "yes_ask": {"open": "0.23456", "low": "0.23456", "high": "0.23456", "close": "0.23456"},
            "price": {"open": null, "low": null, "high": null, "close": null, "mean": null, "previous": "0.10001"},
            "volume": "1.2345",
            "open_interest": "7.8901"
        })).unwrap();
        assert_eq!(candle.yes_bid.open, "0.12345");
        assert_eq!(candle.price.previous.as_deref(), Some("0.10001"));
        assert_eq!(candle.price.mean, None);
        assert_eq!(candle.volume, "1.2345");
    }
}
