use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::{internal_error, is_hx_request, require_admin_session};
use crate::web::state::WebState;

use super::admin_input::normalize_slug_input;

pub async fn handle_post_admin_delete(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    let slug = match normalize_slug_input(&slug.into_inner(), "slug") {
        Ok(slug) => slug,
        Err(message) => return HttpResponse::BadRequest().body(message),
    };

    match state.content_store.delete_article(&slug).await {
        Ok(()) if !is_hx_request(&request) => HttpResponse::NoContent().finish(),
        Ok(()) => HttpResponse::SeeOther()
            .append_header(("Location", "/admin/trash?status=deleted"))
            .finish(),
        Err(crate::error::AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            HttpResponse::NotFound().finish()
        }
        Err(error) => internal_error(error),
    }
}
