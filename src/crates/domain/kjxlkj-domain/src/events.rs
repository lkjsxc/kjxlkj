// Event types per /docs/spec/domain/events.md
use serde::{Deserialize, Serialize};

/// Event types for note streams
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteEventType {
    Created,
    Updated,
    TitleChanged,
    Deleted,
    Restored,
    MetadataSet,
    MetadataDeleted,
    TagsReplaced,
    RolledBack,
}

/// Event types for workspace streams
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceEventType {
    ProjectCreated,
    ProjectUpdated,
    ProjectDeleted,
    MemberAdded,
    MemberRoleChanged,
    MemberRemoved,
    ViewCreated,
    ViewUpdated,
    ViewDeleted,
    AutomationRuleCreated,
    AutomationRunStarted,
    AutomationRunCompleted,
}
