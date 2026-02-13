//! Note repository.

use sqlx::SqlitePool;
use uuid::Uuid;
use time::OffsetDateTime;
use serde_json::Value as JsonValue;

use kjxlkj_domain::{
    Note, NoteState, NoteKind, AccessScope, Version,
    NoteHistoryEvent, NoteEventType, NoteMetadata, NoteTag, Backlink,
};

/// Note repository.
pub struct NoteRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NoteRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new note.
    pub async fn create(&self, note: &Note) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO notes (id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(note.id.to_string())
        .bind(note.workspace_id.to_string())
        .bind(note.project_id.map(|p| p.to_string()))
        .bind(&note.title)
        .bind(&note.body)
        .bind(serde_json::to_string(&note.note_kind).unwrap())
        .bind(serde_json::to_string(&note.access_scope).unwrap())
        .bind(serde_json::to_string(&note.state).unwrap())
        .bind(note.version.0 as i64)
        .bind(note.created_at)
        .bind(note.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find note by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Note>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, Option<String>, String, String, String, String, String, i64, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at
            FROM notes WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at)| Note {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
            project_id: project_id.and_then(|p| Uuid::parse_str(&p).ok()),
            title,
            body,
            note_kind: serde_json::from_str(&note_kind).unwrap_or_default(),
            access_scope: serde_json::from_str(&access_scope).unwrap_or_default(),
            state: serde_json::from_str(&state).unwrap_or_default(),
            version: Version(version as u64),
            created_at,
            updated_at,
        }))
    }

    /// List notes for workspace.
    pub async fn list_by_workspace(&self, workspace_id: Uuid, include_deleted: bool) -> Result<Vec<Note>, sqlx::Error> {
        let rows = if include_deleted {
            sqlx::query_as::<_, (String, String, Option<String>, String, String, String, String, String, i64, OffsetDateTime, OffsetDateTime)>(
                r#"
                SELECT id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at
                FROM notes WHERE workspace_id = ? ORDER BY updated_at DESC
                "#,
            )
            .bind(workspace_id.to_string())
            .fetch_all(self.pool)
            .await?
        } else {
            sqlx::query_as::<_, (String, String, Option<String>, String, String, String, String, String, i64, OffsetDateTime, OffsetDateTime)>(
                r#"
                SELECT id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at
                FROM notes WHERE workspace_id = ? AND state = 'active' ORDER BY updated_at DESC
                "#,
            )
            .bind(workspace_id.to_string())
            .fetch_all(self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|(id, workspace_id, project_id, title, body, note_kind, access_scope, state, version, created_at, updated_at)| Note {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                project_id: project_id.and_then(|p| Uuid::parse_str(&p).ok()),
                title,
                body,
                note_kind: serde_json::from_str(&note_kind).unwrap_or_default(),
                access_scope: serde_json::from_str(&access_scope).unwrap_or_default(),
                state: serde_json::from_str(&state).unwrap_or_default(),
                version: Version(version as u64),
                created_at,
                updated_at,
            })
            .collect())
    }

    /// Update note.
    pub async fn update(&self, note: &Note) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE notes SET title = ?, body = ?, version = ?, updated_at = ? WHERE id = ?
            "#,
        )
        .bind(&note.title)
        .bind(&note.body)
        .bind(note.version.0 as i64)
        .bind(note.updated_at)
        .bind(note.id.to_string())
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Soft delete note.
    pub async fn soft_delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE notes SET state = 'soft_deleted', updated_at = CURRENT_TIMESTAMP WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .execute(self.pool)
        .await?;
        Ok(())
    }
}

/// Note history repository.
pub struct NoteHistoryRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NoteHistoryRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create history event.
    pub async fn create(&self, event: &NoteHistoryEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO note_history (id, note_id, event_type, title, body, version, actor_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(event.id.to_string())
        .bind(event.note_id.to_string())
        .bind(serde_json::to_string(&event.event_type).unwrap())
        .bind(&event.title)
        .bind(&event.body)
        .bind(event.version.0 as i64)
        .bind(event.actor_id.to_string())
        .bind(event.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List history for note.
    pub async fn list_by_note(&self, note_id: Uuid) -> Result<Vec<NoteHistoryEvent>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, i64, String, OffsetDateTime)>(
            r#"
            SELECT id, note_id, event_type, title, body, version, actor_id, created_at
            FROM note_history WHERE note_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(note_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, note_id, event_type, title, body, version, actor_id, created_at)| NoteHistoryEvent {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                note_id: Uuid::parse_str(&note_id).unwrap_or_default(),
                event_type: serde_json::from_str(&event_type).unwrap_or(NoteEventType::Created),
                title,
                body,
                version: Version(version as u64),
                actor_id: Uuid::parse_str(&actor_id).unwrap_or_default(),
                created_at,
            })
            .collect())
    }
}

/// Note metadata repository.
pub struct NoteMetadataRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NoteMetadataRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create metadata.
    pub async fn create(&self, metadata: &NoteMetadata) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO note_metadata (note_id, key, value, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(metadata.note_id.to_string())
        .bind(&metadata.key)
        .bind(serde_json::to_string(&metadata.value).unwrap())
        .bind(metadata.created_at)
        .bind(metadata.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List metadata for note.
    pub async fn list_by_note(&self, note_id: Uuid) -> Result<Vec<NoteMetadata>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT note_id, key, value, created_at, updated_at
            FROM note_metadata WHERE note_id = ?
            "#,
        )
        .bind(note_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(note_id, key, value, created_at, updated_at)| NoteMetadata {
                note_id: Uuid::parse_str(&note_id).unwrap_or_default(),
                key,
                value: serde_json::from_str(&value).unwrap_or(JsonValue::Null),
                created_at,
                updated_at,
            })
            .collect())
    }
}

/// Note tag repository.
pub struct NoteTagRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NoteTagRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create tag.
    pub async fn create(&self, tag: &NoteTag) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO note_tags (note_id, tag, created_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(tag.note_id.to_string())
        .bind(&tag.tag)
        .bind(tag.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List tags for note.
    pub async fn list_by_note(&self, note_id: Uuid) -> Result<Vec<NoteTag>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, OffsetDateTime)>(
            r#"
            SELECT note_id, tag, created_at
            FROM note_tags WHERE note_id = ?
            "#,
        )
        .bind(note_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(note_id, tag, created_at)| NoteTag {
                note_id: Uuid::parse_str(&note_id).unwrap_or_default(),
                tag,
                created_at,
            })
            .collect())
    }
}

/// Backlink repository.
pub struct BacklinkRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> BacklinkRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create backlink.
    pub async fn create(&self, backlink: &Backlink) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO backlinks (source_note_id, target_note_id, link_text, created_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(backlink.source_note_id.to_string())
        .bind(backlink.target_note_id.to_string())
        .bind(&backlink.link_text)
        .bind(backlink.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List backlinks for note.
    pub async fn list_by_target(&self, target_note_id: Uuid) -> Result<Vec<Backlink>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, OffsetDateTime)>(
            r#"
            SELECT source_note_id, target_note_id, link_text, created_at
            FROM backlinks WHERE target_note_id = ?
            "#,
        )
        .bind(target_note_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(source_note_id, target_note_id, link_text, created_at)| Backlink {
                source_note_id: Uuid::parse_str(&source_note_id).unwrap_or_default(),
                target_note_id: Uuid::parse_str(&target_note_id).unwrap_or_default(),
                link_text,
                created_at,
            })
            .collect())
    }
}
