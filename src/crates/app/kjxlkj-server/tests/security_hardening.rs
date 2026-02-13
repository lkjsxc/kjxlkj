use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;
use time::{Duration, OffsetDateTime};

#[tokio::test]
async fn mutation_routes_enforce_csrf_and_role_boundaries() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");
    let viewer_hash = hash_password("viewer-password").expect("hash viewer password");

    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-sec-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-sec-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let viewer = repos::users::create_user(
        &pool,
        &format!("viewer-sec-{token}@example.com"),
        "Viewer",
        &viewer_hash,
        "viewer",
    )
    .await
    .expect("create viewer user");

    repos::workspaces::upsert_workspace_member(&pool, workspace.id, viewer.id, "viewer")
        .await
        .expect("upsert viewer membership");

    let owner_session = new_session_id();
    let owner_csrf = new_csrf_token();
    repos::auth::create_session(
        &pool,
        owner_session,
        owner.id,
        &owner_csrf,
        OffsetDateTime::now_utc() + Duration::days(7),
    )
    .await
    .expect("create owner session");

    let viewer_session = new_session_id();
    let viewer_csrf = new_csrf_token();
    repos::auth::create_session(
        &pool,
        viewer_session,
        viewer.id,
        &viewer_csrf,
        OffsetDateTime::now_utc() + Duration::days(7),
    )
    .await
    .expect("create viewer session");

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

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");
    let base_url = format!("http://{}", address);

    let viewer_create_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={viewer_session}"))
        .header("x-csrf-token", &viewer_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {"kind": "noop"},
            "enabled": true
        }))
        .send()
        .await
        .expect("viewer create rule request");
    assert_eq!(viewer_create_rule.status(), StatusCode::FORBIDDEN);

    let viewer_backup = client
        .post(format!("{base_url}/api/admin/backup/sql"))
        .header("Cookie", format!("kjxlkj_session={viewer_session}"))
        .header("x-csrf-token", &viewer_csrf)
        .send()
        .await
        .expect("viewer backup request");
    assert_eq!(viewer_backup.status(), StatusCode::FORBIDDEN);

    let owner_view_without_csrf = client
        .post(format!("{base_url}/api/views"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .json(&json!({
            "workspace_id": workspace.id,
            "query_json": {},
            "sort": "updated_desc",
            "filters": {}
        }))
        .send()
        .await
        .expect("owner create view without csrf request");
    assert_eq!(owner_view_without_csrf.status(), StatusCode::FORBIDDEN);

    let owner_rule_without_csrf = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {"kind": "noop"},
            "enabled": true
        }))
        .send()
        .await
        .expect("owner create rule without csrf request");
    assert_eq!(owner_rule_without_csrf.status(), StatusCode::FORBIDDEN);

    server_handle.stop(true).await;
}

#[tokio::test]
async fn expired_sessions_are_rejected_and_login_is_rate_limited() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");

    let (owner, _workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-exp-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-exp-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let expired_session = new_session_id();
    let expired_csrf = new_csrf_token();
    repos::auth::create_session(
        &pool,
        expired_session,
        owner.id,
        &expired_csrf,
        OffsetDateTime::now_utc() - Duration::minutes(1),
    )
    .await
    .expect("create expired session");

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

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");
    let base_url = format!("http://{}", address);

    let expired_session_response = client
        .get(format!("{base_url}/api/auth/session"))
        .header("Cookie", format!("kjxlkj_session={expired_session}"))
        .send()
        .await
        .expect("expired session request");
    assert_eq!(expired_session_response.status(), StatusCode::UNAUTHORIZED);

    let mut saw_rate_limited = false;
    for _ in 0..24 {
        let response = client
            .post(format!("{base_url}/api/auth/login"))
            .json(&json!({
                "email": format!("owner-exp-{token}@example.com"),
                "password": "wrong-password"
            }))
            .send()
            .await
            .expect("login request in rate-limit loop");

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            saw_rate_limited = true;
            break;
        }
    }

    assert!(saw_rate_limited, "expected login rate limiter to return 429");

    server_handle.stop(true).await;
}

#[tokio::test]
async fn secure_cookie_flag_is_present_when_secure_cookies_enabled() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
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
        &format!("owner-cookie-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-cookie-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let state = AppState::new(pool.clone(), true);
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
        .post(format!("http://{address}/api/auth/login"))
        .json(&json!({
            "email": format!("owner-cookie-{token}@example.com"),
            "password": "owner-password"
        }))
        .send()
        .await
        .expect("login request");

    assert_eq!(response.status(), StatusCode::OK);
    let cookie_header = response
        .headers()
        .get("set-cookie")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    assert!(cookie_header.contains("Secure"));

    server_handle.stop(true).await;
}
