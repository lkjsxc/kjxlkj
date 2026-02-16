/// E2E acceptance tests: search, automation, export
/// per /docs/spec/technical/testing.md
///
/// Covers:
/// - API-SEARCH-03: embedding outage degrades gracefully
/// - API-AUTO-03: kjxlkj-agent rule validates prompt JSON
/// - Export lifecycle E2E
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;

/// API-SEARCH-03: Embedding outage degrades gracefully.
#[tokio::test]
async fn api_search_03_embedding_degraded() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    app.oneshot(
        Request::post("/api/notes")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "workspace_id": ws, "title": "Embedding test",
                    "markdown": "neural network content"
                }).to_string(),
            )).unwrap(),
    ).await.unwrap();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::get(&format!("/api/search?q=neural&mode=semantic&workspace_id={}", ws))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
    let status = resp.status();
    assert!(
        status == StatusCode::BAD_GATEWAY || status == StatusCode::OK,
        "semantic search without provider should degrade (got {})", status
    );
}

/// API-AUTO-03: kjxlkj-agent rule validates prompt JSON fields.
#[tokio::test]
async fn api_auto_03_agent_rule_validation() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    app.oneshot(
        Request::post("/api/workspaces")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"name":"TestWS","slug":"test-ws"}).to_string(),
            )).unwrap(),
    ).await.unwrap();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/automation/rules")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "workspace_id": ws, "name": "bad rule",
                        "trigger": "manual", "action_json": {}
                    }).to_string(),
                )).unwrap(),
        ).await.unwrap();
    assert_ne!(resp.status(), StatusCode::CREATED);
}

/// Export lifecycle E2E: create job, query status.
#[tokio::test]
async fn e2e_export_lifecycle() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/admin/export")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws, "kind": "markdown"}).to_string(),
                )).unwrap(),
        ).await.unwrap();
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
    let body = json_body(resp).await;
    let job_id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["status"], "queued");
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::get(&format!("/api/admin/export/{}", job_id))
                .body(Body::empty()).unwrap(),
        ).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = json_body(resp).await;
    assert_eq!(body["id"], job_id);
    assert_eq!(body["status"], "queued");
}
