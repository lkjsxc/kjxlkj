//! Homepage handler

use crate::error::AppError;
use crate::web::db::{self, DbPool, PopularWindow};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/")]
pub async fn home_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let is_admin = session::check_session(&req, &pool).await?;
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    let window = PopularWindow::Days30;
    let popular =
        db::list_popular_resources(&pool, is_admin, settings.home_popular_limit, window).await?;
    let recent = db::list_recent_resources(&pool, is_admin, settings.home_recent_limit).await?;
    let favorites =
        db::list_favorite_resources(&pool, is_admin, settings.home_favorite_limit).await?;
    Ok(html(templates::home_page(
        &settings,
        &popular
            .iter()
            .map(|resource| view::popular_index_item(resource, is_admin, window))
            .collect::<Vec<_>>(),
        &recent
            .iter()
            .map(|resource| view::index_item(resource, is_admin))
            .collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|resource| view::index_item(resource, is_admin))
            .collect::<Vec<_>>(),
        window,
        is_admin,
        &site,
    )))
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
