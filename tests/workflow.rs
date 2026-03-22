mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use chrono::{TimeZone, Utc};
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
    assert!(setup_get_text.contains("Create password for fixed admin account <code>admin</code>."));
    assert!(setup_get_text.contains("name=\"password\""));
    assert!(!setup_get_text.contains("name=\"username\""));

    let setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("password", "s3cret")])
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

    let login_get =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(login_get.status(), StatusCode::OK);
    let login_get_text =
        String::from_utf8(test::read_body(login_get).await.to_vec()).expect("utf8");
    assert!(login_get_text.contains("id=\"login-form\""));
    assert!(login_get_text.contains("name=\"password\""));
    assert!(!login_get_text.contains("name=\"username\""));

    let login_bad = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("password", "wrong")])
            .to_request(),
    )
    .await;
    assert_eq!(login_bad.status(), StatusCode::UNAUTHORIZED);

    let login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("password", "s3cret")])
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
    assert!(!home_public_text.contains("byline"));
    assert!(!home_public_text.contains("data-author"));

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
    assert!(!home_admin_text.contains("byline"));
    assert!(!home_admin_text.contains("data-author"));

    let search_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/search?q=first")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(search_admin.status(), StatusCode::OK);
    let search_admin_text =
        String::from_utf8(test::read_body(search_admin).await.to_vec()).expect("utf8");
    assert!(!search_admin_text.contains("byline"));
    assert!(!search_admin_text.contains("data-author"));

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
    assert!(article_admin_text.contains("id=\"article-edit-form\""));
    assert!(article_admin_text.contains("hx-post=\"/article/first-post/edit\""));
    let private_idx = article_admin_text
        .find("label for=\"private\"")
        .expect("private label exists");
    let body_idx = article_admin_text
        .find("label for=\"body\"")
        .expect("body label exists");
    assert!(private_idx < body_idx);
    assert!(!article_admin_text.contains(">Save</button>"));
    assert!(!article_admin_text.contains(">Preview</button>"));
    assert!(!article_admin_text.contains("split-view"));
    assert!(article_admin_text.contains("id=\"article-updated\""));
    assert!(article_admin_text.contains("id=\"article-next\""));
    assert!(article_admin_text.contains("href=\"/article/second-post\""));
    assert!(article_admin_text.contains("View history"));
    assert!(!article_admin_text.contains("byline"));
    assert!(!article_admin_text.contains("data-author"));

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
    assert!(edit_text.contains("hx-post=\"/article/first-post/edit\""));
    let edit_private_idx = edit_text
        .find("label for=\"private\"")
        .expect("private label exists");
    let edit_body_idx = edit_text
        .find("label for=\"body\"")
        .expect("body label exists");
    assert!(edit_private_idx < edit_body_idx);
    assert!(!edit_text.contains(">Save</button>"));
    assert!(!edit_text.contains(">Preview</button>"));
    assert!(!edit_text.contains("split-view"));

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
    assert!(!article_public_text.contains("byline"));
    assert!(!article_public_text.contains("data-author"));

    let edit_private = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/article/first-post/edit")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("title", "Draft title"),
                ("body", "# Edited private"),
                ("private", "true"),
                ("last_known_revision", ""),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(edit_private.status(), StatusCode::OK);

    let article_public_hidden = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post")
            .to_request(),
    )
    .await;
    assert_eq!(article_public_hidden.status(), StatusCode::NOT_FOUND);

    let history_public = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post/history")
            .to_request(),
    )
    .await;
    assert_eq!(history_public.status(), StatusCode::NOT_FOUND);

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
    assert!(history_text.contains("id=\"article-history-list\""));
    assert!(history_text.contains("<code>"));
    assert!(history_text.contains("<time>"));
    assert!(history_text.contains("<p>autosave</p>"));
    assert!(history_text.contains("action=\"/article/first-post/history/restore\""));
    assert!(history_text.contains(">Restore</button>"));
    let history_commit_ids = history_text
        .split("<code>")
        .skip(1)
        .filter_map(|segment| segment.split("</code>").next())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    assert!(!history_commit_ids.is_empty());
    let restore_commit_id = history_commit_ids
        .last()
        .cloned()
        .expect("history commit id");

    let restore_requires_admin = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/article/first-post/history/restore")
            .set_form([("commit_id", restore_commit_id.as_str())])
            .to_request(),
    )
    .await;
    assert_eq!(restore_requires_admin.status(), StatusCode::FOUND);
    assert_eq!(
        restore_requires_admin
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok()),
        Some("/login")
    );

    let restore_history = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/article/first-post/history/restore")
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([("commit_id", restore_commit_id.as_str())])
            .to_request(),
    )
    .await;
    assert_eq!(restore_history.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        restore_history
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok()),
        Some("/article/first-post")
    );

    let article_restored = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/article/first-post")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(article_restored.status(), StatusCode::OK);
    let article_restored_text =
        String::from_utf8(test::read_body(article_restored).await.to_vec()).expect("utf8");
    assert!(article_restored_text.contains("Private first"));
    assert!(!article_restored_text.contains("Edited private"));

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

    let obsolete_save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(obsolete_save.status(), StatusCode::NOT_FOUND);

    let obsolete_toggle_private = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/toggle-private/first-post")
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(obsolete_toggle_private.status(), StatusCode::NOT_FOUND);

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

#[actix_web::test]
async fn article_navigation_and_last_updated_follow_created_timeline() {
    let (state, _, _, content) = make_web_state();
    content.insert_article("alpha", false, "# Alpha");
    content.insert_article("beta", false, "# Beta");
    content.insert_article("gamma", false, "# Gamma");
    assert!(content.set_article_timeline(
        "alpha",
        Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
    ));
    assert!(content.set_article_timeline(
        "beta",
        Utc.with_ymd_and_hms(2025, 1, 2, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2025, 1, 9, 15, 30, 0).unwrap()
    ));
    assert!(content.set_article_timeline(
        "gamma",
        Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap()
    ));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let response = test::call_service(
        &app,
        test::TestRequest::get().uri("/article/beta").to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let text = String::from_utf8(test::read_body(response).await.to_vec()).expect("utf8");
    assert!(text.contains("id=\"article-updated\""));
    assert!(text.contains("Last updated: 2025-01-09 15:30 UTC"));
    assert!(text.contains("id=\"article-prev\" href=\"/article/alpha\""));
    assert!(text.contains("id=\"article-next\" href=\"/article/gamma\""));

    let alpha_idx = text.find("data-slug=\"alpha\"").expect("alpha in nav");
    let beta_idx = text.find("data-slug=\"beta\"").expect("beta in nav");
    let gamma_idx = text.find("data-slug=\"gamma\"").expect("gamma in nav");
    assert!(alpha_idx < beta_idx);
    assert!(beta_idx < gamma_idx);
}

fn parse_session_id(cookie: &str) -> Uuid {
    let value = cookie
        .split(';')
        .find_map(|part| part.trim().strip_prefix("session_id="))
        .expect("session id cookie value");
    Uuid::parse_str(value).expect("valid session uuid")
}
