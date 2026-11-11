use crate::KalshiError;
use crate::types::ErrorResponse;
use rand::random;
use reqwest::header::{HeaderMap, RETRY_AFTER};
use reqwest::{Method, StatusCode};
use std::time::SystemTime;
use tokio::time::Duration;

/// HTTP retry policy for transient REST failures.
///
/// # Default
///
/// - Retries enabled for idempotent methods (`GET`, `DELETE`)
/// - Retries disabled for non-idempotent methods (`POST`, `PUT`, `PATCH`)
/// - `max_retries = 3` (attempts after the initial request)
/// - Exponential backoff with jitter
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum retries after the initial attempt.
    pub max_retries: u32,
    /// Initial backoff delay.
    pub base_delay: Duration,
    /// Maximum backoff delay.
    pub max_delay: Duration,
    /// Jitter factor in range `[0.0, 1.0]`.
    pub jitter: f64,
    /// Whether to retry non-idempotent methods.
    pub retry_non_idempotent: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(5),
            jitter: 0.2,
            retry_non_idempotent: false,
        }
    }
}

impl RetryConfig {
    pub(crate) fn allows_method(&self, method: &Method) -> bool {
        matches!(*method, Method::GET | Method::DELETE)
            || (self.retry_non_idempotent
                && matches!(*method, Method::POST | Method::PUT | Method::PATCH))
    }

    pub(crate) fn backoff_delay(&self, retry_number: u32) -> Duration {
        let exp = 2f64.powi(retry_number.saturating_sub(1) as i32);
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

pub(crate) fn build_http_error(
    status: reqwest::StatusCode,
    bytes: &[u8],
    request_id: Option<String>,
) -> KalshiError {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct WrappedErrorBody {
        error: ErrorResponse,
    }

    let raw_body = String::from_utf8_lossy(bytes).to_string();
    let normalize = |error: ErrorResponse| {
        if error.code.is_some() || error.message.is_some() || error.details.is_some() {
            Some(error)
        } else {
            None
        }
    };
    let api_error = serde_json::from_slice::<WrappedErrorBody>(bytes)
        .ok()
        .and_then(|wrapped| normalize(wrapped.error))
        .or_else(|| {
            serde_json::from_slice::<ErrorResponse>(bytes)
                .ok()
                .and_then(normalize)
        });
    KalshiError::Http {
        status,
        api_error,
        raw_body,
        request_id,
    }
}

pub(crate) fn retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::REQUEST_TIMEOUT
            | StatusCode::TOO_EARLY
            | StatusCode::TOO_MANY_REQUESTS
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

/// 429 codes that report an exhausted quota rather than request-rate
/// limiting. Kalshi documents that retrying does not help for these.
const QUOTA_EXHAUSTED_CODES: [&str; 2] = [
    "open_rfq_limit_exceeded",
    "weather_point_market_limit_exceeded",
];

/// Whether a failed response may be retried, given its status and body.
pub(crate) fn retryable_response(status: StatusCode, body: &[u8]) -> bool {
    if !retryable_status(status) {
        return false;
    }
    if status != StatusCode::TOO_MANY_REQUESTS {
        return true;
    }
    #[derive(serde::Deserialize)]
    struct Code<'a> {
        #[serde(borrow, default)]
        code: Option<std::borrow::Cow<'a, str>>,
        #[serde(borrow, default)]
        error: Option<Box<Code<'a>>>,
    }
    let code = serde_json::from_slice::<Code<'_>>(body)
        .ok()
        .and_then(|body| {
            body.code
                .or_else(|| body.error.and_then(|inner| inner.code))
        });
    !code.is_some_and(|code| QUOTA_EXHAUSTED_CODES.contains(&code.as_ref()))
}

pub(crate) fn retryable_reqwest_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

pub(crate) fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?;
    let text = value.to_str().ok()?.trim();

    if let Ok(seconds) = text.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let ts = httpdate::parse_http_date(text).ok()?;
    let now = SystemTime::now();
    let delta = ts.duration_since(now).ok()?;
    Some(delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quota_exhausted_429_is_not_retryable() {
        let tmr = StatusCode::TOO_MANY_REQUESTS;
        assert!(retryable_response(tmr, b""));
        assert!(retryable_response(tmr, br#"{"code":"too_many_requests"}"#));
        assert!(!retryable_response(
            tmr,
            br#"{"code":"open_rfq_limit_exceeded"}"#
        ));
        assert!(!retryable_response(
            tmr,
            br#"{"error":{"code":"weather_point_market_limit_exceeded","message":"m"}}"#
        ));
        assert!(retryable_response(
            StatusCode::BAD_GATEWAY,
            br#"{"code":"open_rfq_limit_exceeded"}"#
        ));
        assert!(!retryable_response(StatusCode::BAD_REQUEST, b""));
    }
}
