use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use super::web_test_support::make_web_state;

#[actix_web::test]
async fn get_home_redirects_to_setup_before_setup_completion() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let response = test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(response.status(), StatusCode::FOUND);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );
}

#[actix_web::test]
async fn get_setup_returns_full_html_form() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let response =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );

    let body = test::read_body(response).await;
    let text = std::str::from_utf8(&body).expect("utf8 body");
    assert!(text.contains("<!doctype html>"));
    assert!(text.contains("<html lang=\"en\">"));
    assert!(text.contains("<title>Initial setup</title>"));
    assert!(text.contains("<h1>Set up first admin account</h1>"));
    assert!(text.contains("<form method=\"post\" action=\"/setup\">"));
    assert!(text.contains("<label for=\"username\">Username</label>"));
    assert!(text.contains(
        "<input id=\"username\" name=\"username\" type=\"text\" autocomplete=\"username\" value=\"\" />"
    ));
    assert!(text.contains("<label for=\"password\">Password</label>"));
    assert!(text.contains(
        "<input id=\"password\" name=\"password\" type=\"password\" autocomplete=\"new-password\" />"
    ));
    assert!(text.contains("<button type=\"submit\">Create admin account</button>"));
}
