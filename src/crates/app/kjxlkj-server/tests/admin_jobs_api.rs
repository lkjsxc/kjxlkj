use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn export_and_backup_job_lifecycle_with_forbidden_path_checks() {
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
        &format!("owner-jobs-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-jobs-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let viewer = repos::users::create_user(
        &pool,
        &format!("viewer-jobs-{token}@example.com"),
        "Viewer",
        &viewer_hash,
        "viewer",
    )
    .await
    .expect("create viewer user");

    repos::workspaces::upsert_workspace_member(&pool, workspace.id, viewer.id, "viewer")
        .await
        .expect("upsert viewer workspace membership");

    let owner_session = new_session_id();
    let owner_csrf = new_csrf_token();
    repos::auth::create_session(&pool, owner_session, owner.id, &owner_csrf, session_expiry(7))
        .await
        .expect("create owner session");

    let viewer_session = new_session_id();
    let viewer_csrf = new_csrf_token();
    repos::auth::create_session(&pool, viewer_session, viewer.id, &viewer_csrf, session_expiry(7))
        .await
        .expect("create viewer session");

    let _ = repos::notes::create_note(
        &pool,
        owner.id,
        repos::notes::CreateNoteInput {
            workspace_id: workspace.id,
            project_id: None,
            title: "Export note".to_owned(),
            note_kind: "markdown".to_owned(),
            access_scope: "workspace".to_owned(),
            markdown: "Export body".to_owned(),
        },
    )
    .await
    .expect("create export note");

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

    let export_job = client
        .post(format!("{base_url}/api/admin/export/markdown"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({ "workspace_id": workspace.id }))
        .send()
        .await
        .expect("post markdown export job request");
    assert_eq!(export_job.status(), StatusCode::ACCEPTED);

    let export_body: serde_json::Value = export_job.json().await.expect("parse export job response");
    let export_job_id = export_body["job"]["id"]
        .as_str()
        .expect("export job id string")
        .to_owned();
    assert_eq!(export_body["job"]["status"], json!("succeeded"));

    let export_artifact_path = export_body["job"]["artifact_path"]
        .as_str()
        .expect("export artifact path string")
        .to_owned();
    assert!(std::path::Path::new(&export_artifact_path).exists());

    let export_status = client
        .get(format!("{base_url}/api/admin/export/{export_job_id}"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .send()
        .await
        .expect("get export job status request");
    assert_eq!(export_status.status(), StatusCode::OK);

    let backup_job = client
        .post(format!("{base_url}/api/admin/backup/sql"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .send()
        .await
        .expect("post sql backup job request");
    assert_eq!(backup_job.status(), StatusCode::ACCEPTED);

    let backup_body: serde_json::Value = backup_job.json().await.expect("parse backup job response");
    assert_eq!(backup_body["job"]["status"], json!("succeeded"));

    let backup_artifact_path = backup_body["job"]["artifact_path"]
        .as_str()
        .expect("backup artifact path string")
        .to_owned();
    assert!(std::path::Path::new(&backup_artifact_path).exists());

    let forbidden_export = client
        .post(format!("{base_url}/api/admin/export/markdown"))
        .header("Cookie", format!("kjxlkj_session={viewer_session}"))
        .header("x-csrf-token", &viewer_csrf)
        .json(&json!({ "workspace_id": workspace.id }))
        .send()
        .await
        .expect("viewer export request");
    assert_eq!(forbidden_export.status(), StatusCode::FORBIDDEN);

    let forbidden_backup = client
        .post(format!("{base_url}/api/admin/backup/sql"))
        .header("Cookie", format!("kjxlkj_session={viewer_session}"))
        .header("x-csrf-token", &viewer_csrf)
        .send()
        .await
        .expect("viewer backup request");
    assert_eq!(forbidden_backup.status(), StatusCode::FORBIDDEN);

    server_handle.stop(true).await;
}
