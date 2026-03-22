use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::core::auth::SessionRecord;
use crate::error::AppError;
use crate::web::state::SessionStore;

#[derive(Clone)]
pub struct RuntimeSessionStore {
    pub app_state: AppState,
}

#[async_trait]
impl SessionStore for RuntimeSessionStore {
    async fn create_session(
        &self,
        admin_id: i64,
        timeout_minutes: i32,
    ) -> Result<SessionRecord, AppError> {
        self.app_state
            .postgres
            .sessions()
            .create(admin_id, timeout_minutes)
            .await
    }

    async fn lookup_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, AppError> {
        self.app_state.postgres.sessions().lookup(session_id).await
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<bool, AppError> {
        self.app_state.postgres.sessions().delete(session_id).await
    }

    async fn cleanup_expired(&self, now: DateTime<Utc>) -> Result<u64, AppError> {
        self.app_state
            .postgres
            .sessions()
            .cleanup_expired(now)
            .await
    }
}
