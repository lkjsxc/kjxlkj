use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/admin/media/new")]
pub async fn media_new_page(
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
    let site = SiteContext::from_settings(&settings);
    Ok(html(templates::media_new_page(
        settings.default_new_resource_is_private,
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
