//! HTTP route definitions

use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use crate::handlers::{
    note, workspace, search, auth,
};
use kjxlkj_db::DbPool;
use kjxlkj_auth::SessionStore;

/// Application state for HTTP handlers
#[derive(Debug, Clone)]
pub struct HttpState {
    pub db_pool: DbPool,
    pub session_store: SessionStore,
}

/// Create the API router
pub fn create_router(state: HttpState) -> Router {
    let api = Router::new()
        // Auth routes
        .route("/auth/session", get(auth::get_session))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/setup/register", post(auth::register))
        // Workspace routes
        .route("/workspaces", get(workspace::list_workspaces))
        .route("/workspaces", post(workspace::create_workspace))
        // Note routes
        .route("/notes", get(note::list_notes))
        .route("/notes", post(note::create_note))
        .route("/notes/:id", get(note::get_note))
        .route("/notes/:id", patch(note::update_note))
        .route("/notes/:id/title", patch(note::update_title))
        .route("/notes/:id", delete(note::delete_note))
        .route("/notes/:id/history", get(note::get_history))
        .route("/notes/:id/backlinks", get(note::get_backlinks))
        // Search routes
        .route("/search", get(search::search));

    Router::new()
        .nest("/api", api)
        .route("/api/healthz", get(health_check))
        .route("/api/readyz", get(ready_check))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "ok"
}

async fn ready_check() -> &'static str {
    "ok"
}
