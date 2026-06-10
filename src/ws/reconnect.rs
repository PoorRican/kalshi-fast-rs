#[cfg(doc)]
use crate::ws::KalshiWsClient;

use rand::random;
use tokio::time::Duration;

/// Configuration for automatic WebSocket reconnection in [`KalshiWsClient`].
///
/// Uses exponential backoff with jitter. The delay for attempt *n* is
/// `min(base_delay * 2^(n-1), max_delay)` ± `jitter`.
///
/// # Default
///
/// | Field | Value |
/// |-------|-------|
/// | `max_retries` | `None` (unlimited) |
/// | `base_delay` | 250 ms |
/// | `max_delay` | 30 s |
/// | `jitter` | 0.2 |
/// | `resubscribe` | `true` |
/// | `ping_interval` | `None` (disabled) |
/// | `pong_timeout` | 10 s |
#[derive(Debug, Clone)]
pub struct WsReconnectConfig {
    /// Maximum reconnection attempts. `None` means unlimited.
    pub max_retries: Option<u32>,
    /// Initial backoff delay (doubles each attempt).
    pub base_delay: Duration,
    /// Upper bound on backoff delay.
    pub max_delay: Duration,
    /// Random jitter factor applied to each delay (0.0–1.0).
    pub jitter: f64,
    /// Whether to resubscribe to active channels after reconnecting.
    pub resubscribe: bool,
    /// Interval between client-originated `Ping` frames. `None` disables
    /// keepalive, leaving reads without a deadline.
    pub ping_interval: Option<Duration>,
    /// How long to wait for any inbound frame after a ping before treating
    /// the connection as dead and reconnecting. Only meaningful when
    /// `ping_interval` is set.
    pub pong_timeout: Duration,
}

impl Default for WsReconnectConfig {
    fn default() -> Self {
        Self {
            max_retries: None,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(30),
            jitter: 0.2,
            resubscribe: true,
            ping_interval: None,
            pong_timeout: Duration::from_secs(10),
        }
    }
}

impl WsReconnectConfig {
    pub(crate) fn backoff_delay(&self, attempt: u32) -> Duration {
        let exp = 2f64.powi(attempt.saturating_sub(1) as i32);
        let mut delay = self.base_delay.mul_f64(exp);
        if delay > self.max_delay {
            delay = self.max_delay;
        }
        let jitter = self.jitter.clamp(0.0, 1.0);
        if jitter > 0.0 {
            let factor = 1.0 - jitter + random::<f64>() * (2.0 * jitter);
            delay = delay.mul_f64(factor);
        }
        delay
    }
}
