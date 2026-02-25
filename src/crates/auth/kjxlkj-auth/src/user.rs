//! User entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_owner: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String, is_owner: bool) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            email,
            password_hash,
            is_owner,
            created_at: Utc::now(),
        }
    }
}
