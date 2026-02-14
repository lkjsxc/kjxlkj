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
async fn api_saved_view_crud_contract() {
    let app = app().await;

    let (_, setup_body, set_cookie) = send_json(
        &app,
        Method::POST,
        "/api/setup/register",
        json!({"email":"owner@example.com","display_name":"Owner","password":"pw"}),
        None,
        None,
    )
    .await;

    let csrf = setup_body
        .get("csrf_token")
        .and_then(Value::as_str)
        .expect("csrf token")
        .to_string();
    let cookie_pair = set_cookie
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_string();

    let (status, create_body, _) = send_json(
        &app,
        Method::POST,
        "/api/views",
        json!({
            "workspace_id":"ws-1",
            "query_json":{"term":"roadmap"},
            "sort":"updated_desc",
            "filters":{"tags":["planning"]}
        }),
        Some(&cookie_pair),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let view_id = create_body
        .get("item")
        .and_then(|item| item.get("id"))
        .and_then(Value::as_str)
        .expect("view id")
        .to_string();

    let (status, list_body, _) = send_json(
        &app,
        Method::GET,
        "/api/views?workspace_id=ws-1",
        json!({}),
        Some(&cookie_pair),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list_body.get("items").and_then(Value::as_array).map(Vec::len), Some(1));

    let (status, update_body, _) = send_json(
        &app,
        Method::PATCH,
        &format!("/api/views/{view_id}"),
        json!({"sort":"title_asc"}),
        Some(&cookie_pair),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        update_body
            .get("item")
            .and_then(|item| item.get("sort"))
            .and_then(Value::as_str),
        Some("title_asc")
    );

    let (status, _, _) = send_json(
        &app,
        Method::DELETE,
        &format!("/api/views/{view_id}"),
        json!({}),
        Some(&cookie_pair),
        Some(&csrf),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, list_body, _) = send_json(
        &app,
        Method::GET,
        "/api/views?workspace_id=ws-1",
        json!({}),
        Some(&cookie_pair),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list_body.get("items").and_then(Value::as_array).map(Vec::len), Some(0));
}
