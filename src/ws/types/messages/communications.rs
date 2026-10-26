use super::{FixedPointCountRef, FixedPointDollarsRef};
use crate::types::{FixedPointCount, FixedPointDollars, YesNo};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize)]
pub struct WsMveSelectedLeg {
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default)]
    pub yes_settlement_value_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqCreated {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    pub created_ts: String,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    #[serde(default)]
    pub mve_selected_legs: Option<Vec<WsMveSelectedLeg>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqDeleted {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    pub deleted_ts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteCreated {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
    pub created_ts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteAccepted {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub accepted_side: Option<YesNo>,
    #[serde(default)]
    pub contracts_accepted_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteExecuted {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub order_id: String,
    pub client_order_id: String,
    pub market_ticker: String,
    pub executed_ts: String,
    #[serde(default)]
    pub subaccount: Option<i64>,
    /// Set when this trade is a child of an RFQ batch.
    #[serde(default)]
    pub rfq_batch_id: Option<String>,
}

// ---------------------------------------------------------------------------
// RFQ batch messages
//
// Batch messages are low-volume, so the borrowed (`*Ref`) message path reuses these owned
// payloads instead of duplicating every nested struct with `Cow` fields.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: YesNo,
    #[serde(default)]
    pub yes_settlement_value_dollars: Option<FixedPointDollars>,
}

/// A child RFQ announced inside [`WsRfqBatchCreated`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchChild {
    pub id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    /// Indices into the batch's `legs`.
    #[serde(default)]
    pub leg_indices: Vec<u32>,
}

/// `rfq_batch_created`: sent to all subscribers (only your own with `user_filter: "self"`).
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchCreated {
    pub id: String,
    pub creator_id: String,
    pub created_ts: String,
    #[serde(default)]
    pub legs: Vec<WsRfqBatchLeg>,
    #[serde(default)]
    pub rfqs: Vec<WsRfqBatchChild>,
}

/// `rfq_batch_deleted`: the batch closed together with all of its child RFQs.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchDeleted {
    pub id: String,
    pub creator_id: String,
    #[serde(default)]
    pub rfq_ids: Vec<String>,
    pub deleted_ts: String,
}

/// One child quote inside [`WsBatchQuoteCreated`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteEntry {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
}

/// `batch_quote_created`: sent to the maker that created the batch quote.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteCreated {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub created_ts: String,
    #[serde(default)]
    pub quotes: Vec<WsBatchQuoteEntry>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

/// Best bid per child RFQ inside [`WsRfqBatchBestQuotes`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuote {
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub yes_bid_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub no_bid_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
}

/// `rfq_batch_best_quotes`: sent to the batch requester when the best bids change.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchBestQuotes {
    pub rfq_batch_id: String,
    #[serde(default)]
    pub client_batch_id: Option<String>,
    pub version: i64,
    pub updated_ts: String,
    /// Every child has a YES-side bid, so the batch can be accepted on YES.
    pub yes_complete: bool,
    /// Every child has a NO-side bid, so the batch can be accepted on NO.
    pub no_complete: bool,
    #[serde(default)]
    pub rfqs: Vec<WsRfqBatchBestQuote>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

/// One child quote inside [`WsBatchQuoteAccepted`].
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAcceptedEntry {
    pub quote_id: String,
    pub rfq_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub contracts_accepted_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
}

/// `batch_quote_accepted`: sent to each winning maker, who must confirm within the window.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteAccepted {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub accepted_side: YesNo,
    pub accepted_ts: String,
    #[serde(default)]
    pub quotes: Vec<WsBatchQuoteAcceptedEntry>,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

/// `rfq_batch_executed`: sent to the batch requester when execution finishes.
#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqBatchExecuted {
    pub rfq_batch_id: String,
    #[serde(default)]
    pub client_batch_id: Option<String>,
    pub rfq_creator_id: String,
    #[serde(default)]
    pub executed_rfq_ids: Vec<String>,
    #[serde(default)]
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

/// `batch_quote_executed`: sent to each winning maker for its own children.
#[derive(Debug, Clone, Deserialize)]
pub struct WsBatchQuoteExecuted {
    pub batch_quote_id: String,
    pub rfq_batch_id: String,
    pub rfq_creator_id: String,
    #[serde(default)]
    pub executed_rfq_ids: Vec<String>,
    #[serde(default)]
    pub failed_rfq_ids: Vec<String>,
    pub executed_ts: String,
    pub close_reason: String,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

/// Communications message payloads (RFQs and quotes).
#[derive(Debug, Clone)]
pub enum WsCommunications {
    RfqCreated(WsRfqCreated),
    RfqDeleted(WsRfqDeleted),
    QuoteCreated(WsQuoteCreated),
    QuoteAccepted(WsQuoteAccepted),
    QuoteExecuted(WsQuoteExecuted),
    RfqBatchCreated(WsRfqBatchCreated),
    RfqBatchDeleted(WsRfqBatchDeleted),
    BatchQuoteCreated(WsBatchQuoteCreated),
    RfqBatchBestQuotes(WsRfqBatchBestQuotes),
    BatchQuoteAccepted(WsBatchQuoteAccepted),
    RfqBatchExecuted(WsRfqBatchExecuted),
    BatchQuoteExecuted(WsBatchQuoteExecuted),
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMveSelectedLegRef<'a> {
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub market_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default, borrow)]
    pub yes_settlement_value_dollars: Option<FixedPointDollarsRef<'a>>,
}

impl<'a> WsMveSelectedLegRef<'a> {
    pub fn into_owned(self) -> WsMveSelectedLeg {
        WsMveSelectedLeg {
            event_ticker: self.event_ticker.map(Cow::into_owned),
            market_ticker: self.market_ticker.map(Cow::into_owned),
            side: self.side,
            yes_settlement_value_dollars: self.yes_settlement_value_dollars.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqCreatedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub contracts_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
    #[serde(default, borrow)]
    pub mve_collection_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub mve_selected_legs: Option<Vec<WsMveSelectedLegRef<'a>>>,
}

impl<'a> WsRfqCreatedRef<'a> {
    pub fn into_owned(self) -> WsRfqCreated {
        WsRfqCreated {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            target_cost_dollars: self.target_cost_dollars.map(Cow::into_owned),
            created_ts: self.created_ts.into_owned(),
            mve_collection_ticker: self.mve_collection_ticker.map(Cow::into_owned),
            mve_selected_legs: self.mve_selected_legs.map(|legs| {
                legs.into_iter()
                    .map(WsMveSelectedLegRef::into_owned)
                    .collect()
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqDeletedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub contracts_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub deleted_ts: Cow<'a, str>,
}

impl<'a> WsRfqDeletedRef<'a> {
    pub fn into_owned(self) -> WsRfqDeleted {
        WsRfqDeleted {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            target_cost_dollars: self.target_cost_dollars.map(Cow::into_owned),
            deleted_ts: self.deleted_ts.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteCreatedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub no_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(default, borrow)]
    pub yes_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub no_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub rfq_target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
}

impl<'a> WsQuoteCreatedRef<'a> {
    pub fn into_owned(self) -> WsQuoteCreated {
        WsQuoteCreated {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
            created_ts: self.created_ts.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteAcceptedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub no_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(default)]
    pub accepted_side: Option<YesNo>,
    #[serde(default, borrow)]
    pub contracts_accepted_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub yes_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub no_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub rfq_target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
}

impl<'a> WsQuoteAcceptedRef<'a> {
    pub fn into_owned(self) -> WsQuoteAccepted {
        WsQuoteAccepted {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            accepted_side: self.accepted_side,
            contracts_accepted_fp: self.contracts_accepted_fp.map(Cow::into_owned),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteExecutedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub order_id: Cow<'a, str>,
    #[serde(borrow)]
    pub client_order_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub executed_ts: Cow<'a, str>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default, borrow)]
    pub rfq_batch_id: Option<Cow<'a, str>>,
}

impl<'a> WsQuoteExecutedRef<'a> {
    pub fn into_owned(self) -> WsQuoteExecuted {
        WsQuoteExecuted {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            rfq_creator_id: self.rfq_creator_id.into_owned(),
            order_id: self.order_id.into_owned(),
            client_order_id: self.client_order_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            executed_ts: self.executed_ts.into_owned(),
            subaccount: self.subaccount,
            rfq_batch_id: self.rfq_batch_id.map(Cow::into_owned),
        }
    }
}

/// Communications message payloads (RFQs and quotes).
#[derive(Debug, Clone)]
pub enum WsCommunicationsRef<'a> {
    RfqCreated(WsRfqCreatedRef<'a>),
    RfqDeleted(WsRfqDeletedRef<'a>),
    QuoteCreated(WsQuoteCreatedRef<'a>),
    QuoteAccepted(WsQuoteAcceptedRef<'a>),
    QuoteExecuted(WsQuoteExecutedRef<'a>),
    RfqBatchCreated(WsRfqBatchCreated),
    RfqBatchDeleted(WsRfqBatchDeleted),
    BatchQuoteCreated(WsBatchQuoteCreated),
    RfqBatchBestQuotes(WsRfqBatchBestQuotes),
    BatchQuoteAccepted(WsBatchQuoteAccepted),
    RfqBatchExecuted(WsRfqBatchExecuted),
    BatchQuoteExecuted(WsBatchQuoteExecuted),
}

impl<'a> WsCommunicationsRef<'a> {
    pub fn into_owned(self) -> WsCommunications {
        match self {
            WsCommunicationsRef::RfqCreated(msg) => WsCommunications::RfqCreated(msg.into_owned()),
            WsCommunicationsRef::RfqDeleted(msg) => WsCommunications::RfqDeleted(msg.into_owned()),
            WsCommunicationsRef::QuoteCreated(msg) => {
                WsCommunications::QuoteCreated(msg.into_owned())
            }
            WsCommunicationsRef::QuoteAccepted(msg) => {
                WsCommunications::QuoteAccepted(msg.into_owned())
            }
            WsCommunicationsRef::QuoteExecuted(msg) => {
                WsCommunications::QuoteExecuted(msg.into_owned())
            }
            WsCommunicationsRef::RfqBatchCreated(msg) => WsCommunications::RfqBatchCreated(msg),
            WsCommunicationsRef::RfqBatchDeleted(msg) => WsCommunications::RfqBatchDeleted(msg),
            WsCommunicationsRef::BatchQuoteCreated(msg) => WsCommunications::BatchQuoteCreated(msg),
            WsCommunicationsRef::RfqBatchBestQuotes(msg) => {
                WsCommunications::RfqBatchBestQuotes(msg)
            }
            WsCommunicationsRef::BatchQuoteAccepted(msg) => {
                WsCommunications::BatchQuoteAccepted(msg)
            }
            WsCommunicationsRef::RfqBatchExecuted(msg) => WsCommunications::RfqBatchExecuted(msg),
            WsCommunicationsRef::BatchQuoteExecuted(msg) => {
                WsCommunications::BatchQuoteExecuted(msg)
            }
        }
    }
}
