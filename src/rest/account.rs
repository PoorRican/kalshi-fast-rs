//! Account, subaccounts, and API key endpoints.
//!
//! Authenticated endpoints for managing account-level configuration:
//! API rate-limit tiers, subaccount creation/balances/transfers/netting,
//! and API key lifecycle (list/create/generate/delete).

use crate::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::rest::pagination::{CursorPager, stream_items};
use crate::types::{
    FixedPointCount, FixedPointDollars, deserialize_null_as_empty_vec, deserialize_string_or_number,
};
use futures::stream::Stream;
use reqwest::Method;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt;

/// Token-bucket rate-limit configuration for one endpoint group.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BucketLimit {
    /// Tokens added to the bucket per second.
    pub refill_rate: i64,
    /// Maximum tokens the bucket can hold.
    pub bucket_capacity: i64,
}

/// An active API usage-level grant (earned via volume or granted manually).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiUsageLevelGrant {
    /// Exchange instance this grant applies to (`"event_contract"` or `"margined"`).
    pub exchange_instance: String,
    /// Usage level this grant confers (e.g. `"premier"`, `"paragon"`, `"prime"`).
    pub level: String,
    /// Unix timestamp (seconds) when the grant expires; `None` for permanent grants.
    #[serde(default)]
    pub expires_ts: Option<i64>,
    /// How the grant was created: `"volume"` or `"manual"`.
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountApiLimitsResponse {
    pub usage_tier: String,
    pub read: BucketLimit,
    pub write: BucketLimit,
    /// Active usage-level grants across exchange lanes. Added 2026-06-06
    /// (automated API rate-limit tiers). Tolerates a missing/`null` array.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub grants: Vec<ApiUsageLevelGrant>,
}

/// One volume-based API usage-level goal for the predictions (event_contract) lane.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeGoal {
    /// API usage level this goal corresponds to (e.g. `"expert"`).
    pub level: String,
    /// Trailing-30d volume required to *earn* this level.
    pub earn_volume_goal_fp: FixedPointCount,
    /// Trailing-30d volume required to *keep* this level once earned.
    pub keep_volume_goal_fp: FixedPointCount,
}

/// Latest cron-computed trading volume progress toward volume-based API usage tiers.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeProgress {
    /// Unix timestamp (seconds) when this progress was computed; `trailing_30d_volume_fp`
    /// covers the trailing 30 days ending at this time.
    pub computed_ts: i64,
    /// Trailing 30-day trading volume (fixed-point contract count).
    pub trailing_30d_volume_fp: FixedPointCount,
    /// Volume goals for each volume-based usage level.
    pub goals: Vec<AccountApiUsageLevelVolumeGoal>,
}

/// Response for `GET /account/api_usage_level/volume_progress`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountApiUsageLevelVolumeProgressResponse {
    /// Volume progress entries toward volume-based API usage tiers for the
    /// predictions (event_contract) lane.
    pub volume_progress: Vec<AccountApiUsageLevelVolumeProgress>,
}

/// Token cost for one API v2 endpoint whose cost differs from the default.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndpointTokenCost {
    /// HTTP method for the endpoint.
    pub method: String,
    /// API route path for the endpoint.
    pub path: String,
    /// Configured token cost for this endpoint.
    pub cost: i64,
}

/// Response for `GET /account/endpoint_costs`. Lists only endpoints whose
/// configured token cost differs from `default_cost`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountEndpointCostsResponse {
    /// Default token cost applied to endpoints not listed in `endpoint_costs`.
    pub default_cost: i64,
    /// Endpoints whose cost differs from the default.
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub endpoint_costs: Vec<EndpointTokenCost>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubaccountResponse {
    pub subaccount_number: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubaccountBalance {
    pub subaccount_number: u32,
    /// Exchange index the balance is held on. Required by the spec.
    pub exchange_index: u32,
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

/// Signature algorithm of an API key pair.
///
/// `Rsa` - 2048-bit RSA; requests are signed with RSA-PSS SHA-256.
/// `Ed25519` - Ed25519 (RFC 8032) signatures over the same pre-sign text, with
/// lower client-side signing cost. Defaults to `rsa` when omitted from a
/// generate request, for compatibility with existing clients.
///
/// Note: this crate's `auth.rs` only implements RSA-PSS SHA-256 signing today.
/// Modeling `ed25519` here only exposes the REST-surface field; actually
/// signing requests with an Ed25519 key is not yet supported.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiKeyType {
    Rsa,
    Ed25519,
    #[serde(other)]
    Unknown,
}

impl ApiKeyType {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiKeyType::Rsa => "rsa",
            ApiKeyType::Ed25519 => "ed25519",
            ApiKeyType::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ApiKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for ApiKeyType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub scopes: Vec<String>,
    /// If set, this API key is restricted to a single sub-account (0-63).
    /// `None` means the key is unrestricted.
    #[serde(default)]
    pub subaccount: Option<u32>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetApiKeysResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub api_keys: Vec<ApiKey>,
    /// Unix timestamp (seconds) when the account's location attestation for
    /// API key requests expires; a past value means the attestation has
    /// lapsed. `None` when the account has never attested.
    #[serde(default)]
    pub api_key_region_expiration_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    /// Restrict the new key to a single sub-account (0-63) that you own.
    /// Omit to leave the key unrestricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
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
    /// Signature algorithm for the generated key pair. Defaults to `rsa` on
    /// the server when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<ApiKeyType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    /// Restrict the new key to a single sub-account (0-63) that you own.
    /// Omit to leave the key unrestricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerateApiKeyResponse {
    pub api_key_id: String,
    /// Signature algorithm of the generated key pair.
    #[serde(default)]
    pub key_type: Option<ApiKeyType>,
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

/// Which exchange instance (lane) an intra-exchange transfer moves funds
/// to/from. This crate does not otherwise model the margin exchange; this
/// enum exists solely to describe transfer endpoints.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeInstance {
    EventContract,
    Margined,
    #[serde(other)]
    Unknown,
}

impl ExchangeInstance {
    pub fn as_str(self) -> &'static str {
        match self {
            ExchangeInstance::EventContract => "event_contract",
            ExchangeInstance::Margined => "margined",
            ExchangeInstance::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ExchangeInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for ExchangeInstance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Status of an intra-exchange instance transfer.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntraExchangeInstanceTransferStatus {
    Pending,
    Complete,
    #[serde(other)]
    Unknown,
}

impl IntraExchangeInstanceTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IntraExchangeInstanceTransferStatus::Pending => "pending",
            IntraExchangeInstanceTransferStatus::Complete => "complete",
            IntraExchangeInstanceTransferStatus::Unknown => "unknown",
        }
    }
}

impl fmt::Display for IntraExchangeInstanceTransferStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for IntraExchangeInstanceTransferStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Request body for `POST /portfolio/intra_exchange_instance_transfer`.
///
/// NOTE: per the OpenAPI spec, `amount` here is a plain integer number of
/// *centicents* (not a [`FixedPointDollars`] string as used by
/// [`IntraExchangeInstanceTransfer::amount`] in list/get responses).
#[derive(Debug, Clone, Serialize)]
pub struct IntraExchangeInstanceTransferRequest {
    /// The source exchange instance.
    pub source: ExchangeInstance,
    /// The destination exchange instance.
    pub destination: ExchangeInstance,
    /// The amount to transfer, in centicents.
    pub amount: i64,
    /// Source exchange shard index. Defaults to 0 server-side when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_exchange_shard: Option<u32>,
    /// Destination exchange shard index. Defaults to 0 server-side when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_exchange_shard: Option<u32>,
    /// Source subaccount number. Defaults to 0 (primary account) server-side
    /// when omitted. Only supported for event-contract to event-contract
    /// transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subaccount: Option<u32>,
    /// Destination subaccount number. Defaults to 0 (primary account)
    /// server-side when omitted. Only supported for event-contract to
    /// event-contract transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_subaccount: Option<u32>,
}

/// Response for `POST /portfolio/intra_exchange_instance_transfer`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransferResponse {
    /// The ID of the transfer that was created.
    pub transfer_id: String,
}

/// A single intra-exchange instance transfer, as returned by the list/get endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransfer {
    pub transfer_id: String,
    pub source: ExchangeInstance,
    pub destination: ExchangeInstance,
    pub source_exchange_shard: u32,
    pub destination_exchange_shard: u32,
    /// Transfer amount in dollars.
    pub amount: FixedPointDollars,
    pub status: IntraExchangeInstanceTransferStatus,
    pub created_ts: i64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIntraExchangeInstanceTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response for `GET /portfolio/intra_exchange_instance_transfers`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransfersResponse {
    pub transfers: Vec<IntraExchangeInstanceTransfer>,
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Response for `GET /portfolio/intra_exchange_instance_transfers/{transfer_id}`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransferResponse {
    pub transfer: IntraExchangeInstanceTransfer,
}

impl KalshiRestClient {
    /// Get API rate-limit and position limits for the account.
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

    /// List API v2 endpoints whose token cost differs from the default cost.
    ///
    /// Public endpoint (no auth required per the OpenAPI spec).
    pub async fn get_account_endpoint_costs(
        &self,
    ) -> Result<GetAccountEndpointCostsResponse, KalshiError> {
        let path = Self::full_path("/account/endpoint_costs");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Get the account's latest cron-computed trading volume progress toward
    /// volume-based API usage tiers for the predictions (event_contract) lane.
    ///
    /// **Requires auth.**
    pub async fn get_account_api_usage_level_volume_progress(
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

    /// Upgrade the account to a permanent Advanced API usage-level grant for
    /// the predictions (event_contract) exchange instance. Requires that at
    /// least 1 of the user's last 100 Predictions orders was created via API.
    ///
    /// **Requires auth.**
    pub async fn upgrade_account_api_usage_level(&self) -> Result<EmptyResponse, KalshiError> {
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

    /// Transfer funds between exchange instances (and/or subaccounts) within
    /// the same account. The transfer is processed asynchronously.
    ///
    /// **Requires auth.**
    pub async fn create_intra_exchange_instance_transfer(
        &self,
        body: IntraExchangeInstanceTransferRequest,
    ) -> Result<IntraExchangeInstanceTransferResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfer");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// List intra-exchange instance transfers. Supports cursor pagination.
    ///
    /// **Requires auth.**
    pub async fn get_intra_exchange_instance_transfers(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> Result<GetIntraExchangeInstanceTransfersResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfers");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Get a single intra-exchange instance transfer by id.
    ///
    /// **Requires auth.**
    pub async fn get_intra_exchange_instance_transfer(
        &self,
        transfer_id: &str,
    ) -> Result<GetIntraExchangeInstanceTransferResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/portfolio/intra_exchange_instance_transfers/{transfer_id}"
        ));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Create a pager for iterating over intra-exchange instance transfers page by page.
    ///
    /// **Requires auth.** See [`CursorPager`].
    pub fn intra_exchange_instance_transfers_pager(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> CursorPager<IntraExchangeInstanceTransfer> {
        let client = self.clone();
        let base_params = params.clone();
        CursorPager::new(params.cursor.clone(), move |cursor| {
            let client = client.clone();
            let mut page_params = base_params.clone();
            page_params.cursor = cursor;
            Box::pin(async move {
                let resp = client
                    .get_intra_exchange_instance_transfers(page_params)
                    .await?;
                Ok((resp.transfers, resp.cursor))
            })
        })
    }

    /// Stream intra-exchange instance transfers one by one.
    ///
    /// **Requires auth.**
    pub fn stream_intra_exchange_instance_transfers(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
        max_items: Option<usize>,
    ) -> impl Stream<Item = Result<IntraExchangeInstanceTransfer, KalshiError>> + Send {
        stream_items(
            self.intra_exchange_instance_transfers_pager(params),
            max_items,
        )
    }

    /// Fetch all pages for intra-exchange instance transfers using cursor pagination.
    pub async fn get_intra_exchange_instance_transfers_all(
        &self,
        params: GetIntraExchangeInstanceTransfersParams,
    ) -> Result<Vec<IntraExchangeInstanceTransfer>, KalshiError> {
        self.paginate_cursor(params.cursor.clone(), |cursor| {
            let mut page_params = params.clone();
            page_params.cursor = cursor;
            async move {
                let resp = self
                    .get_intra_exchange_instance_transfers(page_params)
                    .await?;
                Ok((resp.transfers, resp.cursor))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    //! Serde round-trip / shape tests for the types added or changed in this
    //! file. `src/rest/account.rs` has no paired `tests/rest_account.rs` and
    //! no pre-existing `#[cfg(test)]` module of its own (unlike
    //! `src/rest/client.rs`, which drives a real mock TCP server), so these
    //! are plain unit tests mirroring the style of `tests/parsing.rs`.

    use super::*;

    // ------------------------------------------------------------------
    // API usage-level volume progress
    // ------------------------------------------------------------------

    #[test]
    fn get_account_api_usage_level_volume_progress_response_deserializes() {
        let json = r#"{
            "volume_progress": [{
                "computed_ts": 1700000000,
                "trailing_30d_volume_fp": "1234.56",
                "goals": [
                    {"level": "expert", "earn_volume_goal_fp": "1000.00", "keep_volume_goal_fp": "500.00"},
                    {"level": "premier", "earn_volume_goal_fp": "5000.00", "keep_volume_goal_fp": "2500.00"}
                ]
            }]
        }"#;

        let resp: GetAccountApiUsageLevelVolumeProgressResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(resp.volume_progress.len(), 1);
        assert_eq!(resp.volume_progress[0].computed_ts, 1700000000);
        assert_eq!(resp.volume_progress[0].trailing_30d_volume_fp, "1234.56");
        assert_eq!(resp.volume_progress[0].goals.len(), 2);
        assert_eq!(resp.volume_progress[0].goals[0].level, "expert");
        assert_eq!(
            resp.volume_progress[0].goals[1].earn_volume_goal_fp,
            "5000.00"
        );
    }

    #[test]
    fn get_account_api_usage_level_volume_progress_response_round_trips() {
        let original = GetAccountApiUsageLevelVolumeProgressResponse {
            volume_progress: vec![AccountApiUsageLevelVolumeProgress {
                computed_ts: 42,
                trailing_30d_volume_fp: "10.00".to_string(),
                goals: vec![AccountApiUsageLevelVolumeGoal {
                    level: "expert".to_string(),
                    earn_volume_goal_fp: "1.00".to_string(),
                    keep_volume_goal_fp: "0.50".to_string(),
                }],
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let round_tripped: GetAccountApiUsageLevelVolumeProgressResponse =
            serde_json::from_str(&json).unwrap();
        assert_eq!(
            round_tripped.volume_progress[0].computed_ts,
            original.volume_progress[0].computed_ts
        );
        assert_eq!(
            round_tripped.volume_progress[0].goals[0].level,
            original.volume_progress[0].goals[0].level
        );
    }

    #[test]
    fn upgrade_account_api_usage_level_response_deserializes_empty_body() {
        // The server responds 201 with no body; `send()` substitutes `{}`.
        let resp: EmptyResponse = serde_json::from_str("{}").unwrap();
        let _ = resp;
    }

    // ------------------------------------------------------------------
    // ApiKeyType
    // ------------------------------------------------------------------

    #[test]
    fn api_key_type_serializes_correctly() {
        assert_eq!(serde_json::to_string(&ApiKeyType::Rsa).unwrap(), "\"rsa\"");
        assert_eq!(
            serde_json::to_string(&ApiKeyType::Ed25519).unwrap(),
            "\"ed25519\""
        );
    }

    #[test]
    fn api_key_type_deserializes_correctly() {
        assert!(matches!(
            serde_json::from_str::<ApiKeyType>("\"rsa\"").unwrap(),
            ApiKeyType::Rsa
        ));
        assert!(matches!(
            serde_json::from_str::<ApiKeyType>("\"ed25519\"").unwrap(),
            ApiKeyType::Ed25519
        ));
        // Forward-compat catch-all.
        assert!(matches!(
            serde_json::from_str::<ApiKeyType>("\"some_future_type\"").unwrap(),
            ApiKeyType::Unknown
        ));
    }

    // ------------------------------------------------------------------
    // API key request/response fields: key_type, subaccount,
    // api_key_region_expiration_ts
    // ------------------------------------------------------------------

    #[test]
    fn generate_api_key_request_serializes_key_type_and_subaccount() {
        let req = GenerateApiKeyRequest {
            name: "my-key".to_string(),
            key_type: Some(ApiKeyType::Ed25519),
            scopes: vec![],
            subaccount: Some(5),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "my-key");
        assert_eq!(json["key_type"], "ed25519");
        assert_eq!(json["subaccount"], 5);
        assert!(json.get("scopes").is_none());
    }

    #[test]
    fn generate_api_key_request_omits_none_fields() {
        let req = GenerateApiKeyRequest {
            name: "my-key".to_string(),
            key_type: None,
            scopes: vec![],
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("key_type").is_none());
        assert!(json.get("subaccount").is_none());
    }

    #[test]
    fn generate_api_key_response_deserializes_key_type() {
        let json = r#"{
            "api_key_id": "key-1",
            "key_type": "ed25519",
            "private_key": "-----BEGIN PRIVATE KEY-----..."
        }"#;
        let resp: GenerateApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.api_key_id, "key-1");
        assert!(matches!(resp.key_type, Some(ApiKeyType::Ed25519)));
        assert_eq!(resp.private_key, "-----BEGIN PRIVATE KEY-----...");
    }

    #[test]
    fn generate_api_key_response_tolerates_missing_key_type() {
        let json = r#"{"api_key_id": "key-1", "private_key": "pk"}"#;
        let resp: GenerateApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(resp.key_type.is_none());
    }

    #[test]
    fn create_api_key_request_serializes_subaccount() {
        let req = CreateApiKeyRequest {
            name: "my-key".to_string(),
            public_key: "-----BEGIN PUBLIC KEY-----...".to_string(),
            scopes: vec![],
            subaccount: Some(3),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["subaccount"], 3);
    }

    #[test]
    fn create_api_key_request_omits_none_subaccount() {
        let req = CreateApiKeyRequest {
            name: "my-key".to_string(),
            public_key: "pk".to_string(),
            scopes: vec![],
            subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("subaccount").is_none());
    }

    #[test]
    fn api_key_deserializes_with_subaccount() {
        let json = r#"{
            "api_key_id": "key-1",
            "name": "my key",
            "scopes": ["read", "write"],
            "subaccount": 7
        }"#;
        let key: ApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.subaccount, Some(7));
    }

    #[test]
    fn api_key_tolerates_missing_subaccount() {
        let json = r#"{"api_key_id": "key-1", "name": "my key", "scopes": []}"#;
        let key: ApiKey = serde_json::from_str(json).unwrap();
        assert!(key.subaccount.is_none());
    }

    #[test]
    fn get_api_keys_response_deserializes_region_expiration_ts() {
        let json = r#"{
            "api_keys": [{"api_key_id": "key-1", "name": "k", "scopes": []}],
            "api_key_region_expiration_ts": 1700000000
        }"#;
        let resp: GetApiKeysResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.api_key_region_expiration_ts, Some(1700000000));
    }

    #[test]
    fn get_api_keys_response_tolerates_missing_region_expiration_ts() {
        let json = r#"{"api_keys": []}"#;
        let resp: GetApiKeysResponse = serde_json::from_str(json).unwrap();
        assert!(resp.api_key_region_expiration_ts.is_none());
    }

    // ------------------------------------------------------------------
    // SubaccountBalance.exchange_index
    // ------------------------------------------------------------------

    #[test]
    fn subaccount_balance_deserializes_exchange_index() {
        let json = r#"{
            "subaccount_number": 1,
            "exchange_index": 2,
            "balance": 100,
            "updated_ts": 1700000000
        }"#;
        let balance: SubaccountBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.subaccount_number, 1);
        assert_eq!(balance.exchange_index, 2);
        assert_eq!(balance.balance, "100");
    }

    // ------------------------------------------------------------------
    // Intra-exchange instance transfer
    // ------------------------------------------------------------------

    #[test]
    fn exchange_instance_serializes_and_deserializes() {
        assert_eq!(
            serde_json::to_string(&ExchangeInstance::EventContract).unwrap(),
            "\"event_contract\""
        );
        assert_eq!(
            serde_json::to_string(&ExchangeInstance::Margined).unwrap(),
            "\"margined\""
        );
        assert!(matches!(
            serde_json::from_str::<ExchangeInstance>("\"event_contract\"").unwrap(),
            ExchangeInstance::EventContract
        ));
        assert!(matches!(
            serde_json::from_str::<ExchangeInstance>("\"some_future_lane\"").unwrap(),
            ExchangeInstance::Unknown
        ));
    }

    #[test]
    fn intra_exchange_instance_transfer_status_serializes_and_deserializes() {
        assert_eq!(
            serde_json::to_string(&IntraExchangeInstanceTransferStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&IntraExchangeInstanceTransferStatus::Complete).unwrap(),
            "\"complete\""
        );
        assert!(matches!(
            serde_json::from_str::<IntraExchangeInstanceTransferStatus>("\"pending\"").unwrap(),
            IntraExchangeInstanceTransferStatus::Pending
        ));
    }

    #[test]
    fn intra_exchange_instance_transfer_request_serializes_all_fields() {
        let req = IntraExchangeInstanceTransferRequest {
            source: ExchangeInstance::EventContract,
            destination: ExchangeInstance::Margined,
            amount: 12345,
            source_exchange_shard: Some(1),
            destination_exchange_shard: Some(2),
            source_subaccount: Some(3),
            destination_subaccount: Some(4),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["source"], "event_contract");
        assert_eq!(json["destination"], "margined");
        // Amount is a plain integer (centicents) per the spec, not a
        // FixedPointDollars string.
        assert_eq!(json["amount"], 12345);
        assert_eq!(json["source_exchange_shard"], 1);
        assert_eq!(json["destination_exchange_shard"], 2);
        assert_eq!(json["source_subaccount"], 3);
        assert_eq!(json["destination_subaccount"], 4);
    }

    #[test]
    fn intra_exchange_instance_transfer_request_omits_none_optional_fields() {
        let req = IntraExchangeInstanceTransferRequest {
            source: ExchangeInstance::EventContract,
            destination: ExchangeInstance::EventContract,
            amount: 100,
            source_exchange_shard: None,
            destination_exchange_shard: None,
            source_subaccount: None,
            destination_subaccount: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("source_exchange_shard").is_none());
        assert!(json.get("destination_exchange_shard").is_none());
        assert!(json.get("source_subaccount").is_none());
        assert!(json.get("destination_subaccount").is_none());
    }

    #[test]
    fn intra_exchange_instance_transfer_response_deserializes() {
        let json = r#"{"transfer_id": "xfer-1"}"#;
        let resp: IntraExchangeInstanceTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transfer_id, "xfer-1");
    }

    #[test]
    fn intra_exchange_instance_transfer_deserializes() {
        let json = r#"{
            "transfer_id": "xfer-1",
            "source": "event_contract",
            "destination": "margined",
            "source_exchange_shard": 0,
            "destination_exchange_shard": 1,
            "amount": "12.3400",
            "status": "pending",
            "created_ts": 1700000000
        }"#;
        let transfer: IntraExchangeInstanceTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.transfer_id, "xfer-1");
        assert!(matches!(transfer.source, ExchangeInstance::EventContract));
        assert!(matches!(transfer.destination, ExchangeInstance::Margined));
        assert_eq!(transfer.destination_exchange_shard, 1);
        // Amount here IS a FixedPointDollars string, unlike the request's
        // plain-integer `amount` field.
        assert_eq!(transfer.amount, "12.3400");
        assert!(matches!(
            transfer.status,
            IntraExchangeInstanceTransferStatus::Pending
        ));
        assert_eq!(transfer.created_ts, 1700000000);
    }

    #[test]
    fn get_intra_exchange_instance_transfers_response_deserializes() {
        let json = r#"{
            "transfers": [{
                "transfer_id": "xfer-1",
                "source": "event_contract",
                "destination": "event_contract",
                "source_exchange_shard": 0,
                "destination_exchange_shard": 0,
                "amount": "1.0000",
                "status": "complete",
                "created_ts": 1700000000
            }],
            "cursor": "next-cursor"
        }"#;
        let resp: GetIntraExchangeInstanceTransfersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transfers.len(), 1);
        assert_eq!(resp.cursor, Some("next-cursor".to_string()));
    }

    #[test]
    fn get_intra_exchange_instance_transfers_response_tolerates_missing_cursor() {
        let json = r#"{"transfers": []}"#;
        let resp: GetIntraExchangeInstanceTransfersResponse = serde_json::from_str(json).unwrap();
        assert!(resp.transfers.is_empty());
        assert!(resp.cursor.is_none());
    }

    #[test]
    fn get_intra_exchange_instance_transfer_response_deserializes() {
        let json = r#"{
            "transfer": {
                "transfer_id": "xfer-1",
                "source": "event_contract",
                "destination": "margined",
                "source_exchange_shard": 0,
                "destination_exchange_shard": 0,
                "amount": "1.0000",
                "status": "pending",
                "created_ts": 1700000000
            }
        }"#;
        let resp: GetIntraExchangeInstanceTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transfer.transfer_id, "xfer-1");
    }

    #[test]
    fn get_intra_exchange_instance_transfers_params_serializes_correctly() {
        let params = GetIntraExchangeInstanceTransfersParams {
            cursor: Some("c1".to_string()),
            limit: Some(50),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["cursor"], "c1");
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn get_intra_exchange_instance_transfers_params_omits_none_fields() {
        let params = GetIntraExchangeInstanceTransfersParams::default();
        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("cursor").is_none());
        assert!(json.get("limit").is_none());
    }
}
