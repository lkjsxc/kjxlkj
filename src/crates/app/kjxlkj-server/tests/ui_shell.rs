use actix_web::{web, App, HttpServer};
use kjxlkj_auth::hash_password;
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn root_serves_workspace_shell_markup() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let address = listener.local_addr().expect("read bound addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let body = reqwest::Client::new()
        .get(format!("http://{address}/"))
        .send()
        .await
        .expect("request root")
        .text()
        .await
        .expect("read html body");

    assert!(body.contains("id=\"commandPalette\""));
    assert!(body.contains("id=\"menuToggle\""));
    assert!(body.contains("id=\"backlinksList\""));
    assert!(body.contains("id=\"contextBack\""));
    assert!(body.contains("id=\"noteTitle\""));
    assert!(body.contains("Setup is locked. Login-only UI is active."));
    assert!(body.contains("event.key.toLowerCase() === 'k'"));
    assert!(body.contains("classList.toggle('nav-collapsed')"));
    assert!(body.contains("@media (max-width: 640px)"));
    assert!(body.contains("overflow-y: auto"));
    assert!(body.contains("Date.now()"));
    assert!(body.contains("Math.random().toString(16).slice(2)"));
    assert!(!body.contains("Save Now"));
    assert!(!body.contains("Delete Note"));

    server_handle.stop(true).await;
}

#[tokio::test]
async fn setup_lock_conflict_is_deterministic_for_login_only_switch() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");

    let _ = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-ui-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-ui-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner and workspace");

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let address = listener.local_addr().expect("read bound addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let response = reqwest::Client::new()
        .post(format!("http://{address}/api/setup/register"))
        .json(&json!({
            "email": format!("blocked-ui-{token}@example.com"),
            "password": "password-123",
            "display_name": "Blocked",
            "workspace_name": "Blocked Workspace"
        }))
        .send()
        .await
        .expect("request setup register");

    assert_eq!(response.status(), StatusCode::CONFLICT);
    let payload: serde_json::Value = response.json().await.expect("parse setup conflict body");
    assert_eq!(payload["code"], json!("SETUP_LOCKED"));

    server_handle.stop(true).await;
}
