use actix_web::{web, App, HttpResponse, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn librarian_provider_modes_store_run_metadata_and_succeed() {
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
        &format!("owner-provider-success-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-provider-success-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let provider_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind provider server");
    let provider_address = provider_listener
        .local_addr()
        .expect("read provider address");

    let provider_server = HttpServer::new(|| {
        App::new()
            .route(
                "/openrouter/v1/chat/completions",
                web::post().to(|| async {
                    HttpResponse::Ok().json(json!({
                        "choices": [{
                            "message": {
                                "content": "<librarian_response><request_id>req_openrouter</request_id><status>ok</status><summary>ok</summary><operations></operations><warnings></warnings></librarian_response>"
                            }
                        }]
                    }))
                }),
            )
            .route(
                "/lmstudio/v1/chat/completions",
                web::post().to(|| async {
                    HttpResponse::Ok().json(json!({
                        "choices": [{
                            "message": {
                                "content": "<librarian_response><request_id>req_lmstudio</request_id><status>ok</status><summary>local</summary><operations></operations><warnings></warnings></librarian_response>"
                            }
                        }]
                    }))
                }),
            )
    })
    .listen(provider_listener)
    .expect("listen provider")
    .run();

    let provider_handle = provider_server.handle();
    let _provider_task = tokio::spawn(provider_server);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read bound app addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");

    let base_url = format!("http://{}", address);

    let openrouter_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/test-model",
                    "base_url": format!("http://{provider_address}/openrouter/v1/chat/completions"),
                    "timeout_ms": 400,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Structure provider output",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops", "docs"]},
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
        .expect("create openrouter rule request");
    assert_eq!(openrouter_rule.status(), StatusCode::CREATED);

    let openrouter_body: serde_json::Value = openrouter_rule
        .json()
        .await
        .expect("parse openrouter rule response");
    let openrouter_rule_id = uuid::Uuid::parse_str(
        openrouter_body["rule"]["id"]
            .as_str()
            .expect("openrouter rule id"),
    )
    .expect("parse openrouter rule id");

    let lmstudio_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "lmstudio",
                    "model": "lmstudio/local-model",
                    "base_url": format!("http://{provider_address}/lmstudio/v1/chat/completions"),
                    "timeout_ms": 400,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Structure local output",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops", "docs"]},
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
        .expect("create lmstudio rule request");
    assert_eq!(lmstudio_rule.status(), StatusCode::CREATED);

    let lmstudio_body: serde_json::Value = lmstudio_rule
        .json()
        .await
        .expect("parse lmstudio rule response");
    let lmstudio_rule_id = uuid::Uuid::parse_str(
        lmstudio_body["rule"]["id"]
            .as_str()
            .expect("lmstudio rule id"),
    )
    .expect("parse lmstudio rule id");

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Provider run target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = note_body["note_id"].as_str().expect("note id string");

    let patch_note = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello provider"}],
            "idempotency_key": "provider-adapter-success"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    let openrouter_run: (uuid::Uuid, String, Option<String>, Option<String>, serde_json::Value) =
        sqlx::query_as(
            "SELECT id, status, provider_kind, model, result_json
             FROM automation_runs
             WHERE rule_id = $1
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .bind(openrouter_rule_id)
        .fetch_one(&pool)
        .await
        .expect("query openrouter run");

    assert_eq!(openrouter_run.1, "succeeded");
    assert_eq!(openrouter_run.2.as_deref(), Some("openrouter"));
    assert_eq!(openrouter_run.3.as_deref(), Some("openrouter/test-model"));
    assert_eq!(openrouter_run.4["provider_kind"], json!("openrouter"));
    assert_eq!(openrouter_run.4["model"], json!("openrouter/test-model"));
    assert!(openrouter_run.4["operation_report"]["parsed_operations"].is_array());
    assert!(openrouter_run.4["operation_report"]["rejected_operations"].is_array());

    let lmstudio_run: (uuid::Uuid, String, Option<String>, Option<String>, serde_json::Value) =
        sqlx::query_as(
            "SELECT id, status, provider_kind, model, result_json
             FROM automation_runs
             WHERE rule_id = $1
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .bind(lmstudio_rule_id)
        .fetch_one(&pool)
        .await
        .expect("query lmstudio run");

    assert_eq!(lmstudio_run.1, "succeeded");
    assert_eq!(lmstudio_run.2.as_deref(), Some("lmstudio"));
    assert_eq!(lmstudio_run.3.as_deref(), Some("lmstudio/local-model"));
    assert_eq!(lmstudio_run.4["provider_kind"], json!("lmstudio"));
    assert_eq!(lmstudio_run.4["model"], json!("lmstudio/local-model"));
    assert!(lmstudio_run.4["operation_report"]["parsed_operations"].is_array());
    assert!(lmstudio_run.4["operation_report"]["rejected_operations"].is_array());

    let get_run = client
        .get(format!("{base_url}/api/automation/runs/{}", openrouter_run.0))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .send()
        .await
        .expect("get run request");
    assert_eq!(get_run.status(), StatusCode::OK);

    let get_run_body: serde_json::Value = get_run.json().await.expect("parse get run body");
    assert_eq!(get_run_body["run"]["provider_kind"], json!("openrouter"));
    assert_eq!(get_run_body["run"]["model"], json!("openrouter/test-model"));

    server_handle.stop(true).await;
    provider_handle.stop(true).await;
}

#[tokio::test]
async fn librarian_parse_failures_capture_diagnostics_with_deterministic_codes() {
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
        &format!("owner-provider-parse-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-provider-parse-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let malformed_output = "<librarian_response><request_id>req_bad</request_id><status>ok</status><summary>bad</summary><operations></operations><warnings></librarian_response>";
    let missing_tag_output = "<librarian_response><request_id>req_missing</request_id><status>ok</status><summary>missing</summary><operations></operations></librarian_response>";

    let provider_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind provider server");
    let provider_address = provider_listener
        .local_addr()
        .expect("read provider address");

    let provider_server = HttpServer::new(move || {
        App::new()
            .route(
                "/malformed/v1/chat/completions",
                web::post().to(move || async move {
                    HttpResponse::Ok().json(json!({
                        "choices": [{
                            "message": {
                                "content": malformed_output
                            }
                        }]
                    }))
                }),
            )
            .route(
                "/missing/v1/chat/completions",
                web::post().to(move || async move {
                    HttpResponse::Ok().json(json!({
                        "choices": [{
                            "message": {
                                "content": missing_tag_output
                            }
                        }]
                    }))
                }),
            )
    })
    .listen(provider_listener)
    .expect("listen provider")
    .run();

    let provider_handle = provider_server.handle();
    let _provider_task = tokio::spawn(provider_server);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read bound app addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");

    let base_url = format!("http://{}", address);

    let malformed_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/malformed-model",
                    "base_url": format!("http://{provider_address}/malformed/v1/chat/completions"),
                    "timeout_ms": 300,
                    "retry_limit": 0
                },
                "plan": {
                    "goal": "Malformed parse",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 8,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create malformed rule request");
    assert_eq!(malformed_rule.status(), StatusCode::CREATED);

    let malformed_rule_body: serde_json::Value = malformed_rule
        .json()
        .await
        .expect("parse malformed rule response");
    let malformed_rule_id = uuid::Uuid::parse_str(
        malformed_rule_body["rule"]["id"]
            .as_str()
            .expect("malformed rule id"),
    )
    .expect("parse malformed rule id");

    let missing_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "lmstudio",
                    "model": "lmstudio/missing-model",
                    "base_url": format!("http://{provider_address}/missing/v1/chat/completions"),
                    "timeout_ms": 300,
                    "retry_limit": 0
                },
                "plan": {
                    "goal": "Missing-tag parse",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 8,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create missing rule request");
    assert_eq!(missing_rule.status(), StatusCode::CREATED);

    let missing_rule_body: serde_json::Value = missing_rule
        .json()
        .await
        .expect("parse missing rule response");
    let missing_rule_id = uuid::Uuid::parse_str(
        missing_rule_body["rule"]["id"]
            .as_str()
            .expect("missing rule id"),
    )
    .expect("parse missing rule id");

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Parse failure target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = note_body["note_id"].as_str().expect("note id string");

    let patch_note = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello parse"}],
            "idempotency_key": "provider-parse-failures"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    let malformed_run: (String, Option<String>, Option<serde_json::Value>) = sqlx::query_as(
        "SELECT status, error_code, result_json
         FROM automation_runs
         WHERE rule_id = $1
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(malformed_rule_id)
    .fetch_one(&pool)
    .await
    .expect("query malformed run");

    assert_eq!(malformed_run.0, "failed");
    assert_eq!(malformed_run.1.as_deref(), Some("LIBRARIAN_PROTOCOL_INVALID"));
    let malformed_result = malformed_run.2.expect("malformed result json");
    assert_eq!(malformed_result["protocol"], json!("xml_attrless"));
    assert_eq!(
        malformed_result["raw_model_outputs"]
            .as_array()
            .map_or(0, Vec::len),
        3
    );
    assert_eq!(
        malformed_result["parse_diagnostics"]
            .as_array()
            .map_or(0, Vec::len),
        3
    );

    let missing_run: (String, Option<String>, Option<serde_json::Value>) = sqlx::query_as(
        "SELECT status, error_code, result_json
         FROM automation_runs
         WHERE rule_id = $1
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(missing_rule_id)
    .fetch_one(&pool)
    .await
    .expect("query missing run");

    assert_eq!(missing_run.0, "failed");
    assert_eq!(missing_run.1.as_deref(), Some("LIBRARIAN_PARSE_FAILED"));
    let missing_result = missing_run.2.expect("missing result json");
    assert_eq!(missing_result["protocol"], json!("xml_attrless"));
    assert_eq!(
        missing_result["raw_model_outputs"]
            .as_array()
            .map_or(0, Vec::len),
        3
    );
    assert_eq!(
        missing_result["parse_diagnostics"]
            .as_array()
            .map_or(0, Vec::len),
        3
    );

    server_handle.stop(true).await;
    provider_handle.stop(true).await;
}

#[tokio::test]
async fn librarian_operation_report_rejects_overflow_operations() {
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
        &format!("owner-provider-overflow-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-provider-overflow-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let provider_output = "<librarian_response><request_id>req_overflow</request_id><status>ok</status><summary>overflow</summary><operations><operation><operation_id>op_1</operation_id><kind>create_note</kind><target_path>/a</target_path><title>A</title><body_markdown>a</body_markdown><reason>a</reason><confidence>0.9</confidence></operation><operation><operation_id>op_2</operation_id><kind>create_note</kind><target_path>/b</target_path><title>B</title><body_markdown>b</body_markdown><reason>b</reason><confidence>0.8</confidence></operation><operation><operation_id>op_3</operation_id><kind>create_note</kind><target_path>/c</target_path><title>C</title><body_markdown>c</body_markdown><reason>c</reason><confidence>0.7</confidence></operation></operations><warnings><warning>overflow-test</warning></warnings></librarian_response>";

    let provider_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind provider server");
    let provider_address = provider_listener
        .local_addr()
        .expect("read provider address");

    let provider_server = HttpServer::new(move || {
        App::new().route(
            "/v1/chat/completions",
            web::post().to(move || async move {
                HttpResponse::Ok().json(json!({
                    "choices": [{
                        "message": {
                            "content": provider_output
                        }
                    }]
                }))
            }),
        )
    })
    .listen(provider_listener)
    .expect("listen provider")
    .run();

    let provider_handle = provider_server.handle();
    let _provider_task = tokio::spawn(provider_server);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read bound app addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
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
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/overflow-model",
                    "base_url": format!("http://{provider_address}/v1/chat/completions"),
                    "timeout_ms": 300,
                    "retry_limit": 0
                },
                "plan": {
                    "goal": "Overflow test",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 1,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create overflow rule request");
    assert_eq!(create_rule.status(), StatusCode::CREATED);

    let create_rule_body: serde_json::Value = create_rule
        .json()
        .await
        .expect("parse create overflow rule response");
    let rule_id = uuid::Uuid::parse_str(
        create_rule_body["rule"]["id"]
            .as_str()
            .expect("overflow rule id"),
    )
    .expect("parse overflow rule id");

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Overflow target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = note_body["note_id"].as_str().expect("note id string");

    let patch_note = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello overflow"}],
            "idempotency_key": "provider-overflow-guard"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    let run: (String, serde_json::Value) = sqlx::query_as(
        "SELECT status, result_json
         FROM automation_runs
         WHERE rule_id = $1
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(rule_id)
    .fetch_one(&pool)
    .await
    .expect("query overflow run");

    assert_eq!(run.0, "succeeded");
    assert_eq!(
        run.1["operation_report"]["parsed_operations"]
            .as_array()
            .map_or(0, Vec::len),
        3
    );
    let rejected = run.1["operation_report"]["rejected_operations"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert_eq!(rejected.len(), 2);
    assert!(
        rejected
            .iter()
            .all(|entry| entry["reason"] == json!("MAX_OPERATIONS_EXCEEDED")),
        "all overflow rejections should be MAX_OPERATIONS_EXCEEDED"
    );

    server_handle.stop(true).await;
    provider_handle.stop(true).await;
}

#[tokio::test]
async fn librarian_provider_timeout_and_outage_fail_with_deterministic_codes() {
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
        &format!("owner-provider-fail-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-provider-fail-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let timeout_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind timeout provider");
    let timeout_address = timeout_listener
        .local_addr()
        .expect("read timeout provider addr");

    let timeout_server = HttpServer::new(|| {
        App::new().route(
            "/slow/v1/chat/completions",
            web::post().to(|| async {
                sleep(Duration::from_millis(250)).await;
                HttpResponse::Ok().json(json!({
                    "choices": [{
                        "message": {
                            "content": "<librarian_response><status>ok</status></librarian_response>"
                        }
                    }]
                }))
            }),
        )
    })
    .listen(timeout_listener)
    .expect("listen timeout provider")
    .run();

    let timeout_handle = timeout_server.handle();
    let _timeout_task = tokio::spawn(timeout_server);

    let closed_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind temporary listener");
    let closed_address = closed_listener
        .local_addr()
        .expect("read temporary listener addr");
    drop(closed_listener);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read bound app addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");

    let base_url = format!("http://{}", address);

    let outage_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/offline-model",
                    "base_url": format!("http://{closed_address}/v1/chat/completions"),
                    "timeout_ms": 120,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Handle outage",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 8,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create outage rule request");
    assert_eq!(outage_rule.status(), StatusCode::CREATED);

    let outage_rule_body: serde_json::Value = outage_rule
        .json()
        .await
        .expect("parse outage rule response");
    let outage_rule_id = uuid::Uuid::parse_str(
        outage_rule_body["rule"]["id"]
            .as_str()
            .expect("outage rule id"),
    )
    .expect("parse outage rule id");

    let timeout_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "lmstudio",
                    "model": "lmstudio/slow-model",
                    "base_url": format!("http://{timeout_address}/slow/v1/chat/completions"),
                    "timeout_ms": 50,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Handle timeout",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["ops"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 8,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create timeout rule request");
    assert_eq!(timeout_rule.status(), StatusCode::CREATED);

    let timeout_rule_body: serde_json::Value = timeout_rule
        .json()
        .await
        .expect("parse timeout rule response");
    let timeout_rule_id = uuid::Uuid::parse_str(
        timeout_rule_body["rule"]["id"]
            .as_str()
            .expect("timeout rule id"),
    )
    .expect("parse timeout rule id");

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Failure target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = note_body["note_id"].as_str().expect("note id string");

    let patch_note = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello failure"}],
            "idempotency_key": "provider-adapter-failure"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    let outage_run: (String, Option<String>, Option<String>, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT status, provider_kind, model, error_code, error_detail
             FROM automation_runs
             WHERE rule_id = $1
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .bind(outage_rule_id)
        .fetch_one(&pool)
        .await
        .expect("query outage run");

    assert_eq!(outage_run.0, "failed");
    assert_eq!(outage_run.1.as_deref(), Some("openrouter"));
    assert_eq!(outage_run.2.as_deref(), Some("openrouter/offline-model"));
    assert_eq!(outage_run.3.as_deref(), Some("LLM_PROVIDER_UNREACHABLE"));
    assert!(
        outage_run
            .4
            .as_deref()
            .unwrap_or_default()
            .contains("attempt=2/2"),
        "expected deterministic retry attempt evidence"
    );

    let timeout_run: (String, Option<String>, Option<String>, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT status, provider_kind, model, error_code, error_detail
             FROM automation_runs
             WHERE rule_id = $1
             ORDER BY created_at DESC
             LIMIT 1",
        )
        .bind(timeout_rule_id)
        .fetch_one(&pool)
        .await
        .expect("query timeout run");

    assert_eq!(timeout_run.0, "failed");
    assert_eq!(timeout_run.1.as_deref(), Some("lmstudio"));
    assert_eq!(timeout_run.2.as_deref(), Some("lmstudio/slow-model"));
    assert_eq!(timeout_run.3.as_deref(), Some("LLM_PROVIDER_TIMEOUT"));
    assert!(
        timeout_run
            .4
            .as_deref()
            .unwrap_or_default()
            .contains("attempt=2/2"),
        "expected deterministic retry attempt evidence"
    );

    server_handle.stop(true).await;
    timeout_handle.stop(true).await;
}

#[tokio::test]
async fn librarian_operation_report_rejects_out_of_scope_operations() {
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
        &format!("owner-provider-scope-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-provider-scope-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let out_of_scope_note_id = uuid::Uuid::now_v7().to_string();
    let provider_output = format!(
        "<librarian_response><request_id>req_scope_1</request_id><status>ok</status><summary>scope</summary><operations><operation><operation_id>op_scope</operation_id><kind>rewrite_note</kind><target_note_id>{}</target_note_id><title>Scoped rewrite</title><body_markdown>updated</body_markdown><reason>scope test</reason><confidence>0.9</confidence></operation></operations><warnings></warnings></librarian_response>",
        out_of_scope_note_id
    );

    let provider_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind provider server");
    let provider_address = provider_listener
        .local_addr()
        .expect("read provider address");

    let provider_server = HttpServer::new(move || {
        let output = provider_output.clone();
        App::new().route(
            "/v1/chat/completions",
            web::post().to(move || {
                let body = output.clone();
                async move {
                    HttpResponse::Ok().json(json!({
                        "choices": [{
                            "message": {
                                "content": body
                            }
                        }]
                    }))
                }
            }),
        )
    })
    .listen(provider_listener)
    .expect("listen provider")
    .run();

    let provider_handle = provider_server.handle();
    let _provider_task = tokio::spawn(provider_server);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read bound app addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
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
            "title": "Scope target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse create note response");
    let note_id = note_body["note_id"].as_str().expect("note id string").to_owned();

    let create_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/scope-model",
                    "base_url": format!("http://{provider_address}/v1/chat/completions"),
                    "timeout_ms": 400,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Scope enforcement",
                    "scope": format!("note:{note_id}"),
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
        .expect("create scope rule request");
    assert_eq!(create_rule.status(), StatusCode::CREATED);

    let create_rule_body: serde_json::Value = create_rule
        .json()
        .await
        .expect("parse create scope rule response");
    let rule_id = uuid::Uuid::parse_str(
        create_rule_body["rule"]["id"]
            .as_str()
            .expect("scope rule id"),
    )
    .expect("parse scope rule id");

    let patch_note = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello scope"}],
            "idempotency_key": "provider-scope-guard"
        }))
        .send()
        .await
        .expect("patch note request");
    assert_eq!(patch_note.status(), StatusCode::OK);

    let run: (String, serde_json::Value) = sqlx::query_as(
        "SELECT status, result_json
         FROM automation_runs
         WHERE rule_id = $1
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(rule_id)
    .fetch_one(&pool)
    .await
    .expect("query scope run");

    assert_eq!(run.0, "succeeded");
    assert_eq!(
        run.1["operation_report"]["parsed_operations"]
            .as_array()
            .map_or(0, Vec::len),
        1
    );
    assert_eq!(
        run.1["operation_report"]["rejected_operations"]
            .as_array()
            .map_or(0, Vec::len),
        1
    );
    assert_eq!(
        run.1["operation_report"]["rejected_operations"][0]["reason"],
        json!("SCOPE_VIOLATION")
    );

    server_handle.stop(true).await;
    provider_handle.stop(true).await;
}
