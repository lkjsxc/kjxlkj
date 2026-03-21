mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;
use uuid::Uuid;

use web_test_support::make_web_state;

#[actix_web::test]
async fn login_logout_session_lifecycle_behaves_as_expected() {
    let (state, _, session_store, _) = make_web_state();

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
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup.status(), StatusCode::CREATED);

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(login.status(), StatusCode::OK);

    let set_cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("session cookie header");
    assert!(set_cookie.contains("session_id="));
    assert!(set_cookie.contains("HttpOnly"));
    assert!(set_cookie.contains("Secure"));
    assert!(set_cookie.contains("Path=/"));

    let session_id = parse_session_id(set_cookie);
    assert!(session_store.has_session(session_id));

    let logout = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/logout")
            .insert_header((header::COOKIE, format!("session_id={session_id}")))
            .to_request(),
    )
    .await;
    assert_eq!(logout.status(), StatusCode::NO_CONTENT);
    let clear_cookie = logout
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("clear session cookie header");
    assert!(clear_cookie.contains("session_id="));
    assert!(clear_cookie.contains("Max-Age=0"));
    assert!(!session_store.has_session(session_id));
}

fn parse_session_id(cookie: &str) -> Uuid {
    let value = cookie
        .split(';')
        .find_map(|part| part.trim().strip_prefix("session_id="))
        .expect("session id cookie value");
    Uuid::parse_str(value).expect("valid session uuid")
}
