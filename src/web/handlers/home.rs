//! Homepage handler

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
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
    let recent = db::list_recent_records(&pool, is_admin, settings.home_recent_limit).await?;
    let favorites =
        db::list_favorite_records(&pool, is_admin, settings.home_favorite_limit).await?;
    Ok(html(templates::home_page(
        &recent
            .iter()
            .map(|record| view::index_item(record, is_admin))
            .collect::<Vec<_>>(),
        &favorites
            .iter()
            .map(|record| view::index_item(record, is_admin))
            .collect::<Vec<_>>(),
        is_admin,
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
