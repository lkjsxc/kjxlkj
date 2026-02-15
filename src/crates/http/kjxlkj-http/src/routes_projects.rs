//! Project update/delete route handlers per /docs/spec/api/http.md.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_project;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::ProjectId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto_views::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// PATCH /projects/{id} per /docs/spec/api/http.md.
pub async fn update_project(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProjectRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let project_id = ProjectId(path.into_inner());
    let name = body.name.as_deref().unwrap_or("");
    let description = body.description.as_deref();
    match repo_project::update_project(pool.get_ref(), project_id, name, description).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "updated", "request_id": rid
        })),
        Ok(false) => {
            domain_error_response(DomainError::NotFound("project".into()), &rid)
        }
        Err(e) => {
            domain_error_response(DomainError::Internal(e.to_string()), &rid)
        }
    }
}

/// DELETE /projects/{id} per /docs/spec/api/http.md.
pub async fn delete_project(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let project_id = ProjectId(path.into_inner());
    match repo_project::delete_project(pool.get_ref(), project_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => {
            domain_error_response(DomainError::NotFound("project".into()), &rid)
        }
        Err(e) => {
            domain_error_response(DomainError::Internal(e.to_string()), &rid)
        }
    }
}
