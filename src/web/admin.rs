use actix_web::{web, HttpRequest, HttpResponse};

use super::session::require_admin_session;
use super::state::AppState;

pub async fn get_admin(request: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8" /><title>Admin</title></head>
<body>
  <main id="admin-page">
    <h1>Admin session active</h1>
    <form method="post" action="/logout"><button type="submit">Sign out</button></form>
  </main>
</body>
</html>"#,
        )
}
