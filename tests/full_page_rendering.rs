mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn full_page_routes_render_expected_contracts_across_setup_and_auth_states() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("public-post", false, "# Public");
    content.insert_article("private-post", true, "# Private");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let home_before_setup =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(home_before_setup.status(), StatusCode::FOUND);
    assert_eq!(
        home_before_setup
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let setup_page =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(setup_page.status(), StatusCode::OK);
    assert_eq!(
        setup_page
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let setup_text = String::from_utf8(test::read_body(setup_page).await.to_vec()).expect("utf8");
    assert!(setup_text.contains("<!doctype html>"));
    assert!(setup_text.contains("<main id=\"setup-page\">"));
    assert!(setup_text.contains("<form id=\"setup-form\" method=\"post\" action=\"/setup\">"));

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        setup
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );

    let setup_after =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(setup_after.status(), StatusCode::NOT_FOUND);

    let login_page =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login_page.status(), StatusCode::OK);
    let login_page_text =
        String::from_utf8(test::read_body(login_page).await.to_vec()).expect("utf8");
    assert!(login_page_text.contains("<!doctype html>"));
    assert!(login_page_text.contains("<main id=\"login-page\">"));
    assert!(login_page_text.contains("<form id=\"login-form\" method=\"post\" action=\"/login\">"));

    let home_public =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(home_public.status(), StatusCode::OK);
    let home_public_text =
        String::from_utf8(test::read_body(home_public).await.to_vec()).expect("utf8");
    assert!(home_public_text.contains("<!doctype html>"));
    assert!(home_public_text.contains("id=\"app-shell\""));
    assert!(home_public_text.contains("id=\"app-nav\""));
    assert!(home_public_text.contains("id=\"app-topbar\""));
    assert!(home_public_text.contains("href=\"/search\""));
    assert!(home_public_text.contains("<main id=\"home-page\">"));
    assert!(home_public_text.contains("<section id=\"home-article-list\">"));
    assert!(home_public_text.contains("public-post"));
    assert!(!home_public_text.contains("private-post"));
    assert!(!home_public_text.contains("admin-affordance"));

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        login
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/admin")
    );
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
            .insert_header((header::COOKIE, cookie.clone()))
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

    let home_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(home_admin.status(), StatusCode::OK);
    let home_admin_text =
        String::from_utf8(test::read_body(home_admin).await.to_vec()).expect("utf8");
    assert!(home_admin_text.contains("public-post"));
    assert!(home_admin_text.contains("private-post"));
    assert!(home_admin_text.contains("admin-affordance"));
    assert!(home_admin_text.contains("href=\"/admin/settings\""));
    assert!(home_admin_text.contains("href=\"/admin/trash\""));

    let admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(admin.status(), StatusCode::OK);
    let admin_text = String::from_utf8(test::read_body(admin).await.to_vec()).expect("utf8");
    assert!(admin_text.contains("<!doctype html>"));
    assert!(admin_text.contains("<main id=\"admin-page\">"));
    assert!(admin_text.contains("<section id=\"admin-article-list\">"));
    assert!(admin_text.contains("<section id=\"admin-editor-pane\""));
    assert!(admin_text.contains("<section id=\"admin-preview-pane\"></section>"));
    assert!(admin_text.contains("id=\"admin-status-banner\""));
    assert!(admin_text.contains("id=\"admin-conflict-banner\""));
}
