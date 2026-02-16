/// Workspace service per /docs/spec/domain/workspaces.md
/// Project service per /docs/spec/domain/projects.md
///
/// Enforces ownership, slug uniqueness, and membership rules.
use kjxlkj_domain::permission::Role;
use kjxlkj_domain::workspace::*;
use kjxlkj_domain::DomainError;
use uuid::Uuid;

/// Workspace and project business logic.
pub struct WorkspaceService;

impl WorkspaceService {
    pub fn new() -> Self { Self }

    /// Validate workspace creation input.
    /// Per /docs/spec/domain/workspaces.md: slug must be non-empty.
    pub fn validate_create_input(input: &CreateWorkspaceInput) -> Result<(), DomainError> {
        if input.slug.is_empty() {
            return Err(DomainError::BadRequest("slug must not be empty".into()));
        }
        if input.slug.len() > 64 {
            return Err(DomainError::BadRequest("slug must be <= 64 chars".into()));
        }
        if !input.slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(DomainError::BadRequest("slug must be lowercase alphanumeric with hyphens".into()));
        }
        if input.name.is_empty() {
            return Err(DomainError::BadRequest("name must not be empty".into()));
        }
        Ok(())
    }

    /// Build a new workspace entity.
    pub fn build_workspace(input: &CreateWorkspaceInput, owner_id: Uuid) -> Workspace {
        let now = chrono::Utc::now().naive_utc();
        Workspace {
            id: Uuid::new_v4(),
            slug: input.slug.clone(),
            name: input.name.clone(),
            owner_user_id: owner_id,
            state: WorkspaceState::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if a user can archive a workspace.
    /// Per /docs/spec/domain/workspaces.md: only owner/admin can archive.
    pub fn can_archive(role: Role) -> Result<(), DomainError> {
        if role.can_manage() { Ok(()) } else { Err(DomainError::RoleForbidden) }
    }

    /// Check if a workspace state transition is valid.
    pub fn validate_state_transition(
        current: WorkspaceState,
        target: WorkspaceState,
    ) -> Result<(), DomainError> {
        match (current, target) {
            (WorkspaceState::Active, WorkspaceState::Archived) => Ok(()),
            (WorkspaceState::Active, WorkspaceState::Deleted) => Ok(()),
            (WorkspaceState::Archived, WorkspaceState::Active) => Ok(()),
            (WorkspaceState::Archived, WorkspaceState::Deleted) => Ok(()),
            _ => Err(DomainError::BadRequest(
                format!("cannot transition from {current:?} to {target:?}"),
            )),
        }
    }
}

impl Default for WorkspaceService {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_slug() {
        let ok = CreateWorkspaceInput { slug: "my-notes".into(), name: "My Notes".into() };
        assert!(WorkspaceService::validate_create_input(&ok).is_ok());
        let empty = CreateWorkspaceInput { slug: "".into(), name: "N".into() };
        assert!(WorkspaceService::validate_create_input(&empty).is_err());
        let upper = CreateWorkspaceInput { slug: "BAD".into(), name: "N".into() };
        assert!(WorkspaceService::validate_create_input(&upper).is_err());
    }

    #[test]
    fn test_state_transitions() {
        assert!(WorkspaceService::validate_state_transition(
            WorkspaceState::Active, WorkspaceState::Archived
        ).is_ok());
        assert!(WorkspaceService::validate_state_transition(
            WorkspaceState::Deleted, WorkspaceState::Active
        ).is_err());
    }

    #[test]
    fn test_build_workspace() {
        let input = CreateWorkspaceInput { slug: "ws".into(), name: "W".into() };
        let ws = WorkspaceService::build_workspace(&input, Uuid::new_v4());
        assert_eq!(ws.slug, "ws");
        assert_eq!(ws.state, WorkspaceState::Active);
    }

    #[test]
    fn test_archive_permission() {
        assert!(WorkspaceService::can_archive(Role::Owner).is_ok());
        assert!(WorkspaceService::can_archive(Role::Admin).is_ok());
        assert!(WorkspaceService::can_archive(Role::Editor).is_err());
        assert!(WorkspaceService::can_archive(Role::Viewer).is_err());
    }
}
