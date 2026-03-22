mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn admin_routes_redirect_when_not_authenticated() {
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
    assert_eq!(
        setup
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );

    let admin_unauth =
        test::call_service(&app, test::TestRequest::get().uri("/admin").to_request()).await;
    assert_eq!(admin_unauth.status(), StatusCode::FOUND);
    assert_eq!(
        admin_unauth
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );

    let create_unauth = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .set_form([("slug", "x"), ("body", "body")])
            .to_request(),
    )
    .await;
    assert_eq!(create_unauth.status(), StatusCode::FOUND);
    assert_eq!(
        create_unauth
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );

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
    let session_cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie key-value")
        .to_owned();

    let admin_auth = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, session_cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(admin_auth.status(), StatusCode::OK);
    assert_eq!(
        admin_auth
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let admin_body = test::read_body(admin_auth).await;
    let admin_text = String::from_utf8(admin_body.to_vec()).expect("utf8");
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

    let create_auth = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .insert_header((header::COOKIE, session_cookie))
            .set_form([("slug", "created-article"), ("body", "body")])
            .to_request(),
    )
    .await;
    assert_eq!(create_auth.status(), StatusCode::CREATED);
}

#[actix_web::test]
async fn admin_redirects_to_setup_when_no_admin_exists() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let admin = test::call_service(&app, test::TestRequest::get().uri("/admin").to_request()).await;
    assert_eq!(admin.status(), StatusCode::FOUND);
    assert_eq!(
        admin
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let create = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .set_form([("slug", "x"), ("body", "body")])
            .to_request(),
    )
    .await;
    assert_eq!(create.status(), StatusCode::FOUND);
    assert_eq!(
        create
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let login = test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login.status(), StatusCode::FOUND);
    assert_eq!(
        login
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );
}
