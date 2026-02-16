/// T1 integration tests: Auth + Session flow.
///
/// Spec: /docs/spec/security/auth.md, /docs/spec/security/sessions.md
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::rate_limit::{RateLimitConfig, RateLimiter};
use kjxlkj_http::state::AppState;
use std::sync::Arc;
use std::time::Duration;
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

/// Per IMP-SEC-02: auth endpoint rate limiting.
/// After exceeding max_requests, login returns 429 Too Many Requests.
#[tokio::test]
async fn auth_rate_limit_rejects_excess() {
    let mut state = AppState::new();
    // Set very low limit for testing: 2 requests per 60s
    state.auth_rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig {
        max_requests: 2,
        window: Duration::from_secs(60),
    }));
    // First two requests should succeed (invalid credentials, but not rate-limited)
    for _ in 0..2 {
        let app = build_app_with_state(state.clone());
        let resp = app
            .oneshot(
                Request::post("/api/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"username":"x","password":"y"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
    }
    // Third request should be rate-limited
    let app3 = build_app_with_state(state.clone());
    let resp3 = app3
        .oneshot(
            Request::post("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"x","password":"y"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp3.status(), StatusCode::TOO_MANY_REQUESTS);
    let retry_after = resp3.headers().get("retry-after");
    assert!(retry_after.is_some(), "must include Retry-After header");
}
