use kjxlkj_domain::ids::{EventId, NoteId, UserId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

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

#[derive(FromRow)]
pub struct NoteEventRow {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

pub async fn create_note_stream(
    pool: &PgPool,
    id: NoteId,
    workspace_id: WorkspaceId,
    title: &str,
    note_kind: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_streams (id, workspace_id, title, note_kind)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id.0)
    .bind(workspace_id.0)
    .bind(title)
    .bind(note_kind)
    .execute(pool)
    .await?;
    // Create initial projection
    sqlx::query(
        "INSERT INTO note_projections (note_id, workspace_id, title, note_kind)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id.0)
    .bind(workspace_id.0)
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

pub async fn append_note_event(
    pool: &PgPool,
    event_id: EventId,
    note_id: NoteId,
    seq: i64,
    event_type: &str,
    payload_json: &serde_json::Value,
    actor_id: UserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_events
         (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(event_id.0)
    .bind(note_id.0)
    .bind(seq)
    .bind(event_type)
    .bind(payload_json)
    .bind(actor_id.0)
    .execute(pool)
    .await?;
    Ok(())
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
    // Update stream version and timestamp
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

pub async fn list_note_events(
    pool: &PgPool,
    note_id: NoteId,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT event_id, note_id, seq, event_type,
                payload_json, actor_id, created_at
         FROM note_events WHERE note_id = $1
         ORDER BY seq ASC",
    )
    .bind(note_id.0)
    .fetch_all(pool)
    .await
}
