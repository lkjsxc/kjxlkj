//! Setup handlers

use crate::error::AppError;
use crate::web::db::DbPool;
use crate::web::templates;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

/// Setup form data
#[derive(Debug, Deserialize)]
pub struct SetupForm {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

/// Setup page GET handler
#[get("/setup")]
pub async fn setup_page(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    if crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/login"));
    }
    Ok(html(templates::setup_page(None)))
}

/// Setup form POST handler
#[post("/setup")]
pub async fn setup_submit(
    pool: web::Data<DbPool>,
    form: web::Form<SetupForm>,
) -> Result<HttpResponse, AppError> {
    if crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/login"));
    }

    let errors = validate_setup_form(&form);
    if !errors.is_empty() {
        return Ok(html_status(
            actix_web::http::StatusCode::BAD_REQUEST,
            templates::setup_page(Some(&errors.join(", "))),
        ));
    }

    crate::web::db::create_admin(&pool, &form.username, &form.password).await?;
    Ok(see_other("/login"))
}

fn validate_setup_form(form: &SetupForm) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if form.username.len() < 3 {
        errors.push("Username must be at least 3 characters");
    }
    if form.password.len() < 8 {
        errors.push("Password must be at least 8 characters");
    }
    if form.password != form.confirm_password {
        errors.push("Passwords do not match");
    }
    errors
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header(("Location", location))
        .finish()
}

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

fn html_status(status: actix_web::http::StatusCode, body: String) -> HttpResponse {
    HttpResponse::build(status)
        .content_type("text/html; charset=utf-8")
        .body(body)
}
