use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use kjxlkj::core::auth::SessionRecord;
use kjxlkj::error::AppError;
use kjxlkj::web::state::SessionStore;

#[derive(Clone, Default)]
pub struct MockSessionStore {
    inner: Arc<Mutex<HashMap<Uuid, SessionRecord>>>,
}

impl MockSessionStore {
    pub fn has_session(&self, session_id: Uuid) -> bool {
        self.inner
            .lock()
            .expect("session lock poisoned")
            .contains_key(&session_id)
    }
}

#[async_trait]
impl SessionStore for MockSessionStore {
    async fn create_session(
        &self,
        admin_id: i64,
        timeout_minutes: i32,
    ) -> Result<SessionRecord, AppError> {
        let session = SessionRecord::new_with_timeout_minutes(
            Uuid::new_v4(),
            admin_id,
            Utc::now(),
            i64::from(timeout_minutes),
        );
        self.inner
            .lock()
            .expect("session lock poisoned")
            .insert(session.id, session.clone());
        Ok(session)
    }

    async fn lookup_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, AppError> {
        let sessions = self.inner.lock().expect("session lock poisoned");
        Ok(sessions.get(&session_id).cloned())
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<bool, AppError> {
        let removed = self
            .inner
            .lock()
            .expect("session lock poisoned")
            .remove(&session_id);
        Ok(removed.is_some())
    }

    async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError> {
        let mut sessions = self.inner.lock().expect("session lock poisoned");
        let before = sessions.len();
        sessions.retain(|_, record| record.expires_at > now);
        Ok((before - sessions.len()) as u64)
    }
}
