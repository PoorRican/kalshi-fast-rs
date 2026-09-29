use crate::error::KalshiError;
use crate::ws::types::{WsMessageV2, WsRawEvent};
#[cfg(doc)]
use crate::ws::{KalshiWsClient, WsReconnectConfig};

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

#[derive(Debug, Clone, Copy)]
pub enum WsReaderMode {
    Owned,
    Raw,
}

#[derive(Debug, Clone)]
pub struct WsReaderConfig {
    pub buffer_size: usize,
    pub mode: WsReaderMode,
}

impl Default for WsReaderConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1024,
            mode: WsReaderMode::Owned,
        }
    }
}

/// Events emitted by [`KalshiWsClient::next_event`].
///
/// The high-level client wraps every raw WebSocket message as well as
/// connection lifecycle transitions into this enum.
#[derive(Debug)]
pub enum WsEvent {
    /// A parsed WebSocket message (data, ack, error, etc.).
    Message(WsMessageV2),
    Raw(WsRawEvent),
    /// Connection was lost and successfully re-established.
    ///
    /// `attempt` is the 1-based retry count that succeeded.
    /// If [`WsReconnectConfig::resubscribe`] is `true`, all previously
    /// active channels have already been resubscribed.
    Reconnected {
        attempt: u32,
    },
    /// Connection was lost and could not be restored within
    /// [`WsReconnectConfig::max_retries`].
    Disconnected {
        error: KalshiError,
    },
}

#[cfg(feature = "timed-reader")]
/// An owned WebSocket event with the instant it became available to the reader.
#[derive(Debug)]
pub struct WsTimedEvent {
    pub event: WsEvent,
    pub available_at: tokio::time::Instant,
}

#[cfg(feature = "timed-reader")]
pub(crate) type ReaderItem = WsTimedEvent;
#[cfg(not(feature = "timed-reader"))]
pub(crate) type ReaderItem = WsEvent;

#[derive(Debug, Clone)]
pub struct WsEventReceiver {
    inner: Arc<Mutex<mpsc::Receiver<ReaderItem>>>,
}

impl WsEventReceiver {
    pub(crate) fn new(rx: mpsc::Receiver<ReaderItem>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn next(&self) -> Option<WsEvent> {
        #[cfg(feature = "timed-reader")]
        {
            self.next_timed().await.map(|event| event.event)
        }

        #[cfg(not(feature = "timed-reader"))]
        {
            let mut rx = self.inner.lock().await;
            rx.recv().await
        }
    }

    #[cfg(feature = "timed-reader")]
    pub async fn next_timed(&self) -> Option<WsTimedEvent> {
        let mut rx = self.inner.lock().await;
        rx.recv().await
    }
}
