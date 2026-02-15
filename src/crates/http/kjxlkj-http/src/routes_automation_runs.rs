//! Automation run routes: launch, list, detail, review.
//! Per /docs/spec/api/http.md and /docs/spec/domain/automation.md.
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::{repo_automation_run, repo_librarian_operation};
use kjxlkj_domain::error::DomainError;
use uuid::Uuid;

use crate::dto::{AutomationRunResponse, LaunchRunRequest, ReviewRunRequest};
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

fn run_to_response(r: &repo_automation_run::AutomationRunRow) -> AutomationRunResponse {
    AutomationRunResponse {
        id: r.id,
        rule_id: r.rule_id,
        status: r.status.clone(),
        result_json: r.result_json.clone(),
    }
}

/// POST /api/automation/rules/{id}/launch
pub async fn launch_run(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<LaunchRunRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let rule_id = path.into_inner();
    // Idempotency: check for existing run with same trigger
    if let Some(trigger_id) = body.triggering_event_id {
        if let Ok(Some(existing)) =
            repo_automation_run::find_run_by_trigger(pool.get_ref(), rule_id, trigger_id).await
        {
            return HttpResponse::Ok().json(run_to_response(&existing));
        }
    }
    let id = Uuid::now_v7();
    match repo_automation_run::create_run(
        pool.get_ref(), id, rule_id, body.triggering_event_id,
    ).await {
        Ok(r) => HttpResponse::Created().json(run_to_response(&r)),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /api/automation/runs?workspace_id=...
pub async fn list_runs(
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
    match repo_automation_run::list_runs(pool.get_ref(), ws_id).await {
        Ok(runs) => {
            let out: Vec<_> = runs.iter().map(run_to_response).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// GET /api/automation/runs/{id}
pub async fn get_run(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let id = path.into_inner();
    match repo_automation_run::find_run(pool.get_ref(), id).await {
        Ok(Some(r)) => HttpResponse::Ok().json(run_to_response(&r)),
        Ok(None) => domain_error_response(
            DomainError::NotFound("run".into()), &rid,
        ),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// POST /api/automation/runs/{id}/review
/// Persists apply/reject decisions for librarian operations.
pub async fn review_run(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReviewRunRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let _run_id = path.into_inner();
    for decision in &body.decisions {
        let status = match decision.decision.as_str() {
            "apply" => "applied",
            "reject" => "rejected",
            _ => return domain_error_response(
                DomainError::BadRequest("decision must be 'apply' or 'reject'".into()), &rid,
            ),
        };
        if let Err(e) = repo_librarian_operation::decide_operation(
            pool.get_ref(), decision.operation_id,
            status, decision.reject_reason.as_deref(),
        ).await {
            return domain_error_response(
                DomainError::Internal(e.to_string()), &rid,
            );
        }
    }
    HttpResponse::Ok().json(serde_json::json!({"status": "reviewed"}))
}
