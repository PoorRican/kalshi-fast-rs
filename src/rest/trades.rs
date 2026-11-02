//! Trades and historical trade/order/fill endpoints.
//!
//! Public trade feed plus the `/historical/*` family of endpoints that return
//! archived fills, orders, markets, and the data-cutoff timestamps that
//! separate live from historical datasets.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::orders::GetOrdersResponse;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::rest::portfolio::{GetFillsResponse, GetPositionsResponse, PositionsPage};
use crate::types::{
    BookSide, FixedPointCount, FixedPointDollars, MveFilter, TradeTakerSide,
    deserialize_null_as_empty_vec,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: String,
    pub ticker: String,
    pub count_fp: FixedPointCount,
    pub yes_price_dollars: FixedPointDollars,
    pub no_price_dollars: FixedPointDollars,
    /// Deprecated 2026-05-07. Use `taker_outcome_side` / `taker_book_side`.
    /// Optional to tolerate eventual removal by the exchange.
    #[serde(default)]
    pub taker_side: Option<TradeTakerSide>,
    /// Normalized taker outcome side (yes | no).
    pub taker_outcome_side: TradeTakerSide,
    /// Normalized taker book side (bid | ask).
    pub taker_book_side: BookSide,
    pub created_time: String,
    /// True for block trades matched off-book (e.g. via RFQ).
    pub is_block_trade: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Filter by block trade status. Omit to return all trades.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTradesResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub trades: Vec<Trade>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetHistoricalMarketsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<MveFilter>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetHistoricalFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetHistoricalOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetHistoricalPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetHistoricalCutoffResponse {
    pub market_settled_ts: String,
    pub trades_created_ts: String,
    pub orders_updated_ts: String,
    /// Backfill horizon, not a guarantee that every older position has moved.
    #[serde(default)]
    pub market_positions_last_updated_ts: Option<String>,
}

impl KalshiRestClient {
    /// List public trades. Supports cursor pagination.
    pub async fn get_trades(
        &self,
        params: GetTradesParams,
    ) -> Result<GetTradesResponse, KalshiError> {
        let path = Self::full_path("/markets/trades");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// List historical fills. Requires auth.
    pub async fn get_historical_fills(
        &self,
        params: GetHistoricalFillsParams,
    ) -> Result<GetFillsResponse, KalshiError> {
        let path = Self::full_path("/historical/fills");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List historical fills. Requires auth.
    pub async fn get_fills_historical(
        &self,
        params: GetHistoricalFillsParams,
    ) -> Result<GetFillsResponse, KalshiError> {
        self.get_historical_fills(params).await
    }

    /// List historical orders. Requires auth.
    pub async fn get_historical_orders(
        &self,
        params: GetHistoricalOrdersParams,
    ) -> Result<GetOrdersResponse, KalshiError> {
        let path = Self::full_path("/historical/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// List settled positions after their whole-event historical handoff.
    ///
    /// Positions not yet handed off remain available through portfolio positions.
    /// **Requires auth.**
    pub async fn get_historical_positions(
        &self,
        params: GetHistoricalPositionsParams,
    ) -> Result<GetPositionsResponse, KalshiError> {
        let path = Self::full_path("/historical/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Create a pager over historical position pages.
    pub fn historical_positions_pager(
        &self,
        params: GetHistoricalPositionsParams,
    ) -> CursorPager<PositionsPage> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_historical_positions(page_params).await?;
                let cursor = resp.cursor.clone();
                Ok((vec![PositionsPage::from(resp)], cursor))
            })
        })
    }

    /// Stream historical position pages, preserving market and event positions.
    pub fn stream_historical_positions(
        &self,
        params: GetHistoricalPositionsParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<PositionsPage, KalshiError>> + Send {
        stream_items(self.historical_positions_pager(params), max_items)
    }

    /// Fetch every historical position page.
    pub async fn get_historical_positions_all(
        &self,
        params: GetHistoricalPositionsParams,
    ) -> Result<Vec<PositionsPage>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_historical_positions(page_params).await?;
                let cursor = resp.cursor.clone();
                Ok((vec![PositionsPage::from(resp)], cursor))
            }
        })
        .await
    }

    /// Get historical data cutoffs that separate live and historical datasets.
    pub async fn get_historical_cutoff(&self) -> Result<GetHistoricalCutoffResponse, KalshiError> {
        let path = Self::full_path("/historical/cutoff");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// List historical trades.
    pub async fn get_trades_historical(
        &self,
        params: GetTradesParams,
    ) -> Result<GetTradesResponse, KalshiError> {
        let path = Self::full_path("/historical/trades");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Create a pager for iterating over trades page by page. See [`CursorPager`].
    pub fn trades_pager(&self, params: GetTradesParams) -> CursorPager<Trade> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_trades(page_params).await?;
                Ok((resp.trades, resp.cursor))
            })
        })
    }

    /// Stream trades one by one.
    pub fn stream_trades(
        &self,
        params: GetTradesParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<Trade, KalshiError>> + Send {
        stream_items(self.trades_pager(params), max_items)
    }

    /// Fetch all pages for trades using cursor pagination.
    pub async fn get_trades_all(&self, params: GetTradesParams) -> Result<Vec<Trade>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_trades(page_params).await?;
                Ok((resp.trades, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_requires_current_direction_and_block_flag() {
        let mut value = serde_json::json!({
            "trade_id": "trade",
            "ticker": "market",
            "count_fp": "1.25",
            "yes_price_dollars": "0.4200",
            "no_price_dollars": "0.5800",
            "taker_outcome_side": "yes",
            "taker_book_side": "bid",
            "created_time": "2026-01-01T00:00:00Z",
            "is_block_trade": false
        });
        let trade: Trade = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(trade.count_fp, "1.25");
        assert!(trade.taker_side.is_none());
        value.as_object_mut().unwrap().remove("taker_book_side");
        assert!(serde_json::from_value::<Trade>(value).is_err());
    }

    #[test]
    fn historical_cutoff_positions_horizon_is_independent() {
        let cutoff: GetHistoricalCutoffResponse = serde_json::from_value(serde_json::json!({
            "market_settled_ts": "2026-01-01T00:00:00Z",
            "trades_created_ts": "2026-02-01T00:00:00Z",
            "orders_updated_ts": "2026-03-01T00:00:00Z",
            "market_positions_last_updated_ts": "2026-04-01T00:00:00Z"
        }))
        .unwrap();
        assert_eq!(
            cutoff.market_positions_last_updated_ts.as_deref(),
            Some("2026-04-01T00:00:00Z")
        );
    }
}
