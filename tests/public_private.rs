mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn public_routes_hide_private_content_for_logged_out_users() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("public-post", false, "# Public");
    content.insert_article("private-post", true, "# Private");

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

    let home = test::call_and_read_body(&app, test::TestRequest::get().uri("/").to_request()).await;
    let home_text = String::from_utf8(home.to_vec()).expect("utf8");
    assert!(home_text.contains("<main id=\"home-page\">"));
    assert!(home_text.contains("<section id=\"home-article-list\">"));
    assert!(home_text.contains("href=\"/article/public-post\""));
    assert!(home_text.contains("public-post"));
    assert!(!home_text.contains("private-post"));

    let public_article = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/public-post")
            .to_request(),
    )
    .await;
    assert_eq!(public_article.status(), StatusCode::OK);

    let private_article = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/private-post")
            .to_request(),
    )
    .await;
    assert_eq!(private_article.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn logged_in_admin_can_view_private_content() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("public-post", false, "# Public");
    content.insert_article("private-post", true, "# Private");

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

    let home = test::call_and_read_body(
        &app,
        test::TestRequest::get()
            .uri("/")
            .insert_header((header::COOKIE, session_cookie.clone()))
            .to_request(),
    )
    .await;
    let home_text = String::from_utf8(home.to_vec()).expect("utf8");
    assert!(home_text.contains("<main id=\"home-page\">"));
    assert!(home_text.contains("<section id=\"home-article-list\">"));
    assert!(home_text.contains("admin-affordance"));
    assert!(home_text.contains("public-post"));
    assert!(home_text.contains("private-post"));

    let private_article = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/private-post")
            .insert_header((header::COOKIE, session_cookie))
            .to_request(),
    )
    .await;
    assert_eq!(private_article.status(), StatusCode::OK);
}
