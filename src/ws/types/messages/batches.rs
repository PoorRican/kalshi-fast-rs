use crate::types::YesNo;
use serde::Deserialize;
use std::borrow::Cow;

/// Payload for RfqBatchLeg.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: YesNo,
    pub yes_settlement_value_dollars: Option<String>,
}

/// Borrowed version of [`WsRfqBatchLeg`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchLegRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    pub side: YesNo,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub yes_settlement_value_dollars: Option<Cow<'a, str>>,
}

impl<'a> WsRfqBatchLegRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchLeg {
        WsRfqBatchLeg {
            event_ticker: self.event_ticker.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            side: self.side,
            yes_settlement_value_dollars: self.yes_settlement_value_dollars.map(Cow::into_owned),
        }
    }
}

/// Payload for RfqBatchChild.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchChild {
    pub id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub contracts_fp: Option<String>,
    pub target_cost_dollars: Option<String>,
    pub mve_collection_ticker: Option<String>,
    pub leg_indices: Vec<i64>,
}

/// Borrowed version of [`WsRfqBatchChild`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchChildRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub contracts_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub target_cost_dollars: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub mve_collection_ticker: Option<Cow<'a, str>>,
    pub leg_indices: Vec<i64>,
}

impl<'a> WsRfqBatchChildRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchChild {
        WsRfqBatchChild {
            id: self.id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            target_cost_dollars: self.target_cost_dollars.map(Cow::into_owned),
            mve_collection_ticker: self.mve_collection_ticker.map(Cow::into_owned),
            leg_indices: self.leg_indices,
        }
    }
}

/// Payload for RfqBatchCreated.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchCreated {
    pub id: String,
    pub creator_id: String,
    pub created_ts: String,
    pub legs: Vec<WsRfqBatchLeg>,
    pub rfqs: Vec<WsRfqBatchChild>,
}

/// Borrowed version of [`WsRfqBatchCreated`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchCreatedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
    #[serde(borrow)]
    pub legs: Vec<WsRfqBatchLegRef<'a>>,
    #[serde(borrow)]
    pub rfqs: Vec<WsRfqBatchChildRef<'a>>,
}

impl<'a> WsRfqBatchCreatedRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchCreated {
        WsRfqBatchCreated {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            created_ts: self.created_ts.into_owned(),
            legs: self
                .legs
                .into_iter()
                .map(WsRfqBatchLegRef::into_owned)
                .collect(),
            rfqs: self
                .rfqs
                .into_iter()
                .map(WsRfqBatchChildRef::into_owned)
                .collect(),
        }
    }
}

/// Payload for RfqBatchDeleted.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchDeleted {
    pub id: String,
    pub creator_id: String,
    pub rfq_ids: Vec<String>,
    pub deleted_ts: String,
}

/// Borrowed version of [`WsRfqBatchDeleted`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchDeletedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub rfq_ids: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub deleted_ts: Cow<'a, str>,
}

impl<'a> WsRfqBatchDeletedRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchDeleted {
        WsRfqBatchDeleted {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            rfq_ids: self.rfq_ids.into_iter().map(Cow::into_owned).collect(),
            deleted_ts: self.deleted_ts.into_owned(),
        }
    }
}

/// Payload for BatchQuoteCreatedChild.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteCreatedChild {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub yes_contracts_offered_fp: Option<String>,
    pub no_contracts_offered_fp: Option<String>,
    pub rfq_target_cost_dollars: Option<String>,
}

/// Borrowed version of [`WsBatchQuoteCreatedChild`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteCreatedChildRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub no_bid_dollars: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub yes_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub no_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub rfq_target_cost_dollars: Option<Cow<'a, str>>,
}

impl<'a> WsBatchQuoteCreatedChildRef<'a> {
    pub fn into_owned(self) -> WsBatchQuoteCreatedChild {
        WsBatchQuoteCreatedChild {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
        }
    }
}

/// Payload for BatchQuoteCreated.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteCreated {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub created_ts: String,
    pub quotes: Vec<WsBatchQuoteCreatedChild>,
    pub subaccount: Option<i64>,
}

/// Borrowed version of [`WsBatchQuoteCreated`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteCreatedRef<'a> {
    #[serde(borrow)]
    pub batch_quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_batch_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
    #[serde(borrow)]
    pub quotes: Vec<WsBatchQuoteCreatedChildRef<'a>>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

impl<'a> WsBatchQuoteCreatedRef<'a> {
    pub fn into_owned(self) -> WsBatchQuoteCreated {
        WsBatchQuoteCreated {
            batch_quote_id: self.batch_quote_id.into_owned(),
            rfq_batch_id: self.rfq_batch_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            created_ts: self.created_ts.into_owned(),
            quotes: self
                .quotes
                .into_iter()
                .map(WsBatchQuoteCreatedChildRef::into_owned)
                .collect(),
            subaccount: self.subaccount,
        }
    }
}

/// Payload for RfqBatchBestQuote.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuote {
    pub rfq_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: Option<String>,
    pub no_bid_dollars: Option<String>,
    pub contracts_fp: Option<String>,
    pub yes_contracts_offered_fp: Option<String>,
    pub no_contracts_offered_fp: Option<String>,
    pub rfq_target_cost_dollars: Option<String>,
}

/// Borrowed version of [`WsRfqBatchBestQuote`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuoteRef<'a> {
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub yes_bid_dollars: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub no_bid_dollars: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub contracts_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub yes_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub no_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub rfq_target_cost_dollars: Option<Cow<'a, str>>,
}

impl<'a> WsRfqBatchBestQuoteRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchBestQuote {
        WsRfqBatchBestQuote {
            rfq_id: self.rfq_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.map(Cow::into_owned),
            no_bid_dollars: self.no_bid_dollars.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
        }
    }
}

/// Payload for RfqBatchBestQuotes.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuotes {
    pub rfq_batch_id: String,
    pub client_batch_id: Option<String>,
    pub version: i64,
    pub updated_ts: String,
    pub yes_complete: bool,
    pub no_complete: bool,
    pub rfqs: Vec<WsRfqBatchBestQuote>,
    pub subaccount: Option<i64>,
}

/// Borrowed version of [`WsRfqBatchBestQuotes`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuotesRef<'a> {
    #[serde(borrow)]
    pub rfq_batch_id: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub client_batch_id: Option<Cow<'a, str>>,
    pub version: i64,
    #[serde(borrow)]
    pub updated_ts: Cow<'a, str>,
    pub yes_complete: bool,
    pub no_complete: bool,
    #[serde(borrow)]
    pub rfqs: Vec<WsRfqBatchBestQuoteRef<'a>>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

impl<'a> WsRfqBatchBestQuotesRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchBestQuotes {
        WsRfqBatchBestQuotes {
            rfq_batch_id: self.rfq_batch_id.into_owned(),
            client_batch_id: self.client_batch_id.map(Cow::into_owned),
            version: self.version,
            updated_ts: self.updated_ts.into_owned(),
            yes_complete: self.yes_complete,
            no_complete: self.no_complete,
            rfqs: self
                .rfqs
                .into_iter()
                .map(WsRfqBatchBestQuoteRef::into_owned)
                .collect(),
            subaccount: self.subaccount,
        }
    }
}

/// Payload for BatchQuoteAcceptedChild.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAcceptedChild {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub contracts_accepted_fp: Option<String>,
    pub yes_contracts_offered_fp: Option<String>,
    pub no_contracts_offered_fp: Option<String>,
    pub rfq_target_cost_dollars: Option<String>,
}

/// Borrowed version of [`WsBatchQuoteAcceptedChild`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAcceptedChildRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub no_bid_dollars: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub contracts_accepted_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub yes_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub no_contracts_offered_fp: Option<Cow<'a, str>>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub rfq_target_cost_dollars: Option<Cow<'a, str>>,
}

impl<'a> WsBatchQuoteAcceptedChildRef<'a> {
    pub fn into_owned(self) -> WsBatchQuoteAcceptedChild {
        WsBatchQuoteAcceptedChild {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            contracts_accepted_fp: self.contracts_accepted_fp.map(Cow::into_owned),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
        }
    }
}

/// Payload for BatchQuoteAccepted.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAccepted {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub accepted_side: YesNo,
    pub accepted_ts: String,
    pub quotes: Vec<WsBatchQuoteAcceptedChild>,
    pub subaccount: Option<i64>,
}

/// Borrowed version of [`WsBatchQuoteAccepted`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAcceptedRef<'a> {
    #[serde(borrow)]
    pub batch_quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_batch_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_creator_id: Cow<'a, str>,
    pub accepted_side: YesNo,
    #[serde(borrow)]
    pub accepted_ts: Cow<'a, str>,
    #[serde(borrow)]
    pub quotes: Vec<WsBatchQuoteAcceptedChildRef<'a>>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

impl<'a> WsBatchQuoteAcceptedRef<'a> {
    pub fn into_owned(self) -> WsBatchQuoteAccepted {
        WsBatchQuoteAccepted {
            batch_quote_id: self.batch_quote_id.into_owned(),
            rfq_batch_id: self.rfq_batch_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            rfq_creator_id: self.rfq_creator_id.into_owned(),
            accepted_side: self.accepted_side,
            accepted_ts: self.accepted_ts.into_owned(),
            quotes: self
                .quotes
                .into_iter()
                .map(WsBatchQuoteAcceptedChildRef::into_owned)
                .collect(),
            subaccount: self.subaccount,
        }
    }
}

/// Payload for RfqBatchExecuted.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchExecuted {
    pub rfq_batch_id: String,
    pub client_batch_id: Option<String>,
    pub rfq_creator_id: String,
    pub executed_rfq_ids: Vec<String>,
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    pub subaccount: Option<i64>,
}

/// Borrowed version of [`WsRfqBatchExecuted`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchExecutedRef<'a> {
    #[serde(borrow)]
    pub rfq_batch_id: Cow<'a, str>,
    #[serde(
        default,
        borrow,
        deserialize_with = "super::deserialize_borrowed_option"
    )]
    pub client_batch_id: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub rfq_creator_id: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub executed_rfq_ids: Vec<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub failed_rfq_ids: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub executed_ts: Cow<'a, str>,
    #[serde(borrow)]
    pub close_reason: Cow<'a, str>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

impl<'a> WsRfqBatchExecutedRef<'a> {
    pub fn into_owned(self) -> WsRfqBatchExecuted {
        WsRfqBatchExecuted {
            rfq_batch_id: self.rfq_batch_id.into_owned(),
            client_batch_id: self.client_batch_id.map(Cow::into_owned),
            rfq_creator_id: self.rfq_creator_id.into_owned(),
            executed_rfq_ids: self
                .executed_rfq_ids
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
            failed_rfq_ids: self
                .failed_rfq_ids
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
            executed_ts: self.executed_ts.into_owned(),
            close_reason: self.close_reason.into_owned(),
            subaccount: self.subaccount,
        }
    }
}

/// Payload for BatchQuoteExecuted.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteExecuted {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub rfq_creator_id: String,
    pub executed_rfq_ids: Vec<String>,
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    pub subaccount: Option<i64>,
}

/// Borrowed version of [`WsBatchQuoteExecuted`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteExecutedRef<'a> {
    #[serde(borrow)]
    pub batch_quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_batch_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_creator_id: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub executed_rfq_ids: Vec<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "super::deserialize_borrowed_strings")]
    pub failed_rfq_ids: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub executed_ts: Cow<'a, str>,
    #[serde(borrow)]
    pub close_reason: Cow<'a, str>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

impl<'a> WsBatchQuoteExecutedRef<'a> {
    pub fn into_owned(self) -> WsBatchQuoteExecuted {
        WsBatchQuoteExecuted {
            batch_quote_id: self.batch_quote_id.into_owned(),
            rfq_batch_id: self.rfq_batch_id.into_owned(),
            rfq_creator_id: self.rfq_creator_id.into_owned(),
            executed_rfq_ids: self
                .executed_rfq_ids
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
            failed_rfq_ids: self
                .failed_rfq_ids
                .into_iter()
                .map(Cow::into_owned)
                .collect(),
            executed_ts: self.executed_ts.into_owned(),
            close_reason: self.close_reason.into_owned(),
            subaccount: self.subaccount,
        }
    }
}
