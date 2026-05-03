//! Popular-note fragment handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, PopularWindow};
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use crate::web::templates;
use crate::web::view;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PopularSectionPath {
    surface: String,
    window: String,
}

pub async fn popular_resources_section(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<PopularSectionPath>,
) -> Result<Response, AppError> {
    if !db::is_setup(&state.pool).await? {
        return Ok(not_found());
    }
    let Some(window) = PopularWindow::parse(&path.window) else {
        return Ok(not_found());
    };
    match path.surface.as_str() {
        "home" => render_home_fragment(&state.pool, &headers, window).await,
        "admin" => render_admin_fragment(&state.pool, &headers, window).await,
        _ => Ok(not_found()),
    }
}

async fn render_home_fragment(
    pool: &DbPool,
    headers: &HeaderMap,
    window: PopularWindow,
) -> Result<Response, AppError> {
    let is_admin = session::check_session(headers, pool).await?;
    let settings = db::get_settings(pool).await?;
    if !settings.home_popular_visible {
        return Ok(not_found());
    }
    let items = popular_items(pool, is_admin, settings.home_popular_limit, window).await?;
    Ok(http::html(templates::home_popular_section(&items, window)))
}

async fn render_admin_fragment(
    pool: &DbPool,
    headers: &HeaderMap,
    window: PopularWindow,
) -> Result<Response, AppError> {
    if !session::check_session(headers, pool).await? {
        return Ok(unauthorized());
    }
    let limit = db::get_settings(pool).await?.home_popular_limit;
    let items = popular_items(pool, true, limit, window).await?;
    Ok(http::html(templates::admin_popular_section(&items, window)))
}

async fn popular_items(
    pool: &DbPool,
    is_admin: bool,
    limit: i64,
    window: PopularWindow,
) -> Result<Vec<templates::IndexItem>, AppError> {
    Ok(
        db::list_popular_resources(pool, None, is_admin, limit, window)
            .await?
            .iter()
            .map(|resource| view::popular_index_item(resource, is_admin, window))
            .collect(),
    )
}

fn not_found() -> Response {
    http::empty(StatusCode::NOT_FOUND)
}

fn unauthorized() -> Response {
    http::empty(StatusCode::UNAUTHORIZED)
}
