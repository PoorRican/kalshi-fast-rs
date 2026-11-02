//! Event endpoints: regular events, multivariate events, forecast percentile
//! history, and event-level candlesticks.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::markets::{Market, MarketCandlestick};
use crate::rest::pagination::{CursorPager, stream_items};
use crate::rest::series::{EventMetadata, SettlementSource};
use crate::types::{EventStatus, FeeType, deserialize_null_as_empty_vec};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
    pub with_milestones: Option<bool>, // default false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>, // open|closed|settled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    /// Comma-separated event tickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>, // seconds since epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>, // seconds since epoch
}

impl GetEventsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if let Some(limit) = self.limit
            && (limit == 0 || limit > 200)
        {
            return Err(KalshiError::InvalidParams(
                "GET /events: limit must be 1..=200".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Milestone {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, rename = "type")]
    pub milestone_type: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub related_event_tickers: Vec<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub notification_message: Option<String>,
    #[serde(default)]
    pub details: Option<Map<String, Value>>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub primary_event_tickers: Vec<String>,
    #[serde(default)]
    pub last_updated_ts: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub source_ids: Option<Map<String, Value>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventData {
    pub event_ticker: String,
    #[serde(default)]
    pub series_ticker: Option<String>,
    #[serde(default)]
    pub collateral_return_type: Option<String>,
    #[serde(default)]
    pub mutually_exclusive: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub sub_title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub strike_date: Option<String>,
    #[serde(default)]
    pub strike_period: Option<String>,
    #[serde(default)]
    pub last_updated_ts: Option<String>,
    #[serde(default)]
    pub markets: Option<Vec<Market>>,
    /// Null means that settlement sources are unavailable.
    #[serde(default)]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    #[serde(default)]
    pub product_metadata: Option<Map<String, Value>>,
    #[serde(default)]
    pub fee_type_override: Option<String>,
    #[serde(default)]
    pub fee_multiplier_override: Option<f64>,
    #[serde(default)]
    pub exchange_index: Option<i32>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventsResponse {
    pub events: Vec<EventData>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub milestones: Vec<Milestone>,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>, // default false
    /// Omit child markets entirely; cannot be combined with nested markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_markets: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventResponse {
    pub event: EventData,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<Market>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMultivariateEventsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ticker: Option<String>,
    /// Comma-separated event tickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMultivariateEventsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub events: Vec<EventData>,
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Query parameters for event fee changes.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventFeeChangesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventFeeChange {
    pub id: String,
    pub event_ticker: String,
    pub series_ticker: String,
    /// Null clears the override and restores the series fee type.
    pub fee_type_override: Option<FeeType>,
    /// Null clears the override and restores the series fee multiplier.
    pub fee_multiplier_override: Option<f64>,
    pub scheduled_ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventFeeChangesResponse {
    pub event_fee_changes: Vec<EventFeeChange>,
    pub cursor: String,
}

pub type GetEventMetadataResponse = EventMetadata;

#[derive(Debug, Clone, Serialize)]
pub struct GetEventForecastPercentileHistoryParams {
    pub percentiles: Vec<u32>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventForecastPercentilesHistoryResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub forecast_history: Vec<ForecastPercentilesPoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForecastPercentilesPoint {
    pub event_ticker: String,
    pub end_period_ts: i64,
    pub period_interval: i32,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub percentile_points: Vec<PercentilePoint>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PercentilePoint {
    pub percentile: i32,
    pub raw_numerical_forecast: f64,
    pub numerical_forecast: f64,
    pub formatted_forecast: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetEventCandlesticksParams {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventCandlesticksResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub market_tickers: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub market_candlesticks: Vec<Vec<MarketCandlestick>>,
    pub adjusted_end_ts: i64,
}

impl KalshiRestClient {
    /// List events (excludes multivariate events). Supports cursor pagination.
    pub async fn get_events(
        &self,
        params: GetEventsParams,
    ) -> Result<GetEventsResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/events");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get an event, with child markets nested, top-level, or excluded.
    pub async fn get_event(
        &self,
        event_ticker: &str,
        params: GetEventParams,
    ) -> Result<GetEventResponse, KalshiError> {
        let path = Self::full_path(&format!("/events/{event_ticker}"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// List scheduled event fee overrides, including cleared overrides.
    pub async fn get_event_fee_changes(
        &self,
        params: GetEventFeeChangesParams,
    ) -> Result<GetEventFeeChangesResponse, KalshiError> {
        let path = Self::full_path("/events/fee_changes");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_event_metadata(
        &self,
        event_ticker: &str,
    ) -> Result<GetEventMetadataResponse, KalshiError> {
        let path = Self::full_path(&format!("/events/{event_ticker}/metadata"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_multivariate_events(
        &self,
        params: GetMultivariateEventsParams,
    ) -> Result<GetMultivariateEventsResponse, KalshiError> {
        let path = Self::full_path("/events/multivariate");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_event_forecast_percentile_history(
        &self,
        series_ticker: &str,
        ticker: &str,
        params: GetEventForecastPercentileHistoryParams,
    ) -> Result<GetEventForecastPercentilesHistoryResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/series/{series_ticker}/events/{ticker}/forecast_percentile_history"
        ));
        let query = Self::event_forecast_percentile_history_query(&params);
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, true)
            .await
    }

    pub async fn get_event_market_candlesticks(
        &self,
        series_ticker: &str,
        ticker: &str,
        params: GetEventCandlesticksParams,
    ) -> Result<GetEventCandlesticksResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/series/{series_ticker}/events/{ticker}/candlesticks"
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

    /// Create a pager for iterating over events page by page.
    pub fn events_pager(&self, params: GetEventsParams) -> CursorPager<EventData> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_events(page_params).await?;
                Ok((resp.events, resp.cursor))
            })
        })
    }

    /// Create a pager for iterating over multivariate events page by page.
    pub fn multivariate_events_pager(
        &self,
        params: GetMultivariateEventsParams,
    ) -> CursorPager<EventData> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_multivariate_events(page_params).await?;
                Ok((resp.events, resp.cursor))
            })
        })
    }

    /// Stream events one by one.
    pub fn stream_events(
        &self,
        params: GetEventsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<EventData, KalshiError>> + Send {
        stream_items(self.events_pager(params), max_items)
    }

    /// Stream multivariate events one by one.
    pub fn stream_multivariate_events(
        &self,
        params: GetMultivariateEventsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<EventData, KalshiError>> + Send {
        stream_items(self.multivariate_events_pager(params), max_items)
    }

    /// Fetch all pages for events using cursor pagination.
    pub async fn get_events_all(
        &self,
        params: GetEventsParams,
    ) -> Result<Vec<EventData>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_events(page_params).await?;
                Ok((resp.events, resp.cursor))
            }
        })
        .await
    }

    /// Fetch all pages for multivariate events using cursor pagination.
    pub async fn get_multivariate_events_all(
        &self,
        params: GetMultivariateEventsParams,
    ) -> Result<Vec<EventData>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_multivariate_events(page_params).await?;
                Ok((resp.events, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_fee_override_can_be_cleared() {
        let response: GetEventFeeChangesResponse = serde_json::from_str(
            r#"{
                "event_fee_changes": [
                    {
                        "id": "change-1",
                        "event_ticker": "EVENT",
                        "series_ticker": "SERIES",
                        "fee_type_override": "quadratic",
                        "fee_multiplier_override": 0.25,
                        "scheduled_ts": "2026-11-02T00:00:00Z"
                    },
                    {
                        "id": "change-2",
                        "event_ticker": "EVENT",
                        "series_ticker": "SERIES",
                        "fee_type_override": null,
                        "fee_multiplier_override": null,
                        "scheduled_ts": "2026-11-03T00:00:00Z"
                    }
                ],
                "cursor": ""
            }"#,
        )
        .unwrap();
        assert!(matches!(
            response.event_fee_changes[0].fee_type_override,
            Some(FeeType::Quadratic)
        ));
        assert_eq!(
            response.event_fee_changes[0].fee_multiplier_override,
            Some(0.25)
        );
        assert!(response.event_fee_changes[1].fee_type_override.is_none());
        assert!(
            response.event_fee_changes[1]
                .fee_multiplier_override
                .is_none()
        );
        assert!(response.cursor.is_empty());
    }

    #[test]
    fn event_nullable_metadata_and_shard_are_preserved() {
        let event: EventData = serde_json::from_str(
            r#"{
                "event_ticker": "EVENT",
                "product_metadata": {"market_details": "arbitrary"},
                "settlement_sources": null,
                "fee_type_override": null,
                "fee_multiplier_override": 0.5,
                "exchange_index": 3
            }"#,
        )
        .unwrap();
        assert!(event.settlement_sources.is_none());
        assert!(event.fee_type_override.is_none());
        assert_eq!(event.fee_multiplier_override, Some(0.5));
        assert_eq!(event.exchange_index, Some(3));
        assert_eq!(
            event.product_metadata.unwrap()["market_details"],
            "arbitrary"
        );
    }
}
