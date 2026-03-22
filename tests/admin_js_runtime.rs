mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn admin_shell_includes_runtime_assets_and_unsaved_indicator() {
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
    assert!(admin_text.contains("id=\"admin-unsaved-indicator\""));
    assert!(admin_text.contains("data-unsaved=\"false\""));
    assert!(admin_text.contains("/static/admin-runtime-core.js"));
    assert!(admin_text.contains("/static/admin-runtime-autosave.js"));
    assert!(admin_text.contains("/static/admin-runtime-shortcuts.js"));
    assert!(admin_text.contains("id=\"app-shell\""));
    assert!(admin_text.contains("id=\"app-nav\""));
    assert!(admin_text.contains("id=\"app-topbar\""));
    assert!(admin_text.contains("id=\"app-nav-toggle\""));
    assert!(admin_text.contains("/static/app-shell.js"));
    assert!(admin_text.contains("id=\"admin-quick-open\""));
    assert!(admin_text.contains("id=\"admin-create-panel\""));
    assert!(admin_text.contains("id=\"admin-create-slug\""));
    assert!(admin_text.contains("data-admin-open=\"true\""));
}

#[actix_web::test]
async fn admin_runtime_assets_are_served() {
    let (state, _, _, _) = make_web_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let core = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/admin-runtime-core.js")
            .to_request(),
    )
    .await;
    assert_eq!(core.status(), StatusCode::OK);
    assert_eq!(
        core.headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/javascript; charset=utf-8")
    );
    let core_text = String::from_utf8(test::read_body(core).await.to_vec()).expect("utf8");
    assert!(core_text.contains("AdminRuntimeShared"));

    let shell_js = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/app-shell.js")
            .to_request(),
    )
    .await;
    assert_eq!(shell_js.status(), StatusCode::OK);
    let shell_js_text = String::from_utf8(test::read_body(shell_js).await.to_vec()).expect("utf8");
    assert!(shell_js_text.contains("app-nav-toggle"));

    let css = test::call_service(
        &app,
        test::TestRequest::get().uri("/static/app.css").to_request(),
    )
    .await;
    assert_eq!(css.status(), StatusCode::OK);
    let css_text = String::from_utf8(test::read_body(css).await.to_vec()).expect("utf8");
    assert!(css_text.contains("#app-shell"));

    let autosave = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/admin-runtime-autosave.js")
            .to_request(),
    )
    .await;
    assert_eq!(autosave.status(), StatusCode::OK);
    let autosave_text = String::from_utf8(test::read_body(autosave).await.to_vec()).expect("utf8");
    assert!(autosave_text.contains("beforeunload"));
    assert!(autosave_text.contains("scheduleAutosave"));

    let shortcuts = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/admin-runtime-shortcuts.js")
            .to_request(),
    )
    .await;
    assert_eq!(shortcuts.status(), StatusCode::OK);
    let shortcuts_text =
        String::from_utf8(test::read_body(shortcuts).await.to_vec()).expect("utf8");
    assert!(shortcuts_text.contains("saveNow(true)"));
    assert!(shortcuts_text.contains("showCreate"));
    assert!(shortcuts_text.contains("focusQuickOpen"));
}
