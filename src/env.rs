use url::Url;

/// REST API prefix (Kalshi Exchange REST v2).
pub const REST_PREFIX: &str = "/trade-api/v2";

/// WebSocket path (Kalshi Exchange WS v2) used for signing:
/// timestamp + "GET" + "/trade-api/ws/v2"
pub const WS_PATH: &str = "/trade-api/ws/v2";

/// WebSocket path for margin/perpetuals — separate host + `/margin` suffix.
pub const MARGIN_WS_PATH: &str = "/trade-api/ws/v2/margin";

const DEMO_REST_HOST: &str = "external-api.demo.kalshi.co";
const DEMO_WS_HOST: &str = "external-api-ws.demo.kalshi.co";
const DEMO_MARGIN_WS_HOST: &str = "external-api-margin-ws.demo.kalshi.co";
const DEMO_WARP_WS_HOST: &str = "warp-ws.demo.kalshi.co";
const DEMO_MARGIN_WARP_WS_HOST: &str = "margin-warp-ws.demo.kalshi.co";
const LIVE_WARP_WS_HOST: &str = "warp-ws.kalshi.com";
const LIVE_MARGIN_WARP_WS_HOST: &str = "margin-warp-ws.kalshi.com";
const DEMO_USER_WS_HOST: &str = "user-ws.demo.kalshi.co";
const DEMO_MARGIN_USER_WS_HOST: &str = "margin-user-ws.demo.kalshi.co";
const LIVE_USER_WS_HOST: &str = "user-ws.kalshi.com";
const LIVE_MARGIN_USER_WS_HOST: &str = "margin-user-ws.kalshi.com";
const LIVE_REST_HOST: &str = "external-api.kalshi.com";
const LIVE_WS_HOST: &str = "external-api-ws.kalshi.com";
const LIVE_MARGIN_WS_HOST: &str = "external-api-margin-ws.kalshi.com";

#[derive(Debug, Clone)]
pub struct KalshiEnvironment {
    /// Origin only, e.g. <https://demo-api.kalshi.co> (Url for reqwest compatibility).
    pub rest_origin: Url,
    /// Pre-computed WS URL string for direct use with tokio-tungstenite (event contracts).
    pub ws_url: String,
    /// Pre-computed WS URL string for margin/perpetuals WebSocket connection.
    pub margin_ws_url: String,
}

impl KalshiEnvironment {
    /// Demo environment (dedicated external API hosts, active since 2026-05-07).
    /// REST origin: <https://external-api.demo.kalshi.co>
    /// WS URL: `wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2`
    pub fn demo() -> Self {
        let ws_url = format!("wss://{DEMO_WS_HOST}{WS_PATH}");
        Self {
            rest_origin: Url::parse(&format!("https://{DEMO_REST_HOST}/"))
                .expect("valid demo REST origin"),
            margin_ws_url: format!("wss://{DEMO_MARGIN_WS_HOST}{MARGIN_WS_PATH}"),
            ws_url,
        }
    }

    /// Production environment (dedicated external API hosts, active since 2026-05-07).
    /// REST origin: <https://external-api.kalshi.com>
    /// WS URL: `wss://external-api-ws.kalshi.com/trade-api/ws/v2`
    pub fn production() -> Self {
        let ws_url = format!("wss://{LIVE_WS_HOST}{WS_PATH}");
        Self {
            rest_origin: Url::parse(&format!("https://{LIVE_REST_HOST}/"))
                .expect("valid prod REST origin"),
            margin_ws_url: format!("wss://{LIVE_MARGIN_WS_HOST}{MARGIN_WS_PATH}"),
            ws_url,
        }
    }
}

impl KalshiEnvironment {
    /// Lower-latency Predictions per-user WebSocket URL (`user-ws`). It serves
    /// `fill` and `user_orders` today; Kalshi will add more per-user channels
    /// over time. Assign it to `ws_url` on a dedicated connection. Signing path
    /// and auth are unchanged.
    pub fn user_ws_url(&self) -> String {
        let host = if self.ws_url.contains(DEMO_WS_HOST) {
            DEMO_USER_WS_HOST
        } else {
            LIVE_USER_WS_HOST
        };
        format!("wss://{host}{WS_PATH}")
    }

    /// Margin counterpart of [`Self::user_ws_url`] (`margin-user-ws`).
    pub fn margin_user_ws_url(&self) -> String {
        let host = if self.margin_ws_url.contains(DEMO_MARGIN_WS_HOST) {
            DEMO_MARGIN_USER_WS_HOST
        } else {
            LIVE_MARGIN_USER_WS_HOST
        };
        format!("wss://{host}{MARGIN_WS_PATH}")
    }

    /// Deprecated low-latency Predictions WebSocket URL serving only `fill` and
    /// `user_orders`; any other channel returns error code 8.
    #[deprecated(
        since = "0.12.0",
        note = "Kalshi deprecated the warp-ws hosts; use user_ws_url()"
    )]
    pub fn warp_ws_url(&self) -> String {
        let host = if self.ws_url.contains(DEMO_WS_HOST) {
            DEMO_WARP_WS_HOST
        } else {
            LIVE_WARP_WS_HOST
        };
        format!("wss://{host}{WS_PATH}")
    }

    /// Deprecated margin counterpart of the warp host (`fill`/`user_orders` only).
    #[deprecated(
        since = "0.12.0",
        note = "Kalshi deprecated the margin-warp-ws hosts; use margin_user_ws_url()"
    )]
    pub fn margin_warp_ws_url(&self) -> String {
        let host = if self.margin_ws_url.contains(DEMO_MARGIN_WS_HOST) {
            DEMO_MARGIN_WARP_WS_HOST
        } else {
            LIVE_MARGIN_WARP_WS_HOST
        };
        format!("wss://{host}{MARGIN_WS_PATH}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_urls_are_valid() {
        let env = KalshiEnvironment::demo();
        assert!(env.rest_origin.as_str().starts_with("https://"));
        let _ = Url::parse(&env.ws_url).expect("valid demo WS URL");
        let _ = Url::parse(&env.margin_ws_url).expect("valid demo margin WS URL");
        assert!(
            env.margin_ws_url.contains("margin-ws"),
            "margin WS URL should use margin-ws host"
        );
        assert!(
            env.margin_ws_url.ends_with("/margin"),
            "margin WS URL should end with /margin"
        );
    }

    #[test]
    fn production_urls_are_valid() {
        let env = KalshiEnvironment::production();
        assert!(env.rest_origin.as_str().starts_with("https://"));
        let _ = Url::parse(&env.ws_url).expect("valid prod WS URL");
        let _ = Url::parse(&env.margin_ws_url).expect("valid prod margin WS URL");
        assert!(
            env.margin_ws_url.contains("margin-ws"),
            "margin WS URL should use margin-ws host"
        );
        assert!(
            env.margin_ws_url.ends_with("/margin"),
            "margin WS URL should end with /margin"
        );
    }

    #[test]
    fn user_urls_match_published_hosts() {
        let d = KalshiEnvironment::demo();
        assert_eq!(
            d.user_ws_url(),
            "wss://user-ws.demo.kalshi.co/trade-api/ws/v2"
        );
        assert_eq!(
            d.margin_user_ws_url(),
            "wss://margin-user-ws.demo.kalshi.co/trade-api/ws/v2/margin"
        );
        let p = KalshiEnvironment::production();
        assert_eq!(p.user_ws_url(), "wss://user-ws.kalshi.com/trade-api/ws/v2");
        assert_eq!(
            p.margin_user_ws_url(),
            "wss://margin-user-ws.kalshi.com/trade-api/ws/v2/margin"
        );
    }

    #[test]
    #[allow(deprecated)]
    fn warp_urls_match_published_hosts() {
        let d = KalshiEnvironment::demo();
        assert_eq!(
            d.warp_ws_url(),
            "wss://warp-ws.demo.kalshi.co/trade-api/ws/v2"
        );
        assert_eq!(
            d.margin_warp_ws_url(),
            "wss://margin-warp-ws.demo.kalshi.co/trade-api/ws/v2/margin"
        );
        let p = KalshiEnvironment::production();
        assert_eq!(p.warp_ws_url(), "wss://warp-ws.kalshi.com/trade-api/ws/v2");
        assert_eq!(
            p.margin_warp_ws_url(),
            "wss://margin-warp-ws.kalshi.com/trade-api/ws/v2/margin"
        );
    }
}
