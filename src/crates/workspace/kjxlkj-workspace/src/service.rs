/// Workspace service per /docs/spec/domain/workspaces.md
/// Project service per /docs/spec/domain/projects.md

/// Workspace and project business logic placeholder.
/// Full DB-backed implementation requires PostgreSQL runtime.
pub struct WorkspaceService;

impl WorkspaceService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkspaceService {
    fn default() -> Self {
        Self::new()
    }
}
