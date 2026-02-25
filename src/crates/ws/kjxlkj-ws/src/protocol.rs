//! WebSocket message protocol

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// Client acknowledges receipt up to event_seq
    Ack {
        note_id: Uuid,
        event_seq: u64,
        version: u64,
    },
    /// Client applies a patch
    ApplyPatch {
        note_id: Uuid,
        base_version: u64,
        patch_ops: Vec<PatchOp>,
        idempotency_key: String,
        client_ts: DateTime<Utc>,
    },
    /// Server confirms patch committed
    PatchCommitted {
        note_id: Uuid,
        version: u64,
        event_seq: u64,
        idempotency_key: String,
    },
    /// Server rejects patch
    PatchRejected {
        note_id: Uuid,
        expected_version: u64,
        current_version: u64,
        reason: String,
    },
    /// Server broadcasts note event
    NoteEvent {
        note_id: Uuid,
        event_seq: u64,
        version: u64,
        event_type: String,
        payload: serde_json::Value,
    },
    /// Error message
    Error {
        code: String,
        message: String,
    },
    /// Ping for heartbeat
    Ping,
    /// Pong response
    Pong,
}

/// Patch operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOp {
    Retain { count: usize },
    Insert { text: String },
    Delete { count: usize },
}

/// Client message envelope
#[derive(Debug, Clone, Deserialize)]
pub struct ClientMessage {
    #[serde(flatten)]
    pub inner: WsMessage,
}

/// Server message envelope
#[derive(Debug, Clone, Serialize)]
pub struct ServerMessage {
    #[serde(flatten)]
    pub inner: WsMessage,
}
