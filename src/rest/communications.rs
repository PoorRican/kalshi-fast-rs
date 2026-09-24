//! RFQ (Request For Quote) and Quote endpoints, plus the communications ID.
//!
//! RFQs express interest in a given market and notional. Market makers respond
//! with quotes that the RFQ creator can accept/confirm to execute a trade.
//! All endpoints require authentication.

use crate::KalshiError;
use crate::rest::account::EmptyResponse;
use crate::rest::client::KalshiRestClient;
use crate::rest::markets::MveSelectedLeg;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{FixedPointCount, FixedPointDollars, YesNo, deserialize_null_as_empty_vec};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetCommunicationsIdResponse {
    pub communications_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quote {
    pub id: String,
    pub rfq_id: String,
    pub creator_id: String,
    pub rfq_creator_id: String,
    pub market_ticker: String,
    pub contracts_fp: FixedPointCount,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    pub created_ts: String,
    pub updated_ts: String,
    pub status: String,
    #[serde(default)]
    pub accepted_side: Option<YesNo>,
    #[serde(default)]
    pub accepted_ts: Option<String>,
    #[serde(default)]
    pub confirmed_ts: Option<String>,
    #[serde(default)]
    pub executed_ts: Option<String>,
    #[serde(default)]
    pub cancelled_ts: Option<String>,
    #[serde(default)]
    pub rest_remainder: Option<bool>,
    #[serde(default)]
    pub cancellation_reason: Option<String>,
    #[serde(default)]
    pub creator_user_id: Option<String>,
    #[serde(default)]
    pub rfq_creator_user_id: Option<String>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(default)]
    pub rfq_creator_order_id: Option<String>,
    #[serde(default)]
    pub creator_order_id: Option<String>,
    #[serde(default)]
    pub yes_contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_fp: Option<FixedPointCount>,
    /// Whether the quote creator's order is post-only (visible when the caller is the quote creator).
    #[serde(default)]
    pub post_only: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RFQ {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    pub contracts_fp: FixedPointCount,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub target_cost_excludes_fees: Option<bool>,
    pub status: String,
    pub created_ts: String,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub mve_selected_legs: Vec<MveSelectedLeg>,
    #[serde(default)]
    pub rest_remainder: Option<bool>,
    #[serde(default)]
    pub cancellation_reason: Option<String>,
    #[serde(default)]
    pub creator_user_id: Option<String>,
    #[serde(default)]
    pub cancelled_ts: Option<String>,
    #[serde(default)]
    pub updated_ts: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetQuotesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Restrict the response to quotes last updated after this Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    /// Restrict the response to quotes last updated before this Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    /// Filter to quotes responding to RFQs created by the authenticated user.
    /// Pass `"self"` to enable. Added 2026-05-07.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_user_filter: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetQuotesResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub quotes: Vec<Quote>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetQuoteResponse {
    pub quote: Quote,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateQuoteRequest {
    pub rfq_id: String,
    pub yes_bid: String,
    pub no_bid: String,
    pub rest_remainder: bool,
    /// If true, the quote creator's resting order will be cancelled rather than
    /// crossed if it would take liquidity. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateQuoteResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AcceptQuoteRequest {
    pub accepted_side: YesNo,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRFQsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRFQsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub rfqs: Vec<RFQ>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRFQResponse {
    pub rfq: RFQ,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRFQRequest {
    pub market_ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_centi_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<FixedPointDollars>,
    /// Size quotes against the target cost as principal only (contracts = target
    /// cost / price), with taker fees charged on top of the target cost. By
    /// default (false/omitted) the target cost caps principal plus Kalshi fees.
    /// Only valid together with a target cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    pub rest_remainder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRFQResponse {
    pub id: String,
}

impl KalshiRestClient {
    pub async fn get_communications_id(&self) -> Result<GetCommunicationsIdResponse, KalshiError> {
        let path = Self::full_path("/communications/id");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_rfqs(&self, params: GetRFQsParams) -> Result<GetRFQsResponse, KalshiError> {
        let path = Self::full_path("/communications/rfqs");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn create_rfq(
        &self,
        body: CreateRFQRequest,
    ) -> Result<CreateRFQResponse, KalshiError> {
        let path = Self::full_path("/communications/rfqs");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn get_rfq(&self, rfq_id: &str) -> Result<GetRFQResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/rfqs/{rfq_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn delete_rfq(&self, rfq_id: &str) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/rfqs/{rfq_id}"));
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_quotes(
        &self,
        params: GetQuotesParams,
    ) -> Result<GetQuotesResponse, KalshiError> {
        let path = Self::full_path("/communications/quotes");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn create_quote(
        &self,
        body: CreateQuoteRequest,
    ) -> Result<CreateQuoteResponse, KalshiError> {
        let path = Self::full_path("/communications/quotes");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    #[deprecated(
        note = "use get_rfq_quote instead; GET /communications/quotes/{quote_id} is deprecated by the Kalshi API"
    )]
    pub async fn get_quote(&self, quote_id: &str) -> Result<GetQuoteResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/quotes/{quote_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    #[deprecated(
        note = "use delete_rfq_quote instead; DELETE /communications/quotes/{quote_id} is deprecated by the Kalshi API"
    )]
    pub async fn delete_quote(&self, quote_id: &str) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/quotes/{quote_id}"));
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    #[deprecated(
        note = "use accept_rfq_quote instead; PUT /communications/quotes/{quote_id}/accept is deprecated by the Kalshi API"
    )]
    pub async fn accept_quote(
        &self,
        quote_id: &str,
        body: AcceptQuoteRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/quotes/{quote_id}/accept"));
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    #[deprecated(
        note = "use confirm_rfq_quote instead; PUT /communications/quotes/{quote_id}/confirm is deprecated by the Kalshi API"
    )]
    pub async fn confirm_quote(&self, quote_id: &str) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/quotes/{quote_id}/confirm"));
        let body = EmptyResponse::default();
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// RFQ-scoped equivalent of [`KalshiRestClient::get_quote`]. Prefer this
    /// method; the quote-ID-only endpoint is deprecated by the Kalshi API.
    pub async fn get_rfq_quote(
        &self,
        rfq_id: &str,
        quote_id: &str,
    ) -> Result<GetQuoteResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/rfqs/{rfq_id}/quotes/{quote_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// RFQ-scoped equivalent of [`KalshiRestClient::delete_quote`]. Prefer this
    /// method; the quote-ID-only endpoint is deprecated by the Kalshi API.
    pub async fn delete_rfq_quote(
        &self,
        rfq_id: &str,
        quote_id: &str,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/communications/rfqs/{rfq_id}/quotes/{quote_id}"));
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// RFQ-scoped equivalent of [`KalshiRestClient::accept_quote`]. Prefer this
    /// method; the quote-ID-only endpoint is deprecated by the Kalshi API.
    pub async fn accept_rfq_quote(
        &self,
        rfq_id: &str,
        quote_id: &str,
        body: AcceptQuoteRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/communications/rfqs/{rfq_id}/quotes/{quote_id}/accept"
        ));
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// RFQ-scoped equivalent of [`KalshiRestClient::confirm_quote`]. Prefer this
    /// method; the quote-ID-only endpoint is deprecated by the Kalshi API.
    pub async fn confirm_rfq_quote(
        &self,
        rfq_id: &str,
        quote_id: &str,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/communications/rfqs/{rfq_id}/quotes/{quote_id}/confirm"
        ));
        let body = EmptyResponse::default();
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Create a pager for iterating over RFQs page by page.
    pub fn rfqs_pager(&self, params: GetRFQsParams) -> CursorPager<RFQ> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_rfqs(page_params).await?;
                Ok((resp.rfqs, resp.cursor))
            })
        })
    }

    /// Create a pager for iterating over quotes page by page.
    pub fn quotes_pager(&self, params: GetQuotesParams) -> CursorPager<Quote> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_quotes(page_params).await?;
                Ok((resp.quotes, resp.cursor))
            })
        })
    }

    /// Stream RFQs one by one.
    pub fn stream_rfqs(
        &self,
        params: GetRFQsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<RFQ, KalshiError>> + Send {
        stream_items(self.rfqs_pager(params), max_items)
    }

    /// Stream quotes one by one.
    pub fn stream_quotes(
        &self,
        params: GetQuotesParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Quote, KalshiError>> + Send {
        stream_items(self.quotes_pager(params), max_items)
    }

    /// Fetch all pages for RFQs using cursor pagination.
    pub async fn get_rfqs_all(&self, params: GetRFQsParams) -> Result<Vec<RFQ>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_rfqs(page_params).await?;
                Ok((resp.rfqs, resp.cursor))
            }
        })
        .await
    }

    /// Fetch all pages for quotes using cursor pagination.
    pub async fn get_quotes_all(&self, params: GetQuotesParams) -> Result<Vec<Quote>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_quotes(page_params).await?;
                Ok((resp.quotes, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::tests::load_test_auth;
    use crate::env::KalshiEnvironment;
    use reqwest::StatusCode;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use url::Url;

    // ---- serde round-trips for the fields added/removed in this reconciliation ----

    #[test]
    fn get_quotes_params_serializes_min_ts_and_max_ts() {
        let params = GetQuotesParams {
            min_ts: Some(1_700_000_000),
            max_ts: Some(1_800_000_000),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["min_ts"], 1_700_000_000);
        assert_eq!(json["max_ts"], 1_800_000_000);
    }

    #[test]
    fn get_quotes_params_omits_min_ts_and_max_ts_when_absent() {
        let json = serde_json::to_value(&GetQuotesParams::default()).unwrap();
        assert!(json.get("min_ts").is_none());
        assert!(json.get("max_ts").is_none());
    }

    #[test]
    fn get_quotes_params_has_no_market_or_event_ticker_fields() {
        // `market_ticker`/`event_ticker` were removed from `GetQuotesParams`
        // entirely (the server now rejects them for this endpoint). Building
        // one from every remaining field and checking the serialized keys is
        // as close as a runtime assertion gets; the real enforcement is that
        // this file no longer compiles if either field is referenced.
        let params = GetQuotesParams {
            cursor: Some("c1".to_string()),
            min_ts: Some(1),
            max_ts: Some(2),
            limit: Some(10),
            status: Some("open".to_string()),
            quote_creator_user_id: Some("u1".to_string()),
            rfq_creator_user_id: Some("u2".to_string()),
            rfq_creator_subtrader_id: Some("s1".to_string()),
            rfq_id: Some("rfq1".to_string()),
            rfq_user_filter: Some("self".to_string()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("market_ticker").is_none());
        assert!(json.get("event_ticker").is_none());
    }

    #[test]
    fn create_rfq_request_serializes_target_cost_excludes_fees_when_set() {
        let req = CreateRFQRequest {
            market_ticker: "TICK".to_string(),
            contracts: Some(1),
            contracts_fp: None,
            target_cost_centi_cents: None,
            target_cost_dollars: None,
            target_cost_excludes_fees: Some(true),
            rest_remainder: false,
            replace_existing: None,
            subtrader_id: None,
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["target_cost_excludes_fees"], true);
    }

    #[test]
    fn create_rfq_request_omits_target_cost_excludes_fees_when_absent() {
        let req = CreateRFQRequest {
            market_ticker: "TICK".to_string(),
            contracts: None,
            contracts_fp: None,
            target_cost_centi_cents: None,
            target_cost_dollars: None,
            target_cost_excludes_fees: None,
            rest_remainder: false,
            replace_existing: None,
            subtrader_id: None,
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("target_cost_excludes_fees").is_none());
    }

    #[test]
    fn rfq_deserializes_target_cost_excludes_fees() {
        let json = r#"{
            "id": "rfq1",
            "creator_id": "creator1",
            "market_ticker": "TICK",
            "contracts_fp": "1",
            "status": "open",
            "created_ts": "2026-01-01T00:00:00Z",
            "target_cost_excludes_fees": true
        }"#;
        let rfq: RFQ = serde_json::from_str(json).unwrap();
        assert_eq!(rfq.target_cost_excludes_fees, Some(true));
    }

    #[test]
    fn rfq_defaults_target_cost_excludes_fees_when_absent() {
        let json = r#"{
            "id": "rfq1",
            "creator_id": "creator1",
            "market_ticker": "TICK",
            "contracts_fp": "1",
            "status": "open",
            "created_ts": "2026-01-01T00:00:00Z"
        }"#;
        let rfq: RFQ = serde_json::from_str(json).unwrap();
        assert_eq!(rfq.target_cost_excludes_fees, None);
    }

    #[test]
    fn create_quote_request_serializes_post_only_when_set() {
        let req = CreateQuoteRequest {
            rfq_id: "rfq1".to_string(),
            yes_bid: "0.50".to_string(),
            no_bid: "0.50".to_string(),
            rest_remainder: false,
            post_only: Some(true),
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["post_only"], true);
    }

    #[test]
    fn create_quote_request_omits_post_only_when_absent() {
        let req = CreateQuoteRequest {
            rfq_id: "rfq1".to_string(),
            yes_bid: "0.50".to_string(),
            no_bid: "0.50".to_string(),
            rest_remainder: false,
            post_only: None,
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("post_only").is_none());
    }

    #[test]
    fn quote_deserializes_post_only() {
        let quote: Quote = serde_json::from_str(&quote_json_with(r#""post_only": true"#)).unwrap();
        assert_eq!(quote.post_only, Some(true));
    }

    #[test]
    fn quote_defaults_post_only_when_absent() {
        let quote: Quote = serde_json::from_str(&quote_json_with("")).unwrap();
        assert_eq!(quote.post_only, None);
    }

    fn quote_json_with(extra_field: &str) -> String {
        let extra = if extra_field.is_empty() {
            String::new()
        } else {
            format!(",{extra_field}")
        };
        format!(
            r#"{{
                "id": "q1",
                "rfq_id": "rfq1",
                "creator_id": "creator1",
                "rfq_creator_id": "rfqcreator1",
                "market_ticker": "TICK",
                "contracts_fp": "1",
                "yes_bid_dollars": "0.50",
                "no_bid_dollars": "0.50",
                "created_ts": "2026-01-01T00:00:00Z",
                "updated_ts": "2026-01-01T00:00:00Z",
                "status": "open"{extra}
            }}"#
        )
    }

    // ---- request-URL/body-shape assertions for the new RFQ-scoped methods ----
    //
    // This crate uses no HTTP-mocking library anywhere (no wiremock/mockito in
    // Cargo.toml); `rest::client`'s own test module instead hand-rolls a tiny
    // one-shot TCP server to capture what reqwest actually sent. That harness
    // is private to that module, so it's reproduced here in miniature rather
    // than introducing a new dependency.

    struct CapturedRequest {
        method: String,
        path: String,
        body: String,
    }

    fn header_end(buf: &[u8]) -> Option<usize> {
        buf.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
    }

    async fn spawn_capturing_server(
        status: StatusCode,
        response_body: String,
    ) -> (Url, tokio::task::JoinHandle<CapturedRequest>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local addr");

        let task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.expect("accept");

            let mut buffer = Vec::new();
            let mut chunk = [0u8; 4096];
            let mut header_len: Option<usize> = None;
            let mut required_body_len = 0usize;

            loop {
                let n = stream.read(&mut chunk).await.expect("read");
                if n == 0 {
                    break;
                }
                buffer.extend_from_slice(&chunk[..n]);

                if header_len.is_none()
                    && let Some(end) = header_end(&buffer)
                {
                    header_len = Some(end);
                    let headers = String::from_utf8_lossy(&buffer[..end]).to_ascii_lowercase();
                    required_body_len = headers
                        .lines()
                        .find_map(|line| line.strip_prefix("content-length:"))
                        .and_then(|value| value.trim().parse::<usize>().ok())
                        .unwrap_or(0);
                }

                if let Some(header_len) = header_len
                    && buffer.len() - header_len >= required_body_len
                {
                    break;
                }
            }

            let header_len = header_len.unwrap_or(buffer.len());
            let head = String::from_utf8_lossy(&buffer[..header_len]).into_owned();
            let body = String::from_utf8_lossy(&buffer[header_len..]).into_owned();

            let mut parts = head.lines().next().unwrap_or_default().split_whitespace();
            let method = parts.next().unwrap_or_default().to_string();
            let path = parts.next().unwrap_or_default().to_string();

            let reason = status.canonical_reason().unwrap_or("Unknown");
            let mut reply = format!(
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                status.as_u16(),
                reason,
                response_body.len()
            );
            reply.push_str(&response_body);
            stream.write_all(reply.as_bytes()).await.expect("write");
            stream.flush().await.expect("flush");

            CapturedRequest { method, path, body }
        });

        (Url::parse(&format!("http://{addr}")).expect("url"), task)
    }

    fn test_client(rest_origin: Url) -> KalshiRestClient {
        KalshiRestClient::builder(KalshiEnvironment {
            rest_origin,
            ws_url: "ws://127.0.0.1/".to_string(),
        })
        .with_auth(load_test_auth())
        .build()
        .expect("build test client")
    }

    #[tokio::test]
    async fn get_rfq_quote_hits_rfq_scoped_path() {
        let body = format!(r#"{{"quote":{}}}"#, quote_json_with(""));
        let (origin, server) = spawn_capturing_server(StatusCode::OK, body).await;
        let client = test_client(origin);

        let resp = client
            .get_rfq_quote("rfq1", "quote1")
            .await
            .expect("get_rfq_quote should succeed");
        assert_eq!(resp.quote.id, "q1");

        let captured = server.await.expect("server task");
        assert_eq!(captured.method, "GET");
        assert_eq!(
            captured.path,
            "/trade-api/v2/communications/rfqs/rfq1/quotes/quote1"
        );
    }

    #[tokio::test]
    async fn delete_rfq_quote_hits_rfq_scoped_path() {
        let (origin, server) = spawn_capturing_server(StatusCode::NO_CONTENT, String::new()).await;
        let client = test_client(origin);

        client
            .delete_rfq_quote("rfq1", "quote1")
            .await
            .expect("delete_rfq_quote should succeed");

        let captured = server.await.expect("server task");
        assert_eq!(captured.method, "DELETE");
        assert_eq!(
            captured.path,
            "/trade-api/v2/communications/rfqs/rfq1/quotes/quote1"
        );
    }

    #[tokio::test]
    async fn accept_rfq_quote_hits_rfq_scoped_path_with_accepted_side_body() {
        let (origin, server) = spawn_capturing_server(StatusCode::NO_CONTENT, String::new()).await;
        let client = test_client(origin);

        client
            .accept_rfq_quote(
                "rfq1",
                "quote1",
                AcceptQuoteRequest {
                    accepted_side: YesNo::Yes,
                },
            )
            .await
            .expect("accept_rfq_quote should succeed");

        let captured = server.await.expect("server task");
        assert_eq!(captured.method, "PUT");
        assert_eq!(
            captured.path,
            "/trade-api/v2/communications/rfqs/rfq1/quotes/quote1/accept"
        );
        let sent: Value = serde_json::from_str(&captured.body).expect("valid JSON body");
        assert_eq!(sent, serde_json::json!({ "accepted_side": "yes" }));
    }

    #[tokio::test]
    async fn confirm_rfq_quote_hits_rfq_scoped_path() {
        let (origin, server) = spawn_capturing_server(StatusCode::NO_CONTENT, String::new()).await;
        let client = test_client(origin);

        client
            .confirm_rfq_quote("rfq1", "quote1")
            .await
            .expect("confirm_rfq_quote should succeed");

        let captured = server.await.expect("server task");
        assert_eq!(captured.method, "PUT");
        assert_eq!(
            captured.path,
            "/trade-api/v2/communications/rfqs/rfq1/quotes/quote1/confirm"
        );
    }
}
