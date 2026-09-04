use serde::Deserialize;
use std::borrow::Cow;

/// Message payload for the `pyth_value` WebSocket channel. Added
/// 2026-07-23.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythValue {
    /// Qualified Pyth underlying ticker (e.g. `"Metal.XAU/USD"`).
    pub underlying_ticker: String,
    /// USD value formatted to 8 decimal places.
    pub value_usd: String,
    /// Pyth source timestamp (unix ms).
    pub source_ts_ms: i64,
    /// When Kalshi received the Pyth update (unix ms).
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

/// Response to the `underlying_list` action on a `pyth_value` subscription:
/// underlying tickers observed on the Pyth stream in the last two hours.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingList {
    pub underlying_tickers: Vec<String>,
}

/// Borrowed version of [`WsPythUnderlyingList`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsPythUnderlyingListRef<'a> {
    #[serde(borrow)]
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

#[cfg(test)]
mod tests {
    use crate::ws::types::{WsDataMessageRef, WsDataMessageV2, WsMessageRef, WsMessageV2};

    #[test]
    fn pyth_value_parses_owned_and_borrowed() {
        let json = r#"{
            "type": "pyth_value",
            "sid": 5,
            "seq": 1,
            "msg": {
                "underlying_ticker": "Metal.XAU/USD",
                "value_usd": "2650.12345678",
                "source_ts_ms": 1700000000000,
                "received_at": 1700000000005
            }
        }"#;

        let owned: WsMessageV2 = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match owned {
            WsMessageV2::Data(WsDataMessageV2::PythValue { sid, seq, msg }) => {
                assert_eq!(sid, Some(5));
                assert_eq!(seq, Some(1));
                assert_eq!(msg.underlying_ticker, "Metal.XAU/USD");
                assert_eq!(msg.value_usd, "2650.12345678");
            }
            other => panic!("unexpected message: {other:?}"),
        }

        let borrowed: WsMessageRef = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        match borrowed {
            WsMessageRef::Data(WsDataMessageRef::PythValue { msg, .. }) => {
                assert_eq!(msg.underlying_ticker, "Metal.XAU/USD");
                assert_eq!(msg.source_ts_ms, 1700000000000);
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }

    #[test]
    fn pyth_value_underlying_list_parses() {
        let json = r#"{
            "type": "pyth_value_underlying_list",
            "sid": 5,
            "seq": 2,
            "msg": {"underlying_tickers": ["Metal.XAU/USD", "Crypto.BTC/USD"]}
        }"#;

        let owned: WsMessageV2 = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match owned {
            WsMessageV2::Data(WsDataMessageV2::PythValueUnderlyingList { msg, .. }) => {
                assert_eq!(
                    msg.underlying_tickers,
                    vec!["Metal.XAU/USD".to_string(), "Crypto.BTC/USD".to_string()]
                );
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }
}
