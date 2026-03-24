use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::http::StatusCode;
use actix_web::{test, web, App};

use kjxlkj::core::auth::DEFAULT_SESSION_TIMEOUT_MINUTES;
use kjxlkj::storage::FsStore;
use kjxlkj::web::{configure_routes, AppState, AuthStore};
use kjxlkj::{core::RecordInput, error::AppError};

use crate::support::TestDatabase;

#[actix_web::test]
async fn record_lifecycle_and_auth_contract() {
    let test_db = TestDatabase::create("record_lifecycle").await;
    let store_root = unique_test_dir("record-lifecycle");
    let state = build_state("test-token", &store_root, &test_db.url)
        .await
        .expect("state");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let health =
        test::call_service(&app, test::TestRequest::get().uri("/healthz").to_request()).await;
    assert_eq!(health.status(), StatusCode::OK);

    let list_initial = test::call_service(
        &app,
        test::TestRequest::get().uri("/v1/records").to_request(),
    )
    .await;
    assert_eq!(list_initial.status(), StatusCode::OK);

    let unauthorized_put = test::call_service(
        &app,
        test::TestRequest::put()
            .uri("/v1/records/demo-note")
            .set_json(RecordInput {
                title: "Demo".to_owned(),
                body: "body".to_owned(),
                tags: vec!["Ops".to_owned(), "ops".to_owned()],
            })
            .to_request(),
    )
    .await;
    assert_eq!(unauthorized_put.status(), StatusCode::UNAUTHORIZED);

    let create = test::call_service(
        &app,
        test::TestRequest::put()
            .uri("/v1/records/demo-note")
            .insert_header(("x-admin-token", "test-token"))
            .set_json(RecordInput {
                title: "Demo".to_owned(),
                body: "v1".to_owned(),
                tags: vec!["Ops".to_owned(), "ops".to_owned(), "qa".to_owned()],
            })
            .to_request(),
    )
    .await;
    assert_eq!(create.status(), StatusCode::CREATED);
    let created: serde_json::Value = test::read_body_json(create).await;
    assert_eq!(created["id"], "demo-note");
    assert_eq!(created["revision"], 1);
    assert_eq!(created["tags"], serde_json::json!(["ops", "qa"]));

    let update = test::call_service(
        &app,
        test::TestRequest::put()
            .uri("/v1/records/demo-note")
            .insert_header(("x-admin-token", "test-token"))
            .set_json(RecordInput {
                title: "Demo Updated".to_owned(),
                body: "v2".to_owned(),
                tags: vec!["qa".to_owned()],
            })
            .to_request(),
    )
    .await;
    assert_eq!(update.status(), StatusCode::OK);
    let updated: serde_json::Value = test::read_body_json(update).await;
    assert_eq!(updated["revision"], 2);

    let fetch = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/v1/records/demo-note")
            .to_request(),
    )
    .await;
    assert_eq!(fetch.status(), StatusCode::OK);

    let delete = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri("/v1/records/demo-note")
            .insert_header(("x-admin-token", "test-token"))
            .to_request(),
    )
    .await;
    assert_eq!(delete.status(), StatusCode::NO_CONTENT);

    let missing = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/v1/records/demo-note")
            .to_request(),
    )
    .await;
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    drop(app);
    let _ = tokio::fs::remove_dir_all(store_root).await;
    test_db.drop().await;
}

fn unique_test_dir(suite: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    PathBuf::from(format!("tmp/tests-{suite}-{nanos}"))
}

async fn build_state(
    token: &str,
    store_root: &PathBuf,
    db_url: &str,
) -> Result<AppState, AppError> {
    tokio::fs::create_dir_all(store_root).await.expect("mkdir");
    let store = FsStore::new(store_root.clone());
    store.ensure_ready().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(db_url)
        .await?;
    let auth_store = AuthStore::new(pool);
    auth_store.ensure_ready().await?;
    Ok(AppState {
        admin_token: token.to_owned(),
        store,
        auth_store,
        session_timeout_minutes: DEFAULT_SESSION_TIMEOUT_MINUTES,
    })
}
