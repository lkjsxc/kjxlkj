//! User domain types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::types::GlobalRole;

/// User entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub global_role: GlobalRole,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    /// Create a new user.
    pub fn new(email: String, password_hash: String, global_role: GlobalRole) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            display_name: None,
            global_role,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if user is an owner.
    pub fn is_owner(&self) -> bool {
        self.global_role == GlobalRole::Owner
    }

    /// Check if user is an admin or higher.
    pub fn is_admin(&self) -> bool {
        matches!(self.global_role, GlobalRole::Owner | GlobalRole::Admin)
    }
}

/// User session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

impl Session {
    /// Create a new session.
    pub fn new(user_id: Uuid, token: String, expires_at: OffsetDateTime) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            expires_at,
            created_at: now,
        }
    }

    /// Check if session is expired.
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() >= self.expires_at
    }
}

/// Security event for audit logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: SecurityEventType,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: OffsetDateTime,
}

/// Security event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    SetupCompleted,
    LoginSuccess,
    LoginFailure,
    Logout,
    RoleChanged,
    MembershipChanged,
    SessionExpired,
    RateLimited,
}
