/// Shared PostgreSQL row types and helper functions.
///
/// Used by pg_note_repo, pg_user_repo, pg_workspace_repo.
/// Keeps each PG repo module under 200 lines.
use kjxlkj_domain::note::NoteProjection;
use kjxlkj_domain::DomainError;
use uuid::Uuid;

/// Map sqlx::Error to DomainError.
pub fn pg_err(e: sqlx::Error) -> DomainError {
    DomainError::Internal(format!("pg: {}", e))
}

/// Row type for note projection from PG.
#[derive(sqlx::FromRow)]
pub struct PgNoteProjection {
    pub note_id: Uuid,
    pub title: String,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<PgNoteProjection> for NoteProjection {
    fn from(r: PgNoteProjection) -> Self {
        Self {
            note_id: r.note_id,
            title: r.title,
            version: r.version,
            markdown: r.markdown,
            metadata_json: r.metadata_json,
            updated_at: r.updated_at,
        }
    }
}

/// Row type for note stream from PG.
#[derive(sqlx::FromRow)]
pub struct PgNoteStreamRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub state: String,
    pub current_version: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Row type for note event from PG.
#[derive(sqlx::FromRow)]
pub struct PgNoteEventRow {
    pub id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub actor_type: String,
    pub actor_id: Uuid,
    pub payload: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
}
