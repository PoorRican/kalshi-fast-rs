use crate::error::KalshiError;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// User-facing message enum
// ---------------------------------------------------------------------------

/// A message received from the margin/perpetuals WebSocket.
///
/// Parsed via [`MarginDataMessage::from_bytes`]; the enum is not
/// `Deserialize`-derived directly.  Use the typed variants when pattern
/// matching on [`WsEvent::Message`](crate::ws::WsEvent::Message).
#[derive(Debug, Clone, PartialEq)]
pub enum MarginDataMessage {
    // -- data messages -------------------------------------------------------
    Ticker(MarginEnvelope<TickerMsg>),
    OrderbookDelta(MarginEnvelope<OrderbookDeltaMsg>),
    Trade(MarginEnvelope<TradeMsg>),
    Fill(MarginEnvelope<FillMsg>),
    UserOrder(MarginEnvelope<UserOrderMsg>),
    OrderGroupUpdates(MarginEnvelope<OrderGroupUpdatesMsg>),
    OrderbookSnapshot(MarginEnvelope<OrderbookSnapshotMsg>),
    CorporateActionsDelta(MarginEnvelope<CorporateActionsDeltaMsg>),
    MarketLifecycleV2(MarginEnvelope<MarketLifecycleV2Msg>),
    // -- control messages ----------------------------------------------------
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        /// Top-level sid if supplied; `sid` remains the subscribed channel's sid.
        envelope_sid: Option<u64>,
        /// A top-level cursor if present on the acknowledgement.
        seq: Option<u64>,
        channel: String,
        sending_ts_ms: Option<i64>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        msg: Option<MarginOkMsg>,
        sending_ts_ms: Option<i64>,
    },
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        msg: MarginErrorMsg,
        sending_ts_ms: Option<i64>,
    },
    /// Catch-all for unrecognised message types (e.g. future channel types).
    Unknown {
        /// The raw `type` field value if present.
        msg_type: Option<String>,
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        sending_ts_ms: Option<i64>,
    },
}

impl MarginDataMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KalshiError> {
        match serde_json::from_slice::<MarginWireMessage>(bytes) {
            Ok(MarginWireMessage::Other) => {
                #[derive(Deserialize)]
                struct Raw {
                    #[serde(rename = "type")]
                    msg_type: Option<String>,
                    id: Option<u64>,
                    sid: Option<u64>,
                    seq: Option<u64>,
                    sending_ts_ms: Option<i64>,
                }
                let raw = serde_json::from_slice::<Raw>(bytes).map_err(|source| {
                    KalshiError::parse_json("margin websocket message", bytes, source)
                })?;
                Ok(Self::Unknown {
                    msg_type: raw.msg_type,
                    id: raw.id,
                    sid: raw.sid,
                    seq: raw.seq,
                    sending_ts_ms: raw.sending_ts_ms,
                })
            }
            Ok(wire) => Ok(wire.into()),
            Err(source) => Err(KalshiError::parse_json(
                "margin websocket message",
                bytes,
                source,
            )),
        }
    }

    /// Timestamp when Kalshi queued this frame at the network layer.
    pub fn sending_ts_ms(&self) -> Option<i64> {
        match self {
            Self::Ticker(e) => e.sending_ts_ms,
            Self::OrderbookDelta(e) => e.sending_ts_ms,
            Self::Trade(e) => e.sending_ts_ms,
            Self::Fill(e) => e.sending_ts_ms,
            Self::UserOrder(e) => e.sending_ts_ms,
            Self::OrderGroupUpdates(e) => e.sending_ts_ms,
            Self::OrderbookSnapshot(e) => e.sending_ts_ms,
            Self::CorporateActionsDelta(e) => e.sending_ts_ms,
            Self::MarketLifecycleV2(e) => e.sending_ts_ms,
            Self::Subscribed { sending_ts_ms, .. }
            | Self::Unsubscribed { sending_ts_ms, .. }
            | Self::Ok { sending_ts_ms, .. }
            | Self::Error { sending_ts_ms, .. }
            | Self::Unknown { sending_ts_ms, .. } => *sending_ts_ms,
        }
    }
}

// ---------------------------------------------------------------------------
// Tagged wire enum  (private – parsed first, then converted)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum MarginWireMessage {
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot(SequencedMarginEnvelope<OrderbookSnapshotMsg>),
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta(SequencedMarginEnvelope<OrderbookDeltaMsg>),
    #[serde(rename = "ticker")]
    Ticker(MarginEnvelope<TickerMsg>),
    #[serde(rename = "trade")]
    Trade(SequencedMarginEnvelope<TradeMsg>),
    #[serde(rename = "fill")]
    Fill(MarginEnvelope<FillMsg>),
    #[serde(rename = "user_order")]
    UserOrder(MarginEnvelope<UserOrderMsg>),
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates(SequencedMarginEnvelope<OrderGroupUpdatesMsg>),
    #[serde(rename = "corporate_actions_delta")]
    CorporateActionsDelta(SequencedMarginEnvelope<CorporateActionsDeltaMsg>),
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2(SequencedMarginEnvelope<MarketLifecycleV2Msg>),
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        msg: MarginSubscribedMsg,
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed {
        id: Option<u64>,
        sid: u64,
        seq: u64,
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        #[serde(default)]
        sid: Option<u64>,
        #[serde(default)]
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<MarginOkMsg>,
        sending_ts_ms: Option<i64>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        #[serde(default)]
        sid: Option<u64>,
        #[serde(default)]
        seq: Option<u64>,
        msg: MarginErrorMsg,
        sending_ts_ms: Option<i64>,
    },
    #[serde(other)]
    Other,
}

impl From<MarginWireMessage> for MarginDataMessage {
    fn from(wire: MarginWireMessage) -> Self {
        match wire {
            MarginWireMessage::OrderbookSnapshot(e) => Self::OrderbookSnapshot(e.into()),
            MarginWireMessage::OrderbookDelta(e) => Self::OrderbookDelta(e.into()),
            MarginWireMessage::Ticker(e) => Self::Ticker(e),
            MarginWireMessage::Trade(e) => Self::Trade(e.into()),
            MarginWireMessage::Fill(e) => Self::Fill(e),
            MarginWireMessage::UserOrder(e) => Self::UserOrder(e),
            MarginWireMessage::OrderGroupUpdates(e) => Self::OrderGroupUpdates(e.into()),
            MarginWireMessage::CorporateActionsDelta(e) => Self::CorporateActionsDelta(e.into()),
            MarginWireMessage::MarketLifecycleV2(e) => Self::MarketLifecycleV2(e.into()),
            MarginWireMessage::Subscribed {
                id,
                sid,
                seq,
                msg,
                sending_ts_ms,
            } => Self::Subscribed {
                id,
                sid: Some(msg.sid),
                envelope_sid: sid,
                seq,
                channel: msg.channel,
                sending_ts_ms,
            },
            MarginWireMessage::Unsubscribed {
                id,
                sid,
                seq,
                sending_ts_ms,
            } => Self::Unsubscribed {
                id,
                sid: Some(sid),
                seq: Some(seq),
                sending_ts_ms,
            },
            MarginWireMessage::Ok {
                id,
                sid,
                seq,
                msg,
                sending_ts_ms,
            } => Self::Ok {
                id,
                sid,
                seq,
                msg,
                sending_ts_ms,
            },
            MarginWireMessage::Error {
                id,
                sid,
                seq,
                msg,
                sending_ts_ms,
            } => Self::Error {
                id,
                sid,
                seq,
                msg,
                sending_ts_ms,
            },
            MarginWireMessage::Other => Self::Unknown {
                msg_type: None,
                id: None,
                sid: None,
                seq: None,
                sending_ts_ms: None,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Control message payload structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginSubscribedMsg {
    pub channel: String,
    pub sid: u64,
}

/// The two documented shapes of an `ok` payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MarginOkMsg {
    Update(MarginUpdateAck),
    Subscriptions(Vec<MarginSubscriptionInfo>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginUpdateAck {
    pub market_tickers: Option<Vec<String>>,
    pub market_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginSubscriptionInfo {
    pub channel: String,
    pub sid: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginErrorMsg {
    pub code: u64,
    pub msg: String,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub market_tickers: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Generic envelope
// ---------------------------------------------------------------------------

/// Shared envelope with `sid`, optional `seq`, and a typed `msg` payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginEnvelope<T> {
    #[serde(default)]
    pub id: Option<u64>,
    pub sid: u64,
    #[serde(default)]
    pub seq: Option<u64>,
    #[serde(default)]
    pub sending_ts_ms: Option<i64>,
    pub msg: T,
}

#[derive(Debug, Deserialize)]
struct SequencedMarginEnvelope<T> {
    #[serde(default)]
    id: Option<u64>,
    sid: u64,
    seq: u64,
    sending_ts_ms: Option<i64>,
    msg: T,
}

impl<T> From<SequencedMarginEnvelope<T>> for MarginEnvelope<T> {
    fn from(envelope: SequencedMarginEnvelope<T>) -> Self {
        Self {
            id: envelope.id,
            sid: envelope.sid,
            seq: Some(envelope.seq),
            sending_ts_ms: envelope.sending_ts_ms,
            msg: envelope.msg,
        }
    }
}

// ---------------------------------------------------------------------------
// orderbook_snapshot  —  full orderbook depth
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginSide {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginOrderSource {
    User,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginSelfTradePreventionType {
    TakerAtCross,
    Maker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MarginLastUpdateReason {
    Decrease,
    Amend,
    MarginCancel,
    SelfTradeCancel,
    ExpiryCancel,
    CloseCancel,
    HaltCancel,
    Trade,
    PostOnlyCrossCancel,
    ReduceOnlyCancel,
    /// The overnight equities settlement band canceled a resting order priced
    /// outside it.
    SettlementBoundsCancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginOrderGroupEventType {
    Created,
    Triggered,
    Reset,
    Deleted,
    LimitUpdated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequiredNullableI64(pub Option<i64>);

impl<'de> Deserialize<'de> for RequiredNullableI64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::<i64>::deserialize(deserializer).map(Self)
    }
}

fn deserialize_required_nullable_i64<'de, D>(
    deserializer: D,
) -> Result<RequiredNullableI64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<i64>::deserialize(deserializer).map(RequiredNullableI64)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderbookSnapshotMsg {
    pub market_ticker: String,
    #[serde(default)]
    pub bid: Option<Vec<[String; 2]>>,
    #[serde(default)]
    pub ask: Option<Vec<[String; 2]>>,
}

// ---------------------------------------------------------------------------
// orderbook_delta  —  incremental price-level change
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderbookDeltaMsg {
    pub market_ticker: String,
    pub price: String,
    pub delta: String,
    pub side: MarginSide,
    #[serde(default)]
    pub last_update_reason: Option<MarginLastUpdateReason>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

// ---------------------------------------------------------------------------
// ticker  —  market summary
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerPrice {
    pub price: String,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerFundingRate {
    pub rate: f64,
    pub next_funding_time_ms: i64,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerMsg {
    pub market_ticker: String,
    pub price: String,
    pub bid: String,
    pub ask: String,
    pub bid_size_fp: String,
    pub ask_size_fp: String,
    pub last_trade_size_fp: String,
    pub volume: String,
    pub volume_notional_value_dollars: String,
    pub volume_24h: String,
    pub volume_24h_notional_value_dollars: String,
    pub open_interest: String,
    pub open_interest_notional_value_dollars: String,
    #[serde(default)]
    pub reference_price: Option<TickerPrice>,
    #[serde(default)]
    pub settlement_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub liquidation_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub funding_rate: Option<TickerFundingRate>,
    pub ts_ms: i64,
}

// ---------------------------------------------------------------------------
// trade  —  public trade notification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradeMsg {
    pub trade_id: String,
    pub market_ticker: String,
    pub price: String,
    pub count: String,
    pub taker_side: MarginSide,
    pub ts_ms: i64,
}

// ---------------------------------------------------------------------------
// fill  —  private fill notification (requires auth)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillMsg {
    pub trade_id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: MarginSide,
    pub ts_ms: i64,
    pub price: String,
    pub count: String,
    pub fee_cost: String,
    pub post_position: String,
    #[serde(default)]
    pub subaccount: Option<i64>,
    pub order_source: MarginOrderSource,
}

// ---------------------------------------------------------------------------
// user_order  —  private order update (requires auth)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserOrderMsg {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub side: MarginSide,
    pub price: String,
    pub fill_count: String,
    pub remaining_count: String,
    #[serde(default)]
    pub self_trade_prevention_type: Option<MarginSelfTradePreventionType>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub expiration_ts_ms: Option<i64>,
    #[serde(deserialize_with = "deserialize_required_nullable_i64")]
    pub created_ts_ms: RequiredNullableI64,
    #[serde(default)]
    pub last_updated_ts_ms: Option<i64>,
    #[serde(default)]
    pub last_update_reason: Option<MarginLastUpdateReason>,
    #[serde(default)]
    pub subaccount_number: Option<i64>,
    pub order_source: MarginOrderSource,
}

// ---------------------------------------------------------------------------
// order_group_updates  —  order-group lifecycle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderGroupUpdatesMsg {
    pub event_type: MarginOrderGroupEventType,
    pub order_group_id: String,
    #[serde(default)]
    pub contracts_limit_fp: Option<String>,
    pub ts_ms: i64,
}

/// An applied corporate action, including the exchange's stable identifiers.
#[derive(Debug, Clone, PartialEq)]
pub struct CorporateAction {
    pub id: String,
    pub market_id: String,
    pub market_ticker: String,
    pub ts_ms: i64,
    pub terms: CorporateActionTerms,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CorporateActionTerms {
    StockSplit(CorporateActionSplitTerms),
    FinalSettlement(CorporateActionFinalSettlementTerms),
    TickerChange(CorporateActionTickerChangeTerms),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CorporateActionSplitTerms {
    pub ratio_old: String,
    pub ratio_new: String,
    pub pre_split_mark_price: String,
    pub post_split_mark_price: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CorporateActionFinalSettlementTerms {
    pub settlement_price_per_contract: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CorporateActionTickerChangeTerms {
    pub old_ticker: String,
    pub new_ticker: String,
}

impl<'de> Deserialize<'de> for CorporateAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Wire {
            id: String,
            market_id: String,
            market_ticker: String,
            #[serde(rename = "type")]
            action_type: String,
            ts_ms: i64,
            split: Option<CorporateActionSplitTerms>,
            final_settlement: Option<CorporateActionFinalSettlementTerms>,
            ticker_change: Option<CorporateActionTickerChangeTerms>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let terms = match (
            wire.action_type.as_str(),
            wire.split,
            wire.final_settlement,
            wire.ticker_change,
        ) {
            ("stock_split", Some(terms), None, None) => CorporateActionTerms::StockSplit(terms),
            ("final_settlement", None, Some(terms), None) => {
                CorporateActionTerms::FinalSettlement(terms)
            }
            ("ticker_change", None, None, Some(terms)) => CorporateActionTerms::TickerChange(terms),
            _ => {
                return Err(serde::de::Error::custom(
                    "corporate action type must have exactly its matching terms",
                ));
            }
        };
        Ok(Self {
            id: wire.id,
            market_id: wire.market_id,
            market_ticker: wire.market_ticker,
            ts_ms: wire.ts_ms,
            terms,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CorporateActionsDeltaMsg {
    pub corporate_action: CorporateAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginMarketLifecycleEventType {
    Activated,
    Deactivated,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarketLifecycleV2Msg {
    pub market_ticker: String,
    pub event_type: MarginMarketLifecycleEventType,
    pub is_deactivated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(bytes: &[u8]) -> MarginDataMessage {
        MarginDataMessage::from_bytes(bytes).expect("parse ok")
    }

    #[test]
    fn parse_orderbook_snapshot() {
        let json = serde_json::json!({
            "type": "orderbook_snapshot",
            "sid": 1,
            "seq": 100,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "bid": [["50000.00", "1.5"]],
                "ask": [["50100.00", "2.0"]]
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::OrderbookSnapshot(e) => {
                assert_eq!(e.sid, 1);
                assert_eq!(e.seq, Some(100));
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(
                    e.msg.bid.as_ref().unwrap()[0],
                    ["50000.00".to_string(), "1.5".to_string()]
                );
            }
            _ => panic!("expected OrderbookSnapshot"),
        }
    }

    #[test]
    fn parse_orderbook_delta() {
        let json = serde_json::json!({
            "type": "orderbook_delta",
            "sid": 1,
            "seq": 101,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "price": "50000.00",
                "delta": "1.5",
                "side": "bid",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::OrderbookDelta(e) => {
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(e.msg.side, MarginSide::Bid);
                assert_eq!(e.msg.ts_ms, Some(1_700_000_000_000));
            }
            _ => panic!("expected OrderbookDelta"),
        }
    }

    #[test]
    fn parse_ticker() {
        let json = serde_json::json!({
            "type": "ticker",
            "sid": 2,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "price": "50000.0000",
                "bid": "49900.0000",
                "ask": "50100.0000",
                "bid_size_fp": "10.0",
                "ask_size_fp": "5.0",
                "last_trade_size_fp": "1.0",
                "volume": "1000",
                "volume_notional_value_dollars": "50000000",
                "volume_24h": "500",
                "volume_24h_notional_value_dollars": "25000000",
                "open_interest": "200",
                "open_interest_notional_value_dollars": "10000000",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::Ticker(e) => {
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(e.msg.price, "50000.0000");
            }
            _ => panic!("expected Ticker"),
        }
    }

    #[test]
    fn parse_trade() {
        let json = serde_json::json!({
            "type": "trade",
            "sid": 3,
            "seq": 42,
            "msg": {
                "trade_id": "t-123",
                "market_ticker": "KXBTCPERP1",
                "price": "50000.00",
                "count": "1.0",
                "taker_side": "bid",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::Trade(e) => {
                assert_eq!(e.msg.trade_id, "t-123");
                assert_eq!(e.msg.taker_side, MarginSide::Bid);
            }
            _ => panic!("expected Trade"),
        }
    }

    #[test]
    fn parse_fill() {
        let json = serde_json::json!({
            "type": "fill",
            "sid": 4,
            "msg": {
                "trade_id": "t-456",
                "order_id": "o-789",
                "client_order_id": "co-001",
                "market_ticker": "KXBTCPERP1",
                "is_taker": true,
                "side": "ask",
                "ts_ms": 1_700_000_000_000_i64,
                "price": "50000.00",
                "count": "1.0",
                "fee_cost": "0.50",
                "post_position": "5.0",
                "order_source": "user",
                "subaccount": 1
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::Fill(e) => {
                assert!(e.msg.is_taker);
                assert_eq!(e.msg.side, MarginSide::Ask);
                assert_eq!(e.msg.subaccount, Some(1));
            }
            _ => panic!("expected Fill"),
        }
    }

    #[test]
    fn parse_user_order() {
        let json = serde_json::json!({
            "type": "user_order",
            "sid": 5,
            "msg": {
                "order_id": "o-789",
                "user_id": "u-001",
                "client_order_id": "co-001",
                "ticker": "KXBTCPERP1",
                "side": "bid",
                "price": "50000.00",
                "fill_count": "0",
                "remaining_count": "1.0",
                "created_ts_ms": null,
                "order_source": "user",
                "last_update_reason": "SettlementBoundsCancel"
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::UserOrder(e) => {
                assert_eq!(e.msg.side, MarginSide::Bid);
                assert_eq!(e.msg.fill_count, "0");
                assert_eq!(e.msg.created_ts_ms.0, None);
                assert_eq!(
                    e.msg.last_update_reason,
                    Some(MarginLastUpdateReason::SettlementBoundsCancel)
                );
            }
            _ => panic!("expected UserOrder"),
        }
    }

    #[test]
    fn user_order_requires_nullable_created_timestamp_field() {
        let bytes = br#"{"type":"user_order","sid":5,"msg":{"order_id":"o","user_id":"u","client_order_id":"c","ticker":"KXBTCPERP1","side":"bid","price":"1","fill_count":"0","remaining_count":"1","order_source":"user"}}"#;
        let error = MarginDataMessage::from_bytes(bytes).unwrap_err();
        assert_eq!(error.parse_context(), Some("margin websocket message"));
    }

    #[test]
    fn nullable_ticker_fields_and_unsequenced_timestamp() {
        let bytes = br#"{"type":"ticker","sid":2,"sending_ts_ms":99,"msg":{"market_ticker":"T","price":"1","bid":"1","ask":"2","bid_size_fp":"1","ask_size_fp":"2","last_trade_size_fp":"1","volume":"1","volume_notional_value_dollars":"1","volume_24h":"1","volume_24h_notional_value_dollars":"1","open_interest":"1","open_interest_notional_value_dollars":"1","reference_price":null,"settlement_mark_price":null,"liquidation_mark_price":null,"funding_rate":null,"ts_ms":98}}"#;
        let parsed = parse(bytes);
        assert_eq!(parsed.sending_ts_ms(), Some(99));
        let MarginDataMessage::Ticker(e) = parsed else {
            panic!("expected ticker")
        };
        assert_eq!(e.sending_ts_ms, Some(99));
        assert_eq!(e.seq, None);
        assert_eq!(e.msg.reference_price, None);
        assert_eq!(e.msg.funding_rate, None);
    }

    #[test]
    fn known_message_rejects_unknown_enum_value() {
        let bytes = br#"{"type":"trade","sid":1,"seq":1,"msg":{"trade_id":"t","market_ticker":"KXBTCPERP1","price":"1","count":"1","taker_side":"sideways","ts_ms":1}}"#;
        let error = MarginDataMessage::from_bytes(bytes).unwrap_err();
        assert_eq!(error.parse_context(), Some("margin websocket message"));
    }

    #[test]
    fn parse_order_group_updates() {
        let json = serde_json::json!({
            "type": "order_group_updates",
            "sid": 21,
            "seq": 7,
            "msg": {
                "event_type": "limit_updated",
                "order_group_id": "og_123",
                "contracts_limit_fp": "150.00",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::OrderGroupUpdates(e) => {
                assert_eq!(e.msg.event_type, MarginOrderGroupEventType::LimitUpdated);
                assert_eq!(e.msg.order_group_id, "og_123");
            }
            _ => panic!("expected OrderGroupUpdates"),
        }
    }

    #[test]
    fn parse_subscribed() {
        let json = serde_json::json!({
            "type": "subscribed",
            "id": 1,
            "msg": {"channel": "ticker", "sid": 42}
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Subscribed {
                id: Some(1),
                sid: Some(42),
                envelope_sid: None,
                seq: None,
                channel: "ticker".into(),
                sending_ts_ms: None,
            }
        );
    }

    #[test]
    fn parse_unsubscribed() {
        let json = serde_json::json!({
            "type": "unsubscribed",
            "sid": 42,
            "seq": 1
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Unsubscribed {
                id: None,
                sid: Some(42),
                seq: Some(1),
                sending_ts_ms: None,
            }
        );
    }

    #[test]
    fn parse_error() {
        let json = serde_json::json!({
            "type": "error",
            "id": 1,
            "sid": 42,
            "seq": 7,
            "msg": {"code": 8, "msg": "Unknown channel name"}
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Error {
                id: Some(1),
                sid: Some(42),
                seq: Some(7),
                msg: MarginErrorMsg {
                    code: 8,
                    msg: "Unknown channel name".into(),
                    market_ticker: None,
                    market_tickers: None,
                },
                sending_ts_ms: None,
            }
        );
    }

    #[test]
    fn parse_ok() {
        let json = serde_json::json!({
            "type": "ok",
            "id": 1,
            "sid": 42,
            "seq": 7,
            "msg": {"market_tickers": ["T"], "market_ids": ["M"]}
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Ok {
                id: Some(1),
                sid: Some(42),
                seq: Some(7),
                msg: Some(MarginOkMsg::Update(MarginUpdateAck {
                    market_tickers: Some(vec!["T".into()]),
                    market_ids: Some(vec!["M".into()]),
                })),
                sending_ts_ms: None,
            }
        );
    }

    #[test]
    fn subscription_list_ack_has_typed_entries() {
        let parsed = parse(
            br#"{"type":"ok","id":8,"sending_ts_ms":9,"msg":[{"channel":"ticker","sid":4}]}"#,
        );
        assert_eq!(parsed.sending_ts_ms(), Some(9));
        let MarginDataMessage::Ok {
            msg: Some(MarginOkMsg::Subscriptions(entries)),
            ..
        } = parsed
        else {
            panic!("expected list")
        };
        assert_eq!(
            entries,
            vec![MarginSubscriptionInfo {
                channel: "ticker".into(),
                sid: 4
            }]
        );
        assert!(
            MarginDataMessage::from_bytes(br#"{"type":"ok","msg":[{"channel":"ticker"}]}"#)
                .is_err()
        );
    }

    #[test]
    fn control_and_unknown_cursor_metadata_is_retained() {
        let subscribed = parse(br#"{"type":"subscribed","id":1,"sid":9,"seq":2,"sending_ts_ms":3,"msg":{"channel":"ticker","sid":4}}"#);
        assert!(matches!(
            subscribed,
            MarginDataMessage::Subscribed {
                id: Some(1),
                sid: Some(4),
                envelope_sid: Some(9),
                seq: Some(2),
                sending_ts_ms: Some(3),
                ..
            }
        ));
        let unknown = parse(br#"{"type":"future","id":1,"sid":4,"seq":2,"sending_ts_ms":3}"#);
        assert!(matches!(
            unknown,
            MarginDataMessage::Unknown {
                id: Some(1),
                sid: Some(4),
                seq: Some(2),
                sending_ts_ms: Some(3),
                ..
            }
        ));
    }

    #[test]
    fn malformed_known_type_is_an_error() {
        let bytes = br#"{"type":"ticker","sid":1,"msg":{"market_ticker":"KXBTCPERP1"}}"#;
        let error = MarginDataMessage::from_bytes(bytes).unwrap_err();
        assert_eq!(error.parse_context(), Some("margin websocket message"));
        assert_eq!(error.parse_raw_bytes(), Some(bytes.as_slice()));
    }

    #[test]
    fn sequenced_message_requires_sequence_number() {
        let bytes = br#"{"type":"trade","sid":1,"msg":{"trade_id":"t","market_ticker":"KXBTCPERP1","price":"1","count":"1","taker_side":"bid","ts_ms":1}}"#;
        let error = MarginDataMessage::from_bytes(bytes).unwrap_err();
        assert_eq!(error.parse_context(), Some("margin websocket message"));
    }

    #[test]
    fn snapshot_rejects_price_levels_with_wrong_length() {
        let bytes = br#"{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{"market_ticker":"KXBTCPERP1","bid":[["1"]]}}"#;
        let error = MarginDataMessage::from_bytes(bytes).unwrap_err();
        assert_eq!(error.parse_context(), Some("margin websocket message"));
    }

    #[test]
    fn parse_unknown_type_falls_back() {
        let json = serde_json::json!({
            "type": "some_future_channel",
            "sid": 1,
            "msg": {"foo": "bar"}
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Unknown {
                msg_type: Some("some_future_channel".into()),
                id: None,
                sid: Some(1),
                seq: None,
                sending_ts_ms: None,
            }
        );
    }

    #[test]
    fn corporate_actions_have_matching_typed_terms_and_identity() {
        let cases = [
            (
                "stock_split",
                "split",
                serde_json::json!({"ratio_old":"1","ratio_new":"4","pre_split_mark_price":"40.0000","post_split_mark_price":"10.0000"}),
            ),
            (
                "final_settlement",
                "final_settlement",
                serde_json::json!({"settlement_price_per_contract":"9.5000"}),
            ),
            (
                "ticker_change",
                "ticker_change",
                serde_json::json!({"old_ticker":"OLD","new_ticker":"NEW"}),
            ),
        ];
        for (kind, field, terms) in cases {
            let mut action = serde_json::json!({"id":"ca-1","market_id":"stable-id","market_ticker":"NEW","type":kind,"ts_ms":100});
            action[field] = terms;
            let frame = serde_json::json!({"type":"corporate_actions_delta","sid":3,"seq":4,"sending_ts_ms":101,"msg":{"corporate_action":action}});
            let parsed = parse(&serde_json::to_vec(&frame).unwrap());
            assert_eq!(parsed.sending_ts_ms(), Some(101));
            let MarginDataMessage::CorporateActionsDelta(e) = parsed else {
                panic!("expected corporate action")
            };
            assert_eq!(e.seq, Some(4));
            assert_eq!(e.msg.corporate_action.id, "ca-1");
            assert_eq!(e.msg.corporate_action.market_id, "stable-id");
            assert_eq!(e.msg.corporate_action.market_ticker, "NEW");
            match (kind, e.msg.corporate_action.terms) {
                ("stock_split", CorporateActionTerms::StockSplit(t)) => {
                    assert_eq!(t.ratio_new, "4")
                }
                ("final_settlement", CorporateActionTerms::FinalSettlement(t)) => {
                    assert_eq!(t.settlement_price_per_contract, "9.5000")
                }
                ("ticker_change", CorporateActionTerms::TickerChange(t)) => {
                    assert_eq!(t.old_ticker, "OLD")
                }
                _ => panic!("wrong terms variant"),
            }
        }
        let bad = br#"{"type":"corporate_actions_delta","sid":1,"seq":2,"msg":{"corporate_action":{"id":"x","market_id":"m","market_ticker":"T","type":"stock_split","ts_ms":1,"ticker_change":{"old_ticker":"A","new_ticker":"B"}}}}"#;
        assert!(MarginDataMessage::from_bytes(bad).is_err());
    }

    #[test]
    fn lifecycle_and_sending_timestamps_survive_every_envelope_kind() {
        let frames: &[&[u8]] = &[
            br#"{"type":"market_lifecycle_v2","sid":1,"seq":2,"sending_ts_ms":42,"msg":{"market_ticker":"T","event_type":"activated","is_deactivated":false}}"#,
            br#"{"type":"orderbook_snapshot","id":7,"sid":1,"seq":2,"sending_ts_ms":42,"msg":{"market_ticker":"T","bid":null,"ask":null}}"#,
            br#"{"type":"subscribed","id":1,"sending_ts_ms":42,"msg":{"channel":"ticker","sid":2}}"#,
            br#"{"type":"unsubscribed","sid":2,"seq":3,"sending_ts_ms":42}"#,
            br#"{"type":"ok","id":1,"sending_ts_ms":42,"msg":null}"#,
            br#"{"type":"error","id":1,"sending_ts_ms":42,"msg":{"code":1,"msg":"bad"}}"#,
            br#"{"type":"future_type","sending_ts_ms":42}"#,
        ];
        for frame in frames {
            assert_eq!(parse(frame).sending_ts_ms(), Some(42));
        }
        let MarginDataMessage::MarketLifecycleV2(e) = parse(frames[0]) else {
            panic!("expected lifecycle")
        };
        assert_eq!(e.msg.event_type, MarginMarketLifecycleEventType::Activated);
        assert!(!e.msg.is_deactivated);
        let MarginDataMessage::OrderbookSnapshot(e) = parse(frames[1]) else {
            panic!("expected snapshot")
        };
        assert_eq!(e.id, Some(7));
        assert_eq!(e.msg.bid, None);
        assert_eq!(e.msg.ask, None);
    }
}
