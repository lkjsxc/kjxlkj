//! Admin settings handlers

use crate::error::AppError;
use crate::web::db::{self, AppSettings, DbPool};
use crate::web::handlers::session;
use crate::web::{templates, view};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SettingsForm {
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub home_popular_limit: i64,
    pub home_recent_position: i64,
    pub home_favorite_position: i64,
    pub home_popular_position: i64,
    pub home_intro_markdown: String,
    pub search_results_per_page: i64,
    pub default_new_note_visibility: String,
    pub home_recent_visible: Option<String>,
    pub home_favorite_visible: Option<String>,
    pub home_popular_visible: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HomeIntroForm {
    pub home_intro_markdown: String,
}

#[get("/settings")]
pub async fn settings_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect("/login"));
    }
    let settings = db::get_settings(&pool).await?;
    let favorites = db::list_all_favorite_records(&pool, true).await?;
    Ok(html(templates::settings_page(
        &settings,
        &favorites
            .iter()
            .map(|record| view::index_item(record, true))
            .collect::<Vec<_>>(),
    )))
}

#[post("/settings")]
pub async fn settings_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<SettingsForm>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    db::update_settings(&pool, &validate(&form)?).await?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/settings"))
        .finish())
}

#[post("/settings/home-intro")]
pub async fn home_intro_submit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Form<HomeIntroForm>,
) -> Result<HttpResponse, AppError> {
    session::require_session(&req, &pool).await?;
    let mut settings = db::get_settings(&pool).await?;
    settings.home_intro_markdown = form.home_intro_markdown.trim().to_string();
    db::update_settings(&pool, &settings).await?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}

fn validate(form: &SettingsForm) -> Result<AppSettings, AppError> {
    let (popular, recent, favorite) = normalized_positions(form);
    let default_new_note_is_private = match form.default_new_note_visibility.as_str() {
        "private" => true,
        "public" => false,
        _ => {
            return Err(AppError::InvalidRequest(
                "invalid new note visibility".to_string(),
            ))
        }
    };
    Ok(AppSettings {
        home_recent_limit: form.home_recent_limit.clamp(1, 24),
        home_favorite_limit: form.home_favorite_limit.clamp(1, 24),
        home_popular_limit: form.home_popular_limit.clamp(1, 24),
        home_recent_visible: form.home_recent_visible.is_some(),
        home_favorite_visible: form.home_favorite_visible.is_some(),
        home_popular_visible: form.home_popular_visible.is_some(),
        home_recent_position: recent,
        home_favorite_position: favorite,
        home_popular_position: popular,
        home_intro_markdown: form.home_intro_markdown.trim().to_string(),
        search_results_per_page: form.search_results_per_page.clamp(5, 100),
        default_new_note_is_private,
    })
}

fn normalized_positions(form: &SettingsForm) -> (i64, i64, i64) {
    let mut items = vec![
        ("popular", form.home_popular_position, 1),
        ("recent", form.home_recent_position, 2),
        ("favorite", form.home_favorite_position, 3),
    ];
    items.sort_by_key(|(_, position, fallback)| (*position, *fallback));
    let mut popular = 1;
    let mut recent = 2;
    let mut favorite = 3;
    for (index, (key, _, _)) in items.into_iter().enumerate() {
        match key {
            "popular" => popular = (index + 1) as i64,
            "recent" => recent = (index + 1) as i64,
            _ => favorite = (index + 1) as i64,
        }
    }
    (popular, recent, favorite)
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
