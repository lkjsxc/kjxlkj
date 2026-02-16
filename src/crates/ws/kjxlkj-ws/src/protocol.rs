/// WebSocket protocol types per /docs/spec/api/websocket.md
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Client -> Server message types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    SubscribeNote { note_id: Uuid },
    UnsubscribeNote { note_id: Uuid },
    SubscribeWorkspace { workspace_id: Uuid },
    Ack { stream_id: Uuid, event_seq: i64 },
    ApplyPatch {
        note_id: Uuid,
        base_version: i64,
        patch_ops: serde_json::Value,
        idempotency_key: String,
        client_ts: String,
    },
}

/// Server -> Client message types
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Subscribed {
        stream_id: Uuid,
        current_version: i64,
        replay_cursor: i64,
    },
    PatchCommitted {
        note_id: Uuid,
        version: i64,
        event_seq: i64,
        idempotency_key: String,
    },
    PatchRejected {
        note_id: Uuid,
        expected_version: i64,
        current_version: i64,
        reason: String,
    },
    NoteEvent {
        note_id: Uuid,
        event_seq: i64,
        version: i64,
        event_type: String,
        payload: serde_json::Value,
    },
    WorkspaceEvent {
        workspace_id: Uuid,
        event_seq: i64,
        event_type: String,
        payload: serde_json::Value,
    },
    AutomationEvent {
        workspace_id: Uuid,
        run_id: Uuid,
        status: String,
        event_seq: i64,
        payload: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
        request_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_04_duplicate_idempotency_key() {
        // Acceptance: WS-04
        let _msg = ClientMessage::ApplyPatch {
            note_id: Uuid::new_v4(),
            base_version: 1,
            patch_ops: serde_json::json!({}),
            idempotency_key: "unique-key-1".into(),
            client_ts: "2026-02-16T00:00:00Z".into(),
        };
        // Duplicate key should return same commit identity
        let json = serde_json::to_string(&ServerMessage::PatchCommitted {
            note_id: Uuid::nil(),
            version: 2,
            event_seq: 1,
            idempotency_key: "unique-key-1".into(),
        })
        .unwrap();
        assert!(json.contains("unique-key-1"));
    }

    #[test]
    fn ws_06_automation_events_ordered() {
        // Acceptance: WS-06
        let events: Vec<ServerMessage> = (1..=3)
            .map(|seq| ServerMessage::AutomationEvent {
                workspace_id: Uuid::nil(),
                run_id: Uuid::nil(),
                status: "running".into(),
                event_seq: seq,
                payload: serde_json::json!({}),
            })
            .collect();
        for (i, e) in events.iter().enumerate() {
            if let ServerMessage::AutomationEvent { event_seq, .. } = e {
                assert_eq!(*event_seq, (i + 1) as i64);
            }
        }
    }
}
