//! On-request markets: weather point markets and concert cancellation markets.
//!
//! These endpoints return the market for a request, creating it when it does
//! not exist yet. The markets live in series starting with `KXRFQ`, are left
//! out of unscoped listings and subscriptions, and trade through RFQs: pass
//! the returned `market_ticker` to
//! [`KalshiRestClient::create_rfq`](crate::KalshiRestClient::create_rfq).
//!
//! A `429` from the create and search operations can carry a capacity code
//! (`*_limit_exceeded`) instead of `too_many_requests`; the client does not
//! retry those.

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
    /// Total precipitation in hundredths of an inch.
    PrecipTotal,
    /// Highest temperature in whole °F.
    MaxTemp,
    /// Lowest temperature in whole °F.
    MinTemp,
    /// A variable published after this crate version. Not valid in requests.
    #[serde(other)]
    Unknown,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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
    /// Local calendar date the window starts on, as `YYYY-MM-DD` in `timezone`.
    pub date: String,
    /// First local hour of the window (0-23).
    pub start_hour: u8,
    /// Local hour the window ends at, exclusive (1-24, after `start_hour`).
    pub end_hour: u8,
    /// IANA time zone of the location.
    pub timezone: String,
}

impl CreateWeatherPointMarketRequest {
    pub fn validate(&self) -> Result<(), KalshiError> {
        let invalid = |reason: &str| {
            Err(KalshiError::InvalidParams(format!(
                "POST /weather_point_markets: {reason}"
            )))
        };
        if !(-90.0..=90.0).contains(&self.latitude) {
            return invalid("latitude must be -90..=90");
        }
        if !(-180.0..=180.0).contains(&self.longitude) {
            return invalid("longitude must be -180..=180");
        }
        if self.variable == WeatherPointVariable::Unknown {
            return invalid("variable must be a published variable");
        }
        if self.start_hour > 23 || !(1..=24).contains(&self.end_hour) {
            return invalid("start_hour must be 0..=23 and end_hour 1..=24");
        }
        if self.end_hour <= self.start_hour {
            return invalid("end_hour must be after start_hour");
        }
        let (floor, cap) = (self.floor_value.is_some(), self.cap_value.is_some());
        let thresholds_fit = match self.strike_type {
            WeatherPointStrikeType::Greater => floor && !cap,
            WeatherPointStrikeType::Less => cap && !floor,
            WeatherPointStrikeType::Between => floor && cap,
        };
        if !thresholds_fit {
            return invalid(
                "greater takes floor_value only, less cap_value only, between takes both",
            );
        }
        Ok(())
    }
}

/// Response of `POST /weather_point_markets`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// `false` when a market for the same request already existed.
    pub created: bool,
    /// Center of the grid cell containing the request, not the request value.
    pub latitude: f64,
    /// Center of the grid cell containing the request, not the request value.
    pub longitude: f64,
}

/// A NOAA MRMS domain a weather point market location may lie in.
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
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<String>,
}

/// Response of `GET /weather_point_markets/availability`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    pub enabled: bool,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<WeatherPointDomain>,
    /// A variable left out cannot be created.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
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

/// Body of `POST /concert_cancellation_markets`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateConcertCancellationMarketRequest {
    /// The show, as `jambase:<id>`, from the concert listing or search.
    pub jambase_event_id: String,
    /// Venue-local date the show is scheduled for, as `YYYY-MM-DD`.
    pub scheduled_date: String,
    pub cover: ConcertCover,
}

/// Response of `POST /concert_cancellation_markets`.
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
    /// Count shows from this venue-local date (`YYYY-MM-DD`); today when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Count shows up to this venue-local date, inclusive (`YYYY-MM-DD`).
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
    /// `0` when no count is available.
    pub upcoming_shows: i32,
}

/// Response of `GET /concert_cancellation_markets/availability`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertCancellationAvailabilityResponse {
    pub enabled: bool,
    /// A cover left out cannot be created.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub covers: Vec<ConcertCover>,
    /// How many days ahead a show may be scheduled for a new market.
    pub max_days_ahead: i32,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub metros: Vec<ConcertMetro>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConcertsParams {
    /// One of the metros from the availability response, such as `jambase:1`.
    pub metro_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Page to return, 1..=20; the first page when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl GetConcertsParams {
    pub fn validate(&self) -> Result<(), KalshiError> {
        if self.metro_id.is_empty() {
            return Err(KalshiError::InvalidParams(
                "GET /concert_cancellation_markets/concerts: metro_id is required".to_string(),
            ));
        }
        if let Some(page) = self.page
            && !(1..=20).contains(&page)
        {
            return Err(KalshiError::InvalidParams(
                "GET /concert_cancellation_markets/concerts: page must be 1..=20".to_string(),
            ));
        }
        Ok(())
    }
}

/// The tickers a show has, or would get, for one offered cover. The market
/// may not exist yet: create it before sending an RFQ.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcertMarket {
    pub cover: ConcertCover,
    pub event_ticker: String,
    pub market_ticker: String,
}

/// An upcoming concert a cancellation market can still be created for.
/// Dates and times are local to the venue.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Concert {
    /// The show's id, as `jambase:<id>`.
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
    /// IANA time zone `scheduled_date` and `start_time` are local to.
    pub timezone: String,
    /// `YYYY-MM-DD`.
    pub scheduled_date: String,
    /// `YYYY-MM-DDTHH:MM:SS` with no UTC offset; absent while only the date
    /// is announced.
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    /// One entry per offered cover.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<ConcertMarket>,
}

/// Response of `GET /concert_cancellation_markets/concerts`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetConcertsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
    pub page: i32,
    pub total_pages: i32,
}

/// Response of `GET /concert_cancellation_markets/concerts/search`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchConcertsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub concerts: Vec<Concert>,
}

impl KalshiRestClient {
    /// Return the on-request weather market for a location, window and
    /// threshold, creating it if it does not exist yet. Requires auth and is
    /// charged against the write budget.
    pub async fn create_weather_point_market(
        &self,
        body: CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        body.validate()?;
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
    /// creating it if it does not exist yet. Requires auth and is charged
    /// against the write budget.
    pub async fn create_concert_cancellation_market(
        &self,
        body: CreateConcertCancellationMarketRequest,
    ) -> Result<CreateConcertCancellationMarketResponse, KalshiError> {
        if body.cover == ConcertCover::Unknown {
            return Err(KalshiError::InvalidParams(
                "POST /concert_cancellation_markets: cover must be a published cover".to_string(),
            ));
        }
        let path = Self::full_path("/concert_cancellation_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Which on-request concert markets can be created, and the metros whose
    /// concerts are listed. Public.
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

    /// List a metro's upcoming concerts, soonest first, 50 to a page and at
    /// most 20 pages. Requires auth.
    pub async fn get_concerts(
        &self,
        params: GetConcertsParams,
    ) -> Result<GetConcertsResponse, KalshiError> {
        params.validate()?;
        let path = Self::full_path("/concert_cancellation_markets/concerts");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Find up to 50 upcoming concerts by name (2 to 80 characters) across
    /// metros. Requires auth. A `429` with `concert_search_limit_exceeded` is
    /// a shared search limit, not request-rate limiting.
    pub async fn search_concerts(&self, q: &str) -> Result<SearchConcertsResponse, KalshiError> {
        let len = q.chars().count();
        if !(2..=80).contains(&len) {
            return Err(KalshiError::InvalidParams(
                "GET /concert_cancellation_markets/concerts/search: q must be 2..=80 characters"
                    .to_string(),
            ));
        }
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

    fn weather_request() -> CreateWeatherPointMarketRequest {
        CreateWeatherPointMarketRequest {
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
        }
    }

    #[test]
    fn weather_request_serializes_published_shape_and_omits_unused_threshold() {
        let request = weather_request();
        request.validate().unwrap();
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "latitude": 40.785, "longitude": -73.965,
                "variable": "precip_hours", "strike_type": "greater",
                "floor_value": 2, "date": "2026-11-11",
                "start_hour": 0, "end_hour": 24, "timezone": "America/New_York"
            })
        );
    }

    #[test]
    fn weather_request_validation_rejects_unusable_requests() {
        let mut request = weather_request();
        request.cap_value = Some(3);
        assert!(request.validate().is_err(), "greater must omit cap_value");
        request.strike_type = WeatherPointStrikeType::Between;
        request.validate().unwrap();
        request.strike_type = WeatherPointStrikeType::Less;
        request.floor_value = None;
        request.validate().unwrap();

        request.end_hour = request.start_hour;
        assert!(request.validate().is_err());
        request.end_hour = 25;
        assert!(request.validate().is_err());
        request.end_hour = 24;
        request.latitude = 91.0;
        assert!(request.validate().is_err());
        request.latitude = 40.0;
        request.variable = WeatherPointVariable::Unknown;
        assert!(request.validate().is_err());
    }

    #[test]
    fn weather_responses_parse_required_fields() {
        let created: CreateWeatherPointMarketResponse = serde_json::from_str(
            r#"{"event_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z-24H",
                "market_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z-24H-G2",
                "created":true,"latitude":40.785,"longitude":-73.965}"#,
        )
        .unwrap();
        assert!(created.created);
        assert_eq!(created.latitude, 40.785);

        // Captured from production on 2026-11-14, plus an unpublished variable.
        let availability: GetWeatherPointAvailabilityResponse = serde_json::from_str(
            r#"{"domains":[{"east":-60.005,"name":"conus","north":54.995,"south":20.005,"west":-129.995}],
                "enabled":true,
                "variables":[{"domains":["conus"],"variable":"precip_hours"},
                             {"domains":null,"variable":"wind_gust"}]}"#,
        )
        .unwrap();
        assert!(availability.enabled);
        assert_eq!(availability.domains[0].name, "conus");
        assert_eq!(
            availability.variables[0].variable,
            WeatherPointVariable::PrecipHours
        );
        assert_eq!(
            availability.variables[1].variable,
            WeatherPointVariable::Unknown
        );
        assert!(availability.variables[1].domains.is_empty());
    }

    #[test]
    fn concert_shapes_parse_and_serialize() {
        let request = CreateConcertCancellationMarketRequest {
            jambase_event_id: "jambase:16543035".into(),
            scheduled_date: "2026-11-20".into(),
            cover: ConcertCover::Postponed,
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "jambase_event_id": "jambase:16543035",
                "scheduled_date": "2026-11-20",
                "cover": "postponed"
            })
        );

        let created: CreateConcertCancellationMarketResponse = serde_json::from_str(
            r#"{"cover":"postponed","event_ticker":"KXRFQCONCERTP-JB16543035-26NOV20",
                "market_ticker":"KXRFQCONCERTP-JB16543035-26NOV20-PPD","created":false}"#,
        )
        .unwrap();
        assert_eq!(created.cover, ConcertCover::Postponed);
        assert!(!created.created);

        // Captured from production on 2026-11-14 (truncated), plus a metro
        // without coordinates.
        let availability: GetConcertCancellationAvailabilityResponse = serde_json::from_str(
            r#"{"covers":["postponed","cancelled","either"],"enabled":true,"max_days_ahead":180,
                "metros":[{"id":"jambase:1","latitude":40.6943,"longitude":-73.9249,"name":"New York","upcoming_shows":2848},
                          {"id":"jambase:9","name":"Nowhere","upcoming_shows":0}]}"#,
        )
        .unwrap();
        assert_eq!(availability.covers.len(), 3);
        assert_eq!(availability.max_days_ahead, 180);
        assert_eq!(availability.metros[0].latitude, Some(40.6943));
        assert!(availability.metros[1].longitude.is_none());

        // Only the required Concert fields, then a fully populated one.
        let listing: GetConcertsResponse = serde_json::from_str(
            r#"{"concerts":[
                {"jambase_event_id":"jambase:1","name":"Show","lineup":["A","B"],
                 "venue_name":"Hall","timezone":"America/New_York","scheduled_date":"2026-11-20",
                 "markets":[{"cover":"either","event_ticker":"E","market_ticker":"M"}]},
                {"jambase_event_id":"jambase:2","name":"Show 2","headliner":"A","lineup":["A"],
                 "venue_name":"Hall","city":"New York","region":"NY","country":"US",
                 "timezone":"America/New_York","scheduled_date":"2026-11-21",
                 "start_time":"2026-11-21T20:00:00","latitude":40.7,"longitude":-73.9,
                 "url":"https://example.com/show","markets":[]}
               ],"page":1,"total_pages":3}"#,
        )
        .unwrap();
        assert_eq!(listing.total_pages, 3);
        assert!(listing.concerts[0].start_time.is_none());
        assert_eq!(listing.concerts[0].markets[0].cover, ConcertCover::Either);
        assert_eq!(listing.concerts[1].headliner.as_deref(), Some("A"));

        let search: SearchConcertsResponse = serde_json::from_str(r#"{"concerts":[]}"#).unwrap();
        assert!(search.concerts.is_empty());
    }

    #[test]
    fn concert_params_validate_and_serialize() {
        let mut params = GetConcertsParams {
            metro_id: "jambase:1".into(),
            page: Some(2),
            ..Default::default()
        };
        params.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            serde_json::json!({"metro_id": "jambase:1", "page": 2})
        );
        params.page = Some(21);
        assert!(params.validate().is_err());
        params.page = None;
        params.metro_id.clear();
        assert!(params.validate().is_err());
    }
}
