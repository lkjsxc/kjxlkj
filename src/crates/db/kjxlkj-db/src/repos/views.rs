use crate::models::DbSavedView;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSavedViewInput {
    pub workspace_id: Uuid,
    pub owner_user_id: Uuid,
    pub query_json: serde_json::Value,
    pub sort: String,
    pub filters: serde_json::Value,
}

pub async fn list_saved_views(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<DbSavedView>, sqlx::Error> {
    sqlx::query_as::<_, DbSavedView>(
        "SELECT id, workspace_id, owner_user_id, query_json, sort, filters, created_at, updated_at
         FROM saved_views
         WHERE workspace_id = $1
         ORDER BY updated_at DESC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn create_saved_view(
    pool: &PgPool,
    input: CreateSavedViewInput,
) -> Result<DbSavedView, sqlx::Error> {
    sqlx::query_as::<_, DbSavedView>(
        "INSERT INTO saved_views (id, workspace_id, owner_user_id, query_json, sort, filters)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, workspace_id, owner_user_id, query_json, sort, filters, created_at, updated_at",
    )
    .bind(Uuid::now_v7())
    .bind(input.workspace_id)
    .bind(input.owner_user_id)
    .bind(input.query_json)
    .bind(input.sort)
    .bind(input.filters)
    .fetch_one(pool)
    .await
}

pub async fn get_saved_view(
    pool: &PgPool,
    view_id: Uuid,
) -> Result<Option<DbSavedView>, sqlx::Error> {
    sqlx::query_as::<_, DbSavedView>(
        "SELECT id, workspace_id, owner_user_id, query_json, sort, filters, created_at, updated_at
         FROM saved_views
         WHERE id = $1",
    )
    .bind(view_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_saved_view(
    pool: &PgPool,
    view_id: Uuid,
    query_json: Option<serde_json::Value>,
    sort: Option<String>,
    filters: Option<serde_json::Value>,
) -> Result<Option<DbSavedView>, sqlx::Error> {
    sqlx::query_as::<_, DbSavedView>(
        "UPDATE saved_views
         SET query_json = COALESCE($2, query_json),
             sort = COALESCE($3, sort),
             filters = COALESCE($4, filters),
             updated_at = NOW()
         WHERE id = $1
         RETURNING id, workspace_id, owner_user_id, query_json, sort, filters, created_at, updated_at",
    )
    .bind(view_id)
    .bind(query_json)
    .bind(sort)
    .bind(filters)
    .fetch_optional(pool)
    .await
}

pub async fn delete_saved_view(pool: &PgPool, view_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM saved_views WHERE id = $1")
        .bind(view_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
