mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;
use uuid::Uuid;

use web_test_support::make_web_state;

#[actix_web::test]
async fn setup_login_and_inline_article_edit_workflow() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("first-post", true, "# Private first");
    content.insert_article("second-post", true, "# Private second");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let home_before_setup =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(home_before_setup.status(), StatusCode::FOUND);

    let setup_get =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    let setup_get_text =
        String::from_utf8(test::read_body(setup_get).await.to_vec()).expect("utf8");
    assert!(setup_get_text.contains("name=\"username\" value=\"admin\""));

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
            .and_then(|v| v.to_str().ok()),
        Some("/login")
    );

    let login_bad = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "wrong")])
            .to_request(),
    )
    .await;
    assert_eq!(login_bad.status(), StatusCode::UNAUTHORIZED);

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("username", "admin"), ("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    let cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("session cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_owned();
    let session_id = parse_session_id(&cookie);
    assert_ne!(session_id, Uuid::nil());

    let home_public =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    let home_public_text =
        String::from_utf8(test::read_body(home_public).await.to_vec()).expect("utf8");
    assert!(!home_public_text.contains("first-post"));

    let home_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    let home_admin_text =
        String::from_utf8(test::read_body(home_admin).await.to_vec()).expect("utf8");
    assert!(home_admin_text.contains("first-post"));
    assert!(home_admin_text.contains("article-private"));

    let article_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    let article_admin_text =
        String::from_utf8(test::read_body(article_admin).await.to_vec()).expect("utf8");
    assert!(article_admin_text.contains("id=\"article-inline-editor\""));
    assert!(article_admin_text.contains("id=\"article-updated\""));
    assert!(article_admin_text.contains("id=\"article-next\""));
    assert!(article_admin_text.contains("href=\"/article/second-post\""));
    assert!(article_admin_text.contains("View history"));

    let edit = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/article/first-post/edit")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("title", "Draft title"),
                ("body", "# Edited"),
                ("private", "false"),
                ("last_known_revision", ""),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(edit.status(), StatusCode::OK);
    let edit_text = String::from_utf8(test::read_body(edit).await.to_vec()).expect("utf8");
    assert!(edit_text.contains("Saved at"));
    assert!(edit_text.contains("id=\"article-edit-form\""));

    let article_public = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post")
            .to_request(),
    )
    .await;
    assert_eq!(article_public.status(), StatusCode::OK);
    let article_public_text =
        String::from_utf8(test::read_body(article_public).await.to_vec()).expect("utf8");
    assert!(article_public_text.contains("id=\"article-nav\""));

    let history = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post/history")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(history.status(), StatusCode::OK);
    let history_text = String::from_utf8(test::read_body(history).await.to_vec()).expect("utf8");
    assert!(history_text.contains("id=\"article-history-page\""));

    let create = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "draft-20260322083632"),
                ("title", "Draft 2026-03-22 08:36 UTC"),
                ("body", ""),
                ("private", "true"),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(create.status(), StatusCode::SEE_OTHER);

    let settings = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/settings")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(settings.status(), StatusCode::OK);

    let delete = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/delete/second-post")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(delete.status(), StatusCode::NO_CONTENT);

    let trash = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/trash")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    let trash_text = String::from_utf8(test::read_body(trash).await.to_vec()).expect("utf8");
    assert!(trash_text.contains("second-post"));

    let restore = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/trash/restore/second-post")
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(restore.status(), StatusCode::SEE_OTHER);
}

fn parse_session_id(cookie: &str) -> Uuid {
    let value = cookie
        .split(';')
        .find_map(|part| part.trim().strip_prefix("session_id="))
        .expect("session id cookie value");
    Uuid::parse_str(value).expect("valid session uuid")
}
