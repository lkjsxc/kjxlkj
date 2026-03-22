mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn stale_save_is_last_write_wins_and_emits_visible_conflict_warning() {
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

    let first_save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "first-post"),
                ("title", ""),
                ("body", "# First save"),
                ("private", "false"),
                ("last_known_revision", ""),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(first_save.status(), StatusCode::OK);
    let first_body = String::from_utf8(test::read_body(first_save).await.to_vec()).expect("utf8");
    assert!(first_body.contains("id=\"last_known_revision\""));
    let first_revision = extract_revision(&first_body);
    assert!(!first_revision.is_empty());

    let second_save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "first-post"),
                ("title", ""),
                ("body", "# Second save"),
                ("private", "false"),
                ("last_known_revision", first_revision.as_str()),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(second_save.status(), StatusCode::OK);
    let second_body = String::from_utf8(test::read_body(second_save).await.to_vec()).expect("utf8");
    let second_revision = extract_revision(&second_body);
    assert_ne!(second_revision, first_revision);
    assert!(!second_revision.is_empty());
    assert!(second_body.contains("data-conflict=\"false\""));

    let stale_save = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "first-post"),
                ("title", ""),
                ("body", "# Third stale overwrite"),
                ("private", "false"),
                ("last_known_revision", first_revision.as_str()),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(stale_save.status(), StatusCode::OK);
    let stale_trigger = stale_save
        .headers()
        .get("HX-Trigger")
        .and_then(|value| value.to_str().ok())
        .expect("hx-trigger")
        .to_owned();
    assert!(stale_trigger.contains("admin-save-conflict"));
    assert!(stale_trigger.contains("stale-overwrite"));
    let stale_body = String::from_utf8(test::read_body(stale_save).await.to_vec()).expect("utf8");
    assert!(stale_body.contains("id=\"admin-conflict-banner\""));
    assert!(stale_body.contains("data-conflict=\"true\""));
    assert!(stale_body.contains("Warning: a stale editor snapshot was saved"));
    assert!(stale_body.contains("data-action=\"reload-latest\""));
    assert!(stale_body.contains("data-action=\"continue-editing\""));
    assert!(stale_body.contains("Saved from a stale snapshot."));

    let admin_open = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin/open/first-post")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(admin_open.status(), StatusCode::OK);
    let open_body = String::from_utf8(test::read_body(admin_open).await.to_vec()).expect("utf8");
    assert!(open_body.contains("id=\"admin-editor-pane\""));
    assert!(open_body.contains("# Third stale overwrite"));
}

fn extract_revision(html: &str) -> String {
    let marker = "id=\"last_known_revision\"";
    let Some(start) = html.find(marker) else {
        return String::new();
    };
    let slice = &html[start..];
    let value_marker = "value=\"";
    let Some(value_start) = slice.find(value_marker) else {
        return String::new();
    };
    let rest = &slice[value_start + value_marker.len()..];
    let Some(value_end) = rest.find('"') else {
        return String::new();
    };
    rest[..value_end].to_owned()
}
