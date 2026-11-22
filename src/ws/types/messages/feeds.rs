use serde::Deserialize;
use std::borrow::Cow;

/// Payload for CfBenchmarksValue5Hz.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValue5Hz {
    pub index_id: String,
    pub value_usd: String,
    pub source_ts_ms: i64,
    pub received_at: i64,
    pub data: String,
}

/// Borrowed version of [`WsCfBenchmarksValue5Hz`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValue5HzRef<'a> {
    #[serde(borrow)]
    pub index_id: Cow<'a, str>,
    #[serde(borrow)]
    pub value_usd: Cow<'a, str>,
    pub source_ts_ms: i64,
    pub received_at: i64,
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

impl<'a> WsCfBenchmarksValue5HzRef<'a> {
    pub fn into_owned(self) -> WsCfBenchmarksValue5Hz {
        WsCfBenchmarksValue5Hz {
            index_id: self.index_id.into_owned(),
            value_usd: self.value_usd.into_owned(),
            source_ts_ms: self.source_ts_ms,
            received_at: self.received_at,
            data: self.data.into_owned(),
        }
    }
}

/// Payload for CfBenchmarksValue5HzIndexList.
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValue5HzIndexList {
    pub index_ids: Vec<String>,
}

/// Borrowed version of [`WsCfBenchmarksValue5HzIndexList`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsCfBenchmarksValue5HzIndexListRef<'a> {
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub index_ids: Vec<Cow<'a, str>>,
}

impl<'a> WsCfBenchmarksValue5HzIndexListRef<'a> {
    pub fn into_owned(self) -> WsCfBenchmarksValue5HzIndexList {
        WsCfBenchmarksValue5HzIndexList {
            index_ids: self.index_ids.into_iter().map(Cow::into_owned).collect(),
        }
    }
}

/// Payload for PythValue.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythValue {
    pub underlying_ticker: String,
    pub value_usd: String,
    pub source_ts_ms: i64,
    pub received_at: i64,
}

/// Borrowed version of [`WsPythValue`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythValueRef<'a> {
    #[serde(borrow)]
    pub underlying_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub value_usd: Cow<'a, str>,
    pub source_ts_ms: i64,
    pub received_at: i64,
}

impl<'a> WsPythValueRef<'a> {
    pub fn into_owned(self) -> WsPythValue {
        WsPythValue {
            underlying_ticker: self.underlying_ticker.into_owned(),
            value_usd: self.value_usd.into_owned(),
            source_ts_ms: self.source_ts_ms,
            received_at: self.received_at,
        }
    }
}

/// Payload for PythUnderlyingList.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingList {
    pub underlying_tickers: Vec<String>,
}

/// Borrowed version of [`WsPythUnderlyingList`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingListRef<'a> {
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub underlying_tickers: Vec<Cow<'a, str>>,
}

impl<'a> WsPythUnderlyingListRef<'a> {
    pub fn into_owned(self) -> WsPythUnderlyingList {
        WsPythUnderlyingList {
            underlying_tickers: self
                .underlying_tickers
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
        }
    }
}

/// Payload for an `ebattle_scores` update.
///
/// There is no initial snapshot or replay; each update carries the current
/// full score state. `received_at` is Kalshi's publication time in
/// milliseconds, not a vendor timestamp.
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScores {
    pub event_id: String,
    /// Kalshi milestone ID.
    pub match_id: String,
    pub received_at: i64,
    /// JSON-encoded full score state; decode it with [`Self::score_state`].
    pub data: String,
}

impl WsEbattleScores {
    /// Decode `data`. Kalshi documents its keys in prose only (`game`,
    /// `status`, `home_score`, `away_score`, `home_periods`, `away_periods`,
    /// `score_format`, `period_unit`, `clock_ticking`, `duration_seconds`,
    /// `current_game_duration_seconds`, `last_updated_ts`,
    /// `live_data_delayed`); a missing key means unavailable data.
    pub fn score_state(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&self.data)
    }
}

/// Borrowed version of [`WsEbattleScores`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScoresRef<'a> {
    #[serde(borrow)]
    pub event_id: Cow<'a, str>,
    #[serde(borrow)]
    pub match_id: Cow<'a, str>,
    pub received_at: i64,
    /// JSON-encoded score state. Its quotes are escaped on the wire, so this
    /// is normally an owned `Cow`.
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

impl<'a> WsEbattleScoresRef<'a> {
    /// See [`WsEbattleScores::score_state`].
    pub fn score_state(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&self.data)
    }

    pub fn into_owned(self) -> WsEbattleScores {
        WsEbattleScores {
            event_id: self.event_id.into_owned(),
            match_id: self.match_id.into_owned(),
            received_at: self.received_at,
            data: self.data.into_owned(),
        }
    }
}

/// Reply to the `matchlist` action: match IDs seen within the past two hours,
/// not a complete fixture catalog.
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScoresMatchlist {
    pub match_ids: Vec<String>,
}

/// Borrowed version of [`WsEbattleScoresMatchlist`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScoresMatchlistRef<'a> {
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub match_ids: Vec<Cow<'a, str>>,
}

impl<'a> WsEbattleScoresMatchlistRef<'a> {
    pub fn into_owned(self) -> WsEbattleScoresMatchlist {
        WsEbattleScoresMatchlist {
            match_ids: self.match_ids.into_iter().map(Cow::into_owned).collect(),
        }
    }
}
