//! Workspace repository.

use sqlx::SqlitePool;
use uuid::Uuid;
use time::OffsetDateTime;
use serde_json::Value as JsonValue;

use kjxlkj_domain::{Workspace, WorkspaceMembership, WorkspaceRole, Project, SavedView, ViewType};

/// Workspace repository.
pub struct WorkspaceRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> WorkspaceRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new workspace.
    pub async fn create(&self, workspace: &Workspace) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO workspaces (id, name, slug, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(workspace.id.to_string())
        .bind(&workspace.name)
        .bind(&workspace.slug)
        .bind(workspace.is_active)
        .bind(workspace.created_at)
        .bind(workspace.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Find workspace by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, name, slug, is_active, created_at, updated_at
            FROM workspaces WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(id, name, slug, is_active, created_at, updated_at)| Workspace {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            name,
            slug,
            is_active,
            created_at,
            updated_at,
        }))
    }

    /// List all workspaces.
    pub async fn list(&self) -> Result<Vec<Workspace>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, name, slug, is_active, created_at, updated_at
            FROM workspaces WHERE is_active = true ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, name, slug, is_active, created_at, updated_at)| Workspace {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                name,
                slug,
                is_active,
                created_at,
                updated_at,
            })
            .collect())
    }
}

/// Membership repository.
pub struct MembershipRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> MembershipRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create or update membership.
    pub async fn upsert(&self, membership: &WorkspaceMembership) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO workspace_memberships (workspace_id, user_id, role, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(workspace_id, user_id) DO UPDATE SET role = excluded.role, updated_at = excluded.updated_at
            "#,
        )
        .bind(membership.workspace_id.to_string())
        .bind(membership.user_id.to_string())
        .bind(serde_json::to_string(&membership.role).unwrap())
        .bind(membership.created_at)
        .bind(membership.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List members of a workspace.
    pub async fn list_members(&self, workspace_id: Uuid) -> Result<Vec<WorkspaceMembership>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT workspace_id, user_id, role, created_at, updated_at
            FROM workspace_memberships WHERE workspace_id = ?
            "#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(workspace_id, user_id, role, created_at, updated_at)| WorkspaceMembership {
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                user_id: Uuid::parse_str(&user_id).unwrap_or_default(),
                role: serde_json::from_str(&role).unwrap_or_default(),
                created_at,
                updated_at,
            })
            .collect())
    }

    /// Find membership for user in workspace.
    pub async fn find_membership(&self, workspace_id: Uuid, user_id: Uuid) -> Result<Option<WorkspaceMembership>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT workspace_id, user_id, role, created_at, updated_at
            FROM workspace_memberships WHERE workspace_id = ? AND user_id = ?
            "#,
        )
        .bind(workspace_id.to_string())
        .bind(user_id.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|(workspace_id, user_id, role, created_at, updated_at)| WorkspaceMembership {
            workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
            user_id: Uuid::parse_str(&user_id).unwrap_or_default(),
            role: serde_json::from_str(&role).unwrap_or_default(),
            created_at,
            updated_at,
        }))
    }
}

/// Project repository.
pub struct ProjectRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ProjectRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new project.
    pub async fn create(&self, project: &Project) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, workspace_id, name, description, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id.to_string())
        .bind(project.workspace_id.to_string())
        .bind(&project.name)
        .bind(&project.description)
        .bind(project.is_active)
        .bind(project.created_at)
        .bind(project.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List projects for workspace.
    pub async fn list_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, bool, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, workspace_id, name, description, is_active, created_at, updated_at
            FROM projects WHERE workspace_id = ? AND is_active = true ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, workspace_id, name, description, is_active, created_at, updated_at)| Project {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                name,
                description,
                is_active,
                created_at,
                updated_at,
            })
            .collect())
    }
}

/// Saved view repository.
pub struct SavedViewRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SavedViewRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a saved view.
    pub async fn create(&self, view: &SavedView) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO saved_views (id, workspace_id, name, view_type, filters, sort, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(view.id.to_string())
        .bind(view.workspace_id.to_string())
        .bind(&view.name)
        .bind(serde_json::to_string(&view.view_type).unwrap())
        .bind(serde_json::to_string(&view.filters).unwrap())
        .bind(&view.sort)
        .bind(view.created_by.to_string())
        .bind(view.created_at)
        .bind(view.updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// List saved views for workspace.
    pub async fn list_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<SavedView>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, String, OffsetDateTime, OffsetDateTime)>(
            r#"
            SELECT id, workspace_id, name, view_type, filters, sort, created_by, created_at, updated_at
            FROM saved_views WHERE workspace_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, workspace_id, name, view_type, filters, sort, created_by, created_at, updated_at)| SavedView {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                workspace_id: Uuid::parse_str(&workspace_id).unwrap_or_default(),
                name,
                view_type: serde_json::from_str(&view_type).unwrap_or_default(),
                filters: serde_json::from_str(&filters).unwrap_or(JsonValue::Null),
                sort,
                created_by: Uuid::parse_str(&created_by).unwrap_or_default(),
                created_at,
                updated_at,
            })
            .collect())
    }
}
