use actix_web::{http::header, web, HttpRequest, HttpResponse};
use chrono::Utc;
use serde::Deserialize;

use crate::core::settings::{SiteSettings, MIN_SESSION_TIMEOUT_MINUTES};
use crate::web::handlers::app_shell::render_shell_page;
use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::handlers::page_html::escape_html;
use crate::web::state::WebState;

#[derive(Debug, Deserialize)]
pub struct SettingsForm {
    site_title: String,
    session_timeout_minutes: i32,
}

#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    status: Option<String>,
}

pub async fn handle_get_admin_settings(
    request: HttpRequest,
    state: web::Data<WebState>,
    query: web::Query<StatusQuery>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let slugs = match state.content_store.list_admin_slugs().await {
        Ok(slugs) => slugs,
        Err(error) => return internal_error(error),
    };
    let main = render_settings_main(&settings, query.status.as_deref().unwrap_or_default());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_shell_page(
            &settings.site_title,
            "Settings",
            &main,
            &slugs,
            true,
        ))
}

pub async fn handle_post_admin_settings_save(
    request: HttpRequest,
    state: web::Data<WebState>,
    form: web::Form<SettingsForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    if form.site_title.trim().is_empty()
        || form.session_timeout_minutes < MIN_SESSION_TIMEOUT_MINUTES
    {
        return HttpResponse::BadRequest().body("invalid settings");
    }
    let timeout = SiteSettings::normalized_timeout_minutes(form.session_timeout_minutes);
    let saved = match state
        .settings_store
        .save_settings(form.site_title.trim(), timeout)
        .await
    {
        Ok(saved) => saved,
        Err(error) => return internal_error(error),
    };
    HttpResponse::SeeOther()
        .append_header((
            header::LOCATION,
            format!("/admin/settings?status={}", saved.session_timeout_minutes),
        ))
        .finish()
}

pub async fn handle_post_admin_settings_reindex(
    request: HttpRequest,
    state: web::Data<WebState>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    if let Err(error) = state.content_store.trigger_search_reindex().await {
        return internal_error(error);
    }
    match state
        .settings_store
        .touch_reindex_timestamp(Utc::now())
        .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin/settings?status=reindex-ok"))
            .finish(),
        Err(error) => internal_error(error),
    }
}

fn render_settings_main(settings: &SiteSettings, status: &str) -> String {
    format!(
        "<main id=\"admin-settings-page\"><h1>Settings</h1><section id=\"admin-settings-status\">{}</section><section id=\"admin-settings-errors\"></section><form id=\"admin-settings-form\" method=\"post\" action=\"/admin/settings/save\"><label for=\"site_title\">Site title</label><input id=\"site_title\" name=\"site_title\" type=\"text\" value=\"{}\" /><label for=\"session_timeout_minutes\">Session timeout minutes</label><input id=\"session_timeout_minutes\" name=\"session_timeout_minutes\" type=\"number\" min=\"5\" value=\"{}\" /><button type=\"submit\">Save settings</button></form><form method=\"post\" action=\"/admin/settings/reindex\"><button type=\"submit\">Reindex search</button></form></main>",
        escape_html(status),
        escape_html(&settings.site_title),
        settings.session_timeout_minutes
    )
}

pub fn render_settings_main_page(settings: &SiteSettings, status: &str) -> String {
    render_settings_main(settings, status)
}
