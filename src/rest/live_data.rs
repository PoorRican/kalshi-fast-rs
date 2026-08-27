//! Incentives, live data feeds, game stats, and milestones.
//!
//! `live_data` endpoints expose real-time feeds tied to sporting-event
//! milestones (scores, play-by-play), event-keyed feeds (crypto price charts,
//! commodity timeseries, weather observations), and the standalone
//! city-temperature weather index. `milestones` endpoints enumerate the
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
    pub period_reward: i64,
    pub paid_out: bool,
    #[serde(default)]
    pub discount_factor_bps: Option<i32>,
    #[serde(default)]
    pub target_size: Option<i32>,
    #[serde(default)]
    pub target_size_fp: Option<FixedPointCount>,
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

/// GET /live_data/events/{event_ticker} query params.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventLiveDataParams {
    /// Optional chart range hint (e.g. `15min`, `1h`, `1d`). When the underlying
    /// live data type supports it, restricts the returned timeseries window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

/// Live data keyed by event ticker (e.g. `KXBTC15M` events, commodity price
/// timeseries, weather observations). Added 2026-07-30.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventLiveData {
    /// Names the schema of `details`.
    #[serde(rename = "type")]
    pub live_data_type: String,
    /// Flexible payload whose shape depends on `live_data_type`.
    #[serde(default)]
    pub details: Map<String, Value>,
    /// Present for crypto live data: true when the event has matured and the
    /// payload is a frozen historical snapshot.
    #[serde(default)]
    pub is_historical: Option<bool>,
    /// Chart range the client should default to (e.g. `15min`, `1h`). Omitted when unset.
    #[serde(default)]
    pub default_range: Option<String>,
    /// Chart range menu options. Omitted when unset.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub range_options: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventLiveDataResponse {
    pub live_data: EventLiveData,
}

/// GET /live_data/weather/{city} query params.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWeatherIndexParams {
    /// Window start, unix milliseconds (inclusive). Defaults to `to` minus 24 hours.
    /// Must be paired with `to` unless `last_sec` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Window end, unix milliseconds (inclusive). Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Trailing window in seconds; equivalent to `from=now-last_sec`, `to=now`.
    /// Mutually exclusive with `from`/`to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sec: Option<i64>,
    /// Include per-station audit readings on every point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
}

/// Per-station audit reading on a [`WeatherIndexPoint`] (only present with `detailed=true`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexStationReading {
    /// Member station (e.g. `KMIA1M`) or its official fallback ID.
    pub station_id: String,
    /// Disposition: `ok`, `missing`, `late`, a QC rejection (`range`, `rate_spatial`,
    /// `extreme`), or `pending` (raw reading on an `incomplete` minute).
    pub code: String,
    /// `hf_asos` (exact-minute primary) or `metar` (carried-forward official
    /// observation). Absent when no reading was available.
    #[serde(default)]
    pub source: Option<String>,
    /// Raw reported temperature in Fahrenheit (unrounded). Absent for `missing` members.
    #[serde(default)]
    pub temp_f: Option<f64>,
    /// Observation time for carried-forward fallbacks. Absent for exact-minute primaries.
    #[serde(default)]
    pub obs_time_ms: Option<i64>,
    /// Local wire-receipt time backing the eligibility deadline.
    #[serde(default)]
    pub received_at_ms: Option<i64>,
    /// Why the primary observation was passed over when a fallback was selected instead.
    #[serde(default)]
    pub primary_code: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// One minute of the Kalshi-computed city temperature index.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexPoint {
    /// Event minute, unix milliseconds UTC.
    pub t: i64,
    /// Published index value, Fahrenheit rounded to 0.01. Absent on `incomplete`
    /// points, which have no canonical value yet (not `0`).
    #[serde(default)]
    pub v: Option<f64>,
    /// `normal`, `degraded`, or (with `detailed=true`) `incomplete`.
    pub status: String,
    /// Number of accepted member stations backing the point. Absent on `incomplete` points.
    #[serde(default)]
    pub contributors: Option<i64>,
    /// Per-station audit readings (only with `detailed=true`), sorted by station ID.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub stations: Vec<WeatherIndexStationReading>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Response for `GET /live_data/weather/{city}`. Added 2026-08-20.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexResponse {
    pub city: String,
    /// Index configuration version of the newest returned point (e.g.
    /// `miami-temperature-v1.0`). Empty when no points matched the window.
    #[serde(default)]
    pub config_version: Option<String>,
    /// Always `fahrenheit`.
    pub units: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub timeseries: Vec<WeatherIndexPoint>,
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

    /// Get live data keyed by event ticker (e.g. crypto price charts, commodity
    /// price timeseries, weather observations). Added 2026-07-30.
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

    /// Get the Kalshi-computed city temperature index (minute-resolution series
    /// behind hourly temperature markets). Added 2026-08-20.
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
