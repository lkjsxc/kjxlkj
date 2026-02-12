use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{NoteEvent, NoteProjection, NoteSummary},
    patch::{apply_patch, normalize_tags, parse_wikilinks, PatchOp},
};

pub async fn create_note(
    pool: &PgPool,
    actor_id: Uuid,
    workspace_id: Option<Uuid>,
    project_id: Option<Uuid>,
    title: &str,
    markdown: &str,
    note_kind: &str,
    access_scope: &str,
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let note_id = Uuid::now_v7();
    let event_id = Uuid::now_v7();
    let metadata = json!({});
    let tags: Vec<String> = vec![];

    sqlx::query(
        "insert into note_streams
         (id, workspace_id, project_id, title, note_kind, access_scope, current_version)
         values ($1, $2, $3, $4, $5, $6, 1)",
    )
    .bind(note_id)
    .bind(workspace_id)
    .bind(project_id)
    .bind(title)
    .bind(note_kind)
    .bind(access_scope)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "insert into note_projections
         (note_id, workspace_id, project_id, title, note_kind, version, markdown, metadata_json, tags, search_vector)
         values ($1, $2, $3, $4, $5, 1, $6, $7, $8, to_tsvector('simple', $4 || ' ' || $6 || ' ' || $7::text))",
    )
    .bind(note_id)
    .bind(workspace_id)
    .bind(project_id)
    .bind(title)
    .bind(note_kind)
    .bind(markdown)
    .bind(&metadata)
    .bind(&tags)
    .execute(&mut *tx)
    .await?;

    let payload = json!({
        "title": title,
        "markdown_after": markdown,
        "metadata_after": metadata,
        "tags_after": tags,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, 1, 'create', $3, $4)",
    )
    .bind(event_id)
    .bind(note_id)
    .bind(payload)
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    refresh_backlinks(&mut tx, note_id, markdown).await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn list_notes(
    pool: &PgPool,
    include_deleted: bool,
    workspace_id: Option<Uuid>,
) -> Result<Vec<NoteSummary>, AppError> {
    let rows = sqlx::query_as::<_, NoteSummary>(
        "select id, workspace_id, project_id, title, note_kind, access_scope, current_version, updated_at, deleted_at
         from note_streams
         where ($1 or deleted_at is null)
           and ($2::uuid is null or workspace_id = $2)
         order by updated_at desc
         limit 200",
    )
    .bind(include_deleted)
    .bind(workspace_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_note(pool: &PgPool, note_id: Uuid) -> Result<NoteProjection, AppError> {
    let row = sqlx::query_as::<_, NoteProjection>(
        "select note_id, workspace_id, project_id, title, note_kind, version, markdown, rendered_html, metadata_json, tags, updated_at
         from note_projections where note_id = $1",
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("note not found".to_string()))?;
    Ok(row)
}

pub async fn note_history(pool: &PgPool, note_id: Uuid) -> Result<Vec<NoteEvent>, AppError> {
    let rows = sqlx::query_as::<_, NoteEvent>(
        "select event_id, note_id, seq, event_type, payload_json, actor_id, created_at
         from note_events where note_id = $1 order by seq asc",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn soft_delete_note(pool: &PgPool, note_id: Uuid) -> Result<(), AppError> {
    let changed = sqlx::query(
        "update note_streams set deleted_at = now(), updated_at = now() where id = $1 and deleted_at is null",
    )
    .bind(note_id)
    .execute(pool)
    .await?
    .rows_affected();
    if changed == 0 {
        return Err(AppError::NotFound("note not found".to_string()));
    }
    Ok(())
}

pub async fn patch_note_title(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    base_version: i64,
    title: &str,
    idempotency_key: &str,
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let (_, current_version) = locked_stream(&mut tx, note_id).await?;
    if base_version != current_version {
        return Err(AppError::VersionConflict {
            expected_version: base_version,
            current_version,
        });
    }
    let projection = locked_projection(&mut tx, note_id).await?;
    let new_version = current_version + 1;
    let payload = json!({
        "title_after": title,
        "markdown_after": projection.markdown,
        "metadata_after": projection.metadata_json,
        "tags_after": projection.tags,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id, idempotency_key)
         values ($1, $2, $3, 'title_patch', $4, $5, $6)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(payload)
    .bind(actor_id)
    .bind(idempotency_key)
    .execute(&mut *tx)
    .await?;

    update_projection_and_stream(
        &mut tx,
        note_id,
        title,
        new_version,
        &projection.markdown,
        &projection.metadata_json,
        &projection.tags,
    )
    .await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn apply_note_patch(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    base_version: i64,
    patch_ops: &[PatchOp],
    idempotency_key: &str,
) -> Result<(i64, i64, NoteProjection), AppError> {
    let mut tx = pool.begin().await?;
    let (title, current_version) = locked_stream(&mut tx, note_id).await?;
    if base_version != current_version {
        if let Some(existing_seq) =
            find_event_seq_by_idempotency(&mut tx, note_id, idempotency_key).await?
        {
            let projection = locked_projection(&mut tx, note_id).await?;
            tx.commit().await?;
            return Ok((existing_seq, existing_seq, projection));
        }
        return Err(AppError::VersionConflict {
            expected_version: base_version,
            current_version,
        });
    }
    let projection = locked_projection(&mut tx, note_id).await?;
    let new_markdown = apply_patch(&projection.markdown, patch_ops)?;
    let new_version = current_version + 1;
    let payload = json!({
        "patch_ops": patch_ops,
        "markdown_after": new_markdown,
        "metadata_after": projection.metadata_json,
        "tags_after": projection.tags,
    });

    let insert_result = sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id, idempotency_key)
         values ($1, $2, $3, 'patch', $4, $5, $6)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(payload)
    .bind(actor_id)
    .bind(idempotency_key)
    .execute(&mut *tx)
    .await;
    if let Err(err) = insert_result {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code().as_deref() == Some("23505") {
                if let Some(existing_seq) =
                    find_event_seq_by_idempotency(&mut tx, note_id, idempotency_key).await?
                {
                    let latest = locked_projection(&mut tx, note_id).await?;
                    tx.commit().await?;
                    return Ok((existing_seq, existing_seq, latest));
                }
            }
        }
        return Err(err.into());
    }

    update_projection_and_stream(
        &mut tx,
        note_id,
        &title,
        new_version,
        &new_markdown,
        &projection.metadata_json,
        &projection.tags,
    )
    .await?;
    refresh_backlinks(&mut tx, note_id, &new_markdown).await?;
    maybe_insert_snapshot(
        &mut tx,
        note_id,
        new_version,
        &new_markdown,
        &projection.metadata_json,
        &projection.tags,
    )
    .await?;
    tx.commit().await?;
    let next = get_note(pool, note_id).await?;
    Ok((new_version, new_version, next))
}

pub async fn rollback_note(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    target_version: i64,
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let (title, current_version) = locked_stream(&mut tx, note_id).await?;
    if target_version < 1 || target_version > current_version {
        return Err(AppError::BadRequest("invalid target_version".to_string()));
    }

    let payload = sqlx::query_scalar::<_, Value>(
        "select payload_json from note_events where note_id = $1 and seq = $2",
    )
    .bind(note_id)
    .bind(target_version)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("target version not found".to_string()))?;

    let markdown = payload
        .get("markdown_after")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Internal)?
        .to_string();
    let metadata = payload
        .get("metadata_after")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let tags = payload
        .get("tags_after")
        .cloned()
        .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
        .unwrap_or_default();

    let new_version = current_version + 1;
    let rollback_payload = json!({
        "target_version": target_version,
        "markdown_after": markdown,
        "metadata_after": metadata,
        "tags_after": tags,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, $3, 'rollback', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(rollback_payload)
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    update_projection_and_stream(
        &mut tx,
        note_id,
        &title,
        new_version,
        &markdown,
        &metadata,
        &tags,
    )
    .await?;
    refresh_backlinks(&mut tx, note_id, &markdown).await?;
    maybe_insert_snapshot(&mut tx, note_id, new_version, &markdown, &metadata, &tags).await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn upsert_metadata(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    key: &str,
    value: Value,
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let (title, current_version) = locked_stream(&mut tx, note_id).await?;
    let mut projection = locked_projection(&mut tx, note_id).await?;
    let mut map = projection
        .metadata_json
        .as_object()
        .cloned()
        .unwrap_or_default();
    map.insert(key.to_string(), value.clone());
    let metadata = Value::Object(map);
    let new_version = current_version + 1;
    let payload = json!({
        "key": key,
        "value": value,
        "markdown_after": projection.markdown,
        "metadata_after": metadata,
        "tags_after": projection.tags,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, $3, 'metadata_upsert', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(payload)
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    update_projection_and_stream(
        &mut tx,
        note_id,
        &title,
        new_version,
        &projection.markdown,
        &metadata,
        &projection.tags,
    )
    .await?;
    projection.metadata_json = metadata;
    maybe_insert_snapshot(
        &mut tx,
        note_id,
        new_version,
        &projection.markdown,
        &projection.metadata_json,
        &projection.tags,
    )
    .await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn delete_metadata(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    key: &str,
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let (title, current_version) = locked_stream(&mut tx, note_id).await?;
    let projection = locked_projection(&mut tx, note_id).await?;
    let mut map = projection
        .metadata_json
        .as_object()
        .cloned()
        .unwrap_or_default();
    map.remove(key);
    let metadata = Value::Object(map);
    let new_version = current_version + 1;
    let payload = json!({
        "key": key,
        "markdown_after": projection.markdown,
        "metadata_after": metadata,
        "tags_after": projection.tags,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, $3, 'metadata_delete', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(payload)
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    update_projection_and_stream(
        &mut tx,
        note_id,
        &title,
        new_version,
        &projection.markdown,
        &metadata,
        &projection.tags,
    )
    .await?;
    maybe_insert_snapshot(
        &mut tx,
        note_id,
        new_version,
        &projection.markdown,
        &metadata,
        &projection.tags,
    )
    .await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn replace_tags(
    pool: &PgPool,
    actor_id: Uuid,
    note_id: Uuid,
    tags: &[String],
) -> Result<NoteProjection, AppError> {
    let mut tx = pool.begin().await?;
    let (title, current_version) = locked_stream(&mut tx, note_id).await?;
    let projection = locked_projection(&mut tx, note_id).await?;
    let tags = normalize_tags(tags);
    let new_version = current_version + 1;
    let payload = json!({
        "tags_after": tags,
        "markdown_after": projection.markdown,
        "metadata_after": projection.metadata_json,
    });
    sqlx::query(
        "insert into note_events (event_id, note_id, seq, event_type, payload_json, actor_id)
         values ($1, $2, $3, 'tags_replace', $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(note_id)
    .bind(new_version)
    .bind(payload)
    .bind(actor_id)
    .execute(&mut *tx)
    .await?;

    update_projection_and_stream(
        &mut tx,
        note_id,
        &title,
        new_version,
        &projection.markdown,
        &projection.metadata_json,
        &tags,
    )
    .await?;
    maybe_insert_snapshot(
        &mut tx,
        note_id,
        new_version,
        &projection.markdown,
        &projection.metadata_json,
        &tags,
    )
    .await?;
    tx.commit().await?;
    get_note(pool, note_id).await
}

pub async fn list_tags(pool: &PgPool) -> Result<Vec<String>, AppError> {
    let rows = sqlx::query_scalar::<_, String>(
        "select distinct unnest(tags) as tag from note_projections order by tag asc",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn backlinks(pool: &PgPool, note_id: Uuid) -> Result<Vec<NoteSummary>, AppError> {
    let rows = sqlx::query_as::<_, NoteSummary>(
        "select s.id, s.workspace_id, s.project_id, s.title, s.note_kind, s.access_scope, s.current_version, s.updated_at, s.deleted_at
         from note_backlinks b
         join note_streams s on s.id = b.source_note_id
         where b.note_id = $1 and s.deleted_at is null
         order by s.updated_at desc",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn search(
    pool: &PgPool,
    q: &str,
    workspace_id: Option<Uuid>,
) -> Result<Vec<NoteSummary>, AppError> {
    let rows = sqlx::query_as::<_, NoteSummary>(
        "select s.id, s.workspace_id, s.project_id, s.title, s.note_kind, s.access_scope, s.current_version, s.updated_at, s.deleted_at
         from note_streams s
         join note_projections p on p.note_id = s.id
         where s.deleted_at is null
         and ($2::uuid is null or s.workspace_id = $2)
         and ($1 = '' or p.search_vector @@ websearch_to_tsquery('simple', $1)
             or s.title ilike '%' || $1 || '%' or p.markdown ilike '%' || $1 || '%'
             or p.metadata_json::text ilike '%' || $1 || '%')
         order by s.updated_at desc limit 200",
    )
    .bind(q)
    .bind(workspace_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

async fn locked_stream(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
) -> Result<(String, i64), AppError> {
    let row = sqlx::query_as::<_, (String, i64, Option<DateTime<Utc>>)>(
        "select title, current_version, deleted_at from note_streams where id = $1 for update",
    )
    .bind(note_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::NotFound("note not found".to_string()))?;
    if row.2.is_some() {
        return Err(AppError::NotFound("note deleted".to_string()));
    }
    Ok((row.0, row.1))
}

async fn locked_projection(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
) -> Result<NoteProjection, AppError> {
    let row = sqlx::query_as::<_, NoteProjection>(
        "select note_id, workspace_id, project_id, title, note_kind, version, markdown, rendered_html, metadata_json, tags, updated_at
         from note_projections where note_id = $1 for update",
    )
    .bind(note_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::NotFound("projection not found".to_string()))?;
    Ok(row)
}

async fn update_projection_and_stream(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
    title: &str,
    version: i64,
    markdown: &str,
    metadata: &Value,
    tags: &[String],
) -> Result<(), AppError> {
    sqlx::query("update note_streams set current_version = $2, updated_at = now() where id = $1")
        .bind(note_id)
        .bind(version)
        .execute(&mut **tx)
        .await?;
    sqlx::query(
        "update note_projections set title = $2, version = $3, markdown = $4, metadata_json = $5, tags = $6,
         search_vector = to_tsvector('simple', $2 || ' ' || $4 || ' ' || $5::text), updated_at = now() where note_id = $1",
    )
    .bind(note_id)
    .bind(title)
    .bind(version)
    .bind(markdown)
    .bind(metadata)
    .bind(tags)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn note_workspace_id(pool: &PgPool, note_id: Uuid) -> Result<Option<Uuid>, AppError> {
    let workspace_id =
        sqlx::query_scalar::<_, Uuid>("select workspace_id from note_streams where id = $1")
            .bind(note_id)
            .fetch_optional(pool)
            .await?;
    Ok(workspace_id)
}

async fn refresh_backlinks(
    tx: &mut Transaction<'_, Postgres>,
    source_note_id: Uuid,
    markdown: &str,
) -> Result<(), AppError> {
    sqlx::query("delete from note_backlinks where source_note_id = $1")
        .bind(source_note_id)
        .execute(&mut **tx)
        .await?;

    for title in parse_wikilinks(markdown) {
        let target = sqlx::query_scalar::<_, Uuid>(
            "select id from note_streams where lower(title) = lower($1) and deleted_at is null order by updated_at desc limit 1",
        )
        .bind(&title)
        .fetch_optional(&mut **tx)
        .await?;
        if let Some(target_id) = target {
            if target_id != source_note_id {
                sqlx::query(
                    "insert into note_backlinks (note_id, source_note_id) values ($1, $2) on conflict do nothing",
                )
                .bind(target_id)
                .bind(source_note_id)
                .execute(&mut **tx)
                .await?;
            }
        }
    }
    Ok(())
}

async fn find_event_seq_by_idempotency(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
    idempotency_key: &str,
) -> Result<Option<i64>, AppError> {
    if idempotency_key.trim().is_empty() {
        return Ok(None);
    }
    let seq = sqlx::query_scalar::<_, i64>(
        "select seq from note_events where note_id = $1 and idempotency_key = $2",
    )
    .bind(note_id)
    .bind(idempotency_key)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(seq)
}

async fn maybe_insert_snapshot(
    tx: &mut Transaction<'_, Postgres>,
    note_id: Uuid,
    version: i64,
    markdown: &str,
    metadata: &Value,
    tags: &[String],
) -> Result<(), AppError> {
    if version % 100 != 0 {
        return Ok(());
    }
    sqlx::query(
        "insert into note_snapshots (note_id, version, markdown, metadata_json, tags)
         values ($1, $2, $3, $4, $5) on conflict do nothing",
    )
    .bind(note_id)
    .bind(version)
    .bind(markdown)
    .bind(metadata)
    .bind(tags)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{apply_note_patch, create_note};
    use crate::{error::AppError, patch::PatchOp};
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    #[ignore = "requires DATABASE_URL"]
    async fn stale_patch_returns_version_conflict(pool: PgPool) -> anyhow::Result<()> {
        let actor = Uuid::now_v7();
        let note = create_note(
            &pool,
            actor,
            None,
            None,
            "conflict note",
            "hello",
            "markdown",
            "workspace",
        )
        .await?;
        let err = apply_note_patch(
            &pool,
            actor,
            note.note_id,
            0,
            &[PatchOp::Insert {
                insert: "x".to_string(),
            }],
            "k-1",
        )
        .await
        .expect_err("must reject stale version");

        match err {
            AppError::VersionConflict {
                expected_version,
                current_version,
            } => {
                assert_eq!(expected_version, 0);
                assert_eq!(current_version, 1);
            }
            other => panic!("expected version conflict, got {other}"),
        }
        Ok(())
    }

    #[sqlx::test]
    #[ignore = "requires DATABASE_URL"]
    async fn duplicate_idempotency_is_not_double_applied(pool: PgPool) -> anyhow::Result<()> {
        let actor = Uuid::now_v7();
        let note = create_note(
            &pool,
            actor,
            None,
            None,
            "idempotent note",
            "abc",
            "markdown",
            "workspace",
        )
        .await?;
        let ops = vec![
            PatchOp::Delete { delete: 3 },
            PatchOp::Insert {
                insert: "abcd".to_string(),
            },
        ];

        let first = apply_note_patch(&pool, actor, note.note_id, 1, &ops, "same-key").await?;
        let second = apply_note_patch(&pool, actor, note.note_id, 1, &ops, "same-key").await?;

        assert_eq!(first.0, 2);
        assert_eq!(second.0, 2);
        assert_eq!(second.2.version, 2);
        assert_eq!(second.2.markdown, "abcd");

        let event_count: i64 = sqlx::query_scalar(
            "select count(*) from note_events where note_id = $1 and idempotency_key = $2",
        )
        .bind(note.note_id)
        .bind("same-key")
        .fetch_one(&pool)
        .await?;
        assert_eq!(event_count, 1);
        Ok(())
    }

    #[sqlx::test]
    #[ignore = "requires DATABASE_URL"]
    async fn snapshot_created_every_100_versions(pool: PgPool) -> anyhow::Result<()> {
        let actor = Uuid::now_v7();
        let note = create_note(
            &pool,
            actor,
            None,
            None,
            "snapshot note",
            "x",
            "markdown",
            "workspace",
        )
        .await?;
        let mut version = 1_i64;
        let mut doc = "x".to_string();

        // version 1 (create) + 99 patches -> version 100 snapshot boundary.
        for _ in 0..99 {
            doc.push('y');
            let ops = vec![
                PatchOp::Delete {
                    delete: doc.len() - 1,
                },
                PatchOp::Insert {
                    insert: doc.clone(),
                },
            ];
            let applied = apply_note_patch(
                &pool,
                actor,
                note.note_id,
                version,
                &ops,
                &Uuid::now_v7().to_string(),
            )
            .await?;
            version = applied.0;
        }

        assert_eq!(version, 100);
        let snapshot_count: i64 = sqlx::query_scalar(
            "select count(*) from note_snapshots where note_id = $1 and version = 100",
        )
        .bind(note.note_id)
        .fetch_one(&pool)
        .await?;
        assert_eq!(snapshot_count, 1);
        Ok(())
    }
}
