//! Note aggregate root and related entities
//! 
//! Implements the note lifecycle with ID/title separation,
//! optimistic concurrency, and event sourcing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Immutable note identity (UUID v4)
/// 
/// This is stable throughout the note's lifecycle and never changes.
pub type NoteId = Uuid;

/// Note kind taxonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    /// Standard note (default)
    Note,
    /// Template for creating other notes
    Template,
    /// Summary or aggregated content
    Summary,
    /// Meeting notes
    Meeting,
    /// Documentation
    Doc,
}

impl Default for NoteKind {
    fn default() -> Self {
        Self::Note
    }
}

/// Access scope for note visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    /// Only visible to owner
    Private,
    /// Visible to workspace members
    Workspace,
    /// Publicly accessible
    Public,
}

impl Default for AccessScope {
    fn default() -> Self {
        Self::Workspace
    }
}

/// Note stream - the aggregate root
/// 
/// Contains the immutable identity and mutable content.
/// Version-based optimistic concurrency control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteStream {
    /// Immutable unique identifier
    pub note_id: NoteId,
    /// Mutable display title
    pub title: String,
    /// Markdown body content
    pub markdown: String,
    /// Owning workspace
    pub workspace_id: Uuid,
    /// Optional project scoping
    pub project_id: Option<Uuid>,
    /// Note type taxonomy
    pub note_kind: NoteKind,
    /// Visibility scope
    pub access_scope: AccessScope,
    /// Optimistic concurrency version
    pub version: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Soft-delete marker (null = active)
    pub deleted_at: Option<DateTime<Utc>>,
}

impl NoteStream {
    /// Create a new note with explicit title
    pub fn new(
        title: String,
        markdown: String,
        workspace_id: Uuid,
        project_id: Option<Uuid>,
        note_kind: NoteKind,
    ) -> Self {
        let now = Utc::now();
        Self {
            note_id: Uuid::new_v4(),
            title,
            markdown,
            workspace_id,
            project_id,
            note_kind,
            access_scope: AccessScope::default(),
            version: 1,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// Create a new note with datetime title (default)
    pub fn new_with_datetime_title(
        markdown: String,
        workspace_id: Uuid,
        project_id: Option<Uuid>,
        note_kind: NoteKind,
    ) -> Self {
        let title = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self::new(title, markdown, workspace_id, project_id, note_kind)
    }

    /// Update title (mutable)
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// Update markdown content with version check
    pub fn update_markdown(
        &mut self,
        markdown: String,
        expected_version: u64,
    ) -> Result<(), ConcurrencyError> {
        if self.version != expected_version {
            return Err(ConcurrencyError {
                expected_version,
                current_version: self.version,
            });
        }
        self.markdown = markdown;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Soft delete
    pub fn delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// Undelete (restore)
    pub fn undelete(&mut self) {
        self.deleted_at = None;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// Check if note is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.deleted_at.is_none()
    }
}

/// Concurrency error for optimistic locking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyError {
    pub expected_version: u64,
    pub current_version: u64,
}

/// Note projection for list/search views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjection {
    pub note_id: NoteId,
    pub title: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    pub version: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// First ~200 chars for preview
    pub snippet: Option<String>,
    /// Backlink count
    pub backlink_count: usize,
}

/// Wiki-link reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiLink {
    /// Source note ID
    pub source_note_id: NoteId,
    /// Target note ID (resolved)
    pub target_note_id: NoteId,
    /// Link text (alias or title)
    pub link_text: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}
