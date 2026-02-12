mod app_state;
mod auth;
mod config;
mod db_notes;
mod error;
mod handlers;
mod models;
mod patch;
mod ws;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

use crate::{
    app_state::AppState,
    config::Config,
    handlers::{admin, attachments, auth as auth_handlers, health, notes, workspace_suite},
    ws::session::ws_notes,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init();

    let config = Config::from_env();
    std::fs::create_dir_all(&config.export_dir).ok();
    std::fs::create_dir_all(&config.backup_dir).ok();

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .with_context(|| "failed to connect postgres")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .with_context(|| "failed running migrations")?;

    let state = AppState::new(config.clone(), pool);
    tracing::info!(bind_addr = %config.bind_addr, "kjxlkj server starting");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api")
                    .route("/healthz", web::get().to(health::healthz))
                    .route("/readyz", web::get().to(health::readyz))
                    .route(
                        "/setup/register",
                        web::post().to(auth_handlers::setup_register),
                    )
                    .route("/auth/login", web::post().to(auth_handlers::login))
                    .route("/auth/logout", web::post().to(auth_handlers::logout))
                    .route(
                        "/auth/session",
                        web::get().to(auth_handlers::current_session),
                    )
                    .route("/users", web::get().to(workspace_suite::list_users))
                    .route("/users", web::post().to(workspace_suite::create_user))
                    .route(
                        "/users/{id}/role",
                        web::patch().to(workspace_suite::update_user_role),
                    )
                    .route(
                        "/users/{id}",
                        web::delete().to(workspace_suite::delete_user),
                    )
                    .route(
                        "/workspaces",
                        web::get().to(workspace_suite::list_workspaces),
                    )
                    .route(
                        "/workspaces",
                        web::post().to(workspace_suite::create_workspace),
                    )
                    .route(
                        "/workspaces/{id}",
                        web::patch().to(workspace_suite::update_workspace),
                    )
                    .route(
                        "/workspaces/{id}",
                        web::delete().to(workspace_suite::delete_workspace),
                    )
                    .route(
                        "/workspaces/{id}/members",
                        web::get().to(workspace_suite::list_workspace_members),
                    )
                    .route(
                        "/workspaces/{id}/members/{user_id}",
                        web::put().to(workspace_suite::upsert_workspace_member),
                    )
                    .route("/projects", web::get().to(workspace_suite::list_projects))
                    .route("/projects", web::post().to(workspace_suite::create_project))
                    .route(
                        "/projects/{id}",
                        web::patch().to(workspace_suite::update_project),
                    )
                    .route(
                        "/projects/{id}",
                        web::delete().to(workspace_suite::delete_project),
                    )
                    .route("/notes", web::post().to(notes::create_note))
                    .route("/notes/media", web::post().to(notes::create_media_note))
                    .route("/notes", web::get().to(notes::list_notes))
                    .route("/notes/{id}", web::get().to(notes::get_note))
                    .route("/notes/{id}", web::patch().to(notes::patch_note))
                    .route(
                        "/notes/{id}/title",
                        web::patch().to(notes::patch_note_title),
                    )
                    .route("/notes/{id}", web::delete().to(notes::delete_note))
                    .route("/notes/{id}/history", web::get().to(notes::note_history))
                    .route("/notes/{id}/rollback", web::post().to(notes::rollback_note))
                    .route(
                        "/notes/{id}/metadata/{key}",
                        web::put().to(notes::put_metadata),
                    )
                    .route(
                        "/notes/{id}/metadata/{key}",
                        web::delete().to(notes::delete_metadata),
                    )
                    .route("/tags", web::get().to(notes::list_tags))
                    .route("/notes/{id}/tags", web::put().to(notes::replace_tags))
                    .route("/notes/{id}/backlinks", web::get().to(notes::backlinks))
                    .route("/search", web::get().to(notes::search))
                    .route(
                        "/notes/{id}/attachments",
                        web::post().to(attachments::upload_attachment),
                    )
                    .route(
                        "/attachments/{id}",
                        web::get().to(attachments::download_attachment),
                    )
                    .route(
                        "/attachments/{id}",
                        web::delete().to(attachments::delete_attachment),
                    )
                    .route("/views", web::get().to(workspace_suite::list_views))
                    .route("/views", web::post().to(workspace_suite::create_view))
                    .route("/views/{id}", web::patch().to(workspace_suite::update_view))
                    .route(
                        "/views/{id}",
                        web::delete().to(workspace_suite::delete_view),
                    )
                    .route(
                        "/dashboards",
                        web::get().to(workspace_suite::list_dashboards),
                    )
                    .route(
                        "/dashboards/widgets",
                        web::post().to(workspace_suite::upsert_dashboard_widget),
                    )
                    .route(
                        "/automation/rules",
                        web::get().to(workspace_suite::list_automation_rules),
                    )
                    .route(
                        "/automation/rules",
                        web::post().to(workspace_suite::create_automation_rule),
                    )
                    .route(
                        "/automation/rules/{id}",
                        web::patch().to(workspace_suite::update_automation_rule),
                    )
                    .route(
                        "/automation/rules/{id}",
                        web::delete().to(workspace_suite::delete_automation_rule),
                    )
                    .route(
                        "/automation/runs/{id}",
                        web::get().to(workspace_suite::get_automation_run),
                    )
                    .route(
                        "/admin/export/markdown",
                        web::post().to(admin::export_markdown),
                    )
                    .route(
                        "/admin/export/{job_id}",
                        web::get().to(admin::export_job_status),
                    )
                    .route("/admin/backup/sql", web::post().to(admin::backup_sql)),
            )
            .route("/ws", web::get().to(ws_notes))
            .service(Files::new("/", "frontend/dist").index_file("index.html"))
    })
    .bind(&config.bind_addr)
    .with_context(|| format!("failed binding to {}", config.bind_addr))?
    .run()
    .await
    .with_context(|| "http server failed")?;

    Ok(())
}
