//! On-request markets: weather point and concert cancellation markets that
//! are created on demand and then traded through an RFQ on the returned
//! `market_ticker` (series starting with `KXRFQ`).
//!
//! Creation limits return `429` with an endpoint-specific code
//! (`*_market_limit_exceeded`); that is not request-rate limiting. A valid
//! request that cannot be offered returns `409` (`*_market_unavailable`).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    PrecipHours,
    PrecipTotal,
    MaxTemp,
    MinTemp,
    #[serde(other)]
    Unknown,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Yes strictly above `floor_value`.
    Greater,
    /// Yes strictly below `cap_value`.
    Less,
    /// Yes from `floor_value` to `cap_value` inclusive.
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
    /// Local date the window starts on, `YYYY-MM-DD` in `timezone`.
    pub date: String,
    /// Omit (or repeat `date`) for a one-day window; the day after `date`
    /// for a window that crosses midnight, which is at most 24 hours long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// First local hour of the window (0-23).
    pub start_hour: i32,
    /// Exclusive end hour (1-24).
    pub end_hour: i32,
    /// IANA zone that `date`, `end_date` and the hours are read in. The
    /// market's title and rules use the location's own zone.
    pub timezone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// False when a market for the same request already existed.
    pub created: bool,
    /// The requested latitude rounded to three decimal places.
    pub latitude: f64,
    /// The requested longitude rounded to three decimal places.
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
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    pub enabled: bool,
    pub domains: Vec<WeatherPointDomain>,
    pub variables: Vec<WeatherPointVariableAvailability>,
    /// 2 where a window may cross midnight, otherwise 1.
    pub max_window_days: i32,
    /// The longest window that crosses midnight, in hours.
    pub max_window_hours: i32,
}

/// What a concert cancellation market pays on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcertCover {
    Postponed,
    Cancelled,
    Either,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateConcertCancellationMarketRequest {
    /// The show, as `jambase:<id>`, from the concert listing or search.
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
    /// False when a market for the same request already existed.
    pub created: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertCancellationAvailabilityParams {
    /// Count shows from this venue-local date (`YYYY-MM-DD`); default today.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Count shows up to this venue-local date, inclusive.
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
    /// Whether concert markets are listed; not a promise they can be created.
    pub enabled: bool,
    pub covers: Vec<ConcertCover>,
    pub max_days_ahead: i32,
    pub metros: Vec<ConcertMetro>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertsParams {
    /// A metro from the availability response, such as `jambase:1`.
    pub metro_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Page to return, 1 through 20; defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
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
    pub lineup: Vec<String>,
    pub venue_name: String,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    /// IANA zone that `scheduled_date` and `start_time` are local to.
    pub timezone: String,
    pub scheduled_date: String,
    /// Venue-local `YYYY-MM-DDTHH:MM:SS` with no UTC offset; omitted while
    /// only the date is announced.
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    /// One entry per offered cover.
    pub markets: Vec<ConcertMarket>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertsResponse {
    pub concerts: Vec<Concert>,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchConcertsResponse {
    pub concerts: Vec<Concert>,
}

impl KalshiRestClient {
    /// Return the weather point market for a location, window and threshold,
    /// creating it if needed. **Requires auth.**
    pub async fn create_weather_point_market(
        &self,
        body: &CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        let path = Self::full_path("/weather_point_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Variables that can be created and the regions each is offered in. Public.
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

    /// Return the concert cancellation market for a show, date and cover,
    /// creating it if needed. **Requires auth.**
    pub async fn create_concert_cancellation_market(
        &self,
        body: &CreateConcertCancellationMarketRequest,
    ) -> Result<CreateConcertCancellationMarketResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Covers that can be created and the metros whose concerts are listed. Public.
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

    /// List a metro's upcoming concerts. **Requires auth.**
    pub async fn get_concerts(
        &self,
        params: GetConcertsParams,
    ) -> Result<GetConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Find concerts by name (2 to 80 characters). **Requires auth.**
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
    fn weather_point_request_omits_unset_thresholds_and_end_date() {
        let mut request = CreateWeatherPointMarketRequest {
            latitude: 40.785,
            longitude: -73.965,
            variable: WeatherPointVariable::PrecipHours,
            strike_type: WeatherPointStrikeType::Greater,
            floor_value: Some(2),
            cap_value: None,
            date: "2026-11-23".into(),
            end_date: None,
            start_hour: 23,
            end_hour: 3,
            timezone: "America/New_York".into(),
        };
        let value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["variable"], "precip_hours");
        assert_eq!(value["strike_type"], "greater");
        assert!(value.get("cap_value").is_none());
        assert!(value.get("end_date").is_none());
        request.end_date = Some("2026-11-24".into());
        assert_eq!(
            serde_json::to_value(&request).unwrap()["end_date"],
            "2026-11-24"
        );
    }

    #[test]
    fn weather_point_responses_require_published_fields() {
        let created: CreateWeatherPointMarketResponse = serde_json::from_value(json!({
            "event_ticker": "E", "market_ticker": "M", "created": true,
            "latitude": 40.785, "longitude": -73.965
        }))
        .unwrap();
        assert!(created.created);

        let availability = json!({
            "enabled": true,
            "domains": [{"name": "conus", "north": 54.9, "south": 20.0, "west": -130.0, "east": -60.0}],
            "variables": [
                {"variable": "max_temp", "domains": ["conus"]},
                {"variable": "future_variable", "domains": []}
            ],
            "max_window_days": 2,
            "max_window_hours": 24
        });
        let parsed: GetWeatherPointAvailabilityResponse =
            serde_json::from_value(availability.clone()).unwrap();
        assert_eq!(parsed.variables[0].variable, WeatherPointVariable::MaxTemp);
        assert_eq!(parsed.variables[1].variable, WeatherPointVariable::Unknown);
        assert_eq!(parsed.max_window_days, 2);

        let mut missing = availability;
        missing.as_object_mut().unwrap().remove("max_window_hours");
        assert!(serde_json::from_value::<GetWeatherPointAvailabilityResponse>(missing).is_err());
    }

    #[test]
    fn concert_models_follow_required_and_optional_fields() {
        let request = CreateConcertCancellationMarketRequest {
            jambase_event_id: "jambase:1".into(),
            scheduled_date: "2026-12-31".into(),
            cover: ConcertCover::Cancelled,
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({"jambase_event_id": "jambase:1", "scheduled_date": "2026-12-31", "cover": "cancelled"})
        );

        let concerts: GetConcertsResponse = serde_json::from_value(json!({
            "concerts": [{
                "jambase_event_id": "jambase:1", "name": "Show", "lineup": ["A", "B"],
                "venue_name": "Hall", "timezone": "America/Chicago",
                "scheduled_date": "2026-12-31",
                "markets": [{"cover": "either", "event_ticker": "E", "market_ticker": "M"}]
            }],
            "page": 1,
            "total_pages": 0
        }))
        .unwrap();
        let concert = &concerts.concerts[0];
        assert!(concert.headliner.is_none() && concert.start_time.is_none());
        assert_eq!(concert.markets[0].cover, ConcertCover::Either);

        let availability: GetConcertCancellationAvailabilityResponse =
            serde_json::from_value(json!({
                "enabled": true, "covers": ["postponed", "cancelled", "either"],
                "max_days_ahead": 90,
                "metros": [{"id": "jambase:1", "name": "NYC", "upcoming_shows": 0}]
            }))
            .unwrap();
        assert_eq!(availability.covers.len(), 3);
        assert!(availability.metros[0].latitude.is_none());

        let params = GetConcertsParams {
            metro_id: "jambase:1".into(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({"metro_id": "jambase:1"})
        );
    }
}
