use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::ReplaceTagsRequest;
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;

/// GET /api/tags?workspace_id=...
pub async fn list_tags(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    // Return tags for a specific note if note_id is provided.
    let note_id = match query.get("note_id").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => {
            return domain_error_response(&DomainError::BadRequest {
                reason: "note_id required".into(),
            })
        }
    };
    match repos::tags::list_tags(pool.get_ref(), note_id).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// PUT /api/notes/{id}/tags — replace all tags for a note.
pub async fn replace_tags(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<ReplaceTagsRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let note_id = path.into_inner();
    match repos::tags::replace_tags(pool.get_ref(), note_id, &body.tags).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"ok": true})),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}
