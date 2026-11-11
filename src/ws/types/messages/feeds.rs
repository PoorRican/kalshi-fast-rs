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

/// Payload for `ebattle_scores`: the full current score state of one match.
///
/// `data` is the JSON-encoded score state exactly as published (`game`,
/// `status`, and whichever score/clock fields are available). Decode it with
/// `serde_json` when needed; the crate keeps it raw so the hot path does not
/// pay for a second parse. `received_at` is Kalshi's publication timestamp in
/// milliseconds, not a vendor timestamp.
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScores {
    pub event_id: String,
    /// Kalshi milestone ID.
    pub match_id: String,
    pub received_at: i64,
    pub data: String,
}

/// Borrowed version of [`WsEbattleScores`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScoresRef<'a> {
    #[serde(borrow)]
    pub event_id: Cow<'a, str>,
    #[serde(borrow)]
    pub match_id: Cow<'a, str>,
    pub received_at: i64,
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

impl<'a> WsEbattleScoresRef<'a> {
    pub fn into_owned(self) -> WsEbattleScores {
        WsEbattleScores {
            event_id: self.event_id.into_owned(),
            match_id: self.match_id.into_owned(),
            received_at: self.received_at,
            data: self.data.into_owned(),
        }
    }
}

/// Payload for `ebattle_scores_matchlist`: match IDs seen within the past two
/// hours. This is not a complete fixture catalog.
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
