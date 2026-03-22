use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::state::WebState;

use super::admin_fragments::{
    html_fragment_response, render_admin_article_list, render_admin_conflict_clear,
    render_admin_editor, render_admin_preview, render_admin_save_fragments,
    render_admin_save_hx_trigger, render_admin_status_banner, render_admin_validation_banner,
    HTML_CONTENT_TYPE,
};
use super::admin_input::{normalize_slug_input, CreateForm, RenameForm, SaveForm};
use super::admin_runtime::{load_admin_items, load_editor_document};

pub async fn handle_post_admin_create(
    request: HttpRequest,
    state: web::Data<WebState>,
    form: web::Form<CreateForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match normalize_slug_input(&form.slug, "slug") {
        Ok(slug) => slug,
        Err(message) => {
            return html_fragment_response(
                StatusCode::BAD_REQUEST,
                render_admin_validation_banner(&message),
            );
        }
    };

    match state
        .content_store
        .create_article(&slug, form.title.clone(), &form.body, false)
        .await
    {
        Ok(()) => {
            match (
                load_admin_items(&state).await,
                load_editor_document(&state, &slug).await,
            ) {
                (Ok(items), Ok(document)) => html_fragment_response(
                    StatusCode::CREATED,
                    format!(
                        "{}{}{}{}{}",
                        render_admin_article_list(&items, Some(&slug), false),
                        render_admin_editor(&document, true),
                        render_admin_preview(&document.body, true),
                        render_admin_status_banner("Article created.", "ok", true),
                        render_admin_conflict_clear(true)
                    ),
                ),
                (Err(e), _) | (_, Err(e)) => internal_error(e),
            }
        }
        Err(error) => internal_error(error),
    }
}

pub async fn handle_post_admin_save(
    request: HttpRequest,
    state: web::Data<WebState>,
    form: web::Form<SaveForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match normalize_slug_input(&form.slug, "slug") {
        Ok(slug) => slug,
        Err(message) => {
            return html_fragment_response(
                StatusCode::BAD_REQUEST,
                render_admin_validation_banner(&message),
            );
        }
    };
    let private = form.private.unwrap_or(false);

    match state
        .content_store
        .save_article(
            &slug,
            form.title.clone(),
            &form.body,
            private,
            form.last_known_revision.as_deref(),
        )
        .await
    {
        Ok(outcome) => HttpResponse::Ok()
            .append_header(("HX-Trigger", render_admin_save_hx_trigger(&outcome)))
            .content_type(HTML_CONTENT_TYPE)
            .body(render_admin_save_fragments(&outcome)),
        Err(error) => internal_error(error),
    }
}

pub async fn handle_post_admin_rename(
    request: HttpRequest,
    state: web::Data<WebState>,
    form: web::Form<RenameForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match normalize_slug_input(&form.slug, "slug") {
        Ok(slug) => slug,
        Err(message) => {
            return html_fragment_response(
                StatusCode::BAD_REQUEST,
                render_admin_validation_banner(&message),
            );
        }
    };
    let new_slug = match normalize_slug_input(&form.new_slug, "new_slug") {
        Ok(new_slug) => new_slug,
        Err(message) => {
            return html_fragment_response(
                StatusCode::BAD_REQUEST,
                render_admin_validation_banner(&message),
            );
        }
    };

    match state.content_store.rename_article(&slug, &new_slug).await {
        Ok(()) => {
            match (
                load_admin_items(&state).await,
                load_editor_document(&state, &new_slug).await,
            ) {
                (Ok(items), Ok(document)) => html_fragment_response(
                    StatusCode::OK,
                    format!(
                        "{}{}{}{}{}",
                        render_admin_article_list(&items, Some(&new_slug), false),
                        render_admin_editor(&document, true),
                        render_admin_preview(&document.body, true),
                        render_admin_status_banner("Article renamed.", "ok", true),
                        render_admin_conflict_clear(true)
                    ),
                ),
                (Err(e), _) | (_, Err(e)) => internal_error(e),
            }
        }
        Err(error) => internal_error(error),
    }
}
