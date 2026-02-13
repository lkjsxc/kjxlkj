use kjxlkj_domain::types::{AccessScope, NoteKind};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct NoteRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    pub version: i64,
    pub is_deleted: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct ProjectRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub archived: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
