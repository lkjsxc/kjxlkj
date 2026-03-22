use actix_web::{http::header, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::error::AppError;
use crate::web::handlers::app_shell::render_shell_page;
use crate::web::handlers::common::internal_error;
use crate::web::handlers::common::require_admin_session;
use crate::web::handlers::page_html::escape_html;
use crate::web::state::WebState;

#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    status: Option<String>,
}

pub async fn handle_get_admin_trash(
    request: HttpRequest,
    state: web::Data<WebState>,
    query: web::Query<StatusQuery>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let slugs = match state.content_store.list_admin_slugs().await {
        Ok(slugs) => slugs,
        Err(error) => return internal_error(error),
    };
    let trash = match state.content_store.list_trashed_admin_slugs().await {
        Ok(slugs) => slugs,
        Err(error) => return internal_error(error),
    };
    let main = render_trash_main(&trash, query.status.as_deref().unwrap_or_default());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_shell_page(
            &settings.site_title,
            "Trash",
            &main,
            &slugs,
            true,
        ))
}

pub async fn handle_post_admin_trash_restore(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    match state
        .content_store
        .restore_article(&slug.into_inner())
        .await
    {
        Ok(()) => HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin/trash?status=restored"))
            .finish(),
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}

pub async fn handle_post_admin_trash_delete_permanent(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    match state
        .content_store
        .permanent_delete_article(&slug.into_inner())
        .await
    {
        Ok(()) => HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin/trash?status=permanent-delete-ok"))
            .finish(),
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}

fn render_trash_main(slugs: &[String], status: &str) -> String {
    let rows = slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            format!(
                "<li><span>{escaped}</span><form method=\"post\" action=\"/admin/trash/restore/{escaped}\"><button type=\"submit\">Restore</button></form><form method=\"post\" action=\"/admin/trash/delete-permanent/{escaped}\"><button type=\"submit\">Delete permanently</button></form></li>"
            )
        })
        .collect::<String>();
    format!(
        "<main id=\"admin-trash-page\"><h1>Trash</h1><section id=\"admin-trash-status\">{}</section><section id=\"admin-trash-list\"><ul>{}</ul></section></main>",
        escape_html(status),
        rows
    )
}

pub fn render_trash_main_page(slugs: &[String], status: &str) -> String {
    render_trash_main(slugs, status)
}
