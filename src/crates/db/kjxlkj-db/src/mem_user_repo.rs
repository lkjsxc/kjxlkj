/// In-memory UserRepo and SessionRepo implementations.
///
/// Spec: /docs/spec/security/auth.md
/// Spec: /docs/spec/security/sessions.md
use crate::user_repo::{SessionRecord, SessionRepo, UserRecord, UserRepo};
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Thread-safe in-memory user store.
pub struct InMemoryUserRepo {
    users: RwLock<HashMap<Uuid, UserRecord>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryUserRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepo for InMemoryUserRepo {
    fn create_user(&self, user: &UserRecord) -> Result<(), DomainError> {
        let mut users = self.users.write().unwrap();
        if users.values().any(|u| u.username == user.username) {
            return Err(DomainError::BadRequest(
                "username already exists".into(),
            ));
        }
        users.insert(user.id, user.clone());
        Ok(())
    }

    fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserRecord>, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.username == username).cloned())
    }

    fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserRecord>, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.get(&id).cloned())
    }

    fn user_count(&self) -> Result<i64, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.len() as i64)
    }
}

/// Thread-safe in-memory session store.
pub struct InMemorySessionRepo {
    sessions: RwLock<HashMap<String, SessionRecord>>,
}

impl InMemorySessionRepo {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemorySessionRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionRepo for InMemorySessionRepo {
    fn create_session(&self, session: &SessionRecord) -> Result<(), DomainError> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session.token.clone(), session.clone());
        Ok(())
    }

    fn get_session_by_token(
        &self,
        token: &str,
    ) -> Result<Option<SessionRecord>, DomainError> {
        let sessions = self.sessions.read().unwrap();
        Ok(sessions.get(token).cloned())
    }

    fn delete_session(&self, token: &str) -> Result<(), DomainError> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(token);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_domain::permission::Role;

    fn make_user(username: &str) -> UserRecord {
        UserRecord {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash: "hash".into(),
            role: Role::Owner,
            disabled: false,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_user_crud() {
        let repo = InMemoryUserRepo::new();
        assert_eq!(repo.user_count().unwrap(), 0);
        let user = make_user("admin");
        repo.create_user(&user).unwrap();
        assert_eq!(repo.user_count().unwrap(), 1);
        let found = repo.get_user_by_username("admin").unwrap().unwrap();
        assert_eq!(found.id, user.id);
    }

    #[test]
    fn test_duplicate_username_rejected() {
        let repo = InMemoryUserRepo::new();
        repo.create_user(&make_user("admin")).unwrap();
        assert!(repo.create_user(&make_user("admin")).is_err());
    }

    #[test]
    fn test_session_lifecycle() {
        let repo = InMemorySessionRepo::new();
        let session = SessionRecord {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token: "tok-123".into(),
            csrf_token: "csrf-abc".into(),
            role: Role::Owner,
            expires_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        repo.create_session(&session).unwrap();
        assert!(repo.get_session_by_token("tok-123").unwrap().is_some());
        repo.delete_session("tok-123").unwrap();
        assert!(repo.get_session_by_token("tok-123").unwrap().is_none());
    }
}
