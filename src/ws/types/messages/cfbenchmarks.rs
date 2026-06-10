use serde::Deserialize;
use std::borrow::Cow;

/// Windowed-average metadata bundled with each CF Benchmarks value tick.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksAvgData {
    /// Average value over the window, formatted to 8 decimal places.
    pub value: String,
    /// Number of ticks counted in the window.
    pub window_size: i64,
    /// Window start boundary (unix ms).
    pub window_start_ts_ms: i64,
    /// Window end boundary, exclusive (unix ms).
    pub window_end_ts_exclusive: i64,
}

/// Message payload for the `cfbenchmarks_value` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValue {
    /// CF Benchmarks index ID (e.g. `"BRTI"`).
    pub index_id: String,
    /// When Kalshi received the upstream frame (unix ms).
    pub received_at: i64,
    /// The raw CF Benchmarks JSON frame, as a string.
    pub data: String,
    /// Trailing 60-second average metadata.
    pub avg_60s_data: WsCfBenchmarksAvgData,
    /// Present only during the final minute before quarter-hour close.
    #[serde(default)]
    pub last_60s_windowed_average_15min: Option<WsCfBenchmarksAvgData>,
}

/// Borrowed version of [`WsCfBenchmarksValue`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValueRef<'a> {
    #[serde(borrow)]
    pub index_id: Cow<'a, str>,
    pub received_at: i64,
    #[serde(borrow)]
    pub data: Cow<'a, str>,
    pub avg_60s_data: WsCfBenchmarksAvgData,
    #[serde(default)]
    pub last_60s_windowed_average_15min: Option<WsCfBenchmarksAvgData>,
}

impl<'a> WsCfBenchmarksValueRef<'a> {
    pub fn into_owned(self) -> WsCfBenchmarksValue {
        WsCfBenchmarksValue {
            index_id: self.index_id.into_owned(),
            received_at: self.received_at,
            data: self.data.into_owned(),
            avg_60s_data: self.avg_60s_data,
            last_60s_windowed_average_15min: self.last_60s_windowed_average_15min,
        }
    }
}

/// Response to the `indexlist` action on a `cfbenchmarks_value` subscription.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksIndexList {
    pub index_ids: Vec<String>,
}

/// Borrowed version of [`WsCfBenchmarksIndexList`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksIndexListRef<'a> {
    #[serde(borrow)]
    pub index_ids: Vec<Cow<'a, str>>,
}

impl<'a> WsCfBenchmarksIndexListRef<'a> {
    pub fn into_owned(self) -> WsCfBenchmarksIndexList {
        WsCfBenchmarksIndexList {
            index_ids: self.index_ids.into_iter().map(Cow::into_owned).collect(),
        }
    }
}
