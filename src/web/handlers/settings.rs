//! Admin settings handler

use crate::error::AppError;
use crate::web::db::{self, AppSettings, DbPool};
use crate::web::handlers::session;
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SettingsForm {
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub search_results_per_page: i64,
    pub default_vim_mode: Option<String>,
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
        .append_header(("Location", "/admin"))
        .finish())
}

fn validate(form: &SettingsForm) -> Result<AppSettings, AppError> {
    let settings = AppSettings {
        home_recent_limit: form.home_recent_limit.clamp(1, 24),
        home_favorite_limit: form.home_favorite_limit.clamp(1, 24),
        search_results_per_page: form.search_results_per_page.clamp(5, 100),
        default_vim_mode: form.default_vim_mode.is_some(),
    };
    if form.home_recent_limit < 1
        || form.home_favorite_limit < 1
        || form.search_results_per_page < 1
    {
        return Err(AppError::InvalidRequest(
            "settings values must be positive".to_string(),
        ));
    }
    Ok(settings)
}
