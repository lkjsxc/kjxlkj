//! Integration tests for the resource service

use axum::body::{to_bytes, Body};
use axum::http::Request;
use axum::routing::get;
use axum::Router;
use kjxlkj::web::handlers::health;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let app = Router::new().route("/healthz", get(health::healthz));
    let req = Request::builder()
        .uri("/healthz")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();

    assert!(resp.status().is_success());
    let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&body[..], b"ok");
}
