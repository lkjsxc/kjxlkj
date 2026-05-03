//! Admin dashboard handlers

use crate::error::AppError;
use crate::web::db::{self, PopularWindow};
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use axum::extract::State;
use axum::http::{HeaderMap, Uri};
use axum::response::Response;

pub async fn admin_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    if !session::check_session(&headers, pool).await? {
        return Ok(http::redirect(&session::login_url(&uri)));
    }
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    let window = PopularWindow::Days30;
    let popular =
        db::list_popular_resources(pool, None, true, settings.home_popular_limit, window).await?;
    let recent = db::list_recent_resources(pool, true, settings.home_recent_limit).await?;
    let favorites = db::list_all_favorite_resources(pool, true).await?;
    let stats = db::get_resource_stats(pool, true).await?;
    Ok(http::html(templates::admin_page(
        &stats,
        &settings,
        &popular
            .iter()
            .map(|resource| view::popular_index_item(resource, true, window))
            .collect::<Vec<_>>(),
        &recent
            .iter()
            .map(|resource| view::index_item(resource, true))
            .collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|resource| view::index_item(resource, true))
            .collect::<Vec<_>>(),
        window,
        &site,
    )))
}
