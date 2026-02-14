// Workspace repository per /docs/spec/domain/workspaces.md
use kjxlkj_domain::types::{Role, Workspace, WorkspaceMember};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_workspace(pool: &PgPool, ws: &Workspace) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspaces (id, slug, name, owner_user_id, created_at)
         VALUES ($1, $2, $3, $4, now())",
    )
    .bind(ws.id)
    .bind(&ws.slug)
    .bind(&ws.name)
    .bind(ws.owner_user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
    let row: Option<(Uuid, String, String, Uuid)> = sqlx::query_as(
        "SELECT id, slug, name, owner_user_id FROM workspaces WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| Workspace {
        id: r.0, slug: r.1, name: r.2, owner_user_id: r.3, created_at: String::new(),
    }))
}

pub async fn list_workspaces(pool: &PgPool) -> Result<Vec<Workspace>, sqlx::Error> {
    let rows: Vec<(Uuid, String, String, Uuid)> = sqlx::query_as(
        "SELECT id, slug, name, owner_user_id FROM workspaces ORDER BY created_at",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| Workspace {
        id: r.0, slug: r.1, name: r.2, owner_user_id: r.3, created_at: String::new(),
    }).collect())
}

pub async fn update_workspace(pool: &PgPool, id: Uuid, name: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE workspaces SET name = $1 WHERE id = $2")
        .bind(name)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_workspace(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM workspaces WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn upsert_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: Role,
) -> Result<(), sqlx::Error> {
    let role_str = match role {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Editor => "editor",
        Role::Viewer => "viewer",
    };
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role, joined_at)
         VALUES ($1, $2, $3, now())
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = $3",
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role_str)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_members(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<WorkspaceMember>, sqlx::Error> {
    let rows: Vec<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT workspace_id, user_id, role FROM workspace_members
         WHERE workspace_id = $1",
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| {
        let role = match r.2.as_str() {
            "owner" => Role::Owner,
            "admin" => Role::Admin,
            "editor" => Role::Editor,
            _ => Role::Viewer,
        };
        WorkspaceMember {
            workspace_id: r.0,
            user_id: r.1,
            role,
            joined_at: String::new(),
        }
    }).collect())
}

pub async fn remove_member(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
