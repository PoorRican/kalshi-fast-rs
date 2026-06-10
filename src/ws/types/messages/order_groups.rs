use super::FixedPointCountRef;
use crate::types::FixedPointCount;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsOrderGroupEventType {
    Created,
    Triggered,
    Reset,
    Deleted,
    LimitUpdated,
    #[serde(other)]
    Unknown,
}

/// Order group update message payload (type: "order_group_updates")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderGroupUpdate {
    pub event_type: WsOrderGroupEventType,
    pub order_group_id: String,
    #[serde(default)]
    pub contracts_limit_fp: Option<FixedPointCount>,
    /// Matching-engine timestamp (Unix epoch milliseconds) at which the event
    /// was processed. Spec marks this required; kept optional for parse safety.
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

/// Order group update message payload (type: "order_group_updates")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderGroupUpdateRef<'a> {
    pub event_type: WsOrderGroupEventType,
    #[serde(borrow)]
    pub order_group_id: Cow<'a, str>,
    #[serde(default, borrow)]
    pub contracts_limit_fp: Option<FixedPointCountRef<'a>>,
    /// Matching-engine timestamp (Unix epoch milliseconds) at which the event
    /// was processed. Spec marks this required; kept optional for parse safety.
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

impl<'a> WsOrderGroupUpdateRef<'a> {
    pub fn into_owned(self) -> WsOrderGroupUpdate {
        WsOrderGroupUpdate {
            event_type: self.event_type,
            order_group_id: self.order_group_id.into_owned(),
            contracts_limit_fp: self.contracts_limit_fp.map(Cow::into_owned),
            ts_ms: self.ts_ms,
        }
    }
}
