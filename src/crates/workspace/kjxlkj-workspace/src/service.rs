//! Workspace service layer

use kjxlkj_domain::Workspace;
use uuid::Uuid;

use kjxlkj_db::{WorkspaceRepo, Result};

/// Workspace service
#[derive(Debug, Clone)]
pub struct WorkspaceService {
    repo: WorkspaceRepo,
}

impl WorkspaceService {
    pub fn new(repo: WorkspaceRepo) -> Self {
        Self { repo }
    }

    pub async fn create_workspace(&self, name: String, owner_id: Uuid) -> Result<Workspace> {
        let workspace = Workspace::new(name, owner_id);
        self.repo.create_workspace(workspace).await
    }

    pub async fn get_workspace(&self, workspace_id: Uuid) -> Result<Option<Workspace>> {
        self.repo.get_workspace(workspace_id).await
    }

    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        self.repo.list_workspaces().await
    }
}
