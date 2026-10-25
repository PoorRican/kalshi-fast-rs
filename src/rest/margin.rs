use crate::error::KalshiError;
use crate::rest::client::KalshiRestClient;
use crate::types::BookSide;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Query parameter helpers
// ---------------------------------------------------------------------------

/// Query parameters for `GET /margin/orders`.
///
/// `limit` accepts 1-10000 (server default 10000).
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginFillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginPositionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

/// Query parameters for `GET /margin/trades`.
///
/// Omit `ticker` to retrieve trades across all margin markets.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginTradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginMarketCandlesticksParams {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginMarketOrderbookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_tick_size: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginFundingHistoryParams {
    pub start_date: String,
    pub end_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginHistoricalFundingRatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
}

/// Query parameters for `GET /margin/corporate_actions`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginCorporateActionsParams {
    /// Filter by market ticker. Omit for all margin markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Filter by market UUID. Omit for all margin markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
}

/// Query parameters for `GET /margin/funding_rates/premium_index`.
///
/// The window `[start_ts, end_ts)` is in Unix seconds and may span at most one hour.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginPremiumIndexParams {
    pub ticker: String,
    pub start_ts: i64,
    pub end_ts: i64,
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

/// Body for `POST /margin/orders`.
///
/// Supply `ticker` or `market_id`; `ticker` takes precedence when both are set.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Stable exchange market UUID. May be supplied instead of `ticker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    pub client_order_id: String,
    pub side: BookSide,
    pub count: String,
    pub price: String,
    pub time_in_force: MarginTimeInForce,
    pub self_trade_prevention_type: MarginSelfTradePreventionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    /// The `market_version` you expect the market to be at. If set (non-zero) and the
    /// market's current version differs, the order is rejected with HTTP 409 and error
    /// code `market_version_mismatch`. `0` or `None` skips the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_version: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecreaseMarginOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<String>,
}

/// Body for `POST /margin/orders/{order_id}/amend`.
///
/// Supply `ticker` or `market_id`; `ticker` takes precedence when both are set.
#[derive(Debug, Clone, Serialize)]
pub struct AmendMarginOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Stable exchange market UUID. May be supplied instead of `ticker`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    pub side: BookSide,
    pub price: String,
    pub count: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    /// New Unix expiration (seconds). Omit to preserve the current expiry, `0` to remove
    /// it (good-till-canceled); a nonzero value must be in the future. For an expiry-only
    /// amendment send the current `price` and total `count` to preserve queue position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginOrderGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMarginOrderGroupLimitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMarginOrderGroupResponse {
    pub order_group_id: String,
    pub subaccount: u32,
    #[serde(default)]
    pub exchange_index: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrderGroupsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub order_groups: Vec<MarginOrderGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrderGroup {
    pub id: String,
    #[serde(default)]
    pub contracts_limit_fp: Option<String>,
    pub is_auto_cancel_enabled: bool,
    #[serde(default)]
    pub exchange_index: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrderGroupResponse {
    pub is_auto_cancel_enabled: bool,
    #[serde(default)]
    pub contracts_limit_fp: Option<String>,
    pub orders: Vec<String>,
    #[serde(default)]
    pub exchange_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyMarginSubaccountTransferRequest {
    pub client_transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginTimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginSelfTradePreventionType {
    TakerAtCross,
    Maker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginMarketStatus {
    Inactive,
    Active,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginOrderReason {
    Liquidation,
    TakeProfitStopLoss,
}

/// Unit for [`MarginMarket::funding_deadband`] and [`MarginMarket::funding_cap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginFundingLimitUnit {
    Percentage,
    Dollar,
}

/// How funding averages a bar that has no premium.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginFundingInvalidPremiumHandling {
    /// The bar counts as a premium of 0.
    Dampen,
    /// The bar is left out of the average.
    Exclude,
}

/// Exit-trigger family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerKind {
    /// Stop-loss / take-profit pair.
    Bracket,
    /// Trailing stop whose level ratchets with the liquidation mark.
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerStatus {
    PendingOnEntry,
    Active,
    Filled,
    Failed,
    Canceled,
    Unknown,
}

/// Which bracket leg fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerLeg {
    StopLoss,
    TakeProfit,
}

/// Why an exit trigger reached its terminal status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerReason {
    UserCanceled,
    PositionClosed,
    EntryNotFilled,
    OrderRejected,
    PositionFlipped,
}

/// Asset class scope for FCM subtrader initial-margin caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginFcmAssetClass {
    Crypto,
    Equities,
    Metals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginCorporateActionType {
    CashDividend,
    SpecialDividend,
    SpinOff,
    StockSplit,
    FinalSettlement,
    TickerChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginCorporateActionStatus {
    Scheduled,
    Cancelled,
    Applied,
    Lapsed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginFeeSchedule {
    SelfClearingMembers,
    KalshiPrime,
    Fcm,
}

// ---------------------------------------------------------------------------
// Exit-trigger and FCM request bodies
// ---------------------------------------------------------------------------

/// Body for `PUT /margin/isolated/positions/{ticker}/exit_trigger`.
///
/// For `bracket` supply `stop_loss_price` and/or `take_profit_price`; for `trailing`
/// supply exactly one of `trail_amount` / `trail_bps`. A leg you omit is cleared.
/// The server rejects a `stop_loss_price` at or past the position's liquidation price
/// (`400 stop_loss_past_liquidation`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetIsolatedExitTriggerRequest {
    /// Defaults to `bracket` server-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExitTriggerKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    /// Trailing distance in basis points of the watermark (1-9999).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<u32>,
}

/// Body for `PUT /margin/cross/positions/{ticker}/exit_trigger`.
///
/// See [`SetIsolatedExitTriggerRequest`] for the shared leg rules.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCrossExitTriggerRequest {
    /// Contracts this trigger closes; whole number. Omit to cover the whole position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// Ties the trigger to an order (bracket only; mutually exclusive with `count`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    /// Idempotency key (max 64 chars); required on `count` / `anchor_order_id` creates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExitTriggerKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<u32>,
}

/// Body for `PUT /margin/cross/positions/{ticker}/exit_trigger/{trigger_id}`.
///
/// At least one leg is required and an omitted leg is cleared.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateExitTriggerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
}

/// Body for `POST /margin/fcm/subtraders`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginFcmSubtraderRequest {
    /// 1-16 alphanumeric characters; the full id is `{user_id}_{subtrader_suffix}`.
    pub subtrader_suffix: String,
    /// When true, ordinary orders require an explicit asset-class initial-margin cap.
    pub require_category_cap: bool,
}

/// Body for `PUT /margin/fcm/subtraders/risk_controls`.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateFcmSubtraderRiskControlsRequest {
    pub subtrader_id: String,
    /// Scopes the cap to one market. Mutually exclusive with `asset_class`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<MarginFcmAssetClass>,
    /// Non-negative fixed-point dollars (up to 4 decimals).
    pub im_cap: String,
}

/// Query parameters for `GET /margin/fcm/subtraders/risk_controls`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFcmSubtraderRiskControlsParams {
    /// Optional for keys bound to a single subtrader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<MarginFcmAssetClass>,
}

/// Query parameters for `DELETE /margin/fcm/subtraders/risk_controls`.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteFcmSubtraderRiskControlsParams {
    pub subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<MarginFcmAssetClass>,
}

/// Body for `PUT /margin/fcm/notional_risk_limit`.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateFcmNotionalRiskLimitRequest {
    /// Non-negative fixed-point dollars (up to 4 decimals).
    pub notional_value_risk_limit: String,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginExchangeStatusResponse {
    pub exchange_active: bool,
    pub trading_active: bool,
    /// Estimated end of the current maintenance window (RFC 3339). Not guaranteed and can
    /// be extended. `None` when absent or null.
    ///
    /// The shared schema also lists `intra_exchange_transfers_active` and
    /// `exchange_index_statuses`, which the Margin exchange status never includes.
    #[serde(default)]
    pub exchange_estimated_resume_time: Option<String>,
}

/// Response for `GET /margin/corporate_actions`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginCorporateActionsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub corporate_actions: Vec<MarginCorporateAction>,
}

/// A margin corporate action notice.
///
/// Scheduled, applied and lapsed notices carry exactly one terms object matching `type`
/// (`cash`, `split`, `final_settlement` or `ticker_change`); cancelled notices carry none
/// and omit `execution_time`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginCorporateAction {
    /// Stable notice id (the Klear review id).
    pub id: String,
    pub market_id: String,
    pub market_ticker: String,
    #[serde(rename = "type")]
    pub action_type: MarginCorporateActionType,
    pub status: MarginCorporateActionStatus,
    pub updated_ts: String,
    #[serde(default)]
    pub execution_time: Option<String>,
    #[serde(default)]
    pub cash: Option<MarginCorporateActionCashTerms>,
    #[serde(default)]
    pub split: Option<MarginCorporateActionSplitTerms>,
    #[serde(default)]
    pub final_settlement: Option<MarginCorporateActionFinalSettlementTerms>,
    #[serde(default)]
    pub ticker_change: Option<MarginCorporateActionTickerChangeTerms>,
}

/// Terms for `cash_dividend`, `special_dividend` and `spin_off`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginCorporateActionCashTerms {
    #[serde(default)]
    pub cash_per_share: Option<String>,
    /// A positive amount credits longs and debits shorts.
    #[serde(default)]
    pub cash_per_contract: Option<String>,
    /// `YYYY-MM-DD`, where available.
    #[serde(default)]
    pub ex_date: Option<String>,
}

/// Terms for `stock_split`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginCorporateActionSplitTerms {
    #[serde(default)]
    pub ratio_old: Option<String>,
    #[serde(default)]
    pub ratio_new: Option<String>,
    /// `YYYY-MM-DD`, where available.
    #[serde(default)]
    pub ex_date: Option<String>,
}

/// Terms for `final_settlement`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginCorporateActionFinalSettlementTerms {
    /// `bankruptcy`, `merger`, `delisting` or `other`.
    pub reason: String,
    #[serde(default)]
    pub settlement_price_per_share: Option<String>,
    #[serde(default)]
    pub settlement_price_per_contract: Option<String>,
}

/// Terms for `ticker_change`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginCorporateActionTickerChangeTerms {
    pub old_ticker: String,
    pub new_ticker: String,
}

/// Response for `GET /margin/markets/orderbooks`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginMarketOrderbooksResponse {
    /// One entry per requested ticker, in request order.
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub orderbooks: Vec<MarginMarketOrderbook>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketOrderbook {
    pub ticker: String,
    pub orderbook: MarginOrderbookCount,
}

/// Response for `GET /margin/funding_rates/premium_index`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginPremiumIndexResponse {
    /// Per-second points, ascending by `second_ts`.
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub points: Vec<MarginPremiumIndexPoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginPremiumIndexPoint {
    /// The one-second bucket (RFC 3339).
    pub second_ts: String,
    /// Signed decimal fraction of the index price (not bps); informational only.
    pub premium_index: String,
}

/// Response for `GET /margin/fee_tier_rates`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFeeTierRatesResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub fee_tier_rates: Vec<MarginFeeTierRate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFeeTierRate {
    pub fee_schedule: MarginFeeSchedule,
    pub tier: u32,
    /// Decimal fraction of notional (0.0005 = 5 bps).
    pub maker_fee_rate: f64,
    /// Decimal fraction of notional (0.0012 = 12 bps).
    pub taker_fee_rate: f64,
}

/// An exit trigger (stop-loss / take-profit bracket or trailing stop).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExitTrigger {
    pub id: String,
    pub ticker: String,
    pub kind: ExitTriggerKind,
    pub status: ExitTriggerStatus,
    #[serde(default)]
    pub status_reason: Option<ExitTriggerReason>,
    #[serde(default)]
    pub triggered_leg: Option<ExitTriggerLeg>,
    #[serde(default)]
    pub triggered_order_id: Option<String>,
    #[serde(default)]
    pub anchor_order_id: Option<String>,
    #[serde(default)]
    pub client_trigger_id: Option<String>,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub trail_amount: Option<String>,
    #[serde(default)]
    pub trail_bps: Option<i64>,
    #[serde(default)]
    pub watermark_price: Option<String>,
    #[serde(default)]
    pub effective_stop_price: Option<String>,
    /// Contracts this trigger closes when it fires; `0` means the whole position.
    pub count: String,
    pub filled_count: String,
    pub created_time: String,
    pub updated_time: String,
}

/// Response for the exit-trigger `GET` endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetExitTriggersResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub exit_triggers: Vec<ExitTrigger>,
}

/// Response for `POST /margin/fcm/subtraders`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMarginFcmSubtraderResponse {
    pub subtrader_id: String,
}

/// Response for `GET /margin/fcm/subtraders/risk_controls`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFcmSubtraderRiskControlsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub risk_controls: Vec<FcmSubtraderRiskControl>,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub notional_limits: Vec<FcmSubtraderNotionalRiskLimit>,
}

/// An FCM-set initial-margin cap. Neither `market_ticker` nor `asset_class` means the cap
/// applies across all markets.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FcmSubtraderRiskControl {
    pub subtrader_id: String,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub asset_class: Option<MarginFcmAssetClass>,
    pub im_cap: String,
    pub current_im: String,
}

/// An admin-set notional value risk limit. No `market_ticker` means the whole-subtrader limit.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FcmSubtraderNotionalRiskLimit {
    pub subtrader_id: String,
    #[serde(default)]
    pub market_ticker: Option<String>,
    pub notional_value_risk_limit: String,
    pub current_notional: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginEnabledResponse {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketResponse {
    pub market: MarginMarket,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginMarketsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub markets: Vec<MarginMarket>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarket {
    /// Stable market UUID; unchanged when the ticker is renamed.
    #[serde(default)]
    pub market_id: Option<String>,
    pub ticker: String,
    pub status: MarginMarketStatus,
    pub title: String,
    /// Current market version. Starts at 1 and increases when trading resumes after a
    /// corporate action (e.g. a stock split). Pass it as
    /// [`CreateMarginOrderRequest::market_version`] to reject orders placed against a stale
    /// version. A read can briefly return the previous version right after trading resumes.
    pub market_version: i32,
    /// Funding deadband, expressed in `funding_limit_unit`.
    pub funding_deadband: f64,
    /// Funding cap, expressed in `funding_limit_unit`.
    pub funding_cap: f64,
    pub funding_limit_unit: MarginFundingLimitUnit,
    pub funding_invalid_premium_handling: MarginFundingInvalidPremiumHandling,
    pub contract_size: String,
    pub underlying_multiplier: String,
    pub tick_size: String,
    pub fractional_trading_enabled: bool,
    #[serde(default)]
    pub schedule: Option<MarginMarketSchedule>,
    pub exchange_index: u32,
    #[serde(default)]
    pub leverage_estimate: Option<f64>,
    #[serde(default)]
    pub leverage_estimates: Option<BTreeMap<String, f64>>,
    #[serde(default)]
    pub long_leverage_estimates: Option<BTreeMap<String, f64>>,
    #[serde(default)]
    pub short_leverage_estimates: Option<BTreeMap<String, f64>>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub volume_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub open_interest: Option<String>,
    #[serde(default)]
    pub open_interest_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub volume_24h: Option<String>,
    #[serde(default)]
    pub volume_24h_notional_value_dollars: Option<String>,
    #[serde(default)]
    pub bid: Option<String>,
    /// Contracts resting at the best bid (`"0.00"` when empty). Omitted when market stats
    /// are unavailable.
    #[serde(default)]
    pub bid_size_fp: Option<String>,
    #[serde(default)]
    pub ask: Option<String>,
    /// Contracts resting at the best ask (`"0.00"` when empty). Omitted when market stats
    /// are unavailable.
    #[serde(default)]
    pub ask_size_fp: Option<String>,
    #[serde(default)]
    pub settlement_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub liquidation_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub reference_price: Option<TickerPrice>,
    #[serde(default)]
    pub asset_class: Option<String>,
    #[serde(default)]
    pub product_metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketSchedule {
    pub is_open: bool,
    #[serde(default)]
    pub next_close_ts: Option<i64>,
    #[serde(default)]
    pub next_open_ts: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerPrice {
    pub price: String,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrderbookResponse {
    pub orderbook: MarginOrderbookCount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrderbookCount {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub bids: Vec<Vec<String>>,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginMarketCandlesticksResponse {
    pub ticker: String,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub candlesticks: Vec<MarginMarketCandlestick>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginMarketCandlestick {
    pub end_period_ts: i64,
    pub bid: MarginBidAskDistribution,
    pub ask: MarginBidAskDistribution,
    pub price: MarginPriceDistribution,
    pub volume: String,
    pub volume_notional_value_dollars: String,
    pub open_interest: String,
    pub open_interest_notional_value_dollars: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginBidAskDistribution {
    pub open: String,
    pub low: String,
    pub high: String,
    pub close: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginPriceDistribution {
    #[serde(default)]
    pub open: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub close: Option<String>,
    #[serde(default)]
    pub mean: Option<String>,
    #[serde(default)]
    pub previous: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub fill_count: String,
    pub remaining_count: String,
    #[serde(default)]
    pub average_fill_price: Option<String>,
    #[serde(default)]
    pub average_fee_paid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrderResponse {
    pub order: MarginOrder,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginOrdersResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub orders: Vec<MarginOrder>,
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOrder {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub side: BookSide,
    pub price: String,
    pub fill_count: String,
    pub remaining_count: String,
    /// One of `""`, `Decrease`, `Amend`, `MarginCancel`, `SelfTradeCancel`, `ExpiryCancel`,
    /// `Trade`, `PostOnlyCrossCancel` or `ReduceOnlyCancel` (`reduce_only` capped the order
    /// at placement). Kept as a string so new reasons never fail parsing.
    pub last_update_reason: String,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[serde(default)]
    pub self_trade_prevention_type: Option<MarginSelfTradePreventionType>,
    #[serde(default)]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub order_source: Option<String>,
    #[serde(default)]
    pub order_reason: Option<MarginOrderReason>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub reduced_by: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecreaseMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub remaining_count: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmendMarginOrderResponse {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub remaining_count: Option<String>,
    #[serde(default)]
    pub fill_count: Option<String>,
    #[serde(default)]
    pub average_fill_price: Option<String>,
    #[serde(default)]
    pub average_fee_paid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFillsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub fills: Vec<MarginFill>,
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFill {
    pub fill_id: String,
    pub order_id: String,
    pub is_taker: bool,
    pub side: BookSide,
    pub count: String,
    pub created_time: String,
    pub ticker: String,
    pub price: String,
    pub entry_price: String,
    pub fees: String,
    pub realized_pnl: String,
    #[serde(default)]
    pub order_source: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginPositionsResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub positions: Vec<MarginPosition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginPosition {
    pub subaccount: u32,
    pub market_ticker: String,
    pub position: String,
    pub entry_price: String,
    pub unrealized_pnl: String,
    #[serde(default)]
    pub margin_used: Option<String>,
    pub fees: String,
    #[serde(default)]
    pub roe: Option<f64>,
    pub is_portfolio: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginTradesResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub trades: Vec<MarginTrade>,
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginTrade {
    pub trade_id: String,
    pub ticker: String,
    pub count: String,
    pub price: String,
    pub created_time: String,
    pub taker_side: BookSide,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionalRiskLimitResponse {
    pub default_notional_value_risk_limit: String,
    pub notional_value_risk_limits_by_market_ticker: BTreeMap<String, String>,
    pub total_current_usage: String,
    pub current_usage_by_market_ticker: BTreeMap<String, String>,
    #[serde(default)]
    pub member_notional_value_risk_limit: Option<String>,
    #[serde(default)]
    pub effective_account_notional_value_risk_limit: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginBalanceResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub subaccount_balances: Vec<MarginSubaccountBalance>,
    pub settled_funds: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginSubaccountBalance {
    pub subaccount: u32,
    pub position_value: String,
    pub account_equity: String,
    pub maintenance_margin: String,
    pub initial_margin: String,
    pub resting_orders_margin: String,
    pub available_balance: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginRiskResponse {
    pub total_position_notional: String,
    pub total_maintenance_margin: String,
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub positions: Vec<MarginRiskPosition>,
    #[serde(default)]
    pub account_leverage: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginRiskPosition {
    pub subaccount: u32,
    pub market_ticker: String,
    pub position: String,
    pub mark_price: String,
    pub position_notional: String,
    #[serde(default)]
    pub maintenance_margin_required: Option<String>,
    #[serde(default)]
    pub position_leverage: Option<f64>,
    #[serde(default)]
    pub estimated_liquidation_price: Option<String>,
    pub is_portfolio: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginRiskParametersResponse {
    pub liquidation_margin_ratio_threshold: f64,
    pub queue_entry_margin_ratio_threshold: f64,
    pub initial_margin_multiplier: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFundingHistoryResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub funding_history: Vec<MarginFundingHistoryEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFundingHistoryEntry {
    pub market_ticker: String,
    pub funding_time: String,
    pub funding_rate: f64,
    pub mark_price: String,
    pub funding_amount: String,
    pub quantity: String,
    #[serde(default)]
    pub subaccount_number: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginHistoricalFundingRatesResponse {
    #[serde(
        default,
        deserialize_with = "crate::types::deserialize_null_as_empty_vec"
    )]
    pub funding_rates: Vec<MarginFundingRate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginFundingRate {
    pub market_ticker: String,
    pub funding_time: String,
    pub funding_rate: f64,
    pub mark_price: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginFundingRateEstimateResponse {
    pub next_funding_time: String,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub computed_time: Option<String>,
    #[serde(default)]
    pub funding_rate: Option<f64>,
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Premium of the final bar evaluated by this estimate, as a signed decimal fraction
    /// before time weighting. Omitted when that bar has no available premium.
    #[serde(default)]
    pub premium_index: Option<String>,
    /// Timestamp of the last second of that bar. Can precede `computed_time` when input
    /// feeds are delayed.
    #[serde(default)]
    pub premium_index_ts: Option<String>,
}

use crate::rest::account::EmptyResponse;

// ---------------------------------------------------------------------------
// Endpoints
// ---------------------------------------------------------------------------

impl KalshiRestClient {
    /// Check if margin trading is enabled for the authenticated user.
    pub async fn get_margin_enabled(&self) -> Result<MarginEnabledResponse, KalshiError> {
        let path = Self::full_path("/margin/enabled");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_markets(
        &self,
        status: Option<MarginMarketStatus>,
    ) -> Result<GetMarginMarketsResponse, KalshiError> {
        let path = Self::full_path("/margin/markets");
        #[derive(Serialize)]
        struct Q<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<&'a MarginMarketStatus>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q {
                status: status.as_ref(),
            }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market(
        &self,
        ticker: &str,
    ) -> Result<MarginMarketResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market_orderbook(
        &self,
        ticker: &str,
        params: GetMarginMarketOrderbookParams,
    ) -> Result<MarginOrderbookResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}/orderbook"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market_candlesticks(
        &self,
        ticker: &str,
        params: GetMarginMarketCandlesticksParams,
    ) -> Result<GetMarginMarketCandlesticksResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/markets/{ticker}/candlesticks"));
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_orders(
        &self,
        params: GetMarginOrdersParams,
    ) -> Result<GetMarginOrdersResponse, KalshiError> {
        let path = Self::full_path("/margin/orders");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn create_margin_order(
        &self,
        body: &CreateMarginOrderRequest,
    ) -> Result<CreateMarginOrderResponse, KalshiError> {
        let path = Self::full_path("/margin/orders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_margin_order(
        &self,
        order_id: &str,
    ) -> Result<GetMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}"));
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn cancel_margin_order(
        &self,
        order_id: &str,
        subaccount: Option<u32>,
    ) -> Result<CancelMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::DELETE,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn decrease_margin_order(
        &self,
        order_id: &str,
        body: &DecreaseMarginOrderRequest,
        subaccount: Option<u32>,
    ) -> Result<DecreaseMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}/decrease"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::POST,
            &path,
            Some(&Q { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    pub async fn amend_margin_order(
        &self,
        order_id: &str,
        body: &AmendMarginOrderRequest,
        subaccount: Option<u32>,
    ) -> Result<AmendMarginOrderResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/orders/{order_id}/amend"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::POST,
            &path,
            Some(&Q { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    pub async fn get_margin_fills(
        &self,
        params: GetMarginFillsParams,
    ) -> Result<GetMarginFillsResponse, KalshiError> {
        let path = Self::full_path("/margin/fills");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_positions(
        &self,
        params: GetMarginPositionsParams,
    ) -> Result<GetMarginPositionsResponse, KalshiError> {
        let path = Self::full_path("/margin/positions");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_trades(
        &self,
        params: GetMarginTradesParams,
    ) -> Result<GetMarginTradesResponse, KalshiError> {
        let path = Self::full_path("/margin/trades");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_balance(
        &self,
        compute_available_balance: Option<bool>,
    ) -> Result<GetMarginBalanceResponse, KalshiError> {
        let path = Self::full_path("/margin/balance");
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            compute_available_balance: Option<bool>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q {
                compute_available_balance,
            }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_risk(&self) -> Result<GetMarginRiskResponse, KalshiError> {
        let path = Self::full_path("/margin/risk");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_risk_parameters(
        &self,
    ) -> Result<GetMarginRiskParametersResponse, KalshiError> {
        let path = Self::full_path("/margin/risk_parameters");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_notional_risk_limit(
        &self,
    ) -> Result<NotionalRiskLimitResponse, KalshiError> {
        let path = Self::full_path("/margin/notional_risk_limit");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_exchange_status(
        &self,
    ) -> Result<GetMarginExchangeStatusResponse, KalshiError> {
        let path = Self::full_path("/margin/exchange/status");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_perps_account_api_limits(
        &self,
    ) -> Result<crate::rest::account::GetAccountApiLimitsResponse, KalshiError> {
        let path = Self::full_path("/account/limits/perps");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn get_margin_funding_rate_estimate(
        &self,
        ticker: &str,
    ) -> Result<GetMarginFundingRateEstimateResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_rates/estimate");
        #[derive(Serialize)]
        struct Q<'a> {
            ticker: &'a str,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { ticker }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_historical_funding_rates(
        &self,
        params: GetMarginHistoricalFundingRatesParams,
    ) -> Result<GetMarginHistoricalFundingRatesResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_rates/historical");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_funding_history(
        &self,
        params: GetMarginFundingHistoryParams,
    ) -> Result<GetMarginFundingHistoryResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_history");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn get_margin_order_groups(
        &self,
        subaccount: Option<u32>,
    ) -> Result<GetMarginOrderGroupsResponse, KalshiError> {
        let path = Self::full_path("/margin/order_groups");
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn create_margin_order_group(
        &self,
        body: &CreateMarginOrderGroupRequest,
    ) -> Result<CreateMarginOrderGroupResponse, KalshiError> {
        let path = Self::full_path("/margin/order_groups/create");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<GetMarginOrderGroupResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn delete_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::DELETE,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn reset_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/reset"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn trigger_margin_order_group(
        &self,
        order_group_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/trigger"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn update_margin_order_group_limit(
        &self,
        order_group_id: &str,
        body: &UpdateMarginOrderGroupLimitRequest,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/order_groups/{order_group_id}/limit"));
        #[derive(Serialize)]
        struct Q {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        self.send(
            Method::PUT,
            &path,
            Some(&Q { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    pub async fn create_margin_subaccount(
        &self,
    ) -> Result<crate::rest::account::CreateSubaccountResponse, KalshiError> {
        let path = Self::full_path("/portfolio/margin/subaccounts");
        self.send(
            Method::POST,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn apply_margin_subaccount_transfer(
        &self,
        body: &ApplyMarginSubaccountTransferRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/portfolio/margin/subaccounts/transfer");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Cancel all resting margin orders (`DELETE /margin/orders`).
    ///
    /// If `subaccount` is `None`, orders from any subaccount are eligible. Orders placed
    /// during the minute after the request may also be cancelled. Costs 10 rate-limit tokens.
    pub async fn cancel_all_margin_orders(
        &self,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/orders");
        self.send(
            Method::DELETE,
            &path,
            Some(&SubaccountQuery { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Get the orderbooks of several margin markets in one request
    /// (`GET /margin/markets/orderbooks`).
    ///
    /// Accepts 1-100 tickers; the response has one entry per ticker, in request order.
    /// A ticker that is not a margin market makes the whole request fail with 404.
    pub async fn get_margin_market_orderbooks<S: AsRef<str>>(
        &self,
        tickers: &[S],
    ) -> Result<GetMarginMarketOrderbooksResponse, KalshiError> {
        let path = Self::full_path("/margin/markets/orderbooks");
        let csv = join_csv(tickers);
        #[derive(Serialize)]
        struct Q {
            tickers: String,
        }
        self.send(
            Method::GET,
            &path,
            Some(&Q { tickers: csv }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// List margin corporate action notices (`GET /margin/corporate_actions`).
    pub async fn get_margin_corporate_actions(
        &self,
        params: GetMarginCorporateActionsParams,
    ) -> Result<GetMarginCorporateActionsResponse, KalshiError> {
        let path = Self::full_path("/margin/corporate_actions");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Informational per-second premium index (`GET /margin/funding_rates/premium_index`).
    pub async fn get_margin_premium_index(
        &self,
        params: GetMarginPremiumIndexParams,
    ) -> Result<GetMarginPremiumIndexResponse, KalshiError> {
        let path = Self::full_path("/margin/funding_rates/premium_index");
        self.send(
            Method::GET,
            &path,
            Some(&params),
            Option::<&()>::None,
            false,
        )
        .await
    }

    /// Fee schedule for the authenticated account (`GET /margin/fee_tier_rates`).
    ///
    /// **Requires auth.**
    pub async fn get_margin_fee_tier_rates(
        &self,
    ) -> Result<GetMarginFeeTierRatesResponse, KalshiError> {
        let path = Self::full_path("/margin/fee_tier_rates");
        self.send(
            Method::GET,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }

    // -- exit triggers -------------------------------------------------------

    /// Set a bracket or trailing stop on an isolated margin position.
    pub async fn set_isolated_margin_exit_trigger(
        &self,
        ticker: &str,
        body: &SetIsolatedExitTriggerRequest,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = Self::full_path(&format!("/margin/isolated/positions/{ticker}/exit_trigger"));
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// List live exit triggers on an isolated margin position.
    pub async fn get_isolated_margin_exit_triggers(
        &self,
        ticker: &str,
        kind: Option<ExitTriggerKind>,
    ) -> Result<GetExitTriggersResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/isolated/positions/{ticker}/exit_trigger"));
        self.send(
            Method::GET,
            &path,
            Some(&ExitTriggerKindQuery { kind }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Cancel exit triggers on an isolated margin position (succeeds if none are live).
    pub async fn delete_isolated_margin_exit_trigger(
        &self,
        ticker: &str,
        kind: Option<ExitTriggerKind>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/isolated/positions/{ticker}/exit_trigger"));
        self.send(
            Method::DELETE,
            &path,
            Some(&ExitTriggerKindQuery { kind }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Set a bracket or trailing stop on a non-isolated margin position.
    pub async fn set_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        body: &SetCrossExitTriggerRequest,
        subaccount: Option<u32>,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = Self::full_path(&format!("/margin/cross/positions/{ticker}/exit_trigger"));
        self.send(
            Method::PUT,
            &path,
            Some(&SubaccountQuery { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    /// List live exit triggers on a non-isolated margin position.
    pub async fn get_cross_margin_exit_triggers(
        &self,
        ticker: &str,
        subaccount: Option<u32>,
        kind: Option<ExitTriggerKind>,
    ) -> Result<GetExitTriggersResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/cross/positions/{ticker}/exit_trigger"));
        self.send(
            Method::GET,
            &path,
            Some(&SubaccountKindQuery { subaccount, kind }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Cancel exit triggers on a non-isolated margin position (succeeds if none are live).
    pub async fn delete_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        subaccount: Option<u32>,
        kind: Option<ExitTriggerKind>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!("/margin/cross/positions/{ticker}/exit_trigger"));
        self.send(
            Method::DELETE,
            &path,
            Some(&SubaccountKindQuery { subaccount, kind }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Update one cross-margin bracket's leg prices.
    pub async fn update_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        trigger_id: &str,
        body: &UpdateExitTriggerRequest,
        subaccount: Option<u32>,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = Self::full_path(&format!(
            "/margin/cross/positions/{ticker}/exit_trigger/{trigger_id}"
        ));
        self.send(
            Method::PUT,
            &path,
            Some(&SubaccountQuery { subaccount }),
            Some(body),
            true,
        )
        .await
    }

    /// Cancel one cross-margin bracket by id.
    pub async fn delete_cross_margin_exit_trigger_by_id(
        &self,
        ticker: &str,
        trigger_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path(&format!(
            "/margin/cross/positions/{ticker}/exit_trigger/{trigger_id}"
        ));
        self.send(
            Method::DELETE,
            &path,
            Some(&SubaccountQuery { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    // -- FCM member endpoints ------------------------------------------------

    /// Create an FCM subtrader (`POST /margin/fcm/subtraders`).
    pub async fn create_margin_fcm_subtrader(
        &self,
        body: &CreateMarginFcmSubtraderRequest,
    ) -> Result<CreateMarginFcmSubtraderResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Get an FCM subtrader's initial-margin caps and notional limits.
    pub async fn get_fcm_subtrader_risk_controls(
        &self,
        params: GetFcmSubtraderRiskControlsParams,
    ) -> Result<GetFcmSubtraderRiskControlsResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders/risk_controls");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    /// Set an FCM subtrader's initial-margin cap.
    pub async fn update_fcm_subtrader_risk_controls(
        &self,
        body: &UpdateFcmSubtraderRiskControlsRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders/risk_controls");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Remove an FCM subtrader's initial-margin cap.
    pub async fn delete_fcm_subtrader_risk_controls(
        &self,
        params: DeleteFcmSubtraderRiskControlsParams,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders/risk_controls");
        self.send(
            Method::DELETE,
            &path,
            Some(&params),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Set the FCM member's own account-level notional risk limit.
    pub async fn update_fcm_notional_risk_limit(
        &self,
        body: &UpdateFcmNotionalRiskLimitRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/notional_risk_limit");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// Clear the FCM member's own account-level notional risk limit.
    pub async fn delete_fcm_notional_risk_limit(&self) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/notional_risk_limit");
        self.send(
            Method::DELETE,
            &path,
            Option::<&()>::None,
            Option::<&()>::None,
            true,
        )
        .await
    }
}

/// Join items with commas using a single exactly-sized allocation.
fn join_csv<S: AsRef<str>>(items: &[S]) -> String {
    let mut csv = String::with_capacity(items.iter().map(|t| t.as_ref().len() + 1).sum());
    for (i, t) in items.iter().enumerate() {
        if i > 0 {
            csv.push(',');
        }
        csv.push_str(t.as_ref());
    }
    csv
}

#[derive(Serialize)]
struct SubaccountQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    subaccount: Option<u32>,
}

#[derive(Serialize)]
struct ExitTriggerKindQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ExitTriggerKind>,
}

#[derive(Serialize)]
struct SubaccountKindQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ExitTriggerKind>,
}

#[cfg(test)]
mod tests {
    use super::{GetMarginOrderGroupResponse, MarginPosition, join_csv};

    #[test]
    fn join_csv_joins_without_trailing_comma() {
        assert_eq!(join_csv::<&str>(&[]), "");
        assert_eq!(join_csv(&["A"]), "A");
        assert_eq!(join_csv(&["A", "B", "C"]), "A,B,C");
        assert_eq!(join_csv(&["A".to_string(), "B".to_string()]), "A,B");
    }

    #[test]
    fn margin_position_deserializes_nullable_margin_used_and_portfolio_flag() {
        let position: MarginPosition = serde_json::from_value(serde_json::json!({
            "subaccount": 2,
            "market_ticker": "KXBTC-26JUN",
            "position": "1.2500",
            "entry_price": "100.00",
            "unrealized_pnl": "2.50",
            "margin_used": null,
            "fees": "0.10",
            "roe": null,
            "is_portfolio": true
        }))
        .unwrap();

        assert_eq!(position.margin_used, None);
        assert_eq!(position.roe, None);
        assert!(position.is_portfolio);
    }

    #[test]
    fn margin_order_group_deserializes_order_ids() {
        let response: GetMarginOrderGroupResponse = serde_json::from_value(serde_json::json!({
            "is_auto_cancel_enabled": true,
            "contracts_limit_fp": "12.5000",
            "orders": ["a", "b"],
            "exchange_index": 3
        }))
        .unwrap();

        assert_eq!(response.orders, vec!["a".to_string(), "b".to_string()]);
        assert!(response.is_auto_cancel_enabled);
        assert_eq!(response.exchange_index, Some(3));
    }
}
