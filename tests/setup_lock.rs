mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn setup_flow_locks_after_first_admin() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let get_before =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(get_before.status(), StatusCode::OK);

    let login_before =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login_before.status(), StatusCode::FOUND);
    assert_eq!(
        login_before
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let setup_post = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup_post.status(), StatusCode::CREATED);

    let get_after =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(get_after.status(), StatusCode::NOT_FOUND);

    let post_after = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "other"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(post_after.status(), StatusCode::NOT_FOUND);

    let login_after =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login_after.status(), StatusCode::OK);
}
