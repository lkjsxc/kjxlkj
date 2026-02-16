use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(FromRow, serde::Serialize)]
pub struct NoteStreamRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub current_version: i64,
    pub is_deleted: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(FromRow, serde::Serialize)]
pub struct NoteProjectionRow {
    pub note_id: Uuid,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
    pub updated_at: OffsetDateTime,
}

/// Create a note stream and its initial projection.
pub async fn create_note(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    title: &str,
    note_kind: &str,
    markdown: &str,
) -> Result<NoteStreamRow, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO note_streams (id, workspace_id, project_id, title, note_kind)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(workspace_id)
    .bind(project_id)
    .bind(title)
    .bind(note_kind)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO note_projections (note_id, version, markdown)
         VALUES ($1, 0, $2)"
    )
    .bind(id)
    .bind(markdown)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(NoteStreamRow {
        id,
        workspace_id,
        project_id,
        title: title.to_string(),
        note_kind: note_kind.to_string(),
        access_scope: "workspace".to_string(),
        current_version: 0,
        is_deleted: false,
        created_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    })
}

/// List notes in a workspace (excluding deleted).
pub async fn list_notes(
    pool: &PgPool,
    workspace_id: Uuid,
    project_id: Option<Uuid>,
) -> Result<Vec<NoteStreamRow>, sqlx::Error> {
    if let Some(pid) = project_id {
        sqlx::query_as::<_, NoteStreamRow>(
            "SELECT id, workspace_id, project_id, title, note_kind,
                    access_scope, current_version, is_deleted, created_at, updated_at
             FROM note_streams
             WHERE workspace_id = $1 AND project_id = $2 AND NOT is_deleted
             ORDER BY updated_at DESC"
        )
        .bind(workspace_id)
        .bind(pid)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, NoteStreamRow>(
            "SELECT id, workspace_id, project_id, title, note_kind,
                    access_scope, current_version, is_deleted, created_at, updated_at
             FROM note_streams
             WHERE workspace_id = $1 AND NOT is_deleted
             ORDER BY updated_at DESC"
        )
        .bind(workspace_id)
        .fetch_all(pool)
        .await
    }
}

/// Get note stream by id.
pub async fn get_note(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Option<NoteStreamRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteStreamRow>(
        "SELECT id, workspace_id, project_id, title, note_kind,
                access_scope, current_version, is_deleted, created_at, updated_at
         FROM note_streams WHERE id = $1"
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await
}

/// Get note projection.
pub async fn get_projection(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Option<NoteProjectionRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteProjectionRow>(
        "SELECT note_id, version, markdown, metadata_json, updated_at
         FROM note_projections WHERE note_id = $1"
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await
}

/// Patch note body with optimistic version check.
/// Returns new version on success.
pub async fn patch_note(
    pool: &PgPool,
    note_id: Uuid,
    base_version: i64,
    markdown: &str,
    actor_id: Uuid,
    actor_type: &str,
) -> Result<i64, PatchError> {
    let mut tx = pool.begin().await.map_err(PatchError::Db)?;

    // Lock and check version
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT current_version FROM note_streams
         WHERE id = $1 FOR UPDATE"
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    let current = match row {
        Some((v,)) => v,
        None => return Err(PatchError::NotFound),
    };

    if current != base_version {
        return Err(PatchError::Conflict {
            expected: base_version,
            actual: current,
        });
    }

    let new_version = current + 1;
    let event_id = Uuid::now_v7();

    // Append event
    sqlx::query(
        "INSERT INTO note_events (id, note_id, seq, event_type, payload, actor_id, actor_type)
         VALUES ($1, $2, $3, 'body_update', $4, $5, $6)"
    )
    .bind(event_id)
    .bind(note_id)
    .bind(new_version)
    .bind(serde_json::json!({"markdown": markdown}))
    .bind(actor_id)
    .bind(actor_type)
    .execute(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    // Update stream version
    sqlx::query(
        "UPDATE note_streams SET current_version = $1, updated_at = now()
         WHERE id = $2"
    )
    .bind(new_version)
    .bind(note_id)
    .execute(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    // Update projection
    sqlx::query(
        "UPDATE note_projections
         SET version = $1, markdown = $2, updated_at = now()
         WHERE note_id = $3"
    )
    .bind(new_version)
    .bind(markdown)
    .bind(note_id)
    .execute(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    tx.commit().await.map_err(PatchError::Db)?;
    Ok(new_version)
}

/// Update note title with optimistic version check.
pub async fn update_title(
    pool: &PgPool,
    note_id: Uuid,
    base_version: i64,
    title: &str,
    actor_id: Uuid,
    actor_type: &str,
) -> Result<i64, PatchError> {
    let mut tx = pool.begin().await.map_err(PatchError::Db)?;

    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT current_version FROM note_streams
         WHERE id = $1 FOR UPDATE"
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    let current = match row {
        Some((v,)) => v,
        None => return Err(PatchError::NotFound),
    };

    if current != base_version {
        return Err(PatchError::Conflict {
            expected: base_version,
            actual: current,
        });
    }

    let new_version = current + 1;
    let event_id = Uuid::now_v7();

    sqlx::query(
        "INSERT INTO note_events (id, note_id, seq, event_type, payload, actor_id, actor_type)
         VALUES ($1, $2, $3, 'title_update', $4, $5, $6)"
    )
    .bind(event_id)
    .bind(note_id)
    .bind(new_version)
    .bind(serde_json::json!({"title": title}))
    .bind(actor_id)
    .bind(actor_type)
    .execute(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    sqlx::query(
        "UPDATE note_streams SET current_version = $1, title = $2, updated_at = now()
         WHERE id = $3"
    )
    .bind(new_version)
    .bind(title)
    .bind(note_id)
    .execute(&mut *tx)
    .await
    .map_err(PatchError::Db)?;

    tx.commit().await.map_err(PatchError::Db)?;
    Ok(new_version)
}

/// Soft-delete a note.
pub async fn soft_delete(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE note_streams SET is_deleted = true, updated_at = now()
         WHERE id = $1"
    )
    .bind(note_id)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug)]
pub enum PatchError {
    NotFound,
    Conflict { expected: i64, actual: i64 },
    Db(sqlx::Error),
}
