//! Multivariate event collection endpoints.
//!
//! Collections group together related events into a single multi-leg market.
//! This module exposes the CRUD endpoints, the ticker-pair lookup helpers, and
//! the lookup history feed.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::markets::Market;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{YesNo, deserialize_null_as_empty_vec};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMultivariateEventCollectionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMultivariateEventCollectionsResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub multivariate_contracts: Vec<MultivariateEventCollection>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMultivariateEventCollectionResponse {
    pub multivariate_contract: MultivariateEventCollection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssociatedEvent {
    pub ticker: String,
    pub is_yes_only: bool,
    #[serde(default)]
    pub size_max: Option<i32>,
    #[serde(default)]
    pub size_min: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub active_quoters: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MultivariateEventCollection {
    pub collection_ticker: String,
    pub series_ticker: String,
    pub title: String,
    pub description: String,
    pub open_date: String,
    pub close_date: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub associated_events: Vec<AssociatedEvent>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub associated_event_tickers: Vec<String>,
    pub is_ordered: bool,
    pub is_single_market_per_event: bool,
    pub is_all_yes: bool,
    pub size_min: i32,
    pub size_max: i32,
    pub functional_description: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerPair {
    pub market_ticker: String,
    pub event_ticker: String,
    pub side: YesNo,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarketInMultivariateEventCollectionRequest {
    pub selected_markets: Vec<TickerPair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_market_payload: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMarketInMultivariateEventCollectionResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    #[serde(default)]
    pub market: Option<Market>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMultivariateEventCollectionLookupHistoryParams {
    pub lookback_seconds: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMultivariateEventCollectionLookupHistoryResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub lookup_points: Vec<LookupPoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LookupPoint {
    pub event_ticker: String,
    pub market_ticker: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub selected_markets: Vec<TickerPair>,
    pub last_queried_ts: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LookupTickersForMarketInMultivariateEventCollectionRequest {
    pub selected_markets: Vec<TickerPair>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LookupTickersForMarketInMultivariateEventCollectionResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl KalshiRestClient {
    pub async fn get_multivariate_event_collections(
        &self,
        params: GetMultivariateEventCollectionsParams,
    ) -> Result<GetMultivariateEventCollectionsResponse, KalshiError> {
        let path = Self::full_path("/multivariate_event_collections");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: &str,
    ) -> Result<GetMultivariateEventCollectionResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/multivariate_event_collections/{collection_ticker}"
        ));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn create_market_in_multivariate_event_collection(
        &self,
        collection_ticker: &str,
        body: CreateMarketInMultivariateEventCollectionRequest,
    ) -> Result<CreateMarketInMultivariateEventCollectionResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/multivariate_event_collections/{collection_ticker}"
        ));
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Get lookup history for a multivariate event collection.
    ///
    /// Fully deprecated per the Kalshi changelog (2026-07-02) and no longer
    /// present in the OpenAPI spec. The endpoint is still live as of this
    /// refresh; see `docs/spec-parity.md`.
    #[deprecated(
        note = "fully deprecated by Kalshi (changelog 2026-07-02); no longer in the OpenAPI spec"
    )]
    pub async fn get_multivariate_event_collection_lookup_history(
        &self,
        collection_ticker: &str,
        params: GetMultivariateEventCollectionLookupHistoryParams,
    ) -> Result<GetMultivariateEventCollectionLookupHistoryResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/multivariate_event_collections/{collection_ticker}/lookup"
        ));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn lookup_tickers_for_market_in_multivariate_event_collection(
        &self,
        collection_ticker: &str,
        body: LookupTickersForMarketInMultivariateEventCollectionRequest,
    ) -> Result<LookupTickersForMarketInMultivariateEventCollectionResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/multivariate_event_collections/{collection_ticker}/lookup"
        ));
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// Create a pager for iterating over multivariate event collections page by page.
    pub fn multivariate_event_collections_pager(
        &self,
        params: GetMultivariateEventCollectionsParams,
    ) -> CursorPager<MultivariateEventCollection> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client
                    .get_multivariate_event_collections(page_params)
                    .await?;
                Ok((resp.multivariate_contracts, resp.cursor))
            })
        })
    }

    /// Stream multivariate event collections one by one.
    pub fn stream_multivariate_event_collections(
        &self,
        params: GetMultivariateEventCollectionsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<MultivariateEventCollection, KalshiError>> + Send {
        stream_items(self.multivariate_event_collections_pager(params), max_items)
    }

    /// Fetch all pages for multivariate collections using cursor pagination.
    pub async fn get_multivariate_event_collections_all(
        &self,
        params: GetMultivariateEventCollectionsParams,
    ) -> Result<Vec<MultivariateEventCollection>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_multivariate_event_collections(page_params).await?;
                Ok((resp.multivariate_contracts, resp.cursor))
            }
        })
        .await
    }
}
