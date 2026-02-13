use crate::models::{
    DbBacklink, DbNoteEvent, DbNoteProjection, DbNoteStream, DbSearchResult, DbWorkspaceEvent,
};
use crate::repos::notes_patch::{apply_patch, extract_backlinks, PatchOp};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateNoteInput {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub markdown: String,
}

#[derive(Debug, Clone)]
pub struct NoteMutationResult {
    pub version: i32,
    pub event_seq: i32,
}

#[derive(Debug, Error)]
pub enum NoteMutationError {
    #[error("note not found")]
    NotFound,
    #[error("version conflict")]
    Conflict { current_version: i32 },
    #[error("invalid patch")]
    InvalidPatch,
    #[error("database")]
    Database(#[from] sqlx::Error),
}

pub async fn create_note(
    pool: &PgPool,
    actor_id: Uuid,
    input: CreateNoteInput,
) -> Result<(DbNoteStream, DbNoteProjection), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let note_id = Uuid::now_v7();

    let stream = sqlx::query_as::<_, DbNoteStream>(
        "INSERT INTO note_streams
         (id, workspace_id, project_id, title, note_kind, access_scope, current_version)
         VALUES ($1, $2, $3, $4, $5, $6, 1)
         RETURNING id, workspace_id, project_id, title, note_kind, access_scope,
                   current_version, deleted_at, created_at, updated_at",
    )
    .bind(note_id)
    .bind(input.workspace_id)
    .bind(input.project_id)
    .bind(&input.title)
    .bind(&input.note_kind)
    .bind(&input.access_scope)
    .fetch_one(&mut *tx)
    .await?;

    let projection = sqlx::query_as::<_, DbNoteProjection>(
        "INSERT INTO note_projections
         (note_id, workspace_id, project_id, title, note_kind, version, markdown, rendered_html, metadata_json)
         VALUES ($1, $2, $3, $4, $5, 1, $6, '', '{}'::jsonb)
         RETURNING note_id, workspace_id, project_id, title, note_kind, version,
                   markdown, rendered_html, metadata_json",
    )
    .bind(note_id)
    .bind(input.workspace_id)
    .bind(input.project_id)
    .bind(&input.title)
    .bind(&input.note_kind)
    .bind(&input.markdown)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, 1, 'note_created', $3, $4)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(json!({"title": input.title, "markdown": input.markdown, "version": 1}))
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    refresh_backlinks_tx(&mut tx, note_id, &projection.markdown).await?;
    append_workspace_event_tx(
        &mut tx,
        stream.workspace_id,
        actor_id,
        "note_created",
        json!({"note_id": note_id, "version": 1}),
    )
    .await?;

    tx.commit().await?;
    Ok((stream, projection))
}

pub async fn list_notes(
    pool: &PgPool,
    workspace_id: Uuid,
    include_deleted: bool,
) -> Result<Vec<DbNoteStream>, sqlx::Error> {
    if include_deleted {
        return sqlx::query_as::<_, DbNoteStream>(
            "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                    current_version, deleted_at, created_at, updated_at
             FROM note_streams
             WHERE workspace_id = $1
             ORDER BY updated_at DESC",
        )
        .bind(workspace_id)
        .fetch_all(pool)
        .await;
    }

    sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE workspace_id = $1 AND deleted_at IS NULL
         ORDER BY updated_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn get_note(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Option<(DbNoteStream, DbNoteProjection)>, sqlx::Error> {
    let stream = sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE id = $1",
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await?;

    let Some(stream) = stream else {
        return Ok(None);
    };

    let projection = sqlx::query_as::<_, DbNoteProjection>(
        "SELECT note_id, workspace_id, project_id, title, note_kind, version,
                markdown, rendered_html, metadata_json
         FROM note_projections
         WHERE note_id = $1",
    )
    .bind(note_id)
    .fetch_one(pool)
    .await?;

    Ok(Some((stream, projection)))
}

pub async fn apply_note_patch(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    base_version: i32,
    patch_ops: &[PatchOp],
    idempotency_key: &str,
) -> Result<NoteMutationResult, NoteMutationError> {
    let mut tx = pool.begin().await?;

    if let Some(existing) = sqlx::query_as::<_, (i32, i32)>(
        "SELECT version, event_seq
         FROM note_patch_idempotency
         WHERE note_id = $1 AND idempotency_key = $2",
    )
    .bind(note_id)
    .bind(idempotency_key)
    .fetch_optional(&mut *tx)
    .await?
    {
        tx.commit().await?;
        return Ok(NoteMutationResult {
            version: existing.0,
            event_seq: existing.1,
        });
    }

    let mut stream = sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(NoteMutationError::NotFound)?;

    if stream.deleted_at.is_some() {
        return Err(NoteMutationError::NotFound);
    }

    if stream.current_version != base_version {
        return Err(NoteMutationError::Conflict {
            current_version: stream.current_version,
        });
    }

    let mut projection = sqlx::query_as::<_, DbNoteProjection>(
        "SELECT note_id, workspace_id, project_id, title, note_kind, version,
                markdown, rendered_html, metadata_json
         FROM note_projections
         WHERE note_id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_one(&mut *tx)
    .await?;

    let next_markdown = apply_patch(&projection.markdown, patch_ops)
        .map_err(|_| NoteMutationError::InvalidPatch)?;
    let next_version = stream.current_version + 1;

    sqlx::query(
        "UPDATE note_streams
         SET current_version = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(note_id)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE note_projections
         SET version = $2, markdown = $3
         WHERE note_id = $1",
    )
    .bind(note_id)
    .bind(next_version)
    .bind(&next_markdown)
    .execute(&mut *tx)
    .await?;

    let event_seq = next_version;
    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, 'note_patched', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(event_seq)
    .bind(json!({
        "title": projection.title,
        "markdown": next_markdown,
        "version": next_version,
        "idempotency_key": idempotency_key,
    }))
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO note_patch_idempotency (note_id, idempotency_key, event_seq, version)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(note_id)
    .bind(idempotency_key)
    .bind(event_seq)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    if next_version % 100 == 0 {
        sqlx::query(
            "INSERT INTO note_snapshots (note_id, version, markdown, metadata_json)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(note_id)
        .bind(next_version)
        .bind(&next_markdown)
        .bind(&projection.metadata_json)
        .execute(&mut *tx)
        .await?;
    }

    stream.current_version = next_version;
    projection.markdown = next_markdown;
    projection.version = next_version;

    refresh_backlinks_tx(&mut tx, note_id, &projection.markdown).await?;
    append_workspace_event_tx(
        &mut tx,
        stream.workspace_id,
        actor_id,
        "note_patched",
        json!({"note_id": note_id, "version": next_version, "event_seq": event_seq}),
    )
    .await?;

    tx.commit().await?;

    Ok(NoteMutationResult {
        version: next_version,
        event_seq,
    })
}

pub async fn update_note_title(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    base_version: i32,
    title: &str,
) -> Result<NoteMutationResult, NoteMutationError> {
    let mut tx = pool.begin().await?;

    let stream = sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(NoteMutationError::NotFound)?;

    if stream.deleted_at.is_some() {
        return Err(NoteMutationError::NotFound);
    }

    if stream.current_version != base_version {
        return Err(NoteMutationError::Conflict {
            current_version: stream.current_version,
        });
    }

    let projection = sqlx::query_as::<_, DbNoteProjection>(
        "SELECT note_id, workspace_id, project_id, title, note_kind, version,
                markdown, rendered_html, metadata_json
         FROM note_projections
         WHERE note_id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_one(&mut *tx)
    .await?;

    let next_version = stream.current_version + 1;
    sqlx::query(
        "UPDATE note_streams
         SET title = $2, current_version = $3, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(note_id)
    .bind(title)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE note_projections
         SET title = $2, version = $3
         WHERE note_id = $1",
    )
    .bind(note_id)
    .bind(title)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    let event_seq = next_version;
    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, 'note_title_updated', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(event_seq)
    .bind(json!({
        "title": title,
        "markdown": projection.markdown,
        "version": next_version,
    }))
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    append_workspace_event_tx(
        &mut tx,
        stream.workspace_id,
        actor_id,
        "note_title_updated",
        json!({"note_id": note_id, "version": next_version, "event_seq": event_seq}),
    )
    .await?;

    tx.commit().await?;
    Ok(NoteMutationResult {
        version: next_version,
        event_seq,
    })
}

pub async fn soft_delete_note(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
) -> Result<(), NoteMutationError> {
    let mut tx = pool.begin().await?;

    let stream = sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(NoteMutationError::NotFound)?;

    if stream.deleted_at.is_some() {
        tx.commit().await?;
        return Ok(());
    }

    let next_version = stream.current_version + 1;
    sqlx::query(
        "UPDATE note_streams
         SET deleted_at = NOW(), current_version = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(note_id)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, 'note_deleted', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(next_version)
    .bind(json!({"version": next_version}))
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    append_workspace_event_tx(
        &mut tx,
        stream.workspace_id,
        actor_id,
        "note_deleted",
        json!({"note_id": note_id, "version": next_version}),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn note_history(pool: &PgPool, note_id: Uuid) -> Result<Vec<DbNoteEvent>, sqlx::Error> {
    sqlx::query_as::<_, DbNoteEvent>(
        "SELECT event_id, note_id, seq, event_type, payload_json, actor_id, created_at
         FROM note_events
         WHERE note_id = $1
         ORDER BY seq ASC",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
}

pub async fn note_events_after(
    pool: &PgPool,
    note_id: Uuid,
    after_seq: i32,
) -> Result<Vec<DbNoteEvent>, sqlx::Error> {
    sqlx::query_as::<_, DbNoteEvent>(
        "SELECT event_id, note_id, seq, event_type, payload_json, actor_id, created_at
         FROM note_events
         WHERE note_id = $1 AND seq > $2
         ORDER BY seq ASC",
    )
    .bind(note_id)
    .bind(after_seq)
    .fetch_all(pool)
    .await
}

pub async fn workspace_events_after(
    pool: &PgPool,
    workspace_id: Uuid,
    after_seq: i32,
) -> Result<Vec<DbWorkspaceEvent>, sqlx::Error> {
    sqlx::query_as::<_, DbWorkspaceEvent>(
        "SELECT workspace_id, seq, event_type, payload_json, actor_id, created_at
         FROM workspace_events
         WHERE workspace_id = $1 AND seq > $2
         ORDER BY seq ASC",
    )
    .bind(workspace_id)
    .bind(after_seq)
    .fetch_all(pool)
    .await
}

pub async fn workspace_latest_seq(pool: &PgPool, workspace_id: Uuid) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT COALESCE(MAX(seq), 0) FROM workspace_events WHERE workspace_id = $1")
        .bind(workspace_id)
        .fetch_one(pool)
        .await
}

pub async fn rollback_note(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    target_version: i32,
) -> Result<NoteMutationResult, NoteMutationError> {
    let mut tx = pool.begin().await?;

    let stream = sqlx::query_as::<_, DbNoteStream>(
        "SELECT id, workspace_id, project_id, title, note_kind, access_scope,
                current_version, deleted_at, created_at, updated_at
         FROM note_streams
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(note_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(NoteMutationError::NotFound)?;

    let target_payload = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT payload_json FROM note_events WHERE note_id = $1 AND seq = $2",
    )
    .bind(note_id)
    .bind(target_version)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(NoteMutationError::NotFound)?;

    let title = target_payload
        .get("title")
        .and_then(|value| value.as_str())
        .ok_or(NoteMutationError::InvalidPatch)?
        .to_owned();
    let markdown = target_payload
        .get("markdown")
        .and_then(|value| value.as_str())
        .ok_or(NoteMutationError::InvalidPatch)?
        .to_owned();

    let next_version = stream.current_version + 1;

    sqlx::query(
        "UPDATE note_streams
         SET title = $2, current_version = $3, deleted_at = NULL, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(note_id)
    .bind(&title)
    .bind(next_version)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE note_projections
         SET title = $2, version = $3, markdown = $4
         WHERE note_id = $1",
    )
    .bind(note_id)
    .bind(&title)
    .bind(next_version)
    .bind(&markdown)
    .execute(&mut *tx)
    .await?;

    let event_seq = next_version;
    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, 'note_rollback', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(event_seq)
    .bind(json!({
        "title": title,
        "markdown": markdown,
        "version": next_version,
        "rolled_back_to": target_version,
    }))
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    refresh_backlinks_tx(&mut tx, note_id, &markdown).await?;
    append_workspace_event_tx(
        &mut tx,
        stream.workspace_id,
        actor_id,
        "note_rollback",
        json!({"note_id": note_id, "version": next_version, "rolled_back_to": target_version}),
    )
    .await?;

    tx.commit().await?;
    Ok(NoteMutationResult {
        version: next_version,
        event_seq,
    })
}

pub async fn note_workspace_id(pool: &PgPool, note_id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>("SELECT workspace_id FROM note_streams WHERE id = $1")
        .bind(note_id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_metadata(
    pool: &PgPool,
    note_id: Uuid,
    key: &str,
    value_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO note_metadata (note_id, key, value_json)
         VALUES ($1, $2, $3)
         ON CONFLICT (note_id, key)
         DO UPDATE SET value_json = EXCLUDED.value_json, updated_at = NOW()",
    )
    .bind(note_id)
    .bind(key)
    .bind(value_json)
    .execute(&mut *tx)
    .await?;

    sync_projection_metadata_tx(&mut tx, note_id).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete_metadata_key(pool: &PgPool, note_id: Uuid, key: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM note_metadata WHERE note_id = $1 AND key = $2")
        .bind(note_id)
        .bind(key)
        .execute(&mut *tx)
        .await?;

    sync_projection_metadata_tx(&mut tx, note_id).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn replace_tags(pool: &PgPool, note_id: Uuid, tags: &[String]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM note_tags WHERE note_id = $1")
        .bind(note_id)
        .execute(&mut *tx)
        .await?;

    for tag in tags {
        sqlx::query("INSERT INTO note_tags (note_id, tag) VALUES ($1, $2)")
            .bind(note_id)
            .bind(tag)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn note_backlinks(pool: &PgPool, note_id: Uuid) -> Result<Vec<DbBacklink>, sqlx::Error> {
    sqlx::query_as::<_, DbBacklink>(
        "SELECT note_id, target_title, updated_at
         FROM note_backlinks
         WHERE note_id = $1
         ORDER BY updated_at DESC",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
}

pub async fn search_notes(
    pool: &PgPool,
    workspace_id: Uuid,
    query_text: &str,
) -> Result<Vec<DbSearchResult>, sqlx::Error> {
    sqlx::query_as::<_, DbSearchResult>(
        "SELECT
            np.note_id,
            np.title,
            np.note_kind,
            np.version,
            np.markdown,
            ts_rank(np.search_vector, plainto_tsquery('english', $2)) AS rank
         FROM note_projections np
         INNER JOIN note_streams ns ON ns.id = np.note_id
         WHERE np.workspace_id = $1
           AND ns.deleted_at IS NULL
           AND (
               np.search_vector @@ plainto_tsquery('english', $2)
               OR np.metadata_json::text ILIKE '%' || $2 || '%'
           )
         ORDER BY rank DESC, ns.updated_at DESC",
    )
    .bind(workspace_id)
    .bind(query_text)
    .fetch_all(pool)
    .await
}

async fn sync_projection_metadata_tx(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
) -> Result<(), sqlx::Error> {
    let metadata = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(jsonb_object_agg(key, value_json), '{}'::jsonb)
         FROM note_metadata
         WHERE note_id = $1",
    )
    .bind(note_id)
    .fetch_one(&mut **tx)
    .await?;

    sqlx::query("UPDATE note_projections SET metadata_json = $2 WHERE note_id = $1")
        .bind(note_id)
        .bind(metadata)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

async fn refresh_backlinks_tx(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
    markdown: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM note_backlinks WHERE note_id = $1")
        .bind(note_id)
        .execute(&mut **tx)
        .await?;

    for target in extract_backlinks(markdown) {
        sqlx::query(
            "INSERT INTO note_backlinks (note_id, target_title)
             VALUES ($1, $2)",
        )
        .bind(note_id)
        .bind(target)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn append_workspace_event_tx(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Uuid,
    actor_id: Uuid,
    event_type: &str,
    payload_json: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let seq: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(seq), 0) + 1 FROM workspace_events WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_one(&mut **tx)
    .await?;

    sqlx::query(
        "INSERT INTO workspace_events (workspace_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(workspace_id)
    .bind(seq)
    .bind(event_type)
    .bind(payload_json)
    .bind(actor_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
