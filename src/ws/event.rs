use crate::error::KalshiError;
use crate::ws::types::{WsMessageV2, WsRawEvent};

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

/// An event delivered by the WebSocket reader.
#[derive(Debug)]
pub enum WsEvent<M = WsMessageV2> {
    /// A decoded WebSocket message.
    Message(M),
    /// The original, unparsed WebSocket payload.
    Raw(WsRawEvent),
    /// The reader reconnected after a disconnection.
    Reconnected { attempt: u32 },
    /// The reader disconnected and could not reconnect.
    Disconnected { error: KalshiError },
}

#[cfg(feature = "timed-reader")]
/// An owned WebSocket event with the instant it became available to the reader.
#[derive(Debug)]
pub struct WsTimedEvent<M = WsMessageV2> {
    pub event: WsEvent<M>,
    pub available_at: tokio::time::Instant,
}

#[cfg(feature = "timed-reader")]
pub(crate) type ReaderItem<M = WsMessageV2> = WsTimedEvent<M>;
#[cfg(not(feature = "timed-reader"))]
pub(crate) type ReaderItem<M = WsMessageV2> = WsEvent<M>;

#[derive(Debug, Clone)]
pub struct WsEventReceiver<M = WsMessageV2> {
    inner: Arc<Mutex<mpsc::Receiver<ReaderItem<M>>>>,
}

impl<M> WsEventReceiver<M> {
    pub(crate) fn new(rx: mpsc::Receiver<ReaderItem<M>>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn next(&self) -> Option<WsEvent<M>> {
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
    pub async fn next_timed(&self) -> Option<WsTimedEvent<M>> {
        let mut rx = self.inner.lock().await;
        rx.recv().await
    }
}
