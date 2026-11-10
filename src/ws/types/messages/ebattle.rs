use serde::Deserialize;
use std::borrow::Cow;

/// Payload for `ebattle_scores`.
///
/// `data` is a JSON-encoded string holding the full current score state; use
/// [`WsEbattleScores::data_json`] to decode it. Fields inside it may be
/// missing when unavailable.
#[derive(Debug, Clone, Deserialize)]
pub struct WsEbattleScores {
    pub event_id: String,
    /// Kalshi milestone ID.
    pub match_id: String,
    /// Publication timestamp in milliseconds (not a vendor timestamp).
    pub received_at: i64,
    pub data: String,
}

impl WsEbattleScores {
    /// Decode the JSON-encoded `data` string.
    pub fn data_json(&self) -> Result<serde_json::Value, serde_json::Error> {
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
    #[serde(borrow)]
    pub data: Cow<'a, str>,
}

impl<'a> WsEbattleScoresRef<'a> {
    /// Decode the JSON-encoded `data` string.
    pub fn data_json(&self) -> Result<serde_json::Value, serde_json::Error> {
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
