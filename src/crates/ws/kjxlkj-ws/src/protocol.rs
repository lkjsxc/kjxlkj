// WebSocket protocol types per /docs/spec/api/websocket.md
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Client-to-server messages
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    SubscribeNote { note_id: Uuid },
    UnsubscribeNote { note_id: Uuid },
    SubscribeWorkspace { workspace_id: Uuid },
    UnsubscribeWorkspace { workspace_id: Uuid },
    ApplyPatch {
        note_id: Uuid,
        base_version: i64,
        patch_ops: Vec<serde_json::Value>,
        idempotency_key: String,
        client_ts: String,
    },
    Ack { stream_id: String, event_seq: i64 },
    PresencePing {
        workspace_id: Uuid,
        note_id: Option<Uuid>,
        cursor: Option<serde_json::Value>,
    },
}

/// Server-to-client messages
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Subscribed {
        stream_id: String,
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
    PresenceEvent {
        workspace_id: Uuid,
        note_id: Option<Uuid>,
        user_id: Uuid,
        state: String,
        server_ts: String,
    },
    AutomationEvent {
        workspace_id: Uuid,
        run_id: Uuid,
        status: String,
        event_seq: i64,
        event_type: String,
        payload: serde_json::Value,
    },
    Heartbeat { server_ts: String },
    Error {
        code: String,
        message: String,
        request_id: String,
    },
}
