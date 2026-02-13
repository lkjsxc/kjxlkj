use crate::models::view::ViewRow;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    name: &str,
    filter_json: &serde_json::Value,
    sort_json: &serde_json::Value,
    created_by: Uuid,
) -> Result<ViewRow, sqlx::Error> {
    sqlx::query_as::<_, ViewRow>(
        "INSERT INTO views (id, workspace_id, name, filter_json, sort_json, created_by)
         VALUES ($1,$2,$3,$4,$5,$6) RETURNING *",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(name)
    .bind(filter_json)
    .bind(sort_json)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

pub async fn list(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<ViewRow>, sqlx::Error> {
    sqlx::query_as::<_, ViewRow>(
        "SELECT * FROM views WHERE workspace_id = $1 ORDER BY created_at",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    filter_json: &serde_json::Value,
    sort_json: &serde_json::Value,
) -> Result<Option<ViewRow>, sqlx::Error> {
    sqlx::query_as::<_, ViewRow>(
        "UPDATE views SET name=$2, filter_json=$3, sort_json=$4, updated_at=now()
         WHERE id=$1 RETURNING *",
    )
    .bind(id)
    .bind(name)
    .bind(filter_json)
    .bind(sort_json)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM views WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(r.rows_affected() > 0)
}
