use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const FIXED_ADMIN_USERNAME: &str = "admin";
pub const DEFAULT_SESSION_TIMEOUT_MINUTES: i32 = 24 * 60;
pub const MIN_SESSION_TIMEOUT_MINUTES: i32 = 5;
pub const MAX_SESSION_TIMEOUT_MINUTES: i32 = 7 * 24 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

impl AdminUser {
    pub fn fixed(password_hash: String) -> Self {
        Self {
            id: 1,
            username: FIXED_ADMIN_USERNAME.to_owned(),
            password_hash,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: Uuid,
    pub admin_id: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl SessionRecord {
    pub fn new_with_timeout_minutes(
        id: Uuid,
        admin_id: i64,
        created_at: DateTime<Utc>,
        timeout_minutes: i32,
    ) -> Self {
        Self {
            id,
            admin_id,
            created_at,
            expires_at: created_at + Duration::minutes(timeout_minutes as i64),
        }
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.expires_at <= now
    }
}

pub fn normalize_session_timeout_minutes(value: i32) -> i32 {
    value.clamp(MIN_SESSION_TIMEOUT_MINUTES, MAX_SESSION_TIMEOUT_MINUTES)
}
