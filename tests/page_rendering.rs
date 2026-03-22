mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn login_get_renders_full_page_after_setup_and_redirects_when_session_exists() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup.status(), StatusCode::SEE_OTHER);

    let login_page =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login_page.status(), StatusCode::OK);
    assert_eq!(
        login_page
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let login_body = test::read_body(login_page).await;
    let login_text = String::from_utf8(login_body.to_vec()).expect("utf8");
    assert!(login_text.contains("<!doctype html>"));
    assert!(login_text.contains("<main id=\"login-page\">"));
    assert!(login_text.contains("<form id=\"login-form\" method=\"post\" action=\"/login\">"));
    assert!(login_text.contains("<section id=\"login-errors\" aria-live=\"polite\"></section>"));
    assert!(login_text.contains("name=\"username\""));
    assert!(login_text.contains("name=\"password\""));

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    let cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();

    let login_with_session = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/login")
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(login_with_session.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        login_with_session
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/admin")
    );
}

#[actix_web::test]
async fn admin_get_renders_full_shell_html_for_authenticated_session() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("first-post", false, "# First");
    content.insert_article("second-private", true, "# Secret");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup.status(), StatusCode::SEE_OTHER);

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    let cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();

    let admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(admin.status(), StatusCode::OK);
    assert_eq!(
        admin
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let admin_body = test::read_body(admin).await;
    let admin_text = String::from_utf8(admin_body.to_vec()).expect("utf8");
    assert!(admin_text.contains("<!doctype html>"));
    assert!(admin_text.contains("<main id=\"admin-page\">"));
    assert!(admin_text.contains("<section id=\"admin-article-list\">"));
    assert!(admin_text.contains("<section id=\"admin-editor-pane\""));
    assert!(admin_text.contains("<section id=\"admin-preview-pane\"></section>"));
    assert!(
        admin_text.contains("<section id=\"admin-status-banner\" aria-live=\"polite\"></section>")
    );
    assert!(admin_text.contains(
        "<section id=\"admin-conflict-banner\" role=\"alert\" aria-live=\"assertive\" data-conflict=\"false\"></section>"
    ));
    assert!(admin_text
        .contains("<form id=\"admin-editor-form\" method=\"post\" action=\"/admin/save\""));
    assert!(admin_text.contains("name=\"slug\""));
    assert!(admin_text.contains("name=\"title\""));
    assert!(admin_text.contains("name=\"body\""));
    assert!(admin_text.contains("name=\"private\""));
    assert!(admin_text.contains("name=\"last_known_revision\""));
    assert!(admin_text.contains("/static/admin-runtime-core.js"));
}
