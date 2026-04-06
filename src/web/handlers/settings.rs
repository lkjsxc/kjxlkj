//! Admin settings handler

use crate::config::Config;
use crate::error::AppError;
use crate::web::db::{self, AppSettings, DbPool};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct SettingsForm {
    pub site_name: String,
    pub site_description: String,
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub home_popular_limit: i64,
    pub home_intro_markdown: String,
    pub home_recent_visible: Option<String>,
    pub home_favorite_visible: Option<String>,
    pub home_popular_visible: Option<String>,
    pub home_recent_position: i64,
    pub home_favorite_position: i64,
    pub home_popular_position: i64,
    pub search_results_per_page: i64,
    pub session_timeout_minutes: i64,
    pub default_new_note_is_private: Option<String>,
}

#[get("/admin/settings")]
pub async fn settings_page(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    session::require_session(&req, &pool).await?;
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&config, &settings);
    Ok(html(templates::settings_page(&settings, &site)))
}

#[post("/admin/settings")]
pub async fn settings_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<SettingsForm>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    db::update_settings(&pool, &validate(&form)?).await?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/settings"))
        .finish())
}

fn validate(form: &SettingsForm) -> Result<AppSettings, AppError> {
    let site_name = form.site_name.trim();
    let site_description = form.site_description.trim();
    if site_name.is_empty() || site_name.len() > 80 {
        return Err(AppError::InvalidRequest(
            "site name must be between 1 and 80 characters".to_string(),
        ));
    }
    if site_description.is_empty() || site_description.len() > 200 {
        return Err(AppError::InvalidRequest(
            "site description must be between 1 and 200 characters".to_string(),
        ));
    }
    if !counts_are_valid(form) {
        return Err(AppError::InvalidRequest(
            "section counts must be between 1 and 24".to_string(),
        ));
    }
    if !(5..=100).contains(&form.search_results_per_page) {
        return Err(AppError::InvalidRequest(
            "search page size must be between 5 and 100".to_string(),
        ));
    }
    if !(5..=10080).contains(&form.session_timeout_minutes) {
        return Err(AppError::InvalidRequest(
            "session timeout must be between 5 and 10080 minutes".to_string(),
        ));
    }
    if !positions_are_valid(form) {
        return Err(AppError::InvalidRequest(
            "section order must use 1, 2, and 3 exactly once".to_string(),
        ));
    }
    Ok(AppSettings {
        site_name: site_name.to_string(),
        site_description: site_description.to_string(),
        home_recent_limit: form.home_recent_limit,
        home_favorite_limit: form.home_favorite_limit,
        home_popular_limit: form.home_popular_limit,
        home_intro_markdown: form.home_intro_markdown.trim().to_string(),
        home_recent_visible: form.home_recent_visible.is_some(),
        home_favorite_visible: form.home_favorite_visible.is_some(),
        home_popular_visible: form.home_popular_visible.is_some(),
        home_recent_position: form.home_recent_position,
        home_favorite_position: form.home_favorite_position,
        home_popular_position: form.home_popular_position,
        search_results_per_page: form.search_results_per_page,
        session_timeout_minutes: form.session_timeout_minutes,
        default_new_note_is_private: form.default_new_note_is_private.is_some(),
    })
}

fn counts_are_valid(form: &SettingsForm) -> bool {
    [
        form.home_recent_limit,
        form.home_favorite_limit,
        form.home_popular_limit,
    ]
    .into_iter()
    .all(|value| (1..=24).contains(&value))
}

fn positions_are_valid(form: &SettingsForm) -> bool {
    let positions = [
        form.home_popular_position,
        form.home_recent_position,
        form.home_favorite_position,
    ];
    positions.iter().all(|value| (1..=3).contains(value))
        && positions.into_iter().collect::<HashSet<_>>().len() == 3
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", location))
        .finish()
}

fn html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[cfg(test)]
mod tests {
    use super::{validate, SettingsForm};

    fn sample_form() -> SettingsForm {
        SettingsForm {
            site_name: "Launchpad".to_string(),
            site_description: "Search-friendly notes.".to_string(),
            home_recent_limit: 5,
            home_favorite_limit: 5,
            home_popular_limit: 5,
            home_intro_markdown: "# Home".to_string(),
            home_recent_visible: Some("on".to_string()),
            home_favorite_visible: Some("on".to_string()),
            home_popular_visible: Some("on".to_string()),
            home_recent_position: 2,
            home_favorite_position: 3,
            home_popular_position: 1,
            search_results_per_page: 20,
            session_timeout_minutes: 1440,
            default_new_note_is_private: None,
        }
    }

    #[test]
    fn validate_accepts_default_timeout() {
        assert_eq!(
            validate(&sample_form()).unwrap().session_timeout_minutes,
            1440
        );
    }

    #[test]
    fn validate_rejects_short_timeout() {
        let mut form = sample_form();
        form.session_timeout_minutes = 4;
        assert!(validate(&form).is_err());
    }

    #[test]
    fn validate_rejects_long_timeout() {
        let mut form = sample_form();
        form.session_timeout_minutes = 10081;
        assert!(validate(&form).is_err());
    }

    #[test]
    fn validate_rejects_blank_site_name() {
        let mut form = sample_form();
        form.site_name = "   ".to_string();
        assert!(validate(&form).is_err());
    }
}
