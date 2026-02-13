use crate::models::note::NoteRow;
use kjxlkj_domain::types::{AccessScope, NoteKind};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    title: &str,
    body: &str,
    note_kind: NoteKind,
    access_scope: AccessScope,
) -> Result<NoteRow, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "INSERT INTO notes (id, workspace_id, project_id, title, body,
         note_kind, access_scope, version)
         VALUES ($1,$2,$3,$4,$5,$6::text,$7::text,1) RETURNING *",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(project_id)
    .bind(title)
    .bind(body)
    .bind(note_kind)
    .bind(access_scope)
    .fetch_one(pool)
    .await
}

pub async fn list_active(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "SELECT * FROM notes WHERE workspace_id = $1
         AND is_deleted = false ORDER BY updated_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update_body(
    pool: &PgPool,
    id: Uuid,
    body: &str,
    base_version: i64,
) -> Result<Option<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "UPDATE notes SET body = $2, version = version + 1, updated_at = now()
         WHERE id = $1 AND version = $3 AND is_deleted = false RETURNING *",
    )
    .bind(id)
    .bind(body)
    .bind(base_version)
    .fetch_optional(pool)
    .await
}

pub async fn update_title(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    base_version: i64,
) -> Result<Option<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "UPDATE notes SET title = $2, version = version + 1, updated_at = now()
         WHERE id = $1 AND version = $3 AND is_deleted = false RETURNING *",
    )
    .bind(id)
    .bind(title)
    .bind(base_version)
    .fetch_optional(pool)
    .await
}

pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE notes SET is_deleted = true, updated_at = now()
         WHERE id = $1 AND is_deleted = false",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(r.rows_affected() > 0)
}

pub async fn get_version(pool: &PgPool, id: Uuid) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT version FROM notes WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0))
}

pub async fn rollback(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    body: &str,
    new_version: i64,
) -> Result<Option<NoteRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteRow>(
        "UPDATE notes SET title = $2, body = $3, version = $4, updated_at = now()
         WHERE id = $1 AND is_deleted = false RETURNING *",
    )
    .bind(id)
    .bind(title)
    .bind(body)
    .bind(new_version)
    .fetch_optional(pool)
    .await
}
