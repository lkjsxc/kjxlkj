//! Access policy enforcement

use kjxlkj_domain::{AccessScope, NoteStream};
use uuid::Uuid;

use crate::permission::{Permission, PermissionCheck};

/// Policy engine for access control
#[derive(Debug, Clone, Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    pub fn new() -> Self {
        Self
    }

    /// Check if user can access a note
    pub fn check_note_access(
        &self,
        note: &NoteStream,
        user_id: Uuid,
        workspace_id: Uuid,
        permission: Permission,
    ) -> PermissionCheck {
        // Check workspace scope
        if note.workspace_id != workspace_id {
            return PermissionCheck::deny("Note is in different workspace");
        }

        // Check access scope
        match note.access_scope {
            AccessScope::Public => {
                // Public notes: read allowed for all
                if permission == Permission::Read {
                    PermissionCheck::allow()
                } else {
                    // Write/delete require workspace membership
                    self.check_workspace_membership(user_id, workspace_id, permission)
                }
            }
            AccessScope::Workspace => {
                // Workspace notes: require membership
                self.check_workspace_membership(user_id, workspace_id, permission)
            }
            AccessScope::Private => {
                // Private notes: owner only
                if user_id == note.workspace_id {
                    PermissionCheck::allow()
                } else {
                    PermissionCheck::deny("Note is private to owner")
                }
            }
        }
    }

    fn check_workspace_membership(
        &self,
        _user_id: Uuid,
        _workspace_id: Uuid,
        permission: Permission,
    ) -> PermissionCheck {
        // Simplified: assume all authenticated users are workspace members
        // In production, check actual membership table
        match permission {
            Permission::Read => PermissionCheck::allow(),
            Permission::Write => PermissionCheck::allow(),
            Permission::Delete => PermissionCheck::allow(),
            Permission::Admin => PermissionCheck::deny("Admin requires explicit role"),
        }
    }
}
