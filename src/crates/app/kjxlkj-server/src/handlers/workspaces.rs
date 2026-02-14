use crate::app_state::AppState;
use crate::authn::require_identity;
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_domain::Role;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpsertWorkspaceMemberRequest {
    role: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/workspaces/{id}/members", web::get().to(list_workspace_members))
        .route(
            "/workspaces/{id}/members/{user_id}",
            web::put().to(upsert_workspace_member),
        );
}

async fn list_workspace_members(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let workspace_id = path.into_inner();

    let members = state
        .workspace_service
        .list_members(identity.user_id, workspace_id)
        .await
        .map_err(|error| match error {
            kjxlkj_workspace::WorkspaceServiceError::Forbidden => {
                ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden")
            }
            _ => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"),
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "members": members,
        "request_id": request_id,
    })))
}

async fn upsert_workspace_member(
    req: HttpRequest,
    path: web::Path<(Uuid, Uuid)>,
    state: web::Data<AppState>,
    body: web::Json<UpsertWorkspaceMemberRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, true).await?;
    let (workspace_id, user_id) = path.into_inner();

    let role = Role::from_str(&body.role)
        .map_err(|_| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "RULE_INVALID", "invalid role"))?;

    state
        .workspace_service
        .upsert_member(identity.user_id, workspace_id, user_id, role, &request_id)
        .await
        .map_err(|error| match error {
            kjxlkj_workspace::WorkspaceServiceError::Forbidden => {
                ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden")
            }
            kjxlkj_workspace::WorkspaceServiceError::InvalidRole => {
                ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "RULE_INVALID", "invalid role")
            }
            _ => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"),
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "workspace_id": workspace_id,
        "user_id": user_id,
        "role": role.as_str(),
        "request_id": request_id,
    })))
}
