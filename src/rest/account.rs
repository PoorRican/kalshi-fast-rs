//! Account, subaccounts, and API key endpoints.
//!
//! Authenticated endpoints for managing account-level configuration:
//! API rate-limit tiers, subaccount creation/balances/transfers/netting,
//! and API key lifecycle (list/create/generate/delete).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{
    FixedPointCount, FixedPointDollars, deserialize_null_as_empty_vec,
    deserialize_string_or_number,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Exchange instance lane (event-contract binary markets vs margin/perps).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeInstance {
    EventContract,
    Margined,
    #[serde(other)]
    Unknown,
}

/// Token-bucket budget for one rate-limit bucket.
///
/// `refill_rate` tokens are added per second; `bucket_capacity` is the
/// maximum the bucket can hold (burst headroom).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BucketLimit {
    /// Tokens added to the bucket per second (equals the sustained RPS budget).
    pub refill_rate: i64,
    /// Maximum tokens the bucket can hold (burst headroom above one second of budget).
    pub bucket_capacity: i64,
}

/// A single API usage level grant for one exchange lane.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiUsageLevelGrant {
    pub exchange_instance: ExchangeInstance,
    /// API usage level this grant confers (e.g. `"premier"`, `"paragon"`, `"prime"`).
    pub level: String,
    /// Unix timestamp (seconds) when the grant expires; `None` for permanent grants.
    #[serde(default)]
    pub expires_ts: Option<i64>,
    /// How the grant was created: `"volume"` (earned) or `"manual"` (assigned by Kalshi).
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountApiLimitsResponse {
    pub usage_tier: String,
    pub read: BucketLimit,
    pub write: BucketLimit,
    pub grants: Vec<ApiUsageLevelGrant>,
}

/// Volume progress toward a single API usage tier level.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeGoal {
    /// API usage level this goal applies to (e.g. `"premier"`).
    pub level: String,
    /// Trailing 30d volume required to *earn* this tier.
    pub earn_volume_goal_fp: FixedPointCount,
    /// Trailing 30d volume required to *keep* this tier once earned.
    pub keep_volume_goal_fp: FixedPointCount,
}

/// Latest cron-computed trading volume progress for one exchange lane.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeProgress {
    /// Unix timestamp (seconds) when progress was computed; volume covers trailing 30d ending here.
    pub computed_ts: i64,
    pub trailing_30d_volume_fp: FixedPointCount,
    pub goals: Vec<AccountApiUsageLevelVolumeGoal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountApiUsageLevelVolumeProgressResponse {
    pub volume_progress: Vec<AccountApiUsageLevelVolumeProgress>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubaccountResponse {
    pub subaccount_number: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubaccountBalance {
    pub subaccount_number: u32,
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub balance: FixedPointDollars,
    pub updated_ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSubaccountBalancesResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub subaccount_balances: Vec<SubaccountBalance>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplySubaccountTransferRequest {
    pub client_transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct ApplySubaccountTransferResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubaccountTransfer {
    pub transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
    pub created_ts: i64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSubaccountTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSubaccountTransfersResponse {
    #[serde(
        default,
        deserialize_with = "deserialize_null_as_empty_vec",
        alias = "subaccount_transfer_arr",
        alias = "transfers"
    )]
    pub subaccount_transfers: Vec<SubaccountTransfer>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenericObject {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmptyResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub scopes: Vec<String>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetApiKeysResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub api_keys: Vec<ApiKey>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateApiKeyRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerateApiKeyResponse {
    pub api_key_id: String,
    pub private_key: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SubaccountQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubaccountNettingRequest {
    pub subaccount_number: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountNettingConfig {
    pub subaccount_number: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSubaccountNettingResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub netting_configs: Vec<SubaccountNettingConfig>,
}

impl KalshiRestClient {
    /// Get API rate-limit and position limits for the account.
    ///
    /// Returns token-bucket budgets (`read`, `write`) and the active `grants` list.
    ///
    /// **Requires auth.**
    pub async fn get_account_api_limits(&self) -> Result<GetAccountApiLimitsResponse, KalshiError> {
        let path = Self::full_path("/account/limits");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Self-promote to the Advanced API usage tier (Predictions lane only).
    ///
    /// Grants a permanent Advanced grant if the authenticated user has at least
    /// one API-created order in their last 100 Predictions orders.
    ///
    /// Returns HTTP 201 on success. Returns HTTP 403 if the criteria are not met.
    ///
    /// **Requires auth.**
    pub async fn upgrade_api_usage_level(&self) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/account/api_usage_level/upgrade");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Get trailing 30d volume progress toward volume-based API usage tiers (Predictions lane).
    ///
    /// **Requires auth.**
    pub async fn get_api_usage_level_volume_progress(
        &self,
    ) -> Result<GetAccountApiUsageLevelVolumeProgressResponse, KalshiError> {
        let path = Self::full_path("/account/api_usage_level/volume_progress");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Create a new subaccount.
    ///
    /// **Requires auth.**
    pub async fn create_subaccount(&self) -> Result<CreateSubaccountResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Get balances for all subaccounts.
    ///
    /// **Requires auth.**
    pub async fn get_subaccount_balances(
        &self,
    ) -> Result<GetSubaccountBalancesResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts/balances");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Transfer funds between subaccounts.
    ///
    /// **Requires auth.**
    pub async fn transfer_subaccount(
        &self,
        body: ApplySubaccountTransferRequest,
    ) -> Result<ApplySubaccountTransferResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts/transfer");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// List subaccount transfers. Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_subaccount_transfers(
        &self,
        params: GetSubaccountTransfersParams,
    ) -> Result<GetSubaccountTransfersResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts/transfers");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get subaccount netting configuration.
    ///
    /// **Requires auth.**
    pub async fn get_subaccount_netting(
        &self,
    ) -> Result<GetSubaccountNettingResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts/netting");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Update netting configuration for a subaccount.
    ///
    /// **Requires auth.**
    pub async fn update_subaccount_netting(
        &self,
        body: UpdateSubaccountNettingRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts/netting");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn get_api_keys(&self) -> Result<GetApiKeysResponse, KalshiError> {
        let path = Self::full_path("/api_keys");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn create_api_key(
        &self,
        body: CreateApiKeyRequest,
    ) -> Result<CreateApiKeyResponse, KalshiError> {
        let path = Self::full_path("/api_keys");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn generate_api_key(
        &self,
        body: GenerateApiKeyRequest,
    ) -> Result<GenerateApiKeyResponse, KalshiError> {
        let path = Self::full_path("/api_keys/generate");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    pub async fn delete_api_key(&self, api_key: &str) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/api_keys/{api_key}"));
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Create a pager for iterating over subaccount transfers page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn subaccount_transfers_pager(
        &self,
        params: GetSubaccountTransfersParams,
    ) -> CursorPager<SubaccountTransfer> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client.get_subaccount_transfers(page_params).await?;
                Ok((resp.subaccount_transfers, resp.cursor))
            })
        })
    }

    /// Stream subaccount transfers one by one.
    ///
    /// **Requires auth.**
    pub fn stream_subaccount_transfers(
        &self,
        params: GetSubaccountTransfersParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<SubaccountTransfer, KalshiError>> + Send {
        stream_items(self.subaccount_transfers_pager(params), max_items)
    }

    /// Fetch all pages for subaccount transfers using cursor pagination.
    pub async fn get_subaccount_transfers_all(
        &self,
        params: GetSubaccountTransfersParams,
    ) -> Result<Vec<SubaccountTransfer>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self.get_subaccount_transfers(page_params).await?;
                Ok((resp.subaccount_transfers, resp.cursor))
            }
        })
        .await
    }
}
