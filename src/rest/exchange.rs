//! Exchange status, schedule, and fee changes.
//!
//! Public endpoints expose exchange operational state and schedules.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::{FeeType, deserialize_null_as_empty_vec};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetExchangeStatusResponse {
    pub exchange_active: bool,
    pub trading_active: bool,
    #[serde(default)]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(default)]
    pub intra_exchange_transfers_active: Option<bool>,
    /// Absent when a per-shard breakdown is unavailable.
    #[serde(default)]
    pub exchange_index_statuses: Option<Vec<ExchangeIndexStatus>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExchangeIndexStatus {
    pub exchange_index: i32,
    pub description: String,
    pub exchange_active: bool,
    pub trading_active: bool,
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
    pub id: String,
    pub series_ticker: String,
    pub fee_type: FeeType,
    pub fee_multiplier: f64,
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
/// Rates are numeric decimal fractions of notional.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFeeTiersResponse {
    /// Market ticker → maker fee rate (decimal fraction of notional).
    pub maker_fee_rates: HashMap<String, f64>,
    /// Market ticker → taker fee rate (decimal fraction of notional).
    pub taker_fee_rates: HashMap<String, f64>,
}

impl KalshiRestClient {
    /// Get the current exchange status (open, closed, etc.).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange_status_handles_nullable_resume_and_optional_shards() {
        let status: GetExchangeStatusResponse = serde_json::from_str(
            r#"{
                "exchange_active": true,
                "trading_active": false,
                "exchange_estimated_resume_time": null,
                "intra_exchange_transfers_active": true,
                "exchange_index_statuses": [{
                    "exchange_index": 2,
                    "description": "shard two",
                    "exchange_active": true,
                    "trading_active": false,
                    "intra_exchange_transfers_active": true
                }]
            }"#,
        )
        .unwrap();
        assert!(status.exchange_estimated_resume_time.is_none());
        assert_eq!(status.intra_exchange_transfers_active, Some(true));
        let shard = &status.exchange_index_statuses.unwrap()[0];
        assert_eq!(shard.exchange_index, 2);
        assert_eq!(shard.description, "shard two");
        assert!(!shard.trading_active);

        let minimal: GetExchangeStatusResponse =
            serde_json::from_str(r#"{"exchange_active":true,"trading_active":true}"#).unwrap();
        assert!(minimal.exchange_index_statuses.is_none());
        assert!(minimal.intra_exchange_transfers_active.is_none());
    }

    #[test]
    fn margin_fee_rates_are_numeric() {
        let response: GetMarginFeeTiersResponse = serde_json::from_str(
            r#"{"maker_fee_rates":{"MARKET":0.0001},"taker_fee_rates":{"MARKET":0.0008}}"#,
        )
        .unwrap();
        assert_eq!(response.maker_fee_rates["MARKET"], 0.0001);
        assert_eq!(response.taker_fee_rates["MARKET"], 0.0008);
    }
}
