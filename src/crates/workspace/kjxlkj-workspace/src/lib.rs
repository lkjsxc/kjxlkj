//! Workspace services for kjxlkj.
//!
//! This crate contains workspace-level business logic and ownership invariants.

use uuid::Uuid;
use thiserror::Error;

use kjxlkj_domain::{Workspace, WorkspaceMembership, User, GlobalRole};

/// Workspace errors.
#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("workspace not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
}

/// Check ownership invariants.
pub fn check_ownership_invariants(
    workspace: &Workspace,
    membership: Option<&WorkspaceMembership>,
) -> Result<(), WorkspaceError> {
    if !workspace.is_active {
        return Err(WorkspaceError::InvalidOperation("workspace is not active".to_string()));
    }

    if membership.is_none() {
        return Err(WorkspaceError::AccessDenied);
    }

    Ok(())
}

/// Check if user can manage workspace.
pub fn can_manage_workspace(user: &User, membership: Option<&WorkspaceMembership>) -> bool {
    if user.global_role == GlobalRole::Owner {
        return true;
    }

    membership.map(|m| m.is_admin()).unwrap_or(false)
}

/// Check if user can access workspace.
pub fn can_access_workspace(user: &User, membership: Option<&WorkspaceMembership>) -> bool {
    if user.global_role == GlobalRole::Owner {
        return true;
    }

    membership.is_some()
}

/// Validate workspace slug.
pub fn validate_slug(slug: &str) -> Result<(), WorkspaceError> {
    if slug.is_empty() {
        return Err(WorkspaceError::InvalidOperation("slug cannot be empty".to_string()));
    }

    if slug.len() > 64 {
        return Err(WorkspaceError::InvalidOperation("slug too long".to_string()));
    }

    if !slug.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(WorkspaceError::InvalidOperation(
            "slug can only contain alphanumeric characters, hyphens, and underscores".to_string(),
        ));
    }

    Ok(())
}

/// Generate a default slug from name.
pub fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_domain::WorkspaceRole;

    #[test]
    fn test_validate_slug() {
        assert!(validate_slug("my-workspace").is_ok());
        assert!(validate_slug("my_workspace").is_ok());
        assert!(validate_slug("workspace123").is_ok());
        assert!(validate_slug("").is_err());
        assert!(validate_slug("invalid!slug").is_err());
    }

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("My Workspace"), "my-workspace");
        assert_eq!(generate_slug("Test 123"), "test-123");
    }

    #[test]
    fn test_can_manage_workspace() {
        let owner = User::new("owner@test.com".to_string(), "hash".to_string(), GlobalRole::Owner);
        let admin_membership = WorkspaceMembership::new(Uuid::nil(), Uuid::nil(), WorkspaceRole::Admin);
        let viewer_membership = WorkspaceMembership::new(Uuid::nil(), Uuid::nil(), WorkspaceRole::Viewer);

        assert!(can_manage_workspace(&owner, None));
        assert!(can_manage_workspace(&owner, Some(&viewer_membership)));
        assert!(can_manage_workspace(&owner, Some(&admin_membership)));
    }
}
