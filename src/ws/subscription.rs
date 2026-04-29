use crate::ws::types::{WsMessageV2, WsSubscriptionParamsV2, WsUpdateSubscriptionParamsV2};

use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
pub(crate) struct SubscriptionTracker {
    pending: HashMap<u64, WsSubscriptionParamsV2>,
    pending_unsubscribes: BTreeMap<u64, Vec<u64>>,
    pending_updates: BTreeMap<u64, WsUpdateSubscriptionParamsV2>,
    active: HashMap<u64, WsSubscriptionParamsV2>,
}

impl SubscriptionTracker {
    pub(crate) fn record_subscribe_cmd(&mut self, id: u64, params: WsSubscriptionParamsV2) {
        self.pending.insert(id, params);
    }

    pub(crate) fn record_unsubscribe_cmd(&mut self, id: u64, sids: Vec<u64>) {
        self.pending_unsubscribes.insert(id, sids);
    }

    pub(crate) fn record_update_cmd(&mut self, id: u64, params: WsUpdateSubscriptionParamsV2) {
        self.pending_updates.insert(id, params);
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

    pub(crate) fn handle_message(&mut self, msg: &WsMessageV2) {
        match msg {
            WsMessageV2::Subscribed {
                id: Some(id),
                sid: Some(sid),
            } => {
                self.handle_subscribed(Some(*id), Some(*sid));
            }
            WsMessageV2::Unsubscribed { id, sid, .. } => {
                self.handle_unsubscribed(*id, *sid);
            }
            WsMessageV2::Ok { id: Some(id), .. } => {
                self.handle_ok(Some(*id));
            }
            WsMessageV2::Error { id: Some(id), .. } => {
                self.drop_pending_subscribe(*id);
                self.drop_pending_unsubscribe(*id);
                self.drop_pending_update(*id);
            }
            _ => {}
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

    pub(crate) fn handle_ok(&mut self, id: Option<u64>) {
        let Some(id) = id else {
            return;
        };
        let Some(update) = self.pending_updates.remove(&id) else {
            return;
        };
        self.apply_update(&update);
    }

    pub(crate) fn apply_update(&mut self, update: &WsUpdateSubscriptionParamsV2) {
        use crate::ws::types::WsUpdateAction;

        let sid = match update.target_sid() {
            Some(sid) => sid,
            None => return,
        };

        let Some(params) = self.active.get_mut(&sid) else {
            return;
        };

        let mut incoming_tickers = update.market_tickers.clone().unwrap_or_default();
        if let Some(single) = update.market_ticker.clone() {
            incoming_tickers.push(single);
        }

        let mut incoming_ids = update.market_ids.clone().unwrap_or_default();
        if let Some(single) = update.market_id.clone() {
            incoming_ids.push(single);
        }

        let apply_vec =
            |target: &mut Option<Vec<String>>, incoming: Vec<String>, action: WsUpdateAction| {
                if incoming.is_empty() {
                    return;
                }
                let values = target.get_or_insert_with(Vec::new);
                match action {
                    WsUpdateAction::AddMarkets => {
                        for value in incoming {
                            if !values.iter().any(|v| v == &value) {
                                values.push(value);
                            }
                        }
                    }
                    WsUpdateAction::DeleteMarkets => {
                        values.retain(|current| !incoming.iter().any(|value| value == current));
                        if values.is_empty() {
                            *target = None;
                        }
                    }
                    WsUpdateAction::GetSnapshot => {}
                }
            };

        apply_vec(&mut params.market_tickers, incoming_tickers, update.action);
        apply_vec(&mut params.market_ids, incoming_ids, update.action);

        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
        if let Some(value) = update.skip_ticker_ack {
            params.skip_ticker_ack = Some(value);
        }
    }

    pub(crate) fn prepare_resubscribe(&mut self) -> Vec<WsSubscriptionParamsV2> {
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

        let mut params: Vec<WsSubscriptionParamsV2> = self.active.values().cloned().collect();
        params.extend(self.pending.values().cloned());
        self.active.clear();
        self.pending.clear();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::types::{WsChannelV2, WsUpdateAction};

    #[test]
    fn subscription_tracker_moves_pending_to_active() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, params.clone());
        tracker.handle_message(&WsMessageV2::Subscribed {
            id: Some(1),
            sid: Some(42),
        });

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
        tracker.handle_message(&WsMessageV2::Subscribed {
            id: Some(1),
            sid: Some(42),
        });

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
        };
        tracker.record_update_cmd(99, update);

        assert_eq!(
            tracker.active.get(&10).unwrap().market_tickers,
            Some(vec!["A".to_string()])
        );

        tracker.handle_message(&WsMessageV2::Ok {
            id: Some(99),
            sid: Some(10),
            seq: Some(7),
        });

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
        };
        tracker.record_update_cmd(99, update);
        tracker.drop_pending_update(99);

        tracker.handle_message(&WsMessageV2::Ok {
            id: Some(99),
            sid: Some(10),
            seq: Some(7),
        });

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

        tracker.handle_message(&WsMessageV2::Unsubscribed {
            id: Some(88),
            sid: Some(10),
            seq: Some(5),
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
            market_ticker: Some("B".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
        };
        tracker.apply_update(&update);

        let updated = tracker.active.get(&10).unwrap();
        assert_eq!(updated.market_tickers, params.market_tickers);
    }
}
