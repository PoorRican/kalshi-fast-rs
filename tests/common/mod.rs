use kalshi_fast::{
    EventData, GetEventsParams, GetHistoricalMarketsParams, GetMarketsParams,
    GetMultivariateEventsParams, GetSeriesListParams, KalshiAuth, KalshiEnvironment,
    KalshiRestClient, KalshiWsLowLevelClient, Market, MarketStatusQuery, RateLimitConfig,
    RetryConfig, WsChannelV2, WsDataMessageV2, WsMessageV2, WsSubscriptionParamsV2,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::{Instant, timeout};

pub const TEST_TIMEOUT: Duration = Duration::from_secs(30);
pub const CHANNEL_TIMEOUT: Duration = Duration::from_secs(30);

#[allow(dead_code)]
pub fn load_env() {
    dotenvy::from_filename(".env.test").ok();
}

// integration tests cannot import from unit tests
#[allow(dead_code)]
pub fn load_auth() -> KalshiAuth {
    dotenvy::from_filename(".env.test").ok();

    let key_id = std::env::var("KALSHI_KEY_ID").expect("KALSHI_KEY_ID required");

    if let Ok(pem_content) = std::env::var("KALSHI_PRIVATE_KEY") {
        let pem_content = pem_content.replace("\\n", "\n");
        KalshiAuth::from_pem_str(key_id, &pem_content).expect("load auth from KALSHI_PRIVATE_KEY")
    } else {
        let pem_path = std::env::var("KALSHI_PRIVATE_KEY_PATH")
            .expect("KALSHI_PRIVATE_KEY or KALSHI_PRIVATE_KEY_PATH required");
        KalshiAuth::from_pem_file(key_id, pem_path).expect("load auth from KALSHI_PRIVATE_KEY_PATH")
    }
}

pub fn demo_env() -> KalshiEnvironment {
    KalshiEnvironment::demo()
}

#[allow(dead_code)]
pub fn prod_env() -> KalshiEnvironment {
    KalshiEnvironment::production()
}

#[allow(dead_code)]
pub async fn connect_ws(env: KalshiEnvironment) -> KalshiWsLowLevelClient {
    load_env();
    let auth = load_auth();

    timeout(TEST_TIMEOUT, async {
        KalshiWsLowLevelClient::connect_authenticated(env, auth).await
    })
    .await
    .expect("timed out connecting to websocket")
    .expect("failed to connect to websocket")
}

#[allow(dead_code)]
pub async fn connect_demo_ws() -> KalshiWsLowLevelClient {
    connect_ws(demo_env()).await
}

#[allow(dead_code)]
pub async fn connect_prod_ws() -> KalshiWsLowLevelClient {
    connect_ws(prod_env()).await
}

#[allow(dead_code)]
pub fn rest_client(env: KalshiEnvironment) -> KalshiRestClient {
    KalshiRestClient::builder(env)
        .with_rate_limit_config(RateLimitConfig {
            read_rps: 4,
            write_rps: 2,
            ..Default::default()
        })
        .with_retry_config(RetryConfig {
            max_retries: 5,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(6),
            jitter: 0.0,
            retry_non_idempotent: false,
        })
        .build()
        .expect("build live test client")
}

#[allow(dead_code)]
pub fn demo_client() -> KalshiRestClient {
    rest_client(demo_env())
}

#[allow(dead_code)]
pub fn prod_client() -> KalshiRestClient {
    rest_client(prod_env())
}

#[allow(dead_code)]
pub fn demo_auth_client(auth: KalshiAuth) -> KalshiRestClient {
    KalshiRestClient::builder(demo_env())
        .with_auth(auth)
        .with_rate_limit_config(RateLimitConfig {
            read_rps: 4,
            write_rps: 2,
            ..Default::default()
        })
        .with_retry_config(RetryConfig {
            max_retries: 5,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(6),
            jitter: 0.0,
            retry_non_idempotent: false,
        })
        .build()
        .expect("build authenticated live test client")
}

#[allow(dead_code)]
pub async fn first_open_market_ticker(client: &KalshiRestClient) -> String {
    let markets_resp = timeout(
        TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(1),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timed out fetching markets")
    .expect("failed to fetch markets");

    markets_resp
        .markets
        .into_iter()
        .next()
        .map(|market| market.ticker)
        .expect("environment returned no open markets")
}

#[allow(dead_code)]
pub async fn first_open_demo_market_ticker() -> String {
    first_open_market_ticker(&demo_client()).await
}

#[allow(dead_code)]
pub async fn active_demo_market_ticker_via_trade() -> String {
    active_demo_market_tickers_via_trade(1).await.remove(0)
}

#[allow(dead_code)]
pub async fn active_demo_market_tickers_via_trade(target_count: usize) -> Vec<String> {
    let mut ws = connect_demo_ws().await;

    let subscribe_id = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Trade],
            ..Default::default()
        })
        .await
        .expect("trade subscribe failed while locating active demo market");

    wait_for_subscribed(&mut ws, subscribe_id).await;

    let deadline = Instant::now() + CHANNEL_TIMEOUT;
    let mut tickers = Vec::new();

    while tickers.len() < target_count {
        let now = Instant::now();
        assert!(
            now < deadline,
            "timed out waiting for {target_count} active trade-backed demo markets"
        );

        let trade = require_trade_data(
            wait_for_message(&mut ws, deadline - now, |msg| {
                matches!(msg, WsMessageV2::Data(WsDataMessageV2::Trade { .. }))
            })
            .await,
        );

        if !tickers.iter().any(|ticker| ticker == &trade.market_ticker) {
            tickers.push(trade.market_ticker);
        }
    }

    tickers
}

#[allow(dead_code)]
pub async fn wait_for_subscribed(ws: &mut KalshiWsLowLevelClient, command_id: u64) -> u64 {
    let message = wait_for_message(
        ws,
        CHANNEL_TIMEOUT,
        |msg| matches!(msg, WsMessageV2::Subscribed { id: Some(id), .. } if *id == command_id),
    )
    .await;

    match message {
        WsMessageV2::Subscribed { sid: Some(sid), .. } => sid,
        other => panic!("expected subscribed response with sid, got {other:?}"),
    }
}

#[allow(dead_code)]
pub async fn wait_for_message<F>(
    ws: &mut KalshiWsLowLevelClient,
    timeout_window: Duration,
    mut predicate: F,
) -> WsMessageV2
where
    F: FnMut(&WsMessageV2) -> bool,
{
    let deadline = Instant::now() + timeout_window;

    loop {
        let now = Instant::now();
        assert!(now < deadline, "timed out waiting for websocket message");
        let remaining = deadline - now;

        let message = timeout(remaining, ws.next_message_v2())
            .await
            .expect("timed out waiting for websocket frame")
            .expect("failed to read websocket message");

        if let WsMessageV2::Error { error, .. } = &message {
            panic!("received websocket error: {:?}", error);
        }

        if predicate(&message) {
            return message;
        }
    }
}

#[allow(dead_code)]
pub fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_secs() as i64
}

#[allow(dead_code)]
pub fn now_minus_days(days: u64) -> i64 {
    now_ts() - (days as i64) * 86_400
}

#[allow(dead_code)]
pub async fn pick_active_market(client: &KalshiRestClient) -> Market {
    let resp = timeout(
        TEST_TIMEOUT,
        client.get_markets(GetMarketsParams {
            limit: Some(10),
            status: Some(MarketStatusQuery::Open),
            ..Default::default()
        }),
    )
    .await
    .expect("timed out fetching active markets")
    .expect("failed to fetch active markets");

    resp.markets
        .into_iter()
        .next()
        .expect("demo environment returned no open markets")
}

#[allow(dead_code)]
pub async fn pick_settled_market(client: &KalshiRestClient) -> Market {
    let resp = timeout(
        TEST_TIMEOUT,
        client.get_historical_markets(GetHistoricalMarketsParams {
            limit: Some(5),
            ..Default::default()
        }),
    )
    .await
    .expect("timed out fetching historical markets")
    .expect("failed to fetch historical markets");

    resp.markets
        .into_iter()
        .next()
        .expect("demo environment returned no historical markets")
}

#[allow(dead_code)]
pub async fn pick_event_with_series(client: &KalshiRestClient) -> (String, String) {
    let series_resp = timeout(
        TEST_TIMEOUT,
        client.get_series_list(GetSeriesListParams::default()),
    )
    .await
    .expect("timed out fetching series list")
    .expect("failed to fetch series list");

    for series in series_resp.series {
        let events = timeout(
            TEST_TIMEOUT,
            client.get_events(GetEventsParams {
                limit: Some(1),
                series_ticker: Some(series.ticker.clone()),
                ..Default::default()
            }),
        )
        .await
        .expect("timed out fetching events for series")
        .expect("failed to fetch events for series");

        if let Some(event) = events.events.into_iter().next() {
            return (series.ticker, event.event_ticker);
        }
    }

    panic!("demo environment had no series with events");
}

#[allow(dead_code)]
pub async fn pick_multivariate_event(client: &KalshiRestClient) -> EventData {
    let resp = timeout(
        TEST_TIMEOUT,
        client.get_multivariate_events(GetMultivariateEventsParams {
            limit: Some(5),
            ..Default::default()
        }),
    )
    .await
    .expect("timed out fetching multivariate events")
    .expect("failed to fetch multivariate events");

    resp.events
        .into_iter()
        .next()
        .expect("demo environment returned no multivariate events")
}

#[allow(dead_code)]
pub fn require_ticker_data(message: WsMessageV2) -> kalshi_fast::WsTicker {
    match message {
        WsMessageV2::Data(WsDataMessageV2::Ticker { msg, .. }) => msg,
        other => panic!("expected ticker data message, got {other:?}"),
    }
}

#[allow(dead_code)]
pub fn require_trade_data(message: WsMessageV2) -> kalshi_fast::WsTrade {
    match message {
        WsMessageV2::Data(WsDataMessageV2::Trade { msg, .. }) => msg,
        other => panic!("expected trade data message, got {other:?}"),
    }
}
