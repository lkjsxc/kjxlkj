use crate::models::note::ProjectRow;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    name: &str,
) -> Result<ProjectRow, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "INSERT INTO projects (id, workspace_id, name) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn list(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "SELECT * FROM projects WHERE workspace_id = $1 AND archived = false ORDER BY created_at",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: &str,
) -> Result<Option<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "UPDATE projects SET name = $2, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(name)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE projects SET archived = true, updated_at = now() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(r.rows_affected() > 0)
}
