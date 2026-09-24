//! Series endpoints and types.
//!
//! Series are the top-level grouping in Kalshi's market taxonomy — a named
//! class of repeatable events (e.g., "weekly presidential approval polls"). A
//! single series spans many events, and each event spans many markets.

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::{FeeType, deserialize_null_as_empty_vec};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettlementSource {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketMetadata {
    pub market_ticker: String,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub color_code: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMetadata {
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub featured_image_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub market_details: Vec<MarketMetadata>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub settlement_sources: Vec<SettlementSource>,
    #[serde(default)]
    pub competition: Option<String>,
    #[serde(default)]
    pub competition_scope: Option<String>,
    #[serde(default)]
    pub cadence: Option<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Series {
    pub ticker: String,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub categories: Vec<String>,
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub settlement_sources: Vec<SettlementSource>,
    #[serde(default)]
    pub contract_url: Option<String>,
    #[serde(default)]
    pub contract_terms_url: Option<String>,
    #[serde(default)]
    pub fee_type: Option<FeeType>,
    #[serde(default)]
    pub fee_multiplier: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub additional_prohibitions: Vec<String>,
    #[serde(default)]
    pub product_metadata: Option<Map<String, Value>>,
    #[serde(default)]
    pub volume: Option<i64>,
    #[serde(default)]
    pub volume_fp: Option<String>,
    #[serde(default)]
    pub latest_event_ticker: Option<String>,
    #[serde(default)]
    pub last_updated_ts: Option<String>,
    #[serde(default)]
    pub inactive: Option<bool>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSeriesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Docs: "tags" is a string (not explicitly CSV-typed), so keep as raw string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_product_metadata: Option<bool>,
    /// If true, includes total volume traded across all events in each series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeriesListResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub series: Vec<Series>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeriesResponse {
    pub series: Series,
}

impl KalshiRestClient {
    /// List all series, optionally filtered by category or tags.
    pub async fn get_series_list(
        &self,
        params: GetSeriesListParams,
    ) -> Result<GetSeriesListResponse, KalshiError> {
        let path = Self::full_path("/series");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get a single series by ticker.
    pub async fn get_series(&self, series_ticker: &str) -> Result<GetSeriesResponse, KalshiError> {
        let path = Self::full_path(&format!("/series/{series_ticker}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_deserializes_categories_distinct_from_category() {
        let json = r#"{
            "ticker": "SER-1",
            "category": "Politics",
            "categories": ["Politics", "Elections"]
        }"#;
        let series: Series = serde_json::from_str(json).unwrap();
        assert_eq!(series.category.as_deref(), Some("Politics"));
        assert_eq!(series.categories, vec!["Politics", "Elections"]);
    }

    #[test]
    fn series_defaults_categories_to_empty_when_absent_or_null() {
        let json = r#"{"ticker": "SER-1"}"#;
        let series: Series = serde_json::from_str(json).unwrap();
        assert!(series.categories.is_empty());

        let json_null = r#"{"ticker": "SER-1", "categories": null}"#;
        let series: Series = serde_json::from_str(json_null).unwrap();
        assert!(series.categories.is_empty());
    }

    #[test]
    fn series_keeps_unknown_fields_in_extra_catch_all() {
        let json = r#"{
            "ticker": "SER-1",
            "categories": [],
            "new_field_from_changelog": "kept"
        }"#;
        let series: Series = serde_json::from_str(json).unwrap();
        assert_eq!(
            series
                .extra
                .get("new_field_from_changelog")
                .and_then(|v| v.as_str()),
            Some("kept")
        );

        // The catch-all round-trips back out on serialize.
        let value = serde_json::to_value(&series).unwrap();
        assert_eq!(value["new_field_from_changelog"], "kept");
    }

    #[test]
    fn event_metadata_deserializes_cadence() {
        let json = r#"{"cadence": "weekly"}"#;
        let meta: EventMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.cadence.as_deref(), Some("weekly"));
    }

    #[test]
    fn event_metadata_defaults_cadence_when_absent() {
        let json = r#"{}"#;
        let meta: EventMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.cadence, None);
    }
}
