use actix_web::{web, HttpRequest, HttpResponse};

use crate::core::content::private_or_default;
use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::state::WebState;

use super::admin_input::{normalize_slug_input, CreateForm, RenameForm};

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
            return HttpResponse::BadRequest().body(message);
        }
    };

    match state
        .content_store
        .create_article(
            &slug,
            form.title.clone(),
            &form.body,
            private_or_default(form.private),
        )
        .await
    {
        Ok(()) => HttpResponse::SeeOther()
            .append_header(("Location", format!("/article/{slug}")))
            .finish(),
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
            return HttpResponse::BadRequest().body(message);
        }
    };
    let new_slug = match normalize_slug_input(&form.new_slug, "new_slug") {
        Ok(new_slug) => new_slug,
        Err(message) => {
            return HttpResponse::BadRequest().body(message);
        }
    };

    match state.content_store.rename_article(&slug, &new_slug).await {
        Ok(()) => HttpResponse::SeeOther()
            .append_header(("Location", format!("/article/{new_slug}")))
            .finish(),
        Err(error) => internal_error(error),
    }
}
