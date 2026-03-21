use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use super::web_test_support::make_web_state;

#[actix_web::test]
async fn post_setup_invalid_payload_returns_deterministic_validation_message() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let first_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "   "), ("password", "   ")])
            .to_request(),
    )
    .await;
    assert_eq!(first_response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        first_response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let first_body = test::read_body(first_response).await;
    let first_text = String::from_utf8(first_body.to_vec()).expect("utf8 body");
    let username_index = first_text
        .find("username is required")
        .expect("username validation message");
    let password_index = first_text
        .find("password is required")
        .expect("password validation message");
    assert!(username_index < password_index);
    assert!(first_text.contains("Unable to complete setup:"));

    let second_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "   "), ("password", "   ")])
            .to_request(),
    )
    .await;
    assert_eq!(second_response.status(), StatusCode::BAD_REQUEST);
    let second_body = test::read_body(second_response).await;
    let second_text = String::from_utf8(second_body.to_vec()).expect("utf8 body");
    assert_eq!(second_text, first_text);

    let missing_password_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "  <admin>  "), ("password", "   ")])
            .to_request(),
    )
    .await;
    assert_eq!(missing_password_response.status(), StatusCode::BAD_REQUEST);
    let missing_password_body = test::read_body(missing_password_response).await;
    let missing_password_text = String::from_utf8(missing_password_body.to_vec()).expect("utf8");
    assert!(!missing_password_text.contains("username is required"));
    assert!(missing_password_text.contains("password is required"));
    assert!(missing_password_text.contains("value=\"&lt;admin&gt;\""));
}

#[actix_web::test]
async fn post_setup_valid_payload_creates_admin_and_redirects_to_login() {
    let (state, admin_store, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let setup_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "  admin  "), ("password", "  s3cret  ")])
            .to_request(),
    )
    .await;
    assert_eq!(setup_response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        setup_response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );

    let admin = admin_store.admin().expect("admin user should be created");
    assert_eq!(admin.username, "admin");
    assert_ne!(admin.password_hash, "s3cret");
    assert!(admin.password_hash.starts_with("$argon2"));

    let login_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(login_response.status(), StatusCode::OK);
    assert!(login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .is_some());
}
