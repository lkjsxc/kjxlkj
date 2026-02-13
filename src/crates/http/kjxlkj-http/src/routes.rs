use actix_web::web;
use crate::handlers;

/// Register all HTTP routes under /api per http.md contract.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Ops
            .route("/healthz", web::get().to(handlers::health::healthz))
            .route("/readyz", web::get().to(handlers::health::readyz))
            // Setup and Auth
            .route("/setup/register", web::post().to(handlers::auth::setup_register))
            .route("/auth/login", web::post().to(handlers::auth::login))
            .route("/auth/logout", web::post().to(handlers::auth::logout))
            .route("/auth/session", web::get().to(handlers::auth::get_session))
            // Users
            .route("/users", web::get().to(handlers::users::list_users))
            .route("/users", web::post().to(handlers::users::create_user))
            .route("/users/{id}/role", web::patch().to(handlers::users::update_role))
            .route("/users/{id}", web::delete().to(handlers::users::disable_user))
            // Workspaces
            .route("/workspaces", web::get().to(handlers::workspaces::list_workspaces))
            .route("/workspaces", web::post().to(handlers::workspaces::create_workspace))
            .route("/workspaces/{id}", web::patch().to(handlers::workspaces::update_workspace))
            .route("/workspaces/{id}", web::delete().to(handlers::workspaces::delete_workspace))
            .route("/workspaces/{id}/members", web::get().to(handlers::workspaces::list_members))
            .route("/workspaces/{id}/members/{user_id}", web::put().to(handlers::workspaces::upsert_member))
            // Projects
            .route("/projects", web::get().to(handlers::projects::list_projects))
            .route("/projects", web::post().to(handlers::projects::create_project))
            .route("/projects/{id}", web::patch().to(handlers::projects::update_project))
            .route("/projects/{id}", web::delete().to(handlers::projects::delete_project))
            // Notes
            .route("/notes", web::post().to(handlers::notes::create_note))
            .route("/notes", web::get().to(handlers::notes::list_notes))
            .route("/notes/{id}", web::get().to(handlers::notes::get_note))
            .route("/notes/{id}", web::patch().to(handlers::notes::patch_note))
            .route("/notes/{id}/title", web::patch().to(handlers::notes::patch_note_title))
            .route("/notes/{id}", web::delete().to(handlers::notes_lifecycle::delete_note))
            .route("/notes/{id}/history", web::get().to(handlers::notes_lifecycle::note_history))
            .route("/notes/{id}/rollback", web::post().to(handlers::notes_lifecycle::rollback_note))
            // Metadata
            .route("/notes/{id}/metadata/{key}", web::put().to(handlers::metadata::upsert_metadata))
            .route("/notes/{id}/metadata/{key}", web::delete().to(handlers::metadata::delete_metadata))
            // Tags
            .route("/tags", web::get().to(handlers::tags::list_tags))
            .route("/notes/{id}/tags", web::put().to(handlers::tags::replace_tags))
            // Search
            .route("/search", web::get().to(handlers::search::search))
            .route("/notes/{id}/backlinks", web::get().to(handlers::search::get_backlinks))
            // Views
            .route("/views", web::get().to(handlers::views::list_views))
            .route("/views", web::post().to(handlers::views::create_view))
            .route("/views/{id}", web::patch().to(handlers::views::update_view))
            .route("/views/{id}", web::delete().to(handlers::views::delete_view))
            // Automation
            .route("/automation/rules", web::get().to(handlers::automation::list_rules))
            .route("/automation/rules", web::post().to(handlers::automation::create_rule))
            .route("/automation/rules/{id}", web::patch().to(handlers::automation::update_rule))
            .route("/automation/rules/{id}", web::delete().to(handlers::automation::delete_rule))
            .route("/automation/rules/{id}/launch", web::post().to(handlers::automation_runs::launch_run))
            .route("/automation/runs", web::get().to(handlers::automation_runs::list_runs))
            .route("/automation/runs/{id}", web::get().to(handlers::automation_runs::get_run))
            .route("/automation/runs/{id}/review", web::post().to(handlers::automation_runs::review_run))
            // Admin
            .route("/admin/export/markdown", web::post().to(handlers::admin::export_markdown))
            .route("/admin/export/{job_id}", web::get().to(handlers::admin::export_status))
            .route("/admin/backup/sql", web::post().to(handlers::admin::backup_sql)),
    );
}
