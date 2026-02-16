/// T1 integration tests: CSRF enforcement.
///
/// Spec: /docs/spec/security/csrf.md
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;

#[tokio::test]
async fn csrf_rejects_cookie_session_without_csrf_header() {
    let state = AppState::new();
    let (token, _csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .header("cookie", format!("kjxlkj_session={}", token))
                .body(Body::from(
                    serde_json::json!({"workspace_id": uuid::Uuid::new_v4(), "title": "t"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn csrf_allows_cookie_session_with_valid_csrf_header() {
    let state = AppState::new();
    let (token, csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .header("cookie", format!("kjxlkj_session={}", token))
                .header("x-csrf-token", &csrf)
                .body(Body::from(
                    serde_json::json!({"workspace_id": uuid::Uuid::new_v4(), "title": "csrf ok"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn csrf_allows_bearer_token_without_csrf_header() {
    let state = AppState::new();
    let (token, _csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    serde_json::json!({"workspace_id": uuid::Uuid::new_v4(), "title": "api"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn csrf_rejects_wrong_csrf_token() {
    let state = AppState::new();
    let (token, _csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/notes")
                .header("content-type", "application/json")
                .header("cookie", format!("kjxlkj_session={}", token))
                .header("x-csrf-token", "wrong-csrf-value")
                .body(Body::from(
                    serde_json::json!({"workspace_id": uuid::Uuid::new_v4(), "title": "t"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
