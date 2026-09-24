use super::channel::WsChannelV2;
use super::envelope::{
    WsDataMessageRef, WsDataMessageV2, WsError, WsErrorRef, WsListSubscriptions,
    WsListSubscriptionsRef, WsMessageRef, WsMessageV2,
};
use super::messages::*;
use super::subscription::{WsSubscriptionInfo, WsSubscriptionInfoRef};
use serde::Deserialize;
use serde_json::Value;
use serde_json::value::RawValue;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(super) enum WsWireMessage {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsg>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
    },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<Value>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default)]
        subscriptions: Vec<WsSubscriptionInfo>,
        #[serde(default)]
        msg: Option<WsListSubscriptions>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<WsError>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTicker,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTrade,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookSnapshot,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookDelta,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsFill,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketPosition,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsEventLifecycle,
    },
    #[serde(rename = "event_fee_update")]
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsEventFeeUpdate,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsRfqCreated,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsRfqDeleted,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteCreated,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteAccepted,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteExecuted,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderGroupUpdate,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
    #[serde(rename = "cfbenchmarks_value")]
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCfBenchmarksValue,
    },
    #[serde(rename = "cfbenchmarks_value_indexlist")]
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCfBenchmarksIndexList,
    },
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfbenchmarksValue5Hz {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCfBenchmarksValue5Hz,
    },
    #[serde(rename = "cfbenchmarks_value_5hz_indexlist")]
    CfbenchmarksValue5HzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCfBenchmarks5HzIndexList,
    },
    #[serde(rename = "pyth_value")]
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsPythValue,
    },
    #[serde(rename = "pyth_value_underlying_list")]
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsPythUnderlyingList,
    },
}

#[derive(Debug, Deserialize)]
pub(super) struct WsSubscribedMsg {
    #[allow(dead_code)]
    pub channel: Option<WsChannelV2>,
    pub sid: Option<u64>,
}

impl WsWireMessage {
    pub(super) fn into_message(self) -> WsMessageV2 {
        match self {
            WsWireMessage::Subscribed { id, sid, seq, msg } => WsMessageV2::Subscribed {
                id,
                sid: sid.or_else(|| msg.and_then(|value| value.sid)),
                seq,
            },
            WsWireMessage::Unsubscribed { id, sid, seq } => {
                WsMessageV2::Unsubscribed { id, sid, seq }
            }
            WsWireMessage::Ok { id, sid, seq, msg } => {
                if let Some(msg) = msg
                    && let Ok(subscriptions) =
                        serde_json::from_value::<Vec<WsSubscriptionInfo>>(msg)
                {
                    return WsMessageV2::ListSubscriptions {
                        id,
                        sid,
                        seq,
                        subscriptions,
                    };
                }
                WsMessageV2::Ok { id, sid, seq }
            }
            WsWireMessage::ListSubscriptions {
                id,
                sid,
                seq,
                subscriptions,
                msg,
            } => {
                let subs = msg
                    .map(|value| value.subscriptions)
                    .unwrap_or(subscriptions);
                WsMessageV2::ListSubscriptions {
                    id,
                    sid,
                    seq,
                    subscriptions: subs,
                }
            }
            WsWireMessage::Error { id, sid, seq, msg } => WsMessageV2::Error {
                id,
                sid,
                seq,
                error: msg.unwrap_or(WsError {
                    code: None,
                    message: None,
                }),
            },
            WsWireMessage::Ticker { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Ticker { sid, seq, msg })
            }
            WsWireMessage::Trade { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Trade { sid, seq, msg })
            }
            WsWireMessage::OrderbookSnapshot { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { sid, seq, msg })
            }
            WsWireMessage::OrderbookDelta { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { sid, seq, msg })
            }
            WsWireMessage::Fill { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Fill { sid, seq, msg })
            }
            WsWireMessage::MarketPosition { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MarketPosition { sid, seq, msg })
            }
            WsWireMessage::MarketLifecycleV2 { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 { sid, seq, msg })
            }
            WsWireMessage::MultivariateMarketLifecycle { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MultivariateMarketLifecycle { sid, seq, msg })
            }
            WsWireMessage::EventLifecycle { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::EventLifecycle { sid, seq, msg })
            }
            WsWireMessage::EventFeeUpdate { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::EventFeeUpdate { sid, seq, msg })
            }
            WsWireMessage::RfqCreated { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::RfqCreated(msg),
                })
            }
            WsWireMessage::RfqDeleted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::RfqDeleted(msg),
                })
            }
            WsWireMessage::QuoteCreated { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteCreated(msg),
                })
            }
            WsWireMessage::QuoteAccepted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteAccepted(msg),
                })
            }
            WsWireMessage::QuoteExecuted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteExecuted(msg),
                })
            }
            WsWireMessage::OrderGroupUpdates { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates { sid, seq, msg })
            }
            WsWireMessage::UserOrder { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::UserOrder { sid, seq, msg })
            }
            WsWireMessage::CfbenchmarksValue { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue { sid, seq, msg })
            }
            WsWireMessage::CfbenchmarksValueIndexlist { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValueIndexlist { sid, seq, msg })
            }
            WsWireMessage::CfbenchmarksValue5Hz { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5Hz { sid, seq, msg })
            }
            WsWireMessage::CfbenchmarksValue5HzIndexlist { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5HzIndexlist { sid, seq, msg })
            }
            WsWireMessage::PythValue { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::PythValue { sid, seq, msg })
            }
            WsWireMessage::PythValueUnderlyingList { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::PythValueUnderlyingList { sid, seq, msg })
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(super) enum WsWireMessageRef<'a> {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsgRef>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
    },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default, borrow)]
        msg: Option<&'a RawValue>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default, borrow)]
        subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
        #[serde(default, borrow)]
        msg: Option<WsListSubscriptionsRef<'a>>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(default, borrow)]
        msg: Option<WsErrorRef<'a>>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsTickerRef<'a>,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsTradeRef<'a>,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderbookSnapshotRef<'a>,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderbookDeltaRef<'a>,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsFillRef<'a>,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketPositionRef<'a>,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsEventLifecycleRef<'a>,
    },
    #[serde(rename = "event_fee_update")]
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsEventFeeUpdateRef<'a>,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsRfqCreatedRef<'a>,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsRfqDeletedRef<'a>,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteCreatedRef<'a>,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteAcceptedRef<'a>,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteExecutedRef<'a>,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderGroupUpdateRef<'a>,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
    #[serde(rename = "cfbenchmarks_value")]
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksValueRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_indexlist")]
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksIndexListRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfbenchmarksValue5Hz {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksValue5HzRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz_indexlist")]
    CfbenchmarksValue5HzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsCfBenchmarks5HzIndexListRef<'a>,
    },
    #[serde(rename = "pyth_value")]
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsPythValueRef<'a>,
    },
    #[serde(rename = "pyth_value_underlying_list")]
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsPythUnderlyingListRef<'a>,
    },
}

#[derive(Debug, Deserialize)]
pub(super) struct WsSubscribedMsgRef {
    #[allow(dead_code)]
    pub channel: Option<WsChannelV2>,
    #[serde(default)]
    pub sid: Option<u64>,
}

impl<'a> WsWireMessageRef<'a> {
    pub(super) fn into_message(self) -> WsMessageRef<'a> {
        match self {
            WsWireMessageRef::Subscribed { id, sid, seq, msg } => WsMessageRef::Subscribed {
                id,
                sid: sid.or_else(|| msg.and_then(|value| value.sid)),
                seq,
            },
            WsWireMessageRef::Unsubscribed { id, sid, seq } => {
                WsMessageRef::Unsubscribed { id, sid, seq }
            }
            WsWireMessageRef::Ok { id, sid, seq, msg } => {
                if let Some(raw) = msg
                    && let Ok(subscriptions) =
                        serde_json::from_str::<Vec<WsSubscriptionInfoRef<'a>>>(raw.get())
                {
                    return WsMessageRef::ListSubscriptions {
                        id,
                        sid,
                        seq,
                        subscriptions,
                    };
                }
                WsMessageRef::Ok { id, sid, seq }
            }
            WsWireMessageRef::ListSubscriptions {
                id,
                sid,
                seq,
                subscriptions,
                msg,
            } => {
                let subs = msg
                    .map(|value| value.subscriptions)
                    .unwrap_or(subscriptions);
                WsMessageRef::ListSubscriptions {
                    id,
                    sid,
                    seq,
                    subscriptions: subs,
                }
            }
            WsWireMessageRef::Error { id, sid, seq, msg } => WsMessageRef::Error {
                id,
                sid,
                seq,
                error: msg.unwrap_or(WsErrorRef {
                    code: None,
                    message: None,
                }),
            },
            WsWireMessageRef::Ticker { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Ticker { sid, seq, msg })
            }
            WsWireMessageRef::Trade { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Trade { sid, seq, msg })
            }
            WsWireMessageRef::OrderbookSnapshot { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderbookSnapshot { sid, seq, msg })
            }
            WsWireMessageRef::OrderbookDelta { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderbookDelta { sid, seq, msg })
            }
            WsWireMessageRef::Fill { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Fill { sid, seq, msg })
            }
            WsWireMessageRef::MarketPosition { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MarketPosition { sid, seq, msg })
            }
            WsWireMessageRef::MarketLifecycleV2 { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MarketLifecycleV2 { sid, seq, msg })
            }
            WsWireMessageRef::MultivariateMarketLifecycle { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MultivariateMarketLifecycle { sid, seq, msg })
            }
            WsWireMessageRef::EventLifecycle { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::EventLifecycle { sid, seq, msg })
            }
            WsWireMessageRef::EventFeeUpdate { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::EventFeeUpdate { sid, seq, msg })
            }
            WsWireMessageRef::RfqCreated { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::RfqCreated(msg),
                })
            }
            WsWireMessageRef::RfqDeleted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::RfqDeleted(msg),
                })
            }
            WsWireMessageRef::QuoteCreated { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteCreated(msg),
                })
            }
            WsWireMessageRef::QuoteAccepted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteAccepted(msg),
                })
            }
            WsWireMessageRef::QuoteExecuted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteExecuted(msg),
                })
            }
            WsWireMessageRef::OrderGroupUpdates { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderGroupUpdates { sid, seq, msg })
            }
            WsWireMessageRef::UserOrder { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::UserOrder { sid, seq, msg })
            }
            WsWireMessageRef::CfbenchmarksValue { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue { sid, seq, msg })
            }
            WsWireMessageRef::CfbenchmarksValueIndexlist { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValueIndexlist { sid, seq, msg })
            }
            WsWireMessageRef::CfbenchmarksValue5Hz { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5Hz { sid, seq, msg })
            }
            WsWireMessageRef::CfbenchmarksValue5HzIndexlist { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5HzIndexlist {
                    sid,
                    seq,
                    msg,
                })
            }
            WsWireMessageRef::PythValue { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::PythValue { sid, seq, msg })
            }
            WsWireMessageRef::PythValueUnderlyingList { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::PythValueUnderlyingList { sid, seq, msg })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Payloads below are taken verbatim (field-for-field) from the AsyncAPI
    // spec's `pythValue` / `pythUnderlyingList` / `cfbenchmarksValue5Hz` /
    // `cfbenchmarks5HzIndexList` message examples.

    const PYTH_VALUE_JSON: &str = r#"{
        "type":"pyth_value",
        "sid":1,
        "seq":42,
        "msg":{
            "underlying_ticker":"Commodities.Index.PYTHOIL/USD",
            "value_usd":"82.12345000",
            "source_ts_ms":1710000000100,
            "received_at":1710000000123
        }
    }"#;

    const PYTH_UNDERLYING_LIST_JSON: &str = r#"{
        "type":"pyth_value_underlying_list",
        "id":2,
        "sid":1,
        "seq":1,
        "msg":{
            "underlying_tickers":["Commodities.Index.NATGAS/USD","Commodities.Index.PYTHOIL/USD","Metal.XAG/USD","Metal.XAU/USD"]
        }
    }"#;

    const CFBENCHMARKS_VALUE_5HZ_JSON: &str = r#"{
        "type":"cfbenchmarks_value_5hz",
        "sid":1,
        "seq":42,
        "msg":{
            "index_id":"BRTI",
            "value_usd":"68000.12000000",
            "source_ts_ms":1710000000323,
            "received_at":1710000000341,
            "data":"{\"type\":\"value\",\"id\":\"BRTI\",\"time\":1710000000323,\"value\":\"68000.12\"}"
        }
    }"#;

    const CFBENCHMARKS_VALUE_5HZ_INDEXLIST_JSON: &str = r#"{
        "type":"cfbenchmarks_value_5hz_indexlist",
        "id":2,
        "sid":1,
        "seq":1,
        "msg":{
            "index_ids":["BRTI","ETHUSD_RTI"]
        }
    }"#;

    #[test]
    fn wire_pyth_value_owned_parses() {
        let wire: WsWireMessage = serde_json::from_str(PYTH_VALUE_JSON).unwrap();
        match wire.into_message() {
            WsMessageV2::Data(WsDataMessageV2::PythValue { sid, seq, msg }) => {
                assert_eq!(sid, Some(1));
                assert_eq!(seq, Some(42));
                assert_eq!(msg.underlying_ticker, "Commodities.Index.PYTHOIL/USD");
                assert_eq!(msg.value_usd, "82.12345000");
                assert_eq!(msg.source_ts_ms, 1710000000100);
                assert_eq!(msg.received_at, 1710000000123);
            }
            other => panic!("expected pyth_value data message, got {other:?}"),
        }
    }

    #[test]
    fn wire_pyth_value_ref_parses_and_round_trips_owned() {
        let wire: WsWireMessageRef<'_> = serde_json::from_str(PYTH_VALUE_JSON).unwrap();
        match wire.into_message() {
            WsMessageRef::Data(WsDataMessageRef::PythValue { sid, seq, msg }) => {
                assert_eq!(sid, Some(1));
                assert_eq!(seq, Some(42));
                assert_eq!(msg.underlying_ticker, "Commodities.Index.PYTHOIL/USD");
                let owned = msg.into_owned();
                assert_eq!(owned.underlying_ticker, "Commodities.Index.PYTHOIL/USD");
                assert_eq!(owned.value_usd, "82.12345000");
            }
            other => panic!("expected borrowed pyth_value data message, got {other:?}"),
        }
    }

    #[test]
    fn wire_pyth_underlying_list_owned_parses() {
        let wire: WsWireMessage = serde_json::from_str(PYTH_UNDERLYING_LIST_JSON).unwrap();
        match wire.into_message() {
            WsMessageV2::Data(WsDataMessageV2::PythValueUnderlyingList { sid, seq, msg }) => {
                assert_eq!(sid, Some(1));
                assert_eq!(seq, Some(1));
                assert_eq!(
                    msg.underlying_tickers,
                    vec![
                        "Commodities.Index.NATGAS/USD".to_string(),
                        "Commodities.Index.PYTHOIL/USD".to_string(),
                        "Metal.XAG/USD".to_string(),
                        "Metal.XAU/USD".to_string(),
                    ]
                );
            }
            other => panic!("expected pyth_value_underlying_list data message, got {other:?}"),
        }
    }

    #[test]
    fn wire_pyth_underlying_list_ref_parses() {
        let wire: WsWireMessageRef<'_> =
            serde_json::from_str(PYTH_UNDERLYING_LIST_JSON).unwrap();
        match wire.into_message() {
            WsMessageRef::Data(WsDataMessageRef::PythValueUnderlyingList { msg, .. }) => {
                assert_eq!(msg.underlying_tickers.len(), 4);
                let owned = msg.into_owned();
                assert_eq!(owned.underlying_tickers[0], "Commodities.Index.NATGAS/USD");
            }
            other => panic!(
                "expected borrowed pyth_value_underlying_list data message, got {other:?}"
            ),
        }
    }

    #[test]
    fn wire_cfbenchmarks_value_5hz_owned_parses() {
        let wire: WsWireMessage = serde_json::from_str(CFBENCHMARKS_VALUE_5HZ_JSON).unwrap();
        match wire.into_message() {
            WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5Hz { sid, seq, msg }) => {
                assert_eq!(sid, Some(1));
                assert_eq!(seq, Some(42));
                assert_eq!(msg.index_id, "BRTI");
                assert_eq!(msg.value_usd, "68000.12000000");
                assert_eq!(msg.source_ts_ms, 1710000000323);
                assert_eq!(msg.received_at, 1710000000341);
                assert!(msg.data.contains("\"id\":\"BRTI\""));
            }
            other => panic!("expected cfbenchmarks_value_5hz data message, got {other:?}"),
        }
    }

    #[test]
    fn wire_cfbenchmarks_value_5hz_ref_parses_and_round_trips_owned() {
        let wire: WsWireMessageRef<'_> =
            serde_json::from_str(CFBENCHMARKS_VALUE_5HZ_JSON).unwrap();
        match wire.into_message() {
            WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5Hz { msg, .. }) => {
                assert_eq!(msg.index_id, "BRTI");
                let owned = msg.into_owned();
                assert_eq!(owned.index_id, "BRTI");
                assert_eq!(owned.value_usd, "68000.12000000");
            }
            other => panic!(
                "expected borrowed cfbenchmarks_value_5hz data message, got {other:?}"
            ),
        }
    }

    #[test]
    fn wire_cfbenchmarks_value_5hz_indexlist_owned_parses() {
        let wire: WsWireMessage =
            serde_json::from_str(CFBENCHMARKS_VALUE_5HZ_INDEXLIST_JSON).unwrap();
        match wire.into_message() {
            WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5HzIndexlist {
                sid,
                seq,
                msg,
            }) => {
                assert_eq!(sid, Some(1));
                assert_eq!(seq, Some(1));
                assert_eq!(
                    msg.index_ids,
                    vec!["BRTI".to_string(), "ETHUSD_RTI".to_string()]
                );
            }
            other => panic!(
                "expected cfbenchmarks_value_5hz_indexlist data message, got {other:?}"
            ),
        }
    }

    #[test]
    fn wire_cfbenchmarks_value_5hz_indexlist_ref_parses() {
        let wire: WsWireMessageRef<'_> =
            serde_json::from_str(CFBENCHMARKS_VALUE_5HZ_INDEXLIST_JSON).unwrap();
        match wire.into_message() {
            WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5HzIndexlist { msg, .. }) => {
                assert_eq!(msg.index_ids.len(), 2);
                let owned = msg.into_owned();
                assert_eq!(owned.index_ids[1], "ETHUSD_RTI");
            }
            other => panic!(
                "expected borrowed cfbenchmarks_value_5hz_indexlist data message, got {other:?}"
            ),
        }
    }
}
