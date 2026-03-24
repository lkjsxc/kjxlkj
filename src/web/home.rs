use actix_web::{web, HttpRequest, HttpResponse};

use super::guards::redirect_to_setup;
use super::session::valid_session;
use super::state::AppState;

pub async fn get_home(request: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match state.auth_store.has_admin_user().await {
        Ok(false) => return redirect_to_setup(),
        Ok(true) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let is_admin = match valid_session(&request, &state).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let title = if is_admin {
        "Record service · admin session"
    } else {
        "Record service"
    };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_home_page(title, is_admin))
}

fn render_home_page(title: &str, is_admin: bool) -> String {
    let session_note = if is_admin {
        "<p id=\"session-state\">Admin session active.</p>"
    } else {
        "<p id=\"session-state\">Public mode.</p>"
    };
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
</head>
<body>
  <main id="home-page">
    <h1>Record service</h1>
    <p>API endpoints remain available at <code>/v1/records</code>.</p>
    {session_note}
  </main>
</body>
</html>"#
    )
}
