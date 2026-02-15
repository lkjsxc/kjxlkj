//! kjxlkj-http: HTTP handlers, DTO mapping, middleware.
//! Per /docs/spec/api/http.md and /docs/spec/architecture/source-layout.md.

pub mod admin;
pub mod attachments;
pub mod automation;
pub mod dto;
pub mod health;
pub mod metadata;
pub mod middleware;
pub mod notes;
pub mod setup;
pub mod users;
pub mod views;
pub mod workspaces;

use actix_web::web;

/// Configure all API routes under /api.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Health
            .route("/healthz", web::get().to(health::healthz))
            .route("/readyz", web::get().to(health::readyz))
            // Setup & Auth
            .route("/setup/register", web::post().to(setup::register))
            .route("/auth/login", web::post().to(setup::login))
            .route("/auth/logout", web::post().to(setup::logout))
            .route("/auth/session", web::get().to(setup::session_info))
            // Users
            .route("/users", web::get().to(users::list))
            .route("/users", web::post().to(users::create))
            .route("/users/{id}/role", web::patch().to(users::update_role))
            .route("/users/{id}", web::delete().to(users::delete))
            // Workspaces
            .route("/workspaces", web::get().to(workspaces::list))
            .route("/workspaces", web::post().to(workspaces::create))
            .route("/workspaces/{id}", web::patch().to(workspaces::update))
            .route("/workspaces/{id}", web::delete().to(workspaces::delete))
            .route("/workspaces/{id}/members", web::get().to(workspaces::list_members))
            .route("/workspaces/{id}/members/{user_id}", web::put().to(workspaces::upsert_member))
            // Projects
            .route("/projects", web::get().to(workspaces::list_projects))
            .route("/projects", web::post().to(workspaces::create_project))
            .route("/projects/{id}", web::patch().to(workspaces::update_project))
            .route("/projects/{id}", web::delete().to(workspaces::delete_project))
            // Notes
            .route("/notes", web::post().to(notes::create))
            .route("/notes/media", web::post().to(notes::create_media))
            .route("/notes", web::get().to(notes::list))
            .route("/notes/{id}", web::get().to(notes::get))
            .route("/notes/{id}", web::patch().to(notes::patch))
            .route("/notes/{id}/title", web::patch().to(notes::update_title))
            .route("/notes/{id}", web::delete().to(notes::delete))
            .route("/notes/{id}/history", web::get().to(notes::history))
            .route("/notes/{id}/rollback", web::post().to(notes::rollback))
            // Metadata, Tags, Backlinks, Search
            .route("/notes/{id}/metadata/{key}", web::put().to(metadata::upsert))
            .route("/notes/{id}/metadata/{key}", web::delete().to(metadata::delete))
            .route("/tags", web::get().to(metadata::list_tags))
            .route("/notes/{id}/tags", web::put().to(metadata::replace_tags))
            .route("/notes/{id}/backlinks", web::get().to(metadata::backlinks))
            .route("/search", web::get().to(metadata::search))
            // Views
            .route("/views", web::get().to(views::list))
            .route("/views", web::post().to(views::create))
            .route("/views/{id}", web::patch().to(views::update))
            .route("/views/{id}", web::delete().to(views::delete))
            // Dashboards (optional)
            .route("/dashboards", web::get().to(views::list_dashboards))
            .route("/dashboards/widgets", web::post().to(views::upsert_widget))
            // Automation
            .route("/automation/rules", web::get().to(automation::list_rules))
            .route("/automation/rules", web::post().to(automation::create_rule))
            .route("/automation/rules/{id}", web::patch().to(automation::update_rule))
            .route("/automation/rules/{id}", web::delete().to(automation::delete_rule))
            .route("/automation/rules/{id}/launch", web::post().to(automation::launch))
            .route("/automation/runs", web::get().to(automation::list_runs))
            .route("/automation/runs/{id}", web::get().to(automation::get_run))
            .route("/automation/runs/{id}/review", web::post().to(automation::review))
            // Attachments
            .route("/notes/{id}/attachments", web::post().to(attachments::upload))
            .route("/attachments/{id}", web::get().to(attachments::download))
            .route("/attachments/{id}", web::delete().to(attachments::delete))
            // Admin
            .route("/admin/export/markdown", web::post().to(admin::export_markdown))
            .route("/admin/export/{job_id}", web::get().to(admin::export_status))
            .route("/admin/backup/sql", web::post().to(admin::backup_sql)),
    );
}
