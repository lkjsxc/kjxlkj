use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use kjxlkj_workspace::service as ws_svc;
use kjxlkj_domain::error::ErrorCode;
use crate::extract;
use crate::response::error_response;

#[derive(Deserialize)]
pub struct CreateWorkspaceBody {
    pub slug: String,
    pub name: String,
}

/// GET /api/workspaces
pub async fn list_workspaces(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    match ws_svc::list_workspaces(&pool, identity.user_id).await {
        Ok(workspaces) => HttpResponse::Ok().json(workspaces),
        Err(_) => error_response(ErrorCode::InternalError, "failed to list workspaces"),
    }
}

/// POST /api/workspaces
pub async fn create_workspace(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateWorkspaceBody>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    match ws_svc::create_workspace(&pool, &body.slug, &body.name, identity.user_id).await {
        Ok(ws) => HttpResponse::Created().json(ws),
        Err(_) => error_response(ErrorCode::InternalError, "failed to create workspace"),
    }
}
