/// T1 integration tests for HTTP API.
///
/// Each test creates AppState + Router and sends real HTTP requests.
/// Spec: /docs/spec/technical/testing.md (Tier T1)
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use kjxlkj_http::{routes::api_router, state::AppState};
use tower::ServiceExt;

fn build_app() -> Router {
    api_router().with_state(AppState::new())
}

async fn json_body(resp: axum::response::Response) -> serde_json::Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
}

// ─── Health ─────────────────────────────────────────────────────────
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

// ─── Auth: setup/register ───────────────────────────────────────────
#[tokio::test]
async fn api_auth_01_first_user_becomes_owner() {
    let app = build_app();
    let resp = app
        .oneshot(
            Request::post("/api/setup/register")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"alice","password":"Str0ngPass!"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    assert_eq!(body["username"], "alice");
    assert_eq!(body["message"], "owner created");
}

#[tokio::test]
async fn api_auth_02_setup_locked_after_first_owner() {
    let state = AppState::new();
    let app = api_router().with_state(state.clone());
    // First register succeeds
    let resp = app
        .oneshot(
            Request::post("/api/setup/register")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"alice","password":"Str0ngPass!"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    // Second register must be rejected
    let app2 = api_router().with_state(state);
    let resp2 = app2
        .oneshot(
            Request::post("/api/setup/register")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"bob","password":"Str0ngPass!"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(resp2.status(), StatusCode::CREATED);
}

// ─── Notes: API-NOTE-01, API-NOTE-02 ───────────────────────────────
#[tokio::test]
async fn api_note_01_default_datetime_title() {
    let state = AppState::new();
    let app = api_router().with_state(state.clone());
    let ws_id = uuid::Uuid::new_v4();
    // Create workspace first
    let resp = app
        .oneshot(
            Request::post("/api/workspaces")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"name":"w","slug":"ws-test"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    // Create note without title
    let app2 = api_router().with_state(state);
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
    // Title must be a date-time string, not empty
    let title = body["title"].as_str().unwrap();
    assert!(!title.is_empty(), "default title must not be empty");
    assert!(
        title.contains('-'),
        "default datetime title should contain date separator"
    );
}

#[tokio::test]
async fn api_note_02_id_stable_while_title_changes() {
    let state = AppState::new();
    let app = api_router().with_state(state.clone());
    // Create note with title
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
    // Change title
    let app2 = api_router().with_state(state.clone());
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
    // Get note — id must be the same
    let app3 = api_router().with_state(state);
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

// ─── Search: API-SEARCH-01 ─────────────────────────────────────────
#[tokio::test]
async fn api_search_01_lexical_deterministic() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    // Create two notes
    let app = api_router().with_state(state.clone());
    app.oneshot(
        Request::post("/api/notes")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"workspace_id": ws, "title": "Rust", "markdown": "systems programming"}).to_string(),
            ))
            .unwrap(),
    ).await.unwrap();
    let app2 = api_router().with_state(state.clone());
    app2.oneshot(
        Request::post("/api/notes")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"workspace_id": ws, "title": "Python", "markdown": "scripting"}).to_string(),
            ))
            .unwrap(),
    ).await.unwrap();
    // Search for "rust"
    let app3 = api_router().with_state(state);
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
    let results = body.as_array().unwrap();
    assert!(!results.is_empty(), "lexical search must find matching note");
}

// ─── Search: API-SEARCH-02 (mode validation) ───────────────────────
#[tokio::test]
async fn api_search_02_unknown_mode_returns_422() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = api_router().with_state(state);
    let resp = app
        .oneshot(
            Request::get(&format!(
                "/api/search?q=test&mode=unknown_mode&workspace_id={}",
                ws
            ))
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ─── Version conflict: 409 ──────────────────────────────────────────
#[tokio::test]
async fn version_conflict_returns_409() {
    let state = AppState::new();
    let ws = uuid::Uuid::new_v4();
    let app = api_router().with_state(state.clone());
    // Create note
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
    // Patch with wrong base_version (0 != 1)
    let app2 = api_router().with_state(state);
    let resp2 = app2
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/notes/{}", note_id))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"base_version": 0, "markdown": "x"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}
