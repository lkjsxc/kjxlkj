/// In-memory WorkspaceRepo implementation.
///
/// Spec: /docs/spec/domain/workspaces.md
use crate::repo::WorkspaceRepo;
use kjxlkj_domain::workspace::*;
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Membership record for in-memory store.
#[derive(Debug, Clone)]
pub struct MemberRecord {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: kjxlkj_domain::permission::Role,
}

/// Thread-safe in-memory workspace store.
pub struct InMemoryWorkspaceRepo {
    workspaces: RwLock<HashMap<Uuid, Workspace>>,
    members: RwLock<Vec<MemberRecord>>,
}

impl InMemoryWorkspaceRepo {
    pub fn new() -> Self {
        Self {
            workspaces: RwLock::new(HashMap::new()),
            members: RwLock::new(Vec::new()),
        }
    }

    /// Add or update membership (idempotent by workspace_id,user_id).
    pub fn upsert_member(&self, member: MemberRecord) {
        let mut members = self.members.write().unwrap();
        if let Some(existing) = members.iter_mut().find(|m| {
            m.workspace_id == member.workspace_id && m.user_id == member.user_id
        }) {
            existing.role = member.role;
        } else {
            members.push(member);
        }
    }

    /// Remove membership.
    pub fn remove_member(&self, workspace_id: Uuid, user_id: Uuid) {
        let mut members = self.members.write().unwrap();
        members.retain(|m| {
            !(m.workspace_id == workspace_id && m.user_id == user_id)
        });
    }
}

impl Default for InMemoryWorkspaceRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceRepo for InMemoryWorkspaceRepo {
    fn create_workspace(&self, ws: &Workspace) -> Result<(), DomainError> {
        let mut workspaces = self.workspaces.write().unwrap();
        if workspaces.values().any(|w| w.slug == ws.slug) {
            return Err(DomainError::BadRequest(
                "workspace slug already exists".into(),
            ));
        }
        workspaces.insert(ws.id, ws.clone());
        Ok(())
    }

    fn list_workspaces(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Workspace>, DomainError> {
        let workspaces = self.workspaces.read().unwrap();
        let members = self.members.read().unwrap();
        let ws_ids: Vec<Uuid> = members
            .iter()
            .filter(|m| m.user_id == user_id)
            .map(|m| m.workspace_id)
            .collect();
        let results: Vec<Workspace> = workspaces
            .values()
            .filter(|w| {
                w.owner_user_id == user_id
                    || ws_ids.contains(&w.id)
            })
            .filter(|w| w.state != WorkspaceState::Deleted)
            .cloned()
            .collect();
        Ok(results)
    }

    fn get_workspace(&self, id: Uuid) -> Result<Option<Workspace>, DomainError> {
        let workspaces = self.workspaces.read().unwrap();
        Ok(workspaces.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ws(slug: &str, owner: Uuid) -> Workspace {
        let now = chrono::Utc::now().naive_utc();
        Workspace {
            id: Uuid::new_v4(),
            slug: slug.into(),
            name: slug.into(),
            owner_user_id: owner,
            state: WorkspaceState::Active,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn test_create_and_list() {
        let repo = InMemoryWorkspaceRepo::new();
        let owner = Uuid::new_v4();
        let ws = make_ws("my-notes", owner);
        repo.create_workspace(&ws).unwrap();
        let list = repo.list_workspaces(owner).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].slug, "my-notes");
    }

    #[test]
    fn test_duplicate_slug_rejected() {
        let repo = InMemoryWorkspaceRepo::new();
        let owner = Uuid::new_v4();
        repo.create_workspace(&make_ws("dup", owner)).unwrap();
        assert!(repo.create_workspace(&make_ws("dup", owner)).is_err());
    }

    #[test]
    fn test_membership_upsert() {
        let repo = InMemoryWorkspaceRepo::new();
        let ws_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        repo.upsert_member(MemberRecord {
            workspace_id: ws_id,
            user_id,
            role: kjxlkj_domain::permission::Role::Editor,
        });
        repo.upsert_member(MemberRecord {
            workspace_id: ws_id,
            user_id,
            role: kjxlkj_domain::permission::Role::Admin,
        });
        let members = repo.members.read().unwrap();
        let count = members
            .iter()
            .filter(|m| m.workspace_id == ws_id && m.user_id == user_id)
            .count();
        assert_eq!(count, 1, "upsert should be idempotent");
    }
}
