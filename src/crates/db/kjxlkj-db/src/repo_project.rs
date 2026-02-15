use kjxlkj_domain::ids::{ProjectId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ProjectRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: OffsetDateTime,
}

pub async fn create_project(
    pool: &PgPool,
    id: ProjectId,
    workspace_id: WorkspaceId,
    name: &str,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO projects (id, workspace_id, name, description)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id.0)
    .bind(workspace_id.0)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_project(
    pool: &PgPool,
    id: ProjectId,
) -> Result<Option<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "SELECT id, workspace_id, name, description, archived, created_at
         FROM projects WHERE id = $1",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn list_projects(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "SELECT id, workspace_id, name, description, archived, created_at
         FROM projects WHERE workspace_id = $1 AND archived = false
         ORDER BY created_at",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

pub async fn update_project(
    pool: &PgPool,
    id: ProjectId,
    name: &str,
    description: Option<&str>,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE projects SET name = $2, description = $3
         WHERE id = $1 AND archived = false",
    )
    .bind(id.0)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_project(
    pool: &PgPool,
    id: ProjectId,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE projects SET archived = true WHERE id = $1",
    )
    .bind(id.0)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
