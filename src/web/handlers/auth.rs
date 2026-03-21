use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use askama::Template;
use serde::Deserialize;
use uuid::Uuid;

use crate::core::auth;
use crate::web::handlers::common::html;
use crate::web::state::AppState;
use crate::web::templates::AuthTemplate;

#[derive(Deserialize)]
pub struct Cred {
    username: String,
    password: String,
}

#[get("/setup")]
pub async fn setup_get(state: web::Data<AppState>) -> impl Responder {
    match state.auth.has_admin().await {
        Ok(true) => HttpResponse::Conflict().body("setup_already_completed"),
        Ok(false) => html(
            AuthTemplate {
                mode: "setup",
                action: "/setup",
                message: "",
            }
            .render(),
        ),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/setup")]
pub async fn setup_post(form: web::Form<Cred>, state: web::Data<AppState>) -> impl Responder {
    match state.auth.has_admin().await {
        Ok(true) => return HttpResponse::Conflict().body("setup_already_completed"),
        Ok(false) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
    let hash = match auth::hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::BadRequest().body("password_hash_failed"),
    };
    match state.auth.create_admin(&form.username, &hash).await {
        Ok(_) => HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/login")]
pub async fn login_get() -> impl Responder {
    html(
        AuthTemplate {
            mode: "login",
            action: "/login",
            message: "",
        }
        .render(),
    )
}

#[post("/login")]
pub async fn login_post(
    form: web::Form<Cred>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let hash = match state.auth.get_password_hash(&form.username).await {
        Ok(Some(hash)) => hash,
        Ok(None) => return HttpResponse::Unauthorized().body("invalid_credentials"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !auth::verify_password(&form.password, &hash) {
        return HttpResponse::Unauthorized().body("invalid_credentials");
    }
    match state.auth.create_session(&form.username).await {
        Ok(sid) => {
            if session.insert("sid", sid.to_string()).is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Found()
                .append_header(("Location", "/admin"))
                .finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/logout")]
pub async fn logout_post(session: Session, state: web::Data<AppState>) -> impl Responder {
    if let Some(raw) = session.get::<String>("sid").ok().flatten() {
        if let Ok(id) = Uuid::parse_str(&raw) {
            let _ = state.auth.delete_session(id).await;
        }
    }
    session.remove("sid");
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
