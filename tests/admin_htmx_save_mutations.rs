mod web_test_support;

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};
use kjxlkj::web::router::configure_routes;

use web_test_support::make_web_state;

#[actix_web::test]
async fn htmx_save_rename_toggle_and_delete_return_expected_fragments() {
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

    let create = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/create")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
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
    let created_revision = extract_revision(&create_body);
    assert!(!created_revision.is_empty());

    let save_ok = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/save")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([
                ("slug", "created-post"),
                ("title", "Created"),
                ("body", "# Created update"),
                ("private", "false"),
                ("last_known_revision", created_revision.as_str()),
            ])
            .to_request(),
    )
    .await;
    assert_eq!(save_ok.status(), StatusCode::OK);
    assert!(save_ok.headers().contains_key("HX-Trigger"));
    let save_ok_body = String::from_utf8(test::read_body(save_ok).await.to_vec()).expect("utf8");
    assert!(save_ok_body.contains("id=\"admin-status-banner\""));
    assert!(save_ok_body.contains("id=\"admin-conflict-banner\""));
    assert!(save_ok_body.contains("data-conflict=\"false\""));

    let rename = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/rename")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .set_form([("slug", "created-post"), ("new_slug", "renamed-post")])
            .to_request(),
    )
    .await;
    assert_eq!(rename.status(), StatusCode::OK);
    let rename_body = String::from_utf8(test::read_body(rename).await.to_vec()).expect("utf8");
    assert!(rename_body.contains("renamed-post"));
    assert!(rename_body.contains("id=\"admin-article-list\""));
    assert!(rename_body.contains("id=\"admin-editor-pane\""));

    let toggle = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/toggle-private/renamed-post")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(toggle.status(), StatusCode::OK);
    let toggle_body = String::from_utf8(test::read_body(toggle).await.to_vec()).expect("utf8");
    assert!(toggle_body.contains("id=\"admin-article-list\""));
    assert!(toggle_body.contains("admin-private-badge"));

    let delete = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/admin/delete/renamed-post")
            .insert_header(("HX-Request", "true"))
            .insert_header((header::COOKIE, cookie))
            .to_request(),
    )
    .await;
    assert_eq!(delete.status(), StatusCode::OK);
    let delete_body = String::from_utf8(test::read_body(delete).await.to_vec()).expect("utf8");
    assert!(delete_body.contains("id=\"admin-article-list\""));
    assert!(delete_body.contains("id=\"admin-editor-pane\""));
    assert!(delete_body.contains("No article selected"));
    assert!(delete_body.contains("id=\"admin-preview-pane\""));
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
