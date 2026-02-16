/// T1 integration tests: Health, Notes, Search.
/// Spec: /docs/spec/technical/testing.md (Tier T1)
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;
#[tokio::test]
async fn healthz_returns_ok() {
    let app = build_app();
    let resp = app
        .oneshot(Request::get("/api/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = json_body(resp).await;
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn readyz_returns_ok() {
    let app = build_app();
    let resp = app
        .oneshot(Request::get("/api/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn api_note_01_default_datetime_title() {
    let state = AppState::new();
    let ws_id = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    app.oneshot(
        Request::post("/api/workspaces")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"name":"w","slug":"ws-test"}).to_string(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();
    let app2 = build_app_with_state(state.clone());
    let resp2 = app2
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws_id}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::CREATED);
    let body = json_body(resp2).await;
    let title = body["title"].as_str().unwrap();
    assert!(!title.is_empty());
    assert!(title.contains('-'));
}

#[tokio::test]
async fn api_note_02_id_stable_while_title_changes() {
    let state = AppState::new();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "workspace_id": uuid::Uuid::new_v4(),
                        "title": "Original Title"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    let note_id = body["id"].as_str().unwrap().to_string();
    let app2 = build_app_with_state(state.clone());
    let resp2 = app2
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/notes/{}/title", note_id))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"base_version": 1, "title": "New Title"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let app3 = build_app_with_state(state.clone());
    let resp3 = app3
        .oneshot(
            Request::get(&format!("/api/notes/{}", note_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body3 = json_body(resp3).await;
    assert_eq!(body3["note_id"].as_str().unwrap(), note_id);
    assert_eq!(body3["title"], "New Title");
}

#[tokio::test]
async fn api_search_01_lexical_deterministic() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    app.oneshot(
        Request::post("/api/notes")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"workspace_id": ws, "title": "Rust", "markdown": "systems"}).to_string(),
            ))
            .unwrap(),
    ).await.unwrap();
    let app2 = build_app_with_state(state.clone());
    app2.oneshot(
        Request::post("/api/notes")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"workspace_id": ws, "title": "Python", "markdown": "scripting"}).to_string(),
            ))
            .unwrap(),
    ).await.unwrap();
    let app3 = build_app_with_state(state.clone());
    let resp = app3
        .oneshot(
            Request::get(&format!("/api/search?q=rust&mode=lexical&workspace_id={}", ws))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = json_body(resp).await;
    let results = body["results"].as_array().unwrap();
    assert!(!results.is_empty());
}

#[tokio::test]
async fn api_search_02_unknown_mode_returns_422() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::get(&format!("/api/search?q=test&mode=unknown_mode&workspace_id={}", ws))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn version_conflict_returns_409() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"workspace_id": ws, "title": "t"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    let note_id = body["id"].as_str().unwrap();
    let app2 = build_app_with_state(state.clone());
    let resp2 = app2.oneshot(
        Request::builder()
            .method("PATCH")
            .uri(format!("/api/notes/{}", note_id))
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"base_version": 0, "markdown": "x"}).to_string(),
            ))
            .unwrap(),
    ).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}
