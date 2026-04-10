//! Admin dashboard handlers

use crate::error::AppError;
use crate::web::db::{self, DbPool, PopularWindow};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/admin")]
pub async fn admin_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req).await
}

#[get("/admin/")]
pub async fn admin_page_slash(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    admin_page_impl(pool, req).await
}

async fn admin_page_impl(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    if !session::check_session(&req, &pool).await? {
        return Ok(redirect(&session::login_url(&req)));
    }
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    let window = PopularWindow::Days30;
    let popular =
        db::list_popular_resources(&pool, true, settings.home_popular_limit, window).await?;
    let recent = db::list_recent_resources(&pool, true, settings.home_recent_limit).await?;
    let favorites = db::list_all_favorite_resources(&pool, true).await?;
    let stats = db::get_resource_stats(&pool, true).await?;
    Ok(html(templates::admin_page(
        &stats,
        &settings,
        &popular
            .iter()
            .map(|resource| view::popular_index_item(resource, true, window))
            .collect::<Vec<_>>(),
        &recent
            .iter()
            .map(|resource| view::index_item(resource, true))
            .collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|resource| view::index_item(resource, true))
            .collect::<Vec<_>>(),
        window,
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
