use crate::ws::types::{WsMessageV2, WsSubscriptionParamsV2, WsUpdateSubscriptionParamsV2};

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct SubscriptionTracker {
    pending: HashMap<u64, WsSubscriptionParamsV2>,
    active: HashMap<u64, WsSubscriptionParamsV2>,
}

impl SubscriptionTracker {
    pub(crate) fn record_subscribe_cmd(&mut self, id: u64, params: WsSubscriptionParamsV2) {
        self.pending.insert(id, params);
    }

    pub(crate) fn handle_message(&mut self, msg: &WsMessageV2) {
        match msg {
            WsMessageV2::Subscribed {
                id: Some(id),
                sid: Some(sid),
            } => {
                self.handle_subscribed(Some(*id), Some(*sid));
            }
            WsMessageV2::Unsubscribed { sid: Some(sid), .. } => {
                self.handle_unsubscribed(Some(*sid));
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

    pub(crate) fn handle_unsubscribed(&mut self, sid: Option<u64>) {
        if let Some(sid) = sid {
            self.active.remove(&sid);
        }
    }

    pub(crate) fn drop_active(&mut self, sid: u64) {
        self.active.remove(&sid);
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

                match action {
                    WsUpdateAction::AddMarkets
                    | WsUpdateAction::SubscribeIndices
                    | WsUpdateAction::SubscribeUnderlyings => {
                        let values = target.get_or_insert_with(Vec::new);
                        for value in incoming {
                            if !values.iter().any(|v| v == &value) {
                                values.push(value);
                            }
                        }
                    }
                    WsUpdateAction::DeleteMarkets
                    | WsUpdateAction::UnsubscribeIndices
                    | WsUpdateAction::UnsubscribeUnderlyings => {
                        let Some(values) = target.as_mut() else {
                            return;
                        };
                        values.retain(|current| !incoming.iter().any(|value| value == current));
                        if values.is_empty() {
                            *target = None;
                        }
                    }
                    WsUpdateAction::GetSnapshot
                    | WsUpdateAction::Indexlist
                    | WsUpdateAction::UnderlyingList => {}
                }
            };

        if update.action.is_index_action() {
            // CF Benchmarks index actions only mutate the tracked index set so
            // that a reconnect resubscribes with the correct indices.
            let incoming_indices = update.index_ids.clone().unwrap_or_default();
            apply_vec(&mut params.index_ids, incoming_indices, update.action);
        } else if update.action.is_underlying_action() {
            // Pyth underlying actions only mutate the tracked underlying set so
            // that a reconnect resubscribes with the correct underlyings.
            let incoming_underlyings = update.underlying_tickers.clone().unwrap_or_default();
            apply_vec(
                &mut params.underlying_tickers,
                incoming_underlyings,
                update.action,
            );
        } else {
            apply_vec(&mut params.market_tickers, incoming_tickers, update.action);
            apply_vec(&mut params.market_ids, incoming_ids, update.action);
        }

        if let Some(value) = update.send_initial_snapshot {
            params.send_initial_snapshot = Some(value);
        }
        if let Some(value) = update.skip_ticker_ack {
            params.skip_ticker_ack = Some(value);
        }
    }

    pub(crate) fn prepare_resubscribe(&mut self) -> Vec<WsSubscriptionParamsV2> {
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
            index_ids: None,
            underlying_tickers: None,
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
            underlying_tickers: None,
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
            market_ticker: Some("B".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
            underlying_tickers: None,
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
            underlying_tickers: None,
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
            underlying_tickers: None,
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
            underlying_tickers: None,
        };
        tracker.apply_update(&remove);
        let updated = tracker.active.get(&7).unwrap();
        let indices = updated.index_ids.as_ref().unwrap();
        assert!(!indices.contains(&"BRTI".to_string()));
        assert!(indices.contains(&"ETHUSD_RR".to_string()));
    }

    #[test]
    fn subscription_tracker_apply_update_tracks_pyth_underlyings() {
        let mut tracker = SubscriptionTracker::default();
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::PythValue],
            underlying_tickers: Some(vec!["BTCUSD".to_string()]),
            ..Default::default()
        };
        tracker.active.insert(8, params);

        let add = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::SubscribeUnderlyings,
            sid: Some(8),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
            underlying_tickers: Some(vec!["ETHUSD".to_string()]),
        };
        tracker.apply_update(&add);
        let updated = tracker.active.get(&8).unwrap();
        let underlyings = updated.underlying_tickers.as_ref().unwrap();
        assert!(underlyings.contains(&"BTCUSD".to_string()));
        assert!(underlyings.contains(&"ETHUSD".to_string()));

        let remove = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::UnsubscribeUnderlyings,
            sid: Some(8),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
            index_ids: None,
            underlying_tickers: Some(vec!["BTCUSD".to_string()]),
        };
        tracker.apply_update(&remove);
        let updated = tracker.active.get(&8).unwrap();
        let underlyings = updated.underlying_tickers.as_ref().unwrap();
        assert!(!underlyings.contains(&"BTCUSD".to_string()));
        assert!(underlyings.contains(&"ETHUSD".to_string()));
    }
}
