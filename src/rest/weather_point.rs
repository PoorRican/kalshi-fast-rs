//! On-request weather point markets (`KXRFQ*` series).
//!
//! [`KalshiRestClient::create_weather_point_market`] returns (creating if
//! needed) the market for a location, time window and threshold; trade it by
//! sending an RFQ for the returned `market_ticker`. These markets are left out
//! of unscoped market, trade and WebSocket listings, so name them explicitly.
//!
//! Refusals carry specific error codes: `409 weather_point_market_unavailable`
//! (valid request not offered, e.g. over open water) and
//! `429 weather_point_market_limit_exceeded` (a creation limit, not request-rate
//! limiting; retrying immediately does not help).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::deserialize_null_as_empty_vec;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// What a weather point market measures over its window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointVariable {
    /// Whole hours with at least 0.01 inch of precipitation (0 to window length).
    PrecipHours,
    /// Total precipitation in hundredths of an inch (0 to 2000).
    PrecipTotal,
    /// Highest temperature in whole °F (-80 to 140).
    MaxTemp,
    /// Lowest temperature in whole °F (-80 to 140).
    MinTemp,
    /// A variable this crate version does not know about.
    #[serde(other)]
    Unknown,
}

/// How the measured value is compared to the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherPointStrikeType {
    /// Settles Yes strictly above `floor_value`.
    Greater,
    /// Settles Yes strictly below `cap_value`.
    Less,
    /// Settles Yes from `floor_value` to `cap_value` inclusive.
    Between,
}

/// Body for `POST /weather_point_markets`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateWeatherPointMarketRequest {
    /// Latitude in degrees (-90 to 90).
    pub latitude: f64,
    /// Longitude in degrees (-180 to 180).
    pub longitude: f64,
    pub variable: WeatherPointVariable,
    pub strike_type: WeatherPointStrikeType,
    /// Required for `Greater` and `Between`; must be omitted for `Less`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_value: Option<i32>,
    /// Required for `Less` and `Between`; must be omitted for `Greater`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_value: Option<i32>,
    /// Local calendar date the window starts on (`YYYY-MM-DD` in `timezone`).
    pub date: String,
    /// First local hour of the window (0-23).
    pub start_hour: i32,
    /// Exclusive local end hour (1-24, after `start_hour`).
    pub end_hour: i32,
    /// IANA time zone; must be offset from UTC by whole hours.
    pub timezone: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateWeatherPointMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    /// `false` when a market for the same request already existed.
    pub created: bool,
    /// Snapped grid-cell center latitude.
    pub latitude: f64,
    /// Snapped grid-cell center longitude.
    pub longitude: f64,
}

/// A NOAA MRMS domain a location may lie in.
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
    /// Names of the domains new markets of this variable may be created in.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetWeatherPointAvailabilityResponse {
    /// Whether weather point markets can be created at all.
    pub enabled: bool,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub domains: Vec<WeatherPointDomain>,
    /// Variables on offer; a variable left out cannot be created.
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub variables: Vec<WeatherPointVariableAvailability>,
}

impl KalshiRestClient {
    /// `POST /weather_point_markets` (auth required; charged to the write budget).
    pub async fn create_weather_point_market(
        &self,
        body: &CreateWeatherPointMarketRequest,
    ) -> Result<CreateWeatherPointMarketResponse, KalshiError> {
        let path = Self::full_path("/weather_point_markets");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_serializes_spec_shape() {
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
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["variable"], "precip_hours");
        assert_eq!(json["strike_type"], "greater");
        assert_eq!(json["floor_value"], 2);
        assert!(json.get("cap_value").is_none());
        assert_eq!(json["end_hour"], 24);
    }

    #[test]
    fn create_response_deserializes() {
        let resp: CreateWeatherPointMarketResponse = serde_json::from_str(
            r#"{"event_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z",
                "market_ticker":"KXRFQRAINHOURS-40785N73965W-26NOV1105Z-24H-G2",
                "created":true,"latitude":40.785,"longitude":-73.965}"#,
        )
        .unwrap();
        assert!(resp.created);
        assert_eq!(
            resp.market_ticker,
            "KXRFQRAINHOURS-40785N73965W-26NOV1105Z-24H-G2"
        );
    }

    #[test]
    fn availability_deserializes_and_tolerates_unknown_variables() {
        let resp: GetWeatherPointAvailabilityResponse = serde_json::from_str(
            r#"{"enabled":true,
                "domains":[{"name":"conus","north":54.995,"south":20.005,"west":-129.995,"east":-60.005}],
                "variables":[{"variable":"max_temp","domains":["conus"]},
                             {"variable":"wind_gust","domains":null}]}"#,
        )
        .unwrap();
        assert!(resp.enabled);
        assert_eq!(resp.domains[0].name, "conus");
        assert_eq!(resp.variables[0].variable, WeatherPointVariable::MaxTemp);
        assert_eq!(resp.variables[1].variable, WeatherPointVariable::Unknown);
        assert!(resp.variables[1].domains.is_empty());
    }
}
