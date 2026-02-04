use crate::error::KalshiError;
use crate::types::{EventStatus, MarketStatus, MveFilter, OrderStatus, PositionCountFilter};
use crate::types::{BuySell, OrderType, SelfTradePreventionType, TimeInForce, YesNo};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// Serialize Option<Vec<T>> as a single comma-separated query param
fn serialize_csv_opt<T, S>(value: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: fmt::Display,
    S: Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(items) => {
            let s = items.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
            serializer.serialize_str(&s)
        }
    }
}

/// --- Series ---

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSeriesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Docs: "tags" is a string (not explicitly CSV-typed), so keep as raw string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_product_metadata: Option<bool>,
    /// If true, includes total volume traded across all events in each series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSeriesListResponse {
    pub series: Vec<serde_json::Value>,
}

/// --- Events ---

/// GET /events query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 200, max 200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_milestones: Option<bool>,     // default false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,       // open|closed|settled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,         // seconds since epoch
}

impl GetEventsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 200 {
                return Err(KalshiError::InvalidParams(
                    "GET /events: limit must be 1..=200".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Response shape in adapter: keep payload flexible.
#[derive(Debug, Clone, Deserialize)]
pub struct GetEventsResponse {
    pub events: Vec<serde_json::Value>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>, // default false
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetEventResponse {
    pub event: serde_json::Value,
}

/// --- Markets ---

/// GET /markets query params and constraints
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarketsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Event tickers comma-separated (max 10)
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_csv_opt")]
    pub event_ticker: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_created_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_created_ts: Option<i64>,

    /// Note in docs: min_updated_ts is incompatible with any other filters besides mve_filter=exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_settled_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_settled_ts: Option<i64>,

    /// Only one status filter may be supplied at a time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MarketStatus>,

    /// Market tickers comma-separated
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_csv_opt")]
    pub tickers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<MveFilter>,
}

impl GetMarketsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: limit must be 1..=1000".to_string(),
                ));
            }
        }
        if let Some(evts) = &self.event_ticker {
            if evts.len() > 10 {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: event_ticker supports up to 10 tickers".to_string(),
                ));
            }
        }

        // Timestamp filter compatibility rules
        let created = self.min_created_ts.is_some() || self.max_created_ts.is_some();
        let close = self.min_close_ts.is_some() || self.max_close_ts.is_some();
        let settled = self.min_settled_ts.is_some() || self.max_settled_ts.is_some();
        let updated = self.min_updated_ts.is_some();

        let groups = [created, close, settled, updated].iter().filter(|x| **x).count();
        if groups > 1 {
            return Err(KalshiError::InvalidParams(
                "GET /markets: timestamp filters are mutually exclusive (created vs close vs settled vs updated)".to_string(),
            ));
        }

        if updated {
            // "Incompatible with any other filters besides mve_filter=exclude"
            if self.status.is_some()
                || self.series_ticker.is_some()
                || self.event_ticker.is_some()
                || self.tickers.is_some()
                || created
                || close
                || settled
            {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: min_updated_ts cannot be combined with other filters (except mve_filter=exclude)".to_string(),
                ));
            }
            if matches!(self.mve_filter, Some(MveFilter::Only)) {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: with min_updated_ts, only mve_filter=exclude is allowed".to_string(),
                ));
            }
        }

        if created {
            if matches!(self.status, Some(MarketStatus::Closed | MarketStatus::Settled | MarketStatus::Paused)) {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: created_ts filters are only compatible with status unopened/open or no status".to_string(),
                ));
            }
        }
        if close {
            if matches!(self.status, Some(MarketStatus::Unopened | MarketStatus::Open | MarketStatus::Settled | MarketStatus::Paused)) {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: close_ts filters are only compatible with status closed or no status".to_string(),
                ));
            }
        }
        if settled {
            if matches!(self.status, Some(MarketStatus::Unopened | MarketStatus::Open | MarketStatus::Closed | MarketStatus::Paused)) {
                return Err(KalshiError::InvalidParams(
                    "GET /markets: settled_ts filters are only compatible with status settled or no status".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarketsResponse {
    pub markets: Vec<serde_json::Value>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarketResponse {
    pub market: serde_json::Value,
}

/// --- Portfolio / Orders ---

#[derive(Debug, Clone, Deserialize)]
pub struct GetBalanceResponse {
    pub balance: i64,
    pub portfolio_value: i64,
    pub updated_ts: i64,
}

/// GET /portfolio/positions query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 1000

    /// CSV of non-zero filters (position,total_traded)
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_csv_opt")]
    pub count_filter: Option<Vec<PositionCountFilter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// CSV max 10
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_csv_opt")]
    pub event_ticker: Option<Vec<String>>,

    /// 0..=32
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

impl GetPositionsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err(KalshiError::InvalidParams(
                    "GET /portfolio/positions: limit must be 1..=1000".to_string(),
                ));
            }
        }
        if let Some(evts) = &self.event_ticker {
            if evts.len() > 10 {
                return Err(KalshiError::InvalidParams(
                    "GET /portfolio/positions: event_ticker supports up to 10 tickers".to_string(),
                ));
            }
        }
        if let Some(sub) = self.subaccount {
            if sub > 32 {
                return Err(KalshiError::InvalidParams(
                    "subaccount must be 0..=32".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPositionsResponse {
    pub market_positions: Vec<serde_json::Value>,
    pub event_positions: Vec<serde_json::Value>,
    #[serde(default)]
    pub cursor: Option<String>,
}

/// GET /portfolio/orders query params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// CSV max 10
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_csv_opt")]
    pub event_ticker: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // default 100, max 200

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

impl GetOrdersParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 200 {
                return Err(KalshiError::InvalidParams(
                    "GET /portfolio/orders: limit must be 1..=200".to_string(),
                ));
            }
        }
        if let Some(evts) = &self.event_ticker {
            if evts.len() > 10 {
                return Err(KalshiError::InvalidParams(
                    "GET /portfolio/orders: event_ticker supports up to 10 tickers".to_string(),
                ));
            }
        }
        if let Some(sub) = self.subaccount {
            if sub > 32 {
                return Err(KalshiError::InvalidParams(
                    "subaccount must be 0..=32".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOrdersResponse {
    pub orders: Vec<serde_json::Value>,
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Create Order body
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOrderRequest {
    /// required
    pub ticker: String,
    /// required: yes|no
    pub side: YesNo,
    /// required: buy|sell
    pub action: BuySell,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Provide count or count_fp; if both provided they must match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OrderType>, // docs list values limit|market

    /// cents 1..=99
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_price: Option<u32>,

    /// fixed-point dollars strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ts: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Maximum cost in cents; when specified, order auto has FoK behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_max_cost: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Deprecated: use reduce_only instead; only accepts 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_position_floor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,

    /// If true, cancel if exchange pauses while order open
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,

    /// default 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderResponse {
    pub order: serde_json::Value,
}

/// DELETE /portfolio/orders/{order_id} supports optional query parameter subaccount
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    pub order: serde_json::Value,
    pub reduced_by: i64,
    pub reduced_by_fp: String,
}
