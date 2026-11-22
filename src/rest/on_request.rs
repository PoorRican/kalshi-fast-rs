//! On-request markets: weather point markets and concert cancellation markets.
//!
//! These endpoints return the market for a request, creating it when it does
//! not exist yet (`created` says which). The returned `market_ticker` is then
//! traded by sending an RFQ through `POST /communications/rfqs`. Their series
//! start with `KXRFQ`, and such markets are left out of unscoped market, trade
//! and WebSocket listings; name them explicitly to see them.
//!
//! A `409` (`*_market_unavailable`) means a valid request cannot be offered,
//! and a `429` (`*_market_limit_exceeded`) is a creation limit, not
//! request-rate limiting: retrying does not help.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::deserialize_null_as_empty_vec;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    /// Whole hours with at least 0.01 inch of precipitation.
    PrecipHours,
    PrecipTotal,
    MaxTemp,
    MinTemp,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Yes strictly above `floor_value`.
    Greater,
    /// Yes strictly below `cap_value`.
    Less,
    /// Yes from `floor_value` to `cap_value`, inclusive.
    Between,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
    /// Local date the window starts on, as YYYY-MM-DD in `timezone`.
    pub date: String,
    /// Set to the day after `date` for a window that crosses midnight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// First local hour of the window (0-23).
    pub start_hour: u8,
    /// Exclusive local end hour (1-24); read on `end_date` when that is set.
    pub end_hour: u8,
    /// IANA zone `date`, `end_date` and the hours are read in. The market is
    /// described in its location's own zone regardless.
    pub timezone: String,
}

impl CreateWeatherPointMarketRequest {
    /// Check the locally decidable constraints of the published schema.
    pub fn validate(&self) -> Result<(), KalshiError> {
        let invalid = |msg: &str| {
            Err(KalshiError::InvalidParams(format!(
                "POST /weather_point_markets: {msg}"
            )))
        };
        if self.start_hour > 23 || !(1..=24).contains(&self.end_hour) {
            return invalid("start_hour must be 0-23 and end_hour 1-24");
        }
        let (floor, cap) = (self.floor_value.is_some(), self.cap_value.is_some());
        let thresholds_match = match self.strike_type {
            WeatherPointStrikeType::Greater => floor && !cap,
            WeatherPointStrikeType::Less => cap && !floor,
            WeatherPointStrikeType::Between => floor && cap,
        };
        if !thresholds_match {
            return invalid(
                "greater takes only floor_value, less only cap_value, between takes both",
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// False when a market for the same request already existed.
    pub created: bool,
    /// The requested coordinates rounded to three decimal places.
    pub latitude: f64,
    pub longitude: f64,
}

/// A NOAA MRMS domain, bounded by its outermost grid cell centers.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WeatherPointDomain {
    pub name: String,
    pub north: f64,
    pub south: f64,
    pub west: f64,
    pub east: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WeatherPointVariableAvailability {
    pub variable: WeatherPointVariable,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    pub enabled: bool,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<WeatherPointDomain>,
    /// A variable left out cannot be created.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub variables: Vec<WeatherPointVariableAvailability>,
    /// 2 where a window may cross midnight (`end_date`), otherwise 1.
    pub max_window_days: i32,
    /// The longest window that crosses midnight, in hours.
    pub max_window_hours: i32,
}

/// What a concert cancellation market pays on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcertCover {
    Postponed,
    Cancelled,
    Either,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateConcertCancellationMarketRequest {
    /// The show, as `jambase:<id>`, from the concert listing or search.
    pub jambase_event_id: String,
    /// Venue-local scheduled date, YYYY-MM-DD; must match the current listing.
    pub scheduled_date: String,
    pub cover: ConcertCover,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
    /// Count shows up to this venue-local date, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetConcertCancellationAvailabilityResponse {
    /// Whether the markets are listed; it does not promise they can be created.
    pub enabled: bool,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub covers: Vec<ConcertCover>,
    pub max_days_ahead: i32,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub metros: Vec<ConcertMetro>,
}

/// The tickers a show has or would get for one cover; the market may not exist yet.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConcertMarket {
    pub cover: ConcertCover,
    pub event_ticker: String,
    pub market_ticker: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Concert {
    pub jambase_event_id: String,
    pub name: String,
    #[serde(default)]
    pub headliner: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub lineup: Vec<String>,
    pub venue_name: String,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    /// IANA zone `scheduled_date` and `start_time` are local to.
    pub timezone: String,
    pub scheduled_date: String,
    /// Venue-local, no UTC offset; omitted while only the date is announced.
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<ConcertMarket>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertsParams {
    /// A metro from the availability response, such as `jambase:1`.
    pub metro_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// 1-20; 50 concerts to a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetConcertsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchConcertsParams {
    /// A name or part of one, 2 to 80 characters.
    pub q: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SearchConcertsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
}

impl KalshiRestClient {
    /// Return the weather point market for a request, creating it if needed.
    /// **Requires auth.** This can create an exchange market.
    pub async fn create_weather_point_market(
        &self,
        body: CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        body.validate()?;
        let path = Self::full_path("/weather_point_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Which weather point markets can be created. Public.
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

    /// Return the cancellation market for a show, date and cover, creating it
    /// if needed. **Requires auth.** Kalshi marks these markets as not live yet.
    pub async fn create_concert_cancellation_market(
        &self,
        body: CreateConcertCancellationMarketRequest,
    ) -> Result<CreateConcertCancellationMarketResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Covers on offer and the metros whose concerts are listed. Public.
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

    /// List a metro's upcoming concerts, soonest first. **Requires auth.**
    pub async fn get_concerts(
        &self,
        params: GetConcertsParams,
    ) -> Result<GetConcertsResponse, KalshiError> {
        if params.page.is_some_and(|page| !(1..=20).contains(&page)) {
            return Err(KalshiError::InvalidParams(
                "GET /concert_cancellation_markets/concerts: page must be 1-20".to_string(),
            ));
        }
        let path = Self::full_path("/concert_cancellation_markets/concerts");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Find upcoming concerts by name, up to 50. **Requires auth.**
    pub async fn search_concerts(
        &self,
        params: SearchConcertsParams,
    ) -> Result<SearchConcertsResponse, KalshiError> {
        let path = Self::full_path("/concert_cancellation_markets/concerts/search");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn weather_request() -> CreateWeatherPointMarketRequest {
        CreateWeatherPointMarketRequest {
            latitude: 40.785,
            longitude: -73.965,
            variable: WeatherPointVariable::PrecipHours,
            strike_type: WeatherPointStrikeType::Greater,
            floor_value: Some(2),
            cap_value: None,
            date: "2026-11-23".into(),
            end_date: None,
            start_hour: 23,
            end_hour: 24,
            timezone: "America/New_York".into(),
        }
    }

    #[test]
    fn weather_request_omits_unset_fields_and_validates_thresholds() {
        let mut request = weather_request();
        request.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "latitude": 40.785, "longitude": -73.965, "variable": "precip_hours",
                "strike_type": "greater", "floor_value": 2, "date": "2026-11-23",
                "start_hour": 23, "end_hour": 24, "timezone": "America/New_York"
            })
        );

        // A window crossing midnight ends on the next date at an earlier hour.
        request.end_date = Some("2026-11-24".into());
        request.end_hour = 3;
        request.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&request).unwrap()["end_date"],
            "2026-11-24"
        );

        request.cap_value = Some(5);
        assert!(request.validate().is_err());
        request.strike_type = WeatherPointStrikeType::Between;
        request.validate().unwrap();
        request.floor_value = None;
        assert!(request.validate().is_err());
        request.strike_type = WeatherPointStrikeType::Less;
        request.validate().unwrap();
        request.end_hour = 0;
        assert!(request.validate().is_err());
    }

    #[test]
    fn weather_responses_parse_required_fields() {
        let created: CreateWeatherPointMarketResponse = serde_json::from_value(json!({
            "event_ticker": "KXRFQRAINHOURS-E", "market_ticker": "KXRFQRAINHOURS-E-M",
            "created": true, "latitude": 40.785, "longitude": -73.965
        }))
        .unwrap();
        assert!(created.created);
        assert_eq!(created.latitude, 40.785);

        let availability: GetWeatherPointAvailabilityResponse = serde_json::from_value(json!({
            "enabled": true,
            "domains": [{"name": "conus", "north": 54.995, "south": 20.005,
                         "west": -129.995, "east": -60.005}],
            "variables": [{"variable": "max_temp", "domains": ["conus"]}],
            "max_window_days": 2, "max_window_hours": 24
        }))
        .unwrap();
        assert_eq!(availability.domains[0].name, "conus");
        assert_eq!(
            availability.variables[0].variable,
            WeatherPointVariable::MaxTemp
        );
        assert_eq!(
            (availability.max_window_days, availability.max_window_hours),
            (2, 24)
        );
    }

    #[test]
    fn concert_shapes_round_trip_optional_fields() {
        let request = CreateConcertCancellationMarketRequest {
            jambase_event_id: "jambase:1".into(),
            scheduled_date: "2026-12-31".into(),
            cover: ConcertCover::Cancelled,
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({"jambase_event_id": "jambase:1", "scheduled_date": "2026-12-31",
                   "cover": "cancelled"})
        );

        let created: CreateConcertCancellationMarketResponse = serde_json::from_value(json!({
            "cover": "either", "event_ticker": "KXRFQCONCERT-E",
            "market_ticker": "KXRFQCONCERT-E-M", "created": false
        }))
        .unwrap();
        assert_eq!(created.cover, ConcertCover::Either);
        assert!(!created.created);

        let availability: GetConcertCancellationAvailabilityResponse =
            serde_json::from_value(json!({
                "enabled": true, "covers": ["postponed", "cancelled", "either"],
                "max_days_ahead": 180,
                "metros": [{"id": "jambase:1", "name": "New York", "upcoming_shows": 0}]
            }))
            .unwrap();
        assert_eq!(availability.covers.len(), 3);
        assert_eq!(availability.metros[0].latitude, None);

        let page: GetConcertsResponse = serde_json::from_value(json!({
            "concerts": [{
                "jambase_event_id": "jambase:1", "name": "Show", "lineup": ["Act"],
                "venue_name": "Hall", "timezone": "America/New_York",
                "scheduled_date": "2026-12-31",
                "markets": [{"cover": "postponed", "event_ticker": "E", "market_ticker": "M"}]
            }],
            "page": 1, "total_pages": 3
        }))
        .unwrap();
        let concert = &page.concerts[0];
        assert_eq!(concert.start_time, None);
        assert_eq!(concert.headliner, None);
        assert_eq!(concert.markets[0].cover, ConcertCover::Postponed);
        assert_eq!(page.total_pages, 3);

        let search: SearchConcertsResponse =
            serde_json::from_value(json!({"concerts": []})).unwrap();
        assert!(search.concerts.is_empty());

        let query = serde_json::to_value(GetConcertsParams {
            metro_id: "jambase:1".into(),
            page: Some(2),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(query, json!({"metro_id": "jambase:1", "page": 2}));
    }
}
