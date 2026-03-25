//! Integration tests for the record service

use actix_web::{test, web, App};
use kjxlkj::config::Config;
use kjxlkj::storage::FilesystemStorage;
use kjxlkj::web::handlers::{health, records};
use std::sync::Arc;
use tempfile::TempDir;

async fn setup_app() -> (
    impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    TempDir,
) {
    let dir = TempDir::new().unwrap();
    let storage = FilesystemStorage::new(dir.path().to_path_buf())
        .await
        .unwrap();
    let storage = Arc::new(storage);

    let config = Config {
        bind_host: "127.0.0.1".to_string(),
        bind_port: 8080,
        data_root: dir.path().to_string_lossy().to_string(),
        database_url: "unused".to_string(),
        admin_token: "test-token".to_string(),
        session_timeout_minutes: 60,
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(storage.clone()))
            .service(health::healthz)
            .service(records::list)
            .service(records::fetch)
            .service(records::upsert)
            .service(records::remove),
    )
    .await;

    (app, dir)
}

#[actix_web::test]
async fn test_health_endpoint() {
    let (app, _dir) = setup_app().await;
    let req = test::TestRequest::get().uri("/healthz").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "ok");
}

#[actix_web::test]
async fn test_list_empty() {
    let (app, _dir) = setup_app().await;
    let req = test::TestRequest::get().uri("/v1/records").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn test_create_record() {
    let (app, _dir) = setup_app().await;

    let req = test::TestRequest::put()
        .uri("/v1/records/test-record")
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"Test","body":"Body","tags":["tag1"]}"#)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "test-record");
    assert_eq!(body["title"], "Test");
    assert_eq!(body["revision"], 1);
}

#[actix_web::test]
async fn test_update_record() {
    let (app, _dir) = setup_app().await;

    let create_req = test::TestRequest::put()
        .uri("/v1/records/update-test")
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"V1","body":"","tags":[]}"#)
        .to_request();
    test::call_service(&app, create_req).await;

    let update_req = test::TestRequest::put()
        .uri("/v1/records/update-test")
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"V2","body":"updated","tags":["new"]}"#)
        .to_request();
    let resp = test::call_service(&app, update_req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "V2");
    assert_eq!(body["revision"], 2);
}

#[actix_web::test]
async fn test_fetch_record() {
    let (app, _dir) = setup_app().await;

    let create_req = test::TestRequest::put()
        .uri("/v1/records/fetch-test")
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"Fetch","body":"me","tags":[]}"#)
        .to_request();
    test::call_service(&app, create_req).await;

    let fetch_req = test::TestRequest::get()
        .uri("/v1/records/fetch-test")
        .to_request();
    let resp = test::call_service(&app, fetch_req).await;

    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "fetch-test");
    assert_eq!(body["title"], "Fetch");
}

#[actix_web::test]
async fn test_fetch_not_found() {
    let (app, _dir) = setup_app().await;
    let req = test::TestRequest::get()
        .uri("/v1/records/nonexistent")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_record() {
    let (app, _dir) = setup_app().await;

    let create_req = test::TestRequest::put()
        .uri("/v1/records/delete-test")
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"Del","body":"","tags":[]}"#)
        .to_request();
    test::call_service(&app, create_req).await;

    let delete_req = test::TestRequest::delete()
        .uri("/v1/records/delete-test")
        .insert_header(("x-admin-token", "test-token"))
        .to_request();
    let resp = test::call_service(&app, delete_req).await;

    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn test_unauthorized_write() {
    let (app, _dir) = setup_app().await;

    let req = test::TestRequest::put()
        .uri("/v1/records/unauth")
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"Test","body":"","tags":[]}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_invalid_id() {
    let (app, _dir) = setup_app().await;

    let req = test::TestRequest::put()
        .uri("/v1/records/AB") // Too short and uppercase
        .insert_header(("x-admin-token", "test-token"))
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"title":"Test","body":"","tags":[]}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
