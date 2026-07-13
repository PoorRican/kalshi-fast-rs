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

#[derive(Debug)]
pub enum WsEvent<M = WsMessageV2> {
    Message(M),
    Raw(WsRawEvent),
    Reconnected { attempt: u32 },
    Disconnected { error: KalshiError },
}

#[derive(Debug, Clone)]
pub struct WsEventReceiver<M = WsMessageV2> {
    inner: Arc<Mutex<mpsc::Receiver<WsEvent<M>>>>,
}

impl<M> WsEventReceiver<M> {
    pub(crate) fn new(rx: mpsc::Receiver<WsEvent<M>>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn next(&self) -> Option<WsEvent<M>> {
        let mut rx = self.inner.lock().await;
        rx.recv().await
    }
}
