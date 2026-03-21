mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{body, test, web, App};
use kjxlkj::web::router::configure_routes;
use uuid::Uuid;

use web_test_support::make_web_state;

#[actix_web::test]
async fn public_and_admin_visibility_is_enforced() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("public-post", false, "# Public");
    content.insert_article("private-post", true, "# Private");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let public_home_before_setup =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(public_home_before_setup.status(), StatusCode::FOUND);
    assert_eq!(
        public_home_before_setup
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "s3cret")])
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

    let public_home =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(public_home.status(), StatusCode::OK);
    let public_home_body = body::to_bytes(public_home.into_body())
        .await
        .expect("public home body");
    let public_home_text = std::str::from_utf8(&public_home_body).expect("utf8 body");
    assert!(public_home_text.contains("public-post"));
    assert!(!public_home_text.contains("private-post"));

    let private_for_guest = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/private-post")
            .to_request(),
    )
    .await;
    assert_eq!(private_for_guest.status(), StatusCode::NOT_FOUND);

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);
    let session_cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("session cookie");
    let session_id = parse_session_id(session_cookie);

    let admin_home = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/")
            .insert_header((header::COOKIE, format!("session_id={session_id}")))
            .to_request(),
    )
    .await;
    assert_eq!(admin_home.status(), StatusCode::OK);
    let admin_home_body = body::to_bytes(admin_home.into_body())
        .await
        .expect("admin home body");
    let admin_home_text = std::str::from_utf8(&admin_home_body).expect("utf8 body");
    assert!(admin_home_text.contains("public-post"));
    assert!(admin_home_text.contains("private-post"));

    let private_for_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/private-post")
            .insert_header((header::COOKIE, format!("session_id={session_id}")))
            .to_request(),
    )
    .await;
    assert_eq!(private_for_admin.status(), StatusCode::OK);
}

fn parse_session_id(cookie: &str) -> Uuid {
    let value = cookie
        .split(';')
        .find_map(|part| part.trim().strip_prefix("session_id="))
        .expect("session id cookie value");
    Uuid::parse_str(value).expect("valid session uuid")
}
