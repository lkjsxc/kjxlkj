use axum::{
    body::{to_bytes, Body},
    http::{header, Method, Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;
use crate::{routes::build_router, state::AppState};

async fn app() -> axum::Router {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite in-memory pool");
    build_router(AppState::new(db_pool))
}

async fn send_json(
    app: &axum::Router,
    method: Method,
    uri: &str,
    body: Value,
    cookie: Option<&str>,
    csrf: Option<&str>,
) -> (StatusCode, Value, Option<String>) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(cookie) = cookie {
        req = req.header(header::COOKIE, cookie);
    }
    if let Some(csrf) = csrf {
        req = req.header("x-csrf-token", csrf);
    }
    let response = app
        .clone()
        .oneshot(req.body(Body::from(body.to_string())).expect("request body"))
        .await
        .expect("response");

    let status = response.status();
    let set_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response bytes");
    let json = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice::<Value>(&bytes).expect("json response")
    };

    (status, json, set_cookie)
}
#[tokio::test]
async fn api_auth_setup_lock_and_session_flow() {
    let app = app().await;

    let (status, body, set_cookie) = send_json(
        &app,
        Method::POST,
        "/api/setup/register",
        json!({"email":"owner@example.com","display_name":"Owner","password":"pw"}),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let csrf = body
        .get("csrf_token")
        .and_then(Value::as_str)
        .expect("csrf token");
    let cookie = set_cookie.expect("set-cookie");
    let cookie_pair = cookie.split(';').next().expect("cookie pair").to_string();

    let (status, body, _) = send_json(
        &app,
        Method::POST,
        "/api/setup/register",
        json!({"email":"other@example.com","display_name":"Other","password":"pw"}),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body.get("code"), Some(&json!("SETUP_LOCKED")));

    let (status, body, _) = send_json(
        &app,
        Method::GET,
        "/api/auth/session",
        json!({}),
        Some(&cookie_pair),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.get("user").and_then(|v| v.get("email")), Some(&json!("owner@example.com")));

    let (status, _, _) = send_json(
        &app,
        Method::POST,
        "/api/auth/logout",
        json!({}),
        Some(&cookie_pair),
        Some(csrf),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body, _) = send_json(
        &app,
        Method::GET,
        "/api/auth/session",
        json!({}),
        Some(&cookie_pair),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body.get("code"), Some(&json!("AUTH_REQUIRED")));
}

#[tokio::test]
async fn api_note_conflict_and_metadata_delete_contract() {
    let app = app().await;

    let (_, body, set_cookie) = send_json(
        &app,
        Method::POST,
        "/api/setup/register",
        json!({"email":"owner@example.com","display_name":"Owner","password":"pw"}),
        None,
        None,
    )
    .await;
    let csrf = body.get("csrf_token").and_then(Value::as_str).expect("csrf");
    let cookie_pair = set_cookie
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_string();

    let (status, body, _) = send_json(
        &app,
        Method::POST,
        "/api/notes",
        json!({"workspace_id":"ws-1","title":"A","markdown":"abc"}),
        Some(&cookie_pair),
        Some(csrf),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let note_id = body
        .get("item")
        .and_then(|item| item.get("id"))
        .and_then(Value::as_str)
        .expect("note id")
        .to_string();

    let (status, body, _) = send_json(
        &app,
        Method::PATCH,
        &format!("/api/notes/{note_id}"),
        json!({"base_version":0,"patch_ops":[{"retain":3}],"idempotency_key":"k1"}),
        Some(&cookie_pair),
        Some(csrf),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body.get("code"), Some(&json!("VERSION_CONFLICT")));

    let (status, _, _) = send_json(
        &app,
        Method::PUT,
        &format!("/api/notes/{note_id}/metadata/category"),
        json!({"value":"test"}),
        Some(&cookie_pair),
        Some(csrf),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _, _) = send_json(
        &app,
        Method::DELETE,
        &format!("/api/notes/{note_id}/metadata/category"),
        json!({}),
        Some(&cookie_pair),
        Some(csrf),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}
