use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_workspace;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{UserId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /workspaces/{id}/members per /docs/spec/api/http.md.
pub async fn list_members(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(path.into_inner());
    if let Err(e) = guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        return domain_error_response(e, &rid);
    }
    match repo_workspace::list_members(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<MemberResponse> = rows
                .into_iter()
                .map(|m| MemberResponse {
                    workspace_id: m.workspace_id,
                    user_id: m.user_id,
                    role: m.role,
                    joined_at: m.joined_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// PUT /workspaces/{id}/members/{user_id} per /docs/spec/api/http.md.
pub async fn upsert_member(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpsertMemberRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let (ws_uuid, user_uuid) = path.into_inner();
    let ws_id = WorkspaceId(ws_uuid);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_admin(role) {
        return domain_error_response(e, &rid);
    }
    if guard::parse_role(&body.role).is_none() {
        return domain_error_response(DomainError::BadRequest("invalid role".into()), &rid);
    }
    let target_id = UserId(user_uuid);
    match repo_workspace::upsert_member(pool.get_ref(), ws_id, target_id, &body.role).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "status": "upserted", "request_id": rid
        })),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}
