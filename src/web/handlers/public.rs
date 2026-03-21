use actix_web::{web, HttpRequest, HttpResponse};

use crate::core::content::VisibilityContext;
use crate::web::handlers::common::internal_error;
use crate::web::render::render_markdown_html;
use crate::web::session::valid_session;
use crate::web::state::WebState;

pub async fn handle_get_home(request: HttpRequest, state: web::Data<WebState>) -> HttpResponse {
    let context = match valid_session(&request, &state).await {
        Ok(Some(_)) => VisibilityContext::Admin,
        Ok(None) => VisibilityContext::Public,
        Err(error) => return internal_error(error),
    };

    let result = match context {
        VisibilityContext::Admin => state.content_store.list_admin_slugs().await,
        VisibilityContext::Public => state.content_store.list_public_slugs().await,
    };

    match result {
        Ok(slugs) => HttpResponse::Ok().body(slugs.join("\n")),
        Err(error) => internal_error(error),
    }
}

pub async fn handle_get_article(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    let slug = slug.into_inner();
    let is_admin = match valid_session(&request, &state).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(error) => return internal_error(error),
    };

    let parsed = match state.content_store.read_article(&slug).await {
        Ok(parsed) => parsed,
        Err(crate::error::AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            return HttpResponse::NotFound().finish()
        }
        Err(error) => return internal_error(error),
    };

    if parsed.frontmatter.private && !is_admin {
        return HttpResponse::NotFound().finish();
    }

    let html = render_markdown_html(&parsed.body);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
