use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    auth::{request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{AutomationRuleRecord, Role, Store},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateRuleRequest {
    pub workspace_id: String,
    pub trigger: String,
    pub condition_json: Option<Value>,
    pub action_json: Value,
    pub enabled: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateRuleRequest {
    pub trigger: Option<String>,
    pub condition_json: Option<Value>,
    pub action_json: Option<Value>,
    pub enabled: Option<bool>,
}

pub async fn rules_list(State(state): State<AppState>, headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let items: Vec<AutomationRuleRecord> = store.rules.values().cloned().collect();
    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn rules_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateRuleRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;
    validate_librarian_contract(&payload.action_json, rid.clone())?;

    let item = AutomationRuleRecord {
        id: Store::next_id(),
        workspace_id: payload.workspace_id,
        trigger: payload.trigger,
        condition_json: payload.condition_json.unwrap_or_else(|| json!({})),
        action_json: payload.action_json,
        enabled: payload.enabled.unwrap_or(true),
    };

    let mut store = state.store.write().await;
    store.rules.insert(item.id.clone(), item.clone());
    Ok((StatusCode::CREATED, Json(json!({ "item": item, "request_id": rid }))))
}

pub async fn rules_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(rule_id): Path<String>,
    Json(payload): Json<UpdateRuleRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let rule = store
        .rules
        .get_mut(&rule_id)
        .ok_or_else(|| ApiError::not_found("RULE_NOT_FOUND", "rule not found", rid.clone()))?;

    if let Some(action_json) = payload.action_json {
        validate_librarian_contract(&action_json, rid.clone())?;
        rule.action_json = action_json;
    }
    if let Some(trigger) = payload.trigger {
        rule.trigger = trigger;
    }
    if let Some(condition_json) = payload.condition_json {
        rule.condition_json = condition_json;
    }
    if let Some(enabled) = payload.enabled {
        rule.enabled = enabled;
    }

    Ok((StatusCode::OK, Json(json!({ "item": rule, "request_id": rid }))))
}

pub async fn rules_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(rule_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let mut store = state.store.write().await;
    store.rules.remove(&rule_id);
    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}

fn validate_librarian_contract(action: &Value, rid: String) -> Result<(), ApiError> {
    if action.get("kind").and_then(Value::as_str) != Some("librarian_structure") {
        return Ok(());
    }

    let provider_kind = action
        .get("provider")
        .and_then(|provider| provider.get("provider_kind"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    let protocol = action
        .get("protocol")
        .and_then(Value::as_str)
        .unwrap_or_default();

    if !matches!(provider_kind, "openrouter" | "lmstudio") {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "unsupported librarian provider",
            rid,
        ));
    }
    if protocol != "xml_attrless" {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "RULE_INVALID",
            "librarian protocol must be xml_attrless",
            rid,
        ));
    }

    Ok(())
}
