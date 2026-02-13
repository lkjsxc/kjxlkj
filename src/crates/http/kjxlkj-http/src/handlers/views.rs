use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateViewRequest, UpdateViewRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_rbac::{guard, roles};
use kjxlkj_workspace::view_service;

/// GET /api/views?workspace_id=...
pub async fn list_views(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => {
            return domain_error_response(&DomainError::BadRequest {
                reason: "workspace_id required".into(),
            })
        }
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_view_workspace,
    ).await {
        return domain_error_response(&e);
    }
    match view_service::list_views(pool.get_ref(), ws_id).await {
        Ok(views) => HttpResponse::Ok().json(views),
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/views
pub async fn create_view(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateViewRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), body.workspace_id, auth.user_id, roles::can_manage_views,
    ).await {
        return domain_error_response(&e);
    }
    let filter = body.filter.clone().unwrap_or(serde_json::json!({}));
    let sort = body.sort.clone().unwrap_or(serde_json::json!({}));
    match view_service::create_view(
        pool.get_ref(), body.workspace_id, &body.name, filter, sort, auth.user_id,
    ).await {
        Ok(v) => HttpResponse::Created().json(v),
        Err(e) => domain_error_response(&e),
    }
}

/// PATCH /api/views/{id}
pub async fn update_view(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateViewRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    let name = body.name.as_deref().unwrap_or("");
    let filter = body.filter.clone().unwrap_or(serde_json::json!({}));
    let sort = body.sort.clone().unwrap_or(serde_json::json!({}));
    match view_service::update_view(
        pool.get_ref(), id, name, &filter, &sort,
    ).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => domain_error_response(&e),
    }
}

/// DELETE /api/views/{id}
pub async fn delete_view(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match view_service::delete_view(pool.get_ref(), id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&e),
    }
}
