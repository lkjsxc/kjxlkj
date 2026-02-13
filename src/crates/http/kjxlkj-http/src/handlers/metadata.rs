use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::UpsertMetadataRequest;
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;

/// PUT /api/notes/{id}/metadata/{key}
pub async fn upsert_metadata(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<(uuid::Uuid, String)>,
    body: web::Json<UpsertMetadataRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let (note_id, key) = path.into_inner();
    match repos::metadata::upsert_metadata(pool.get_ref(), note_id, &key, body.value.clone()).await
    {
        Ok(row) => HttpResponse::Ok().json(row),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// DELETE /api/notes/{id}/metadata/{key} — returns 204 per http.md.
pub async fn delete_metadata(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<(uuid::Uuid, String)>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let (note_id, key) = path.into_inner();
    match repos::metadata::delete_metadata(pool.get_ref(), note_id, &key).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}
