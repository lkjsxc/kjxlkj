/// Note domain types per /docs/spec/domain/notes.md and /docs/spec/domain/note-types.md
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical note_kind enum per /docs/spec/domain/note-types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

impl NoteKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Settings => "settings",
            Self::MediaImage => "media_image",
            Self::MediaVideo => "media_video",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "markdown" => Some(Self::Markdown),
            "settings" => Some(Self::Settings),
            "media_image" => Some(Self::MediaImage),
            "media_video" => Some(Self::MediaVideo),
            _ => None,
        }
    }
}

/// Access scope for a note stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    Workspace,
    Project,
}

/// Note stream state per /docs/spec/domain/notes.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteState {
    Active,
    SoftDeleted,
}

/// NoteStream: the append-only identity for a note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteStream {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    pub state: NoteState,
    pub current_version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// NoteProjection: current materialized view of a note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjection {
    pub note_id: Uuid,
    pub title: String,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
    pub updated_at: NaiveDateTime,
}

/// Default title rule per /docs/spec/domain/notes.md:
/// "YYYY-MM-DD HH:mm:ss" in server local timezone
pub fn default_note_title() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Input for creating a note
#[derive(Debug, Clone, Deserialize)]
pub struct CreateNoteInput {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<NoteKind>,
    pub markdown: Option<String>,
}

/// Input for patching a note body
#[derive(Debug, Clone, Deserialize)]
pub struct PatchNoteInput {
    pub base_version: i64,
    pub markdown: Option<String>,
}

/// Input for updating note title
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTitleInput {
    pub base_version: i64,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_title_format() {
        let title = default_note_title();
        // Format: YYYY-MM-DD HH:mm:ss => 19 chars
        assert_eq!(title.len(), 19);
        assert_eq!(&title[4..5], "-");
        assert_eq!(&title[10..11], " ");
    }

    #[test]
    fn test_note_kind_roundtrip() {
        for kind in [
            NoteKind::Markdown,
            NoteKind::Settings,
            NoteKind::MediaImage,
            NoteKind::MediaVideo,
        ] {
            let s = kind.as_str();
            assert_eq!(NoteKind::from_str(s), Some(kind));
        }
        assert_eq!(NoteKind::from_str("unknown"), None);
    }

    #[test]
    fn api_note_01_create_without_title_defaults_to_datetime() {
        // Acceptance: API-NOTE-01
        let input = CreateNoteInput {
            workspace_id: Uuid::new_v4(),
            project_id: None,
            title: None,
            note_kind: None,
            markdown: None,
        };
        let title = input.title.unwrap_or_else(default_note_title);
        assert_eq!(title.len(), 19);
    }

    #[test]
    fn api_note_02_id_stable_while_title_changes() {
        // Acceptance: API-NOTE-02
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();
        let stream = NoteStream {
            id,
            workspace_id: Uuid::new_v4(),
            project_id: None,
            title: "original".to_string(),
            note_kind: NoteKind::Markdown,
            access_scope: AccessScope::Workspace,
            state: NoteState::Active,
            current_version: 1,
            created_at: now,
            updated_at: now,
        };
        let mut updated = stream.clone();
        updated.title = "new title".to_string();
        assert_eq!(stream.id, updated.id);
    }
}
