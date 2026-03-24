use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::http::{header, StatusCode};
use actix_web::{test, web, App};

use kjxlkj::core::auth::DEFAULT_SESSION_TIMEOUT_MINUTES;
use kjxlkj::error::AppError;
use kjxlkj::storage::FsStore;
use kjxlkj::web::{configure_routes, AppState, AuthStore};

use crate::support::TestDatabase;

#[actix_web::test]
async fn setup_login_and_session_routes_follow_expected_flow() {
    let test_db = TestDatabase::create("setup_login_flow").await;
    let store_root = unique_test_dir("setup-login-flow");
    let state = build_state("test-token", &store_root, &test_db.url)
        .await
        .expect("state");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(configure_routes),
    )
    .await;

    let before_setup_home =
        test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(before_setup_home.status(), StatusCode::FOUND);
    assert_eq!(
        header_value(&before_setup_home, header::LOCATION),
        Some("/setup")
    );

    let get_setup =
        test::call_service(&app, test::TestRequest::get().uri("/setup").to_request()).await;
    assert_eq!(get_setup.status(), StatusCode::OK);
    assert_eq!(
        header_value(&get_setup, header::CONTENT_TYPE),
        Some("text/html; charset=utf-8")
    );
    let setup_body = String::from_utf8(test::read_body(get_setup).await.to_vec()).expect("utf8");
    assert!(setup_body.contains("<title>Initial setup</title>"));

    let bad_setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("password", "   ")])
            .to_request(),
    )
    .await;
    assert_eq!(bad_setup.status(), StatusCode::BAD_REQUEST);
    assert!(String::from_utf8(test::read_body(bad_setup).await.to_vec())
        .expect("utf8")
        .contains("password is required"));

    let good_setup = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/setup")
            .set_form([("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(good_setup.status(), StatusCode::SEE_OTHER);
    assert_eq!(header_value(&good_setup, header::LOCATION), Some("/login"));

    let get_login =
        test::call_service(&app, test::TestRequest::get().uri("/login").to_request()).await;
    assert_eq!(get_login.status(), StatusCode::OK);
    assert!(String::from_utf8(test::read_body(get_login).await.to_vec())
        .expect("utf8")
        .contains("<title>Admin login</title>"));

    let bad_login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("password", "wrong")])
            .to_request(),
    )
    .await;
    assert_eq!(bad_login.status(), StatusCode::UNAUTHORIZED);

    let good_login = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/login")
            .set_form([("password", "s3cret")])
            .to_request(),
    )
    .await;
    assert_eq!(good_login.status(), StatusCode::SEE_OTHER);
    assert_eq!(header_value(&good_login, header::LOCATION), Some("/admin"));
    let session_cookie = parse_session_cookie(&good_login);

    let get_admin = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/admin")
            .insert_header((header::COOKIE, session_cookie.clone()))
            .to_request(),
    )
    .await;
    assert_eq!(get_admin.status(), StatusCode::OK);

    let logout = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/logout")
            .insert_header((header::COOKIE, session_cookie))
            .to_request(),
    )
    .await;
    assert_eq!(logout.status(), StatusCode::NO_CONTENT);
    let clear_cookie = header_value(&logout, header::SET_COOKIE).expect("clear cookie");
    assert!(clear_cookie.contains("session_id="));
    assert!(clear_cookie.contains("Max-Age=0"));

    let get_admin_after_logout =
        test::call_service(&app, test::TestRequest::get().uri("/admin").to_request()).await;
    assert_eq!(get_admin_after_logout.status(), StatusCode::FOUND);
    assert_eq!(
        header_value(&get_admin_after_logout, header::LOCATION),
        Some("/login")
    );

    drop(app);
    let _ = tokio::fs::remove_dir_all(store_root).await;
    test_db.drop().await;
}

fn header_value(
    response: &actix_web::dev::ServiceResponse,
    name: header::HeaderName,
) -> Option<&str> {
    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
}

fn parse_session_cookie(response: &actix_web::dev::ServiceResponse) -> String {
    let raw = header_value(response, header::SET_COOKIE).expect("set-cookie header");
    assert!(raw.contains("HttpOnly"));
    raw.split(';').next().expect("cookie pair").to_owned()
}

fn unique_test_dir(suite: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    PathBuf::from(format!("tmp/tests-{suite}-{nanos}"))
}

async fn build_state(
    token: &str,
    store_root: &PathBuf,
    db_url: &str,
) -> Result<AppState, AppError> {
    tokio::fs::create_dir_all(store_root).await.expect("mkdir");
    let store = FsStore::new(store_root.clone());
    store.ensure_ready().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(db_url)
        .await?;
    let auth_store = AuthStore::new(pool);
    auth_store.ensure_ready().await?;
    Ok(AppState {
        admin_token: token.to_owned(),
        store,
        auth_store,
        session_timeout_minutes: DEFAULT_SESSION_TIMEOUT_MINUTES,
    })
}
