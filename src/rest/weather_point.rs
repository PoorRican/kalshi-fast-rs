//! On-request weather point markets.
//!
//! `POST /weather_point_markets` returns (creating if needed) the RFQ-traded
//! weather market for a location, local-hour window and threshold.
//! `GET /weather_point_markets/availability` lists what can be created.
//! These markets live in `KXRFQ` series: they are left out of unscoped market
//! listings and of unscoped `trade`/`ticker`/`market_lifecycle_v2` feeds.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::deserialize_null_as_empty_vec;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    /// Whole hours with at least 0.01 inch of precipitation (0 to window length).
    PrecipHours,
    /// Total precipitation in hundredths of an inch (0 to 2000).
    PrecipTotal,
    /// Highest temperature, whole °F (-80 to 140).
    MaxTemp,
    /// Lowest temperature, whole °F (-80 to 140).
    MinTemp,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Yes strictly above `floor_value`; `cap_value` must be omitted.
    Greater,
    /// Yes strictly below `cap_value`; `floor_value` must be omitted.
    Less,
    /// Yes from `floor_value` to `cap_value` inclusive.
    Between,
}

/// Body for [`KalshiRestClient::create_weather_point_market`].
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateWeatherPointMarketRequest {
    /// Latitude in degrees (-90 to 90).
    pub latitude: f64,
    /// Longitude in degrees (-180 to 180).
    pub longitude: f64,
    pub variable: WeatherPointVariable,
    pub strike_type: WeatherPointStrikeType,
    /// Lower threshold in the variable's unit; required for `greater`/`between`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_value: Option<i32>,
    /// Upper threshold in the variable's unit; required for `less`/`between`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_value: Option<i32>,
    /// Local calendar date the window starts on, `YYYY-MM-DD` in `timezone`.
    pub date: String,
    /// First local hour of the window (0-23).
    pub start_hour: u8,
    /// Local hour the window ends at, exclusive (1-24, after `start_hour`).
    pub end_hour: u8,
    /// IANA time zone the date and hours are read in.
    pub timezone: String,
}

impl CreateWeatherPointMarketRequest {
    fn validate(&self) -> Result<(), KalshiError> {
        let invalid = |msg: &str| Err(KalshiError::InvalidParams(msg.to_string()));
        if !(-90.0..=90.0).contains(&self.latitude) || !(-180.0..=180.0).contains(&self.longitude) {
            return invalid("create_weather_point_market: latitude/longitude out of range");
        }
        if self.start_hour > 23 || !(1..=24).contains(&self.end_hour) {
            return invalid("create_weather_point_market: start_hour must be 0-23, end_hour 1-24");
        }
        if self.end_hour <= self.start_hour {
            return invalid("create_weather_point_market: end_hour must be after start_hour");
        }
        let (needs_floor, needs_cap) = match self.strike_type {
            WeatherPointStrikeType::Greater => (true, false),
            WeatherPointStrikeType::Less => (false, true),
            WeatherPointStrikeType::Between => (true, true),
        };
        if self.floor_value.is_some() != needs_floor || self.cap_value.is_some() != needs_cap {
            return invalid(
                "create_weather_point_market: greater needs only floor_value, less only cap_value, between both",
            );
        }
        if self.date.is_empty() || self.timezone.is_empty() {
            return invalid("create_weather_point_market: date and timezone are required");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    /// Pass to `POST /communications/rfqs` to trade the market.
    pub market_ticker: String,
    /// False when a market for the same request already existed.
    pub created: bool,
    /// Latitude of the snapped NOAA MRMS grid-cell center.
    pub latitude: f64,
    /// Longitude of the snapped NOAA MRMS grid-cell center.
    pub longitude: f64,
}

/// A NOAA MRMS domain a location may lie in (bounds are grid-cell centers).
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
    /// Variables on offer; a variable left out can't be created.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub variables: Vec<WeatherPointVariableAvailability>,
}

impl KalshiRestClient {
    /// Return the on-request weather market for a location, window and
    /// threshold, creating it if it doesn't exist yet. Charged against the
    /// write budget. A just-created market may briefly 404 on `get_market`.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> CreateWeatherPointMarketRequest {
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
    fn create_request_serializes_and_validates_thresholds() {
        let body = request();
        body.validate().unwrap();
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            serde_json::json!({
                "latitude": 40.785, "longitude": -73.965, "variable": "precip_hours",
                "strike_type": "greater", "floor_value": 2, "date": "2026-11-11",
                "start_hour": 0, "end_hour": 24, "timezone": "America/New_York"
            })
        );
        for bad in [
            CreateWeatherPointMarketRequest {
                cap_value: Some(3),
                ..request()
            },
            CreateWeatherPointMarketRequest {
                strike_type: WeatherPointStrikeType::Between,
                ..request()
            },
            CreateWeatherPointMarketRequest {
                start_hour: 5,
                end_hour: 5,
                ..request()
            },
            CreateWeatherPointMarketRequest {
                latitude: 91.0,
                ..request()
            },
        ] {
            assert!(bad.validate().is_err());
        }
    }

    #[test]
    fn responses_deserialize_published_shapes() {
        let created: CreateWeatherPointMarketResponse = serde_json::from_str(
            r#"{"event_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z","market_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z-24H-G2","created":true,"latitude":40.785,"longitude":-73.965}"#,
        )
        .unwrap();
        assert!(created.created);
        let availability: GetWeatherPointAvailabilityResponse = serde_json::from_str(
            r#"{"enabled":true,"domains":[{"name":"conus","north":54.995,"south":20.005,"west":-129.995,"east":-60.005}],"variables":[{"variable":"max_temp","domains":["conus"]}]}"#,
        )
        .unwrap();
        assert_eq!(
            availability.variables[0].variable,
            WeatherPointVariable::MaxTemp
        );
        assert_eq!(availability.domains[0].name, "conus");
    }
}
