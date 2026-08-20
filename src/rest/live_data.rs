//! Incentives, live data feeds, game stats, and milestones.
//!
//! `live_data` endpoints expose real-time feeds tied to sporting-event
//! milestones (scores, play-by-play), to events (crypto/commodity price
//! timeseries, weather observations), and to the Kalshi city temperature index.
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
    /// Filter by exact incentive description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incentive_description: Option<String>,
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
    /// `liquidity` or `volume`.
    pub incentive_type: String,
    /// Human-readable description of the incentive program.
    pub incentive_description: String,
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

/// Query params for `GET /live_data/batch`.
///
/// `milestone_ids` is serialized as a repeated query parameter
/// (`milestone_ids=a&milestone_ids=b`), per the OpenAPI `form`/`explode: true`
/// style. Maximum 100 IDs.
#[derive(Debug, Clone, Default)]
pub struct GetLiveDatasParams {
    pub milestone_ids: Vec<String>,
    /// When true, includes player-level statistics in the live data response.
    pub include_player_stats: Option<bool>,
}

impl GetLiveDatasParams {
    /// Flatten into repeated `milestone_ids` query pairs.
    ///
    /// `serde_urlencoded` (used by `reqwest::RequestBuilder::query`) cannot
    /// serialize a struct field holding a sequence, so the pairs are built
    /// explicitly here.
    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(self.milestone_ids.len() + 1);
        for id in &self.milestone_ids {
            pairs.push(("milestone_ids", id.clone()));
        }
        if let Some(include_player_stats) = self.include_player_stats {
            pairs.push(("include_player_stats", include_player_stats.to_string()));
        }
        pairs
    }
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

/// Query params for `GET /live_data/events/{event_ticker}`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEventLiveDataParams {
    /// Optional chart range hint (e.g. `15min`, `1h`, `1d`). Restricts the
    /// returned timeseries window when the live data type supports it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

/// Event-keyed live data: crypto price charts, commodity price timeseries,
/// weather observations. Added 2026-07-30.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventLiveData {
    /// Type of live data. Names the schema of [`Self::details`].
    #[serde(rename = "type")]
    pub live_data_type: String,
    /// Flexible payload whose shape depends on [`Self::live_data_type`].
    #[serde(default)]
    pub details: Map<String, Value>,
    /// Present for crypto live data: true when the event has matured and the
    /// payload is a frozen historical snapshot.
    #[serde(default)]
    pub is_historical: Option<bool>,
    /// Chart range the client should default to (e.g. `15min`, `1h`).
    #[serde(default)]
    pub default_range: Option<String>,
    /// Chart range menu options. Empty when unset.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub range_options: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEventLiveDataResponse {
    pub live_data: EventLiveData,
}

/// Query params for `GET /live_data/weather/{city}`.
///
/// Use either `from`/`to` (unix milliseconds, inclusive) or `last_sec`; they
/// are mutually exclusive. Defaults to the trailing 24 hours.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWeatherIndexParams {
    /// Window start, unix milliseconds (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Window end, unix milliseconds (inclusive). Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Trailing window in seconds. Mutually exclusive with `from`/`to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sec: Option<i64>,
    /// Include per-station audit readings on every point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
}

/// Response for `GET /live_data/weather/{city}`. Added 2026-08-20.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWeatherIndexResponse {
    /// Index city ID (e.g. `miami`).
    pub city: String,
    /// Index configuration version of the newest returned point (e.g.
    /// `miami-temperature-v1.0`). Empty when no points matched the window.
    #[serde(default)]
    pub config_version: Option<String>,
    /// Always `fahrenheit`.
    pub units: String,
    /// Minute-resolution index series. Minutes where the index quorum failed
    /// are never returned, so gaps in the series are real gaps.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub timeseries: Vec<WeatherIndexPoint>,
}

/// One minute of the Kalshi city temperature index.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexPoint {
    /// Event minute, unix milliseconds UTC.
    pub t: i64,
    /// Published index value, Fahrenheit rounded to 0.01. `None` on
    /// `incomplete` points, which have no canonical value yet (the field is
    /// absent, not `0`).
    #[serde(default)]
    pub v: Option<f64>,
    /// `normal`, `degraded`, or (with `detailed=true`) `incomplete`.
    pub status: String,
    /// Number of accepted member stations backing the point. `None` on
    /// `incomplete` points.
    #[serde(default)]
    pub contributors: Option<u32>,
    /// Per-station audit readings, only populated with `detailed=true`.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub stations: Vec<WeatherIndexStationReading>,
}

/// A single member station's reading and quality-control disposition, before
/// incorporation into the index.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeatherIndexStationReading {
    /// Member station (e.g. `KMIA1M`) or its official fallback ID.
    pub station_id: String,
    /// Disposition: `ok`, `missing`, `late`, a QC rejection (`range`,
    /// `rate_spatial`, `extreme`), or `pending`.
    pub code: String,
    /// `hf_asos` (exact-minute primary) or `metar` (carried-forward official
    /// observation). `None` when no reading was available.
    #[serde(default)]
    pub source: Option<String>,
    /// Raw reported temperature in Fahrenheit (unrounded). `None` for
    /// `missing` members.
    #[serde(default)]
    pub temp_f: Option<f64>,
    /// Observation time for carried-forward fallbacks. `None` for
    /// exact-minute primaries.
    #[serde(default)]
    pub obs_time_ms: Option<i64>,
    /// Local wire-receipt time backing the eligibility deadline.
    #[serde(default)]
    pub received_at_ms: Option<i64>,
    /// Why the primary observation was passed over when a fallback was used.
    #[serde(default)]
    pub primary_code: Option<String>,
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
    /// Filter to milestones whose metadata was updated after this Unix
    /// timestamp (in seconds). Useful for polling only what changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
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

    /// Get live data for up to 100 milestones in one request.
    pub async fn get_live_data_batch(
        &self,
        params: GetLiveDatasParams,
    ) -> Result<GetLiveDatasResponse, KalshiError> {
        let path = Self::full_path("/live_data/batch");
        let query = params.query_pairs();
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, false)
            .await
    }

    /// Get live data for an event by its event ticker.
    ///
    /// Serves event-keyed live data such as crypto price charts, commodity
    /// price timeseries, and weather observations. The response's
    /// [`EventLiveData::live_data_type`] names the schema of
    /// [`EventLiveData::details`].
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

    /// Get the Kalshi-computed city temperature index for `city` (e.g. `miami`).
    ///
    /// The canonical minute-resolution series behind hourly temperature
    /// markets. Values are Fahrenheit rounded to 0.01; minutes where the index
    /// quorum failed are omitted entirely, so gaps are real gaps.
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
