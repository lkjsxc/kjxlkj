use crate::models::{DbWorkspace, DbWorkspaceMemberView};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_workspace(
    pool: &PgPool,
    owner_user_id: Uuid,
    slug: &str,
    name: &str,
) -> Result<DbWorkspace, sqlx::Error> {
    let workspace = sqlx::query_as::<_, DbWorkspace>(
        "INSERT INTO workspaces (id, slug, name, owner_user_id, state)
         VALUES ($1, $2, $3, $4, 'active')
         RETURNING id, slug, name, owner_user_id, state, created_at, updated_at",
    )
    .bind(Uuid::now_v7())
    .bind(slug)
    .bind(name)
    .bind(owner_user_id)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "INSERT INTO workspace_memberships (workspace_id, user_id, role)
         VALUES ($1, $2, 'owner')",
    )
    .bind(workspace.id)
    .bind(owner_user_id)
    .execute(pool)
    .await?;

    Ok(workspace)
}

pub async fn actor_workspace_role(
    pool: &PgPool,
    workspace_id: Uuid,
    actor_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        "SELECT role
         FROM workspace_memberships
         WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(actor_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_workspace_members(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<DbWorkspaceMemberView>, sqlx::Error> {
    sqlx::query_as::<_, DbWorkspaceMemberView>(
        "SELECT
            wm.workspace_id,
            wm.user_id,
            wm.role,
            wm.joined_at,
            u.email,
            u.display_name
         FROM workspace_memberships wm
         INNER JOIN users u ON u.id = wm.user_id
         WHERE wm.workspace_id = $1
         ORDER BY wm.joined_at ASC",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

pub async fn upsert_workspace_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_memberships (workspace_id, user_id, role)
         VALUES ($1, $2, $3)
         ON CONFLICT (workspace_id, user_id)
         DO UPDATE SET role = EXCLUDED.role",
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role)
    .execute(pool)
    .await?;
    Ok(())
}
