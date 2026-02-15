use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

// Re-export event types and functions for callers using repo_note::*.
pub use crate::repo_note_event::{
    append_note_event, list_note_events, NoteEventRow,
};
pub use crate::repo_note_snapshot::{
    find_latest_snapshot, list_note_events_from, store_snapshot, SnapshotRow,
};

/// Note stream row per /docs/spec/domain/notes.md.
#[derive(FromRow)]
pub struct NoteStreamRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub current_version: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(FromRow)]
pub struct NoteProjectionRow {
    pub note_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub version: i64,
    pub markdown: String,
    pub rendered_html: String,
    pub metadata_json: serde_json::Value,
}

pub async fn create_note_stream(
    pool: &PgPool,
    id: NoteId,
    workspace_id: WorkspaceId,
    project_id: Option<Uuid>,
    title: &str,
    note_kind: &str,
    access_scope: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_streams
         (id, workspace_id, project_id, title, note_kind, access_scope)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id.0)
    .bind(workspace_id.0)
    .bind(project_id)
    .bind(title)
    .bind(note_kind)
    .bind(access_scope)
    .execute(pool)
    .await?;
    sqlx::query(
        "INSERT INTO note_projections
         (note_id, workspace_id, project_id, title, note_kind)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id.0)
    .bind(workspace_id.0)
    .bind(project_id)
    .bind(title)
    .bind(note_kind)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_note_stream(
    pool: &PgPool,
    id: NoteId,
) -> Result<Option<NoteStreamRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteStreamRow>(
        "SELECT id, workspace_id, project_id, title, note_kind,
                access_scope, current_version, created_at, updated_at,
                deleted_at
         FROM note_streams WHERE id = $1",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn find_note_projection(
    pool: &PgPool,
    id: NoteId,
) -> Result<Option<NoteProjectionRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteProjectionRow>(
        "SELECT note_id, workspace_id, project_id, title, note_kind,
                version, markdown, rendered_html, metadata_json
         FROM note_projections WHERE note_id = $1",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn list_notes(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<NoteStreamRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteStreamRow>(
        "SELECT id, workspace_id, project_id, title, note_kind,
                access_scope, current_version, created_at, updated_at,
                deleted_at
         FROM note_streams
         WHERE workspace_id = $1 AND deleted_at IS NULL
         ORDER BY updated_at DESC",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

pub async fn update_note_projection(
    pool: &PgPool,
    note_id: NoteId,
    title: &str,
    version: i64,
    markdown: &str,
    rendered_html: &str,
    metadata_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE note_projections
         SET title = $2, version = $3, markdown = $4,
             rendered_html = $5, metadata_json = $6,
             search_vector = to_tsvector('english',
                 coalesce($2,'') || ' ' || coalesce($4,''))
         WHERE note_id = $1",
    )
    .bind(note_id.0)
    .bind(title)
    .bind(version)
    .bind(markdown)
    .bind(rendered_html)
    .bind(metadata_json)
    .execute(pool)
    .await?;
    sqlx::query(
        "UPDATE note_streams SET current_version = $2,
         updated_at = now(), title = $3
         WHERE id = $1",
    )
    .bind(note_id.0)
    .bind(version)
    .bind(title)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn soft_delete_note(
    pool: &PgPool,
    id: NoteId,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE note_streams SET deleted_at = now()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id.0)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
