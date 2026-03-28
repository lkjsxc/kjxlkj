//! Integration tests for the note service

use actix_web::{test, App};
use kjxlkj::web::handlers::health;

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(App::new().service(health::healthz)).await;
    let req = test::TestRequest::get().uri("/healthz").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "ok");
}
