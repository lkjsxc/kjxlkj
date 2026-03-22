use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use super::web_test_support::make_web_state;

#[actix_web::test]
async fn setup_lock_is_enforced_after_admin_creation() {
    let (state, admin_store, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let first_setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(first_setup.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        first_setup
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );
    let first_admin = admin_store.admin().expect("admin should exist");

    let get_setup_after =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(get_setup_after.status(), StatusCode::NOT_FOUND);

    let post_setup_after = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "other-admin"), ("password", "other-secret")])
            .to_request(),
    )
    .await;
    assert_eq!(post_setup_after.status(), StatusCode::NOT_FOUND);

    let admin_after_lock = admin_store.admin().expect("admin should still exist");
    assert_eq!(admin_after_lock.username, first_admin.username);
    assert_eq!(admin_after_lock.password_hash, first_admin.password_hash);

    let login = test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login.status(), StatusCode::OK);
    assert_eq!(
        login
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let login_body = test::read_body(login).await;
    let login_text = String::from_utf8(login_body.to_vec()).expect("utf8 body");
    assert!(login_text.contains("<main id=\"login-page\">"));
    assert!(login_text.contains("<form id=\"login-form\" method=\"post\" action=\"/login\">"));
    assert!(login_text.contains("<section id=\"login-errors\" aria-live=\"polite\"></section>"));
    assert!(login_text.contains("<!doctype html>"));

    let home = test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(home.status(), StatusCode::OK);
    assert_eq!(
        home.headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let home_body = test::read_body(home).await;
    let home_text = String::from_utf8(home_body.to_vec()).expect("utf8 body");
    assert!(home_text.contains("<main id=\"home-page\">"));
    assert!(home_text.contains("<section id=\"home-article-list\">"));
}
