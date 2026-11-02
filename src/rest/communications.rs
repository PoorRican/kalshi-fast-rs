//! RFQ (Request For Quote), quote, block trade proposal, and communications ID endpoints.
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
    pub post_only: Option<bool>,
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
    /// Whether the RFQ target cost excludes taker fees.
    #[serde(default)]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(default)]
    pub rfq_creator_order_id: Option<String>,
    #[serde(default)]
    pub creator_order_id: Option<String>,
    #[serde(default)]
    pub creator_subaccount: Option<u32>,
    #[serde(default)]
    pub rfq_creator_subaccount: Option<u32>,
    #[serde(default)]
    pub yes_contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_fp: Option<FixedPointCount>,
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
    /// Whether the target cost excludes taker fees.
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
    pub creator_subaccount: Option<u32>,
    #[serde(default)]
    pub cancelled_ts: Option<String>,
    #[serde(default)]
    pub updated_ts: Option<String>,
    #[serde(default)]
    pub rfq_batch_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetQuotesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_creator_user_id: Option<String>,
    /// Filter quotes created by the authenticated user with `"self"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
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
    /// Filter RFQs created by the authenticated user with `"self"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
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
    /// Size against principal only; taker fees are charged on top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    /// Hide the creator's communications ID from other users until execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obscure_creator_id: Option<bool>,
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

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBlockTradeProposalsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockTradeProposal {
    pub id: String,
    pub proposer_user_id: String,
    pub buyer_user_id: String,
    #[serde(default)]
    pub buyer_subtrader_id: Option<String>,
    pub seller_user_id: String,
    #[serde(default)]
    pub seller_subtrader_id: Option<String>,
    pub market_ticker: String,
    pub price_centi_cents: i64,
    pub centicount: i64,
    pub maker_side: YesNo,
    pub expiration_ts: String,
    pub status: String,
    pub created_ts: String,
    pub updated_ts: String,
    pub buyer_accepted: bool,
    pub seller_accepted: bool,
    #[serde(default)]
    pub buyer_accepted_ts: Option<String>,
    #[serde(default)]
    pub seller_accepted_ts: Option<String>,
    #[serde(default)]
    pub executed_ts: Option<String>,
    #[serde(default)]
    pub buyer_order_id: Option<String>,
    #[serde(default)]
    pub seller_order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockTradeProposalsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub block_trade_proposals: Vec<BlockTradeProposal>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProposeBlockTradeRequest {
    pub buyer_user_id: String,
    /// Specify either the buyer subtrader or subaccount, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_subaccount: Option<u32>,
    pub seller_user_id: String,
    /// Specify either the seller subtrader or subaccount, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_subaccount: Option<u32>,
    pub market_ticker: String,
    pub price_centi_cents: i64,
    pub centicount: i64,
    pub maker_side: YesNo,
    pub expiration_ts: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProposeBlockTradeResponse {
    pub block_trade_proposal_id: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AcceptBlockTradeProposalRequest {
    /// Specify either a subtrader or subaccount, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
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

    pub async fn get_block_trade_proposals(
        &self,
        params: GetBlockTradeProposalsParams,
    ) -> Result<GetBlockTradeProposalsResponse, KalshiError> {
        let path = Self::full_path("/communications/block-trade-proposals");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn propose_block_trade(
        &self,
        body: ProposeBlockTradeRequest,
    ) -> Result<ProposeBlockTradeResponse, KalshiError> {
        let path = Self::full_path("/communications/block-trade-proposals");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn accept_block_trade_proposal(
        &self,
        block_trade_proposal_id: &str,
        body: Option<AcceptBlockTradeProposalRequest>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/communications/block-trade-proposals/{block_trade_proposal_id}/accept"
        ));
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            body.as_ref(),
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
    use serde_json::json;

    #[test]
    fn rfq_and_quote_decode_private_fields_and_batch_metadata() {
        let rfq: RFQ = serde_json::from_value(json!({
            "id": "rfq",
            "creator_id": "obscured",
            "market_ticker": "MARKET",
            "contracts_fp": "10.00",
            "status": "open",
            "created_ts": "2026-11-02T00:00:00Z",
            "target_cost_excludes_fees": true,
            "creator_user_id": null,
            "creator_subaccount": 0,
            "rfq_batch_id": "batch",
            "mve_selected_legs": null
        }))
        .unwrap();
        assert_eq!(rfq.target_cost_excludes_fees, Some(true));
        assert_eq!(rfq.creator_user_id, None);
        assert_eq!(rfq.creator_subaccount, Some(0));
        assert_eq!(rfq.rfq_batch_id.as_deref(), Some("batch"));
        assert!(rfq.mve_selected_legs.is_empty());

        let quote: Quote = serde_json::from_value(json!({
            "id": "quote",
            "rfq_id": "rfq",
            "creator_id": "maker",
            "rfq_creator_id": "obscured",
            "market_ticker": "MARKET",
            "contracts_fp": "10.00",
            "yes_bid_dollars": "0.5200",
            "no_bid_dollars": "0.4800",
            "created_ts": "2026-11-02T00:00:00Z",
            "updated_ts": "2026-11-02T00:00:01Z",
            "status": "open",
            "creator_user_id": null,
            "creator_subaccount": null,
            "rfq_creator_subaccount": 0,
            "post_only": true,
            "target_cost_excludes_fees": true,
            "yes_contracts_fp": "8.25",
            "no_contracts_fp": "9.50"
        }))
        .unwrap();
        assert_eq!(quote.creator_user_id, None);
        assert_eq!(quote.creator_subaccount, None);
        assert_eq!(quote.rfq_creator_subaccount, Some(0));
        assert_eq!(quote.post_only, Some(true));
        assert_eq!(quote.target_cost_excludes_fees, Some(true));
        assert_eq!(quote.yes_contracts_fp.as_deref(), Some("8.25"));
        assert_eq!(quote.no_contracts_fp.as_deref(), Some("9.50"));
    }

    #[test]
    fn rfq_and_quote_requests_preserve_fee_privacy_and_post_only_flags() {
        let rfq = CreateRFQRequest {
            market_ticker: "MARKET".into(),
            contracts: None,
            contracts_fp: None,
            target_cost_centi_cents: None,
            target_cost_dollars: Some("10.0000".into()),
            target_cost_excludes_fees: Some(true),
            obscure_creator_id: Some(true),
            rest_remainder: false,
            replace_existing: None,
            subtrader_id: None,
            subaccount: Some(0),
        };
        let value = serde_json::to_value(rfq).unwrap();
        assert_eq!(value["target_cost_excludes_fees"], true);
        assert_eq!(value["obscure_creator_id"], true);
        assert_eq!(value["subaccount"], 0);
        let quote = CreateQuoteRequest {
            rfq_id: "rfq".into(),
            yes_bid: "0.5200".into(),
            no_bid: "0.4800".into(),
            rest_remainder: true,
            post_only: Some(true),
            subaccount: None,
        };
        assert_eq!(serde_json::to_value(quote).unwrap()["post_only"], true);
    }

    #[test]
    fn block_trade_preserves_zero_subaccount_and_optional_identity() {
        let body = AcceptBlockTradeProposalRequest {
            subtrader_id: None,
            subaccount: Some(0),
        };
        assert_eq!(
            serde_json::to_value(body).unwrap(),
            json!({"subaccount": 0})
        );
        let proposal: BlockTradeProposal = serde_json::from_value(json!({
            "id": "id",
            "proposer_user_id": "proposer",
            "buyer_user_id": "",
            "buyer_subtrader_id": null,
            "seller_user_id": "seller",
            "market_ticker": "MARKET",
            "price_centi_cents": 5200,
            "centicount": 125,
            "maker_side": "yes",
            "expiration_ts": "2026-11-02T00:00:00Z",
            "status": "pending",
            "created_ts": "2026-11-01T00:00:00Z",
            "updated_ts": "2026-11-01T00:00:00Z",
            "buyer_accepted": false,
            "seller_accepted": true
        }))
        .unwrap();
        assert_eq!(proposal.buyer_user_id, "");
        assert_eq!(proposal.buyer_subtrader_id, None);
        assert_eq!(proposal.centicount, 125);
        assert!(proposal.seller_accepted);
    }
}
