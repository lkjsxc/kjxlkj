use actix_session::Session;
use actix_web::HttpResponse;
use askama::Error as TemplateError;
use uuid::Uuid;

use crate::web::state::AppState;

pub fn html(body: Result<String, TemplateError>) -> HttpResponse {
    match body {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn is_admin(session: &Session, state: &AppState) -> anyhow::Result<bool> {
    let Some(raw) = session.get::<String>("sid").ok().flatten() else {
        return Ok(false);
    };
    let Ok(uuid) = Uuid::parse_str(&raw) else {
        return Ok(false);
    };
    state.auth.valid_session(uuid).await
}
