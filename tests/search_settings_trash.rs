mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn search_settings_and_trash_flows_work_with_role_rules() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("public-post", false, "public body");
    content.insert_article("private-post", true, "private body");
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

    let search_public = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/search?q=private")
            .to_request(),
    )
    .await;
    assert_eq!(search_public.status(), StatusCode::OK);
    let search_public_text =
        String::from_utf8(test::read_body(search_public).await.to_vec()).expect("utf8");
    assert!(search_public_text.contains("<main id=\"search-page\">"));
    assert!(!search_public_text.contains("private-post"));

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
        .and_then(|v| v.to_str().ok())
        .expect("set cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();

    let search_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/search?q=private")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    let search_admin_text =
        String::from_utf8(test::read_body(search_admin).await.to_vec()).expect("utf8");
    assert!(search_admin_text.contains("private-post"));
    assert!(search_admin_text.contains("id=\"app-shell\""));

    let settings = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/settings")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(settings.status(), StatusCode::OK);
    let settings_text = String::from_utf8(test::read_body(settings).await.to_vec()).expect("utf8");
    assert!(settings_text.contains("<main id=\"admin-settings-page\">"));
    assert!(settings_text.contains("id=\"admin-settings-form\""));

    let save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/settings/save")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("site_title", "Team Wiki"),
                ("session_timeout_minutes", "60"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(save.status(), StatusCode::SEE_OTHER);

    let moved = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/delete/public-post")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert!(moved.status().is_success());

    let trash = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/trash")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    let trash_text = String::from_utf8(test::read_body(trash).await.to_vec()).expect("utf8");
    assert!(trash_text.contains("<main id=\"admin-trash-page\">"));
    assert!(trash_text.contains("public-post"));

    let restore = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/trash/restore/public-post")
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(restore.status(), StatusCode::SEE_OTHER);

    let login2 = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "secret")])
            .to_request(),
    )
    .await;
    let cookie2 = login2
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("set cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();
    let missing_restore = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/trash/restore/does-not-exist")
            .insert_header((header::COOKIE, cookie2))
            .to_request(),
    )
    .await;
    assert_eq!(missing_restore.status(), StatusCode::NOT_FOUND);
}
