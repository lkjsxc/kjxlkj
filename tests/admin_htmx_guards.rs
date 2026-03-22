mod web_test_support;

use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn htmx_admin_requests_require_setup_and_session() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let setup_missing = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/preview")
            .insert_header(("HX-Request", "true"))
            .set_form([
                ("slug", "draft"),
                ("title", "Draft"),
                ("body", "# body"),
                ("private", "false"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(setup_missing.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        setup_missing
            .headers()
            .get("HX-Redirect")
            .and_then(|value| value.to_str().ok()),
        Some("/setup")
    );

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    assert_eq!(setup.status(), StatusCode::SEE_OTHER);

    let session_missing = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/preview")
            .insert_header(("HX-Request", "true"))
            .set_form([
                ("slug", "draft"),
                ("title", "Draft"),
                ("body", "# body"),
                ("private", "false"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(session_missing.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        session_missing
            .headers()
            .get("HX-Redirect")
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );
}
