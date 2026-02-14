use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    auth::{now_iso, request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{AutomationRunRecord, Role, Store},
    state::{AppState, WsEnvelope},
};

#[derive(Deserialize)]
pub struct ReviewRunRequest {
    pub decisions: Value,
}

pub async fn rule_launch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(rule_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let rule = store
        .rules
        .get(&rule_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("RULE_NOT_FOUND", "rule not found", rid.clone()))?;
    let run = AutomationRunRecord {
        id: Store::next_id(),
        rule_id: rule.id,
        status: "completed".to_string(),
        started_at: now_iso(),
        finished_at: Some(now_iso()),
        result_json: json!({"provider_report":"deterministic-local"}),
    };
    store.runs.insert(run.id.clone(), run.clone());

    let ws_payload = json!({"type":"automation_event","workspace_id":rule.workspace_id,"run_id":run.id,"status":run.status,"event_seq":1,"event_type":"run_completed","payload":run.result_json});
    let _ = state.ws_tx.send(WsEnvelope {
        stream_id: format!("workspace:{}", rule.workspace_id),
        payload: ws_payload,
    });

    Ok((StatusCode::OK, Json(json!({ "item": run, "request_id": rid }))))
}

pub async fn runs_list(State(state): State<AppState>, headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let items: Vec<AutomationRunRecord> = store.runs.values().cloned().collect();
    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn run_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let run = store
        .runs
        .get(&run_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("RUN_NOT_FOUND", "run not found", rid.clone()))?;
    Ok((StatusCode::OK, Json(json!({ "item": run, "request_id": rid }))))
}

pub async fn run_review(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
    Json(payload): Json<ReviewRunRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let run = store
        .runs
        .get_mut(&run_id)
        .ok_or_else(|| ApiError::not_found("RUN_NOT_FOUND", "run not found", rid.clone()))?;
    run.result_json = json!({"review": payload.decisions});
    Ok((StatusCode::OK, Json(json!({ "item": run, "request_id": rid }))))
}
