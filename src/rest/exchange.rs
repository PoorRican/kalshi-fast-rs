//! Exchange status and schedule.
//!
//! Public endpoints exposing operational state of the Kalshi exchange: whether
//! trading is active (globally and per exchange index/shard), standard and
//! maintenance hours, and timestamps/fee changes useful for cache invalidation.
//!
//! Note: `GET /exchange/announcements` was removed from the Kalshi API on
//! 2026-07-04. Use [`KalshiRestClient::get_exchange_schedule`] for downtime
//! windows instead.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::{FeeType, deserialize_null_as_empty_vec};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Response for `GET /exchange/status`.
///
/// The top-level `exchange_active` / `trading_active` /
/// `intra_exchange_transfers_active` flags reflect the default exchange index
/// (`0`); [`Self::exchange_index_statuses`] carries the per-shard breakdown
/// (added 2026-07-02, shard `description` added 2026-08-13).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetExchangeStatusResponse {
    pub exchange_active: bool,
    pub trading_active: bool,
    /// True if intra-exchange transfers are currently permitted. `None` on
    /// deployments predating the per-index status fields.
    #[serde(default)]
    pub intra_exchange_transfers_active: Option<bool>,
    #[serde(default)]
    pub exchange_estimated_resume_time: Option<String>,
    /// Status of each exchange index. Empty when the per-index breakdown is
    /// unavailable.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub exchange_index_statuses: Vec<ExchangeIndexStatus>,
}

/// Status of a single exchange index (shard).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExchangeIndexStatus {
    /// Identifier for this exchange shard.
    pub exchange_index: u32,
    /// Human-readable description of this exchange shard.
    pub description: String,
    /// False if this exchange index is no longer taking any state changes.
    pub exchange_active: bool,
    /// True if trading is currently permitted on this exchange index.
    pub trading_active: bool,
    /// True if intra-exchange transfers are permitted on this exchange index.
    pub intra_exchange_transfers_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DailySchedule {
    pub open_time: String,
    pub close_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StandardHours {
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub monday: Vec<DailySchedule>,
    #[serde(default)]
    pub tuesday: Vec<DailySchedule>,
    #[serde(default)]
    pub wednesday: Vec<DailySchedule>,
    #[serde(default)]
    pub thursday: Vec<DailySchedule>,
    #[serde(default)]
    pub friday: Vec<DailySchedule>,
    #[serde(default)]
    pub saturday: Vec<DailySchedule>,
    #[serde(default)]
    pub sunday: Vec<DailySchedule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MaintenanceWindow {
    pub start_datetime: String,
    pub end_datetime: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExchangeSchedule {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub standard_hours: Vec<StandardHours>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub maintenance_windows: Vec<MaintenanceWindow>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetExchangeScheduleResponse {
    pub schedule: ExchangeSchedule,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserDataTimestampResponse {
    pub as_of_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeriesFeeChange {
    /// Unique identifier for this fee change.
    pub id: String,
    /// Series ticker this fee change applies to.
    pub series_ticker: String,
    /// New fee type for the series.
    pub fee_type: FeeType,
    /// New fee multiplier for the series.
    pub fee_multiplier: f64,
    /// RFC3339 timestamp when this fee change is scheduled to take effect.
    pub scheduled_ts: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSeriesFeeChangesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_historical: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeriesFeeChangesResponse {
    #[serde(rename = "series_fee_change_arr")]
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub series_fee_change_arr: Vec<SeriesFeeChange>,
}

/// Response for `GET /margin/fee_tiers`.
///
/// Response was restructured 2026-05-11: the previous `maker_fee_tiers` /
/// `taker_fee_tiers` tier-name maps were replaced by `maker_fee_rates` /
/// `taker_fee_rates`, which map market ticker → decimal fee rate.
/// Fee = `notional * rate` (e.g. `0.0008` = 8 bps).
///
/// Note: this endpoint is documented in the Kalshi changelog but is not present
/// in the published OpenAPI spec, so the rate values are kept as untyped JSON
/// (`serde_json::Value`) to tolerate either numeric or string encodings. Any
/// fields beyond the two rate maps are ignored.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFeeTiersResponse {
    /// Market ticker → maker fee rate (decimal fraction of notional).
    #[serde(default)]
    pub maker_fee_rates: Map<String, Value>,
    /// Market ticker → taker fee rate (decimal fraction of notional).
    #[serde(default)]
    pub taker_fee_rates: Map<String, Value>,
}

impl KalshiRestClient {
    /// Get the current exchange status (open, closed, etc.).
    ///
    /// Includes the per-exchange-index breakdown in
    /// [`GetExchangeStatusResponse::exchange_index_statuses`] when available.
    pub async fn get_exchange_status(&self) -> Result<GetExchangeStatusResponse, KalshiError> {
        let path = Self::full_path("/exchange/status");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get the exchange trading schedule.
    pub async fn get_exchange_schedule(&self) -> Result<GetExchangeScheduleResponse, KalshiError> {
        let path = Self::full_path("/exchange/schedule");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get the timestamp of the latest user-data change (useful for cache invalidation).
    pub async fn get_user_data_timestamp(
        &self,
    ) -> Result<GetUserDataTimestampResponse, KalshiError> {
        let path = Self::full_path("/exchange/user_data_timestamp");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get per-market margin fee rates.
    ///
    /// Returns `maker_fee_rates` and `taker_fee_rates` as market-ticker → decimal maps.
    /// Fee = `notional * rate` (e.g. `0.0008` = 8 bps).
    ///
    /// **Requires auth.**
    pub async fn get_margin_fee_tiers(&self) -> Result<GetMarginFeeTiersResponse, KalshiError> {
        let path = Self::full_path("/margin/fee_tiers");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// List fee changes for a series.
    pub async fn get_series_fee_changes(
        &self,
        params: GetSeriesFeeChangesParams,
    ) -> Result<GetSeriesFeeChangesResponse, KalshiError> {
        let path = Self::full_path("/series/fee_changes");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }
}
