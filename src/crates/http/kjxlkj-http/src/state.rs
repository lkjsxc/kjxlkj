/// Application state for dependency injection.
///
/// Holds shared references to all services and repositories.
/// Passed as axum State to all route handlers.
///
/// Spec: /docs/spec/architecture/runtime.md
use kjxlkj_db::mem_automation_repo::InMemoryAutomationRepo;
use kjxlkj_db::mem_note_repo::InMemoryNoteRepo;
use kjxlkj_db::mem_search_repo::InMemorySearchRepo;
use kjxlkj_db::mem_user_repo::{InMemorySessionRepo, InMemoryUserRepo};
use kjxlkj_db::mem_workspace_repo::InMemoryWorkspaceRepo;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub note_repo: Arc<InMemoryNoteRepo>,
    pub user_repo: Arc<InMemoryUserRepo>,
    pub session_repo: Arc<InMemorySessionRepo>,
    pub workspace_repo: Arc<InMemoryWorkspaceRepo>,
    pub automation_repo: Arc<InMemoryAutomationRepo>,
    pub search_repo: Arc<InMemorySearchRepo>,
    /// Idempotency key cache for WS patches (key -> (note_id, version, seq))
    pub idempotency_keys: Arc<RwLock<HashMap<String, IdempotencyRecord>>>,
}

/// Stored idempotency result per /docs/spec/api/websocket.md WS-04
#[derive(Debug, Clone)]
pub struct IdempotencyRecord {
    pub note_id: uuid::Uuid,
    pub version: i64,
    pub event_seq: i64,
}

impl AppState {
    /// Create a fresh AppState with empty in-memory stores.
    pub fn new() -> Self {
        Self {
            note_repo: Arc::new(InMemoryNoteRepo::new()),
            user_repo: Arc::new(InMemoryUserRepo::new()),
            session_repo: Arc::new(InMemorySessionRepo::new()),
            workspace_repo: Arc::new(InMemoryWorkspaceRepo::new()),
            automation_repo: Arc::new(InMemoryAutomationRepo::new()),
            search_repo: Arc::new(InMemorySearchRepo::new()),
            idempotency_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
