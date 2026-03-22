mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn autosave_unsaved_guard_shortcuts_and_conflict_ui_contracts_are_embedded_in_runtime_assets()
{
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
    assert!(admin_text.contains("id=\"admin-quick-open\""));
    assert!(admin_text.contains("id=\"admin-create-form\""));
    assert!(admin_text.contains("data-admin-nav-form=\"create\""));
    assert!(admin_text.contains("id=\"admin-preview-button\""));

    let autosave = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/admin-runtime-autosave.js")
            .to_request(),
    )
    .await;
    assert_eq!(autosave.status(), StatusCode::OK);
    let autosave_js = String::from_utf8(test::read_body(autosave).await.to_vec()).expect("utf8");

    assert!(autosave_js.contains("const fieldIds = new Set([\"title\", \"body\", \"private\"])"));
    assert!(autosave_js.contains("window.setTimeout(() =>"));
    assert!(autosave_js.contains("}, 2000);"));
    assert!(autosave_js.contains("document.addEventListener(\n    \"blur\""));
    assert!(autosave_js.contains("if (fieldIds.has(event.target.id) && shared.state.dirty)"));
    assert!(autosave_js.contains("void saveNow();"));
    assert!(autosave_js.contains("window.addEventListener(\"beforeunload\""));
    assert!(autosave_js.contains("void shared.postEditor(savePath, true);"));
    assert!(autosave_js.contains("event.returnValue = \"\";"));
    assert!(autosave_js.contains("return window.confirm(\"Discard unsaved changes?\");"));
    assert!(autosave_js.contains("form.matches(\"[data-admin-nav-form]\")"));
    assert!(autosave_js.contains("const openLink = event.target.closest(\"a[data-admin-open]\");"));
    assert!(autosave_js.contains("window.htmx.ajax(\"GET\", hxGet"));
    assert!(autosave_js
        .contains("const previewButton = event.target.closest(\"#admin-preview-button\")"));
    assert!(autosave_js.contains(
        "const continueButton = event.target.closest('button[data-action=\"continue-editing\"]')"
    ));
    assert!(autosave_js.contains("banner.dataset.conflict = \"false\""));
    assert!(autosave_js.contains("banner.textContent = \"\";"));

    let shortcuts = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/static/admin-runtime-shortcuts.js")
            .to_request(),
    )
    .await;
    assert_eq!(shortcuts.status(), StatusCode::OK);
    let shortcuts_js = String::from_utf8(test::read_body(shortcuts).await.to_vec()).expect("utf8");
    assert!(shortcuts_js.contains("if (key === \"s\" && !event.shiftKey)"));
    assert!(shortcuts_js.contains("void runtime.saveNow(true);"));
    assert!(shortcuts_js.contains("if (key === \"n\" && !event.shiftKey)"));
    assert!(shortcuts_js.contains("runtime.showCreate();"));
    assert!(shortcuts_js.contains("if (key === \"p\" && event.shiftKey)"));
    assert!(shortcuts_js.contains("void runtime.previewNow();"));
    assert!(shortcuts_js.contains("if (key === \"k\" && !event.shiftKey)"));
    assert!(shortcuts_js.contains("runtime.focusQuickOpen();"));
}
