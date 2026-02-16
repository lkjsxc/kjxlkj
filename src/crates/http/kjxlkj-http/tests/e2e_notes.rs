/// E2E note acceptance tests per /docs/spec/technical/testing.md
///
/// Covers acceptance IDs:
/// - E2E-06: markdown editor autosave path via API
/// - E2E-17: draft integrity under conflicts
/// - E2E-23: create-new-note creates and selects immediately
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;

/// E2E-06: Markdown editor autosave confidence path.
#[tokio::test]
async fn e2e_06_autosave_path() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws, "title": "autosave-test"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    let note_id = body["id"].as_str().unwrap().to_string();
    for (i, version) in [(1, "draft 1"), (2, "draft 2"), (3, "draft 3")] {
        let app = build_app_with_state(state.clone());
        let resp = app
            .oneshot(
                Request::builder().method("PATCH")
                    .uri(format!("/api/notes/{}", note_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({"base_version": i, "markdown": version}).to_string(),
                    )).unwrap(),
            ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(Request::get(&format!("/api/notes/{}", note_id)).body(Body::empty()).unwrap())
        .await.unwrap();
    let body = json_body(resp).await;
    assert_eq!(body["markdown"], "draft 3");
    assert_eq!(body["version"], 4);
}

/// E2E-17: Draft integrity under version conflict.
#[tokio::test]
async fn e2e_17_draft_integrity_conflict() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws, "title": "conflict-test"}).to_string(),
                )).unwrap(),
        ).await.unwrap();
    let body = json_body(resp).await;
    let note_id = body["id"].as_str().unwrap().to_string();
    // First patch succeeds
    let app = build_app_with_state(state.clone());
    let resp1 = app
        .oneshot(
            Request::builder().method("PATCH")
                .uri(format!("/api/notes/{}", note_id))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"base_version": 1, "markdown": "user A"}).to_string(),
                )).unwrap(),
        ).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);
    // Second patch at stale version → 409
    let app = build_app_with_state(state.clone());
    let resp2 = app
        .oneshot(
            Request::builder().method("PATCH")
                .uri(format!("/api/notes/{}", note_id))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"base_version": 1, "markdown": "user B"}).to_string(),
                )).unwrap(),
        ).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}

/// E2E-23: Create-new-note creates and returns note immediately.
#[tokio::test]
async fn e2e_23_create_note_immediate() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws, "title": "Quick Note"}).to_string(),
                )).unwrap(),
        ).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    assert!(body["id"].is_string());
    assert_eq!(body["title"], "Quick Note");
    assert!(body["current_version"].is_number());
    let note_id = body["id"].as_str().unwrap();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(Request::get(&format!("/api/notes/{}", note_id)).body(Body::empty()).unwrap())
        .await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
