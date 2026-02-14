// Route configuration per /docs/spec/api/http.md
use actix_web::web;

/// Configure all API routes under /api
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Ops
            .route("/healthz", web::get().to(super::ops::healthz))
            .route("/readyz", web::get().to(super::ops::readyz))
            // Setup
            .route("/setup/status", web::get().to(super::setup::status))
            .route("/setup/register", web::post().to(super::setup::register))
            // Auth
            .route("/auth/login", web::post().to(super::auth::login))
            .route("/auth/logout", web::post().to(super::auth::logout))
            .route("/auth/session", web::get().to(super::auth::session_info))
            // Users
            .route("/users", web::get().to(super::users::list))
            .route("/users", web::post().to(super::users::create))
            .route("/users/{id}/role", web::patch().to(super::users::update_role))
            .route("/users/{id}", web::delete().to(super::users::delete))
            // Workspaces
            .route("/workspaces", web::get().to(super::workspaces::list))
            .route("/workspaces", web::post().to(super::workspaces::create))
            .route("/workspaces/{id}", web::patch().to(super::workspaces::update))
            .route("/workspaces/{id}", web::delete().to(super::workspaces::delete))
            .route("/workspaces/{id}/members", web::get().to(super::workspaces::list_members))
            .route("/workspaces/{id}/members/{user_id}", web::put().to(super::workspaces::upsert_member))
            // Projects
            .route("/projects", web::get().to(super::projects::list))
            .route("/projects", web::post().to(super::projects::create))
            .route("/projects/{id}", web::patch().to(super::projects::update))
            .route("/projects/{id}", web::delete().to(super::projects::delete))
            // Notes
            .route("/notes", web::post().to(super::notes::create))
            .route("/notes/media", web::post().to(super::notes::create_media))
            .route("/notes", web::get().to(super::notes::list))
            .route("/notes/{id}", web::get().to(super::notes::get))
            .route("/notes/{id}", web::patch().to(super::notes::update))
            .route("/notes/{id}/title", web::patch().to(super::notes::update_title))
            .route("/notes/{id}", web::delete().to(super::notes::delete))
            .route("/notes/{id}/history", web::get().to(super::notes::history))
            .route("/notes/{id}/rollback", web::post().to(super::notes::rollback))
            // Metadata
            .route("/notes/{id}/metadata/{key}", web::put().to(super::notes::upsert_metadata))
            .route("/notes/{id}/metadata/{key}", web::delete().to(super::notes::delete_metadata))
            // Search
            .route("/search", web::get().to(super::search::search))
            .route("/notes/{id}/backlinks", web::get().to(super::search::backlinks))
            // Tags
            .route("/tags", web::get().to(super::tags::list))
            .route("/notes/{id}/tags", web::put().to(super::tags::replace_tags))
            // Attachments
            .route("/notes/{id}/attachments", web::post().to(super::attachments::upload))
            .route("/attachments/{id}", web::get().to(super::attachments::download))
            .route("/attachments/{id}", web::delete().to(super::attachments::delete))
            // Workspace-scoped note and search aliases (frontend convenience)
            .route("/workspaces/{ws_id}/notes", web::get().to(super::notes::list))
            .route("/workspaces/{ws_id}/notes", web::post().to(super::notes::create))
            .route("/workspaces/{ws_id}/search", web::get().to(super::search::search))
            .route("/workspaces/{ws_id}/automation/runs", web::get().to(super::automation::list_runs))
            // Views
            .route("/views", web::get().to(super::views::list))
            .route("/views", web::post().to(super::views::create))
            .route("/views/{id}", web::patch().to(super::views::update))
            .route("/views/{id}", web::delete().to(super::views::delete))
            // Automation
            .route("/automation/rules", web::get().to(super::automation::list_rules))
            .route("/automation/rules", web::post().to(super::automation::create_rule))
            .route("/automation/rules/{id}", web::patch().to(super::automation::update_rule))
            .route("/automation/rules/{id}", web::delete().to(super::automation::delete_rule))
            .route("/automation/rules/{id}/launch", web::post().to(super::automation::launch_run))
            .route("/automation/runs", web::get().to(super::automation::list_runs))
            .route("/automation/runs/{id}", web::get().to(super::automation::run_detail))
            .route("/automation/runs/{id}/review", web::post().to(super::automation::review_run))
            // Admin
            .route("/admin/export/markdown", web::post().to(super::admin::launch_export))
            .route("/admin/export/{job_id}", web::get().to(super::admin::export_status))
            .route("/admin/backup/sql", web::post().to(super::admin::launch_backup))
    );
}
