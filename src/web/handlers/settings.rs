//! Admin settings handler

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::handlers::settings_input::{validate_settings_form, SettingsForm};
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

#[get("/admin/settings")]
pub async fn settings_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    session::require_session(&req, &pool).await?;
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    Ok(html(templates::settings_page(&settings, &site)))
}

#[post("/admin/settings")]
pub async fn settings_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<SettingsForm>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let current = db::get_settings(&pool).await?;
    db::update_settings(&pool, &validate_settings_form(&form, &current)?).await?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/settings"))
        .finish())
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
