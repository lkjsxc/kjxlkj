use kjxlkj_domain::ids::{UserId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow)]
pub struct WorkspaceRow {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub status: String,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct MemberRow {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: OffsetDateTime,
}

pub async fn create_workspace(
    pool: &PgPool,
    id: WorkspaceId,
    slug: &str,
    name: &str,
    owner_user_id: UserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspaces (id, slug, name, owner_user_id)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id.0)
    .bind(slug)
    .bind(name)
    .bind(owner_user_id.0)
    .execute(pool)
    .await?;
    // Auto-add owner as member per /docs/spec/domain/workspaces.md
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, 'owner')
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = 'owner'",
    )
    .bind(id.0)
    .bind(owner_user_id.0)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_workspace(
    pool: &PgPool,
    id: WorkspaceId,
) -> Result<Option<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT id, slug, name, owner_user_id, status, created_at
         FROM workspaces WHERE id = $1 AND status != 'deleted'",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

pub async fn list_workspaces(pool: &PgPool) -> Result<Vec<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT id, slug, name, owner_user_id, status, created_at
         FROM workspaces WHERE status != 'deleted'
         ORDER BY created_at",
    )
    .fetch_all(pool)
    .await
}

pub async fn update_workspace(
    pool: &PgPool,
    id: WorkspaceId,
    name: &str,
    slug: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE workspaces SET name = $2, slug = $3
         WHERE id = $1 AND status = 'active'",
    )
    .bind(id.0)
    .bind(name)
    .bind(slug)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_workspace(
    pool: &PgPool,
    id: WorkspaceId,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE workspaces SET status = 'deleted' WHERE id = $1",
    )
    .bind(id.0)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn upsert_member(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    user_id: UserId,
    role: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, $3)
         ON CONFLICT (workspace_id, user_id)
         DO UPDATE SET role = EXCLUDED.role",
    )
    .bind(workspace_id.0)
    .bind(user_id.0)
    .bind(role)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_member(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    user_id: UserId,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM workspace_members
         WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id.0)
    .bind(user_id.0)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn list_members(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<Vec<MemberRow>, sqlx::Error> {
    sqlx::query_as::<_, MemberRow>(
        "SELECT workspace_id, user_id, role, joined_at
         FROM workspace_members WHERE workspace_id = $1
         ORDER BY joined_at",
    )
    .bind(workspace_id.0)
    .fetch_all(pool)
    .await
}

pub async fn find_member(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    user_id: UserId,
) -> Result<Option<MemberRow>, sqlx::Error> {
    sqlx::query_as::<_, MemberRow>(
        "SELECT workspace_id, user_id, role, joined_at
         FROM workspace_members
         WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id.0)
    .bind(user_id.0)
    .fetch_optional(pool)
    .await
}
