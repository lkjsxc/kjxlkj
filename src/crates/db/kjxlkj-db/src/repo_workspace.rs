use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(FromRow, serde::Serialize)]
pub struct WorkspaceRow {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub state: String,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct MemberRow {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: OffsetDateTime,
}

/// Create workspace.
pub async fn create_workspace(
    pool: &PgPool,
    id: Uuid,
    slug: &str,
    name: &str,
    owner_user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspaces (id, slug, name, owner_user_id)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(slug)
    .bind(name)
    .bind(owner_user_id)
    .execute(pool)
    .await?;
    // Add owner as member
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, 'owner')
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = 'owner'"
    )
    .bind(id)
    .bind(owner_user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// List workspaces accessible by user.
pub async fn list_workspaces_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT w.id, w.slug, w.name, w.owner_user_id, w.state, w.created_at
         FROM workspaces w
         JOIN workspace_members m ON w.id = m.workspace_id
         WHERE m.user_id = $1 AND w.state = 'active'
         ORDER BY w.created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Get user's role in a workspace.
pub async fn get_member_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT role FROM workspace_members
         WHERE workspace_id = $1 AND user_id = $2"
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

/// Upsert workspace membership (idempotent).
pub async fn upsert_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role)
         VALUES ($1, $2, $3)
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = $3"
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove workspace membership.
pub async fn remove_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM workspace_members
         WHERE workspace_id = $1 AND user_id = $2"
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
