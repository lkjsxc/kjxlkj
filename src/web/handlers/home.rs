//! Homepage handler

use crate::error::AppError;
use crate::web::db::{self, PopularWindow};
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::markdown;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::templates::home::HomeView;
use crate::web::view;
use axum::extract::State;
use axum::http::{HeaderMap, Uri};
use axum::response::Response;

pub async fn home_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    let is_admin = session::check_session(&headers, pool).await?;
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    let window = PopularWindow::Days30;
    let popular =
        db::list_popular_resources(pool, is_admin, settings.home_popular_limit, window).await?;
    let recent = db::list_recent_resources(pool, is_admin, settings.home_recent_limit).await?;
    let favorites =
        db::list_favorite_resources(pool, is_admin, settings.home_favorite_limit).await?;
    let popular_items = popular
        .iter()
        .map(|resource| view::popular_index_item(resource, is_admin, window))
        .collect::<Vec<_>>();
    let recent_items = recent
        .iter()
        .map(|resource| view::index_item(resource, is_admin))
        .collect::<Vec<_>>();
    let favorite_items = favorites
        .iter()
        .map(|resource| view::index_item(resource, is_admin))
        .collect::<Vec<_>>();
    let guest_login_href = session::login_url(&uri);
    let intro_html = markdown::render_markdown_page(
        pool,
        &settings.home_intro_markdown,
        None,
        is_admin,
        site.public_base_url.as_deref(),
        Some(&settings.google_maps_embed_api_key),
    )
    .await?;
    Ok(http::html(templates::home_page(HomeView {
        settings: &settings,
        intro_html: &intro_html,
        popular: &popular_items,
        recent: &recent_items,
        favorites: &favorite_items,
        window,
        is_admin,
        guest_login_href: &guest_login_href,
        site: &site,
    })))
}
