mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn htmx_open_preview_and_create_return_expected_fragments() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("first-post", false, "# First");

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

    let open = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/open/first-post")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(open.status(), StatusCode::OK);
    let open_body = String::from_utf8(test::read_body(open).await.to_vec()).expect("utf8");
    assert!(open_body.contains("id=\"admin-editor-pane\""));
    assert!(open_body.contains("id=\"admin-preview-pane\""));
    assert!(open_body.contains("hx-swap-oob=\"outerHTML\""));

    let preview = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/preview")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "first-post"),
                ("title", "First"),
                ("body", "# Preview"),
                ("private", "false"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(preview.status(), StatusCode::OK);
    let preview_body = String::from_utf8(test::read_body(preview).await.to_vec()).expect("utf8");
    assert!(preview_body.contains("id=\"admin-preview-pane\""));
    assert!(preview_body.contains("<h1>Preview</h1>"));

    let create = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie))
            .set_form([
                ("slug", "created-post"),
                ("title", "Created"),
                ("body", "# Created"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(create.status(), StatusCode::CREATED);
    let create_body = String::from_utf8(test::read_body(create).await.to_vec()).expect("utf8");
    assert!(create_body.contains("id=\"admin-article-list\""));
    assert!(create_body.contains("id=\"admin-editor-pane\""));
    assert!(create_body.contains("id=\"admin-preview-pane\""));
    assert!(create_body.contains("id=\"admin-status-banner\""));
}
