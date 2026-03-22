mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn admin_open_and_preview_return_expected_fragments() {
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
            .insert_header((header::COOKIE, cookie.clone()))
            .insert_header(("HX-Request", "true"))
            .to_request(),
    )
    .await;
    assert_eq!(open.status(), StatusCode::OK);
    let open_text = String::from_utf8(test::read_body(open).await.to_vec()).expect("utf8");
    assert!(open_text.contains("id=\"admin-editor-pane\""));
    assert!(open_text.contains("name=\"last_known_revision\""));
    assert!(open_text.contains("id=\"admin-preview-pane\""));
    assert!(open_text.contains("hx-swap-oob=\"outerHTML\""));

    let preview = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/preview")
            .insert_header((header::COOKIE, cookie))
            .set_form([("slug", "first-post"), ("body", "# Previewed")])
            .to_request(),
    )
    .await;
    assert_eq!(preview.status(), StatusCode::OK);
    let preview_text = String::from_utf8(test::read_body(preview).await.to_vec()).expect("utf8");
    assert!(preview_text.contains("id=\"admin-preview-pane\""));
    assert!(preview_text.contains("Previewed"));
}

#[actix_web::test]
async fn admin_save_emits_revision_and_conflict_fragments() {
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
    let cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();

    let save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "first-post"),
                ("title", ""),
                ("body", "# First update"),
                ("last_known_revision", ""),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(save.status(), StatusCode::OK);
    let trigger = save
        .headers()
        .get("HX-Trigger")
        .and_then(|value| value.to_str().ok())
        .expect("hx-trigger");
    assert!(trigger.contains("admin-save"));
    let save_text = String::from_utf8(test::read_body(save).await.to_vec()).expect("utf8");
    assert!(save_text.contains("id=\"admin-status-banner\""));
    assert!(save_text.contains("id=\"last_known_revision\""));
    assert!(save_text.contains("data-conflict=\"false\""));

    let conflict = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header((header::COOKIE, cookie))
            .set_form([
                ("slug", "first-post"),
                ("title", ""),
                ("body", "# First overwrite"),
                ("last_known_revision", "stale-token"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(conflict.status(), StatusCode::OK);
    let conflict_trigger = conflict
        .headers()
        .get("HX-Trigger")
        .and_then(|value| value.to_str().ok())
        .expect("hx-trigger");
    assert!(conflict_trigger.contains("admin-save-conflict"));
    let conflict_text = String::from_utf8(test::read_body(conflict).await.to_vec()).expect("utf8");
    assert!(conflict_text.contains("data-conflict=\"true\""));
    assert!(conflict_text.contains("data-telemetry=\"admin-save-conflict\""));
}
