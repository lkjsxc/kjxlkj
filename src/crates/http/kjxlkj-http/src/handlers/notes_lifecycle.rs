use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::RollbackRequest;
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;

/// DELETE /api/notes/{id} — soft-delete per http.md (returns 204).
pub async fn delete_note(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match repos::notes::soft_delete(pool.get_ref(), id).await {
        Ok(_) => {
            let _ = repos::events::append_note_event(
                pool.get_ref(), uuid::Uuid::new_v4(), id, 0,
                "note.deleted",
                &serde_json::json!({}), auth.user_id,
            ).await;
            HttpResponse::NoContent().finish()
        }
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// GET /api/notes/{id}/history
pub async fn note_history(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match repos::events::list_note_events(pool.get_ref(), id).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}

/// POST /api/notes/{id}/rollback
pub async fn rollback_note(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<RollbackRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    let current_version = repos::notes::get_version(pool.get_ref(), id)
        .await.ok().flatten().unwrap_or(0);
    match repos::notes::rollback(pool.get_ref(), id, "", "", current_version + 1).await {
        Ok(Some(note)) => {
            let _ = repos::events::append_note_event(
                pool.get_ref(), uuid::Uuid::new_v4(), id, note.version,
                "note.rolled_back",
                &serde_json::json!({"to_version": body.target_version}), auth.user_id,
            ).await;
            HttpResponse::Ok().json(note)
        }
        Ok(None) => domain_error_response(&DomainError::NotFound {
            entity: "note".into(),
        }),
        Err(e) => domain_error_response(&DomainError::Internal(e.to_string())),
    }
}
