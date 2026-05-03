//! Resource history HTML handlers

use crate::error::AppError;
use crate::web::db::{self, ListDirection};
use crate::web::handlers::http;
use crate::web::handlers::resource_history::HistoryParams;
use crate::web::handlers::session;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::Response;

pub async fn history_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
    Path(reference): Path<String>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    let is_admin = session::check_session(&headers, pool).await?;
    if !is_admin {
        return Ok(http::redirect(&session::login_url(&uri)));
    }
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    let Some(resource) = db::get_resource_by_ref(pool, &reference).await? else {
        return Ok(not_found(&site));
    };
    if resource
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == resource.id
    {
        return Ok(http::redirect(&with_query(
            &view::history_href(&resource),
            uri.query().unwrap_or_default(),
        )));
    }
    let page = db::list_resource_snapshots(
        pool,
        &resource.id,
        true,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    let chrome = view::resource_chrome(pool, &resource, true).await?;
    let history = view::history_links(&page.snapshots, params.cursor.is_none());
    Ok(http::html(templates::history_page(
        &resource,
        &chrome,
        templates::HistoryPage {
            history: &history,
            previous_cursor: page.previous_cursor.as_deref(),
            next_cursor: page.next_cursor.as_deref(),
            limit: params.limit.unwrap_or(settings.search_results_per_page),
        },
        true,
        &site,
    )))
}

pub async fn history_page_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: Uri,
    Path((_user, reference)): Path<(String, String)>,
    Query(params): Query<HistoryParams>,
) -> Result<Response, AppError> {
    history_page(State(state), headers, uri, Path(reference), Query(params)).await
}

fn with_query(path: &str, query: &str) -> String {
    if query.is_empty() {
        path.to_string()
    } else {
        format!("{path}?{query}")
    }
}

fn not_found(site: &SiteContext) -> Response {
    http::html_status(
        StatusCode::NOT_FOUND,
        templates::not_found_page(&site.page_meta(
            "Not Found",
            "The requested resource could not be found.",
            false,
            None,
        )),
    )
}
