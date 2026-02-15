//! Automation rule and run handlers per /docs/spec/api/http.md.

use crate::dto::*;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// GET /api/automation/rules
pub async fn list_rules(
    req: HttpRequest, pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "workspace_id required")),
    };
    match kjxlkj_db::repo::automation::list_rules(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|r| {
                serde_json::json!({
                    "id": r.id, "trigger": r.trigger, "enabled": r.enabled,
                    "condition_json": r.condition_json, "action_json": r.action_json
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/automation/rules
pub async fn create_rule(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    body: web::Json<CreateRuleReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let role = kjxlkj_rbac::parse_role(&ctx.role).unwrap_or(kjxlkj_domain::types::Role::Viewer);
    if !kjxlkj_rbac::can_manage_automation(role) {
        return middleware::forbidden();
    }
    // Validate provider for librarian rules per /docs/spec/api/http.md contract
    if let Some(kind) = body.action_json.get("kind").and_then(|v| v.as_str()) {
        if kind == "librarian_structure" {
            if let Some(prov) = body.action_json.get("provider") {
                if let Some(pk) = prov.get("provider_kind").and_then(|v| v.as_str()) {
                    if pk != "openrouter" && pk != "lmstudio" {
                        return HttpResponse::UnprocessableEntity()
                            .json(ApiError::new("RULE_INVALID",
                                format!("unknown provider kind: {pk}")));
                    }
                }
            }
        }
    }
    let rid = kjxlkj_domain::types::new_id();
    let cj = body.condition_json.clone().unwrap_or(serde_json::json!({}));
    let enabled = body.enabled.unwrap_or(true);
    match kjxlkj_db::repo::automation::create_rule(
        pool.get_ref(), rid, body.workspace_id, &body.trigger, &cj,
        &body.action_json, enabled,
    ).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"id": rid})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// PATCH /api/automation/rules/{id}
pub async fn update_rule(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<UpdateRuleReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let rid = path.into_inner();
    let trigger = body.trigger.as_deref().unwrap_or("");
    let cj = body.condition_json.clone().unwrap_or(serde_json::json!({}));
    let aj = body.action_json.clone().unwrap_or(serde_json::json!({}));
    let enabled = body.enabled.unwrap_or(true);
    match kjxlkj_db::repo::automation::update_rule(
        pool.get_ref(), rid, trigger, &cj, &aj, enabled,
    ).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"id": rid})),
        Ok(false) => middleware::not_found("rule"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// DELETE /api/automation/rules/{id}
pub async fn delete_rule(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let rid = path.into_inner();
    match kjxlkj_db::repo::automation::delete_rule(pool.get_ref(), rid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("rule"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/automation/rules/{id}/launch — manually launch run.
pub async fn launch(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let rid = path.into_inner();
    let run_id = kjxlkj_domain::types::new_id();
    match kjxlkj_db::repo::automation::create_run(pool.get_ref(), run_id, rid).await {
        Ok(()) => HttpResponse::Created()
            .json(serde_json::json!({"run_id": run_id, "status": "queued"})),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/automation/runs
pub async fn list_runs(
    req: HttpRequest, pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let ws_id = match query.get("workspace_id").and_then(|s| s.parse::<Uuid>().ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest()
            .json(ApiError::new("BAD_REQUEST", "workspace_id required")),
    };
    match kjxlkj_db::repo::automation::list_runs(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let out: Vec<serde_json::Value> = rows.iter().map(|r| {
                serde_json::json!({
                    "id": r.id, "rule_id": r.rule_id, "status": r.status,
                    "started_at": r.started_at
                })
            }).collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// GET /api/automation/runs/{id}
pub async fn get_run(
    req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    let rid = path.into_inner();
    match kjxlkj_db::repo::automation::get_run(pool.get_ref(), rid).await {
        Ok(Some(r)) => HttpResponse::Ok().json(serde_json::json!({
            "id": r.id, "rule_id": r.rule_id, "status": r.status,
            "started_at": r.started_at, "finished_at": r.finished_at,
            "result_json": r.result_json
        })),
        Ok(None) => middleware::not_found("run"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}

/// POST /api/automation/runs/{id}/review
pub async fn review(
    req: HttpRequest, pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>, body: web::Json<ReviewReq>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c, Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) { return r; }
    let rid = path.into_inner();
    // Record review decisions
    match kjxlkj_db::repo::automation::update_run_result(
        pool.get_ref(), rid, &body.decisions,
    ).await {
        Ok(true) => HttpResponse::Ok()
            .json(serde_json::json!({"run_id": rid, "status": "reviewed"})),
        Ok(false) => middleware::not_found("run"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
