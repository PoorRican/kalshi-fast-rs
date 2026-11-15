//! On-request markets: weather point markets and concert cancellation markets.
//!
//! These markets live in series starting with `KXRFQ`, are created on demand
//! and traded through RFQs: pass the returned `market_ticker` to
//! [`KalshiRestClient::create_rfq`]. They are left out of unscoped market and
//! trade listings and of WebSocket subscriptions that don't name markets. A
//! market created a moment ago may not be readable yet; retry a 404 from
//! `get_market`.
//!
//! Creation refusals are ordinary HTTP errors whose `code` distinguishes them
//! from request-rate limiting: 409 `*_market_unavailable` and 429
//! `*_market_limit_exceeded` (retrying at once does not help).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::deserialize_null_as_empty_vec;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
///
/// Threshold units: `PrecipHours` whole hours; `PrecipTotal` hundredths of an
/// inch; `MaxTemp`/`MinTemp` whole °F.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    PrecipHours,
    PrecipTotal,
    MaxTemp,
    MinTemp,
    /// A variable published after this crate version. Not valid in requests.
    #[serde(other)]
    Unknown,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Yes strictly above `floor_value`.
    Greater,
    /// Yes strictly below `cap_value`.
    Less,
    /// Yes between `floor_value` and `cap_value`.
    Between,
}

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
    /// Local calendar date the window starts on, as YYYY-MM-DD in `timezone`.
    pub date: String,
    /// First local hour of the window (0-23).
    pub start_hour: i32,
    /// Local hour the window ends at, exclusive (1-24, after `start_hour`).
    pub end_hour: i32,
    /// IANA time zone the date and hours are read in.
    pub timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// False when a market for the same request already existed.
    pub created: bool,
    /// Snapped to the center of the grid cell containing the request.
    pub latitude: f64,
    pub longitude: f64,
}

/// A NOAA MRMS domain a location may lie in; bounds are grid cell centers.
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
    /// Names of the domains new markets of this variable may be created in.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    /// Whether weather point markets can be created at all.
    pub enabled: bool,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<WeatherPointDomain>,
    /// A variable left out can't be created.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub variables: Vec<WeatherPointVariableAvailability>,
}

/// What a concert cancellation market pays on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcertCover {
    Postponed,
    Cancelled,
    Either,
    /// A cover published after this crate version. Not valid in requests.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateConcertCancellationMarketRequest {
    /// The show, as `jambase:<id>`, from the concert listing or search.
    pub jambase_event_id: String,
    /// Venue-local date the show is scheduled for, as YYYY-MM-DD.
    pub scheduled_date: String,
    pub cover: ConcertCover,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateConcertCancellationMarketResponse {
    pub cover: ConcertCover,
    pub event_ticker: String,
    pub market_ticker: String,
    /// False when a market for the same request already existed.
    pub created: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertCancellationAvailabilityParams {
    /// Count shows from this venue-local date (YYYY-MM-DD); omitted counts from today.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Count shows up to this venue-local date, inclusive (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcertMetro {
    /// Pass as `metro_id` when listing concerts.
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    /// 0 when no count is available.
    pub upcoming_shows: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertCancellationAvailabilityResponse {
    /// Whether concert cancellation markets can be created at all.
    pub enabled: bool,
    /// A cover left out can't be created.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub covers: Vec<ConcertCover>,
    /// How many days ahead a show may be scheduled for a new market.
    pub max_days_ahead: i32,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub metros: Vec<ConcertMetro>,
}

/// Tickers a show has or would get for one cover. The market may not exist
/// yet: create it before sending an RFQ.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcertMarket {
    pub cover: ConcertCover,
    pub event_ticker: String,
    pub market_ticker: String,
}

/// An upcoming concert. Dates and times are local to the venue.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Concert {
    /// The show's id, as `jambase:<id>`.
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
    /// IANA time zone of the venue.
    pub timezone: String,
    /// Venue-local scheduled date, as YYYY-MM-DD.
    pub scheduled_date: String,
    /// Venue-local start time with no UTC offset; absent while only the date is announced.
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    /// One entry per offered cover.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<ConcertMarket>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertsParams {
    /// One of the metros from the availability response, such as `jambase:1`.
    pub metro_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Page to return, 1 to 20; 50 concerts to a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
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

impl KalshiRestClient {
    /// Return the on-request weather market for a location, window and
    /// threshold, creating it if it doesn't exist yet. Charged against the
    /// write budget.
    pub async fn create_weather_point_market(
        &self,
        body: CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        let path = Self::full_path("/weather_point_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Which on-request weather markets can be created. Public.
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

    /// Return the on-request market on whether a concert is called off,
    /// creating it if it doesn't exist yet. Charged against the write budget.
    pub async fn create_concert_cancellation_market(
        &self,
        body: CreateConcertCancellationMarketRequest,
    ) -> Result<CreateConcertCancellationMarketResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Which on-request concert markets can be created. Public.
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

    /// List a metro's upcoming concerts, soonest first.
    pub async fn get_concerts(
        &self,
        params: GetConcertsParams,
    ) -> Result<GetConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Search upcoming concerts by name across metros (up to 50). `q` is 2 to
    /// 80 characters. A shared search limit returns 429
    /// `concert_search_limit_exceeded`.
    pub async fn search_concerts(&self, q: &str) -> Result<SearchConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts/search");
        self.send(
            Method::GET,
            &path,
            Some(&[("q", q)]),
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
    fn weather_request_omits_unused_threshold_and_response_decodes() {
        let request = CreateWeatherPointMarketRequest {
            latitude: 40.71,
            longitude: -74.0,
            variable: WeatherPointVariable::PrecipHours,
            strike_type: WeatherPointStrikeType::Greater,
            floor_value: Some(2),
            cap_value: None,
            date: "2026-11-20".into(),
            start_hour: 0,
            end_hour: 24,
            timezone: "America/New_York".into(),
        };
        let value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["variable"], "precip_hours");
        assert_eq!(value["strike_type"], "greater");
        assert!(value.get("cap_value").is_none());

        let response: CreateWeatherPointMarketResponse = serde_json::from_value(json!({
            "event_ticker": "KXRFQRAINHOURS-E", "market_ticker": "KXRFQRAINHOURS-E-M",
            "created": true, "latitude": 40.715, "longitude": -74.005
        }))
        .unwrap();
        assert!(response.created);

        let availability: GetWeatherPointAvailabilityResponse = serde_json::from_value(json!({
            "enabled": true,
            "domains": [{"name": "conus", "north": 54.9, "south": 20.0, "west": -130.0, "east": -60.0}],
            "variables": [
                {"variable": "max_temp", "domains": ["conus"]},
                {"variable": "wind_gust", "domains": []}
            ]
        }))
        .unwrap();
        assert_eq!(
            availability.variables[0].variable,
            WeatherPointVariable::MaxTemp
        );
        assert_eq!(
            availability.variables[1].variable,
            WeatherPointVariable::Unknown
        );
    }

    #[test]
    fn concert_shapes_decode_with_only_required_fields() {
        let request = CreateConcertCancellationMarketRequest {
            jambase_event_id: "jambase:1".into(),
            scheduled_date: "2026-12-01".into(),
            cover: ConcertCover::Either,
        };
        assert_eq!(serde_json::to_value(&request).unwrap()["cover"], "either");

        let created: CreateConcertCancellationMarketResponse = serde_json::from_value(json!({
            "cover": "cancelled", "event_ticker": "E", "market_ticker": "M", "created": false
        }))
        .unwrap();
        assert_eq!(created.cover, ConcertCover::Cancelled);

        let availability: GetConcertCancellationAvailabilityResponse =
            serde_json::from_value(json!({
                "enabled": true, "covers": ["postponed", "either"], "max_days_ahead": 90,
                "metros": [{"id": "jambase:1", "name": "New York", "upcoming_shows": 0}]
            }))
            .unwrap();
        assert_eq!(availability.metros[0].latitude, None);

        let concerts: GetConcertsResponse = serde_json::from_value(json!({
            "concerts": [{
                "jambase_event_id": "jambase:9", "name": "Show", "lineup": ["Act"],
                "venue_name": "Hall", "timezone": "America/New_York",
                "scheduled_date": "2026-12-01",
                "markets": [{"cover": "either", "event_ticker": "E", "market_ticker": "M"}]
            }],
            "page": 1, "total_pages": 0
        }))
        .unwrap();
        assert_eq!(concerts.concerts[0].start_time, None);
        assert_eq!(concerts.concerts[0].markets[0].cover, ConcertCover::Either);

        let params = GetConcertsParams {
            metro_id: "jambase:1".into(),
            page: Some(2),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({"metro_id": "jambase:1", "page": 2})
        );
    }
}
