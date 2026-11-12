use kalshi_fast::{WsEnvelope, WsEnvelopeRef, WsMessageRef, WsMessageV2};
use serde_json::Value;

const CAPTURED_MUTATION_FRAMES: &str = include_str!("fixtures/control_frame_mutation.jsonl");

fn captured_frames() -> impl Iterator<Item = &'static str> {
    CAPTURED_MUTATION_FRAMES
        .lines()
        .filter(|line| !line.is_empty())
}

#[test]
fn captured_mutation_control_acks_preserve_subscription_sequences() {
    let frames: Vec<_> = captured_frames().collect();
    assert_eq!(
        frames.len(),
        12,
        "fixture should remain a small captured window"
    );

    let expected_sequences = [
        386119, 386120, 386121, 386122, 386127, 386128, 386129, 386130, 386131, 386132, 386133,
        386134,
    ];

    for (frame, expected_sequence) in frames.iter().zip(expected_sequences) {
        let owned = WsMessageV2::from_bytes(frame.as_bytes()).expect("captured frame must parse");
        assert_eq!(owned.subscription_id(), Some(1));
        assert_eq!(owned.sequence(), Some(expected_sequence));

        let borrowed =
            WsMessageRef::from_bytes(frame.as_bytes()).expect("captured frame must parse");
        assert_eq!(borrowed.subscription_id(), Some(1));
        assert_eq!(borrowed.sequence(), Some(expected_sequence));
        assert_eq!(
            borrowed
                .into_owned()
                .expect("borrowed frame must own")
                .sequence(),
            Some(expected_sequence)
        );
    }

    assert!(matches!(
        WsMessageV2::from_bytes(frames[2].as_bytes()).expect("delete acknowledgement must parse"),
        WsMessageV2::Ok {
            id: Some(2),
            sid: Some(1),
            seq: Some(386121),
            ..
        }
    ));
    assert!(matches!(
        WsMessageV2::from_bytes(frames[3].as_bytes()).expect("add acknowledgement must parse"),
        WsMessageV2::Ok {
            id: Some(3),
            sid: Some(1),
            seq: Some(386122),
            ..
        }
    ));
}

#[test]
fn captured_control_envelope_fields_are_not_silently_dropped() {
    for raw in [
        captured_frames()
            .nth(2)
            .expect("delete acknowledgement fixture"),
        captured_frames()
            .nth(3)
            .expect("add acknowledgement fixture"),
    ] {
        let json: Value = serde_json::from_str(raw).expect("fixture must be JSON");
        let mut raw_fields = json.as_object().expect("fixture must be an object").clone();
        let expected_type = raw_fields
            .remove("type")
            .expect("type must be present")
            .as_str()
            .expect("type must be a string")
            .to_owned();
        let expected_id = raw_fields
            .remove("id")
            .expect("id must be present")
            .as_u64();
        let expected_sid = raw_fields
            .remove("sid")
            .expect("sid must be present")
            .as_u64();
        let expected_sequence = raw_fields
            .remove("seq")
            .expect("seq must be present")
            .as_u64();
        let expected_msg = raw_fields
            .remove("msg")
            .expect("msg must be present")
            .to_string();
        assert!(
            raw_fields.is_empty(),
            "every captured envelope field must be represented: {raw_fields:?}"
        );

        let owned_envelope: WsEnvelope =
            serde_json::from_str(raw).expect("owned envelope must parse");
        assert_eq!(owned_envelope.msg_type.as_str(), expected_type);
        assert_eq!(owned_envelope.id, expected_id);
        assert_eq!(owned_envelope.sid, expected_sid);
        assert_eq!(owned_envelope.seq, expected_sequence);
        assert_eq!(owned_envelope.msg_raw(), Some(expected_msg.as_str()));
        let owned = owned_envelope
            .into_message()
            .expect("owned envelope must convert");
        assert_eq!(owned.subscription_id(), expected_sid);
        assert_eq!(owned.sequence(), expected_sequence);

        let borrowed_envelope: WsEnvelopeRef<'_> =
            serde_json::from_str(raw).expect("borrowed envelope must parse");
        assert_eq!(borrowed_envelope.msg_type.as_str(), expected_type);
        assert_eq!(borrowed_envelope.id, expected_id);
        assert_eq!(borrowed_envelope.sid, expected_sid);
        assert_eq!(borrowed_envelope.seq, expected_sequence);
        assert_eq!(borrowed_envelope.msg_raw(), Some(expected_msg.as_str()));
        let borrowed = borrowed_envelope
            .into_message()
            .expect("borrowed envelope must convert");
        assert_eq!(borrowed.subscription_id(), expected_sid);
        assert_eq!(borrowed.sequence(), expected_sequence);
    }
}

#[test]
fn every_control_arm_preserves_optional_position_metadata() {
    let frames = [
        (
            r#"{"type":"subscribed","id":1,"sid":99,"seq":1,"msg":{"sid":7}}"#,
            Some(99),
            Some(1),
        ),
        (
            r#"{"type":"unsubscribed","id":2,"sid":7,"seq":2}"#,
            Some(7),
            Some(2),
        ),
        (
            r#"{"type":"list_subscriptions","id":3,"sid":7,"seq":3,"subscriptions":[]}"#,
            Some(7),
            Some(3),
        ),
        (
            r#"{"type":"ok","id":4,"sid":7,"seq":4,"msg":{}}"#,
            Some(7),
            Some(4),
        ),
        (
            r#"{"type":"error","id":5,"sid":7,"seq":5,"msg":{"code":10,"msg":"bad request"}}"#,
            Some(7),
            Some(5),
        ),
    ];

    for (frame, sid, sequence) in frames {
        let wire_owned = WsMessageV2::from_bytes(frame.as_bytes()).expect("wire frame must parse");
        assert_eq!(wire_owned.subscription_id(), sid);
        assert_eq!(wire_owned.sequence(), sequence);

        let wire_borrowed =
            WsMessageRef::from_bytes(frame.as_bytes()).expect("borrowed wire frame must parse");
        assert_eq!(wire_borrowed.subscription_id(), sid);
        assert_eq!(wire_borrowed.sequence(), sequence);

        let envelope_owned: WsEnvelope =
            serde_json::from_str(frame).expect("owned envelope must parse");
        let envelope_owned = envelope_owned
            .into_message()
            .expect("owned envelope must convert");
        assert_eq!(envelope_owned.subscription_id(), sid);
        assert_eq!(envelope_owned.sequence(), sequence);

        let envelope_borrowed: WsEnvelopeRef<'_> =
            serde_json::from_str(frame).expect("borrowed envelope must parse");
        let envelope_borrowed = envelope_borrowed
            .into_message()
            .expect("borrowed envelope must convert");
        assert_eq!(envelope_borrowed.subscription_id(), sid);
        assert_eq!(envelope_borrowed.sequence(), sequence);
    }
}

fn assert_borrowed_from(bytes: &[u8], value: &str) {
    let start = bytes.as_ptr() as usize;
    let ptr = value.as_ptr() as usize;
    assert!(ptr >= start && ptr + value.len() <= start + bytes.len());
}

#[test]
fn control_payloads_preserve_metadata_and_borrow_filter_strings() {
    use kalshi_fast::WsChannelV2;

    let ack = br#"{"type":"subscribed","id":1,"sid":7,"seq":8,"sending_ts_ms":123,"msg":{"channel":"pyth_value","sid":7}}"#;
    let owned = WsMessageV2::from_bytes(ack).unwrap();
    assert!(matches!(
        owned,
        WsMessageV2::Subscribed {
            channel: Some(WsChannelV2::PythValue),
            ..
        }
    ));
    assert_eq!(owned.sending_ts_ms(), Some(123));
    let borrowed = WsMessageRef::from_bytes(ack).unwrap();
    assert!(matches!(
        borrowed,
        WsMessageRef::Subscribed {
            channel: Some(WsChannelV2::PythValue),
            ..
        }
    ));
    assert_eq!(borrowed.into_owned().unwrap().sending_ts_ms(), Some(123));

    let filters = br#"{"type":"ok","sid":7,"seq":9,"sending_ts_ms":124,"msg":{"market_tickers":["MARKET"],"market_ids":["ID"],"index_ids":["BRTI"],"underlying_tickers":["Metal.XAU/USD"]}}"#;
    let borrowed = WsMessageRef::from_bytes(filters).unwrap();
    assert_eq!(borrowed.sending_ts_ms(), Some(124));
    match &borrowed {
        WsMessageRef::Ok { msg: Some(msg), .. } => {
            for value in [
                &msg.market_tickers.as_ref().unwrap()[0],
                &msg.market_ids.as_ref().unwrap()[0],
                &msg.index_ids.as_ref().unwrap()[0],
                &msg.underlying_tickers.as_ref().unwrap()[0],
            ] {
                assert_borrowed_from(filters, value);
            }
        }
        other => panic!("expected filter acknowledgement, got {other:?}"),
    }
    match borrowed.into_owned().unwrap() {
        WsMessageV2::Ok { msg: Some(msg), .. } => {
            assert_eq!(msg.underlying_tickers.unwrap(), ["Metal.XAU/USD"]);
            assert_eq!(msg.index_ids.unwrap(), ["BRTI"]);
        }
        other => panic!("expected owned acknowledgement, got {other:?}"),
    }

    let list = br#"{"type":"ok","id":3,"sending_ts_ms":125,"msg":[{"channel":"ticker","sid":10}]}"#;
    let parsed = WsMessageRef::from_bytes(list).unwrap();
    match &parsed {
        WsMessageRef::ListSubscriptions { subscriptions, .. } => {
            assert_eq!(subscriptions[0].sid, 10);
            assert_borrowed_from(list, &subscriptions[0].channel);
        }
        other => panic!("expected list acknowledgement, got {other:?}"),
    }
    assert_eq!(parsed.sending_ts_ms(), Some(125));

    for code in [10, 25] {
        let frame = format!(
            r#"{{"type":"error","sid":7,"seq":10,"sending_ts_ms":126,"msg":{{"code":{code},"msg":"channel closed","market_ticker":"M","market_tickers":["M","N"]}}}}"#
        );
        let borrowed = WsMessageRef::from_bytes(frame.as_bytes()).unwrap();
        assert_eq!(borrowed.subscription_id(), Some(7));
        assert_eq!(borrowed.sequence(), Some(10));
        assert_eq!(borrowed.sending_ts_ms(), Some(126));
        match &borrowed {
            WsMessageRef::Error { error, .. } => {
                assert_eq!(error.code, Some(code));
                assert_borrowed_from(frame.as_bytes(), error.msg.as_ref().unwrap());
                assert_borrowed_from(frame.as_bytes(), &error.market_tickers.as_ref().unwrap()[0]);
            }
            other => panic!("expected terminal error, got {other:?}"),
        }
        match borrowed.into_owned().unwrap() {
            WsMessageV2::Error { error, .. } => {
                assert_eq!(error.msg.as_deref(), Some("channel closed"));
                assert_eq!(error.market_ticker.as_deref(), Some("M"));
                assert_eq!(error.market_tickers.unwrap(), ["M", "N"]);
            }
            other => panic!("expected owned terminal error, got {other:?}"),
        }
    }
}

#[test]
fn published_feed_and_batch_events_dispatch_in_all_decode_paths() {
    use kalshi_fast::{WsCommunications, WsDataMessageV2};

    let cases = [
        (
            "cfbenchmarks_value_5hz",
            r#"{"index_id":"BRTI","value_usd":"68000.12","source_ts_ms":100,"received_at":101,"data":"{}"}"#,
        ),
        (
            "cfbenchmarks_value_5hz_indexlist",
            r#"{"index_ids":["BRTI","ETHUSD_RTI"]}"#,
        ),
        (
            "pyth_value",
            r#"{"underlying_ticker":"Metal.XAU/USD","value_usd":"82.1","source_ts_ms":100,"received_at":101}"#,
        ),
        (
            "pyth_value_underlying_list",
            r#"{"underlying_tickers":["Metal.XAU/USD"]}"#,
        ),
        (
            "ebattle_scores",
            r#"{"event_id":"KXEBATTLE-1","match_id":"ms-1","received_at":1700000000000,"data":"{\"game\":\"ebasketball\",\"status\":\"live\",\"home_score\":21}"}"#,
        ),
        (
            "ebattle_scores_matchlist",
            r#"{"match_ids":["ms-1","ms-2"]}"#,
        ),
        (
            "rfq_batch_created",
            r#"{"id":"batch","creator_id":"creator","created_ts":"2024-12-01T10:00:00Z","legs":[],"rfqs":[]}"#,
        ),
        (
            "rfq_batch_deleted",
            r#"{"id":"batch","creator_id":"creator","rfq_ids":["rfq"],"deleted_ts":"2024-12-01T10:05:00Z"}"#,
        ),
        (
            "batch_quote_created",
            r#"{"batch_quote_id":"quote","rfq_batch_id":"batch","quote_creator_id":"creator","created_ts":"2024-12-01T10:02:00Z","quotes":[]}"#,
        ),
        (
            "rfq_batch_best_quotes",
            r#"{"rfq_batch_id":"batch","version":7,"updated_ts":"2024-12-01T10:02:30Z","yes_complete":false,"no_complete":true,"rfqs":[]}"#,
        ),
        (
            "batch_quote_accepted",
            r#"{"batch_quote_id":"quote","rfq_batch_id":"batch","quote_creator_id":"maker","rfq_creator_id":"creator","accepted_side":"no","accepted_ts":"2024-12-01T10:03:00Z","quotes":[]}"#,
        ),
        (
            "rfq_batch_executed",
            r#"{"rfq_batch_id":"batch","rfq_creator_id":"creator","executed_rfq_ids":["rfq"],"failed_rfq_ids":[],"executed_ts":"2024-12-01T10:03:05Z","close_reason":"executed"}"#,
        ),
        (
            "batch_quote_executed",
            r#"{"batch_quote_id":"quote","rfq_batch_id":"batch","rfq_creator_id":"creator","executed_rfq_ids":["rfq"],"failed_rfq_ids":[],"executed_ts":"2024-12-01T10:03:05Z","close_reason":"executed"}"#,
        ),
    ];

    fn kind(message: &WsMessageV2) -> &'static str {
        match message {
            WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5hz { .. }) => {
                "cfbenchmarks_value_5hz"
            }
            WsMessageV2::Data(WsDataMessageV2::CfbenchmarksValue5hzIndexlist { .. }) => {
                "cfbenchmarks_value_5hz_indexlist"
            }
            WsMessageV2::Data(WsDataMessageV2::PythValue { .. }) => "pyth_value",
            WsMessageV2::Data(WsDataMessageV2::PythValueUnderlyingList { .. }) => {
                "pyth_value_underlying_list"
            }
            WsMessageV2::Data(WsDataMessageV2::EbattleScores { .. }) => "ebattle_scores",
            WsMessageV2::Data(WsDataMessageV2::EbattleScoresMatchlist { .. }) => {
                "ebattle_scores_matchlist"
            }
            WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
                WsCommunications::RfqBatchCreated(_) => "rfq_batch_created",
                WsCommunications::RfqBatchDeleted(_) => "rfq_batch_deleted",
                WsCommunications::BatchQuoteCreated(_) => "batch_quote_created",
                WsCommunications::RfqBatchBestQuotes(_) => "rfq_batch_best_quotes",
                WsCommunications::BatchQuoteAccepted(_) => "batch_quote_accepted",
                WsCommunications::RfqBatchExecuted(_) => "rfq_batch_executed",
                WsCommunications::BatchQuoteExecuted(_) => "batch_quote_executed",
                _ => "other communications",
            },
            _ => "other",
        }
    }

    for (event, payload) in cases {
        let frame = format!(
            r#"{{"type":"{event}","sid":15,"seq":23,"sending_ts_ms":1234,"msg":{payload}}}"#
        );
        let owned = WsMessageV2::from_bytes(frame.as_bytes()).unwrap();
        let borrowed = WsMessageRef::from_bytes(frame.as_bytes()).unwrap();
        let owned_env: WsEnvelope = serde_json::from_str(&frame).unwrap();
        let borrowed_env: WsEnvelopeRef<'_> = serde_json::from_str(&frame).unwrap();
        let messages = [
            owned,
            borrowed.into_owned().unwrap(),
            owned_env.into_message().unwrap(),
            borrowed_env.into_message().unwrap().into_owned().unwrap(),
        ];
        for message in messages {
            assert_eq!(kind(&message), event);
            assert_eq!(message.subscription_id(), Some(15));
            assert_eq!(message.sequence(), Some(23));
            assert_eq!(message.sending_ts_ms(), Some(1234));
        }
    }
}

#[test]
fn unpublished_message_types_remain_forward_compatible() {
    for kind in ["multivariate", "multivariate_lookup", "future_type"] {
        let frame =
            format!(r#"{{"type":"{kind}","sid":9,"seq":10,"sending_ts_ms":11,"msg":{{"x":1}}}}"#);
        let owned = WsMessageV2::from_bytes(frame.as_bytes()).unwrap();
        assert!(
            matches!(&owned, WsMessageV2::Unknown { msg_type, .. } if msg_type.as_str() == kind)
        );
        let borrowed = WsMessageRef::from_bytes(frame.as_bytes()).unwrap();
        assert!(
            matches!(&borrowed, WsMessageRef::Unknown { msg_type, .. } if msg_type.as_str() == kind)
        );
        assert_eq!(borrowed.sending_ts_ms(), Some(11));
        assert_eq!(borrowed.into_owned().unwrap().sequence(), Some(10));
    }
}

#[test]
fn feed_and_batch_strings_borrow_the_original_wire_buffer() {
    use kalshi_fast::{WsCommunicationsRef, WsDataMessageRef};

    let feed = br#"{"type":"pyth_value","sid":1,"seq":2,"sending_ts_ms":3,"msg":{"underlying_ticker":"Metal.XAU/USD","value_usd":"82.1","source_ts_ms":100,"received_at":101}}"#;
    let message = WsMessageRef::from_bytes(feed).unwrap();
    match &message {
        WsMessageRef::Data(WsDataMessageRef::PythValue { msg, .. }) => {
            assert_borrowed_from(feed, &msg.underlying_ticker);
            assert_borrowed_from(feed, &msg.value_usd);
        }
        other => panic!("expected Pyth value, got {other:?}"),
    }
    assert_eq!(message.sending_ts_ms(), Some(3));

    let batch = br#"{"type":"rfq_batch_deleted","sid":1,"seq":3,"sending_ts_ms":4,"msg":{"id":"batch","creator_id":"creator","rfq_ids":["rfq"],"deleted_ts":"2024-12-01T10:05:00Z"}}"#;
    match WsMessageRef::from_bytes(batch).unwrap() {
        WsMessageRef::Data(WsDataMessageRef::Communications {
            msg: WsCommunicationsRef::RfqBatchDeleted(msg),
            ..
        }) => {
            assert_borrowed_from(batch, &msg.id);
            assert_borrowed_from(batch, &msg.rfq_ids[0]);
        }
        other => panic!("expected RFQ batch deletion, got {other:?}"),
    }
}

#[test]
fn malformed_published_events_are_errors_not_unknown_messages() {
    for frame in [
        br#"{"type":"pyth_value","sid":1,"msg":{}}"#.as_slice(),
        br#"{"type":"rfq_batch_deleted","sid":1,"msg":{}}"#.as_slice(),
    ] {
        assert!(WsMessageV2::from_bytes(frame).is_err());
        assert!(WsMessageRef::from_bytes(frame).is_err());
    }
}

#[test]
fn escaped_control_filter_strings_remain_decodable() {
    let frame = br#"{"type":"ok","msg":{"underlying_tickers":["Metal.\u0058AU/USD"]}}"#;
    match WsMessageRef::from_bytes(frame).unwrap() {
        WsMessageRef::Ok { msg: Some(msg), .. } => {
            let value = &msg.underlying_tickers.unwrap()[0];
            assert!(matches!(value, std::borrow::Cow::Owned(_)));
            assert_eq!(value, "Metal.XAU/USD");
        }
        other => panic!("expected filters, got {other:?}"),
    }
}

#[test]
fn ebattle_scores_payloads_and_match_ack_decode() {
    use kalshi_fast::{WsDataMessageRef, WsDataMessageV2};

    let frame = r#"{"type":"ebattle_scores","sid":3,"seq":4,"sending_ts_ms":9,"msg":{"event_id":"E","match_id":"ms-1","received_at":1700000000000,"data":"{\"game\":\"esoccer\",\"status\":\"live\",\"home_periods\":{\"period_1\":1},\"live_data_delayed\":false}"}}"#;
    match WsMessageV2::from_bytes(frame.as_bytes()).unwrap() {
        WsMessageV2::Data(WsDataMessageV2::EbattleScores { msg, .. }) => {
            assert_eq!(msg.match_id, "ms-1");
            assert_eq!(msg.received_at, 1_700_000_000_000);
            let data: Value = msg.parse_data().unwrap();
            assert_eq!(data["game"], "esoccer");
            assert_eq!(data["home_periods"]["period_1"], 1);
        }
        other => panic!("expected ebattle_scores, got {other:?}"),
    }
    match WsMessageRef::from_bytes(frame.as_bytes()).unwrap() {
        WsMessageRef::Data(WsDataMessageRef::EbattleScores { msg, .. }) => {
            assert_borrowed_from(frame.as_bytes(), &msg.match_id);
            let data: Value = msg.parse_data().unwrap();
            assert_eq!(data["live_data_delayed"], false);
        }
        other => panic!("expected borrowed ebattle_scores, got {other:?}"),
    }

    // Required msg fields are enforced.
    let missing = r#"{"type":"ebattle_scores","sid":3,"seq":4,"msg":{"event_id":"E","match_id":"ms-1","data":"{}"}}"#;
    assert!(WsMessageV2::from_bytes(missing.as_bytes()).is_err());

    let ack = r#"{"type":"ok","id":2,"sid":1,"seq":2,"msg":{"match_ids":["all"]}}"#;
    for message in [
        WsMessageV2::from_bytes(ack.as_bytes()).unwrap(),
        WsMessageRef::from_bytes(ack.as_bytes())
            .unwrap()
            .into_owned()
            .unwrap(),
    ] {
        match message {
            WsMessageV2::Ok { msg: Some(msg), .. } => {
                assert_eq!(msg.match_ids, Some(vec!["all".to_string()]));
            }
            other => panic!("expected match ack, got {other:?}"),
        }
    }
}
