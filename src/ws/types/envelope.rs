use super::channel::WsChannelV2;
use super::messages::*;
use super::msg_type::WsMsgType;
use super::subscription::{WsSubscriptionInfo, WsSubscriptionInfoRef};
use super::wire::{WsOkWire, WsOkWireRef, WsWireMessage, WsWireMessageRef};
use crate::error::KalshiError;
use bytes::Bytes;
use serde::Deserialize;
use serde::de::Error as _;
use serde_json::value::RawValue;
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize)]
pub struct WsListSubscriptions {
    #[serde(default)]
    pub subscriptions: Vec<WsSubscriptionInfo>,
}

/// Filters returned after a successful subscription update.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WsOkPayload {
    pub market_tickers: Option<Vec<String>>,
    pub market_ids: Option<Vec<String>>,
    pub index_ids: Option<Vec<String>>,
    pub underlying_tickers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WsOkPayloadRef<'a> {
    #[serde(default, borrow, deserialize_with = "optional_borrowed_strings")]
    pub market_tickers: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_strings")]
    pub market_ids: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_strings")]
    pub index_ids: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_strings")]
    pub underlying_tickers: Option<Vec<Cow<'a, str>>>,
}

fn optional_borrowed_strings<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<Cow<'de, str>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Strings<'a>(
        #[serde(
            borrow,
            deserialize_with = "super::messages::deserialize_borrowed_strings"
        )]
        Vec<Cow<'a, str>>,
    );
    Ok(Option::<Strings<'de>>::deserialize(deserializer)?.map(|value| value.0))
}

fn optional_borrowed_string<'de, D>(deserializer: D) -> Result<Option<Cow<'de, str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Text<'a>(#[serde(borrow)] Cow<'a, str>);
    Ok(Option::<Text<'de>>::deserialize(deserializer)?.map(|value| value.0))
}

impl<'a> WsOkPayloadRef<'a> {
    pub fn into_owned(self) -> WsOkPayload {
        fn own(values: Option<Vec<Cow<'_, str>>>) -> Option<Vec<String>> {
            values.map(|values| values.into_iter().map(Cow::into_owned).collect())
        }
        WsOkPayload {
            market_tickers: own(self.market_tickers),
            market_ids: own(self.market_ids),
            index_ids: own(self.index_ids),
            underlying_tickers: own(self.underlying_tickers),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WsError {
    #[serde(default)]
    pub code: Option<i64>,
    pub msg: Option<String>,
    pub market_ticker: Option<String>,
    pub market_tickers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsListSubscriptionsRef<'a> {
    #[serde(default, borrow)]
    pub subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
}

impl<'a> WsListSubscriptionsRef<'a> {
    pub fn into_owned(self) -> WsListSubscriptions {
        WsListSubscriptions {
            subscriptions: self
                .subscriptions
                .into_iter()
                .map(WsSubscriptionInfoRef::into_owned)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WsErrorRef<'a> {
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_string")]
    pub msg: Option<Cow<'a, str>>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_string")]
    pub market_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow, deserialize_with = "optional_borrowed_strings")]
    pub market_tickers: Option<Vec<Cow<'a, str>>>,
}

impl<'a> WsErrorRef<'a> {
    pub fn into_owned(self) -> WsError {
        WsError {
            code: self.code,
            msg: self.msg.map(Cow::into_owned),
            market_ticker: self.market_ticker.map(Cow::into_owned),
            market_tickers: self
                .market_tickers
                .map(|values| values.into_iter().map(Cow::into_owned).collect()),
        }
    }
}

/// Envelope used by Kalshi WS (data + errors use "type")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEnvelope {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub msg_type: WsMsgType,
    pub sid: Option<u64>,
    pub seq: Option<u64>,
    pub sending_ts_ms: Option<i64>,
    pub msg: Option<Box<RawValue>>,
    #[serde(default)]
    pub subscriptions: Option<Vec<WsSubscriptionInfo>>,
}

impl WsEnvelope {
    pub fn msg_raw(&self) -> Option<&str> {
        self.msg.as_deref().map(|raw| raw.get())
    }

    pub fn into_message(self) -> Result<WsMessageV2, KalshiError> {
        fn parse_msg<T: for<'de> Deserialize<'de>>(
            msg: &Option<Box<RawValue>>,
        ) -> Result<T, serde_json::Error> {
            let raw = msg
                .as_deref()
                .ok_or_else(|| serde_json::Error::custom("missing msg"))?;
            serde_json::from_str(raw.get())
        }

        #[derive(Deserialize)]
        struct SubscribedMsg {
            channel: Option<WsChannelV2>,
            sid: Option<u64>,
        }

        let WsEnvelope {
            id,
            msg_type,
            sid,
            seq,
            sending_ts_ms,
            msg,
            subscriptions,
        } = self;

        match msg_type {
            WsMsgType::Subscribed => {
                let subscribed = parse_msg::<SubscribedMsg>(&msg).ok();
                let sid = sid.or_else(|| subscribed.as_ref().and_then(|value| value.sid));
                let channel = subscribed.and_then(|value| value.channel);
                Ok(WsMessageV2::Subscribed {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    channel,
                })
            }
            WsMsgType::Unsubscribed => Ok(WsMessageV2::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            }),
            WsMsgType::Ok => {
                let msg = msg.as_ref().map(|_| parse_msg(&msg)).transpose()?;
                match msg {
                    Some(WsOkWire::Subscriptions(subscriptions)) => {
                        Ok(WsMessageV2::ListSubscriptions {
                            id,
                            sid,
                            seq,
                            sending_ts_ms,
                            subscriptions,
                        })
                    }
                    Some(WsOkWire::Filters(msg)) => Ok(WsMessageV2::Ok {
                        id,
                        sid,
                        seq,
                        sending_ts_ms,
                        msg: Some(msg),
                    }),
                    None => Ok(WsMessageV2::Ok {
                        id,
                        sid,
                        seq,
                        sending_ts_ms,
                        msg: None,
                    }),
                }
            }
            WsMsgType::ListSubscriptions => {
                let subs = if msg.is_some() {
                    let parsed: WsListSubscriptions = parse_msg(&msg)?;
                    parsed.subscriptions
                } else {
                    subscriptions.unwrap_or_default()
                };
                Ok(WsMessageV2::ListSubscriptions {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    subscriptions: subs,
                })
            }
            WsMsgType::Error => {
                let error = if msg.is_some() {
                    parse_msg(&msg)?
                } else {
                    WsError::default()
                };
                Ok(WsMessageV2::Error {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    error,
                })
            }
            WsMsgType::Ticker => Ok(WsMessageV2::Data(WsDataMessageV2::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Trade => Ok(WsMessageV2::Data(WsDataMessageV2::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::OrderbookSnapshot => {
                Ok(WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::OrderbookDelta => Ok(WsMessageV2::Data(WsDataMessageV2::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Fill => Ok(WsMessageV2::Data(WsDataMessageV2::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::MarketPosition => Ok(WsMessageV2::Data(WsDataMessageV2::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::MarketLifecycleV2 => {
                Ok(WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::MultivariateMarketLifecycle => Ok(WsMessageV2::Data(
                WsDataMessageV2::MultivariateMarketLifecycle {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                },
            )),
            WsMsgType::EventLifecycle => Ok(WsMessageV2::Data(WsDataMessageV2::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::EventFeeUpdate => Ok(WsMessageV2::Data(WsDataMessageV2::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::RfqCreated => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqCreated(parse_msg(&msg)?),
            })),
            WsMsgType::RfqDeleted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqDeleted(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteCreated => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteCreated(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteAccepted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteAccepted(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteExecuted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::QuoteExecuted(parse_msg(&msg)?),
            })),
            WsMsgType::OrderGroupUpdates => {
                Ok(WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::UserOrder => Ok(WsMessageV2::Data(WsDataMessageV2::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::CfbenchmarksValue => {
                Ok(WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::CfbenchmarksValueIndexlist => Ok(WsMessageV2::Data(
                WsDataMessageV2::CfbenchmarksValueIndexlist {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                },
            )),
            WsMsgType::RfqBatchCreated => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchCreated(parse_msg(&msg)?),
            })),
            WsMsgType::RfqBatchDeleted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchDeleted(parse_msg(&msg)?),
            })),
            WsMsgType::BatchQuoteCreated => {
                Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunications::BatchQuoteCreated(parse_msg(&msg)?),
                }))
            }
            WsMsgType::RfqBatchBestQuotes => {
                Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunications::RfqBatchBestQuotes(parse_msg(&msg)?),
                }))
            }
            WsMsgType::BatchQuoteAccepted => {
                Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunications::BatchQuoteAccepted(parse_msg(&msg)?),
                }))
            }
            WsMsgType::RfqBatchExecuted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunications::RfqBatchExecuted(parse_msg(&msg)?),
            })),
            WsMsgType::BatchQuoteExecuted => {
                Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunications::BatchQuoteExecuted(parse_msg(&msg)?),
                }))
            }
            WsMsgType::CfbenchmarksValue5hz => {
                Ok(WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5hz {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::CfbenchmarksValue5hzIndexlist => Ok(WsMessageV2::Data(
                WsDataMessageV2::CfbenchmarksValue5hzIndexlist {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                },
            )),
            WsMsgType::PythValue => Ok(WsMessageV2::Data(WsDataMessageV2::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::PythValueUnderlyingList => Ok(WsMessageV2::Data(
                WsDataMessageV2::PythValueUnderlyingList {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_msg(&msg)?,
                },
            )),
            other => Ok(WsMessageV2::Unknown {
                msg_type: other,
                sid,
                seq,
                sending_ts_ms,
                raw: msg,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEnvelopeRef<'a> {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub msg_type: WsMsgType,
    pub sid: Option<u64>,
    pub seq: Option<u64>,
    pub sending_ts_ms: Option<i64>,
    #[serde(borrow)]
    pub msg: Option<&'a RawValue>,
    #[serde(default, borrow)]
    pub subscriptions: Option<Vec<WsSubscriptionInfoRef<'a>>>,
}

fn parse_borrowed_msg<'a, T: Deserialize<'a>>(
    msg: Option<&'a RawValue>,
) -> Result<T, serde_json::Error> {
    let raw = msg.ok_or_else(|| serde_json::Error::custom("missing msg"))?;
    serde_json::from_str(raw.get())
}

impl<'a> WsEnvelopeRef<'a> {
    pub fn msg_raw(&self) -> Option<&str> {
        self.msg.map(|raw| raw.get())
    }

    pub fn into_message(self) -> Result<WsMessageRef<'a>, KalshiError> {
        #[derive(Deserialize)]
        struct SubscribedMsg {
            channel: Option<WsChannelV2>,
            sid: Option<u64>,
        }

        let WsEnvelopeRef {
            id,
            msg_type,
            sid,
            seq,
            sending_ts_ms,
            msg,
            subscriptions,
        } = self;
        match msg_type {
            WsMsgType::Subscribed => {
                let subscribed = parse_borrowed_msg::<SubscribedMsg>(msg).ok();
                let sid = sid.or_else(|| subscribed.as_ref().and_then(|value| value.sid));
                let channel = subscribed.and_then(|value| value.channel);
                Ok(WsMessageRef::Subscribed {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    channel,
                })
            }
            WsMsgType::Unsubscribed => Ok(WsMessageRef::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            }),
            WsMsgType::Ok => {
                let msg = msg.map(|_| parse_borrowed_msg(msg)).transpose()?;
                match msg {
                    Some(WsOkWireRef::Subscriptions(subscriptions)) => {
                        Ok(WsMessageRef::ListSubscriptions {
                            id,
                            sid,
                            seq,
                            sending_ts_ms,
                            subscriptions,
                        })
                    }
                    Some(WsOkWireRef::Filters(msg)) => Ok(WsMessageRef::Ok {
                        id,
                        sid,
                        seq,
                        sending_ts_ms,
                        msg: Some(msg),
                    }),
                    None => Ok(WsMessageRef::Ok {
                        id,
                        sid,
                        seq,
                        sending_ts_ms,
                        msg: None,
                    }),
                }
            }
            WsMsgType::ListSubscriptions => {
                let subs = if msg.is_some() {
                    let parsed: WsListSubscriptionsRef<'a> = parse_borrowed_msg(msg)?;
                    parsed.subscriptions
                } else {
                    subscriptions.unwrap_or_default()
                };
                Ok(WsMessageRef::ListSubscriptions {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    subscriptions: subs,
                })
            }
            WsMsgType::Error => {
                let error = if msg.is_some() {
                    parse_borrowed_msg::<WsErrorRef<'a>>(msg)?
                } else {
                    WsErrorRef::default()
                };
                Ok(WsMessageRef::Error {
                    id,
                    sid,
                    seq,
                    sending_ts_ms,
                    error,
                })
            }
            WsMsgType::Ticker => Ok(WsMessageRef::Data(WsDataMessageRef::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Trade => Ok(WsMessageRef::Data(WsDataMessageRef::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::OrderbookSnapshot => {
                Ok(WsMessageRef::Data(WsDataMessageRef::OrderbookSnapshot {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::OrderbookDelta => Ok(WsMessageRef::Data(WsDataMessageRef::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Fill => Ok(WsMessageRef::Data(WsDataMessageRef::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::MarketPosition => Ok(WsMessageRef::Data(WsDataMessageRef::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::MarketLifecycleV2 => {
                Ok(WsMessageRef::Data(WsDataMessageRef::MarketLifecycleV2 {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::MultivariateMarketLifecycle => Ok(WsMessageRef::Data(
                WsDataMessageRef::MultivariateMarketLifecycle {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                },
            )),
            WsMsgType::EventLifecycle => Ok(WsMessageRef::Data(WsDataMessageRef::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::EventFeeUpdate => Ok(WsMessageRef::Data(WsDataMessageRef::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::RfqCreated => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqCreated(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::RfqDeleted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::RfqDeleted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteCreated => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteCreated(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteAccepted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteAccepted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteExecuted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: WsCommunicationsRef::QuoteExecuted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::OrderGroupUpdates => {
                Ok(WsMessageRef::Data(WsDataMessageRef::OrderGroupUpdates {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::UserOrder => Ok(WsMessageRef::Data(WsDataMessageRef::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::CfbenchmarksValue => {
                Ok(WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::CfbenchmarksValueIndexlist => Ok(WsMessageRef::Data(
                WsDataMessageRef::CfbenchmarksValueIndexlist {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                },
            )),
            WsMsgType::RfqBatchCreated => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::RfqBatchCreated(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::RfqBatchDeleted => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::RfqBatchDeleted(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::BatchQuoteCreated => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::BatchQuoteCreated(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::RfqBatchBestQuotes => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::RfqBatchBestQuotes(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::BatchQuoteAccepted => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::BatchQuoteAccepted(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::RfqBatchExecuted => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::RfqBatchExecuted(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::BatchQuoteExecuted => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: WsCommunicationsRef::BatchQuoteExecuted(parse_borrowed_msg(msg)?),
                }))
            }
            WsMsgType::CfbenchmarksValue5hz => {
                Ok(WsMessageRef::Data(WsDataMessageRef::CfbenchmarksValue5hz {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::CfbenchmarksValue5hzIndexlist => Ok(WsMessageRef::Data(
                WsDataMessageRef::CfbenchmarksValue5hzIndexlist {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                },
            )),
            WsMsgType::PythValue => Ok(WsMessageRef::Data(WsDataMessageRef::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::PythValueUnderlyingList => Ok(WsMessageRef::Data(
                WsDataMessageRef::PythValueUnderlyingList {
                    sid,
                    seq,
                    sending_ts_ms,
                    msg: parse_borrowed_msg(msg)?,
                },
            )),
            other => Ok(WsMessageRef::Unknown {
                msg_type: other,
                sid,
                seq,
                sending_ts_ms,
                raw: msg,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WsMessageV2 {
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        channel: Option<WsChannelV2>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        subscriptions: Vec<WsSubscriptionInfo>,
    },
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: Option<WsOkPayload>,
    },
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        error: WsError,
    },
    Data(WsDataMessageV2),
    Unknown {
        msg_type: WsMsgType,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        raw: Option<Box<RawValue>>,
    },
}

#[derive(Debug, Clone)]
pub enum WsDataMessageV2 {
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTicker,
    },
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTrade,
    },
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookSnapshot,
    },
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookDelta,
    },
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsFill,
    },
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketPosition,
    },
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2,
    },
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2,
    },
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventLifecycle,
    },
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventFeeUpdate,
    },
    Communications {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCommunications,
    },
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderGroupUpdate,
    },
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsUserOrder,
    },
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue,
    },
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksIndexList,
    },
    CfbenchmarksValue5hz {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5Hz,
    },
    CfbenchmarksValue5hzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5HzIndexList,
    },
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythValue,
    },
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythUnderlyingList,
    },
}

macro_rules! data_message_position {
    ($message:expr, $field:ident) => {
        match $message {
            Self::Ticker { $field, .. }
            | Self::Trade { $field, .. }
            | Self::OrderbookSnapshot { $field, .. }
            | Self::OrderbookDelta { $field, .. }
            | Self::Fill { $field, .. }
            | Self::MarketPosition { $field, .. }
            | Self::MarketLifecycleV2 { $field, .. }
            | Self::MultivariateMarketLifecycle { $field, .. }
            | Self::EventLifecycle { $field, .. }
            | Self::EventFeeUpdate { $field, .. }
            | Self::Communications { $field, .. }
            | Self::OrderGroupUpdates { $field, .. }
            | Self::UserOrder { $field, .. }
            | Self::CfbenchmarksValue { $field, .. }
            | Self::CfbenchmarksValue5hz { $field, .. }
            | Self::CfbenchmarksValue5hzIndexlist { $field, .. }
            | Self::PythValue { $field, .. }
            | Self::PythValueUnderlyingList { $field, .. }
            | Self::CfbenchmarksValueIndexlist { $field, .. } => *$field,
        }
    };
}

impl WsDataMessageV2 {
    fn subscription_id(&self) -> Option<u64> {
        data_message_position!(self, sid)
    }

    fn sequence(&self) -> Option<u64> {
        data_message_position!(self, seq)
    }

    fn sending_ts_ms(&self) -> Option<i64> {
        data_message_position!(self, sending_ts_ms)
    }
}

#[derive(Debug, Clone)]
pub enum WsDataMessageRef<'a> {
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTickerRef<'a>,
    },
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsTradeRef<'a>,
    },
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookSnapshotRef<'a>,
    },
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderbookDeltaRef<'a>,
    },
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsFillRef<'a>,
    },
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketPositionRef<'a>,
    },
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventLifecycleRef<'a>,
    },
    EventFeeUpdate {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsEventFeeUpdateRef<'a>,
    },
    Communications {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCommunicationsRef<'a>,
    },
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsOrderGroupUpdateRef<'a>,
    },
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsUserOrderRef<'a>,
    },
    CfbenchmarksValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValueRef<'a>,
    },
    CfbenchmarksValueIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksIndexListRef<'a>,
    },
    CfbenchmarksValue5hz {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5HzRef<'a>,
    },
    CfbenchmarksValue5hzIndexlist {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsCfBenchmarksValue5HzIndexListRef<'a>,
    },
    PythValue {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythValueRef<'a>,
    },
    PythValueUnderlyingList {
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: WsPythUnderlyingListRef<'a>,
    },
}

impl<'a> WsDataMessageRef<'a> {
    fn subscription_id(&self) -> Option<u64> {
        data_message_position!(self, sid)
    }

    fn sequence(&self) -> Option<u64> {
        data_message_position!(self, seq)
    }

    fn sending_ts_ms(&self) -> Option<i64> {
        data_message_position!(self, sending_ts_ms)
    }
}

impl<'a> WsDataMessageRef<'a> {
    pub fn into_owned(self) -> WsDataMessageV2 {
        match self {
            WsDataMessageRef::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::Ticker {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::Trade {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::OrderbookSnapshot {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::OrderbookDelta {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::Fill {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::MarketPosition {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::MarketLifecycleV2 {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::MultivariateMarketLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::EventLifecycle {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::EventFeeUpdate {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::Communications {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::OrderGroupUpdates {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::UserOrder {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::CfbenchmarksValue {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::CfbenchmarksValue5hz {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::CfbenchmarksValue5hzIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::PythValue {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::PythValueUnderlyingList {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsDataMessageV2::CfbenchmarksValueIndexlist {
                sid,
                seq,
                sending_ts_ms,
                msg: msg.into_owned(),
            },
        }
    }
}

/// Borrowed WS message view.
///
/// Note: A smaller, purpose-built struct would be faster, but the library
/// prioritizes feature completeness across all message types.
#[derive(Debug, Clone)]
pub enum WsMessageRef<'a> {
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        channel: Option<WsChannelV2>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
    ListSubscriptions {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
    },
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        msg: Option<WsOkPayloadRef<'a>>,
    },
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        error: WsErrorRef<'a>,
    },
    Data(WsDataMessageRef<'a>),
    Unknown {
        msg_type: WsMsgType,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
        raw: Option<&'a RawValue>,
    },
}

impl<'a> WsMessageRef<'a> {
    /// Subscription id carried by any frame that has one.
    ///
    /// Consumers MUST use this instead of matching on variants, so a newly
    /// sequenced frame type cannot silently escape cursor accounting.
    pub fn subscription_id(&self) -> Option<u64> {
        match self {
            Self::Subscribed { sid, .. }
            | Self::Unsubscribed { sid, .. }
            | Self::ListSubscriptions { sid, .. }
            | Self::Ok { sid, .. }
            | Self::Error { sid, .. } => *sid,
            Self::Data(data) => data.subscription_id(),
            Self::Unknown { sid, .. } => *sid,
        }
    }

    /// Unix milliseconds at which Kalshi queued this frame at the network layer.
    pub fn sending_ts_ms(&self) -> Option<i64> {
        match self {
            Self::Subscribed { sending_ts_ms, .. }
            | Self::Unsubscribed { sending_ts_ms, .. }
            | Self::ListSubscriptions { sending_ts_ms, .. }
            | Self::Ok { sending_ts_ms, .. }
            | Self::Error { sending_ts_ms, .. }
            | Self::Unknown { sending_ts_ms, .. } => *sending_ts_ms,
            Self::Data(data) => data.sending_ts_ms(),
        }
    }

    /// Sequence number carried by any frame that has one.
    ///
    /// Consumers MUST use this instead of matching on variants, so a newly
    /// sequenced frame type cannot silently escape cursor accounting.
    pub fn sequence(&self) -> Option<u64> {
        match self {
            Self::Subscribed { seq, .. }
            | Self::Unsubscribed { seq, .. }
            | Self::ListSubscriptions { seq, .. }
            | Self::Ok { seq, .. }
            | Self::Error { seq, .. } => *seq,
            Self::Data(data) => data.sequence(),
            Self::Unknown { seq, .. } => *seq,
        }
    }

    pub fn into_owned(self) -> Result<WsMessageV2, KalshiError> {
        let owned = match self {
            WsMessageRef::Subscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
                channel,
            } => WsMessageV2::Subscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
                channel,
            },
            WsMessageRef::Unsubscribed {
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
            WsMessageRef::ListSubscriptions {
                id,
                sid,
                seq,
                sending_ts_ms,
                subscriptions,
            } => WsMessageV2::ListSubscriptions {
                id,
                sid,
                seq,
                sending_ts_ms,
                subscriptions: subscriptions
                    .into_iter()
                    .map(WsSubscriptionInfoRef::into_owned)
                    .collect(),
            },
            WsMessageRef::Ok {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg,
            } => WsMessageV2::Ok {
                id,
                sid,
                seq,
                sending_ts_ms,
                msg: msg.map(WsOkPayloadRef::into_owned),
            },
            WsMessageRef::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                error,
            } => WsMessageV2::Error {
                id,
                sid,
                seq,
                sending_ts_ms,
                error: error.into_owned(),
            },
            WsMessageRef::Data(data) => WsMessageV2::Data(data.into_owned()),
            WsMessageRef::Unknown {
                msg_type,
                sid,
                seq,
                sending_ts_ms,
                raw,
            } => {
                let raw_owned = match raw {
                    Some(value) => Some(serde_json::from_str::<Box<RawValue>>(value.get())?),
                    None => None,
                };
                WsMessageV2::Unknown {
                    msg_type,
                    sid,
                    seq,
                    sending_ts_ms,
                    raw: raw_owned,
                }
            }
        };
        Ok(owned)
    }
}

#[derive(Debug, Clone)]
pub struct WsRawEvent {
    bytes: Bytes,
}

impl WsRawEvent {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }

    pub fn parse_owned(&self) -> Result<WsMessageV2, KalshiError> {
        WsMessageV2::from_bytes(&self.bytes)
    }

    pub fn parse_borrowed(&self) -> Result<WsMessageRef<'_>, KalshiError> {
        WsMessageRef::from_bytes(&self.bytes)
    }
}

impl WsMessageV2 {
    /// Subscription id carried by any frame that has one.
    ///
    /// Consumers MUST use this instead of matching on variants, so a newly
    /// sequenced frame type cannot silently escape cursor accounting.
    pub fn subscription_id(&self) -> Option<u64> {
        match self {
            Self::Subscribed { sid, .. }
            | Self::Unsubscribed { sid, .. }
            | Self::ListSubscriptions { sid, .. }
            | Self::Ok { sid, .. }
            | Self::Error { sid, .. } => *sid,
            Self::Data(data) => data.subscription_id(),
            Self::Unknown { sid, .. } => *sid,
        }
    }

    /// Unix milliseconds at which Kalshi queued this frame at the network layer.
    pub fn sending_ts_ms(&self) -> Option<i64> {
        match self {
            Self::Subscribed { sending_ts_ms, .. }
            | Self::Unsubscribed { sending_ts_ms, .. }
            | Self::ListSubscriptions { sending_ts_ms, .. }
            | Self::Ok { sending_ts_ms, .. }
            | Self::Error { sending_ts_ms, .. }
            | Self::Unknown { sending_ts_ms, .. } => *sending_ts_ms,
            Self::Data(data) => data.sending_ts_ms(),
        }
    }

    /// Sequence number carried by any frame that has one.
    ///
    /// Consumers MUST use this instead of matching on variants, so a newly
    /// sequenced frame type cannot silently escape cursor accounting.
    pub fn sequence(&self) -> Option<u64> {
        match self {
            Self::Subscribed { seq, .. }
            | Self::Unsubscribed { seq, .. }
            | Self::ListSubscriptions { seq, .. }
            | Self::Ok { seq, .. }
            | Self::Error { seq, .. } => *seq,
            Self::Data(data) => data.sequence(),
            Self::Unknown { seq, .. } => *seq,
        }
    }
}

impl WsMessageV2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KalshiError> {
        match serde_json::from_slice::<WsWireMessage>(bytes) {
            Ok(wire) => Ok(wire.into_message()),
            Err(first_err) => match serde_json::from_slice::<WsEnvelope>(bytes) {
                Ok(env) => env.into_message(),
                Err(second_err) => Err(KalshiError::parse_reason(
                    "websocket message",
                    bytes,
                    format!(
                        "failed to parse as WsWireMessage ({first_err}); failed to parse as WsEnvelope ({second_err})"
                    ),
                )),
            },
        }
        .map_err(|err| match err {
            KalshiError::Json(source) => {
                KalshiError::parse_json("websocket message payload", bytes, source)
            }
            other => other,
        })
    }
}

impl<'a> WsMessageRef<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, KalshiError> {
        match serde_json::from_slice::<WsWireMessageRef<'a>>(bytes) {
            Ok(wire) => Ok(wire.into_message()),
            Err(first_err) => match serde_json::from_slice::<WsEnvelopeRef<'a>>(bytes) {
                Ok(env) => env.into_message(),
                Err(second_err) => Err(KalshiError::parse_reason(
                    "websocket borrowed message",
                    bytes,
                    format!(
                        "failed to parse as WsWireMessageRef ({first_err}); failed to parse as WsEnvelopeRef ({second_err})"
                    ),
                )),
            },
        }
        .map_err(|err| match err {
            KalshiError::Json(source) => {
                KalshiError::parse_json("websocket borrowed message payload", bytes, source)
            }
            other => other,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn ws_envelope_into_message_known_type() {
        let json = r#"{
            "type":"ticker",
            "sid":1,
            "seq":2,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let env: WsEnvelope = serde_json::from_str(json).unwrap();
        let msg = env.into_message().unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Ticker { .. })
        ));
    }

    #[test]
    fn ws_envelope_into_message_event_fee_update() {
        // Delivered on the market_lifecycle_v2 channel; both overrides set.
        let json = r#"{
            "type":"event_fee_update",
            "sid":7,
            "seq":8,
            "msg":{
                "event_ticker":"KXHIGHNY-24JAN01",
                "fee_type_override":"quadratic_with_maker_fees",
                "fee_multiplier_override":1.5
            }
        }"#;
        let env: WsEnvelope = serde_json::from_str(json).unwrap();
        let msg = env.into_message().unwrap();
        match msg {
            WsMessageV2::Data(WsDataMessageV2::EventFeeUpdate { msg, .. }) => {
                assert_eq!(msg.event_ticker, "KXHIGHNY-24JAN01");
                assert_eq!(
                    msg.fee_type_override.as_deref(),
                    Some("quadratic_with_maker_fees")
                );
                assert_eq!(msg.fee_multiplier_override, Some(1.5));
            }
            _ => panic!("expected event_fee_update data message"),
        }

        // Cleared override arrives as JSON null -> None, and the borrowed path
        // must round-trip to the same owned shape.
        let cleared = r#"{
            "type":"event_fee_update",
            "sid":7,
            "msg":{
                "event_ticker":"KXHIGHNY-24JAN01",
                "fee_type_override":null,
                "fee_multiplier_override":null
            }
        }"#;
        let msg_ref = WsMessageRef::from_bytes(cleared.as_bytes()).unwrap();
        match msg_ref.into_owned().unwrap() {
            WsMessageV2::Data(WsDataMessageV2::EventFeeUpdate { msg, .. }) => {
                assert!(msg.fee_type_override.is_none());
                assert!(msg.fee_multiplier_override.is_none());
            }
            _ => panic!("expected event_fee_update data message"),
        }
    }

    #[test]
    fn ws_envelope_into_message_unknown_type() {
        let json = r#"{"type":"mystery","msg":{"foo":1}}"#;
        let env: WsEnvelope = serde_json::from_str(json).unwrap();
        let msg = env.into_message().unwrap();
        match msg {
            WsMessageV2::Unknown {
                msg_type: WsMsgType::Unknown(value),
                raw,
                ..
            } => {
                assert_eq!(value, "mystery");
                assert!(raw.is_some());
            }
            _ => panic!("expected unknown message"),
        }
    }

    #[test]
    fn ws_message_from_bytes_known_type() {
        let json = r#"{
            "type":"ticker",
            "sid":1,
            "seq":2,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let msg = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Ticker { .. })
        ));
    }

    #[test]
    fn ws_frames_accept_deprecated_fields_absent_null_and_present() {
        let cases = [
            (
                "ticker",
                r#""market_ticker":"TEST","market_id":"m1","price_dollars":"0.50","yes_bid_dollars":"0.49","yes_ask_dollars":"0.51","yes_bid_size_fp":"1.00","yes_ask_size_fp":"1.00","last_trade_size_fp":"1.00","volume_fp":"10.00","open_interest_fp":"5.00","dollar_volume":5,"dollar_open_interest":2,"ts_ms":1704067200000"#,
                [
                    "",
                    r#","ts":null,"time":null"#,
                    r#","ts":1704067200,"time":"2024-01-01T00:00:00Z""#,
                ],
            ),
            (
                "trade",
                r#""trade_id":"t1","market_ticker":"TEST","count_fp":"1.00","yes_price_dollars":"0.50","no_price_dollars":"0.50","taker_outcome_side":"yes","taker_book_side":"bid","is_block_trade":false,"ts_ms":1704067200000"#,
                [
                    "",
                    r#","taker_side":null,"ts":null"#,
                    r#","taker_side":"yes","ts":1704067200"#,
                ],
            ),
            (
                "fill",
                r#""trade_id":"t","order_id":"o","market_ticker":"T","exchange_index":0,"outcome_side":"no","book_side":"ask","count_fp":"1","yes_price_dollars":"0.01","is_taker":true,"fee_cost":"0.00","ts_ms":123000,"post_position_fp":"1.00""#,
                [
                    "",
                    r#","side":null,"action":null,"purchased_side":null,"ts":null"#,
                    r#","side":"no","action":"buy","purchased_side":"yes","ts":123"#,
                ],
            ),
            (
                "user_order",
                r#""order_id":"o","user_id":"u","ticker":"T","exchange_index":0,"status":"resting","outcome_side":"yes","book_side":"bid","yes_price_dollars":"0.50","fill_count_fp":"1.00","remaining_count_fp":"2.00","initial_count_fp":"3.00","taker_fill_cost_dollars":"0.00","maker_fill_cost_dollars":"0.50","taker_fees_dollars":"0.00","maker_fees_dollars":"0.00","client_order_id":"c","created_ts_ms":1000,"last_updated_ts_ms":2000,"expiration_ts_ms":3000"#,
                [
                    "",
                    r#","side":null,"is_yes":null,"created_time":null,"last_update_time":null,"expiration_time":null"#,
                    r#","side":"yes","is_yes":true,"created_time":"created","last_update_time":"updated","expiration_time":"expires""#,
                ],
            ),
            (
                "orderbook_delta",
                r#""market_ticker":"TEST","market_id":"1","price_dollars":"0.01","delta_fp":"1","side":"yes","ts_ms":1669149841000"#,
                ["", r#","ts":null"#, r#","ts":"2022-11-22T20:44:01Z""#],
            ),
        ];

        for (kind, payload, legacy_cases) in cases {
            for legacy in legacy_cases {
                let frame =
                    format!(r#"{{"type":"{kind}","sid":1,"seq":2,"msg":{{{payload}{legacy}}}}}"#);
                let owned = WsMessageV2::from_bytes(frame.as_bytes())
                    .unwrap_or_else(|err| panic!("owned {kind} frame {legacy:?}: {err}"));
                let borrowed = WsMessageRef::from_bytes(frame.as_bytes())
                    .unwrap_or_else(|err| panic!("borrowed {kind} frame {legacy:?}: {err}"));
                assert!(
                    matches!(
                        (kind, &owned, &borrowed),
                        (
                            "ticker",
                            WsMessageV2::Data(WsDataMessageV2::Ticker { .. }),
                            WsMessageRef::Data(WsDataMessageRef::Ticker { .. })
                        ) | (
                            "trade",
                            WsMessageV2::Data(WsDataMessageV2::Trade { .. }),
                            WsMessageRef::Data(WsDataMessageRef::Trade { .. })
                        ) | (
                            "fill",
                            WsMessageV2::Data(WsDataMessageV2::Fill { .. }),
                            WsMessageRef::Data(WsDataMessageRef::Fill { .. })
                        ) | (
                            "user_order",
                            WsMessageV2::Data(WsDataMessageV2::UserOrder { .. }),
                            WsMessageRef::Data(WsDataMessageRef::UserOrder { .. })
                        ) | (
                            "orderbook_delta",
                            WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { .. }),
                            WsMessageRef::Data(WsDataMessageRef::OrderbookDelta { .. })
                        )
                    ),
                    "unexpected {kind} frame {legacy:?}: owned {owned:?}, borrowed {borrowed:?}"
                );
            }
        }
    }

    #[test]
    fn ws_message_from_bytes_list_subscriptions() {
        let json = r#"{
            "id":3,
            "type":"ok",
            "msg":[
                {
                    "channel":"communications",
                    "sid":11
                }
            ]
        }"#;

        let msg = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match msg {
            WsMessageV2::ListSubscriptions {
                id, subscriptions, ..
            } => {
                assert_eq!(id, Some(3));
                assert_eq!(subscriptions.len(), 1);
                assert_eq!(subscriptions[0].sid, 11);
                assert_eq!(subscriptions[0].channel, "communications");
            }
            other => panic!("expected list_subscriptions response, got {other:?}"),
        }

        let msg_ref = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        match msg_ref {
            WsMessageRef::ListSubscriptions {
                id, subscriptions, ..
            } => {
                assert_eq!(id, Some(3));
                assert_eq!(subscriptions.len(), 1);
                assert_eq!(subscriptions[0].sid, 11);
                assert_eq!(subscriptions[0].channel, "communications");
            }
            other => panic!("expected borrowed list_subscriptions response, got {other:?}"),
        }
    }

    #[test]
    fn ws_message_from_bytes_unknown_type() {
        let json = r#"{"type":"mystery","msg":{"foo":1}}"#;
        let msg = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match msg {
            WsMessageV2::Unknown {
                msg_type: WsMsgType::Unknown(value),
                raw,
                ..
            } => {
                assert_eq!(value, "mystery");
                assert!(raw.is_some());
            }
            _ => panic!("expected unknown message"),
        }
    }

    #[test]
    fn ws_message_unknown_preserves_sequence_metadata() {
        let json = r#"{"type":"mystery","sid":17,"seq":18,"msg":{"foo":1}}"#;

        let owned = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        assert_eq!(owned.subscription_id(), Some(17));
        assert_eq!(owned.sequence(), Some(18));

        let borrowed = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        assert_eq!(borrowed.subscription_id(), Some(17));
        assert_eq!(borrowed.sequence(), Some(18));
        let borrowed_owned = borrowed.into_owned().unwrap();
        assert_eq!(borrowed_owned.subscription_id(), Some(17));
        assert_eq!(borrowed_owned.sequence(), Some(18));
    }

    #[test]
    fn ws_message_from_bytes_invalid_json_exposes_raw_bytes_and_reason() {
        let raw = br#"{"type":"ticker","msg":{"market_ticker":"TEST"}"#;
        let err = WsMessageV2::from_bytes(raw).expect_err("invalid JSON should fail");
        match err {
            KalshiError::Parse {
                context,
                reason,
                raw: parse_raw,
                ..
            } => {
                assert_eq!(context, "websocket message");
                assert_eq!(parse_raw.as_slice(), raw);
                assert!(reason.contains("failed to parse as WsWireMessage"));
                assert!(reason.contains("failed to parse as WsEnvelope"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn ws_message_from_bytes_payload_parse_error_exposes_raw_bytes_and_reason() {
        let raw = br#"{"type":"ticker","sid":1,"seq":2,"msg":{"market_ticker":"TEST"}}"#;
        let err = WsMessageV2::from_bytes(raw).expect_err("invalid payload should fail");
        assert_eq!(err.parse_context(), Some("websocket message payload"));
        assert_eq!(err.parse_raw_bytes(), Some(&raw[..]));
        let reason = err
            .parse_error_reason()
            .expect("parse errors should include a reason");
        assert!(reason.contains("missing field"));
    }

    #[test]
    fn ws_message_ref_roundtrip_owned() {
        let json = r#"{
            "type":"trade",
            "sid":3,
            "seq":4,
            "msg":{
                "trade_id":"t1",
                "market_ticker":"TST",
                "count_fp":"2",
                "yes_price_dollars":"0.10",
                "no_price_dollars":"0.90",
                "taker_side":"yes",
                "taker_outcome_side":"yes",
                "taker_book_side":"bid",
                "is_block_trade":false,
                "ts":1704067200,
                "ts_ms":1704067200000
            }
        }"#;
        let msg_ref = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        let msg = msg_ref.into_owned().unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Trade { .. })
        ));
    }

    #[test]
    fn ws_raw_event_parse_borrowed() {
        let json = r#"{
            "type":"ticker",
            "sid":9,
            "seq":10,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let raw = WsRawEvent::new(Bytes::from(json));
        let msg = raw.parse_borrowed().unwrap();
        assert!(matches!(
            msg,
            WsMessageRef::Data(WsDataMessageRef::Ticker { .. })
        ));
    }
}
