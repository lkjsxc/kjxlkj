/// User repository for auth persistence.
///
/// Spec: /docs/spec/security/auth.md
/// Spec: /docs/spec/security/sessions.md
use chrono::NaiveDateTime;
use kjxlkj_domain::permission::Role;
use kjxlkj_domain::DomainError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub disabled: bool,
    pub created_at: NaiveDateTime,
}

/// Session record per /docs/spec/security/sessions.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    /// CSRF token bound to this session per /docs/spec/security/csrf.md
    pub csrf_token: String,
    pub role: Role,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

/// User repository trait
pub trait UserRepo: Send + Sync {
    fn create_user(&self, user: &UserRecord) -> Result<(), DomainError>;
    fn get_user_by_username(&self, username: &str) -> Result<Option<UserRecord>, DomainError>;
    fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserRecord>, DomainError>;
    fn user_count(&self) -> Result<i64, DomainError>;
}

/// Session repository trait
pub trait SessionRepo: Send + Sync {
    fn create_session(&self, session: &SessionRecord) -> Result<(), DomainError>;
    fn get_session_by_token(&self, token: &str) -> Result<Option<SessionRecord>, DomainError>;
    fn delete_session(&self, token: &str) -> Result<(), DomainError>;
}
