//! Core domain types for kjxlkj.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Global user role in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalRole {
    Owner,
    Admin,
    Editor,
    Viewer,
}

impl Default for GlobalRole {
    fn default() -> Self {
        Self::Viewer
    }
}

/// Workspace-specific role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRole {
    Admin,
    Editor,
    Viewer,
}

impl Default for WorkspaceRole {
    fn default() -> Self {
        Self::Viewer
    }
}

/// Note kind enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

impl Default for NoteKind {
    fn default() -> Self {
        Self::Markdown
    }
}

/// Access scope for notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    Private,
    Workspace,
    Project,
}

impl Default for AccessScope {
    fn default() -> Self {
        Self::Workspace
    }
}

/// Note lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteState {
    Active,
    SoftDeleted,
}

impl Default for NoteState {
    fn default() -> Self {
        Self::Active
    }
}

/// Automation rule state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleState {
    Enabled,
    Disabled,
}

impl Default for RuleState {
    fn default() -> Self {
        Self::Enabled
    }
}

/// Automation run state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl Default for RunState {
    fn default() -> Self {
        Self::Queued
    }
}

/// Provider mode for librarian automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderMode {
    OpenRouter,
    LmStudio,
}

impl Default for ProviderMode {
    fn default() -> Self {
        Self::OpenRouter
    }
}

/// Timestamped entity base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamped {
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Entity identifier wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityId<T> {
    pub value: Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> EntityId<T> {
    pub fn new(value: Uuid) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
}

impl<T> From<Uuid> for EntityId<T> {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl<T> From<EntityId<T>> for Uuid {
    fn from(id: EntityId<T>) -> Self {
        id.value
    }
}

/// Version counter for optimistic concurrency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version(pub u64);

impl Default for Version {
    fn default() -> Self {
        Self(1)
    }
}

impl Version {
    pub fn increment(&self) -> Self {
        Version(self.0 + 1)
    }
}
