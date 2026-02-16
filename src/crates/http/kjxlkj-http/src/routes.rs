/// HTTP route tree per /docs/spec/api/http.md
///
/// This module assembles the full API router from per-resource modules.
/// Each resource module stays under 200 lines per /docs/policy/STRUCTURE.md.
use axum::{
    middleware as axum_mw,
    routing::{get, patch, post},
    Router,
};

use crate::middleware::csrf_middleware;
use crate::routes_attachment;
use crate::routes_auth;
use crate::routes_automation;
use crate::routes_health;
use crate::routes_note;
use crate::routes_search;
use crate::routes_workspace;
use crate::state::AppState;
use crate::tracing_mw::tracing_middleware;
use crate::csp::csp_middleware;
use crate::metrics;

/// Build the complete API router per /docs/spec/api/http.md
///
/// Takes AppState so CSRF middleware can validate tokens against
/// session store per /docs/spec/security/csrf.md.
pub fn api_router(state: AppState) -> Router {
    Router::new()
        // Auth and Session
        .route("/api/setup/register", post(routes_auth::setup_register))
        .route("/api/auth/login", post(routes_auth::auth_login))
        .route("/api/auth/logout", post(routes_auth::auth_logout))
        .route("/api/auth/session", get(routes_auth::auth_session))
        // Workspaces
        .route("/api/workspaces", get(routes_workspace::list_workspaces).post(routes_workspace::create_workspace))
        // Notes — combine methods on same path, use :param (matchit 0.7)
        .route("/api/notes", get(routes_note::list_notes).post(routes_note::create_note))
        .route("/api/notes/:id", get(routes_note::get_note).patch(routes_note::patch_note).delete(routes_note::delete_note))
        .route("/api/notes/:id/title", patch(routes_note::update_title))
        .route("/api/notes/:id/history", get(routes_note::note_history))
        .route("/api/notes/:id/backlinks", get(routes_note::note_backlinks))
        // Attachments per /docs/spec/domain/attachments.md
        .route("/api/notes/:id/attachments", post(routes_attachment::upload_attachment).get(routes_attachment::list_attachments))
        .route("/api/attachments/:id/download", get(routes_attachment::download_attachment))
        .route("/api/attachments/:id", axum::routing::delete(routes_attachment::delete_attachment))
        // Search
        .route("/api/search", get(routes_search::search_notes))
        // Automation
        .route("/api/automation/rules", get(routes_automation::list_rules).post(routes_automation::create_rule))
        .route("/api/automation/rules/:id", patch(routes_automation::update_rule))
        .route("/api/automation/rules/:id/launch", post(routes_automation::launch_rule))
        .route("/api/automation/runs", get(routes_automation::list_runs))
        .route("/api/automation/runs/:id", get(routes_automation::get_run))
        .route("/api/automation/runs/:id/review", post(routes_automation::review_run))
        // Health
        .route("/api/healthz", get(routes_health::healthz))
        .route("/api/readyz", get(routes_health::readyz))
        // Metrics per /docs/spec/technical/performance.md (IMP-OPS-02)
        .route("/api/metrics", get(metrics::metrics_handler))
        // CSRF middleware per /docs/spec/security/csrf.md
        .layer(axum_mw::from_fn_with_state(state.clone(), csrf_middleware))
        // Tracing + metrics middleware per IMP-OPS-01 / IMP-OPS-02
        .layer(axum_mw::from_fn_with_state(state.clone(), tracing_middleware))
        // CSP nonce per /docs/spec/security/transport.md (IMP-SEC-01)
        .layer(axum_mw::from_fn(csp_middleware))
        .with_state(state)
}
