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

async fn call(
    app: &axum::Router,
    method: Method,
    uri: &str,
    body: Value,
    cookie: Option<&str>,
    csrf: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    if let Some(csrf) = csrf {
        builder = builder.header("x-csrf-token", csrf);
    }

    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).expect("request"))
        .await
        .expect("response");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let value = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice::<Value>(&bytes).expect("json response")
    };
    (status, value)
}

#[tokio::test]
async fn automation_librarian_provider_validation_and_review() {
    let app = app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/setup/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({"email":"owner@example.com","display_name":"Owner","password":"pw"})
                        .to_string(),
                ))
                .expect("setup request"),
        )
        .await
        .expect("setup response");

    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_string();
    let body = serde_json::from_slice::<Value>(
        &to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("setup bytes"),
    )
    .expect("setup body");
    let csrf = body
        .get("csrf_token")
        .and_then(Value::as_str)
        .expect("csrf token")
        .to_string();

    let (status, body) = call(
        &app,
        Method::POST,
        "/api/automation/rules",
        json!({
            "workspace_id":"ws-1",
            "trigger":"manual",
            "action_json":{
                "kind":"librarian_structure",
                "provider":{"provider_kind":"invalid"},
                "protocol":"xml_attrless"
            }
        }),
        Some(&cookie),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body.get("code"), Some(&json!("RULE_INVALID")));

    let (status, body) = call(
        &app,
        Method::POST,
        "/api/automation/rules",
        json!({
            "workspace_id":"ws-1",
            "trigger":"manual",
            "action_json":{
                "kind":"librarian_structure",
                "provider":{"provider_kind":"lmstudio"},
                "protocol":"xml_attrless"
            }
        }),
        Some(&cookie),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let rule_id = body
        .get("item")
        .and_then(|item| item.get("id"))
        .and_then(Value::as_str)
        .expect("rule id")
        .to_string();

    let (status, body) = call(
        &app,
        Method::POST,
        &format!("/api/automation/rules/{rule_id}/launch"),
        json!({}),
        Some(&cookie),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let run_id = body
        .get("item")
        .and_then(|item| item.get("id"))
        .and_then(Value::as_str)
        .expect("run id")
        .to_string();

    let (status, _) = call(
        &app,
        Method::POST,
        &format!("/api/automation/runs/{run_id}/review"),
        json!({"decisions":{"operation-1":"apply"}}),
        Some(&cookie),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
}
