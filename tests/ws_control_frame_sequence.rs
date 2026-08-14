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
        }
    ));
    assert!(matches!(
        WsMessageV2::from_bytes(frames[3].as_bytes()).expect("add acknowledgement must parse"),
        WsMessageV2::Ok {
            id: Some(3),
            sid: Some(1),
            seq: Some(386122),
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
            r#"{"type":"error","id":5,"sid":7,"seq":5,"msg":{"code":400,"message":"bad request"}}"#,
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
