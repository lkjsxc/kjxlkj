use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn restart_recovery_preserves_committed_events_and_projections() {
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
        &format!("owner-ops-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-ops-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let start_server = |state: AppState| {
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
        (address, server)
    };

    let state_a = AppState::new(pool.clone(), false);
    let (address_a, server_a) = start_server(state_a);
    let handle_a = server_a.handle();
    let _task_a = tokio::spawn(server_a);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");

    let base_url_a = format!("http://{}", address_a);
    let create_note = client
        .post(format!("{base_url_a}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Recovery note",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "before restart"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let create_body: serde_json::Value = create_note.json().await.expect("parse create note response");
    let note_id = create_body["note_id"].as_str().expect("note id string").to_owned();

    let patch_note = client
        .patch(format!("{base_url_a}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 14}, {"insert": "after restart"}],
            "idempotency_key": "ops-restart-patch"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    handle_a.stop(true).await;

    let state_b = AppState::new(pool.clone(), false);
    let (address_b, server_b) = start_server(state_b);
    let handle_b = server_b.handle();
    let _task_b = tokio::spawn(server_b);

    let base_url_b = format!("http://{}", address_b);

    let ready = client
        .get(format!("{base_url_b}/api/readyz"))
        .send()
        .await
        .expect("readyz request after restart");
    assert_eq!(ready.status(), StatusCode::OK);

    let get_note = client
        .get(format!("{base_url_b}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .send()
        .await
        .expect("get note after restart request");
    assert_eq!(get_note.status(), StatusCode::OK);

    let note_body: serde_json::Value = get_note.json().await.expect("parse get note response");
    assert_eq!(note_body["markdown"], json!("after restart"));

    let history = client
        .get(format!("{base_url_b}/api/notes/{note_id}/history"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .send()
        .await
        .expect("get note history request");
    assert_eq!(history.status(), StatusCode::OK);

    let history_body: serde_json::Value = history.json().await.expect("parse note history response");
    assert!(history_body["events"].as_array().map_or(0, Vec::len) >= 2);

    handle_b.stop(true).await;
}
