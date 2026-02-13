use actix_web::{web, App, HttpServer};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn saved_view_lifecycle_and_role_denial() {
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
        &format!("owner-view-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-view-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner and workspace");

    let viewer = repos::users::create_user(
        &pool,
        &format!("viewer-view-{token}@example.com"),
        "Viewer",
        &viewer_hash,
        "viewer",
    )
    .await
    .expect("create viewer user");

    repos::workspaces::upsert_workspace_member(&pool, workspace.id, viewer.id, "viewer")
        .await
        .expect("add viewer member");

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

    let create_response = client
        .post(format!("{base_url}/api/views"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "workspace_id": workspace.id,
            "query_json": {"q": "alpha", "scope": "workspace"},
            "sort": "updated_desc",
            "filters": {"tags": ["ops"]}
        }))
        .send()
        .await
        .expect("create view request");
    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created_body: serde_json::Value = create_response.json().await.expect("parse create response json");
    let view_id = created_body["view"]["id"]
        .as_str()
        .expect("view id string")
        .to_owned();

    let list_response = client
        .get(format!("{base_url}/api/views"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("list views request");
    assert_eq!(list_response.status(), StatusCode::OK);

    let list_body: serde_json::Value = list_response.json().await.expect("parse list response json");
    assert_eq!(list_body["views"].as_array().map_or(0, Vec::len), 1);

    let patch_response = client
        .patch(format!("{base_url}/api/views/{view_id}"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .json(&json!({
            "sort": "title_asc"
        }))
        .send()
        .await
        .expect("patch view request");
    assert_eq!(patch_response.status(), StatusCode::OK);

    let patched_body: serde_json::Value = patch_response.json().await.expect("parse patch response json");
    assert_eq!(patched_body["view"]["sort"], json!("title_asc"));

    let viewer_patch = client
        .patch(format!("{base_url}/api/views/{view_id}"))
        .header("Cookie", format!("kjxlkj_session={viewer_session}"))
        .header("x-csrf-token", &viewer_csrf)
        .json(&json!({
            "sort": "updated_asc"
        }))
        .send()
        .await
        .expect("viewer patch request");
    assert_eq!(viewer_patch.status(), StatusCode::FORBIDDEN);

    let delete_response = client
        .delete(format!("{base_url}/api/views/{view_id}"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .header("x-csrf-token", &owner_csrf)
        .send()
        .await
        .expect("delete view request");
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let list_after_delete = client
        .get(format!("{base_url}/api/views"))
        .header("Cookie", format!("kjxlkj_session={owner_session}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("list views after delete request");
    assert_eq!(list_after_delete.status(), StatusCode::OK);

    let list_after_delete_body: serde_json::Value = list_after_delete
        .json()
        .await
        .expect("parse list after delete response json");
    assert_eq!(list_after_delete_body["views"].as_array().map_or(0, Vec::len), 0);

    server_handle.stop(true).await;
}
