//! Domain events for event sourcing and WebSocket broadcasting.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::notes::NotePatch;
use crate::types::RunState;

/// Workspace event for WebSocket broadcasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkspaceEvent {
    NoteCreated {
        note_id: Uuid,
        workspace_id: Uuid,
        title: String,
    },
    NotePatched {
        note_id: Uuid,
        workspace_id: Uuid,
        patch: NotePatch,
        new_version: u64,
    },
    NoteDeleted {
        note_id: Uuid,
        workspace_id: Uuid,
    },
    NoteTitleChanged {
        note_id: Uuid,
        workspace_id: Uuid,
        old_title: String,
        new_title: String,
    },
    AutomationRunQueued {
        run_id: Uuid,
        rule_id: Uuid,
        workspace_id: Uuid,
    },
    AutomationRunRunning {
        run_id: Uuid,
        rule_id: Uuid,
        workspace_id: Uuid,
    },
    AutomationRunSucceeded {
        run_id: Uuid,
        rule_id: Uuid,
        workspace_id: Uuid,
        operations_count: usize,
    },
    AutomationRunFailed {
        run_id: Uuid,
        rule_id: Uuid,
        workspace_id: Uuid,
        error: String,
    },
    AutomationRunReviewed {
        run_id: Uuid,
        rule_id: Uuid,
        workspace_id: Uuid,
        accepted: usize,
        rejected: usize,
    },
    OperationPreview {
        run_id: Uuid,
        operation_id: Uuid,
        workspace_id: Uuid,
        op_type: String,
        diff: String,
    },
    OperationApplied {
        run_id: Uuid,
        operation_id: Uuid,
        workspace_id: Uuid,
        note_id: Uuid,
    },
    OperationRejected {
        run_id: Uuid,
        operation_id: Uuid,
        workspace_id: Uuid,
        reason: String,
    },
}

impl WorkspaceEvent {
    /// Get the workspace ID for this event.
    pub fn workspace_id(&self) -> Uuid {
        match self {
            Self::NoteCreated { workspace_id, .. } => *workspace_id,
            Self::NotePatched { workspace_id, .. } => *workspace_id,
            Self::NoteDeleted { workspace_id, .. } => *workspace_id,
            Self::NoteTitleChanged { workspace_id, .. } => *workspace_id,
            Self::AutomationRunQueued { workspace_id, .. } => *workspace_id,
            Self::AutomationRunRunning { workspace_id, .. } => *workspace_id,
            Self::AutomationRunSucceeded { workspace_id, .. } => *workspace_id,
            Self::AutomationRunFailed { workspace_id, .. } => *workspace_id,
            Self::AutomationRunReviewed { workspace_id, .. } => *workspace_id,
            Self::OperationPreview { workspace_id, .. } => *workspace_id,
            Self::OperationApplied { workspace_id, .. } => *workspace_id,
            Self::OperationRejected { workspace_id, .. } => *workspace_id,
        }
    }

    /// Get event code for client compatibility.
    pub fn event_code(&self) -> &'static str {
        match self {
            Self::NoteCreated { .. } => "note_created",
            Self::NotePatched { .. } => "note_patched",
            Self::NoteDeleted { .. } => "note_deleted",
            Self::NoteTitleChanged { .. } => "note_title_changed",
            Self::AutomationRunQueued { .. } => "automation_run_queued",
            Self::AutomationRunRunning { .. } => "automation_run_running",
            Self::AutomationRunSucceeded { .. } => "automation_run_succeeded",
            Self::AutomationRunFailed { .. } => "automation_run_failed",
            Self::AutomationRunReviewed { .. } => "automation_run_reviewed",
            Self::OperationPreview { .. } => "operation_preview",
            Self::OperationApplied { .. } => "operation_applied",
            Self::OperationRejected { .. } => "operation_rejected",
        }
    }
}

/// Event envelope for WebSocket messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub event: WorkspaceEvent,
    pub timestamp: OffsetDateTime,
}

impl EventEnvelope {
    pub fn new(event: WorkspaceEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            event,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}

/// WebSocket message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    SubscribeNote { note_id: Uuid },
    SubscribeWorkspace { workspace_id: Uuid },
    UnsubscribeNote { note_id: Uuid },
    UnsubscribeWorkspace { workspace_id: Uuid },
    ApplyPatch { patch: NotePatch },
    Ack { cursor: Uuid },
    Event { envelope: EventEnvelope },
    Error { code: String, message: String },
}

/// WebSocket error codes.
pub const ERROR_STALE_CURSOR: &str = "STALE_CURSOR";
pub const ERROR_VERSION_CONFLICT: &str = "VERSION_CONFLICT";
pub const ERROR_UNAUTHORIZED: &str = "UNAUTHORIZED";
pub const ERROR_NOT_FOUND: &str = "NOT_FOUND";
