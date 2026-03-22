use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::{internal_error, is_hx_request, require_admin_session};
use crate::web::state::WebState;

use super::admin_fragments::{
    html_fragment_response, render_admin_article_list, render_admin_conflict_clear,
    render_admin_editor_placeholder, render_admin_preview_empty, render_admin_status_banner,
};
use super::admin_input::valid_path_slug;
use super::admin_response::htmx_editor_state_response;
use super::admin_runtime::{is_missing_article, load_admin_items};

pub async fn handle_post_admin_delete(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match valid_path_slug(slug.into_inner()) {
        Ok(slug) => slug,
        Err(response) => return response,
    };

    match state.content_store.delete_article(&slug).await {
        Ok(()) if !is_hx_request(&request) => HttpResponse::NoContent().finish(),
        Ok(()) => match load_admin_items(&state).await {
            Ok(items) => html_fragment_response(
                StatusCode::OK,
                format!(
                    "{}{}{}{}{}",
                    render_admin_article_list(&items, None, false),
                    render_admin_editor_placeholder(true),
                    render_admin_preview_empty(true),
                    render_admin_status_banner("Article moved to trash.", "deleted", true),
                    render_admin_conflict_clear(true)
                ),
            ),
            Err(error) => internal_error(error),
        },
        Err(error) if is_missing_article(&error) => HttpResponse::NotFound().finish(),
        Err(error) => internal_error(error),
    }
}

pub async fn handle_post_admin_toggle_private(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match valid_path_slug(slug.into_inner()) {
        Ok(slug) => slug,
        Err(response) => return response,
    };

    match state.content_store.toggle_article_private(&slug).await {
        Ok(private) if !is_hx_request(&request) => {
            HttpResponse::Ok().body(format!("private={private}"))
        }
        Ok(_) => {
            htmx_editor_state_response(&state, &slug, StatusCode::OK, "Privacy updated.").await
        }
        Err(error) if is_missing_article(&error) => HttpResponse::NotFound().finish(),
        Err(error) => internal_error(error),
    }
}
