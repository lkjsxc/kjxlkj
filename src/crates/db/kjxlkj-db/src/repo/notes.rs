// Notes repository per /docs/spec/domain/notes.md
use kjxlkj_domain::types::{NoteStream, NoteProjection, NoteEvent, NoteKind, AccessScope};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_note_stream(pool: &PgPool, n: &NoteStream) -> Result<(), sqlx::Error> {
    let kind = note_kind_str(n.note_kind);
    let scope = scope_str(n.access_scope);
    sqlx::query(
        "INSERT INTO note_streams (id, workspace_id, project_id, title, note_kind, access_scope, created_at, updated_at, current_version)
         VALUES ($1, $2, $3, $4, $5, $6, now(), now(), 0)",
    )
    .bind(n.id).bind(n.workspace_id).bind(n.project_id)
    .bind(&n.title).bind(kind).bind(scope)
    .execute(pool).await?;

    // Init projection
    sqlx::query(
        "INSERT INTO note_projections (note_id, workspace_id, project_id, title, note_kind, version, markdown, metadata_json, search_vector)
         VALUES ($1, $2, $3, $4, $5, 0, '', '{}', to_tsvector('english', $4))",
    )
    .bind(n.id).bind(n.workspace_id).bind(n.project_id)
    .bind(&n.title).bind(kind)
    .execute(pool).await?;

    Ok(())
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<NoteProjection>, sqlx::Error> {
    let row: Option<(Uuid, Uuid, Option<Uuid>, String, String, i64, String, serde_json::Value)> =
        sqlx::query_as(
            "SELECT note_id, workspace_id, project_id, title, note_kind, version, markdown, metadata_json
             FROM note_projections WHERE note_id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| NoteProjection {
        note_id: r.0, workspace_id: r.1, project_id: r.2, title: r.3,
        note_kind: parse_note_kind(&r.4), version: r.5, markdown: r.6, metadata_json: r.7,
    }))
}

pub async fn list_notes(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<NoteProjection>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, Option<Uuid>, String, String, i64, String, serde_json::Value)> =
        sqlx::query_as(
            "SELECT np.note_id, np.workspace_id, np.project_id, np.title, np.note_kind, np.version, np.markdown, np.metadata_json
             FROM note_projections np
             JOIN note_streams ns ON ns.id = np.note_id
             WHERE np.workspace_id = $1 AND ns.deleted_at IS NULL
             ORDER BY ns.updated_at DESC",
        )
        .bind(workspace_id)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| NoteProjection {
        note_id: r.0, workspace_id: r.1, project_id: r.2, title: r.3,
        note_kind: parse_note_kind(&r.4), version: r.5, markdown: r.6, metadata_json: r.7,
    }).collect())
}

/// Apply a note mutation with version check.
/// Returns new version or None on conflict.
pub async fn apply_mutation(
    pool: &PgPool,
    note_id: Uuid,
    base_version: i64,
    new_markdown: &str,
    actor_id: Uuid,
    event_type: &str,
    payload: &serde_json::Value,
) -> Result<Option<i64>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Check version
    let current: (i64,) = sqlx::query_as(
        "SELECT current_version FROM note_streams WHERE id = $1 FOR UPDATE",
    )
    .bind(note_id)
    .fetch_one(&mut *tx)
    .await?;

    if current.0 != base_version {
        return Ok(None); // Version conflict
    }

    let new_version = base_version + 1;
    let event_id = Uuid::now_v7();
    let seq = new_version;

    // Append event
    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, now())",
    )
    .bind(event_id).bind(note_id).bind(seq)
    .bind(event_type).bind(payload).bind(actor_id)
    .execute(&mut *tx).await?;

    // Update stream
    sqlx::query(
        "UPDATE note_streams SET current_version = $1, updated_at = now() WHERE id = $2",
    )
    .bind(new_version).bind(note_id)
    .execute(&mut *tx).await?;

    // Update projection
    sqlx::query(
        "UPDATE note_projections SET version = $1, markdown = $2,
         search_vector = to_tsvector('english', title || ' ' || $2)
         WHERE note_id = $3",
    )
    .bind(new_version).bind(new_markdown).bind(note_id)
    .execute(&mut *tx).await?;

    // Snapshot every 100 events
    if new_version % 100 == 0 {
        sqlx::query(
            "INSERT INTO note_snapshots (note_id, version, markdown, metadata_json, created_at)
             SELECT note_id, version, markdown, metadata_json, now()
             FROM note_projections WHERE note_id = $1",
        )
        .bind(note_id)
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(Some(new_version))
}

/// Update title with version check.
pub async fn update_title(
    pool: &PgPool,
    note_id: Uuid,
    base_version: i64,
    new_title: &str,
    actor_id: Uuid,
) -> Result<Option<i64>, sqlx::Error> {
    let payload = serde_json::json!({ "title": new_title });
    let mut tx = pool.begin().await?;

    let current: (i64,) = sqlx::query_as(
        "SELECT current_version FROM note_streams WHERE id = $1 FOR UPDATE",
    )
    .bind(note_id)
    .fetch_one(&mut *tx)
    .await?;

    if current.0 != base_version {
        return Ok(None);
    }

    let new_version = base_version + 1;
    let event_id = Uuid::now_v7();

    sqlx::query(
        "INSERT INTO note_events (event_id, note_id, seq, event_type, payload_json, actor_id, created_at)
         VALUES ($1, $2, $3, 'title_changed', $4, $5, now())",
    )
    .bind(event_id).bind(note_id).bind(new_version)
    .bind(&payload).bind(actor_id)
    .execute(&mut *tx).await?;

    sqlx::query(
        "UPDATE note_streams SET current_version = $1, title = $2, updated_at = now() WHERE id = $3",
    )
    .bind(new_version).bind(new_title).bind(note_id)
    .execute(&mut *tx).await?;

    sqlx::query(
        "UPDATE note_projections SET version = $1, title = $2,
         search_vector = to_tsvector('english', $2 || ' ' || markdown)
         WHERE note_id = $3",
    )
    .bind(new_version).bind(new_title).bind(note_id)
    .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(Some(new_version))
}

/// Soft-delete note.
pub async fn soft_delete(pool: &PgPool, note_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE note_streams SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(note_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Get event history for a note.
pub async fn event_history(pool: &PgPool, note_id: Uuid) -> Result<Vec<NoteEvent>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, i64, String, serde_json::Value, Uuid)> = sqlx::query_as(
        "SELECT event_id, note_id, seq, event_type, payload_json, actor_id
         FROM note_events WHERE note_id = $1 ORDER BY seq",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| NoteEvent {
        event_id: r.0, note_id: r.1, seq: r.2, event_type: r.3,
        payload_json: r.4, actor_id: r.5, created_at: String::new(),
    }).collect())
}

/// Get current version for a note stream.
pub async fn current_version(pool: &PgPool, note_id: Uuid) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT current_version FROM note_streams WHERE id = $1",
    )
    .bind(note_id)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

fn note_kind_str(k: NoteKind) -> &'static str {
    match k {
        NoteKind::Markdown => "markdown",
        NoteKind::Settings => "settings",
        NoteKind::MediaImage => "media_image",
        NoteKind::MediaVideo => "media_video",
    }
}

fn scope_str(s: AccessScope) -> &'static str {
    match s {
        AccessScope::Workspace => "workspace",
        AccessScope::Project => "project",
        AccessScope::Private => "private",
    }
}

fn parse_note_kind(s: &str) -> NoteKind {
    match s {
        "settings" => NoteKind::Settings,
        "media_image" => NoteKind::MediaImage,
        "media_video" => NoteKind::MediaVideo,
        _ => NoteKind::Markdown,
    }
}
