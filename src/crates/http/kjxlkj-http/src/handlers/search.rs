use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::SearchQuery;
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;

use kjxlkj_search::{backlinks, fts};

/// GET /api/search?workspace_id=...&q=...
pub async fn search(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    match fts::search_notes(pool.get_ref(), query.workspace_id, &query.q).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => domain_error_response(&e),
    }
}

/// GET /api/notes/{id}/backlinks
pub async fn get_backlinks(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let note_id = path.into_inner();
    match backlinks::list_backlinks(pool.get_ref(), note_id).await {
        Ok(links) => HttpResponse::Ok().json(links),
        Err(e) => domain_error_response(&e),
    }
}
