//! Project service

use kjxlkj_domain::Project;
use uuid::Uuid;

use kjxlkj_db::{WorkspaceRepo, Result};

/// Project service
#[derive(Debug, Clone)]
pub struct ProjectService {
    repo: WorkspaceRepo,
}

impl ProjectService {
    pub fn new(repo: WorkspaceRepo) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, name: String, workspace_id: Uuid) -> Result<Project> {
        let project = Project::new(name, workspace_id);
        self.repo.create_project(project).await
    }

    pub async fn get_project(&self, project_id: Uuid) -> Result<Option<Project>> {
        self.repo.get_project(project_id).await
    }

    pub async fn list_projects(&self, workspace_id: Uuid) -> Result<Vec<Project>> {
        self.repo.list_projects(workspace_id).await
    }
}
