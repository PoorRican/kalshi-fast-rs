use url::Url;

/// REST API prefix (Kalshi Exchange REST v2).
pub const REST_PREFIX: &str = "/trade-api/v2";

/// WebSocket path (Kalshi Exchange WS v2) used for signing:
/// timestamp + "GET" + "/trade-api/ws/v2"
pub const WS_PATH: &str = "/trade-api/ws/v2";

const DEMO_REST_HOST: &str = "external-api.demo.kalshi.co";
const DEMO_WS_HOST: &str = "external-api-ws.demo.kalshi.co";
const LIVE_REST_HOST: &str = "external-api.kalshi.com";
const LIVE_WS_HOST: &str = "external-api-ws.kalshi.com";

#[derive(Debug, Clone)]
pub struct KalshiEnvironment {
    /// Origin only, e.g. <https://demo-api.kalshi.co> (Url for reqwest compatibility).
    pub rest_origin: Url,
    /// Pre-computed WS URL string for direct use with tokio-tungstenite
    pub ws_url: String,
}

impl KalshiEnvironment {
    /// Demo environment (dedicated external API hosts, active since 2026-05-07).
    /// REST origin: <https://external-api.demo.kalshi.co>
    /// WS URL: `wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2`
    pub fn demo() -> Self {
        Self {
            rest_origin: Url::parse(&format!("https://{DEMO_REST_HOST}/"))
                .expect("valid demo REST origin"),
            ws_url: format!("wss://{DEMO_WS_HOST}{WS_PATH}"),
        }
    }

    /// Production environment (dedicated external API hosts, active since 2026-05-07).
    /// REST origin: <https://external-api.kalshi.com>
    /// WS URL: `wss://external-api-ws.kalshi.com/trade-api/ws/v2`
    pub fn production() -> Self {
        Self {
            rest_origin: Url::parse(&format!("https://{LIVE_REST_HOST}/"))
                .expect("valid prod REST origin"),
            ws_url: format!("wss://{LIVE_WS_HOST}{WS_PATH}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_urls_are_valid() {
        let env = KalshiEnvironment::demo();
        assert!(env.rest_origin.as_str().starts_with("https://"));
        // Validate ws_url by parsing it
        let _ = Url::parse(&env.ws_url).expect("valid demo WS URL");
    }

    #[test]
    fn production_urls_are_valid() {
        let env = KalshiEnvironment::production();
        assert!(env.rest_origin.as_str().starts_with("https://"));
        let _ = Url::parse(&env.ws_url).expect("valid prod WS URL");
    }
}
