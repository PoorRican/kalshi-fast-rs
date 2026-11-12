use serde::Deserialize;
use std::borrow::Cow;

/// Payload for `ebattle_scores`: one match's full score state.
///
/// `data` is a JSON-encoded string carrying `game`, `status` and whichever
/// score/clock fields are available (`home_score`, `away_score`,
/// `home_periods`, `away_periods`, `score_format`, `period_unit`,
/// `clock_ticking`, `duration_seconds`, `current_game_duration_seconds`,
/// `last_updated_ts`, `live_data_delayed`). Kalshi does not publish a typed
/// schema for it, so it is kept verbatim; decode it with [`Self::parse_data`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScores {
    pub event_id: String,
    /// Kalshi milestone ID.
    pub match_id: String,
    /// Kalshi publication timestamp in milliseconds (not a vendor timestamp).
    pub received_at: i64,
    /// JSON-encoded full score state.
    pub data: String,
}

impl WsEbattleScores {
    /// Decode the JSON-encoded `data` score state into a caller-chosen type.
    pub fn parse_data<'de, T: Deserialize<'de>>(&'de self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.data)
    }
}

/// Borrowed version of [`WsEbattleScores`].
///
/// `data` borrows only when the encoded string contains no JSON escapes; the
/// embedded JSON's quotes normally force an owned copy.
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
    /// Decode the JSON-encoded `data` score state into a caller-chosen type.
    pub fn parse_data<'de, T: Deserialize<'de>>(&'de self) -> Result<T, serde_json::Error> {
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

/// Payload for `ebattle_scores_matchlist`: match IDs seen within the past two
/// hours (not a complete fixture catalog).
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleMatchlist {
    pub match_ids: Vec<String>,
}

/// Borrowed version of [`WsEbattleMatchlist`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleMatchlistRef<'a> {
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub match_ids: Vec<Cow<'a, str>>,
}

impl<'a> WsEbattleMatchlistRef<'a> {
    pub fn into_owned(self) -> WsEbattleMatchlist {
        WsEbattleMatchlist {
            match_ids: self.match_ids.into_iter().map(Cow::into_owned).collect(),
        }
    }
}
