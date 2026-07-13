use crate::ws::protocol::ControlAction;
use crate::ws::types::{WsSubscriptionParamsV2, WsUpdateSubscriptionParamsV2};

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct SubscriptionTracker<P: Clone = WsSubscriptionParamsV2> {
    pending: HashMap<u64, P>,
    active: HashMap<u64, P>,
}

impl<P: Clone> SubscriptionTracker<P> {
    pub(crate) fn record_subscribe_cmd(&mut self, id: u64, params: P) {
        self.pending.insert(id, params);
    }

    pub(crate) fn handle_control_action(&mut self, action: ControlAction) {
        match action {
            ControlAction::Subscribed { cmd_id, sid } => {
                self.handle_subscribed(cmd_id, Some(sid));
            }
            ControlAction::Unsubscribed { sid } => {
                self.handle_unsubscribed(Some(sid));
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

    pub(crate) fn handle_unsubscribed(&mut self, sid: Option<u64>) {
        if let Some(sid) = sid {
            self.active.remove(&sid);
        }
    }

    pub(crate) fn drop_active(&mut self, sid: u64) {
        self.active.remove(&sid);
    }

    pub(crate) fn prepare_resubscribe(&mut self) -> Vec<P> {
        let mut params: Vec<P> = self.active.values().cloned().collect();
        params.extend(self.pending.values().cloned());
        self.active.clear();
        self.pending.clear();
        params
    }
}

impl SubscriptionTracker<WsSubscriptionParamsV2> {
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
                    WsUpdateAction::AddMarkets | WsUpdateAction::SubscribeIndices => {
                        let values = target.get_or_insert_with(Vec::new);
                        for value in incoming {
                            if !values.iter().any(|v| v == &value) {
                                values.push(value);
                            }
                        }
                    }
                    WsUpdateAction::DeleteMarkets | WsUpdateAction::UnsubscribeIndices => {
                        let Some(values) = target.as_mut() else {
                            return;
                        };
                        values.retain(|current| !incoming.iter().any(|value| value == current));
                        if values.is_empty() {
                            *target = None;
                        }
                    }
                    WsUpdateAction::GetSnapshot | WsUpdateAction::Indexlist => {}
                }
            };

        if update.action.is_index_action() {
            let incoming_indices = update.index_ids.clone().unwrap_or_default();
            apply_vec(&mut params.index_ids, incoming_indices, update.action);
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
