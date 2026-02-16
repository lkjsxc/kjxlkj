/// Automation route handlers per /docs/spec/api/http.md
///
/// GET  /api/automation/rules          — list rules
/// POST /api/automation/rules          — create rule
/// PATCH /api/automation/rules/{id}    — update rule
/// POST /api/automation/rules/{id}/launch — launch run
/// GET  /api/automation/runs           — list runs
/// GET  /api/automation/runs/{id}      — get run
/// POST /api/automation/runs/{id}/review — review run
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use kjxlkj_db::repo::AutomationRepo;
use kjxlkj_domain::automation::*;
use uuid::Uuid;

pub async fn list_rules(State(state): State<AppState>) -> Response {
    match state.automation_repo.list_rules(Uuid::nil()) {
        Ok(rules) => Json(serde_json::to_value(&rules).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn create_rule(
    State(state): State<AppState>,
    Json(input): Json<CreateRuleInput>,
) -> Response {
    let now = chrono::Utc::now().naive_utc();
    let rule = AutomationRule {
        id: Uuid::new_v4(),
        workspace_id: input.workspace_id,
        trigger: input.trigger,
        condition_json: input.condition_json,
        action_json: input.action_json,
        enabled: input.enabled.unwrap_or(true),
        created_at: now,
        updated_at: now,
    };
    if let Err(e) = state.automation_repo.create_rule(&rule) {
        return domain_error_response(e);
    }
    (StatusCode::CREATED, Json(serde_json::json!({"id": rule.id}))).into_response()
}

pub async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Response {
    let existing = match state.automation_repo.list_rules(Uuid::nil()) {
        Ok(rules) => rules.into_iter().find(|r| r.id == id),
        Err(e) => return domain_error_response(e),
    };
    let mut rule = match existing {
        Some(r) => r,
        None => return domain_error_response(
            kjxlkj_domain::DomainError::BadRequest("rule not found".into()),
        ),
    };
    if let Some(enabled) = input.get("enabled").and_then(|v| v.as_bool()) {
        rule.enabled = enabled;
    }
    rule.updated_at = chrono::Utc::now().naive_utc();
    if let Err(e) = state.automation_repo.update_rule(&rule) {
        return domain_error_response(e);
    }
    Json(serde_json::json!({"updated": true})).into_response()
}

pub async fn launch_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<Uuid>,
) -> Response {
    let now = chrono::Utc::now().naive_utc();
    let run = AutomationRun {
        id: Uuid::new_v4(),
        rule_id,
        status: RunStatus::Queued,
        started_at: None,
        finished_at: None,
        result_json: None,
        created_at: now,
    };
    if let Err(e) = state.automation_repo.create_run(&run) {
        return domain_error_response(e);
    }
    (StatusCode::ACCEPTED, Json(serde_json::json!({
        "run_id": run.id,
        "status": "queued",
    }))).into_response()
}

pub async fn list_runs(State(state): State<AppState>) -> Response {
    match state.automation_repo.list_runs(Uuid::nil()) {
        Ok(runs) => Json(serde_json::to_value(&runs).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn get_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    match state.automation_repo.get_run(id) {
        Ok(Some(run)) => Json(serde_json::to_value(&run).unwrap()).into_response(),
        Ok(None) => domain_error_response(
            kjxlkj_domain::DomainError::BadRequest("run not found".into()),
        ),
        Err(e) => domain_error_response(e),
    }
}

pub async fn review_run(
    Path(_id): Path<Uuid>,
    Json(_input): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({"reviewed": true}))
}
