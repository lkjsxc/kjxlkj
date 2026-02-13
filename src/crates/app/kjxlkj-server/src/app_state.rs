use crate::rate_limit::FixedWindowRateLimiter;
use kjxlkj_workspace::WorkspaceService;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub workspace_service: WorkspaceService,
    pub auth_rate_limiter: Arc<FixedWindowRateLimiter>,
    pub automation_rate_limiter: Arc<FixedWindowRateLimiter>,
    pub secure_cookies: bool,
    ws_ack_cursor: Arc<Mutex<HashMap<(Uuid, String), i32>>>,
}

impl AppState {
    pub fn new(pool: PgPool, secure_cookies: bool) -> Self {
        let workspace_service = WorkspaceService::new(pool.clone());
        Self {
            pool,
            workspace_service,
            auth_rate_limiter: Arc::new(FixedWindowRateLimiter::new(20, 60)),
            automation_rate_limiter: Arc::new(FixedWindowRateLimiter::new(60, 60)),
            secure_cookies,
            ws_ack_cursor: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn ws_ack_cursor(&self, user_id: Uuid, stream_id: &str) -> i32 {
        self.ws_ack_cursor
            .lock()
            .expect("ws cursor mutex poisoned")
            .get(&(user_id, stream_id.to_owned()))
            .copied()
            .unwrap_or(0)
    }

    pub fn set_ws_ack_cursor(&self, user_id: Uuid, stream_id: &str, event_seq: i32) {
        self.ws_ack_cursor
            .lock()
            .expect("ws cursor mutex poisoned")
            .insert((user_id, stream_id.to_owned()), event_seq);
    }
}
