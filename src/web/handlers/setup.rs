//! Setup handlers

use crate::error::AppError;
use crate::web::handlers::http;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::Response;
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
pub async fn setup_page(State(state): State<AppState>) -> Result<Response, AppError> {
    if crate::web::db::is_setup(&state.pool).await? {
        return Ok(http::redirect("/login"));
    }
    let settings = crate::web::db::get_settings(&state.pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(http::html(templates::setup_page(&site, None)))
}

/// Setup form POST handler
pub async fn setup_submit(
    State(state): State<AppState>,
    Form(form): Form<SetupForm>,
) -> Result<Response, AppError> {
    if crate::web::db::is_setup(&state.pool).await? {
        return Ok(http::redirect("/login"));
    }

    let errors = validate_setup_form(&form, &state.setup_code);
    if !errors.is_empty() {
        let settings = crate::web::db::get_settings(&state.pool).await?;
        let site = SiteContext::from_settings(&settings);
        return Ok(http::html_status(
            StatusCode::BAD_REQUEST,
            templates::setup_page(&site, Some(&errors.join(", "))),
        ));
    }

    crate::web::db::create_admin(&state.pool, &form.username, &form.password).await?;
    Ok(http::see_other("/login"))
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
