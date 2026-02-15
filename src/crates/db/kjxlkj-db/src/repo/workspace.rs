//! Workspace/project/view/member repository per /docs/spec/domain/workspaces.md
//! and /docs/spec/domain/projects.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct WorkspaceRow {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct MemberRow {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, FromRow)]
pub struct ProjectRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: String,
    pub archived: bool,
    pub created_at: String,
}

#[derive(Debug, FromRow)]
pub struct ViewRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub query_json: serde_json::Value,
    pub sort: String,
    pub filters: serde_json::Value,
    pub owner_user_id: Uuid,
    pub created_at: String,
}

#[derive(Debug, FromRow)]
pub struct WidgetRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
}

// --- Workspaces ---

pub async fn create_workspace(
    pool: &PgPool, id: Uuid, slug: &str, name: &str, owner_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspaces (id, slug, name, owner_user_id) VALUES ($1,$2,$3,$4)",
    )
    .bind(id).bind(slug).bind(name).bind(owner_id)
    .execute(pool).await?;
    // Auto-add owner as member
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role) VALUES ($1,$2,'owner')",
    )
    .bind(id).bind(owner_id)
    .execute(pool).await?;
    Ok(())
}

pub async fn list_workspaces(pool: &PgPool) -> Result<Vec<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT id, slug, name, owner_user_id, \
         created_at::text as created_at, \
         deleted_at::text as deleted_at \
         FROM workspaces WHERE deleted_at IS NULL ORDER BY created_at"
    ).fetch_all(pool).await
}

pub async fn find_workspace(pool: &PgPool, id: Uuid) -> Result<Option<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        "SELECT id, slug, name, owner_user_id, \
         created_at::text as created_at, \
         deleted_at::text as deleted_at \
         FROM workspaces WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).fetch_optional(pool).await
}

pub async fn update_workspace(
    pool: &PgPool, id: Uuid, name: &str, slug: &str,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE workspaces SET name=$1, slug=$2 WHERE id=$3 AND deleted_at IS NULL")
        .bind(name).bind(slug).bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn delete_workspace(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE workspaces SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL")
        .bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// --- Members ---

pub async fn list_members(pool: &PgPool, ws_id: Uuid) -> Result<Vec<MemberRow>, sqlx::Error> {
    sqlx::query_as::<_, MemberRow>(
        "SELECT workspace_id, user_id, role, \
         joined_at::text as joined_at \
         FROM workspace_members WHERE workspace_id = $1"
    ).bind(ws_id).fetch_all(pool).await
}

pub async fn upsert_member(
    pool: &PgPool, ws_id: Uuid, user_id: Uuid, role: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role) VALUES ($1,$2,$3) \
         ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = $3"
    ).bind(ws_id).bind(user_id).bind(role).execute(pool).await?;
    Ok(())
}

pub async fn get_member_role(
    pool: &PgPool, ws_id: Uuid, user_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT role FROM workspace_members WHERE workspace_id=$1 AND user_id=$2"
    ).bind(ws_id).bind(user_id).fetch_optional(pool).await?;
    Ok(row.map(|r| r.0))
}

pub async fn remove_member(
    pool: &PgPool, ws_id: Uuid, user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "DELETE FROM workspace_members WHERE workspace_id=$1 AND user_id=$2"
    ).bind(ws_id).bind(user_id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// --- Projects ---

pub async fn create_project(
    pool: &PgPool, id: Uuid, ws_id: Uuid, name: &str, desc: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO projects (id, workspace_id, name, description) VALUES ($1,$2,$3,$4)"
    ).bind(id).bind(ws_id).bind(name).bind(desc).execute(pool).await?;
    Ok(())
}

pub async fn list_projects(pool: &PgPool, ws_id: Uuid) -> Result<Vec<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        "SELECT id, workspace_id, name, description, archived, \
         created_at::text as created_at \
         FROM projects WHERE workspace_id = $1 AND archived = FALSE ORDER BY name"
    ).bind(ws_id).fetch_all(pool).await
}

pub async fn update_project(
    pool: &PgPool, id: Uuid, name: &str, desc: &str,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE projects SET name=$1, description=$2 WHERE id=$3")
        .bind(name).bind(desc).bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn delete_project(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("UPDATE projects SET archived=TRUE WHERE id=$1")
        .bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// --- Saved Views ---

pub async fn create_view(
    pool: &PgPool, id: Uuid, ws_id: Uuid, name: &str,
    query_json: &serde_json::Value, sort: &str,
    filters: &serde_json::Value, owner_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO saved_views (id,workspace_id,name,query_json,sort,filters,owner_user_id) \
         VALUES ($1,$2,$3,$4,$5,$6,$7)"
    ).bind(id).bind(ws_id).bind(name).bind(query_json)
     .bind(sort).bind(filters).bind(owner_id)
     .execute(pool).await?;
    Ok(())
}

pub async fn list_views(pool: &PgPool, ws_id: Uuid) -> Result<Vec<ViewRow>, sqlx::Error> {
    sqlx::query_as::<_, ViewRow>(
        "SELECT id, workspace_id, name, query_json, sort, filters, \
         owner_user_id, created_at::text as created_at \
         FROM saved_views WHERE workspace_id = $1 ORDER BY name"
    ).bind(ws_id).fetch_all(pool).await
}

pub async fn update_view(
    pool: &PgPool, id: Uuid, name: &str,
    query_json: &serde_json::Value, sort: &str, filters: &serde_json::Value,
) -> Result<bool, sqlx::Error> {
    let r = sqlx::query(
        "UPDATE saved_views SET name=$1,query_json=$2,sort=$3,filters=$4 WHERE id=$5"
    ).bind(name).bind(query_json).bind(sort).bind(filters).bind(id)
     .execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn delete_view(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM saved_views WHERE id=$1")
        .bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// --- Dashboard Widgets ---

pub async fn upsert_widget(
    pool: &PgPool, id: Uuid, ws_id: Uuid, widget_type: &str,
    config_json: &serde_json::Value, layout: Option<&serde_json::Value>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO dashboard_widgets (id,workspace_id,widget_type,config_json,layout) \
         VALUES ($1,$2,$3,$4,$5) \
         ON CONFLICT (id) DO UPDATE SET config_json=$4, layout=$5"
    ).bind(id).bind(ws_id).bind(widget_type).bind(config_json).bind(layout)
     .execute(pool).await?;
    Ok(())
}

pub async fn list_widgets(pool: &PgPool, ws_id: Uuid) -> Result<Vec<WidgetRow>, sqlx::Error> {
    sqlx::query_as::<_, WidgetRow>(
        "SELECT id, workspace_id, widget_type, config_json, layout \
         FROM dashboard_widgets WHERE workspace_id = $1"
    ).bind(ws_id).fetch_all(pool).await
}
