use actix_web::{web, HttpRequest, HttpResponse};

use crate::error::AppError;
use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::state::WebState;

use super::admin_input::{normalize_slug_input, valid_path_slug, CreateForm, RenameForm, SaveForm};

pub async fn handle_get_admin_shell(
    request: HttpRequest,
    state: web::Data<WebState>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    match state.content_store.list_admin_slugs().await {
        Ok(slugs) => HttpResponse::Ok().body(slugs.join("\n")),
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

    let slug = match valid_path_slug(slug.into_inner()) {
        Ok(slug) => slug,
        Err(response) => return response,
    };

    match state.content_store.read_article(&slug).await {
        Ok(parsed) => HttpResponse::Ok().body(parsed.body),
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}

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
        Err(response) => return response,
    };

    match state
        .content_store
        .create_article(&slug, form.title.clone(), &form.body, false)
        .await
    {
        Ok(()) => HttpResponse::Created().finish(),
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
        Err(response) => return response,
    };
    let private = form.private.unwrap_or(false);

    match state
        .content_store
        .save_article(&slug, form.title.clone(), &form.body, private)
        .await
    {
        Ok(()) => HttpResponse::NoContent().finish(),
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
        Err(response) => return response,
    };
    let new_slug = match normalize_slug_input(&form.new_slug, "new_slug") {
        Ok(new_slug) => new_slug,
        Err(response) => return response,
    };

    match state.content_store.rename_article(&slug, &new_slug).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(error) => internal_error(error),
    }
}

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
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
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
        Ok(private) => HttpResponse::Ok().body(format!("private={private}")),
        Err(AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}
