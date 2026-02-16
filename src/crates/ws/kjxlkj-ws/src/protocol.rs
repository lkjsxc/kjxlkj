use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Client-to-server message types per docs/spec/api/websocket.md.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    SubscribeNote { note_id: Uuid },
    UnsubscribeNote { note_id: Uuid },
    SubscribeWorkspace { workspace_id: Uuid },
    Ack { stream_id: String, event_seq: i64 },
    ApplyPatch {
        note_id: Uuid,
        base_version: i64,
        patch_ops: serde_json::Value,
        idempotency_key: String,
        client_ts: String,
    },
}

/// Server-to-client message types.
#[derive(Debug, Clone, Serialize)]
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
