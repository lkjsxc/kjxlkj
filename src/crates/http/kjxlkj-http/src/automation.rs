// Automation handlers per /docs/spec/api/http.md
use actix_web::{web, HttpResponse};
use kjxlkj_auth::middleware::{require_role, AuthSession};
use kjxlkj_automation::rules;
use kjxlkj_db::repo::automation as auto_repo;
use kjxlkj_domain::types::{AutomationRule, AutomationRun, Role, RunStatus};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::ErrorBody;

/// GET /api/automation/rules
pub async fn list_rules(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    query: web::Query<WsFilter>,
) -> HttpResponse {
    match auto_repo::list_rules(pool.get_ref(), query.workspace_id).await {
        Ok(rules) => HttpResponse::Ok().json(rules),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

/// POST /api/automation/rules — admin+ only
pub async fn create_rule(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    body: web::Json<AutomationRule>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    if let Err(_) = require_role(&auth, Role::Admin) {
        return HttpResponse::Forbidden().json(ErrorBody {
            code: "FORBIDDEN".into(), message: "Admin role required".into(),
            details: None, request_id: rid,
        });
    }

    // Validate action
    if let Err(e) = rules::validate_rule_action(&body.action_json) {
        return HttpResponse::UnprocessableEntity().json(ErrorBody {
            code: "RULE_INVALID".into(), message: e,
            details: None, request_id: rid,
        });
    }

    let rule = AutomationRule {
        id: Uuid::now_v7(),
        workspace_id: body.workspace_id,
        trigger: body.trigger.clone(),
        condition_json: body.condition_json.clone(),
        action_json: body.action_json.clone(),
        enabled: body.enabled,
    };

    match auto_repo::insert_rule(pool.get_ref(), &rule).await {
        Ok(()) => HttpResponse::Created().json(&rule),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// PATCH /api/automation/rules/{id}
pub async fn update_rule(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let action = body.get("action_json").cloned().unwrap_or_default();
    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    if let Err(e) = rules::validate_rule_action(&action) {
        return HttpResponse::UnprocessableEntity().json(ErrorBody {
            code: "RULE_INVALID".into(), message: e,
            details: None, request_id: rid,
        });
    }

    match auto_repo::update_rule(pool.get_ref(), path.into_inner(), &action, enabled).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"status": "updated"})),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "Rule not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// DELETE /api/automation/rules/{id}
pub async fn delete_rule(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    match auto_repo::delete_rule(pool.get_ref(), path.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(), message: "Rule not found".into(),
            details: None, request_id: rid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// POST /api/automation/rules/{id}/launch
pub async fn launch_run(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = Uuid::now_v7().to_string();
    let run = AutomationRun {
        id: Uuid::now_v7(),
        rule_id: path.into_inner(),
        status: RunStatus::Pending,
        started_at: String::new(),
        finished_at: None,
        result_json: None,
    };
    match auto_repo::insert_run(pool.get_ref(), &run).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "run_id": run.id, "status": "pending"
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: rid,
        }),
    }
}

/// GET /api/automation/runs
pub async fn list_runs(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    query: web::Query<WsFilter>,
) -> HttpResponse {
    match auto_repo::list_runs(pool.get_ref(), query.workspace_id).await {
        Ok(runs) => HttpResponse::Ok().json(runs),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            code: "INTERNAL_ERROR".into(), message: e.to_string(),
            details: None, request_id: Uuid::now_v7().to_string(),
        }),
    }
}

#[derive(serde::Deserialize)]
pub struct WsFilter {
    pub workspace_id: Uuid,
}

/// GET /api/automation/runs/{id} — run status/details with librarian operations
pub async fn run_detail(
    pool: web::Data<PgPool>,
    _auth: AuthSession,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let run_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();

    let run_row: Option<(String, Option<String>, Option<serde_json::Value>)> =
        sqlx::query_as(
            "SELECT status, error_detail, result_json FROM automation_runs WHERE id = $1",
        )
        .bind(run_id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    let Some((status, error_detail, result_json)) = run_row else {
        return HttpResponse::NotFound().json(ErrorBody {
            code: "NOT_FOUND".into(),
            message: "Run not found".into(),
            details: None,
            request_id: rid,
        });
    };

    // Fetch librarian operations if any
    let ops: Vec<(Uuid, i32, String, Option<Uuid>, Option<String>, Option<f32>, String)> =
        sqlx::query_as(
            "SELECT id, operation_index, kind, target_note_id, title, confidence, status
             FROM librarian_operations WHERE run_id = $1 ORDER BY operation_index",
        )
        .bind(run_id)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    let operations: Vec<serde_json::Value> = ops
        .iter()
        .map(|(id, idx, kind, target, title, conf, st)| {
            serde_json::json!({
                "id": id,
                "index": idx,
                "kind": kind,
                "target_note_id": target,
                "title": title,
                "confidence": conf,
                "status": st,
            })
        })
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "run_id": run_id,
        "status": status,
        "error_detail": error_detail,
        "result_json": result_json,
        "operations": operations,
    }))
}

/// POST /api/automation/runs/{id}/review — persist accept/reject decisions
pub async fn review_run(
    pool: web::Data<PgPool>,
    auth: AuthSession,
    path: web::Path<Uuid>,
    body: web::Json<ReviewRequest>,
) -> HttpResponse {
    let run_id = path.into_inner();
    let rid = Uuid::now_v7().to_string();
    let reviewer_id = auth.user.id;

    for decision in &body.decisions {
        let status = if decision.accept { "accepted" } else { "rejected" };
        if let Err(e) = sqlx::query(
            "UPDATE librarian_operations
             SET status = $1, review_decision = $1, reviewer_id = $2, reviewed_at = now()
             WHERE run_id = $3 AND id = $4",
        )
        .bind(status)
        .bind(reviewer_id)
        .bind(run_id)
        .bind(decision.operation_id)
        .execute(pool.get_ref())
        .await
        {
            return HttpResponse::InternalServerError().json(ErrorBody {
                code: "INTERNAL_ERROR".into(),
                message: e.to_string(),
                details: None,
                request_id: rid,
            });
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "run_id": run_id,
        "reviewed_count": body.decisions.len()
    }))
}

#[derive(serde::Deserialize)]
pub struct ReviewRequest {
    pub decisions: Vec<ReviewDecision>,
}

#[derive(serde::Deserialize)]
pub struct ReviewDecision {
    pub operation_id: Uuid,
    pub accept: bool,
}
