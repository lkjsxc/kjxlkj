/// Snapshot tests for API contract stability per IMP-TEST-02.
///
/// Spec: /docs/spec/technical/testing.md
/// Spec: /docs/spec/api/types.md
///
/// These tests assert the exact JSON structure of API responses
/// to detect accidental contract breakage during reconstruction.
use kjxlkj_http::state::AppState;
use kjxlkj_http::routes::api_router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

/// Convenience: build the test app
fn app() -> axum::Router {
    api_router(AppState::new())
}

/// Convenience: send a request and return (status, json)
async fn send(
    router: axum::Router,
    req: Request<Body>,
) -> (StatusCode, Value) {
    let resp = router
        .oneshot(req)
        .await
        .unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: Value =
        serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

/// Snapshot: healthz response shape
#[tokio::test]
async fn snapshot_healthz() {
    let req = Request::get("/api/healthz")
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
    // Must have exactly this shape
    assert!(json.as_object().unwrap().contains_key("status"));
    assert_eq!(json.as_object().unwrap().len(), 1);
}

/// Snapshot: readyz response shape
#[tokio::test]
async fn snapshot_readyz() {
    let req = Request::get("/api/readyz")
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
}

/// Snapshot: create note → response has id, title, workspace_id, note_kind, current_version
#[tokio::test]
async fn snapshot_create_note_response() {
    let body = serde_json::json!({
        "workspace_id": "00000000-0000-0000-0000-000000000001",
        "title": "Snapshot Test Note",
        "note_kind": "markdown"
    });
    let req = Request::post("/api/notes")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::CREATED);
    let obj = json.as_object().unwrap();
    // Required fields per /docs/spec/api/types.md
    assert!(obj.contains_key("id"), "must have id");
    assert!(obj.contains_key("title"), "must have title");
    assert!(obj.contains_key("workspace_id"), "must have workspace_id");
    assert!(obj.contains_key("note_kind"), "must have note_kind");
    assert!(obj.contains_key("current_version"), "must have current_version");
    assert_eq!(json["title"], "Snapshot Test Note");
    assert_eq!(json["note_kind"], "markdown");
    assert_eq!(json["current_version"], 1);
}

/// Snapshot: list notes with no results
#[tokio::test]
async fn snapshot_list_notes_empty() {
    let ws_id = "00000000-0000-0000-0000-000000000002";
    let req = Request::get(&format!("/api/notes?workspace_id={ws_id}"))
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    assert!(json.is_array(), "list response must be array");
    assert_eq!(json.as_array().unwrap().len(), 0);
}

/// Snapshot: search response shape
#[tokio::test]
async fn snapshot_search_empty() {
    let ws_id = "00000000-0000-0000-0000-000000000003";
    let req = Request::get(&format!(
        "/api/search?q=nonexistent&workspace_id={ws_id}"
    ))
    .body(Body::empty())
    .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["results"].is_array(), "search results must be array");
}

/// Snapshot: session check returns authenticated: false
#[tokio::test]
async fn snapshot_session_unauthenticated() {
    let req = Request::get("/api/auth/session")
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["authenticated"], false);
}

/// Snapshot: register → response has message, username, id
#[tokio::test]
async fn snapshot_register_response() {
    let body = serde_json::json!({
        "username": "snapshot_admin",
        "password": "snapshot_pass_123"
    });
    let req = Request::post("/api/setup/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::CREATED);
    let obj = json.as_object().unwrap();
    assert!(obj.contains_key("message"), "must have message");
    assert!(obj.contains_key("username"), "must have username");
    assert!(obj.contains_key("id"), "must have id");
    assert_eq!(json["username"], "snapshot_admin");
}

/// Snapshot: metrics endpoint returns counters
#[tokio::test]
async fn snapshot_metrics_response() {
    let req = Request::get("/api/metrics")
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::OK);
    let obj = json.as_object().unwrap();
    assert!(obj.contains_key("total_requests"));
    assert!(obj.contains_key("total_errors_4xx"));
    assert!(obj.contains_key("total_errors_5xx"));
    assert!(obj.contains_key("avg_latency_us"));
}

/// Snapshot: error response envelope shape
#[tokio::test]
async fn snapshot_error_response_shape() {
    // Hit list_notes without workspace_id → 400 with error envelope
    let req = Request::get("/api/notes")
        .body(Body::empty())
        .unwrap();
    let (status, json) = send(app(), req).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let obj = json.as_object().unwrap();
    assert!(obj.contains_key("code"), "error must have code");
    assert!(obj.contains_key("message"), "error must have message");
    assert!(obj.contains_key("request_id"), "error must have request_id");
}
