use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::dto::{LaunchRunRequest, ReviewRunRequest};
use crate::error_response::domain_error_response;
use crate::middleware::session_extractor::get_auth_user;
use kjxlkj_automation::{rules, runs};
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::RunStatus;

/// POST /api/automation/rules/{id}/launch
pub async fn launch_run(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<LaunchRunRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let rule_id = path.into_inner();
    let rule = match rules::get_rule(pool.get_ref(), rule_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(&e),
    };
    match runs::launch_run(
        pool.get_ref(), rule_id, rule.workspace_id, body.trigger_event_id,
    ).await {
        Ok(run) => HttpResponse::Created().json(run),
        Err(e) => domain_error_response(&e),
    }
}

/// GET /api/automation/runs?workspace_id=...
pub async fn list_runs(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return domain_error_response(&DomainError::BadRequest {
            reason: "workspace_id required".into(),
        }),
    };
    match runs::list_runs(pool.get_ref(), ws_id).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => domain_error_response(&e),
    }
}

/// GET /api/automation/runs/{id}
pub async fn get_run(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    match runs::get_run(pool.get_ref(), id).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => domain_error_response(&e),
    }
}

/// POST /api/automation/runs/{id}/review
pub async fn review_run(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<ReviewRunRequest>,
) -> HttpResponse {
    let _auth = match get_auth_user(pool.get_ref(), &req).await {
        Ok(a) => a,
        Err(e) => return domain_error_response(&e),
    };
    let id = path.into_inner();
    let status = match body.decision.as_str() {
        "apply" => RunStatus::Succeeded,
        "reject" => RunStatus::Failed,
        _ => return domain_error_response(&DomainError::BadRequest {
            reason: "decision must be 'apply' or 'reject'".into(),
        }),
    };
    let result_json = body
        .summary
        .as_ref()
        .map(|s| serde_json::json!({"summary": s}));
    let error_msg = if status == RunStatus::Failed {
        Some("rejected by reviewer")
    } else {
        None
    };
    match runs::update_run_status(pool.get_ref(), id, status, result_json, error_msg).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"ok": true})),
        Err(e) => domain_error_response(&e),
    }
}
