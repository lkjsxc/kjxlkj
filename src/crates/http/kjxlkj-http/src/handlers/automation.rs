use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{CreateRuleRequest, UpdateRuleRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_automation::rules;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_rbac::{guard, roles};
pub async fn list_rules(
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
        None => return domain_error_response(&DomainError::BadRequest {
            reason: "workspace_id required".into(),
        }),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), ws_id, auth.user_id, roles::can_manage_automation,
    ).await {
        return domain_error_response(&e);
    }
    match rules::list_rules(pool.get_ref(), ws_id).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/automation/rules
pub async fn create_rule(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateRuleRequest>,
) -> HttpResponse {
    let auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    if let Err(e) = guard::require_workspace_role(
        pool.get_ref(), body.workspace_id, auth.user_id, roles::can_manage_automation,
    ).await {
        return domain_error_response(&e);
    }
    let condition = body.condition.clone().unwrap_or(serde_json::json!({}));
    match rules::create_rule(
        pool.get_ref(), body.workspace_id, &body.name, &body.trigger, condition, body.action.clone(),
    ).await {
        Ok(r) => HttpResponse::Created().json(r),
        Err(e) => domain_error_response(&e),
    }
}

/// PATCH /api/automation/rules/{id}
pub async fn update_rule(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateRuleRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    // Fetch existing rule to fill defaults for unchanged fields.
    let existing = match rules::get_rule(pool.get_ref(), id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(&e),
    };
    let name = body.name.as_deref().unwrap_or(&existing.name);
    let trigger = body.trigger.as_deref().unwrap_or(&existing.trigger);
    let condition = body.condition.as_ref().unwrap_or(&existing.condition_json);
    let action = body.action.as_ref().unwrap_or(&existing.action_json);
    let enabled = body.enabled.unwrap_or(existing.enabled);
    match rules::update_rule(
        pool.get_ref(), id, name, trigger, condition, action, enabled,
    ).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => domain_error_response(&e),
    }
}

/// DELETE /api/automation/rules/{id}
pub async fn delete_rule(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match rules::delete_rule(pool.get_ref(), id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(&e),
    }
}


