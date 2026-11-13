use super::*;

/// Filters for scheduled and recently updated corporate action notices.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarginCorporateActionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginCorporateActionType {
    CashDividend,
    SpecialDividend,
    SpinOff,
    StockSplit,
    FinalSettlement,
    TickerChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginCorporateActionStatus {
    Scheduled,
    Cancelled,
    Applied,
    Lapsed,
}

/// Cancelled notices omit their terms and execution time. Other statuses have
/// exactly one terms object, selected by `action_type`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginCorporateAction {
    pub id: String,
    pub market_id: String,
    pub market_ticker: String,
    #[serde(rename = "type")]
    pub action_type: MarginCorporateActionType,
    pub status: MarginCorporateActionStatus,
    pub updated_ts: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<MarginCorporateActionCashTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<MarginCorporateActionSplitTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_settlement: Option<MarginCorporateActionFinalSettlementTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker_change: Option<MarginCorporateActionTickerChangeTerms>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginCorporateActionCashTerms {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_per_share: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_per_contract: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ex_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginCorporateActionSplitTerms {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio_old: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio_new: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ex_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginCorporateActionFinalSettlementTerms {
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_price_per_share: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_price_per_contract: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginCorporateActionTickerChangeTerms {
    pub old_ticker: String,
    pub new_ticker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginCorporateActionsResponse {
    pub corporate_actions: Vec<MarginCorporateAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginHolidayWindow {
    pub from: String,
    pub until: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginMarketScheduleDetails {
    pub kalshi_trading_holidays: Vec<MarginHolidayWindow>,
}

/// The response itself is a map keyed by market ticker, not an envelope.
pub type GetMarginMarketSchedulesResponse = BTreeMap<String, MarginMarketScheduleDetails>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginMarketOrderbook {
    pub ticker: String,
    pub orderbook: MarginOrderbookCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginMarketOrderbooksResponse {
    pub orderbooks: Vec<MarginMarketOrderbook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginFeeSchedule {
    SelfClearingMembers,
    KalshiPrime,
    Fcm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginFeeTierRate {
    pub fee_schedule: MarginFeeSchedule,
    pub tier: i32,
    pub maker_fee_rate: f64,
    pub taker_fee_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginFeeTierRatesResponse {
    pub fee_tier_rates: Vec<MarginFeeTierRate>,
}

/// The time window is inclusive at `start_ts`, exclusive at `end_ts`, and at most one hour.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarginPremiumIndexParams {
    pub ticker: String,
    pub start_ts: i64,
    pub end_ts: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginPremiumIndexPoint {
    pub second_ts: String,
    pub premium_index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarginPremiumIndexResponse {
    pub points: Vec<MarginPremiumIndexPoint>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerKind {
    Bracket,
    Trailing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerStatus {
    PendingOnEntry,
    Active,
    Filled,
    Failed,
    Canceled,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerReason {
    UserCanceled,
    PositionClosed,
    EntryNotFilled,
    OrderRejected,
    PositionFlipped,
    MarketDeactivated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitTriggerLeg {
    StopLoss,
    TakeProfit,
}

/// A bracket needs at least one leg; a trailing trigger needs exactly one trail
/// distance. The two families' fields cannot be mixed. Omitted legs are cleared.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetIsolatedExitTriggerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExitTriggerKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<i32>,
}

/// `count` and `anchor_order_id` are mutually exclusive. Either makes this an
/// append and requires a `client_trigger_id`; other writes reject that ID.
/// Anchoring is bracket-only. Family and leg rules match isolated writes.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCrossExitTriggerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
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
    pub trail_bps: Option<i32>,
}

/// At least one bracket leg is required; an omitted leg is cleared.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateExitTriggerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitTrigger {
    pub id: String,
    pub ticker: String,
    pub kind: ExitTriggerKind,
    pub status: ExitTriggerStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<ExitTriggerReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<ExitTriggerLeg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watermark_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_stop_price: Option<String>,
    pub count: String,
    pub filled_count: String,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExitTriggersResponse {
    pub exit_triggers: Vec<ExitTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FCMAssetClass {
    Crypto,
    Equities,
    Metals,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginFCMSubtraderRequest {
    pub subtrader_suffix: String,
    pub require_category_cap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMarginFCMSubtraderResponse {
    pub subtrader_id: String,
}

/// Side of the perps position moved by a margin FCM position transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginPositionTransferSide {
    Long,
    Short,
}

/// Body of `POST /margin/fcm/positions/transfers`.
///
/// Supply `ticker` or `market_id` (ticker wins when both are set).
/// `client_transfer_id` (a UUID) makes the request idempotent per source
/// subtrader, but Kalshi does not keep transfer IDs across an exchange
/// restart; check the source position before retrying after one.
#[derive(Debug, Clone, Serialize)]
pub struct CreateMarginFCMPositionTransferRequest {
    pub source_subtrader_id: String,
    pub target_subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    pub side: MarginPositionTransferSide,
    /// Whole number of contracts as a fixed-point count string.
    pub count: String,
    /// Transfer price in dollars per contract (2-4 decimals, > 0).
    pub price: String,
    pub client_transfer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMarginFCMPositionTransferResponse {
    pub position_transfer_id: String,
}

/// The filters restrict risk controls, but not the returned notional limits.
/// `subtrader_id` may be omitted only for a subtrader-bound API key. Market
/// and asset class filters are mutually exclusive.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFCMSubtraderRiskControlsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<FCMAssetClass>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteFCMSubtraderRiskControlsParams {
    pub subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<FCMAssetClass>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFCMSubtraderRiskControlsRequest {
    pub subtrader_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<FCMAssetClass>,
    pub im_cap: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCMSubtraderRiskControls {
    pub subtrader_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<FCMAssetClass>,
    pub im_cap: String,
    pub current_im: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCMSubtraderNotionalRiskLimit {
    pub subtrader_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    pub notional_value_risk_limit: String,
    pub current_notional: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFCMSubtraderRiskControlsResponse {
    pub risk_controls: Vec<FCMSubtraderRiskControls>,
    pub notional_limits: Vec<FCMSubtraderNotionalRiskLimit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFCMNotionalRiskLimitRequest {
    pub notional_value_risk_limit: String,
}

#[derive(Serialize)]
struct ExitTriggerQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    subaccount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ExitTriggerKind>,
}

// Encode each dynamic path segment independently, before signing the path.
fn path_segment(value: &str) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            result.push(char::from(byte));
        } else {
            write!(&mut result, "%{byte:02X}").expect("writing to a String cannot fail");
        }
    }
    result
}

fn exit_trigger_path(margin_type: &str, ticker: &str) -> String {
    KalshiRestClient::full_path(&format!(
        "/margin/{margin_type}/positions/{}/exit_trigger",
        path_segment(ticker)
    ))
}

impl KalshiRestClient {
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

    /// Cancels resting orders, including new orders during the following minute.
    /// Omitting `subaccount` selects all subaccounts.
    pub async fn cancel_all_margin_orders(
        &self,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        #[derive(Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            subaccount: Option<u32>,
        }
        let path = Self::full_path("/margin/orders");
        self.send(
            Method::DELETE,
            &path,
            Some(&Query { subaccount }),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// `tickers` is encoded as the OpenAPI form-style, non-exploded list.
    pub async fn get_margin_market_schedules(
        &self,
        tickers: &[String],
    ) -> Result<GetMarginMarketSchedulesResponse, KalshiError> {
        #[derive(Serialize)]
        struct Query {
            tickers: String,
        }
        let path = Self::full_path("/margin/markets/schedules");
        self.send(
            Method::GET,
            &path,
            Some(&Query {
                tickers: tickers.join(","),
            }),
            Option::<&()>::None,
            false,
        )
        .await
    }

    pub async fn get_margin_market_orderbooks(
        &self,
        tickers: &[String],
    ) -> Result<GetMarginMarketOrderbooksResponse, KalshiError> {
        #[derive(Serialize)]
        struct Query {
            tickers: String,
        }
        let path = Self::full_path("/margin/markets/orderbooks");
        self.send(
            Method::GET,
            &path,
            Some(&Query {
                tickers: tickers.join(","),
            }),
            Option::<&()>::None,
            false,
        )
        .await
    }

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

    pub async fn set_isolated_margin_exit_trigger(
        &self,
        ticker: &str,
        body: &SetIsolatedExitTriggerRequest,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = exit_trigger_path("isolated", ticker);
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_isolated_margin_exit_triggers(
        &self,
        ticker: &str,
        kind: Option<ExitTriggerKind>,
    ) -> Result<GetExitTriggersResponse, KalshiError> {
        let path = exit_trigger_path("isolated", ticker);
        let query = ExitTriggerQuery {
            subaccount: None,
            kind,
        };
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, true)
            .await
    }

    pub async fn delete_isolated_margin_exit_trigger(
        &self,
        ticker: &str,
        kind: Option<ExitTriggerKind>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = exit_trigger_path("isolated", ticker);
        let query = ExitTriggerQuery {
            subaccount: None,
            kind,
        };
        self.send(
            Method::DELETE,
            &path,
            Some(&query),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// With no subaccount, the server selects the primary (or key-bound) subaccount.
    pub async fn set_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        subaccount: Option<u32>,
        body: &SetCrossExitTriggerRequest,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = exit_trigger_path("cross", ticker);
        let query = ExitTriggerQuery {
            subaccount,
            kind: None,
        };
        self.send(Method::PUT, &path, Some(&query), Some(body), true)
            .await
    }

    pub async fn get_cross_margin_exit_triggers(
        &self,
        ticker: &str,
        subaccount: Option<u32>,
        kind: Option<ExitTriggerKind>,
    ) -> Result<GetExitTriggersResponse, KalshiError> {
        let path = exit_trigger_path("cross", ticker);
        let query = ExitTriggerQuery { subaccount, kind };
        self.send(Method::GET, &path, Some(&query), Option::<&()>::None, true)
            .await
    }

    pub async fn delete_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        subaccount: Option<u32>,
        kind: Option<ExitTriggerKind>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = exit_trigger_path("cross", ticker);
        let query = ExitTriggerQuery { subaccount, kind };
        self.send(
            Method::DELETE,
            &path,
            Some(&query),
            Option::<&()>::None,
            true,
        )
        .await
    }

    /// Updates one bracket; trailing stops use the collection route.
    pub async fn update_cross_margin_exit_trigger(
        &self,
        ticker: &str,
        trigger_id: &str,
        subaccount: Option<u32>,
        body: &UpdateExitTriggerRequest,
    ) -> Result<ExitTrigger, KalshiError> {
        let path = format!(
            "{}/{}",
            exit_trigger_path("cross", ticker),
            path_segment(trigger_id)
        );
        let query = ExitTriggerQuery {
            subaccount,
            kind: None,
        };
        self.send(Method::PUT, &path, Some(&query), Some(body), true)
            .await
    }

    pub async fn delete_cross_margin_exit_trigger_by_id(
        &self,
        ticker: &str,
        trigger_id: &str,
        subaccount: Option<u32>,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = format!(
            "{}/{}",
            exit_trigger_path("cross", ticker),
            path_segment(trigger_id)
        );
        let query = ExitTriggerQuery {
            subaccount,
            kind: None,
        };
        self.send(
            Method::DELETE,
            &path,
            Some(&query),
            Option::<&()>::None,
            true,
        )
        .await
    }

    pub async fn create_margin_fcm_subtrader(
        &self,
        body: &CreateMarginFCMSubtraderRequest,
    ) -> Result<CreateMarginFCMSubtraderResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    /// `POST /margin/fcm/positions/transfers`: move a perps position from an
    /// FCM subtrader into a subtrader under the FCM's error account. Must be
    /// enabled for the FCM by Kalshi.
    pub async fn create_margin_fcm_position_transfer(
        &self,
        body: &CreateMarginFCMPositionTransferRequest,
    ) -> Result<CreateMarginFCMPositionTransferResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/positions/transfers");
        self.send(Method::POST, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn get_fcm_subtrader_risk_controls(
        &self,
        params: GetFCMSubtraderRiskControlsParams,
    ) -> Result<GetFCMSubtraderRiskControlsResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders/risk_controls");
        self.send(Method::GET, &path, Some(&params), Option::<&()>::None, true)
            .await
    }

    pub async fn update_fcm_subtrader_risk_controls(
        &self,
        body: &UpdateFCMSubtraderRiskControlsRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/subtraders/risk_controls");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

    pub async fn delete_fcm_subtrader_risk_controls(
        &self,
        params: DeleteFCMSubtraderRiskControlsParams,
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

    /// Sets the member limit, which cannot raise the effective administrative limit.
    pub async fn update_fcm_notional_risk_limit(
        &self,
        body: &UpdateFCMNotionalRiskLimitRequest,
    ) -> Result<EmptyResponse, KalshiError> {
        let path = Self::full_path("/margin/fcm/notional_risk_limit");
        self.send(Method::PUT, &path, Option::<&()>::None, Some(body), true)
            .await
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn margin_fcm_position_transfer_matches_published_example() {
        let body = CreateMarginFCMPositionTransferRequest {
            source_subtrader_id: "src_CUST123".into(),
            target_subtrader_id: "err_ERR1".into(),
            ticker: None,
            market_id: Some("0a1b2c3d".into()),
            side: MarginPositionTransferSide::Short,
            count: "5".into(),
            price: "4500.00".into(),
            client_transfer_id: "7e2a1d6b-3c4f-4a8b-9d0e-1f2a3b4c5d6e".into(),
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "source_subtrader_id": "src_CUST123",
                "target_subtrader_id": "err_ERR1",
                "market_id": "0a1b2c3d",
                "side": "short",
                "count": "5",
                "price": "4500.00",
                "client_transfer_id": "7e2a1d6b-3c4f-4a8b-9d0e-1f2a3b4c5d6e"
            })
        );
        let resp: CreateMarginFCMPositionTransferResponse =
            serde_json::from_value(json!({"position_transfer_id": "pt_1"})).unwrap();
        assert_eq!(resp.position_transfer_id, "pt_1");
    }

    #[test]
    fn exit_trigger_writes_preserve_family_and_append_fields() {
        let bracket = SetIsolatedExitTriggerRequest {
            stop_loss_price: Some("100.0000".into()),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(bracket).unwrap(),
            json!({"stop_loss_price":"100.0000"})
        );
        let trailing = SetCrossExitTriggerRequest {
            kind: Some(ExitTriggerKind::Trailing),
            trail_bps: Some(100),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(trailing).unwrap(),
            json!({"kind":"trailing", "trail_bps":100})
        );
        let anchored = SetCrossExitTriggerRequest {
            anchor_order_id: Some("order".into()),
            client_trigger_id: Some("key".into()),
            take_profit_price: Some("120.00".into()),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(anchored).unwrap(),
            json!({"anchor_order_id":"order", "client_trigger_id":"key", "take_profit_price":"120.00"})
        );
        let update = UpdateExitTriggerRequest {
            stop_loss_price: Some("90.00".into()),
            take_profit_price: None,
        };
        assert_eq!(
            serde_json::to_value(update).unwrap(),
            json!({"stop_loss_price":"90.00"})
        );
        let query = ExitTriggerQuery {
            subaccount: Some(0),
            kind: Some(ExitTriggerKind::Trailing),
        };
        assert_eq!(
            serde_json::to_value(query).unwrap(),
            json!({"subaccount":0,"kind":"trailing"})
        );
        assert_eq!(path_segment("a/b ?%"), "a%2Fb%20%3F%25");
    }

    #[test]
    fn exit_trigger_responses_preserve_lifecycle_and_required_fields() {
        assert!(serde_json::from_value::<GetExitTriggersResponse>(json!({})).is_err());
        let empty: GetExitTriggersResponse =
            serde_json::from_value(json!({"exit_triggers": []})).unwrap();
        assert!(empty.exit_triggers.is_empty());

        // An anchored trigger can return to pending after a prior close.
        let pending = json!({
            "id": "trigger", "ticker": "T", "kind": "bracket",
            "status": "pending_on_entry", "anchor_order_id": "entry",
            "client_trigger_id": "key", "triggered_order_id": "previous-close",
            "triggered_leg": "stop_loss", "stop_loss_price": "90.0000",
            "count": "2.00", "filled_count": "1.00",
            "created_time": "2026-01-01T00:00:00Z",
            "updated_time": "2026-01-01T00:01:00Z"
        });
        let response: GetExitTriggersResponse =
            serde_json::from_value(json!({"exit_triggers": [pending.clone()]})).unwrap();
        let trigger = &response.exit_triggers[0];
        assert!(matches!(trigger.status, ExitTriggerStatus::PendingOnEntry));
        assert!(matches!(
            trigger.triggered_leg,
            Some(ExitTriggerLeg::StopLoss)
        ));
        assert_eq!(
            trigger.triggered_order_id.as_deref(),
            Some("previous-close")
        );
        assert_eq!(trigger.filled_count, "1.00");
        assert!(trigger.trail_amount.is_none());
        assert!(trigger.status_reason.is_none());
        assert_eq!(serde_json::to_value(trigger).unwrap(), pending);

        let terminal = json!({
            "id": "trigger", "ticker": "T", "kind": "trailing",
            "status": "canceled", "status_reason": "user_canceled",
            "trail_bps": 100, "watermark_price": "105.0000",
            "effective_stop_price": "103.9500",
            "count": "0.00", "filled_count": "0.00",
            "created_time": "2026-01-01T00:00:00Z",
            "updated_time": "2026-01-01T00:01:00Z"
        });
        let trigger: ExitTrigger = serde_json::from_value(terminal.clone()).unwrap();
        assert!(matches!(trigger.status, ExitTriggerStatus::Canceled));
        assert!(matches!(
            trigger.status_reason,
            Some(ExitTriggerReason::UserCanceled)
        ));
        assert!(trigger.anchor_order_id.is_none());
        assert_eq!(serde_json::to_value(&trigger).unwrap(), terminal);

        let mut missing_count = terminal;
        missing_count.as_object_mut().unwrap().remove("count");
        assert!(serde_json::from_value::<ExitTrigger>(missing_count).is_err());
    }

    #[test]
    fn corporate_action_terms_and_nullable_fields_round_trip() {
        let notices = json!({"corporate_actions":[
            {"id":"1","market_id":"m","market_ticker":"T","type":"cash_dividend","status":"scheduled","updated_ts":"2026-01-01T00:00:00Z","execution_time":null,"cash":{"cash_per_contract":"1.2500","ex_date":"2026-01-03"}},
            {"id":"2","market_id":"m","market_ticker":"T","type":"stock_split","status":"applied","updated_ts":"2026-01-01T00:00:00Z","split":{"ratio_old":"1","ratio_new":"2"}},
            {"id":"3","market_id":"m","market_ticker":"T","type":"final_settlement","status":"lapsed","updated_ts":"2026-01-01T00:00:00Z","final_settlement":{"reason":"merger","settlement_price_per_share":"12.0000"}},
            {"id":"4","market_id":"m","market_ticker":"T","type":"ticker_change","status":"scheduled","updated_ts":"2026-01-01T00:00:00Z","ticker_change":{"old_ticker":"OLD","new_ticker":"NEW"}},
            {"id":"5","market_id":"m","market_ticker":"T","type":"spin_off","status":"cancelled","updated_ts":"2026-01-01T00:00:00Z","cash":null,"split":null,"final_settlement":null,"ticker_change":null}
        ]});
        let response: GetMarginCorporateActionsResponse = serde_json::from_value(notices).unwrap();
        assert!(response.corporate_actions[0].execution_time.is_none());
        assert_eq!(
            response.corporate_actions[0]
                .cash
                .as_ref()
                .unwrap()
                .cash_per_contract
                .as_deref(),
            Some("1.2500")
        );
        assert_eq!(
            response.corporate_actions[1]
                .split
                .as_ref()
                .unwrap()
                .ratio_new
                .as_deref(),
            Some("2")
        );
        assert_eq!(
            response.corporate_actions[2]
                .final_settlement
                .as_ref()
                .unwrap()
                .reason,
            "merger"
        );
        assert_eq!(
            response.corporate_actions[3]
                .ticker_change
                .as_ref()
                .unwrap()
                .new_ticker,
            "NEW"
        );
        let cancelled: Value = serde_json::to_value(&response.corporate_actions[4]).unwrap();
        for field in [
            "cash",
            "split",
            "final_settlement",
            "ticker_change",
            "execution_time",
        ] {
            assert!(cancelled.get(field).is_none());
        }
    }
}
