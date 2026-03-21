mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn login_creates_session_cookie_and_logout_clears_it() {
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
            .set_form([("username", "admin"), ("password", "correct-password")])
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

    let bad_login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "wrong-password")])
            .to_request(),
    )
    .await;
    assert_eq!(bad_login.status(), StatusCode::UNAUTHORIZED);

    let good_login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "correct-password")])
            .to_request(),
    )
    .await;
    assert_eq!(good_login.status(), StatusCode::OK);

    let cookie_header = good_login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie");
    assert!(cookie_header.contains("session_id="));
    assert!(cookie_header.contains("HttpOnly"));
    assert!(cookie_header.contains("Secure"));

    let session_id = cookie_header
        .split(';')
        .next()
        .and_then(|kv| kv.split_once('='))
        .map(|(_, value)| value.to_owned())
        .expect("session id");
    let cookie_pair = format!("session_id={session_id}");

    let admin_authenticated = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, cookie_pair.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(admin_authenticated.status(), StatusCode::OK);

    let logout = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/logout")
            .insert_header((header::COOKIE, cookie_pair.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(logout.status(), StatusCode::NO_CONTENT);

    let clear_cookie = logout
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("clear cookie");
    assert!(clear_cookie.contains("session_id="));
    assert!(clear_cookie.contains("Max-Age=0"));

    let admin_after_logout = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, cookie_pair))
            .to_request(),
    )
    .await;
    assert_eq!(admin_after_logout.status(), StatusCode::FOUND);
    assert_eq!(
        admin_after_logout
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );
}
