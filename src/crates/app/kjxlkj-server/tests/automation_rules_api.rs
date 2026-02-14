use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn automation_rule_crud_validation_and_forbidden_paths() {
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
    let editor_hash = hash_password("editor-password").expect("hash editor password");

    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-auto-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-auto-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let editor = repos::users::create_user(
        &pool,
        &format!("editor-auto-{token}@example.com"),
        "Editor",
        &editor_hash,
        "editor",
    )
    .await
    .expect("create editor user");

    repos::workspaces::upsert_workspace_member(&pool, workspace.id, editor.id, "editor")
        .await
        .expect("upsert editor workspace membership");

    let owner_session = new_session_id();
    let owner_csrf = new_csrf_token();
    repos::auth::create_session(&pool, owner_session, owner.id, &owner_csrf, session_expiry(7))
        .await
        .expect("create owner session");

    let editor_session = new_session_id();
    let editor_csrf = new_csrf_token();
    repos::auth::create_session(&pool, editor_session, editor.id, &editor_csrf, session_expiry(7))
        .await
        .expect("create editor session");

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

    let create_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"workspace_scope": "all"},
            "action_json": {"kind": "tag_note", "tag": "triaged"},
            "enabled": true
        }))
        .send()
        .await
        .expect("create automation rule request");
    assert_eq!(create_rule.status(), StatusCode::CREATED);

    let create_body: serde_json::Value = create_rule
        .json()
        .await
        .expect("parse create automation rule response");
    let rule_id = create_body["rule"]["id"]
        .as_str()
        .expect("rule id string")
        .to_owned();

    let list_rules = client
        .get(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("list automation rules request");
    assert_eq!(list_rules.status(), StatusCode::OK);

    let list_body: serde_json::Value = list_rules.json().await.expect("parse list rules response");
    assert_eq!(list_body["rules"].as_array().map_or(0, Vec::len), 1);

    let patch_rule = client
        .patch(format!("{base_url}/api/automation/rules/{rule_id}"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "enabled": false
        }))
        .send()
        .await
        .expect("patch automation rule request");
    assert_eq!(patch_rule.status(), StatusCode::OK);

    let patch_body: serde_json::Value = patch_rule.json().await.expect("parse patch rule response");
    assert_eq!(patch_body["rule"]["enabled"], json!(false));

    let forbidden_create = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={editor_session}"))
        .header("x-csrf-token", &editor_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {"kind": "tag_note", "tag": "blocked"},
            "enabled": true
        }))
        .send()
        .await
        .expect("forbidden create automation rule request");
    assert_eq!(forbidden_create.status(), StatusCode::FORBIDDEN);

    let invalid_librarian_provider = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {
                "kind": "librarian_structure",
                "provider": {"provider_kind": "unknown"},
                "protocol": "xml_attrless",
                "model": "missing-provider-model",
                "plan": {
                    "goal": "Organize",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["docs"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 12,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("invalid librarian provider create request");
    assert_eq!(invalid_librarian_provider.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let missing_plan = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {
                "kind": "librarian_structure",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/model",
                    "base_url": "http://127.0.0.1:1234/v1/chat/completions",
                    "timeout_ms": 200,
                    "retry_limit": 1
                },
                "protocol": "xml_attrless"
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("missing plan create request");
    assert_eq!(missing_plan.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let invalid_max_operations = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {},
            "action_json": {
                "kind": "librarian_structure",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/model",
                    "base_url": "http://127.0.0.1:1234/v1/chat/completions",
                    "timeout_ms": 200,
                    "retry_limit": 1
                },
                "protocol": "xml_attrless",
                "plan": {
                    "goal": "Organize",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["docs"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 0,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("invalid max operations create request");
    assert_eq!(invalid_max_operations.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let delete_rule = client
        .delete(format!("{base_url}/api/automation/rules/{rule_id}"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .send()
        .await
        .expect("delete automation rule request");
    assert_eq!(delete_rule.status(), StatusCode::NO_CONTENT);

    let list_after_delete = client
        .get(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("list rules after delete request");
    assert_eq!(list_after_delete.status(), StatusCode::OK);

    let after_delete_body: serde_json::Value = list_after_delete
        .json()
        .await
        .expect("parse list rules after delete response");
    assert_eq!(after_delete_body["rules"].as_array().map_or(0, Vec::len), 0);

    server_handle.stop(true).await;
}
