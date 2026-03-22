use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::error::AppError;
use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::state::WebState;

use super::admin_fragments::{
    html_fragment_response, render_admin_conflict_clear, render_admin_editor_pane,
    render_admin_preview, HTML_CONTENT_TYPE,
};
use super::admin_page::render_admin_shell;
use crate::core::content::{revision_token, serialize_markdown_document};

pub use super::admin_actions::{
    handle_post_admin_create, handle_post_admin_rename, handle_post_admin_save,
};
pub use super::admin_mutation::{handle_post_admin_delete, handle_post_admin_toggle_private};

pub async fn handle_get_admin_shell(
    request: HttpRequest,
    state: web::Data<WebState>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    match state.content_store.list_admin_slugs().await {
        Ok(slugs) => HttpResponse::Ok()
            .content_type(HTML_CONTENT_TYPE)
            .body(render_admin_shell(&slugs)),
        Err(error) => internal_error(error),
    }
}

pub async fn handle_get_admin_open(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug_str = slug.into_inner();
    if slug_str.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    match state.content_store.read_article(&slug_str).await {
        Ok(parsed) => {
            let markdown = serialize_markdown_document(&parsed.frontmatter, &parsed.body);
            let revision = revision_token(&markdown);
            let editor = render_admin_editor_pane(
                &slug_str,
                parsed.frontmatter.title.as_deref(),
                &parsed.body,
                parsed.frontmatter.private,
                &revision,
                false,
            );
            let preview = render_admin_preview(&parsed.body, true);
            let conflict_clear = render_admin_conflict_clear(true);
            html_fragment_response(StatusCode::OK, format!("{editor}{preview}{conflict_clear}"))
        }
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}
