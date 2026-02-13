//! WebSocket handling for kjxlkj.
//!
//! This crate contains WebSocket protocol handling for realtime sync.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use kjxlkj_domain::{WsMessage, EventEnvelope, WorkspaceEvent, NotePatch, Version};

/// WebSocket session state.
#[derive(Debug, Clone)]
pub struct SessionState {
    pub user_id: Uuid,
    pub workspace_subscriptions: Vec<Uuid>,
    pub note_subscriptions: Vec<Uuid>,
    pub last_cursor: Option<Uuid>,
}

impl SessionState {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            workspace_subscriptions: Vec::new(),
            note_subscriptions: Vec::new(),
            last_cursor: None,
        }
    }

    pub fn subscribe_workspace(&mut self, workspace_id: Uuid) {
        if !self.workspace_subscriptions.contains(&workspace_id) {
            self.workspace_subscriptions.push(workspace_id);
        }
    }

    pub fn unsubscribe_workspace(&mut self, workspace_id: Uuid) {
        self.workspace_subscriptions.retain(|id| *id != workspace_id);
    }

    pub fn subscribe_note(&mut self, note_id: Uuid) {
        if !self.note_subscriptions.contains(&note_id) {
            self.note_subscriptions.push(note_id);
        }
    }

    pub fn unsubscribe_note(&mut self, note_id: Uuid) {
        self.note_subscriptions.retain(|id| *id != note_id);
    }
}

/// WebSocket protocol message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProtocolMessage {
    SubscribeWorkspace { workspace_id: Uuid },
    UnsubscribeWorkspace { workspace_id: Uuid },
    SubscribeNote { note_id: Uuid },
    UnsubscribeNote { note_id: Uuid },
    ApplyPatch {
        note_id: Uuid,
        base_version: u64,
        patch_id: Uuid,
        operations: Vec<PatchOp>,
    },
    Ack { cursor: Uuid },
    Event {
        id: Uuid,
        event: WorkspaceEventPayload,
        timestamp: OffsetDateTime,
    },
    Error { code: String, message: String },
}

/// Patch operation for WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PatchOp {
    BodyReplace { content: String },
    TitleReplace { content: String },
}

/// Workspace event payload for WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkspaceEventPayload {
    NoteCreated { note_id: Uuid, workspace_id: Uuid, title: String },
    NotePatched { note_id: Uuid, workspace_id: Uuid, new_version: u64 },
    NoteDeleted { note_id: Uuid, workspace_id: Uuid },
    AutomationRunQueued { run_id: Uuid, rule_id: Uuid, workspace_id: Uuid },
    AutomationRunRunning { run_id: Uuid, rule_id: Uuid, workspace_id: Uuid },
    AutomationRunSucceeded { run_id: Uuid, rule_id: Uuid, workspace_id: Uuid },
    AutomationRunFailed { run_id: Uuid, rule_id: Uuid, workspace_id: Uuid, error: String },
    OperationPreview { run_id: Uuid, operation_id: Uuid, workspace_id: Uuid, op_type: String },
    OperationApplied { run_id: Uuid, operation_id: Uuid, workspace_id: Uuid, note_id: Uuid },
    OperationRejected { run_id: Uuid, operation_id: Uuid, workspace_id: Uuid, reason: String },
}

/// Error codes for WebSocket.
pub const ERROR_STALE_CURSOR: &str = "STALE_CURSOR";
pub const ERROR_VERSION_CONFLICT: &str = "VERSION_CONFLICT";
pub const ERROR_UNAUTHORIZED: &str = "UNAUTHORIZED";

/// Create a version conflict error.
pub fn version_conflict_error(expected: u64, provided: u64) -> ProtocolMessage {
    ProtocolMessage::Error {
        code: ERROR_VERSION_CONFLICT.to_string(),
        message: format!("version conflict: expected {}, got {}", expected, provided),
    }
}

/// Create a stale cursor error.
pub fn stale_cursor_error() -> ProtocolMessage {
    ProtocolMessage::Error {
        code: ERROR_STALE_CURSOR.to_string(),
        message: "stale cursor provided".to_string(),
    }
}
