use crate::ws::protocol::{ControlAction, EventContractProtocol, WsProtocol};

use std::collections::{BTreeMap, HashMap};

pub(crate) struct SubscriptionTracker<P: WsProtocol = EventContractProtocol> {
    pending: HashMap<u64, P::SubscribeParams>,
    active: HashMap<u64, P::SubscribeParams>,
    pending_unsubscribes: BTreeMap<u64, Vec<u64>>,
    pending_updates: BTreeMap<u64, P::UpdateParams>,
}

impl<P: WsProtocol> Default for SubscriptionTracker<P> {
    fn default() -> Self {
        Self {
            pending: HashMap::new(),
            active: HashMap::new(),
            pending_unsubscribes: BTreeMap::new(),
            pending_updates: BTreeMap::new(),
        }
    }
}

impl<P: WsProtocol> SubscriptionTracker<P> {
    pub(crate) fn record_subscribe_cmd(&mut self, id: u64, params: P::SubscribeParams) {
        self.pending.insert(id, params);
    }

    pub(crate) fn record_unsubscribe_cmd(&mut self, id: u64, sids: Vec<u64>) {
        self.pending_unsubscribes.insert(id, sids);
    }

    pub(crate) fn record_update_cmd(&mut self, id: u64, update: P::UpdateParams) {
        if P::records_update(&update) {
            self.pending_updates.insert(id, update);
        }
    }

    pub(crate) fn drop_pending_subscribe(&mut self, id: u64) {
        self.pending.remove(&id);
    }

    pub(crate) fn drop_pending_unsubscribe(&mut self, id: u64) {
        self.pending_unsubscribes.remove(&id);
    }

    pub(crate) fn drop_pending_update(&mut self, id: u64) {
        self.pending_updates.remove(&id);
    }

    pub(crate) fn apply_update(&mut self, update: &P::UpdateParams) {
        P::apply_update(&mut self.active, update);
    }

    pub(crate) fn handle_control_action(&mut self, action: ControlAction) {
        match action {
            ControlAction::Subscribed { cmd_id, sid } => {
                self.handle_subscribed(cmd_id, Some(sid));
            }
            ControlAction::Unsubscribed { cmd_id, sid } => {
                self.handle_unsubscribed(cmd_id, sid);
            }
            ControlAction::Ok { cmd_id } => {
                self.handle_ok(cmd_id);
            }
            ControlAction::Error { cmd_id } => {
                self.drop_pending_subscribe(cmd_id);
                self.drop_pending_unsubscribe(cmd_id);
                self.drop_pending_update(cmd_id);
            }
        }
    }

    pub(crate) fn handle_subscribed(&mut self, id: Option<u64>, sid: Option<u64>) {
        let (id, sid) = match (id, sid) {
            (Some(id), Some(sid)) => (id, sid),
            _ => return,
        };
        if let Some(params) = self.pending.remove(&id) {
            self.active.insert(sid, params);
        }
    }

    pub(crate) fn handle_unsubscribed(&mut self, id: Option<u64>, sid: Option<u64>) {
        if let Some(sid) = sid {
            self.active.remove(&sid);
        }

        let Some(id) = id else {
            return;
        };

        let Some(pending_sids) = self.pending_unsubscribes.get_mut(&id) else {
            return;
        };
        if let Some(sid) = sid {
            pending_sids.retain(|pending_sid| *pending_sid != sid);
            if pending_sids.is_empty() {
                self.pending_unsubscribes.remove(&id);
            }
            return;
        }

        for sid in self.pending_unsubscribes.remove(&id).unwrap_or_default() {
            self.active.remove(&sid);
        }
    }

    pub(crate) fn handle_ok(&mut self, id: u64) {
        let Some(update) = self.pending_updates.remove(&id) else {
            return;
        };
        self.apply_update(&update);
    }

    /// Prepare subscription parameters for replay upon reconnect.
    ///
    /// A command sent but unacknowledged at disconnect has an unknown venue outcome,
    /// so replay uses the caller's latest intent. An explicit `error` or a local
    /// send failure cancels the intent.
    pub(crate) fn prepare_resubscribe(&mut self) -> Vec<P::SubscribeParams> {
        for update in self.pending_updates.values().cloned().collect::<Vec<_>>() {
            self.apply_update(&update);
        }
        self.pending_updates.clear();

        for sid in self
            .pending_unsubscribes
            .values()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
        {
            self.active.remove(&sid);
        }
        self.pending_unsubscribes.clear();

        let mut params: Vec<P::SubscribeParams> = self.active.values().cloned().collect();
        params.extend(self.pending.values().cloned());
        self.active.clear();
        self.pending.clear();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::protocol::EventContractProtocol;
    use crate::ws::types::{
        WsChannelV2, WsSubscriptionParamsV2, WsUpdateAction, WsUpdateSubscriptionParamsV2,
    };

    type SubscriptionTracker = super::SubscriptionTracker<EventContractProtocol>;
    #[test]
    fn subscription_tracker_moves_pending_to_active() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, params.clone());
        tracker.handle_subscribed(Some(1), Some(42));

        assert!(tracker.pending.is_empty());
        assert_eq!(tracker.active.len(), 1);
        assert_eq!(tracker.active.get(&42), Some(&params));
    }

    #[test]
    fn subscription_tracker_prepare_resubscribe_clears_state() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, params.clone());
        tracker.handle_subscribed(Some(1), Some(42));

        let params = tracker.prepare_resubscribe();
        assert_eq!(params.len(), 1);
        assert!(tracker.pending.is_empty());
        assert!(tracker.active.is_empty());
    }

    #[test]
    fn subscription_tracker_apply_update_changes_fields() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["A".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(10, params);

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["B".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: Some(true),
            skip_ticker_ack: Some(true),
            index_ids: None,
        };
        tracker.apply_update(&update);

        let updated = tracker.active.get(&10).unwrap();
        assert!(
            updated
                .market_tickers
                .as_ref()
                .unwrap()
                .contains(&"A".to_string())
        );
        assert!(
            updated
                .market_tickers
                .as_ref()
                .unwrap()
                .contains(&"B".to_string())
        );
        assert_eq!(updated.send_initial_snapshot, Some(true));
        assert_eq!(updated.skip_ticker_ack, Some(true));
    }

    #[test]
    fn subscription_tracker_get_snapshot_preserves_absent_plural_targets() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ticker: Some("A".to_string()),
            market_tickers: None,
            market_id: Some("ID-A".to_string()),
            market_ids: None,
            ..Default::default()
        };
        tracker.active.insert(10, params.clone());

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(10),
            sids: None,
            market_ticker: Some("B".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        tracker.apply_update(&update);

        let updated = tracker.active.get(&10).unwrap();
        assert_eq!(updated, &params);
    }

    #[test]
    fn subscription_tracker_get_snapshot_does_not_mutate_targets() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["A".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(10, params.clone());

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        tracker.apply_update(&update);

        let updated = tracker.active.get(&10).unwrap();
        assert_eq!(updated.market_tickers, params.market_tickers);
    }

    #[test]
    fn subscription_tracker_apply_update_tracks_cfbenchmarks_indices() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::CfbenchmarksValue],
            index_ids: Some(vec!["BRTI".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(7, params);

        let add = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::SubscribeIndices,
            sid: Some(7),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: Some(vec!["ETHUSD_RR".to_string()]),
        };
        tracker.apply_update(&add);
        let updated = tracker.active.get(&7).unwrap();
        let indices = updated.index_ids.as_ref().unwrap();
        assert!(indices.contains(&"BRTI".to_string()));
        assert!(indices.contains(&"ETHUSD_RR".to_string()));

        let remove = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::UnsubscribeIndices,
            sid: Some(7),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: Some(vec!["BRTI".to_string()]),
        };
        tracker.apply_update(&remove);
        let updated = tracker.active.get(&7).unwrap();
        let indices = updated.index_ids.as_ref().unwrap();
        assert!(!indices.contains(&"BRTI".to_string()));
        assert!(indices.contains(&"ETHUSD_RR".to_string()));
    }

    #[test]
    fn subscription_tracker_applies_update_after_ok_ack() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["A".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(10, params);

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["B".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        tracker.record_update_cmd(99, update);

        assert_eq!(
            tracker.active.get(&10).unwrap().market_tickers,
            Some(vec!["A".to_string()])
        );

        tracker.handle_control_action(ControlAction::Ok { cmd_id: 99 });

        assert_eq!(
            tracker.active.get(&10).unwrap().market_tickers,
            Some(vec!["A".to_string(), "B".to_string()])
        );
        assert!(tracker.pending_updates.is_empty());
    }

    #[test]
    fn subscription_tracker_discards_update_after_send_error() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["A".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(10, params);

        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["B".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        tracker.record_update_cmd(99, update);
        tracker.drop_pending_update(99);

        tracker.handle_control_action(ControlAction::Ok { cmd_id: 99 });

        assert_eq!(
            tracker.active.get(&10).unwrap().market_tickers,
            Some(vec!["A".to_string()])
        );
    }

    #[test]
    fn subscription_tracker_applies_unsubscribe_after_ack() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.active.insert(10, params);
        tracker.record_unsubscribe_cmd(88, vec![10]);

        assert!(tracker.active.contains_key(&10));

        tracker.handle_control_action(ControlAction::Unsubscribed {
            cmd_id: Some(88),
            sid: Some(10),
        });

        assert!(!tracker.active.contains_key(&10));
        assert!(tracker.pending_unsubscribes.is_empty());
    }

    #[test]
    fn subscription_tracker_prepare_resubscribe_folds_pending_desired_state() {
        let mut tracker = SubscriptionTracker::default();
        tracker.active.insert(
            10,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::OrderbookDelta],
                market_tickers: Some(vec!["A".to_string()]),
                ..Default::default()
            },
        );
        tracker.active.insert(
            20,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker],
                market_tickers: Some(vec!["REMOVE".to_string()]),
                ..Default::default()
            },
        );
        tracker.record_update_cmd(
            99,
            WsUpdateSubscriptionParamsV2 {
                action: WsUpdateAction::AddMarkets,
                sid: Some(10),
                sids: None,
                market_ticker: None,
                market_tickers: Some(vec!["B".to_string()]),
                market_id: None,
                market_ids: None,
                send_initial_snapshot: None,
                skip_ticker_ack: None,
                index_ids: None,
            },
        );
        tracker.record_unsubscribe_cmd(88, vec![20]);

        let params = tracker.prepare_resubscribe();

        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0].market_tickers,
            Some(vec!["A".to_string(), "B".to_string()])
        );
        assert!(tracker.pending_updates.is_empty());
        assert!(tracker.pending_unsubscribes.is_empty());
    }

    #[test]
    fn subscription_tracker_error_ack_cancels_pending_update() {
        let mut tracker = SubscriptionTracker::default();
        tracker.active.insert(
            10,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::OrderbookDelta],
                market_tickers: Some(vec!["A".to_string()]),
                ..Default::default()
            },
        );
        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(10),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["B".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        tracker.record_update_cmd(99, update);
        assert_eq!(tracker.pending_updates.len(), 1);

        tracker.handle_control_action(ControlAction::Error { cmd_id: 99 });
        assert!(tracker.pending_updates.is_empty());

        let params = tracker.prepare_resubscribe();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].market_tickers, Some(vec!["A".to_string()]));
    }

    #[test]
    fn subscription_tracker_does_not_record_nonmutating_updates() {
        let mut tracker = SubscriptionTracker::default();
        tracker.active.insert(
            10,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::OrderbookDelta],
                market_tickers: Some(vec!["A".to_string()]),
                ..Default::default()
            },
        );
        let update = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::GetSnapshot,
            sid: Some(10),
            sids: None,
            market_ticker: Some("B".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
        };
        let mut indexlist = update.clone();
        indexlist.action = WsUpdateAction::Indexlist;
        indexlist.market_ticker = None;
        tracker.record_update_cmd(42, update);
        tracker.record_update_cmd(43, indexlist);
        assert!(tracker.pending_updates.is_empty());
    }

    #[test]
    fn validate_subscription_requires_market_tickers_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_subscription_send_initial_snapshot_only_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_orderbook_delta_rejects_market_ids() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ids: Some(vec!["mid-1".to_string()]),
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_enforces_market_target_exclusivity() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_tickers: Some(vec!["B".to_string()]),
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_id: Some("uuid".to_string()),
            ..Default::default()
        };
        assert!(crate::ws::types::validate_subscription(&params).is_err());
    }
}
