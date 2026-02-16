use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;
use crate::note_kind::NoteKind;

/// Core note stream entity per docs/spec/domain/notes.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteStream {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: NoteKind,
    pub access_scope: String,
    pub current_version: i64,
    pub is_deleted: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Full note projection including body and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjection {
    pub note_id: Uuid,
    pub title: String,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
}

/// Request to create a new note.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<NoteKind>,
    pub markdown: Option<String>,
}

/// Request to patch a note body.
#[derive(Debug, Clone, Deserialize)]
pub struct PatchNoteRequest {
    pub base_version: i64,
    pub markdown: Option<String>,
}

/// Request to update note title.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTitleRequest {
    pub base_version: i64,
    pub title: String,
}

/// Default title rule: assign current datetime as title.
/// Format: YYYY-MM-DD HH:mm:ss in server local timezone.
pub fn default_note_title() -> String {
    let now = OffsetDateTime::now_utc();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_note_title_format() {
        let title = default_note_title();
        // Format: YYYY-MM-DD HH:mm:ss
        assert_eq!(title.len(), 19);
        assert_eq!(&title[4..5], "-");
        assert_eq!(&title[7..8], "-");
        assert_eq!(&title[10..11], " ");
        assert_eq!(&title[13..14], ":");
        assert_eq!(&title[16..17], ":");
    }
}
