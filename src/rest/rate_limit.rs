use reqwest::Method;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// Per-second rate limits for read (GET) and write (POST/DELETE) requests.
///
/// The Kalshi API enforces separate rate limits for reads and writes.
/// Set either value to `0` to disable throttling for that category.
///
/// # Default
///
/// The default matches the **Basic** tier: 20 read RPS, 10 write RPS, 1 balance RPS.
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    /// Maximum GET requests per second (0 = unlimited).
    pub read_rps: u32,
    /// Maximum POST/DELETE requests per second (0 = unlimited).
    pub write_rps: u32,
    /// Maximum `GET /portfolio/balance` requests per second (0 = unlimited).
    ///
    /// The balance endpoint has its own bucket, separate from the read budget: 1/s on Basic and
    /// Advanced, 5/s on higher tiers, with no burst.
    pub balance_rps: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        // Basic tier defaults.
        Self {
            read_rps: 20,
            write_rps: 10,
            balance_rps: 1,
        }
    }
}

/// Named rate-limit tier matching Kalshi's published API tiers.
///
/// Pass to [`crate::KalshiRestClient::with_rate_limit_tier`] for quick configuration.
#[derive(Debug, Clone, Copy)]
pub enum RateLimitTier {
    /// 20 read RPS, 10 write RPS, 1 balance RPS.
    Basic,
}

impl RateLimitTier {
    pub(crate) fn config(self) -> RateLimitConfig {
        match self {
            RateLimitTier::Basic => RateLimitConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RateLimitKind {
    Read,
    Write,
    Balance,
}

pub(crate) fn rate_limit_kind(method: &Method, full_path: &str) -> RateLimitKind {
    if *method == Method::GET {
        if full_path.ends_with("/portfolio/balance") {
            RateLimitKind::Balance
        } else {
            RateLimitKind::Read
        }
    } else {
        RateLimitKind::Write
    }
}

#[derive(Debug)]
pub(crate) struct RateLimiter {
    read: Mutex<Instant>,
    write: Mutex<Instant>,
    balance: Mutex<Instant>,
    read_interval: Duration,
    write_interval: Duration,
    balance_interval: Duration,
}

impl RateLimiter {
    pub(crate) fn new(config: RateLimitConfig) -> Self {
        let read_interval = if config.read_rps == 0 {
            Duration::from_secs(0)
        } else {
            Duration::from_secs_f64(1.0 / config.read_rps as f64)
        };
        let write_interval = if config.write_rps == 0 {
            Duration::from_secs(0)
        } else {
            Duration::from_secs_f64(1.0 / config.write_rps as f64)
        };

        let balance_interval = if config.balance_rps == 0 {
            Duration::from_secs(0)
        } else {
            Duration::from_secs_f64(1.0 / config.balance_rps as f64)
        };

        let now = Instant::now();
        Self {
            read: Mutex::new(now - read_interval),
            write: Mutex::new(now - write_interval),
            balance: Mutex::new(now - balance_interval),
            read_interval,
            write_interval,
            balance_interval,
        }
    }

    pub(crate) async fn wait(&self, kind: RateLimitKind) {
        let (lock, interval) = match kind {
            RateLimitKind::Read => (&self.read, self.read_interval),
            RateLimitKind::Write => (&self.write, self.write_interval),
            RateLimitKind::Balance => (&self.balance, self.balance_interval),
        };

        if interval.is_zero() {
            return;
        }

        let mut last = lock.lock().await;
        let now = Instant::now();
        let scheduled = if *last + interval > now {
            *last + interval
        } else {
            now
        };
        *last = scheduled;
        drop(last);

        if scheduled > now {
            tokio::time::sleep(scheduled - now).await;
        }
    }
}
