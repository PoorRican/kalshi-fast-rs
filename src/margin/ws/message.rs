use crate::error::KalshiError;
use serde::Deserialize;
use serde_json::Value;

// ---------------------------------------------------------------------------
// User-facing message enum
// ---------------------------------------------------------------------------

/// A message received from the margin/perpetuals WebSocket.
///
/// Parsed via [`MarginDataMessage::from_bytes`]; the enum is not
/// `Deserialize`-derived directly.  Use the typed variants when pattern
/// matching on [`WsEvent::Message`](crate::ws::event::WsEvent::Message).
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
    // -- control messages ----------------------------------------------------
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
    },
    Ok {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        msg: Option<Value>,
    },
    Error {
        id: Option<u64>,
        sid: Option<u64>,
        seq: Option<u64>,
        msg: MarginErrorMsg,
    },
    /// Catch-all for unrecognised message types (e.g. future channel types).
    Unknown {
        /// The raw `type` field value if present.
        msg_type: Option<String>,
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
                }
                let raw = serde_json::from_slice::<Raw>(bytes).map_err(|source| {
                    KalshiError::parse_json("margin websocket message", bytes, source)
                })?;
                Ok(Self::Unknown {
                    msg_type: raw.msg_type,
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
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        msg: MarginSubscribedMsg,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed { id: Option<u64>, sid: u64, seq: u64 },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        #[serde(default)]
        sid: Option<u64>,
        #[serde(default)]
        seq: Option<u64>,
        #[serde(default)]
        msg: Option<Value>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        #[serde(default)]
        sid: Option<u64>,
        #[serde(default)]
        seq: Option<u64>,
        msg: MarginErrorMsg,
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
            MarginWireMessage::Subscribed { id, msg } => Self::Subscribed {
                id,
                sid: Some(msg.sid),
            },
            MarginWireMessage::Unsubscribed { id, sid, seq } => Self::Unsubscribed {
                id,
                sid: Some(sid),
                seq: Some(seq),
            },
            MarginWireMessage::Ok { id, sid, seq, msg } => Self::Ok { id, sid, seq, msg },
            MarginWireMessage::Error { id, sid, seq, msg } => Self::Error { id, sid, seq, msg },
            MarginWireMessage::Other => Self::Unknown { msg_type: None },
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
    pub msg: T,
}

#[derive(Debug, Deserialize)]
struct SequencedMarginEnvelope<T> {
    #[serde(default)]
    id: Option<u64>,
    sid: u64,
    seq: u64,
    msg: T,
}

impl<T> From<SequencedMarginEnvelope<T>> for MarginEnvelope<T> {
    fn from(envelope: SequencedMarginEnvelope<T>) -> Self {
        Self {
            id: envelope.id,
            sid: envelope.sid,
            seq: Some(envelope.seq),
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
                "order_source": "user"
            }
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        match msg {
            MarginDataMessage::UserOrder(e) => {
                assert_eq!(e.msg.side, MarginSide::Bid);
                assert_eq!(e.msg.fill_count, "0");
                assert_eq!(e.msg.created_ts_ms.0, None);
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
                sid: Some(42)
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
                seq: Some(1)
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
                }
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
            "msg": {"ok": true}
        });
        let msg = parse(&serde_json::to_vec(&json).unwrap());
        assert_eq!(
            msg,
            MarginDataMessage::Ok {
                id: Some(1),
                sid: Some(42),
                seq: Some(7),
                msg: Some(serde_json::json!({"ok": true})),
            }
        );
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
                msg_type: Some("some_future_channel".into())
            }
        );
    }
}
