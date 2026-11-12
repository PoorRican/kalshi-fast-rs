//! Incentives, live data feeds, game stats, and milestones.
//!
//! `live_data` endpoints expose real-time feeds tied to sporting-event
//! milestones (scores, play-by-play). `milestones` endpoints enumerate the
//! milestones themselves. `incentive_programs` lists maker-rebate programs.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::events::Milestone;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{FixedPointCount, deserialize_null_as_empty_vec};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIncentiveProgramsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub incentive_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incentive_description: Option<String>,
    /// Filter to one series' incentives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    /// Filter to one event's incentives. A perpetual's incentives belong to no event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIncentiveProgramsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub incentive_programs: Vec<IncentiveProgram>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncentiveProgram {
    pub id: String,
    pub market_id: Option<String>,
    pub market_ticker: Option<String>,
    pub incentive_type: String,
    pub incentive_description: String,
    pub start_date: String,
    pub end_date: String,
    pub period_reward: i64,
    pub paid_out: bool,
    #[serde(default)]
    pub discount_factor_bps: Option<i32>,
    #[serde(default)]
    pub target_size_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub max_reward_per_account: Option<i64>,
    /// Series of the program's market.
    #[serde(default)]
    pub series_ticker: Option<String>,
    /// Event of the program's market; empty for a perpetual.
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Type of incentive an [`IncentiveSchedule`] creates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncentiveScheduleType {
    Liquidity,
    Volume,
    /// A type this crate version does not know about.
    #[serde(other)]
    Unknown,
}

/// A recurring incentive: what each market of a series (or one event) can
/// expect to earn window after window. Anticipated, not promised, unless
/// `all_markets` is true.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IncentiveSchedule {
    pub series_ticker: String,
    pub series_title: String,
    pub series_category: String,
    /// The one event covered; empty when the schedule covers the whole series.
    pub event_ticker: String,
    /// Empty when the schedule covers the whole series.
    pub event_title: String,
    pub incentive_type: IncentiveScheduleType,
    /// Publishing job; with series, event and type it identifies the schedule.
    pub source: String,
    /// Per-market reward per window, in centi-cents (as `period_reward`).
    pub reward: i64,
    /// Days per window; 0 rewards each market once, up to its close.
    pub window_days: i32,
    /// Liquidity only.
    #[serde(default)]
    pub discount_factor_bps: Option<i32>,
    /// Liquidity only.
    #[serde(default)]
    pub target_size_fp: Option<FixedPointCount>,
    /// Exclusive end; `None` when the schedule has no end.
    #[serde(default)]
    pub ends: Option<String>,
    pub description: String,
    /// Whether the terms are promised to every covered market, including
    /// markets listed later.
    pub all_markets: bool,
    pub posted_ts: String,
    pub updated_ts: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetIncentiveSchedulesResponse {
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub schedules: Vec<IncentiveSchedule>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetLiveDatasParams {
    pub milestone_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLiveDatasResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub live_datas: Vec<LiveData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLiveDataResponse {
    pub live_data: LiveData,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLiveDataByMilestoneParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveData {
    #[serde(rename = "type")]
    pub live_data_type: String,
    #[serde(default)]
    pub details: Map<String, Value>,
    pub milestone_id: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventLiveDataParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventLiveDataResponse {
    pub live_data: EventLiveData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventLiveData {
    #[serde(rename = "type")]
    pub live_data_type: String,
    pub details: Map<String, Value>,
    #[serde(default)]
    pub is_historical: Option<bool>,
    #[serde(default)]
    pub default_range: Option<String>,
    #[serde(default)]
    pub range_options: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWeatherIndexParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sec: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexResponse {
    pub city: String,
    #[serde(default)]
    pub config_version: Option<String>,
    pub units: String,
    pub timeseries: Vec<WeatherIndexPoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexPoint {
    /// Event minute in Unix milliseconds.
    pub t: i64,
    /// Fahrenheit index value; absent for incomplete minutes.
    #[serde(default)]
    pub v: Option<f64>,
    pub status: String,
    #[serde(default)]
    pub contributors: Option<i32>,
    /// Present for historical backfill, which is not settlement-eligible.
    #[serde(default)]
    pub receipt_basis: Option<String>,
    #[serde(default)]
    pub stations: Option<Vec<WeatherIndexStationReading>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexStationReading {
    pub station_id: String,
    pub code: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub temp_f: Option<f64>,
    #[serde(default)]
    pub obs_time_ms: Option<i64>,
    #[serde(default)]
    pub received_at_ms: Option<i64>,
    #[serde(default)]
    pub primary_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexCalibrationsResponse {
    pub city: String,
    pub units: String,
    pub calibrations: Vec<WeatherIndexCalibration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexCalibration {
    pub config_version: String,
    #[serde(default)]
    pub published_at_ms: Option<i64>,
    pub effective_at_ms: i64,
    #[serde(default)]
    pub change_reason: Option<String>,
    #[serde(default)]
    pub calibration_window_start_ms: Option<i64>,
    #[serde(default)]
    pub calibration_window_end_ms: Option<i64>,
    pub city_reference_c: f64,
    pub stations: Vec<WeatherIndexCalibrationStation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexCalibrationStation {
    pub station_id: String,
    pub weight: f64,
    pub offset_c: f64,
    #[serde(default)]
    pub update_note: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetGameStatsResponse {
    #[serde(default)]
    pub pbp: Option<Value>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMilestonesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub milestone_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMilestonesResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub milestones: Vec<Milestone>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMilestoneResponse {
    pub milestone: Milestone,
}

impl KalshiRestClient {
    pub async fn get_incentive_programs(
        &self,
        params: GetIncentiveProgramsParams,
    ) -> Result<GetIncentiveProgramsResponse, KalshiError> {
        let path = Self::full_path("/incentive_programs");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// `GET /incentive_schedules` (public): recurring incentive schedules
    /// that have not ended.
    pub async fn get_incentive_schedules(
        &self,
    ) -> Result<GetIncentiveSchedulesResponse, KalshiError> {
        let path = Self::full_path("/incentive_schedules");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_live_data_batch(
        &self,
        params: GetLiveDatasParams,
    ) -> Result<GetLiveDatasResponse, KalshiError> {
        let path = Self::full_path("/live_data/batch");
        let mut query = Vec::with_capacity(
            params.milestone_ids.len() + usize::from(params.include_player_stats.is_some()),
        );
        query.extend(
            params
                .milestone_ids
                .iter()
                .map(|id| ("milestone_ids", id.as_str())),
        );
        if let Some(include_player_stats) = params.include_player_stats {
            query.push((
                "include_player_stats",
                if include_player_stats {
                    "true"
                } else {
                    "false"
                },
            ));
        }
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, false)
            .await
    }

    pub async fn get_live_data(
        &self,
        live_data_type: &str,
        milestone_id: &str,
        params: GetLiveDataByMilestoneParams,
    ) -> Result<GetLiveDataResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/live_data/{live_data_type}/milestone/{milestone_id}"
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

    pub async fn get_live_data_by_milestone(
        &self,
        milestone_id: &str,
        params: GetLiveDataByMilestoneParams,
    ) -> Result<GetLiveDataResponse, KalshiError> {
        let path = Self::full_path(&format!("/live_data/milestone/{milestone_id}"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_event_live_data(
        &self,
        event_ticker: &str,
        params: GetEventLiveDataParams,
    ) -> Result<GetEventLiveDataResponse, KalshiError> {
        let path = Self::full_path(&format!("/live_data/events/{event_ticker}"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_weather_index(
        &self,
        city: &str,
        params: GetWeatherIndexParams,
    ) -> Result<GetWeatherIndexResponse, KalshiError> {
        let path = Self::full_path(&format!("/live_data/weather/{city}"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_weather_index_calibrations(
        &self,
        city: &str,
    ) -> Result<GetWeatherIndexCalibrationsResponse, KalshiError> {
        let path = Self::full_path(&format!("/live_data/weather/{city}/calibrations"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_game_stats(
        &self,
        milestone_id: &str,
    ) -> Result<GetGameStatsResponse, KalshiError> {
        let path = Self::full_path(&format!("/live_data/milestone/{milestone_id}/game_stats"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_milestones(
        &self,
        params: GetMilestonesParams,
    ) -> Result<GetMilestonesResponse, KalshiError> {
        let path = Self::full_path("/milestones");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_milestone(
        &self,
        milestone_id: &str,
    ) -> Result<GetMilestoneResponse, KalshiError> {
        let path = Self::full_path(&format!("/milestones/{milestone_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Create a pager for iterating over milestones page by page.
    pub fn milestones_pager(&self, params: GetMilestonesParams) -> CursorPager<Milestone> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_milestones(page_params).await?;
                Ok((resp.milestones, resp.cursor))
            })
        })
    }

    /// Stream milestones one by one.
    pub fn stream_milestones(
        &self,
        params: GetMilestonesParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Milestone, KalshiError>> + Send {
        stream_items(self.milestones_pager(params), max_items)
    }

    /// Fetch all pages for milestones using cursor pagination.
    pub async fn get_milestones_all(
        &self,
        params: GetMilestonesParams,
    ) -> Result<Vec<Milestone>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_milestones(page_params).await?;
                Ok((resp.milestones, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incentive_schedules_deserialize_spec_shape() {
        let response: GetIncentiveSchedulesResponse = serde_json::from_str(
            r#"{"schedules":[{
                "series_ticker":"KXHIGHNY","series_title":"NYC high","series_category":"Climate",
                "event_ticker":"","event_title":"","incentive_type":"liquidity",
                "source":"daily_liquidity","reward":500000,"window_days":1,
                "discount_factor_bps":5000,"target_size_fp":"1000.00","ends":null,
                "description":"Daily liquidity","all_markets":true,
                "posted_ts":"2026-11-19T00:00:00Z","updated_ts":"2026-11-19T00:00:00Z"},
               {"series_ticker":"S","series_title":"S","series_category":"C",
                "event_ticker":"E","event_title":"E","incentive_type":"volume",
                "source":"v","reward":1,"window_days":0,"ends":"2026-12-01T00:00:00Z",
                "description":"d","all_markets":false,
                "posted_ts":"2026-11-19T00:00:00Z","updated_ts":"2026-11-19T00:00:00Z"}]}"#,
        )
        .unwrap();
        let first = &response.schedules[0];
        assert_eq!(first.incentive_type, IncentiveScheduleType::Liquidity);
        assert!(first.all_markets);
        assert_eq!(first.ends, None);
        assert_eq!(first.reward, 500000);
        let second = &response.schedules[1];
        assert_eq!(second.incentive_type, IncentiveScheduleType::Volume);
        assert_eq!(second.discount_factor_bps, None);
        assert_eq!(second.target_size_fp, None);
        assert_eq!(second.ends.as_deref(), Some("2026-12-01T00:00:00Z"));
    }

    #[test]
    fn incentive_programs_carry_series_and_event_and_filter_by_them() {
        let params = GetIncentiveProgramsParams {
            series_ticker: Some("KXBTC".into()),
            event_ticker: Some("KXBTC-26NOV19".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["series_ticker"], "KXBTC");
        assert_eq!(json["event_ticker"], "KXBTC-26NOV19");

        let base = r#""id":"p","market_id":"m","market_ticker":"T","incentive_type":"liquidity",
            "incentive_description":"new_event","start_date":"2026-11-12T00:00:00Z",
            "end_date":"2026-11-15T00:00:00Z","period_reward":850000,"paid_out":false"#;
        let with: IncentiveProgram = serde_json::from_str(&format!(
            r#"{{{base},"series_ticker":"KXBTCPERP","event_ticker":""}}"#
        ))
        .unwrap();
        assert_eq!(with.series_ticker.as_deref(), Some("KXBTCPERP"));
        assert_eq!(with.event_ticker.as_deref(), Some(""));
        assert!(with.extra.is_empty());
        let without: IncentiveProgram = serde_json::from_str(&format!("{{{base}}}")).unwrap();
        assert_eq!(without.series_ticker, None);
    }

    #[test]
    fn weather_index_retains_incomplete_and_historical_points() {
        let response: GetWeatherIndexResponse = serde_json::from_str(
            r#"{
                "city": "miami", "units": "fahrenheit",
                "timeseries": [
                    {"t": 1793577600000, "status": "incomplete", "stations": [
                        {"station_id": "KMIA1M", "code": "pending", "temp_f": 83.12345},
                        {"station_id": "KOPF1M", "code": "missing"}
                    ]},
                    {"t": 1793577540000, "status": "degraded", "v": 83.12,
                     "contributors": 2, "receipt_basis": "synoptic_latency"}
                ]
            }"#,
        )
        .unwrap();

        let incomplete = &response.timeseries[0];
        assert_eq!(incomplete.v, None);
        assert_eq!(incomplete.contributors, None);
        assert_eq!(incomplete.receipt_basis, None);
        let stations = incomplete.stations.as_ref().unwrap();
        assert_eq!(stations[0].temp_f, Some(83.12345));
        assert_eq!(stations[1].temp_f, None);
        assert_eq!(response.timeseries[1].v, Some(83.12));
        assert_eq!(
            response.timeseries[1].receipt_basis.as_deref(),
            Some("synoptic_latency")
        );
    }

    #[test]
    fn weather_calibration_retains_precision_and_optional_window() {
        let response: GetWeatherIndexCalibrationsResponse = serde_json::from_str(
            r#"{
                "city": "miami", "units": "celsius",
                "calibrations": [{
                    "config_version": "miami-temperature-v1.0",
                    "effective_at_ms": 1793577600000,
                    "city_reference_c": -0.123456789,
                    "stations": [{
                        "station_id": "KMIA1M", "weight": 0.333333333,
                        "offset_c": -0.7654321
                    }]
                }]
            }"#,
        )
        .unwrap();

        let calibration = &response.calibrations[0];
        assert_eq!(calibration.calibration_window_start_ms, None);
        assert_eq!(calibration.city_reference_c, -0.123456789);
        assert_eq!(calibration.stations[0].weight, 0.333333333);
        assert_eq!(calibration.stations[0].offset_c, -0.7654321);
    }

    #[test]
    fn margin_incentive_accepts_absent_or_null_market_fields() {
        let program: IncentiveProgram = serde_json::from_str(
            r#"{
                "id": "margin", "market_ticker": null,
                "incentive_type": "margin_maker_volume",
                "incentive_description": "Margin maker",
                "start_date": "2026-08-27T00:00:00Z",
                "end_date": "2026-09-03T00:00:00Z",
                "period_reward": 500000, "paid_out": false,
                "target_size_fp": null, "max_reward_per_account": 12345
            }"#,
        )
        .unwrap();

        assert_eq!(program.market_id, None);
        assert_eq!(program.market_ticker, None);
        assert_eq!(program.target_size_fp, None);
        assert_eq!(program.max_reward_per_account, Some(12345));
    }
}
