use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::patch::PatchOp;

pub mod session;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvent {
    #[serde(rename = "type")]
    pub kind: String,
    pub note_id: Uuid,
    pub event_seq: i64,
    pub version: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
}

impl ServerEvent {
    pub fn note_event(
        note_id: Uuid,
        seq: i64,
        version: i64,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            kind: "note_event".to_string(),
            note_id,
            event_seq: seq,
            version,
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
    #[serde(rename = "apply_patch")]
    ApplyPatch {
        note_id: Uuid,
        base_version: i64,
        patch_ops: Vec<PatchOp>,
        idempotency_key: String,
        client_ts: Option<String>,
    },
    #[serde(rename = "ack")]
    Ack { note_id: Uuid, event_seq: i64 },
}
