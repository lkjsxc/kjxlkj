use crate::models::workspace::{MemberRow, WorkspaceRow};
use kjxlkj_domain::types::WorkspaceRole;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    slug: &str,
    owner_id: Uuid,
) -> Result<WorkspaceRow, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "INSERT INTO workspaces (id, name, slug, owner_id)
         VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(id)
    .bind(name)
    .bind(slug)
    .bind(owner_id)
    .fetch_one(pool)
    .await
}

pub async fn list(pool: &PgPool) -> Result<Vec<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT * FROM workspaces WHERE state = 'active' ORDER BY created_at",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: &str,
) -> Result<Option<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "UPDATE workspaces SET name = $2, updated_at = now()
         WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(name)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE workspaces SET state = 'deleted', updated_at = now() WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(r.rows_affected() > 0)
}

pub async fn upsert_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: WorkspaceRole,
) -> Result<MemberRow, sqlx::Error> {
    sqlx::query_as::<_, MemberRow>(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, $3::text)
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = $3::text
         RETURNING *",
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role)
    .fetch_one(pool)
    .await
}

pub async fn list_members(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<MemberRow>, sqlx::Error> {
    sqlx::query_as::<_, MemberRow>(
        "SELECT * FROM workspace_members WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn get_member_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Option<WorkspaceRole>, sqlx::Error> {
    let row: Option<MemberRow> = sqlx::query_as(
        "SELECT * FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.role))
}
