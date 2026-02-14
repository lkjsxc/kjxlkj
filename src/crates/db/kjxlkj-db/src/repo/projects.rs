// Project repository per /docs/spec/domain/projects.md
use kjxlkj_domain::types::Project;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_project(pool: &PgPool, p: &Project) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO projects (id, workspace_id, name, description, created_at)
         VALUES ($1, $2, $3, $4, now())",
    )
    .bind(p.id)
    .bind(p.workspace_id)
    .bind(&p.name)
    .bind(&p.description)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
    let row: Option<(Uuid, Uuid, String, Option<String>)> = sqlx::query_as(
        "SELECT id, workspace_id, name, description FROM projects WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| Project {
        id: r.0, workspace_id: r.1, name: r.2, description: r.3, created_at: String::new(),
    }))
}

pub async fn list_projects(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, String, Option<String>)> = sqlx::query_as(
        "SELECT id, workspace_id, name, description FROM projects
         WHERE workspace_id = $1 ORDER BY created_at",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| Project {
        id: r.0, workspace_id: r.1, name: r.2, description: r.3, created_at: String::new(),
    }).collect())
}

pub async fn update_project(pool: &PgPool, id: Uuid, name: &str, desc: Option<&str>) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE projects SET name = $1, description = $2 WHERE id = $3")
        .bind(name)
        .bind(desc)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_project(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
