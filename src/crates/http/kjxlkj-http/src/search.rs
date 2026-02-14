// Search handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_search::{backlinks, query};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::{ErrorBody, SearchQuery};

/// GET /api/search
pub async fn search(
    pool: web::Data<PgPool>,
    params: web::Query<SearchQuery>,
) -> HttpResponse {
    match query::search_notes(pool.get_ref(), params.workspace_id, &params.q).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// GET /api/notes/{id}/backlinks
pub async fn backlinks(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    match backlinks::get_backlinks(pool.get_ref(), path.into_inner()).await {
        Ok(links) => HttpResponse::Ok().json(links),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}
