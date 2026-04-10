//! Setup handlers

use crate::error::AppError;
use crate::web::db::DbPool;
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone)]
pub struct SetupCode {
    value: String,
}

impl SetupCode {
    pub fn new(configured: Option<String>) -> Self {
        Self {
            value: configured.unwrap_or_else(|| Uuid::new_v4().to_string()),
        }
    }

    pub fn reveal(&self) -> &str {
        &self.value
    }

    fn matches(&self, candidate: &str) -> bool {
        constant_time_eq(self.value.as_bytes(), candidate.trim().as_bytes())
    }
}

/// Setup form data
#[derive(Debug, Deserialize)]
pub struct SetupForm {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub setup_code: String,
}

/// Setup page GET handler
#[get("/setup")]
pub async fn setup_page(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    if crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/login"));
    }
    let settings = crate::web::db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(html(templates::setup_page(&site, None)))
}

/// Setup form POST handler
#[post("/setup")]
pub async fn setup_submit(
    pool: web::Data<DbPool>,
    setup_code: web::Data<SetupCode>,
    form: web::Form<SetupForm>,
) -> Result<HttpResponse, AppError> {
    if crate::web::db::is_setup(&pool).await? {
        return Ok(redirect("/login"));
    }

    let errors = validate_setup_form(&form, &setup_code);
    if !errors.is_empty() {
        let settings = crate::web::db::get_settings(&pool).await?;
        let site = SiteContext::from_settings(&settings);
        return Ok(html_status(
            actix_web::http::StatusCode::BAD_REQUEST,
            templates::setup_page(&site, Some(&errors.join(", "))),
        ));
    }

    crate::web::db::create_admin(&pool, &form.username, &form.password).await?;
    Ok(see_other("/login"))
}

fn validate_setup_form(form: &SetupForm, setup_code: &SetupCode) -> Vec<&'static str> {
    let mut errors = Vec::new();
    if !setup_code.matches(&form.setup_code) {
        errors.push("Setup failed");
        return errors;
    }
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

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let mut diff = left.len() ^ right.len();
    let max = left.len().max(right.len());
    for index in 0..max {
        diff |= left.get(index).copied().unwrap_or(0) as usize
            ^ right.get(index).copied().unwrap_or(0) as usize;
    }
    diff == 0
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
