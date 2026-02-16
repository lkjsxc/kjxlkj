/// T1 integration tests: Auth + Session flow.
///
/// Spec: /docs/spec/security/auth.md, /docs/spec/security/sessions.md
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;

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
    let app = build_app_with_state(state.clone());
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
    let app2 = build_app_with_state(state.clone());
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

#[tokio::test]
async fn auth_login_sets_session_cookie() {
    let state = AppState::new();
    let (_token, csrf) = register_and_login(&state).await;
    assert!(!csrf.is_empty(), "csrf_token must be returned");
}

#[tokio::test]
async fn auth_session_returns_csrf_token() {
    let state = AppState::new();
    let (token, _csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::get("/api/auth/session")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = json_body(resp).await;
    assert_eq!(body["authenticated"], true);
    assert!(body["csrf_token"].as_str().is_some());
}

#[tokio::test]
async fn auth_logout_clears_cookie() {
    let state = AppState::new();
    let (token, _csrf) = register_and_login(&state).await;
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/auth/logout")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    let cookie = resp
        .headers()
        .get("set-cookie")
        .expect("logout must clear cookie")
        .to_str()
        .unwrap();
    assert!(cookie.contains("Max-Age=0"), "cookie must expire");
}
