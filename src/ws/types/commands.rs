use super::subscription::{
    WsSubscriptionParamsV2, WsUnsubscribeParamsV2, WsUpdateSubscriptionParamsV2,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsSubscribeCmd<P = WsSubscriptionParamsV2> {
    pub id: u64,
    pub cmd: &'static str,
    pub params: P,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsUnsubscribeCmd {
    pub id: u64,
    pub cmd: &'static str,
    pub params: WsUnsubscribeParamsV2,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsListSubscriptionsCmd {
    pub id: u64,
    pub cmd: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsUpdateSubscriptionCmd {
    pub id: u64,
    pub cmd: &'static str,
    pub params: WsUpdateSubscriptionParamsV2,
}
