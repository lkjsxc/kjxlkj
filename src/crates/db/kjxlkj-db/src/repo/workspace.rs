//! Workspace repository - in-memory implementation

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use kjxlkj_domain::{Workspace, Project};
use crate::error::{DbError, Result};

/// In-memory workspace repository
#[derive(Debug, Clone)]
pub struct WorkspaceRepo {
    workspaces: Arc<RwLock<HashMap<Uuid, Workspace>>>,
    projects: Arc<RwLock<HashMap<Uuid, Project>>>,
}

impl WorkspaceRepo {
    pub fn new() -> Self {
        Self {
            workspaces: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create workspace
    pub async fn create_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let mut workspaces = self.workspaces.write().await;
        workspaces.insert(workspace.workspace_id, workspace.clone());
        Ok(workspace)
    }

    /// Get workspace by ID
    pub async fn get_workspace(&self, workspace_id: Uuid) -> Result<Option<Workspace>> {
        let workspaces = self.workspaces.read().await;
        Ok(workspaces.get(&workspace_id).cloned())
    }

    /// List all workspaces
    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        let workspaces = self.workspaces.read().await;
        Ok(workspaces.values().cloned().collect())
    }

    /// Create project
    pub async fn create_project(&self, project: Project) -> Result<Project> {
        let mut projects = self.projects.write().await;
        projects.insert(project.project_id, project.clone());
        Ok(project)
    }

    /// Get project by ID
    pub async fn get_project(&self, project_id: Uuid) -> Result<Option<Project>> {
        let projects = self.projects.read().await;
        Ok(projects.get(&project_id).cloned())
    }

    /// List projects for workspace
    pub async fn list_projects(&self, workspace_id: Uuid) -> Result<Vec<Project>> {
        let projects = self.projects.read().await;
        Ok(projects
            .values()
            .filter(|p| p.workspace_id == workspace_id)
            .cloned()
            .collect())
    }
}

impl Default for WorkspaceRepo {
    fn default() -> Self {
        Self::new()
    }
}
