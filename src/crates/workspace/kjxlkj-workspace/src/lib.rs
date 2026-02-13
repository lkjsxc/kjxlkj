use kjxlkj_db::repos;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{
    ensure_workspace_member_read, ensure_workspace_member_write, RbacError,
};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WorkspaceService {
    pool: PgPool,
}

#[derive(Debug, Error)]
pub enum WorkspaceServiceError {
    #[error("forbidden")]
    Forbidden,
    #[error("role parse failed")]
    InvalidRole,
    #[error("database failure")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceMemberView {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: String,
    pub email: String,
    pub display_name: String,
}

impl WorkspaceService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_members(
        &self,
        actor_id: Uuid,
        workspace_id: Uuid,
    ) -> Result<Vec<WorkspaceMemberView>, WorkspaceServiceError> {
        let actor_workspace_role = self.resolve_actor_workspace_role(actor_id, workspace_id).await?;
        ensure_workspace_member_read(actor_workspace_role).map_err(map_forbidden)?;

        let members = repos::workspaces::list_workspace_members(&self.pool, workspace_id).await?;
        Ok(members
            .into_iter()
            .map(|member| WorkspaceMemberView {
                workspace_id: member.workspace_id,
                user_id: member.user_id,
                role: member.role,
                joined_at: member
                    .joined_at
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc().to_string()),
                email: member.email,
                display_name: member.display_name,
            })
            .collect())
    }

    pub async fn upsert_member(
        &self,
        actor_id: Uuid,
        workspace_id: Uuid,
        user_id: Uuid,
        role: Role,
        request_id: &str,
    ) -> Result<(), WorkspaceServiceError> {
        let actor_workspace_role = self.resolve_actor_workspace_role(actor_id, workspace_id).await?;
        ensure_workspace_member_write(actor_workspace_role).map_err(map_forbidden)?;

        repos::workspaces::upsert_workspace_member(&self.pool, workspace_id, user_id, role.as_str())
            .await?;
        repos::audit::emit_security_event(
            &self.pool,
            request_id,
            Some(actor_id),
            Some(workspace_id),
            "workspace_membership_upserted",
            json!({
                "target_user_id": user_id,
                "role": role.as_str(),
            }),
        )
        .await?;

        Ok(())
    }

    async fn resolve_actor_workspace_role(
        &self,
        actor_id: Uuid,
        workspace_id: Uuid,
    ) -> Result<Role, WorkspaceServiceError> {
        let role_text = repos::workspaces::actor_workspace_role(&self.pool, workspace_id, actor_id)
            .await?
            .ok_or(WorkspaceServiceError::Forbidden)?;
        Role::from_str(&role_text).map_err(|_| WorkspaceServiceError::InvalidRole)
    }
}

fn map_forbidden(_: RbacError) -> WorkspaceServiceError {
    WorkspaceServiceError::Forbidden
}
