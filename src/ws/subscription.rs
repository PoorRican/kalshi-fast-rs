use crate::ws::protocol::{ControlAction, EventContractProtocol, WsProtocol};

use std::collections::{BTreeMap, HashMap};

pub(crate) struct SubscriptionTracker<P: WsProtocol = EventContractProtocol> {
    next_id: u64,
    pending: HashMap<u64, P::SubscribeParams>,
    active: HashMap<u64, P::SubscribeParams>,
    pending_unsubscribes: BTreeMap<u64, Vec<u64>>,
    pending_updates: BTreeMap<u64, P::UpdateParams>,
    /// Client-side updates owed to acknowledged subscriptions, not yet sent.
    follow_ups: Vec<(u64, P::UpdateParams)>,
}

impl<P: WsProtocol> Default for SubscriptionTracker<P> {
    fn default() -> Self {
        Self {
            next_id: 1,
            pending: HashMap::new(),
            active: HashMap::new(),
            pending_unsubscribes: BTreeMap::new(),
            pending_updates: BTreeMap::new(),
            follow_ups: Vec::new(),
        }
    }
}

impl<P: WsProtocol> SubscriptionTracker<P> {
    pub(crate) fn next_cmd_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("websocket command IDs exhausted");
        id
    }

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

    /// Apply a control frame. Returns `true` when follow-up updates are owed;
    /// drain them with [`Self::take_follow_ups`].
    pub(crate) fn handle_control_action(&mut self, action: ControlAction) -> bool {
        match action {
            ControlAction::Subscribed {
                cmd_id,
                sid,
                channel,
            } => {
                self.handle_subscribed_channel(cmd_id, Some(sid), channel.as_deref());
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
        !self.follow_ups.is_empty()
    }

    /// Take the owed follow-up updates. Each is already recorded as a pending
    /// update under its command ID; drop it on a local send failure.
    pub(crate) fn take_follow_ups(&mut self) -> Vec<(u64, P::UpdateParams)> {
        std::mem::take(&mut self.follow_ups)
    }

    #[cfg(test)]
    pub(crate) fn handle_subscribed(&mut self, id: Option<u64>, sid: Option<u64>) {
        self.handle_subscribed_channel(id, sid, None);
    }

    fn handle_subscribed_channel(
        &mut self,
        id: Option<u64>,
        sid: Option<u64>,
        channel: Option<&str>,
    ) {
        let (id, sid) = match (id, sid) {
            (Some(id), Some(sid)) => (id, sid),
            _ => return,
        };
        if let Some(params) = P::take_subscribed(&mut self.pending, id, channel) {
            let active = match self.active.entry(sid) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    let active = entry.into_mut();
                    P::merge_subscribe(active, params);
                    active
                }
                std::collections::hash_map::Entry::Vacant(entry) => entry.insert(params),
            };
            if let Some(update) = P::follow_up_update(sid, active) {
                let id = self.next_cmd_id();
                self.record_update_cmd(id, update.clone());
                self.follow_ups.push((id, update));
            }
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
        self.follow_ups.clear();

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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: Some(vec!["ETHUSD_RR".to_string()]),
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: Some(vec!["BRTI".to_string()]),
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
                index_ids: None,
                underlying_tickers: None,
                match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
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
    fn update(action: WsUpdateAction, sid: u64) -> WsUpdateSubscriptionParamsV2 {
        WsUpdateSubscriptionParamsV2 {
            action,
            sid: Some(sid),
            sids: None,
            market_ticker: None,
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            index_ids: None,
            underlying_tickers: None,
            match_ids: None,
        }
    }

    #[test]
    fn repeated_orderbook_sid_unions_and_replays_all_targets_and_options() {
        let mut tracker = SubscriptionTracker::default();
        let first = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ticker: Some("A".into()),
            use_yes_price: Some(true),
            update_interval_ms: Some(100),
            ..Default::default()
        };
        let second = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["B".into(), "A".into()]),
            use_yes_price: Some(true),
            update_interval_ms: Some(100),
            ..Default::default()
        };
        tracker.record_subscribe_cmd(1, first);
        tracker.handle_subscribed(Some(1), Some(9));
        tracker.record_subscribe_cmd(2, second);
        tracker.handle_subscribed(Some(2), Some(9));
        let mut add = update(WsUpdateAction::AddMarkets, 9);
        add.market_ticker = Some("C".into());
        tracker.record_update_cmd(3, add);
        let replay = tracker.prepare_resubscribe();
        assert_eq!(replay.len(), 1);
        assert_eq!(
            replay[0].market_tickers.as_deref(),
            Some(&["A".to_owned(), "B".to_owned(), "C".to_owned()][..])
        );
        assert_eq!(replay[0].use_yes_price, Some(true));
        assert_eq!(replay[0].update_interval_ms, Some(100));
    }

    #[test]
    fn each_channel_ack_retains_other_pending_channels() {
        let mut tracker = SubscriptionTracker::default();
        tracker.record_subscribe_cmd(
            1,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::Ticker, WsChannelV2::PythValue],
                underlying_tickers: Some(vec!["Metal.XAU/USD".into()]),
                ..Default::default()
            },
        );
        tracker.handle_control_action(ControlAction::Subscribed {
            cmd_id: Some(1),
            sid: 5,
            channel: Some("ticker".into()),
        });
        tracker.handle_control_action(ControlAction::Subscribed {
            cmd_id: Some(1),
            sid: 6,
            channel: Some("pyth_value".into()),
        });
        let replay = tracker.prepare_resubscribe();
        assert_eq!(replay.len(), 2);
        assert!(replay.iter().any(|p| p.channels == [WsChannelV2::PythValue]
            && p.underlying_tickers == Some(vec!["Metal.XAU/USD".into()])));
        assert!(replay.iter().any(|p| p.channels == [WsChannelV2::Ticker]));
    }

    #[test]
    fn pyth_updates_acknowledge_and_fold_without_losing_explicit_all_mode_selections() {
        let mut tracker = SubscriptionTracker::default();
        tracker.active.insert(
            7,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::PythValue],
                underlying_tickers: Some(vec!["Metal.XAU/USD".into()]),
                ..Default::default()
            },
        );
        let mut all = update(WsUpdateAction::SubscribeUnderlyings, 7);
        all.underlying_tickers = Some(vec!["all".into()]);
        tracker.record_update_cmd(10, all);
        tracker.handle_control_action(ControlAction::Ok { cmd_id: 10 });
        let mut remove_all = update(WsUpdateAction::UnsubscribeUnderlyings, 7);
        remove_all.underlying_tickers = Some(vec!["all".into()]);
        tracker.record_update_cmd(11, remove_all);
        let replay = tracker.prepare_resubscribe();
        assert_eq!(
            replay[0].underlying_tickers,
            Some(vec!["Metal.XAU/USD".into()])
        );
    }
    #[test]
    fn ebattle_matches_follow_up_after_ack_and_after_replay() {
        let mut tracker = SubscriptionTracker::default();
        let id = tracker.next_cmd_id();
        tracker.record_subscribe_cmd(
            id,
            WsSubscriptionParamsV2 {
                channels: vec![WsChannelV2::EbattleScores, WsChannelV2::Ticker],
                match_ids: Some(vec!["m1".into()]),
                ..Default::default()
            },
        );
        // The ticker ack must not carry the match filter or owe a follow-up.
        assert!(!tracker.handle_control_action(ControlAction::Subscribed {
            cmd_id: Some(id),
            sid: 3,
            channel: Some("ticker".into()),
        }));
        assert_eq!(tracker.active[&3].match_ids, None);
        assert!(tracker.handle_control_action(ControlAction::Subscribed {
            cmd_id: Some(id),
            sid: 4,
            channel: Some("ebattle_scores".into()),
        }));
        let follow_ups = tracker.take_follow_ups();
        assert_eq!(follow_ups.len(), 1);
        let (cmd_id, follow_up) = &follow_ups[0];
        assert_eq!(follow_up.action, WsUpdateAction::SubscribeMatches);
        assert_eq!(follow_up.sid, Some(4));
        assert_eq!(follow_up.match_ids, Some(vec!["m1".to_string()]));
        crate::ws::types::validate_update(follow_up).expect("follow-up valid");
        tracker.handle_control_action(ControlAction::Ok { cmd_id: *cmd_id });

        let mut add = update(WsUpdateAction::SubscribeMatches, 4);
        add.match_ids = Some(vec!["m2".into()]);
        tracker.record_update_cmd(20, add);
        tracker.handle_control_action(ControlAction::Ok { cmd_id: 20 });
        tracker.record_update_cmd(21, update(WsUpdateAction::Matchlist, 4));
        assert!(tracker.take_follow_ups().is_empty());

        let replay = tracker.prepare_resubscribe();
        let ebattle = replay
            .into_iter()
            .find(|p| p.channels == [WsChannelV2::EbattleScores])
            .expect("ebattle replay");
        assert_eq!(ebattle.match_ids, Some(vec!["m1".into(), "m2".into()]));
        // match_ids never reaches the subscribe command.
        assert!(
            serde_json::to_value(&ebattle)
                .unwrap()
                .get("match_ids")
                .is_none()
        );

        let id = tracker.next_cmd_id();
        tracker.record_subscribe_cmd(id, ebattle);
        assert!(tracker.handle_control_action(ControlAction::Subscribed {
            cmd_id: Some(id),
            sid: 9,
            channel: Some("ebattle_scores".into()),
        }));
        let follow_ups = tracker.take_follow_ups();
        assert_eq!(follow_ups[0].1.sid, Some(9));
        assert_eq!(
            follow_ups[0].1.match_ids,
            Some(vec!["m1".to_string(), "m2".to_string()])
        );
    }

    #[test]
    fn margin_repeated_orderbook_and_pending_update_replay_union() {
        use crate::margin::ws::channel::MarginChannel;
        use crate::margin::ws::subscription::{
            MarginSubscribeParams, MarginUpdateAction, MarginUpdateSubscriptionParams,
        };
        let mut tracker =
            super::SubscriptionTracker::<crate::ws::protocol::MarginProtocol>::default();
        tracker.record_subscribe_cmd(
            1,
            MarginSubscribeParams {
                channels: vec![MarginChannel::OrderbookDelta],
                market_ticker: Some("A".into()),
                update_interval_ms: Some(100),
                ..Default::default()
            },
        );
        tracker.handle_subscribed(Some(1), Some(9));
        tracker.record_subscribe_cmd(
            2,
            MarginSubscribeParams {
                channels: vec![MarginChannel::OrderbookDelta],
                market_tickers: Some(vec!["B".into()]),
                update_interval_ms: Some(100),
                ..Default::default()
            },
        );
        tracker.handle_subscribed(Some(2), Some(9));
        tracker.record_update_cmd(
            3,
            MarginUpdateSubscriptionParams {
                action: MarginUpdateAction::AddMarkets,
                sid: Some(9),
                sids: None,
                market_ticker: Some("C".into()),
                market_tickers: None,
                send_initial_snapshot: None,
            },
        );
        let replay = tracker.prepare_resubscribe();
        assert_eq!(replay.len(), 1);
        assert_eq!(
            replay[0].market_tickers,
            Some(vec!["A".into(), "B".into(), "C".into()])
        );
        assert_eq!(replay[0].update_interval_ms, Some(100));
    }
    #[test]
    fn event_replay_projects_options_after_partial_and_complete_channel_acks() {
        use WsChannelV2 as C;
        let channels = [
            C::Ticker,
            C::OrderbookDelta,
            C::Communications,
            C::CfbenchmarksValue,
            C::CfbenchmarksValue5hz,
            C::PythValue,
        ];
        for reverse in [false, true] {
            let mut order = channels;
            if reverse {
                order.reverse();
            }
            for ack_count in 1..=order.len() {
                let mut tracker = SubscriptionTracker::default();
                let params = WsSubscriptionParamsV2 {
                    channels: channels.to_vec(),
                    market_ticker: Some("MARKET".into()),
                    send_initial_snapshot: Some(true),
                    skip_ticker_ack: Some(true),
                    use_yes_price: Some(true),
                    update_interval_ms: Some(100),
                    shard_factor: Some(4),
                    shard_key: Some(2),
                    user_filter: Some("self".into()),
                    index_ids: Some(vec!["BRTI".into()]),
                    underlying_tickers: Some(vec!["Crypto.BTC/USD".into()]),
                    ..Default::default()
                };
                EventContractProtocol::validate_subscribe(&params).expect("original valid");
                tracker.record_subscribe_cmd(1, params);
                for (i, channel) in order.iter().take(ack_count).enumerate() {
                    tracker.handle_control_action(ControlAction::Subscribed {
                        cmd_id: Some(1),
                        sid: i as u64 + 10,
                        channel: Some(channel.as_str().into()),
                    });
                }
                let replay = tracker.prepare_resubscribe();
                assert_eq!(
                    replay.len(),
                    ack_count + usize::from(ack_count < order.len())
                );
                for p in replay {
                    EventContractProtocol::validate_subscribe(&p).expect("replay valid");
                    let has = |c| p.channels.contains(&c);
                    assert_eq!(p.market_ticker.as_deref(), Some("MARKET"));
                    assert_eq!(
                        p.send_initial_snapshot.is_some(),
                        has(C::Ticker) || has(C::OrderbookDelta)
                    );
                    assert_eq!(p.skip_ticker_ack, Some(true));
                    assert_eq!(p.use_yes_price.is_some(), has(C::OrderbookDelta));
                    assert_eq!(p.update_interval_ms.is_some(), has(C::OrderbookDelta));
                    assert_eq!(p.shard_factor.is_some(), has(C::Communications));
                    assert_eq!(p.shard_key.is_some(), has(C::Communications));
                    assert_eq!(p.user_filter.is_some(), has(C::Communications));
                    assert_eq!(
                        p.index_ids.is_some(),
                        has(C::CfbenchmarksValue) || has(C::CfbenchmarksValue5hz)
                    );
                    assert_eq!(p.underlying_tickers.is_some(), has(C::PythValue));
                    if has(C::OrderbookDelta) {
                        assert_eq!(p.use_yes_price, Some(true));
                        assert_eq!(p.update_interval_ms, Some(100));
                    }
                    if has(C::Communications) {
                        assert_eq!(p.shard_factor, Some(4));
                        assert_eq!(p.shard_key, Some(2));
                        assert_eq!(p.user_filter.as_deref(), Some("self"));
                    }
                    if has(C::CfbenchmarksValue) || has(C::CfbenchmarksValue5hz) {
                        assert_eq!(p.index_ids.as_deref(), Some(&["BRTI".to_owned()][..]));
                    }
                    if has(C::PythValue) {
                        assert_eq!(
                            p.underlying_tickers.as_deref(),
                            Some(&["Crypto.BTC/USD".to_owned()][..])
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn margin_replay_projects_options_after_partial_and_complete_channel_acks() {
        use crate::margin::ws::channel::MarginChannel as C;
        use crate::margin::ws::subscription::MarginSubscribeParams;
        use crate::ws::protocol::{Channel, MarginProtocol};
        for first in [C::Ticker, C::OrderbookDelta] {
            for complete in [false, true] {
                let mut tracker = super::SubscriptionTracker::<MarginProtocol>::default();
                let params = MarginSubscribeParams {
                    channels: vec![C::Ticker, C::OrderbookDelta],
                    market_tickers: Some(vec!["BTC".into()]),
                    send_initial_snapshot: Some(true),
                    skip_ticker_ack: Some(true),
                    update_interval_ms: Some(100),
                    ..Default::default()
                };
                MarginProtocol::validate_subscribe(&params).expect("original valid");
                tracker.record_subscribe_cmd(1, params);
                let second = if first == C::Ticker {
                    C::OrderbookDelta
                } else {
                    C::Ticker
                };
                for (i, channel) in [first.clone(), second]
                    .iter()
                    .take(if complete { 2 } else { 1 })
                    .enumerate()
                {
                    tracker.handle_control_action(ControlAction::Subscribed {
                        cmd_id: Some(1),
                        sid: i as u64 + 10,
                        channel: Some(channel.as_str().into()),
                    });
                }
                let replay = tracker.prepare_resubscribe();
                assert_eq!(replay.len(), 2);
                for p in replay {
                    MarginProtocol::validate_subscribe(&p).expect("replay valid");
                    assert_eq!(p.market_tickers.as_deref(), Some(&["BTC".to_owned()][..]));
                    assert_eq!(p.send_initial_snapshot, Some(true));
                    assert_eq!(p.skip_ticker_ack, Some(true));
                    assert_eq!(
                        p.update_interval_ms.is_some(),
                        p.channels.contains(&C::OrderbookDelta)
                    );
                }
            }
        }
    }
}
