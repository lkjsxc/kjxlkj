//! Automation rule CRUD routes.
//! Per /docs/spec/api/http.md and /docs/spec/domain/automation.md.
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_automation_rule;
use kjxlkj_domain::error::DomainError;
use uuid::Uuid;

use crate::dto::{AutomationRuleResponse, CreateRuleRequest, UpdateRuleRequest};
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

fn rule_to_response(r: &repo_automation_rule::AutomationRuleRow) -> AutomationRuleResponse {
    AutomationRuleResponse {
        id: r.id,
        workspace_id: r.workspace_id,
        name: r.name.clone(),
        trigger_type: r.trigger_type.clone(),
        condition_json: r.condition_json.clone(),
        action_json: r.action_json.clone(),
        enabled: r.enabled,
    }
}

/// Validate librarian action_json if kind = "librarian_structure".
fn validate_action(action: &serde_json::Value, rid: &str) -> Result<(), HttpResponse> {
    if let Some(kind) = action.get("kind").and_then(|v| v.as_str()) {
        if kind == "librarian_structure" {
            let provider = action.get("provider").and_then(|p| {
                p.get("provider_kind").and_then(|v| v.as_str())
            });
            match provider {
                Some("openrouter" | "lmstudio") => {}
                _ => return Err(domain_error_response(
                    DomainError::BadRequest("unknown provider kind".into()), rid,
                )),
            }
        }
    }
    Ok(())
}

/// POST /api/automation/rules
pub async fn create_rule(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    body: web::Json<CreateRuleRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    if let Err(resp) = validate_action(&body.action_json, &rid) {
        return resp;
    }
    let id = Uuid::now_v7();
    match repo_automation_rule::create_rule(
        pool.get_ref(), id, body.workspace_id,
        &body.name, &body.trigger_type, &body.condition_json, &body.action_json,
    ).await {
        Ok(r) => HttpResponse::Created().json(rule_to_response(&r)),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /api/automation/rules?workspace_id=...
pub async fn list_rules(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return domain_error_response(
            DomainError::BadRequest("workspace_id required".into()), &rid,
        ),
    };
    match repo_automation_rule::list_rules(pool.get_ref(), ws_id).await {
        Ok(rules) => {
            let out: Vec<_> = rules.iter().map(rule_to_response).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// PATCH /api/automation/rules/{id}
pub async fn update_rule(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRuleRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let id = path.into_inner();
    let existing = match repo_automation_rule::find_rule(pool.get_ref(), id).await {
        Ok(Some(r)) => r,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("rule".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };
    let name = body.name.as_deref().unwrap_or(&existing.name);
    let trigger = body.trigger_type.as_deref().unwrap_or(&existing.trigger_type);
    let cond = body.condition_json.as_ref().unwrap_or(&existing.condition_json);
    let action = body.action_json.as_ref().unwrap_or(&existing.action_json);
    let enabled = body.enabled.unwrap_or(existing.enabled);
    if let Err(resp) = validate_action(action, &rid) {
        return resp;
    }
    match repo_automation_rule::update_rule(
        pool.get_ref(), id, name, trigger, cond, action, enabled,
    ).await {
        Ok(r) => HttpResponse::Ok().json(rule_to_response(&r)),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// DELETE /api/automation/rules/{id}
pub async fn delete_rule(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let id = path.into_inner();
    match repo_automation_rule::delete_rule(pool.get_ref(), id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
