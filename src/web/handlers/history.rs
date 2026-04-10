//! Resource history HTML handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, ListDirection};
use crate::web::handlers::resource_history::HistoryParams;
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/{id}/history")]
pub async fn history_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    params: web::Query<HistoryParams>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let reference = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    if !is_admin {
        return Ok(redirect(&session::login_url(&req)));
    }
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    let Some(resource) = db::get_resource_by_ref(&pool, &reference).await? else {
        return Ok(not_found(&site));
    };
    if resource
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == resource.id
    {
        return Ok(redirect(&with_query(
            &view::history_href(&resource),
            req.query_string(),
        )));
    }
    let page = db::list_resource_snapshots(
        &pool,
        &resource.id,
        true,
        params.limit.unwrap_or(settings.search_results_per_page),
        &ListDirection::resolve(params.direction.as_deref(), params.cursor.as_deref()),
        params.cursor.as_deref(),
    )
    .await?;
    let chrome = view::resource_chrome(&pool, &resource, true).await?;
    let history = view::history_links(&page.snapshots, params.cursor.is_none());
    Ok(html(templates::history_page(
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

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn with_query(path: &str, query: &str) -> String {
    if query.is_empty() {
        path.to_string()
    } else {
        format!("{path}?{query}")
    }
}

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

fn not_found(site: &SiteContext) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(templates::not_found_page(&site.page_meta(
            "Not Found",
            "The requested resource could not be found.",
            false,
            None,
        )))
}
