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
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

/// One volume-based API usage tier's earn/keep goals within [`AccountApiUsageLevelVolumeProgress`].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeGoal {
    /// API usage level this goal describes (e.g. `"expert"`, `"premier"`).
    pub level: String,
    /// Trailing-30d fixed-point contract volume required to earn this level.
    pub earn_volume_goal_fp: FixedPointCount,
    /// Trailing-30d fixed-point contract volume required to keep this level.
    pub keep_volume_goal_fp: FixedPointCount,
}

/// One computed snapshot of trailing-30d volume progress toward volume-based tiers.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountApiUsageLevelVolumeProgress {
    /// Unix timestamp (seconds) this snapshot was computed at.
    pub computed_ts: i64,
    /// Trailing-30d fixed-point contract volume as of `computed_ts`.
    pub trailing_30d_volume_fp: FixedPointCount,
    pub goals: Vec<AccountApiUsageLevelVolumeGoal>,
}

/// Response for `GET /account/api_usage_level/volume_progress`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountApiUsageLevelVolumeProgressResponse {
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

/// POST /portfolio/subaccounts request body.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSubaccountRequest {
    /// Exchange shard to create the subaccount on. Defaults to 0 if unspecified.
    /// Added 2026-07-02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubaccountResponse {
    pub subaccount_number: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubaccountBalance {
    pub subaccount_number: u32,
    /// Exchange index this balance is held on. Added 2026-07-02.
    #[serde(default)]
    pub exchange_index: Option<i64>,
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
    /// Exchange shard to apply the transfer on. Defaults to 0 if unspecified. Added
    /// 2026-07-02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i64>,
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

// --- Intra-exchange-instance transfers (added 2026-08-13) ---
//
// Distinct from subaccount transfers above: these move funds between exchange
// *instances* (e.g. event-contract vs. margined), optionally also crossing exchange
// shards and subaccounts. `source`/`destination` are kept as `String` rather than a
// typed `ExchangeInstance` enum so the raw value round-trips losslessly and tolerates
// future exchange-instance additions without a crate update (same rationale as
// `ApiUsageLevelGrant::exchange_instance`).

/// `POST /portfolio/intra_exchange_instance_transfer` request body.
#[derive(Debug, Clone, Serialize)]
pub struct IntraExchangeInstanceTransferRequest {
    pub source: String,
    pub destination: String,
    /// Amount to transfer in centicents.
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_exchange_shard: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_exchange_shard: Option<i64>,
    /// Only supported for event-contract to event-contract transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransferResponse {
    pub transfer_id: String,
}

/// `GET /portfolio/intra_exchange_instance_transfers` query params.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIntraExchangeInstanceTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntraExchangeInstanceTransfer {
    pub transfer_id: String,
    pub source: String,
    pub destination: String,
    pub source_exchange_shard: i64,
    pub destination_exchange_shard: i64,
    /// Transfer amount in fixed-point dollars.
    pub amount: FixedPointDollars,
    /// `"pending"` | `"complete"`.
    pub status: String,
    pub created_ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransfersResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub transfers: Vec<IntraExchangeInstanceTransfer>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetIntraExchangeInstanceTransferResponse {
    pub transfer: IntraExchangeInstanceTransfer,
}

// --- Target balance allocation (added 2026-08-20) ---

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetBalanceAllocation {
    pub exchange_index: i64,
    /// Target percentage (0-100) of sweepable balance for this exchange index.
    pub percent: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTargetBalanceAllocationResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub allocations: Vec<TargetBalanceAllocation>,
}

/// Collateral an automatic rebalance leaves behind for resting orders.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestingMarginReservation {
    /// Reserve the largest single market-side commitment.
    Max,
    /// Reserve the summed margin of every resting order. This is the default when
    /// the field is omitted.
    Sum,
}

/// `POST /portfolio/target_balance_allocation` request body. Percentages must total
/// 100; an empty `allocations` array disables automatic rebalancing.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetTargetBalanceAllocationRequest {
    pub allocations: Vec<TargetBalanceAllocation>,
    /// Defaults to `sum` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting_margin_reservation: Option<RestingMarginReservation>,
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
    /// If set, this key is restricted to a single sub-account (0-63) and may only
    /// read and trade on it. Added 2026-07-02.
    #[serde(default)]
    pub subaccount: Option<u32>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetApiKeysResponse {
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub api_keys: Vec<ApiKey>,
    /// Unix timestamp (seconds) when the account's location attestation for API key
    /// requests expires. Added 2026-08-16. Absent when the account has never attested.
    #[serde(default)]
    pub api_key_region_expiration_ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    /// Restrict the new key to a single sub-account (0-63) that you own. Added
    /// 2026-07-02. Omit to leave the key unrestricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateApiKeyRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    /// Restrict the new key to a single sub-account (0-63) that you own. Added
    /// 2026-07-02. Omit to leave the key unrestricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
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
    /// Exchange shard index. Supported on the order-group mutation endpoints
    /// (delete/reset/trigger/update-limit); ignored elsewhere. Added 2026-08-20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i64>,
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

    /// Self-promote to the Advanced API usage level.
    ///
    /// Grants a permanent Advanced usage-level grant for the Predictions exchange
    /// instance, provided at least one of the user's last 100 Predictions orders was
    /// created via API. Use [`get_account_api_limits`](Self::get_account_api_limits)
    /// to inspect the resulting tier and grants.
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

    /// Get trailing-30d volume progress toward volume-based API usage tiers.
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

    /// Create a new subaccount.
    ///
    /// **Requires auth.**
    pub async fn create_subaccount(
        &self,
        body: CreateSubaccountRequest,
    ) -> Result<CreateSubaccountResponse, KalshiError> {
        let path = Self::full_path("/portfolio/subaccounts");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
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

    /// Transfer funds between exchange instances (e.g. event-contract and margined),
    /// optionally also crossing exchange shards and subaccounts. Runs in up to three
    /// non-atomic steps; if a later step fails, completed steps are not undone.
    ///
    /// **Requires auth.**
    pub async fn intra_exchange_instance_transfer(
        &self,
        body: IntraExchangeInstanceTransferRequest,
    ) -> Result<IntraExchangeInstanceTransferResponse, KalshiError> {
        let path = Self::full_path("/portfolio/intra_exchange_instance_transfer");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
            .await
    }

    /// List intra-exchange-instance transfer history. Supports cursor pagination.
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

    /// Get a single intra-exchange-instance transfer by ID.
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

    /// Get the caller's target balance allocation across exchange indexes.
    ///
    /// **Requires auth.**
    pub async fn get_target_balance_allocation(
        &self,
    ) -> Result<GetTargetBalanceAllocationResponse, KalshiError> {
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Replace the caller's target balance allocation across exchange indexes.
    ///
    /// Percentages must total 100. Passing an empty `allocations` array disables
    /// automatic rebalancing.
    ///
    /// **Requires auth.**
    pub async fn set_target_balance_allocation(
        &self,
        body: SetTargetBalanceAllocationRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/target_balance_allocation");
        self.send(Method::POST, &path, Option::<&()>::None, Some(&body), true)
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

    /// Create a pager for iterating over intra-exchange-instance transfers page by page.
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

    /// Stream intra-exchange-instance transfers one by one.
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
}
