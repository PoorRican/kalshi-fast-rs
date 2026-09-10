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

/// Message payload for the `cfbenchmarks_value_5hz` WebSocket channel: a lean
/// raw tick (no 60-second/quarter-hour averages) for the smaller 5Hz coin set
/// (BRTI, ETHUSD_RTI, SOLUSD_RTI, XRPUSD_RTI, DOGEUSD_RTI). Added 2026-09-03.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarks5HzValue {
    /// CF Benchmarks index ID (e.g. `"BRTI"`).
    pub index_id: String,
    /// Index value in USD, formatted with exactly 8 decimal places.
    pub value_usd: String,
    /// Upstream publication timestamp of the tick (unix ms).
    pub source_ts_ms: i64,
    /// When Kalshi received the upstream frame (unix ms).
    pub received_at: i64,
    /// The raw CF Benchmarks JSON frame, as a string.
    pub data: String,
}

/// Borrowed version of [`WsCfBenchmarks5HzValue`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarks5HzValueRef<'a> {
    #[serde(borrow)]
    pub index_id: Cow<'a, str>,
    #[serde(borrow)]
    pub value_usd: Cow<'a, str>,
    pub source_ts_ms: i64,
    pub received_at: i64,
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

impl<'a> WsCfBenchmarks5HzValueRef<'a> {
    pub fn into_owned(self) -> WsCfBenchmarks5HzValue {
        WsCfBenchmarks5HzValue {
            index_id: self.index_id.into_owned(),
            value_usd: self.value_usd.into_owned(),
            source_ts_ms: self.source_ts_ms,
            received_at: self.received_at,
            data: self.data.into_owned(),
        }
    }
}

/// Response to the `indexlist` action on a `cfbenchmarks_value` subscription.
///
/// Also reused for the `indexlist` action on a `cfbenchmarks_value_5hz`
/// subscription (`cfbenchmarks_value_5hz_indexlist` message) — both channels
/// respond with an identical `{ index_ids: [...] }` shape.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The 5Hz tick is a lean raw tick: no `avg_60s_data` or windowed-average
    /// fields, but it does carry `value_usd` and `source_ts_ms` which the
    /// once-per-second payload does not.
    #[test]
    fn ws_cfbenchmarks_5hz_value_parses_owned_and_borrowed() {
        let json = r#"{
            "index_id": "BRTI",
            "value_usd": "68000.12000000",
            "source_ts_ms": 1710000000323,
            "received_at": 1710000000341,
            "data": "{\"type\":\"value\",\"id\":\"BRTI\",\"time\":1710000000323,\"value\":\"68000.12\"}"
        }"#;

        let owned: WsCfBenchmarks5HzValue = serde_json::from_str(json).unwrap();
        assert_eq!(owned.index_id, "BRTI");
        assert_eq!(owned.value_usd, "68000.12000000");
        assert_eq!(owned.source_ts_ms, 1710000000323);
        assert_eq!(owned.received_at, 1710000000341);

        let borrowed: WsCfBenchmarks5HzValueRef = serde_json::from_str(json).unwrap();
        let round_tripped = borrowed.into_owned();
        assert_eq!(round_tripped.index_id, owned.index_id);
        assert_eq!(round_tripped.value_usd, owned.value_usd);
        assert_eq!(round_tripped.data, owned.data);
    }
}
