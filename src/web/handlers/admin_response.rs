use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::{internal_error, is_hx_request};
use crate::web::state::WebState;

use super::admin_fragments::{
    html_fragment_response, render_admin_article_list, render_admin_conflict_clear,
    render_admin_editor, render_admin_preview, render_admin_status_banner,
    render_admin_validation_banner,
};
use super::admin_runtime::{is_missing_article, load_admin_items, load_editor_document};

#[allow(dead_code)]
pub fn validation_error_response(request: &HttpRequest, message: &str) -> HttpResponse {
    if is_hx_request(request) {
        return html_fragment_response(
            StatusCode::BAD_REQUEST,
            render_admin_validation_banner(message),
        );
    }
    HttpResponse::BadRequest().body(message.to_owned())
}

pub async fn htmx_editor_state_response(
    state: &web::Data<WebState>,
    slug: &str,
    status: StatusCode,
    message: &str,
) -> HttpResponse {
    match (
        load_admin_items(state).await,
        load_editor_document(state, slug).await,
    ) {
        (Ok(items), Ok(document)) => html_fragment_response(
            status,
            format!(
                "{}{}{}{}{}",
                render_admin_article_list(&items, Some(slug), false),
                render_admin_editor(&document, true),
                render_admin_preview(&document.body, true),
                render_admin_status_banner(message, "ok", true),
                render_admin_conflict_clear(true)
            ),
        ),
        (_, Err(error)) if is_missing_article(&error) => HttpResponse::NotFound().finish(),
        (Err(error), _) | (_, Err(error)) => internal_error(error),
    }
}
