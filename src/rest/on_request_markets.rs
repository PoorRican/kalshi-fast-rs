//! On-request market endpoints: weather point markets and concert
//! cancellation markets (series starting with `KXRFQ`).
//!
//! The create endpoints return (creating if needed) a market that is then
//! traded by sending an RFQ for the returned `market_ticker` with
//! [`KalshiRestClient::create_rfq`]. A valid request that cannot be offered
//! returns HTTP 409, and reaching a creation limit returns HTTP 429 with a
//! product-specific error code; that 429 is not request-rate limiting.
//!
//! Kalshi documents concert cancellation markets as not live yet.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::deserialize_null_as_empty_vec;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    /// Whole hours with at least 0.01 inch of precipitation.
    PrecipHours,
    /// Total precipitation in hundredths of an inch.
    PrecipTotal,
    /// Highest temperature in whole °F.
    MaxTemp,
    /// Lowest temperature in whole °F.
    MinTemp,
    #[serde(other)]
    Unknown,
}

/// How a weather point market compares the measured value to its thresholds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Yes strictly above `floor_value`.
    Greater,
    /// Yes strictly below `cap_value`.
    Less,
    /// Yes from `floor_value` to `cap_value` inclusive.
    Between,
}

/// Body of `POST /weather_point_markets`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWeatherPointMarketRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub variable: WeatherPointVariable,
    pub strike_type: WeatherPointStrikeType,
    /// Required for `greater` and `between`; must be omitted for `less`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_value: Option<i32>,
    /// Required for `less` and `between`; must be omitted for `greater`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_value: Option<i32>,
    /// Local start date as `YYYY-MM-DD` in `timezone`.
    pub date: String,
    /// First local hour of the window (0-23).
    pub start_hour: i32,
    /// Exclusive local end hour (1-24, after `start_hour`).
    pub end_hour: i32,
    /// IANA time zone; fractional-hour offsets are refused.
    pub timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// `false` when a market for the same request already existed.
    pub created: bool,
    /// Center of the grid cell containing the request.
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherPointDomain {
    pub name: String,
    pub north: f64,
    pub south: f64,
    pub west: f64,
    pub east: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherPointVariableAvailability {
    pub variable: WeatherPointVariable,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    pub enabled: bool,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<WeatherPointDomain>,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub variables: Vec<WeatherPointVariableAvailability>,
}

/// What a concert cancellation market pays on.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ConcertCover {
    Postponed,
    Cancelled,
    Either,
    #[serde(other)]
    Unknown,
}

/// Body of `POST /concert_cancellation_markets`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateConcertCancellationMarketRequest {
    /// The show, as `jambase:<id>`.
    pub jambase_event_id: String,
    /// Venue-local scheduled date, `YYYY-MM-DD`; must match the listing.
    pub scheduled_date: String,
    pub cover: ConcertCover,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateConcertCancellationMarketResponse {
    pub cover: ConcertCover,
    pub event_ticker: String,
    pub market_ticker: String,
    /// `false` when a market for the same request already existed.
    pub created: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertCancellationAvailabilityParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcertMetro {
    /// Pass as `metro_id` to [`KalshiRestClient::get_concerts`].
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    /// `0` when no count is available.
    pub upcoming_shows: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertCancellationAvailabilityResponse {
    pub enabled: bool,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub covers: Vec<ConcertCover>,
    pub max_days_ahead: i32,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub metros: Vec<ConcertMetro>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetConcertsParams {
    pub metro_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// 1-20; defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl GetConcertsParams {
    pub fn new(metro_id: impl Into<String>) -> Self {
        Self {
            metro_id: metro_id.into(),
            date_from: None,
            date_to: None,
            page: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcertMarket {
    pub cover: ConcertCover,
    pub event_ticker: String,
    /// The market may not exist yet.
    pub market_ticker: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Concert {
    pub jambase_event_id: String,
    pub name: String,
    #[serde(default)]
    pub headliner: Option<String>,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub lineup: Vec<String>,
    pub venue_name: String,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    pub timezone: String,
    pub scheduled_date: String,
    /// Venue-local `YYYY-MM-DDTHH:MM:SS` with no UTC offset.
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<ConcertMarket>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertsResponse {
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchConcertsResponse {
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
}

#[derive(Serialize)]
struct SearchConcertsQuery<'a> {
    q: &'a str,
}

impl KalshiRestClient {
    /// `POST /weather_point_markets` (authenticated).
    pub async fn create_weather_point_market(
        &self,
        body: CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        let path = Self::full_path("/weather_point_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// `GET /weather_point_markets/availability` (public).
    pub async fn get_weather_point_availability(
        &self,
    ) -> Result<GetWeatherPointAvailabilityResponse, KalshiError> {
        let path = Self::full_path("/weather_point_markets/availability");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// `POST /concert_cancellation_markets` (authenticated).
    pub async fn create_concert_cancellation_market(
        &self,
        body: CreateConcertCancellationMarketRequest,
    ) -> Result<CreateConcertCancellationMarketResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// `GET /concert_cancellation_markets/availability` (public).
    pub async fn get_concert_cancellation_availability(
        &self,
        params: GetConcertCancellationAvailabilityParams,
    ) -> Result<GetConcertCancellationAvailabilityResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/availability");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// `GET /concert_cancellation_markets/concerts` (authenticated).
    pub async fn get_concerts(
        &self,
        params: GetConcertsParams,
    ) -> Result<GetConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// `GET /concert_cancellation_markets/concerts/search` (authenticated).
    /// `q` must be 2-80 characters.
    pub async fn search_concerts(&self, q: &str) -> Result<SearchConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts/search");
        self.send(
            Method::GET,
            &path,
            Some(&SearchConcertsQuery { q }),
            Option::<&()>::None,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn weather_request_omits_unused_threshold() {
        let body = CreateWeatherPointMarketRequest {
            latitude: 40.785,
            longitude: -73.965,
            variable: WeatherPointVariable::PrecipHours,
            strike_type: WeatherPointStrikeType::Greater,
            floor_value: Some(2),
            cap_value: None,
            date: "2026-11-11".into(),
            start_hour: 0,
            end_hour: 24,
            timezone: "America/New_York".into(),
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["variable"], "precip_hours");
        assert_eq!(v["strike_type"], "greater");
        assert_eq!(v["floor_value"], 2);
        assert!(v.get("cap_value").is_none());
    }

    #[test]
    fn concerts_response_decodes_required_and_optional_fields() {
        let resp: GetConcertsResponse = serde_json::from_value(json!({
            "concerts": [{
                "jambase_event_id": "jambase:16543035",
                "name": "Show",
                "lineup": ["A", "B"],
                "venue_name": "Hall",
                "timezone": "America/New_York",
                "scheduled_date": "2026-11-20",
                "markets": [{
                    "cover": "postponed",
                    "event_ticker": "KXRFQCONCERTP-JB16543035-26NOV20",
                    "market_ticker": "KXRFQCONCERTP-JB16543035-26NOV20-PPD"
                }]
            }],
            "page": 1,
            "total_pages": 0
        }))
        .unwrap();
        let c = &resp.concerts[0];
        assert!(c.headliner.is_none() && c.start_time.is_none());
        assert_eq!(c.markets[0].cover, ConcertCover::Postponed);
        assert_eq!(resp.total_pages, 0);
    }

    #[test]
    fn availability_responses_decode() {
        let w: GetWeatherPointAvailabilityResponse = serde_json::from_value(json!({
            "enabled": true,
            "domains": [{"name": "conus", "north": 54.995, "south": 20.005, "west": -129.995, "east": -60.005}],
            "variables": [{"variable": "max_temp", "domains": ["conus"]}, {"variable": "new_var", "domains": []}]
        }))
        .unwrap();
        assert_eq!(w.variables[0].variable, WeatherPointVariable::MaxTemp);
        assert_eq!(w.variables[1].variable, WeatherPointVariable::Unknown);

        let c: GetConcertCancellationAvailabilityResponse = serde_json::from_value(json!({
            "enabled": false,
            "covers": ["either", "cancelled"],
            "max_days_ahead": 30,
            "metros": [{"id": "jambase:1", "name": "New York", "upcoming_shows": 0}]
        }))
        .unwrap();
        assert_eq!(
            c.covers,
            vec![ConcertCover::Either, ConcertCover::Cancelled]
        );
        assert!(c.metros[0].latitude.is_none());
    }
}
