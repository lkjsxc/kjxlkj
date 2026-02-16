use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use kjxlkj_db::repo_automation;
use kjxlkj_domain::error::ErrorCode;
use kjxlkj_rbac::check::{self, RbacError};
use kjxlkj_domain::permission::Role;
use crate::extract;
use crate::response::error_response;

#[derive(Deserialize)]
pub struct CreateRuleBody {
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ListRulesQuery {
    pub workspace_id: Uuid,
}

/// GET /api/automation/rules
pub async fn list_rules(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<ListRulesQuery>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    if let Err(e) = check::require_role(&pool, query.workspace_id, identity.user_id, Role::Admin).await {
        return rbac_err(e);
    }

    match repo_automation::list_rules(&pool, query.workspace_id).await {
        Ok(rules) => HttpResponse::Ok().json(
            rules.iter().map(|r| serde_json::json!({
                "id": r.id,
                "workspace_id": r.workspace_id,
                "trigger": r.trigger_kind,
                "condition_json": r.condition_json,
                "action_json": r.action_json,
                "enabled": r.enabled,
                "created_at": r.created_at.to_string(),
            })).collect::<Vec<_>>()
        ),
        Err(_) => error_response(ErrorCode::InternalError, "failed to list rules"),
    }
}

/// POST /api/automation/rules
pub async fn create_rule(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateRuleBody>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    if let Err(e) = check::require_role(&pool, body.workspace_id, identity.user_id, Role::Admin).await {
        return rbac_err(e);
    }

    // Validate action_json for kjxlkj_agent rules
    if body.action_json.get("kind").and_then(|v| v.as_str()) == Some("kjxlkj_agent") {
        if body.action_json.get("mode").is_none() {
            return error_response(ErrorCode::RuleInvalid, "agent rule requires mode");
        }
    }

    let id = Uuid::now_v7();
    match repo_automation::create_rule(&pool, id, body.workspace_id, &body.trigger, &body.condition_json, &body.action_json).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({"id": id})),
        Err(_) => error_response(ErrorCode::InternalError, "failed to create rule"),
    }
}

/// POST /api/automation/rules/{id}/launch
pub async fn launch_run(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };
    if let Err(resp) = extract::validate_csrf(&req, &identity) {
        return resp;
    }

    let rule_id = path.into_inner();
    let run_id = Uuid::now_v7();
    match repo_automation::create_run(&pool, run_id, rule_id, None).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "run_id": run_id,
            "rule_id": rule_id,
            "status": "queued",
        })),
        Err(_) => error_response(ErrorCode::InternalError, "failed to launch run"),
    }
}

/// GET /api/automation/runs/{id}
pub async fn get_run(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _identity = match extract::require_auth(&req, &pool).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let run_id = path.into_inner();
    match repo_automation::get_run(&pool, run_id).await {
        Ok(Some(run)) => HttpResponse::Ok().json(serde_json::json!({
            "id": run.id,
            "rule_id": run.rule_id,
            "status": run.status,
            "started_at": run.started_at.to_string(),
            "finished_at": run.finished_at.map(|t| t.to_string()),
            "result_json": run.result_json,
        })),
        Ok(None) => error_response(ErrorCode::NoteNotFound, "run not found"),
        Err(_) => error_response(ErrorCode::InternalError, "failed to get run"),
    }
}

fn rbac_err(e: RbacError) -> HttpResponse {
    match e {
        RbacError::Forbidden => error_response(ErrorCode::RoleForbidden, "role forbidden"),
        RbacError::NotMember => error_response(ErrorCode::WorkspaceForbidden, "not member"),
        RbacError::Db(_) => error_response(ErrorCode::InternalError, "auth check failed"),
    }
}
