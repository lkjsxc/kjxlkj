//! Note repository per /docs/spec/domain/notes.md and /docs/spec/domain/events.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct NoteRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub access_scope: String,
    pub created_at: String,
    pub updated_at: String,
    pub current_version: i64,
    pub deleted_at: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ProjectionRow {
    pub note_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: String,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, FromRow)]
pub struct EventRow {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: String,
}

/// Create a new note stream and its initial projection.
pub async fn create_note(
    pool: &PgPool, id: Uuid, ws_id: Uuid, project_id: Option<Uuid>,
    title: &str, note_kind: &str, access_scope: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_streams (id,workspace_id,project_id,title,note_kind,access_scope) \
         VALUES ($1,$2,$3,$4,$5,$6)"
    ).bind(id).bind(ws_id).bind(project_id).bind(title)
     .bind(note_kind).bind(access_scope)
     .execute(pool).await?;
    sqlx::query(
        "INSERT INTO note_projections (note_id,workspace_id,project_id,title,note_kind) \
         VALUES ($1,$2,$3,$4,$5)"
    ).bind(id).bind(ws_id).bind(project_id).bind(title).bind(note_kind)
     .execute(pool).await?;
    Ok(())
}

/// List non-deleted notes in workspace.
pub async fn list_notes(
    pool: &PgPool, ws_id: Uuid,
) -> Result<Vec<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "SELECT id, workspace_id, project_id, title, note_kind, \
         access_scope, created_at::text as created_at, \
         updated_at::text as updated_at, \
         current_version, deleted_at::text as deleted_at \
         FROM note_streams \
         WHERE workspace_id = $1 AND deleted_at IS NULL \
         ORDER BY updated_at DESC"
    ).bind(ws_id).fetch_all(pool).await
}

/// Get note by id.
pub async fn get_note(pool: &PgPool, id: Uuid) -> Result<Option<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "SELECT id, workspace_id, project_id, title, note_kind, \
         access_scope, created_at::text as created_at, \
         updated_at::text as updated_at, \
         current_version, deleted_at::text as deleted_at \
         FROM note_streams WHERE id = $1"
    ).bind(id).fetch_optional(pool).await
}

/// Get note projection.
pub async fn get_projection(pool: &PgPool, note_id: Uuid) -> Result<Option<ProjectionRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectionRow>(
        "SELECT note_id, workspace_id, project_id, title, note_kind, \
         version, markdown, metadata_json \
         FROM note_projections WHERE note_id = $1"
    ).bind(note_id).fetch_optional(pool).await
}

/// Apply a patch: increment version, append event, update projection.
/// Returns the new version. Callers must verify base_version == current_version.
pub async fn apply_mutation(
    pool: &PgPool, note_id: Uuid, base_version: i64,
    new_markdown: &str, new_title: Option<&str>,
    event_type: &str, payload: &serde_json::Value,
    actor_id: Uuid,
) -> Result<Option<i64>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    // Lock and check version
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT current_version FROM note_streams WHERE id=$1 FOR UPDATE"
    ).bind(note_id).fetch_optional(&mut *tx).await?;
    let current = match row {
        Some((v,)) => v,
        None => return Ok(None),
    };
    if current != base_version {
        return Ok(None); // conflict
    }
    let new_version = current + 1;
    let event_id = kjxlkj_domain::types::new_id();
    let seq = new_version;
    // Append event
    sqlx::query(
        "INSERT INTO note_events (event_id,note_id,seq,event_type,payload_json,actor_id) \
         VALUES ($1,$2,$3,$4,$5,$6)"
    ).bind(event_id).bind(note_id).bind(seq)
     .bind(event_type).bind(payload).bind(actor_id)
     .execute(&mut *tx).await?;
    // Update stream
    let title_update = new_title.unwrap_or("");
    if new_title.is_some() {
        sqlx::query(
            "UPDATE note_streams SET current_version=$1, updated_at=NOW(), title=$2 WHERE id=$3"
        ).bind(new_version).bind(title_update).bind(note_id)
         .execute(&mut *tx).await?;
    } else {
        sqlx::query(
            "UPDATE note_streams SET current_version=$1, updated_at=NOW() WHERE id=$2"
        ).bind(new_version).bind(note_id)
         .execute(&mut *tx).await?;
    }
    // Update projection
    let md_update = new_markdown;
    if new_title.is_some() {
        sqlx::query(
            "UPDATE note_projections SET version=$1, markdown=$2, title=$3, \
             search_vector=to_tsvector('english', $3 || ' ' || $2) WHERE note_id=$4"
        ).bind(new_version).bind(md_update).bind(title_update).bind(note_id)
         .execute(&mut *tx).await?;
    } else {
        sqlx::query(
            "UPDATE note_projections SET version=$1, markdown=$2, \
             search_vector=to_tsvector('english', title || ' ' || $2) WHERE note_id=$3"
        ).bind(new_version).bind(md_update).bind(note_id)
         .execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(Some(new_version))
}

/// Soft-delete a note per /docs/spec/domain/notes.md.
pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE note_streams SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL"
    ).bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

/// Get event history for a note.
pub async fn get_history(pool: &PgPool, note_id: Uuid) -> Result<Vec<EventRow>, sqlx::Error> {
    sqlx::query_as::<_, EventRow>(
        "SELECT event_id, note_id, seq, event_type, payload_json, \
         actor_id, created_at::text as created_at \
         FROM note_events WHERE note_id = $1 ORDER BY seq"
    ).bind(note_id).fetch_all(pool).await
}

/// Update title only (with version check).
pub async fn update_title(
    pool: &PgPool, note_id: Uuid, base_version: i64,
    new_title: &str, actor_id: Uuid,
) -> Result<Option<i64>, sqlx::Error> {
    // Get current markdown from projection
    let proj = get_projection(pool, note_id).await?;
    let md = proj.map(|p| p.markdown).unwrap_or_default();
    let payload = serde_json::json!({"title": new_title});
    apply_mutation(
        pool, note_id, base_version, &md, Some(new_title),
        "title_update", &payload, actor_id,
    ).await
}

/// Upsert metadata key per /docs/spec/domain/metadata.md.
pub async fn upsert_metadata(
    pool: &PgPool, note_id: Uuid, key: &str, value: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE note_projections SET metadata_json = \
         jsonb_set(metadata_json, ARRAY[$1], $2::jsonb, true) WHERE note_id = $3"
    ).bind(key).bind(value.to_string()).bind(note_id)
     .execute(pool).await?;
    Ok(())
}

/// Delete metadata key per /docs/spec/domain/metadata.md.
pub async fn delete_metadata(
    pool: &PgPool, note_id: Uuid, key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE note_projections SET metadata_json = metadata_json - $1 WHERE note_id = $2"
    ).bind(key).bind(note_id)
     .execute(pool).await?;
    Ok(())
}

/// Replace tags on a note.
pub async fn replace_tags(
    pool: &PgPool, note_id: Uuid, tags: &[String],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM note_tags WHERE note_id = $1")
        .bind(note_id).execute(&mut *tx).await?;
    for tag in tags {
        sqlx::query("INSERT INTO note_tags (note_id, tag) VALUES ($1, $2)")
            .bind(note_id).bind(tag).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

/// List tags per /docs/spec/api/http.md.
pub async fn list_tags(pool: &PgPool, ws_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT tag FROM note_tags nt \
         JOIN note_streams ns ON nt.note_id = ns.id \
         WHERE ns.workspace_id = $1 AND ns.deleted_at IS NULL ORDER BY tag"
    ).bind(ws_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

/// Update backlinks for a note.
pub async fn update_backlinks(
    pool: &PgPool, source_id: Uuid, targets: &[String],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM backlinks WHERE source_note_id = $1")
        .bind(source_id).execute(&mut *tx).await?;
    for t in targets {
        sqlx::query(
            "INSERT INTO backlinks (source_note_id, target_title) VALUES ($1, $2)"
        ).bind(source_id).bind(t).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Get backlinks for a note (by title).
pub async fn get_backlinks(
    pool: &PgPool, note_id: Uuid,
) -> Result<Vec<NoteRow>, sqlx::Error> {
    let title: Option<(String,)> = sqlx::query_as(
        "SELECT title FROM note_streams WHERE id = $1"
    ).bind(note_id).fetch_optional(pool).await?;
    let title = match title {
        Some((t,)) => t,
        None => return Ok(vec![]),
    };
    sqlx::query_as::<_, NoteRow>(
        "SELECT ns.id, ns.workspace_id, ns.project_id, ns.title, \
         ns.note_kind, ns.access_scope, \
         ns.created_at::text as created_at, \
         ns.updated_at::text as updated_at, \
         ns.current_version, ns.deleted_at::text as deleted_at \
         FROM backlinks b \
         JOIN note_streams ns ON b.source_note_id = ns.id \
         WHERE b.target_title = $1 AND ns.deleted_at IS NULL \
         ORDER BY ns.updated_at DESC"
    ).bind(title).fetch_all(pool).await
}

/// Idempotency key check for WS patches.
pub async fn check_idempotency(
    pool: &PgPool, note_id: Uuid, key: &str,
) -> Result<Option<(Uuid, i64)>, sqlx::Error> {
    let row: Option<(Uuid, i64)> = sqlx::query_as(
        "SELECT event_id, version FROM idempotency_keys \
         WHERE note_id = $1 AND idempotency_key = $2"
    ).bind(note_id).bind(key).fetch_optional(pool).await?;
    Ok(row)
}

/// Record idempotency key.
pub async fn record_idempotency(
    pool: &PgPool, note_id: Uuid, key: &str,
    event_id: Uuid, version: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO idempotency_keys (note_id,idempotency_key,event_id,version) \
         VALUES ($1,$2,$3,$4) ON CONFLICT DO NOTHING"
    ).bind(note_id).bind(key).bind(event_id).bind(version)
     .execute(pool).await?;
    Ok(())
}
