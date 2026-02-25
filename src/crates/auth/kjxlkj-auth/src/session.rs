//! Session management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session token
pub type SessionToken = String;

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub token: SessionToken,
    pub csrf_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: Uuid, csrf_token: String, ttl_hours: u64) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(ttl_hours as i64);
        Self {
            session_id: Uuid::new_v4(),
            user_id,
            token: Uuid::new_v4().to_string(),
            csrf_token,
            expires_at,
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Session store (in-memory)
#[derive(Debug, Clone, Default)]
pub struct SessionStore {
    sessions: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<SessionToken, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn create(&self, session: Session) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.token.clone(), session);
    }

    pub async fn get(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(token).cloned()
    }

    pub async fn delete(&self, token: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(token);
    }

    pub async fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();
        sessions.retain(|_, s| s.expires_at > now);
    }
}
