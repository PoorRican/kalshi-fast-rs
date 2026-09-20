//! Incentives, live data feeds, game stats, and milestones.
//!
//! `live_data` endpoints expose real-time feeds tied to sporting-event
//! milestones (scores, play-by-play), to events (crypto price charts,
//! commodity timeseries, weather observations), and the Kalshi Weather Index.
//! `milestones` endpoints enumerate the milestones themselves.
//! `incentive_programs` lists maker-rebate programs.

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
    pub market_id: String,
    pub market_ticker: String,
    pub incentive_type: String,
    pub start_date: String,
    pub end_date: String,
    /// Human-readable description of the program.
    #[serde(default)]
    pub incentive_description: Option<String>,
    pub period_reward: i64,
    pub paid_out: bool,
    #[serde(default)]
    pub discount_factor_bps: Option<i32>,
    #[serde(default)]
    pub target_size_fp: Option<FixedPointCount>,
    /// Optional per-account reward cap. Present on the volume-based program
    /// types added in 2026-08/09; omitted when the program has no cap.
    #[serde(default)]
    pub max_reward_per_account: Option<i64>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetLiveDatasParams {
    pub milestone_ids: Vec<String>,
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

/// Live data keyed by event ticker (`GET /live_data/events/{event_ticker}`).
///
/// Added 2026-07-30. `live_data_type` names the schema of the flexible
/// `details` object (e.g. crypto price charts, commodity timeseries, weather
/// observations).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventLiveData {
    #[serde(rename = "type")]
    pub live_data_type: String,
    #[serde(default)]
    pub details: Map<String, Value>,
    /// Present for crypto live data: true once the event has matured and the
    /// payload is a frozen historical snapshot.
    #[serde(default)]
    pub is_historical: Option<bool>,
    /// Chart range the client should default to (e.g. `"15min"`, `"1h"`).
    #[serde(default)]
    pub default_range: Option<String>,
    /// Chart range menu options.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub range_options: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventLiveDataResponse {
    pub live_data: EventLiveData,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventLiveDataParams {
    /// Restrict the returned timeseries window (e.g. `"15min"`, `"1h"`, `"1d"`)
    /// where the underlying live data type supports it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

/// GET /live_data/weather/{city} query params. Added 2026-08-20.
///
/// Window with `from`/`to` (unix ms) or `last_sec`; the default is the
/// trailing 24 hours.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWeatherIndexParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sec: Option<i64>,
    /// Include each member station's reported reading and quality-control
    /// disposition, plus trailing `incomplete` minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
}

/// Per-station audit reading on a weather index point (`detailed=true` only).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexStationReading {
    pub station_id: String,
    /// Disposition: `ok`, `missing`, `late`, a QC rejection (`range`,
    /// `rate_spatial`, `extreme`), or `pending` on an `incomplete` minute.
    pub code: String,
    /// `hf_asos` (exact-minute primary) or `metar` (carried-forward official
    /// observation). Absent when no reading was available.
    #[serde(default)]
    pub source: Option<String>,
    /// Raw reported temperature in Fahrenheit, unrounded.
    #[serde(default)]
    pub temp_f: Option<f64>,
    #[serde(default)]
    pub obs_time_ms: Option<i64>,
    #[serde(default)]
    pub received_at_ms: Option<i64>,
    #[serde(default)]
    pub primary_code: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// One minute of the Kalshi city temperature index.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexPoint {
    /// Event minute, unix milliseconds UTC.
    pub t: i64,
    /// Published index value, Fahrenheit rounded to 0.01. Absent (**not** `0`)
    /// on `incomplete` points, which have no canonical value yet.
    #[serde(default)]
    pub v: Option<f64>,
    /// `normal`, `degraded`, or (with `detailed=true`) `incomplete`.
    pub status: String,
    /// Number of accepted member stations backing the point. Absent on
    /// `incomplete` points.
    #[serde(default)]
    pub contributors: Option<i32>,
    /// Present only on points written by the labelled historical backfill that
    /// seeds a newly listed city's series. Added 2026-09-10. The single
    /// documented value is `synoptic_latency`, meaning the receipt deadline was
    /// judged against `observation_time + Synoptic ingest latency` instead of
    /// Kalshi's local receipt clock. **Such points are not settlement-eligible.**
    /// Absent on canonical points.
    #[serde(default)]
    pub receipt_basis: Option<String>,
    /// Per-station audit readings, only with `detailed=true`.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub stations: Vec<WeatherIndexStationReading>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexResponse {
    pub city: String,
    /// Index configuration version of the newest returned point. Empty when no
    /// points matched the window.
    #[serde(default)]
    pub config_version: Option<String>,
    /// Always `fahrenheit`.
    pub units: String,
    /// Minutes where the index quorum failed are omitted, so gaps are real gaps.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub timeseries: Vec<WeatherIndexPoint>,
}

/// One member station's weight and offset within a weather index configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexCalibrationStation {
    pub station_id: String,
    /// Base weight; weights sum to 1.0 across members.
    pub weight: f64,
    /// Station offset in Celsius (positive = normally runs warmer than peers).
    pub offset_c: f64,
    /// Weekly-calibration disposition, present only on weekly records.
    #[serde(default)]
    pub update_note: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// One published weather index configuration version.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexCalibration {
    /// Configuration version; index points report the version they were
    /// computed under in their own `config_version`.
    pub config_version: String,
    #[serde(default)]
    pub published_at_ms: Option<i64>,
    /// Governs event minutes at or after this time, until superseded.
    pub effective_at_ms: i64,
    #[serde(default)]
    pub change_reason: Option<String>,
    /// Absent on records not derived from a calibration window (the launch
    /// configuration).
    #[serde(default)]
    pub calibration_window_start_ms: Option<i64>,
    #[serde(default)]
    pub calibration_window_end_ms: Option<i64>,
    /// City reference `B_c` in Celsius: the weight-dot-offset sum over members.
    pub city_reference_c: f64,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub stations: Vec<WeatherIndexCalibrationStation>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexCalibrationsResponse {
    pub city: String,
    /// Always `celsius` — offsets and the city reference are Celsius
    /// quantities, while the published index value itself is Fahrenheit.
    pub units: String,
    /// Configuration records, ascending by effective time.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub calibrations: Vec<WeatherIndexCalibration>,
}

impl KalshiRestClient {
    /// Get live data keyed by event ticker.
    ///
    /// Serves event-keyed live data such as crypto price charts (e.g.
    /// `KXBTC15M` events), commodity price timeseries, and weather
    /// observations. The response's `live_data.live_data_type` names the schema
    /// of the flexible `details` object.
    ///
    /// Added 2026-07-30.
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

    /// Get the Kalshi-computed minute-resolution city temperature index.
    ///
    /// Values are Fahrenheit rounded to 0.01. Minutes where the index quorum
    /// failed are omitted, so gaps in `timeseries` are real gaps. With
    /// `detailed = Some(true)`, each point carries every member station's
    /// reading and quality-control disposition, and trailing minutes still
    /// inside the receipt deadline are additionally served as `incomplete`
    /// points whose `v` is absent (not `0`).
    ///
    /// Added 2026-08-20.
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

    /// Get the published weather-index configuration timeline for a city.
    ///
    /// Returns the launch configuration plus every weekly offset calibration
    /// and methodology update, ascending by effective time, so index values can
    /// be reproduced under the configuration version that computed them (which
    /// each point reports in `config_version`).
    ///
    /// Added 2026-08-31.
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

    pub async fn get_live_data_batch(
        &self,
        params: GetLiveDatasParams,
    ) -> Result<GetLiveDatasResponse, KalshiError> {
        let path = Self::full_path("/live_data/batch");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_live_data(
        &self,
        live_data_type: &str,
        milestone_id: &str,
    ) -> Result<GetLiveDataResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/live_data/{live_data_type}/milestone/{milestone_id}"
        ));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
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
