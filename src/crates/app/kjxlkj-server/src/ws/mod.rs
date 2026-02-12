use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::patch::PatchOp;

pub mod session;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerEvent {
    #[serde(rename = "note_event")]
    NoteEvent {
        note_id: Uuid,
        event_seq: i64,
        version: i64,
        event_type: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "workspace_event")]
    WorkspaceEvent {
        workspace_id: Uuid,
        event_seq: i64,
        event_type: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "presence_event")]
    PresenceEvent {
        workspace_id: Uuid,
        note_id: Uuid,
        user_id: Uuid,
        state: String,
        server_ts: String,
    },
    #[serde(rename = "automation_event")]
    AutomationEvent {
        workspace_id: Uuid,
        run_id: Uuid,
        status: String,
        payload: serde_json::Value,
    },
}

impl ServerEvent {
    pub fn note_event(
        note_id: Uuid,
        seq: i64,
        version: i64,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Self {
        Self::NoteEvent {
            note_id,
            event_seq: seq,
            version,
            event_type: event_type.to_string(),
            payload,
        }
    }

    pub fn workspace_event(
        workspace_id: Uuid,
        seq: i64,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Self {
        Self::WorkspaceEvent {
            workspace_id,
            event_seq: seq,
            event_type: event_type.to_string(),
            payload,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "subscribe_note")]
    SubscribeNote { note_id: Uuid },
    #[serde(rename = "unsubscribe_note")]
    UnsubscribeNote { note_id: Uuid },
    #[serde(rename = "subscribe_workspace")]
    SubscribeWorkspace { workspace_id: Uuid },
    #[serde(rename = "unsubscribe_workspace")]
    UnsubscribeWorkspace { workspace_id: Uuid },
    #[serde(rename = "apply_patch")]
    ApplyPatch {
        note_id: Uuid,
        base_version: i64,
        patch_ops: Vec<PatchOp>,
        idempotency_key: String,
        client_ts: Option<String>,
    },
    #[serde(rename = "ack")]
    Ack { stream_id: String, event_seq: i64 },
    #[serde(rename = "presence_ping")]
    PresencePing {
        workspace_id: Uuid,
        note_id: Uuid,
        cursor: Option<i64>,
    },
}
