/// HTTP route tree per /docs/spec/api/http.md
///
/// This module assembles the full API router from per-resource modules.
/// Each resource module stays under 200 lines per /docs/policy/STRUCTURE.md.
use axum::{
    routing::{get, patch, post, delete},
    Router,
};

use crate::routes_auth;
use crate::routes_automation;
use crate::routes_health;
use crate::routes_note;
use crate::routes_search;
use crate::routes_workspace;

/// Build the complete API router per /docs/spec/api/http.md
pub fn api_router() -> Router {
    Router::new()
        // Auth and Session
        .route("/api/setup/register", post(routes_auth::setup_register))
        .route("/api/auth/login", post(routes_auth::auth_login))
        .route("/api/auth/logout", post(routes_auth::auth_logout))
        .route("/api/auth/session", get(routes_auth::auth_session))
        // Workspaces
        .route("/api/workspaces", get(routes_workspace::list_workspaces))
        .route("/api/workspaces", post(routes_workspace::create_workspace))
        // Notes
        .route("/api/notes", get(routes_note::list_notes))
        .route("/api/notes", post(routes_note::create_note))
        .route("/api/notes/{id}", get(routes_note::get_note))
        .route("/api/notes/{id}", patch(routes_note::patch_note))
        .route("/api/notes/{id}", delete(routes_note::delete_note))
        .route("/api/notes/{id}/title", patch(routes_note::update_title))
        .route("/api/notes/{id}/history", get(routes_note::note_history))
        .route("/api/notes/{id}/backlinks", get(routes_note::note_backlinks))
        // Search
        .route("/api/search", get(routes_search::search_notes))
        // Automation
        .route("/api/automation/rules", get(routes_automation::list_rules))
        .route("/api/automation/rules", post(routes_automation::create_rule))
        .route("/api/automation/rules/{id}", patch(routes_automation::update_rule))
        .route("/api/automation/rules/{id}/launch", post(routes_automation::launch_rule))
        .route("/api/automation/runs", get(routes_automation::list_runs))
        .route("/api/automation/runs/{id}", get(routes_automation::get_run))
        .route("/api/automation/runs/{id}/review", post(routes_automation::review_run))
        // Health
        .route("/api/healthz", get(routes_health::healthz))
        .route("/api/readyz", get(routes_health::readyz))
}
