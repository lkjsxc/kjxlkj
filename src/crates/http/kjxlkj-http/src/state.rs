/// Application state for dependency injection.
///
/// Holds shared references to all services and repositories.
/// Passed as axum State to all route handlers.
///
/// Spec: /docs/spec/architecture/runtime.md
use crate::metrics::Metrics;
use crate::rate_limit::{RateLimitConfig, RateLimiter};
use kjxlkj_db::mem_attachment_repo::InMemoryAttachmentRepo;
use kjxlkj_db::mem_export_repo::InMemoryExportRepo;
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
    /// Rate limiter for auth endpoints per IMP-SEC-02
    pub auth_rate_limiter: Arc<RateLimiter>,
    /// Attachment repository per /docs/spec/domain/attachments.md
    pub attachment_repo: Arc<InMemoryAttachmentRepo>,
    /// Request metrics per /docs/spec/technical/performance.md (IMP-OPS-02)
    pub metrics: Arc<Metrics>,
    /// Export job repository per /docs/spec/domain/export.md
    pub export_repo: Arc<InMemoryExportRepo>,
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
            auth_rate_limiter: Arc::new(RateLimiter::new(RateLimitConfig::default())),
            attachment_repo: Arc::new(InMemoryAttachmentRepo::new()),
            metrics: Arc::new(Metrics::new()),
            export_repo: Arc::new(InMemoryExportRepo::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
