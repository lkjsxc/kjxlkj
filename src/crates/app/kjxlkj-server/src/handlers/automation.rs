use crate::app_state::AppState;
use crate::authn::{client_key, require_identity};
use crate::error::{new_request_id, ApiError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repos;
use kjxlkj_db::repos::automation::CreateAutomationRuleInput;
use kjxlkj_domain::Role;
use kjxlkj_rbac::{ensure_automation_manage, ensure_workspace_member_read};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ListRulesQuery {
    workspace_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct ListRunsQuery {
    workspace_id: Uuid,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CreateRuleRequest {
    workspace_id: Uuid,
    trigger: String,
    condition_json: serde_json::Value,
    action_json: serde_json::Value,
    enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct UpdateRuleRequest {
    trigger: Option<String>,
    condition_json: Option<serde_json::Value>,
    action_json: Option<serde_json::Value>,
    enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct LaunchRunRequest {
    workspace_id: Uuid,
    note_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
struct ReviewDecision {
    operation_id: String,
    decision: String,
}

#[derive(Debug, Deserialize)]
struct ReviewRunRequest {
    apply: bool,
    decisions: Vec<ReviewDecision>,
}

#[derive(Debug, Clone)]
struct LibrarianProviderConfig {
    provider_kind: String,
    model: String,
    base_url: String,
    timeout_ms: u64,
    retry_limit: u8,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    api_key: Option<String>,
}

#[derive(Debug)]
struct ProviderCallOutcome {
    raw_response: String,
    attempts: u8,
}

#[derive(Debug)]
struct ProviderFailure {
    code: &'static str,
    detail: String,
    retryable: bool,
}

#[derive(Debug, Clone)]
struct LibrarianPlanConfig {
    goal: String,
    scope: String,
    taxonomy_json: serde_json::Value,
    style_profile: String,
    strict_mode: bool,
    max_operations: usize,
    allow_delete: bool,
}

#[derive(Debug, Clone)]
struct LibrarianActionConfig {
    provider: LibrarianProviderConfig,
    plan: LibrarianPlanConfig,
}

#[derive(Debug, Clone)]
struct LibrarianOperationCandidate {
    operation_id: String,
    kind: String,
    target_note_id: Option<String>,
    target_path: Option<String>,
    title: Option<String>,
    body_markdown: Option<String>,
    reason: Option<String>,
    confidence: Option<f32>,
}

#[derive(Debug)]
struct ActionExecutionError {
    code: String,
    detail: String,
    result_json: Option<serde_json::Value>,
}

#[derive(Debug)]
struct ParsedLibrarianResponse {
    request_id: String,
    status: String,
    summary: String,
    operations: Vec<LibrarianOperationCandidate>,
    warnings: Vec<String>,
}

#[derive(Debug)]
struct LibrarianParseFailure {
    code: String,
    detail: String,
    errors: Vec<String>,
}

#[derive(Debug, Clone)]
struct XmlElementNode {
    name: String,
    text: String,
    children: Vec<XmlElementNode>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/automation/rules", web::get().to(list_rules))
        .route("/automation/rules", web::post().to(create_rule))
        .route("/automation/rules/{id}", web::patch().to(update_rule))
        .route("/automation/rules/{id}", web::delete().to(delete_rule))
        .route("/automation/rules/{id}/launch", web::post().to(launch_rule_run))
        .route("/automation/runs", web::get().to(list_runs))
        .route("/automation/runs/{id}/review", web::post().to(review_run))
        .route("/automation/runs/{id}", web::get().to(get_run));
}

async fn list_runs(
    req: HttpRequest,
    query: web::Query<ListRunsQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    let workspace_role = actor_workspace_role(&state, query.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let runs = repos::automation::list_runs(&state.pool, query.workspace_id, limit)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "runs": runs.into_iter().map(run_json).collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn list_rules(
    req: HttpRequest,
    query: web::Query<ListRulesQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;

    let workspace_role = actor_workspace_role(&state, query.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let rules = repos::automation::list_rules(&state.pool, query.workspace_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "rules": rules.into_iter().map(rule_json).collect::<Vec<_>>(),
        "request_id": request_id,
    })))
}

async fn create_rule(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateRuleRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_automation_rate_limit(&state, &req, "automation-rule-create", &request_id)?;
    let identity = require_identity(&req, &state, true).await?;

    validate_rule_payload(&body.trigger, &body.condition_json, &body.action_json)?;

    let workspace_role = actor_workspace_role(&state, body.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let rule = repos::automation::create_rule(
        &state.pool,
        CreateAutomationRuleInput {
            workspace_id: body.workspace_id,
            trigger: body.trigger.clone(),
            condition_json: body.condition_json.clone(),
            action_json: body.action_json.clone(),
            enabled: body.enabled.unwrap_or(true),
            actor_id: identity.user_id,
        },
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid automation rule payload"))?;

    Ok(HttpResponse::Created().json(json!({
        "rule": rule_json(rule),
        "request_id": request_id,
    })))
}

async fn update_rule(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<UpdateRuleRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_automation_rate_limit(&state, &req, "automation-rule-update", &request_id)?;
    let identity = require_identity(&req, &state, true).await?;
    let rule_id = path.into_inner();

    if body.trigger.is_none()
        && body.condition_json.is_none()
        && body.action_json.is_none()
        && body.enabled.is_none()
    {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "BAD_REQUEST",
            "at least one field is required",
        ));
    }

    let existing = repos::automation::get_rule(&state.pool, rule_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RULE_NOT_FOUND", "automation rule not found"))?;

    let next_trigger = body.trigger.as_deref().unwrap_or(&existing.trigger);
    let next_condition = body
        .condition_json
        .as_ref()
        .unwrap_or(&existing.condition_json);
    let next_action = body.action_json.as_ref().unwrap_or(&existing.action_json);
    validate_rule_payload(next_trigger, next_condition, next_action)?;

    let workspace_role = actor_workspace_role(&state, existing.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let updated = repos::automation::update_rule(
        &state.pool,
        rule_id,
        body.trigger.clone(),
        body.condition_json.clone(),
        body.action_json.clone(),
        body.enabled,
        identity.user_id,
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", "invalid automation rule payload"))?
    .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RULE_NOT_FOUND", "automation rule not found"))?;

    Ok(HttpResponse::Ok().json(json!({
        "rule": rule_json(updated),
        "request_id": request_id,
    })))
}

async fn delete_rule(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_automation_rate_limit(&state, &req, "automation-rule-delete", &request_id)?;
    let identity = require_identity(&req, &state, true).await?;
    let rule_id = path.into_inner();

    let existing = repos::automation::get_rule(&state.pool, rule_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RULE_NOT_FOUND", "automation rule not found"))?;

    let workspace_role = actor_workspace_role(&state, existing.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let deleted = repos::automation::delete_rule(&state.pool, rule_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    if !deleted {
        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "RULE_NOT_FOUND",
            "automation rule not found",
        ));
    }

    Ok(HttpResponse::NoContent().finish())
}

async fn get_run(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    let identity = require_identity(&req, &state, false).await?;
    let run_id = path.into_inner();

    let run = repos::automation::get_run(&state.pool, run_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RUN_NOT_FOUND", "automation run not found"))?;

    let workspace_role = actor_workspace_role(&state, run.workspace_id, identity.user_id).await?;
    ensure_workspace_member_read(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    Ok(HttpResponse::Ok().json(json!({
        "run": run_json(run),
        "request_id": request_id,
    })))
}

async fn launch_rule_run(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<LaunchRunRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_automation_rate_limit(&state, &req, "automation-run-launch", &request_id)?;
    let identity = require_identity(&req, &state, true).await?;
    let rule_id = path.into_inner();

    let rule = repos::automation::get_rule(&state.pool, rule_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RULE_NOT_FOUND", "automation rule not found"))?;

    if rule.workspace_id != body.workspace_id {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "BAD_REQUEST",
            "workspace mismatch for rule launch",
        ));
    }

    if !rule.enabled {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_DISABLED",
            "automation rule is disabled",
        ));
    }

    let workspace_role = actor_workspace_role(&state, rule.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let (provider_kind, model) = extract_rule_provider_metadata(&rule);
    let triggering_event_id = format!("manual:{}:{}", rule.id, Uuid::now_v7());
    let queued = repos::automation::queue_run(
        &state.pool,
        rule.id,
        rule.workspace_id,
        &triggering_event_id,
        provider_kind.as_deref(),
        model.as_deref(),
        identity.user_id,
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    let run = if let Some(running) = repos::automation::mark_run_running(&state.pool, queued.id, identity.user_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    {
        let event_payload = json!({
            "workspace_id": rule.workspace_id,
            "note_id": body.note_id,
            "source": "manual_launch",
            "rule_id": rule.id,
        });

        match execute_rule_action(&state, &rule, &event_payload).await {
            Ok(result_json) => repos::automation::mark_run_succeeded(
                &state.pool,
                running.id,
                identity.user_id,
                result_json,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
            .unwrap_or(running),
            Err(error) => repos::automation::mark_run_failed(
                &state.pool,
                running.id,
                identity.user_id,
                &error.code,
                &error.detail,
                error.result_json,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
            .unwrap_or(running),
        }
    } else {
        queued
    };

    let _ = repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        Some(rule.workspace_id),
        "automation_run_launch",
        json!({
            "rule_id": rule.id,
            "run_id": run.id,
            "status": run.status,
            "triggering_event_id": run.triggering_event_id,
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(json!({
        "run": run_json(run),
        "request_id": request_id,
    })))
}

async fn review_run(
    req: HttpRequest,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
    body: web::Json<ReviewRunRequest>,
) -> Result<HttpResponse, ApiError> {
    let request_id = new_request_id();
    enforce_automation_rate_limit(&state, &req, "automation-run-review", &request_id)?;
    let identity = require_identity(&req, &state, true).await?;
    let run_id = path.into_inner();

    let run = repos::automation::get_run(&state.pool, run_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RUN_NOT_FOUND", "automation run not found"))?;

    let workspace_role = actor_workspace_role(&state, run.workspace_id, identity.user_id).await?;
    ensure_automation_manage(workspace_role)
        .map_err(|_| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    let mut decisions_payload = Vec::with_capacity(body.decisions.len());
    let mut decision_map: HashMap<String, String> = HashMap::with_capacity(body.decisions.len());
    for decision in &body.decisions {
        if decision.operation_id.trim().is_empty() {
            return Err(ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "BAD_REQUEST",
                "operation_id is required",
            ));
        }

        if !matches!(decision.decision.as_str(), "accept" | "reject") {
            return Err(ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "BAD_REQUEST",
                "decision must be accept or reject",
            ));
        }

        decisions_payload.push(json!({
            "operation_id": decision.operation_id,
            "decision": decision.decision,
        }));
        decision_map.insert(decision.operation_id.clone(), decision.decision.clone());
    }

    let parsed_operations = run
        .result_json
        .get("operation_report")
        .and_then(|value| value.get("parsed_operations"))
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let mut applied_operations: Vec<serde_json::Value> = Vec::new();
    let mut rejected_operations: Vec<serde_json::Value> = Vec::new();

    if body.apply {
        for (index, operation) in parsed_operations.iter().enumerate() {
            let operation_id = operation
                .get("operation_id")
                .and_then(|value| value.as_str())
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("op-{}", index + 1));

            let kind = operation
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
                .to_owned();

            if decision_map
                .get(&operation_id)
                .map(|value| value.as_str())
                != Some("accept")
            {
                rejected_operations.push(json!({
                    "operation_id": operation_id,
                    "kind": kind,
                    "reason": "USER_REJECTED",
                }));
                continue;
            }

            match kind.as_str() {
                "create_note" => {
                    let title = operation
                        .get("title")
                        .and_then(|value| value.as_str())
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .unwrap_or("Generated note")
                        .to_owned();

                    let markdown = operation
                        .get("body_markdown")
                        .and_then(|value| value.as_str())
                        .unwrap_or_default()
                        .to_owned();

                    match repos::notes::create_note(
                        &state.pool,
                        identity.user_id,
                        repos::notes::CreateNoteInput {
                            workspace_id: run.workspace_id,
                            project_id: None,
                            title,
                            note_kind: "markdown".to_owned(),
                            access_scope: "workspace".to_owned(),
                            markdown,
                        },
                    )
                    .await
                    {
                        Ok((stream, _)) => applied_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "note_id": stream.id,
                            "result": "created",
                        })),
                        Err(_) => rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "APPLY_FAILED",
                        })),
                    }
                }
                "rewrite_note" => {
                    let target_note_id = operation
                        .get("target_note_id")
                        .and_then(|value| value.as_str())
                        .and_then(|value| Uuid::parse_str(value).ok());

                    let Some(target_note_id) = target_note_id else {
                        rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "MISSING_TARGET_NOTE",
                        }));
                        continue;
                    };

                    let Some((stream, projection)) = repos::notes::get_note(&state.pool, target_note_id)
                        .await
                        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
                    else {
                        rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "TARGET_NOTE_NOT_FOUND",
                        }));
                        continue;
                    };

                    let next_markdown = operation
                        .get("body_markdown")
                        .and_then(|value| value.as_str())
                        .unwrap_or_default();

                    let mut patch_ops: Vec<repos::notes_patch::PatchOp> = Vec::new();
                    if !projection.markdown.is_empty() {
                        patch_ops.push(repos::notes_patch::PatchOp::Delete {
                            delete: projection.markdown.chars().count(),
                        });
                    }
                    if !next_markdown.is_empty() {
                        patch_ops.push(repos::notes_patch::PatchOp::Insert {
                            insert: next_markdown.to_owned(),
                        });
                    }

                    if patch_ops.is_empty() {
                        applied_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "note_id": target_note_id,
                            "result": "no_change",
                        }));
                        continue;
                    }

                    match repos::notes::apply_note_patch(
                        &state.pool,
                        identity.user_id,
                        target_note_id,
                        stream.current_version,
                        &patch_ops,
                        &format!("run-review:{}:{}", run.id, operation_id),
                    )
                    .await
                    {
                        Ok(result) => applied_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "note_id": target_note_id,
                            "version": result.version,
                            "event_seq": result.event_seq,
                            "result": "rewritten",
                        })),
                        Err(_) => rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "APPLY_FAILED",
                        })),
                    }
                }
                "retitle_note" => {
                    let target_note_id = operation
                        .get("target_note_id")
                        .and_then(|value| value.as_str())
                        .and_then(|value| Uuid::parse_str(value).ok());
                    let title = operation
                        .get("title")
                        .and_then(|value| value.as_str())
                        .map(str::trim)
                        .filter(|value| !value.is_empty());

                    let (Some(target_note_id), Some(title)) = (target_note_id, title) else {
                        rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "INVALID_RETITLE_PAYLOAD",
                        }));
                        continue;
                    };

                    let Some((stream, _)) = repos::notes::get_note(&state.pool, target_note_id)
                        .await
                        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
                    else {
                        rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "TARGET_NOTE_NOT_FOUND",
                        }));
                        continue;
                    };

                    match repos::notes::update_note_title(
                        &state.pool,
                        identity.user_id,
                        target_note_id,
                        stream.current_version,
                        title,
                    )
                    .await
                    {
                        Ok(result) => applied_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "note_id": target_note_id,
                            "version": result.version,
                            "event_seq": result.event_seq,
                            "result": "retitled",
                        })),
                        Err(_) => rejected_operations.push(json!({
                            "operation_id": operation_id,
                            "kind": kind,
                            "reason": "APPLY_FAILED",
                        })),
                    }
                }
                _ => rejected_operations.push(json!({
                    "operation_id": operation_id,
                    "kind": kind,
                    "reason": "UNSUPPORTED_APPLY_KIND",
                })),
            }
        }
    }

    let mut result_json = run.result_json.clone();
    result_json["review"] = json!({
        "apply": body.apply,
        "decisions": decisions_payload.clone(),
    });
    result_json["operation_report"]["applied_operations"] = json!(applied_operations);
    result_json["operation_report"]["rejected_operations"] = json!(rejected_operations);

    let reviewed = repos::automation::record_run_review(
        &state.pool,
        run_id,
        identity.user_id,
        body.apply,
        decisions_payload,
        result_json,
    )
    .await
    .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
    .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "RUN_NOT_FOUND", "automation run not found"))?;

    let _ = repos::audit::emit_security_event(
        &state.pool,
        &request_id,
        Some(identity.user_id),
        Some(reviewed.workspace_id),
        "automation_run_reviewed",
        json!({
            "run_id": reviewed.id,
            "rule_id": reviewed.rule_id,
            "apply": body.apply,
            "decision_count": body.decisions.len(),
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(json!({
        "run": run_json(reviewed),
        "request_id": request_id,
    })))
}

fn validate_rule_payload(
    trigger: &str,
    condition_json: &serde_json::Value,
    action_json: &serde_json::Value,
) -> Result<(), ApiError> {
    if trigger.trim().is_empty()
        || trigger.len() > 64
        || !trigger
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
    {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "invalid trigger",
        ));
    }

    if !condition_json.is_object() {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "condition_json must be an object",
        ));
    }

    let Some(action_obj) = action_json.as_object() else {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "action_json must be an object",
        ));
    };

    let kind = action_obj
        .get("kind")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "RULE_INVALID",
                "action kind is required",
            )
        })?;

    if kind == "librarian_structure" {
        parse_librarian_action_config(action_obj).map_err(|detail| {
            ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "RULE_INVALID",
                "invalid librarian provider config",
            )
            .with_details(json!({ "detail": detail }))
        })?;
    }

    Ok(())
}

fn rule_json(rule: kjxlkj_db::models::DbAutomationRule) -> serde_json::Value {
    json!({
        "id": rule.id,
        "workspace_id": rule.workspace_id,
        "trigger": rule.trigger,
        "condition_json": rule.condition_json,
        "action_json": rule.action_json,
        "enabled": rule.enabled,
        "created_by": rule.created_by,
        "updated_by": rule.updated_by,
        "created_at": rule.created_at.format(&Rfc3339).unwrap_or_else(|_| rule.created_at.to_string()),
        "updated_at": rule.updated_at.format(&Rfc3339).unwrap_or_else(|_| rule.updated_at.to_string()),
    })
}

fn run_json(run: kjxlkj_db::models::DbAutomationRun) -> serde_json::Value {
    json!({
        "id": run.id,
        "rule_id": run.rule_id,
        "workspace_id": run.workspace_id,
        "triggering_event_id": run.triggering_event_id,
        "status": run.status,
        "provider_kind": run.provider_kind,
        "model": run.model,
        "result_json": run.result_json,
        "error_code": run.error_code,
        "error_detail": run.error_detail,
        "started_at": run.started_at.and_then(|value| value.format(&Rfc3339).ok()),
        "finished_at": run.finished_at.and_then(|value| value.format(&Rfc3339).ok()),
        "created_at": run.created_at.format(&Rfc3339).unwrap_or_else(|_| run.created_at.to_string()),
    })
}

pub(crate) async fn evaluate_workspace_event(
    state: &AppState,
    request_id: &str,
    actor_id: Uuid,
    workspace_id: Uuid,
    trigger: &str,
    triggering_event_id: &str,
    event_payload: &serde_json::Value,
) -> Result<(), ApiError> {
    let rules = repos::automation::list_enabled_rules_by_trigger(&state.pool, workspace_id, trigger)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

    for rule in rules {
        let (provider_kind, model) = extract_rule_provider_metadata(&rule);
        let trigger_key = format!("automation-trigger:{}:{}", workspace_id, rule.id);
        if !state.automation_rate_limiter.check(&trigger_key) {
            let run = repos::automation::queue_run(
                &state.pool,
                rule.id,
                workspace_id,
                triggering_event_id,
                provider_kind.as_deref(),
                model.as_deref(),
                actor_id,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

            if let Some(failed) = repos::automation::mark_run_failed(
                &state.pool,
                run.id,
                actor_id,
                "RATE_LIMITED",
                "automation trigger rate limited",
                None,
            )
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
            {
                let _ = repos::audit::emit_security_event(
                    &state.pool,
                    request_id,
                    Some(actor_id),
                    Some(workspace_id),
                    "automation_run_failed",
                    json!({
                        "run_id": failed.id,
                        "rule_id": failed.rule_id,
                        "trigger": trigger,
                        "triggering_event_id": triggering_event_id,
                        "error_code": "RATE_LIMITED",
                    }),
                )
                .await;
            }
            continue;
        }

        let run = repos::automation::queue_run(
            &state.pool,
            rule.id,
            workspace_id,
            triggering_event_id,
            provider_kind.as_deref(),
            model.as_deref(),
            actor_id,
        )
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?;

        if run.status != "queued" {
            continue;
        }

        let _ = repos::audit::emit_security_event(
            &state.pool,
            request_id,
            Some(actor_id),
            Some(workspace_id),
            "automation_run_queued",
            json!({
                "run_id": run.id,
                "rule_id": run.rule_id,
                "trigger": trigger,
                "triggering_event_id": triggering_event_id,
            }),
        )
        .await;

        let Some(running) = repos::automation::mark_run_running(&state.pool, run.id, actor_id)
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        else {
            continue;
        };

        let _ = repos::audit::emit_security_event(
            &state.pool,
            request_id,
            Some(actor_id),
            Some(workspace_id),
            "automation_run_running",
            json!({
                "run_id": running.id,
                "rule_id": running.rule_id,
                "trigger": trigger,
                "triggering_event_id": triggering_event_id,
            }),
        )
        .await;

        match execute_rule_action(state, &rule, event_payload).await {
            Ok(result_json) => {
                if let Some(succeeded) = repos::automation::mark_run_succeeded(
                    &state.pool,
                    running.id,
                    actor_id,
                    result_json,
                )
                .await
                .map_err(|_| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error")
                })?
                {
                    let _ = repos::audit::emit_security_event(
                        &state.pool,
                        request_id,
                        Some(actor_id),
                        Some(workspace_id),
                        "automation_run_succeeded",
                        json!({
                            "run_id": succeeded.id,
                            "rule_id": succeeded.rule_id,
                            "trigger": trigger,
                            "triggering_event_id": triggering_event_id,
                        }),
                    )
                    .await;
                }
            }
            Err(error) => {
                if let Some(failed) = repos::automation::mark_run_failed(
                    &state.pool,
                    running.id,
                    actor_id,
                    &error.code,
                    &error.detail,
                    error.result_json,
                )
                .await
                .map_err(|_| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error")
                })?
                {
                    let _ = repos::audit::emit_security_event(
                        &state.pool,
                        request_id,
                        Some(actor_id),
                        Some(workspace_id),
                        "automation_run_failed",
                        json!({
                            "run_id": failed.id,
                            "rule_id": failed.rule_id,
                            "trigger": trigger,
                            "triggering_event_id": triggering_event_id,
                            "error_code": error.code,
                        }),
                    )
                    .await;
                }
            }
        }
    }

    Ok(())
}

fn enforce_automation_rate_limit(
    state: &AppState,
    req: &HttpRequest,
    bucket: &str,
    request_id: &str,
) -> Result<(), ApiError> {
    let key = format!("{}:{}", client_key(req), bucket);
    if state.automation_rate_limiter.check(&key) {
        return Ok(());
    }

    Err(ApiError::new(
        StatusCode::TOO_MANY_REQUESTS,
        "RATE_LIMITED",
        "rate limited",
    )
    .with_request_id(request_id.to_owned()))
}

async fn execute_rule_action(
    state: &AppState,
    rule: &kjxlkj_db::models::DbAutomationRule,
    event_payload: &serde_json::Value,
) -> Result<serde_json::Value, ActionExecutionError> {
    let action_obj = rule
        .action_json
        .as_object()
        .ok_or_else(|| ActionExecutionError {
            code: "RULE_ACTION_INVALID".to_owned(),
            detail: "action_json must be object".to_owned(),
            result_json: None,
        })?;

    let kind = action_obj
        .get("kind")
        .and_then(|value| value.as_str())
        .ok_or_else(|| ActionExecutionError {
            code: "RULE_ACTION_INVALID".to_owned(),
            detail: "action kind is required".to_owned(),
            result_json: None,
        })?;

    if kind == "noop" {
        return Ok(json!({ "kind": "noop", "applied": false }));
    }

    if kind == "tag_note" {
        let tag = action_obj
            .get("tag")
            .and_then(|value| value.as_str())
            .ok_or_else(|| ActionExecutionError {
                code: "RULE_ACTION_INVALID".to_owned(),
                detail: "tag is required".to_owned(),
                result_json: None,
            })?;

        let note_id = event_payload
            .get("note_id")
            .and_then(|value| value.as_str())
            .and_then(|value| Uuid::parse_str(value).ok())
            .ok_or_else(|| ActionExecutionError {
                code: "RULE_ACTION_INVALID".to_owned(),
                detail: "note_id missing from triggering event payload".to_owned(),
                result_json: None,
            })?;

        repos::notes::upsert_metadata(
            &state.pool,
            note_id,
            "automation.last-action",
            json!({ "kind": "tag_note", "tag": tag }),
        )
        .await
        .map_err(|_| ActionExecutionError {
            code: "RULE_ACTION_FAILED".to_owned(),
            detail: "failed to write note metadata side-effect".to_owned(),
            result_json: None,
        })?;

        return Ok(json!({
            "kind": "tag_note",
            "applied": true,
            "note_id": note_id,
            "tag": tag,
        }));
    }

    if kind == "librarian_structure" {
        let action_config = parse_librarian_action_config(action_obj).map_err(|detail| {
            ActionExecutionError {
                code: "RULE_ACTION_INVALID".to_owned(),
                detail: format!("invalid librarian provider config: {detail}"),
                result_json: None,
            }
        })?;

        let prompt = build_librarian_prompt(rule, event_payload, &action_config.plan);
        let mut raw_outputs: Vec<String> = Vec::new();
        let mut parse_diagnostics: Vec<serde_json::Value> = Vec::new();

        let first_outcome = invoke_librarian_provider(&action_config.provider, &prompt)
            .await
            .map_err(|(code, detail)| ActionExecutionError {
                code,
                detail,
                result_json: None,
            })?;
        let mut total_attempts = first_outcome.attempts;
        raw_outputs.push(first_outcome.raw_response.clone());

        let parse_outcome = match parse_librarian_response(&first_outcome.raw_response, &action_config.plan) {
            Ok(value) => value,
            Err(first_failure) => {
                parse_diagnostics.push(json!({
                    "attempt": 1,
                    "code": first_failure.code,
                    "detail": first_failure.detail,
                    "errors": first_failure.errors,
                }));

                let mut resolved: Option<ParsedLibrarianResponse> = None;
                let mut last_failure = first_failure;

                for repair_attempt in 1..=2 {
                    let repair_prompt = build_librarian_repair_prompt(&last_failure.errors, &raw_outputs[0]);
                    let repaired = invoke_librarian_provider(&action_config.provider, &repair_prompt)
                        .await
                        .map_err(|(code, detail)| ActionExecutionError {
                            code,
                            detail,
                            result_json: Some(json!({
                                "kind": "librarian_structure",
                                "provider_kind": action_config.provider.provider_kind,
                                "model": action_config.provider.model,
                                "protocol": "xml_attrless",
                                "attempts": total_attempts,
                                "raw_model_outputs": raw_outputs,
                                "parse_diagnostics": parse_diagnostics,
                            })),
                        })?;

                    total_attempts = total_attempts.saturating_add(repaired.attempts);
                    raw_outputs.push(repaired.raw_response.clone());

                    match parse_librarian_response(&repaired.raw_response, &action_config.plan) {
                        Ok(value) => {
                            resolved = Some(value);
                            break;
                        }
                        Err(next_failure) => {
                            parse_diagnostics.push(json!({
                                "attempt": repair_attempt + 1,
                                "code": next_failure.code,
                                "detail": next_failure.detail,
                                "errors": next_failure.errors,
                            }));
                            last_failure = next_failure;
                        }
                    }
                }

                if let Some(value) = resolved {
                    value
                } else {
                    let final_code = last_failure.code;
                    let final_detail = last_failure.detail;
                    return Err(ActionExecutionError {
                        code: final_code,
                        detail: format!(
                            "librarian parse failed after repair retries (diagnostics={}): {}",
                            parse_diagnostics.len(),
                            final_detail
                        ),
                        result_json: Some(json!({
                            "kind": "librarian_structure",
                            "provider_kind": action_config.provider.provider_kind,
                            "model": action_config.provider.model,
                            "protocol": "xml_attrless",
                            "attempts": total_attempts,
                            "raw_model_output": raw_outputs.last().cloned().unwrap_or_default(),
                            "raw_model_outputs": raw_outputs,
                            "parse_diagnostics": parse_diagnostics,
                        })),
                    });
                }
            }
        };

        let operation_report = build_librarian_operation_report(
            &action_config.plan,
            rule.workspace_id,
            event_payload,
            &parse_outcome.operations,
            parse_outcome.warnings,
        );

        return Ok(json!({
            "kind": "librarian_structure",
            "applied": false,
            "provider_kind": action_config.provider.provider_kind,
            "model": action_config.provider.model,
            "protocol": "xml_attrless",
            "attempts": total_attempts,
            "raw_model_output": raw_outputs.last().cloned().unwrap_or_default(),
            "raw_model_outputs": raw_outputs,
            "request_id": parse_outcome.request_id,
            "response_status": parse_outcome.status,
            "response_summary": parse_outcome.summary,
            "parse_diagnostics": parse_diagnostics,
            "operation_report": operation_report,
        }));
    }

    Err(ActionExecutionError {
        code: "RULE_ACTION_INVALID".to_owned(),
        detail: "unsupported action kind".to_owned(),
        result_json: None,
    })
}

async fn actor_workspace_role(
    state: &AppState,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Role, ApiError> {
    let role_text = repos::workspaces::actor_workspace_role(&state.pool, workspace_id, user_id)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "internal error"))?
        .ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "ROLE_FORBIDDEN", "forbidden"))?;

    Role::from_str(&role_text)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "invalid role data"))
}

fn parse_librarian_provider_config(
    action_obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<LibrarianProviderConfig, String> {
    let provider_obj = action_obj
        .get("provider")
        .and_then(|value| value.as_object())
        .cloned()
        .unwrap_or_default();

    let provider_kind = provider_obj
        .get("provider_kind")
        .and_then(|value| value.as_str())
        .or_else(|| action_obj.get("provider_kind").and_then(|value| value.as_str()))
        .ok_or_else(|| "librarian provider_kind is required".to_owned())?
        .to_owned();

    if !matches!(provider_kind.as_str(), "openrouter" | "lmstudio") {
        return Err("invalid librarian provider_kind".to_owned());
    }

    let protocol = action_obj
        .get("protocol")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    if protocol != "xml_attrless" {
        return Err("invalid librarian protocol".to_owned());
    }

    let model = provider_obj
        .get("model")
        .and_then(|value| value.as_str())
        .or_else(|| action_obj.get("model").and_then(|value| value.as_str()))
        .ok_or_else(|| "librarian model is required".to_owned())?
        .trim()
        .to_owned();

    if model.is_empty() || model.len() > 128 {
        return Err("invalid librarian model".to_owned());
    }

    let default_base_url = if provider_kind == "openrouter" {
        "https://openrouter.ai/api/v1/chat/completions"
    } else {
        "http://127.0.0.1:1234/v1/chat/completions"
    };

    let base_url = provider_obj
        .get("base_url")
        .and_then(|value| value.as_str())
        .or_else(|| action_obj.get("base_url").and_then(|value| value.as_str()))
        .unwrap_or(default_base_url)
        .to_owned();

    let parsed_url = reqwest::Url::parse(&base_url)
        .map_err(|_| "invalid librarian base_url".to_owned())?;
    if !matches!(parsed_url.scheme(), "http" | "https") {
        return Err("invalid librarian base_url".to_owned());
    }

    let timeout_ms = provider_obj
        .get("timeout_ms")
        .and_then(|value| value.as_u64())
        .or_else(|| action_obj.get("timeout_ms").and_then(|value| value.as_u64()))
        .unwrap_or(2_000);
    if !(50..=120_000).contains(&timeout_ms) {
        return Err("invalid librarian timeout_ms".to_owned());
    }

    let retry_limit = provider_obj
        .get("retry_limit")
        .and_then(|value| value.as_u64())
        .or_else(|| action_obj.get("retry_limit").and_then(|value| value.as_u64()))
        .unwrap_or(1);
    if retry_limit > 2 {
        return Err("invalid librarian retry_limit".to_owned());
    }

    let max_tokens = provider_obj
        .get("max_tokens")
        .and_then(|value| value.as_u64())
        .or_else(|| action_obj.get("max_tokens").and_then(|value| value.as_u64()))
        .map(|value| value as u32);

    if max_tokens == Some(0) {
        return Err("invalid librarian max_tokens".to_owned());
    }

    let temperature = provider_obj
        .get("temperature")
        .and_then(|value| value.as_f64())
        .or_else(|| action_obj.get("temperature").and_then(|value| value.as_f64()))
        .map(|value| value as f32);

    if let Some(value) = temperature {
        if !(0.0..=2.0).contains(&value) {
            return Err("invalid librarian temperature".to_owned());
        }
    }

    let api_key = provider_obj
        .get("api_key")
        .and_then(|value| value.as_str())
        .or_else(|| action_obj.get("api_key").and_then(|value| value.as_str()))
        .map(ToOwned::to_owned);

    Ok(LibrarianProviderConfig {
        provider_kind,
        model,
        base_url,
        timeout_ms,
        retry_limit: retry_limit as u8,
        max_tokens,
        temperature,
        api_key,
    })
}

fn parse_librarian_plan_config(
    action_obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<LibrarianPlanConfig, String> {
    let plan_obj = action_obj
        .get("plan")
        .and_then(|value| value.as_object())
        .ok_or_else(|| "librarian plan is required".to_owned())?;

    let goal = plan_obj
        .get("goal")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "librarian plan goal is required".to_owned())?
        .to_owned();

    if goal.len() > 256 {
        return Err("invalid librarian plan goal".to_owned());
    }

    let scope = plan_obj
        .get("scope")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "librarian plan scope is required".to_owned())?
        .to_owned();

    if scope.len() > 128 {
        return Err("invalid librarian plan scope".to_owned());
    }

    let taxonomy_json = plan_obj
        .get("taxonomy_json")
        .cloned()
        .ok_or_else(|| "librarian plan taxonomy_json is required".to_owned())?;

    if !taxonomy_json.is_object() && !taxonomy_json.is_array() {
        return Err("invalid librarian plan taxonomy_json".to_owned());
    }

    let style_profile = plan_obj
        .get("style_profile")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "librarian plan style_profile is required".to_owned())?
        .to_owned();

    if style_profile.len() > 64 {
        return Err("invalid librarian plan style_profile".to_owned());
    }

    let strict_mode = plan_obj
        .get("strict_mode")
        .and_then(|value| value.as_bool())
        .ok_or_else(|| "librarian plan strict_mode is required".to_owned())?;

    let max_operations = plan_obj
        .get("max_operations")
        .and_then(|value| value.as_u64())
        .ok_or_else(|| "librarian plan max_operations is required".to_owned())?;

    if !(1..=128).contains(&max_operations) {
        return Err("invalid librarian plan max_operations".to_owned());
    }

    let allow_delete = plan_obj
        .get("allow_delete")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    Ok(LibrarianPlanConfig {
        goal,
        scope,
        taxonomy_json,
        style_profile,
        strict_mode,
        max_operations: max_operations as usize,
        allow_delete,
    })
}

fn parse_librarian_action_config(
    action_obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<LibrarianActionConfig, String> {
    let provider = parse_librarian_provider_config(action_obj)?;
    let plan = parse_librarian_plan_config(action_obj)?;
    Ok(LibrarianActionConfig { provider, plan })
}

fn extract_rule_provider_metadata(
    rule: &kjxlkj_db::models::DbAutomationRule,
) -> (Option<String>, Option<String>) {
    let Some(action_obj) = rule.action_json.as_object() else {
        return (None, None);
    };

    if action_obj
        .get("kind")
        .and_then(|value| value.as_str())
        != Some("librarian_structure")
    {
        return (None, None);
    }

    match parse_librarian_action_config(action_obj) {
        Ok(config) => (Some(config.provider.provider_kind), Some(config.provider.model)),
        Err(_) => (None, None),
    }
}

fn build_librarian_prompt(
    rule: &kjxlkj_db::models::DbAutomationRule,
    event_payload: &serde_json::Value,
    plan: &LibrarianPlanConfig,
) -> String {
    json!({
        "protocol": "xml_attrless",
        "rule_id": rule.id,
        "workspace_id": rule.workspace_id,
        "goal": plan.goal.clone(),
        "scope": plan.scope.clone(),
        "taxonomy": plan.taxonomy_json.clone(),
        "style_profile": plan.style_profile.clone(),
        "constraints": {
            "strict_mode": plan.strict_mode,
            "max_operations": plan.max_operations,
            "allow_delete": plan.allow_delete,
        },
        "event_payload": event_payload,
    })
    .to_string()
}

async fn invoke_librarian_provider(
    config: &LibrarianProviderConfig,
    prompt: &str,
) -> Result<ProviderCallOutcome, (String, String)> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|_| {
            (
                "LLM_PROVIDER_UNREACHABLE".to_owned(),
                "failed to initialize provider http client".to_owned(),
            )
        })?;

    let max_attempts = config.retry_limit.saturating_add(1);
    let mut attempt: u8 = 0;

    loop {
        attempt = attempt.saturating_add(1);
        match call_librarian_provider_once(&client, config, prompt).await {
            Ok(raw_response) => {
                return Ok(ProviderCallOutcome {
                    raw_response,
                    attempts: attempt,
                });
            }
            Err(error) => {
                if error.retryable && attempt < max_attempts {
                    continue;
                }

                return Err((
                    error.code.to_owned(),
                    format!(
                        "provider_kind={} model={} attempt={}/{} detail={}",
                        config.provider_kind, config.model, attempt, max_attempts, error.detail
                    ),
                ));
            }
        }
    }
}

async fn call_librarian_provider_once(
    client: &reqwest::Client,
    config: &LibrarianProviderConfig,
    prompt: &str,
) -> Result<String, ProviderFailure> {
    let mut payload = json!({
        "model": config.model,
        "messages": [
            {
                "role": "system",
                "content": "You are the kjxlkj librarian. Respond in xml_attrless format only."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    if let Some(max_tokens) = config.max_tokens {
        payload["max_tokens"] = json!(max_tokens);
    }

    if let Some(temperature) = config.temperature {
        payload["temperature"] = json!(temperature);
    }

    let mut request = client
        .post(&config.base_url)
        .timeout(Duration::from_millis(config.timeout_ms))
        .json(&payload);

    if config.provider_kind == "openrouter" {
        if let Some(api_key) = config
            .api_key
            .clone()
            .or_else(|| std::env::var("OPENROUTER_API_KEY").ok())
        {
            request = request.bearer_auth(api_key);
        }
    }

    let response = request.send().await.map_err(classify_provider_send_error)?;
    let status = response.status();

    if !status.is_success() {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".to_owned());
        let body_snippet: String = body.chars().take(256).collect();
        let retryable = status.as_u16() == 429 || status.is_server_error();
        return Err(ProviderFailure {
            code: "LLM_UPSTREAM_ERROR",
            detail: format!("status={} body={}", status.as_u16(), body_snippet),
            retryable,
        });
    }

    let parsed: serde_json::Value = response.json().await.map_err(|_| ProviderFailure {
        code: "LLM_UPSTREAM_ERROR",
        detail: "invalid upstream response body".to_owned(),
        retryable: false,
    })?;

    let Some(content) = extract_llm_text_content(&parsed) else {
        return Err(ProviderFailure {
            code: "LLM_UPSTREAM_ERROR",
            detail: "missing chat completion content".to_owned(),
            retryable: false,
        });
    };

    Ok(content)
}

fn classify_provider_send_error(error: reqwest::Error) -> ProviderFailure {
    if error.is_timeout() {
        return ProviderFailure {
            code: "LLM_PROVIDER_TIMEOUT",
            detail: error.to_string(),
            retryable: true,
        };
    }

    if error.is_connect() || error.is_request() {
        return ProviderFailure {
            code: "LLM_PROVIDER_UNREACHABLE",
            detail: error.to_string(),
            retryable: true,
        };
    }

    ProviderFailure {
        code: "LLM_UPSTREAM_ERROR",
        detail: error.to_string(),
        retryable: false,
    }
}

fn extract_llm_text_content(value: &serde_json::Value) -> Option<String> {
    let first_choice = value
        .get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|choices| choices.first())?;

    let direct = first_choice
        .get("message")
        .and_then(|message| message.get("content"));

    if let Some(content) = direct.and_then(|content| content.as_str()) {
        return Some(content.to_owned());
    }

    let content_parts = direct
        .and_then(|content| content.as_array())
        .cloned()
        .unwrap_or_default();

    if content_parts.is_empty() {
        return None;
    }

    let joined = content_parts
        .into_iter()
        .filter_map(|part| {
            part.get("text")
                .and_then(|text| text.as_str())
                .map(ToOwned::to_owned)
        })
        .collect::<Vec<_>>()
        .join("\n");

    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

fn build_librarian_operation_report(
    plan: &LibrarianPlanConfig,
    workspace_id: Uuid,
    event_payload: &serde_json::Value,
    parsed_operations: &[LibrarianOperationCandidate],
    mut warnings: Vec<String>,
) -> serde_json::Value {
    let mut parsed_json: Vec<serde_json::Value> = Vec::new();
    let mut rejected_json: Vec<serde_json::Value> = Vec::new();

    if parsed_operations.is_empty() {
        warnings.push("NO_OPERATIONS_PARSED".to_owned());
    }

    let scope_note_id = plan
        .scope
        .strip_prefix("note:")
        .map(|value| value.trim().to_owned());

    let event_note_id = event_payload
        .get("note_id")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned);

    let allowed_kinds: [&str; 6] = [
        "create_note",
        "rewrite_note",
        "retitle_note",
        "relink_note",
        "retag_note",
        "defer",
    ];

    for (index, operation) in parsed_operations.iter().enumerate() {
        parsed_json.push(json!({
            "operation_id": operation.operation_id,
            "kind": operation.kind,
            "target_note_id": operation.target_note_id,
            "target_path": operation.target_path,
            "title": operation.title,
            "body_markdown": operation.body_markdown,
            "reason": operation.reason,
            "confidence": operation.confidence,
        }));

        if index >= plan.max_operations {
            rejected_json.push(json!({
                "operation_id": operation.operation_id,
                "kind": operation.kind,
                "reason": "MAX_OPERATIONS_EXCEEDED",
            }));
            continue;
        }

        if !allowed_kinds.contains(&operation.kind.as_str()) {
            rejected_json.push(json!({
                "operation_id": operation.operation_id,
                "kind": operation.kind,
                "reason": "INVALID_KIND",
            }));
            continue;
        }

        if plan.strict_mode && !matches!(operation.kind.as_str(), "create_note" | "rewrite_note") {
            rejected_json.push(json!({
                "operation_id": operation.operation_id,
                "kind": operation.kind,
                "reason": "STRICT_MODE_KIND_BLOCKED",
            }));
            continue;
        }

        if !plan.allow_delete && operation.kind.contains("delete") {
            rejected_json.push(json!({
                "operation_id": operation.operation_id,
                "kind": operation.kind,
                "reason": "DELETE_FORBIDDEN",
            }));
            continue;
        }

        let confidence = operation.confidence.unwrap_or(-1.0);
        if !(0.0..=1.0).contains(&confidence) {
            rejected_json.push(json!({
                "operation_id": operation.operation_id,
                "kind": operation.kind,
                "reason": "INVALID_CONFIDENCE",
            }));
            continue;
        }

        if plan.scope == "workspace" {
            continue;
        }

        if let Some(scope_workspace_id) = plan.scope.strip_prefix("workspace:") {
            if scope_workspace_id.trim() != workspace_id.to_string() {
                rejected_json.push(json!({
                    "operation_id": operation.operation_id,
                    "kind": operation.kind,
                    "reason": "SCOPE_VIOLATION",
                }));
            }
            continue;
        }

        if let Some(scope_note) = scope_note_id.clone() {
            let target_note = operation
                .target_note_id
                .clone()
                .or_else(|| event_note_id.clone())
                .unwrap_or_default();

            if target_note != scope_note {
                rejected_json.push(json!({
                    "operation_id": operation.operation_id,
                    "kind": operation.kind,
                    "reason": "SCOPE_VIOLATION",
                }));
            }
        }
    }

    json!({
        "parsed_operations": parsed_json,
        "applied_operations": [],
        "rejected_operations": rejected_json,
        "warnings": warnings,
    })
}

fn build_librarian_repair_prompt(errors: &[String], original_response: &str) -> String {
    json!({
        "mode": "repair",
        "protocol": "xml_attrless",
        "validation_errors": errors,
        "original_response": original_response,
    })
    .to_string()
}

fn parse_librarian_response(
    raw_output: &str,
    _plan: &LibrarianPlanConfig,
) -> Result<ParsedLibrarianResponse, LibrarianParseFailure> {
    let root = parse_xml_attrless_document(raw_output).map_err(|detail| LibrarianParseFailure {
        code: "LIBRARIAN_PROTOCOL_INVALID".to_owned(),
        detail,
        errors: vec!["MALFORMED_NESTING_OR_TAG".to_owned()],
    })?;

    if root.name != "librarian_response" {
        return Err(LibrarianParseFailure {
            code: "LIBRARIAN_PARSE_FAILED".to_owned(),
            detail: "missing root tag <librarian_response>".to_owned(),
            errors: vec!["MISSING_ROOT_LIBRARIAN_RESPONSE".to_owned()],
        });
    }

    let mut errors: Vec<String> = Vec::new();

    let request_id = extract_required_text(&root, "request_id", &mut errors);
    let status = extract_required_text(&root, "status", &mut errors);
    let summary = extract_required_text(&root, "summary", &mut errors);
    let operations_node = extract_required_child(&root, "operations", &mut errors);
    let warnings_node = extract_required_child(&root, "warnings", &mut errors);

    let status_value = status.unwrap_or_default();
    if !status_value.is_empty()
        && !matches!(status_value.as_str(), "ok" | "needs_clarification" | "rejected")
    {
        errors.push(format!("INVALID_STATUS:{status_value}"));
    }

    let operations = operations_node
        .map(|node| parse_librarian_operations(node, &mut errors))
        .unwrap_or_default();

    let warnings = warnings_node
        .map(parse_librarian_warnings)
        .unwrap_or_default();

    if !errors.is_empty() {
        return Err(LibrarianParseFailure {
            code: "LIBRARIAN_PARSE_FAILED".to_owned(),
            detail: format!("librarian response validation failed ({})", errors.len()),
            errors,
        });
    }

    Ok(ParsedLibrarianResponse {
        request_id: request_id.unwrap_or_default(),
        status: status_value,
        summary: summary.unwrap_or_default(),
        operations,
        warnings,
    })
}

fn parse_librarian_operations(
    operations_node: &XmlElementNode,
    errors: &mut Vec<String>,
) -> Vec<LibrarianOperationCandidate> {
    let mut operations: Vec<LibrarianOperationCandidate> = Vec::new();

    for (index, operation_node) in operations_node
        .children
        .iter()
        .filter(|child| child.name == "operation")
        .enumerate()
    {
        let prefix = format!("operation[{index}]");

        let operation_id = extract_required_text(operation_node, "operation_id", errors)
            .unwrap_or_else(|| format!("op_{}", index + 1));
        let kind = extract_required_text(operation_node, "kind", errors).unwrap_or_default();

        if !kind.is_empty()
            && !matches!(
                kind.as_str(),
                "create_note"
                    | "rewrite_note"
                    | "retitle_note"
                    | "relink_note"
                    | "retag_note"
                    | "defer"
            )
        {
            errors.push(format!("{prefix}:INVALID_KIND:{kind}"));
        }

        let target_note_id = extract_optional_text(operation_node, "target_note_id");
        let target_path = extract_optional_text(operation_node, "target_path");
        if target_note_id.is_none() && target_path.is_none() {
            errors.push(format!("{prefix}:MISSING_TARGET"));
        }

        let title = extract_required_text(operation_node, "title", errors);
        let body_markdown = extract_required_text(operation_node, "body_markdown", errors);
        let reason = extract_required_text(operation_node, "reason", errors);
        let confidence_raw = extract_required_text(operation_node, "confidence", errors);

        let confidence = confidence_raw
            .as_deref()
            .and_then(|value| value.parse::<f32>().ok());
        if let Some(value) = confidence {
            if !(0.0..=1.0).contains(&value) {
                errors.push(format!("{prefix}:INVALID_CONFIDENCE_RANGE:{value}"));
            }
        } else {
            errors.push(format!("{prefix}:INVALID_CONFIDENCE_FORMAT"));
        }

        operations.push(LibrarianOperationCandidate {
            operation_id,
            kind,
            target_note_id,
            target_path,
            title,
            body_markdown,
            reason,
            confidence,
        });
    }

    operations
}

fn parse_librarian_warnings(warnings_node: &XmlElementNode) -> Vec<String> {
    let warning_children = warnings_node
        .children
        .iter()
        .filter(|child| child.name == "warning")
        .map(element_text)
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    if !warning_children.is_empty() {
        return warning_children;
    }

    let root_warning = element_text(warnings_node);
    if root_warning.is_empty() {
        Vec::new()
    } else {
        vec![root_warning]
    }
}

fn parse_xml_attrless_document(input: &str) -> Result<XmlElementNode, String> {
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
    let bytes = normalized.as_bytes();
    let mut cursor = 0usize;
    let mut stack: Vec<XmlElementNode> = Vec::new();
    let mut root: Option<XmlElementNode> = None;

    while cursor < bytes.len() {
        if bytes[cursor] == b'<' {
            let mut end = cursor + 1;
            while end < bytes.len() && bytes[end] != b'>' {
                end += 1;
            }

            if end >= bytes.len() {
                return Err("unterminated tag".to_owned());
            }

            let raw_tag = normalized[cursor + 1..end].trim();
            if raw_tag.is_empty() {
                return Err("empty tag".to_owned());
            }

            if raw_tag.contains(char::is_whitespace) {
                return Err(format!("attributes not allowed in tag <{raw_tag}>"));
            }

            if raw_tag.ends_with('/') {
                return Err("self-closing tags are not supported".to_owned());
            }

            if let Some(close_name) = raw_tag.strip_prefix('/') {
                if !is_valid_xml_attrless_tag(close_name) {
                    return Err(format!("invalid closing tag </{close_name}>"));
                }

                let Some(current) = stack.pop() else {
                    return Err(format!("unexpected closing tag </{close_name}>"));
                };

                if current.name != close_name {
                    return Err(format!(
                        "malformed nesting expected </{}> but found </{}>",
                        current.name, close_name
                    ));
                }

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(current);
                } else if root.is_none() {
                    root = Some(current);
                } else {
                    return Err("multiple root elements".to_owned());
                }
            } else {
                if !is_valid_xml_attrless_tag(raw_tag) {
                    return Err(format!("invalid opening tag <{raw_tag}>"));
                }

                stack.push(XmlElementNode {
                    name: raw_tag.to_owned(),
                    text: String::new(),
                    children: Vec::new(),
                });
            }

            cursor = end + 1;
            continue;
        }

        let mut next_tag = cursor;
        while next_tag < bytes.len() && bytes[next_tag] != b'<' {
            next_tag += 1;
        }

        let text_segment = &normalized[cursor..next_tag];
        if let Some(current) = stack.last_mut() {
            current.text.push_str(text_segment);
        } else if !text_segment.trim().is_empty() {
            return Err("text outside root element".to_owned());
        }

        cursor = next_tag;
    }

    if !stack.is_empty() {
        return Err("unclosed tags in response".to_owned());
    }

    root.ok_or_else(|| "missing root element".to_owned())
}

fn is_valid_xml_attrless_tag(tag: &str) -> bool {
    if tag.is_empty() {
        return false;
    }

    tag.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | ':')
    })
}

fn extract_required_child<'a>(
    parent: &'a XmlElementNode,
    tag: &str,
    errors: &mut Vec<String>,
) -> Option<&'a XmlElementNode> {
    let mut children = parent.children.iter().filter(|child| child.name == tag);
    let first = children.next();

    if first.is_none() {
        errors.push(format!("MISSING_TAG:{tag}"));
    }

    if children.next().is_some() {
        errors.push(format!("MULTIPLE_TAGS:{tag}"));
    }

    first
}

fn extract_required_text(
    parent: &XmlElementNode,
    tag: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let Some(node) = extract_required_child(parent, tag, errors) else {
        return None;
    };

    let text = element_text(node);
    if text.is_empty() {
        errors.push(format!("EMPTY_TAG:{tag}"));
        return None;
    }

    Some(text)
}

fn extract_optional_text(parent: &XmlElementNode, tag: &str) -> Option<String> {
    let text = parent
        .children
        .iter()
        .find(|child| child.name == tag)
        .map(element_text)?;

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn element_text(node: &XmlElementNode) -> String {
    node.text.trim().to_owned()
}
