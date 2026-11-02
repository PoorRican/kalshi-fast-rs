use super::channel::WsChannelV2;
use super::envelope::{
    WsDataMessageRef, WsDataMessageV2, WsError, WsErrorRef, WsListSubscriptions,
    WsListSubscriptionsRef, WsMessageRef, WsMessageV2, WsOkPayload, WsOkPayloadRef,
};
use super::messages::*;
use super::subscription::{WsSubscriptionInfo, WsSubscriptionInfoRef};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum WsOkWire {
    Subscriptions(Vec<WsSubscriptionInfo>),
    Filters(WsOkPayload),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum WsOkWireRef<'a> {
    Subscriptions(#[serde(borrow)] Vec<WsSubscriptionInfoRef<'a>>),
    Filters(#[serde(borrow)] WsOkPayloadRef<'a>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(super) enum WsWireMessage {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsg>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(default)]
        msg: Option<WsOkWire>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
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
        sending_ts_ms: Option<i64>,
        #[serde(default)]
        msg: Option<WsError>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTicker,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTrade,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookSnapshot,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookDelta,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsFill,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketPosition,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventLifecycle,
    },
    #[serde(rename = "event_fee_update")]
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventFeeUpdate,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqCreated,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqDeleted,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsQuoteCreated,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsQuoteAccepted,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsQuoteExecuted,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderGroupUpdate,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsUserOrder,
    },
    #[serde(rename = "cfbenchmarks_value")]
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue,
    },
    #[serde(rename = "cfbenchmarks_value_indexlist")]
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksIndexList,
    },
    #[serde(rename = "rfq_batch_created")]
    RfqBatchCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqBatchCreated,
    },
    #[serde(rename = "rfq_batch_deleted")]
    RfqBatchDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqBatchDeleted,
    },
    #[serde(rename = "batch_quote_created")]
    BatchQuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsBatchQuoteCreated,
    },
    #[serde(rename = "rfq_batch_best_quotes")]
    RfqBatchBestQuotes {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqBatchBestQuotes,
    },
    #[serde(rename = "batch_quote_accepted")]
    BatchQuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsBatchQuoteAccepted,
    },
    #[serde(rename = "rfq_batch_executed")]
    RfqBatchExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsRfqBatchExecuted,
    },
    #[serde(rename = "batch_quote_executed")]
    BatchQuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsBatchQuoteExecuted,
    },
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfbenchmarksValue5hz {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5Hz,
    },
    #[serde(rename = "cfbenchmarks_value_5hz_indexlist")]
    CfbenchmarksValue5hzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5HzIndexList,
    },
    #[serde(rename = "pyth_value")]
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythValue,
    },
    #[serde(rename = "pyth_value_underlying_list")]
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythUnderlyingList,
    },
}

#[derive(Debug, Deserialize)]
pub(super) struct WsSubscribedMsg {
    pub channel: Option<WsChannelV2>,
    pub sid: Option<u64>,
}

impl WsWireMessage {
    pub(super) fn into_message(self) -> WsMessageV2 {
        match self {
            WsWireMessage::Subscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Subscribed {
                id,
                sid: sid.or_else(|| msg.as_ref().and_then(|value| value.sid)),
                channel: msg.and_then(|value| value.channel),
                seq,
                sending_ts_ms,
            },
            WsWireMessage::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            } => WsMessageV2::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            },
            WsWireMessage::Ok {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => match msg {
                Some(WsOkWire::Subscriptions(subscriptions)) => WsMessageV2::ListSubscriptions {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    subscriptions,
                },
                Some(WsOkWire::Filters(msg)) => WsMessageV2::Ok {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: Some(msg),
                },
                None => WsMessageV2::Ok {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: None,
                },
            },
            WsWireMessage::ListSubscriptions {
                id,
                sid,
                seq,
                sending_ts_ms,
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
                    sending_ts_ms,
                    subscriptions: subs,
                }
            }
            WsWireMessage::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                error: msg.unwrap_or(WsError::default()),
            },
            WsWireMessage::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::RfqCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqCreated(msg),
            }),
            WsWireMessage::RfqDeleted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqDeleted(msg),
            }),
            WsWireMessage::QuoteCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteCreated(msg),
            }),
            WsWireMessage::QuoteAccepted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteAccepted(msg),
            }),
            WsWireMessage::QuoteExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteExecuted(msg),
            }),
            WsWireMessage::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::RfqBatchCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchCreated(msg),
            }),
            WsWireMessage::RfqBatchDeleted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchDeleted(msg),
            }),
            WsWireMessage::BatchQuoteCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::BatchQuoteCreated(msg),
            }),
            WsWireMessage::RfqBatchBestQuotes {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchBestQuotes(msg),
            }),
            WsWireMessage::BatchQuoteAccepted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::BatchQuoteAccepted(msg),
            }),
            WsWireMessage::RfqBatchExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchExecuted(msg),
            }),
            WsWireMessage::BatchQuoteExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::BatchQuoteExecuted(msg),
            }),
            WsWireMessage::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessage::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
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
        sending_ts_ms: Option<i64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsgRef>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(default, borrow)]
        msg: Option<WsOkWireRef<'a>>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
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
        sending_ts_ms: Option<i64>,
        #[serde(default, borrow)]
        msg: Option<WsErrorRef<'a>>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsTickerRef<'a>,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsTradeRef<'a>,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsOrderbookSnapshotRef<'a>,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsOrderbookDeltaRef<'a>,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsFillRef<'a>,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsMarketPositionRef<'a>,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsEventLifecycleRef<'a>,
    },
    #[serde(rename = "event_fee_update")]
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsEventFeeUpdateRef<'a>,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqCreatedRef<'a>,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqDeletedRef<'a>,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsQuoteCreatedRef<'a>,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsQuoteAcceptedRef<'a>,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsQuoteExecutedRef<'a>,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsOrderGroupUpdateRef<'a>,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsUserOrderRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value")]
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksValueRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_indexlist")]
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksIndexListRef<'a>,
    },
    #[serde(rename = "rfq_batch_created")]
    RfqBatchCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqBatchCreatedRef<'a>,
    },
    #[serde(rename = "rfq_batch_deleted")]
    RfqBatchDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqBatchDeletedRef<'a>,
    },
    #[serde(rename = "batch_quote_created")]
    BatchQuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsBatchQuoteCreatedRef<'a>,
    },
    #[serde(rename = "rfq_batch_best_quotes")]
    RfqBatchBestQuotes {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqBatchBestQuotesRef<'a>,
    },
    #[serde(rename = "batch_quote_accepted")]
    BatchQuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsBatchQuoteAcceptedRef<'a>,
    },
    #[serde(rename = "rfq_batch_executed")]
    RfqBatchExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsRfqBatchExecutedRef<'a>,
    },
    #[serde(rename = "batch_quote_executed")]
    BatchQuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsBatchQuoteExecutedRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz")]
    CfbenchmarksValue5hz {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksValue5HzRef<'a>,
    },
    #[serde(rename = "cfbenchmarks_value_5hz_indexlist")]
    CfbenchmarksValue5hzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsCfBenchmarksValue5HzIndexListRef<'a>,
    },
    #[serde(rename = "pyth_value")]
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsPythValueRef<'a>,
    },
    #[serde(rename = "pyth_value_underlying_list")]
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        #[serde(borrow)]
        msg: WsPythUnderlyingListRef<'a>,
    },
}

#[derive(Debug, Deserialize)]
pub(super) struct WsSubscribedMsgRef {
    pub channel: Option<WsChannelV2>,
    #[serde(default)]
    pub sid: Option<u64>,
}

impl<'a> WsWireMessageRef<'a> {
    pub(super) fn into_message(self) -> WsMessageRef<'a> {
        match self {
            WsWireMessageRef::Subscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Subscribed {
                id,
                sid: sid.or_else(|| msg.as_ref().and_then(|value| value.sid)),
                channel: msg.and_then(|value| value.channel),
                seq,
                sending_ts_ms,
            },
            WsWireMessageRef::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            } => WsMessageRef::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            },
            WsWireMessageRef::Ok {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => match msg {
                Some(WsOkWireRef::Subscriptions(subscriptions)) => {
                    WsMessageRef::ListSubscriptions {
                        id,
                        sid,
                        seq,
                        sending_ts_ms,
                        subscriptions,
                    }
                }
                Some(WsOkWireRef::Filters(msg)) => WsMessageRef::Ok {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: Some(msg),
                },
                None => WsMessageRef::Ok {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: None,
                },
            },
            WsWireMessageRef::ListSubscriptions {
                id,
                sid,
                seq,
                sending_ts_ms,
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
                    sending_ts_ms,
                    subscriptions: subs,
                }
            }
            WsWireMessageRef::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                error: msg.unwrap_or(WsErrorRef::default()),
            },
            WsWireMessageRef::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::RfqCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqCreated(msg),
            }),
            WsWireMessageRef::RfqDeleted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqDeleted(msg),
            }),
            WsWireMessageRef::QuoteCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteCreated(msg),
            }),
            WsWireMessageRef::QuoteAccepted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteAccepted(msg),
            }),
            WsWireMessageRef::QuoteExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteExecuted(msg),
            }),
            WsWireMessageRef::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::RfqBatchCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqBatchCreated(msg),
            }),
            WsWireMessageRef::RfqBatchDeleted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqBatchDeleted(msg),
            }),
            WsWireMessageRef::BatchQuoteCreated {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::BatchQuoteCreated(msg),
            }),
            WsWireMessageRef::RfqBatchBestQuotes {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqBatchBestQuotes(msg),
            }),
            WsWireMessageRef::BatchQuoteAccepted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::BatchQuoteAccepted(msg),
            }),
            WsWireMessageRef::RfqBatchExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqBatchExecuted(msg),
            }),
            WsWireMessageRef::BatchQuoteExecuted {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::BatchQuoteExecuted(msg),
            }),
            WsWireMessageRef::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
            WsWireMessageRef::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            }),
        }
    }
}
