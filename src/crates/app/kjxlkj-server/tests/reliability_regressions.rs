use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn imp_002_imp_003_and_usr_001_regression_guards() {
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
    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-reg-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-reg-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

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

    let preauth_session = client
        .get(format!("{base_url}/api/auth/session"))
        .send()
        .await
        .expect("pre-auth session request");
    assert_eq!(preauth_session.status(), StatusCode::UNAUTHORIZED);

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Regression note",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let create_note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = create_note_body["note_id"]
        .as_str()
        .expect("note id string")
        .to_owned();

    let patch_once = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello replay"}],
            "idempotency_key": "imp-002-key"
        }))
        .send()
        .await
        .expect("first patch request");
    assert_eq!(patch_once.status(), StatusCode::OK);

    let patch_once_body: serde_json::Value = patch_once
        .json()
        .await
        .expect("parse first patch response");

    let patch_twice = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"retain": 1}],
            "idempotency_key": "imp-002-key"
        }))
        .send()
        .await
        .expect("duplicate patch request");
    assert_eq!(patch_twice.status(), StatusCode::OK);

    let patch_twice_body: serde_json::Value = patch_twice
        .json()
        .await
        .expect("parse duplicate patch response");

    assert_eq!(patch_once_body["version"], patch_twice_body["version"]);
    assert_eq!(patch_once_body["event_seq"], patch_twice_body["event_seq"]);

    let upsert_metadata = client
        .put(format!("{base_url}/api/notes/{note_id}/metadata/priority"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({"level": 1}))
        .send()
        .await
        .expect("upsert metadata request");
    assert_eq!(upsert_metadata.status(), StatusCode::OK);

    let delete_metadata = client
        .delete(format!("{base_url}/api/notes/{note_id}/metadata/priority"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .send()
        .await
        .expect("delete metadata request");
    assert_eq!(delete_metadata.status(), StatusCode::NO_CONTENT);

    server_handle.stop(true).await;
}
