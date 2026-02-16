/// Shared test helpers for T1 integration tests.
///
/// Provides app builder and response parsing utilities.
/// Spec: /docs/spec/technical/testing.md (Tier T1)
use axum::{body::Body, http::Request, Router};
use http_body_util::BodyExt;
use kjxlkj_http::{routes::api_router, state::AppState};
use tower::ServiceExt;

#[allow(dead_code)]
pub fn build_app() -> Router {
    api_router(AppState::new())
}

#[allow(dead_code)]
pub fn build_app_with_state(state: AppState) -> Router {
    api_router(state)
}

#[allow(dead_code)]
pub async fn json_body(resp: axum::response::Response) -> serde_json::Value {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
}

/// Register first user and login. Returns (token, csrf_token).
#[allow(dead_code)]
pub async fn register_and_login(state: &AppState) -> (String, String) {
    let app = build_app_with_state(state.clone());
    app.oneshot(
        Request::post("/api/setup/register")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"username":"admin","password":"Test1234!"}"#))
            .unwrap(),
    )
    .await
    .unwrap();
    let app2 = build_app_with_state(state.clone());
    let resp = app2
        .oneshot(
            Request::post("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"admin","password":"Test1234!"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let cookie = resp
        .headers()
        .get("set-cookie")
        .expect("login must set cookie")
        .to_str()
        .unwrap()
        .to_string();
    assert!(cookie.contains("kjxlkj_session="));
    assert!(cookie.contains("HttpOnly"));
    assert!(cookie.contains("SameSite=Lax"));
    assert!(cookie.contains("Path=/"));
    let body = json_body(resp).await;
    let token = body["token"].as_str().unwrap().to_string();
    let csrf = body["csrf_token"].as_str().unwrap().to_string();
    (token, csrf)
}
