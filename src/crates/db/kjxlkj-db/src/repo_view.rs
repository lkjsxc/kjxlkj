//! Saved views repository per /docs/spec/api/http.md.

use kjxlkj_domain::ids::WorkspaceId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SavedViewRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: serde_json::Value,
    pub owner_user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn list_views(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<SavedViewRow>, sqlx::Error> {
    sqlx::query_as::<_, SavedViewRow>(
        "SELECT id, workspace_id, name, query_json, sort, filters,
                owner_user_id, created_at, updated_at
         FROM saved_views WHERE workspace_id = $1
         ORDER BY created_at",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

pub async fn create_view(
    pool: &PgPool,
    id: Uuid,
    workspace_id: WorkspaceId,
    name: &str,
    query_json: &serde_json::Value,
    sort: Option<&str>,
    filters: &serde_json::Value,
    owner_user_id: Uuid,
) -> Result<SavedViewRow, sqlx::Error> {
    sqlx::query_as::<_, SavedViewRow>(
        "INSERT INTO saved_views
         (id, workspace_id, name, query_json, sort, filters, owner_user_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, workspace_id, name, query_json, sort, filters,
                   owner_user_id, created_at, updated_at",
    )
    .bind(id)
    .bind(workspace_id.0)
    .bind(name)
    .bind(query_json)
    .bind(sort)
    .bind(filters)
    .bind(owner_user_id)
    .fetch_one(pool)
    .await
}

pub async fn update_view(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    query_json: &serde_json::Value,
    sort: Option<&str>,
    filters: &serde_json::Value,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE saved_views
         SET name = $2, query_json = $3, sort = $4, filters = $5,
             updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(name)
    .bind(query_json)
    .bind(sort)
    .bind(filters)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_view(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM saved_views WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
