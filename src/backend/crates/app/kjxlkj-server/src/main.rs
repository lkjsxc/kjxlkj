use std::{env, net::SocketAddr};

use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::{net::TcpListener, signal};
use tracing::info;

mod auth;
mod error;
mod handlers_automation;
mod handlers_automation_runs;
mod handlers_auth;
mod handlers_notes_core;
mod handlers_notes_history;
mod handlers_notes_meta;
mod handlers_notes_mutations;
mod handlers_ops;
mod handlers_projects;
mod handlers_stub;
mod handlers_users;
mod handlers_views;
mod handlers_workspace;
mod model;
mod routes;
mod state;
mod ws;
#[cfg(test)]
mod tests_contract;
#[cfg(test)]
mod tests_automation;
#[cfg(test)]
mod tests_views;
#[cfg(test)]
mod tests_ws;
#[cfg(test)]
mod tests_ws_replay;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = normalize_database_url(
        env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://./data/kjxlkj.db?mode=rwc".to_string()),
    );

    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .with_context(|| format!("failed to connect to sqlite url: {database_url}"))?;

    sqlx::query("SELECT 1")
        .execute(&db_pool)
        .await
        .context("startup readiness query failed")?;

    let state = state::AppState::new(db_pool);
    let app = routes::build_router(state);

    let socket_addr: SocketAddr = format!("{bind_address}:{port}")
        .parse()
        .with_context(|| format!("invalid bind address: {bind_address}:{port}"))?;

    info!(%socket_addr, "kjxlkj-server listening");

    let listener = TcpListener::bind(socket_addr)
        .await
        .context("failed to bind listener")?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server failure")?;

    Ok(())
}

fn init_tracing() {
    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn normalize_database_url(raw: String) -> String {
    if let Some(rest) = raw.strip_prefix("sqlite:/") {
        if !rest.starts_with('/') {
            return format!("sqlite:///{rest}");
        }
    }
    raw
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
}
