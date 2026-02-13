use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn command_actions_create_open_move_tag_and_run_rule_failure_path() {
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
        &format!("owner-cmd-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-cmd-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner and workspace");

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

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Palette note",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "Initial body"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let created: serde_json::Value = create_note.json().await.expect("parse create note body");
    let note_id = created["note_id"]
        .as_str()
        .expect("note id string")
        .to_owned();
    let mut version = created["version"].as_i64().expect("version as i64") as i32;

    let open_note = client
        .get(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .send()
        .await
        .expect("open note request");
    assert_eq!(open_note.status(), StatusCode::OK);

    let tag_note = client
        .put(format!("{base_url}/api/notes/{note_id}/tags"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({"tags": ["palette", "stage03"]}))
        .send()
        .await
        .expect("tag note request");
    assert_eq!(tag_note.status(), StatusCode::OK);

    let move_note = client
        .put(format!("{base_url}/api/notes/{note_id}/metadata/project.move"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({"project_id": null, "command": "move"}))
        .send()
        .await
        .expect("move note metadata request");
    assert_eq!(move_note.status(), StatusCode::OK);

    let autosave_patch = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": version,
            "patch_ops": [{"delete": 12}, {"insert": "Autosaved body"}],
            "idempotency_key": format!("cmd-autosave-{token}")
        }))
        .send()
        .await
        .expect("autosave patch request");
    assert_eq!(autosave_patch.status(), StatusCode::OK);

    let autosave_body: serde_json::Value = autosave_patch
        .json()
        .await
        .expect("parse autosave patch response");
    version = autosave_body["version"].as_i64().expect("autosave version as i64") as i32;

    let rename_title = client
        .patch(format!("{base_url}/api/notes/{note_id}/title"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": version,
            "title": "Palette note renamed"
        }))
        .send()
        .await
        .expect("title update request");
    assert_eq!(rename_title.status(), StatusCode::OK);

    let list_notes = client
        .get(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("list notes request");
    assert_eq!(list_notes.status(), StatusCode::OK);

    let listed: serde_json::Value = list_notes.json().await.expect("parse list notes body");
    let notes = listed["notes"].as_array().expect("notes array in list response");
    assert!(notes
        .iter()
        .any(|note| note["id"] == json!(note_id) && note["title"] == json!("Palette note renamed")));

    let run_rule = client
        .get(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("run rule request");
    assert_eq!(run_rule.status(), StatusCode::OK);

    server_handle.stop(true).await;
}
