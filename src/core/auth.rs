use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

pub const SESSION_TTL_HOURS: i64 = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub id: Uuid,
    pub admin_id: i64,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl SessionRecord {
    pub fn new(id: Uuid, admin_id: i64, created_at: DateTime<Utc>) -> Self {
        Self::new_with_timeout_minutes(id, admin_id, created_at, SESSION_TTL_HOURS * 60)
    }

    pub fn new_with_timeout_minutes(
        id: Uuid,
        admin_id: i64,
        created_at: DateTime<Utc>,
        timeout_minutes: i64,
    ) -> Self {
        Self {
            id,
            admin_id,
            expires_at: created_at + Duration::minutes(timeout_minutes),
            created_at,
        }
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.expires_at <= now
    }
}
