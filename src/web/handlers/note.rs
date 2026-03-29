//! Note page handler

use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/{reference}")]
pub async fn note_page(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if !db::is_setup(&pool).await? {
        return Ok(redirect("/setup"));
    }
    let reference = path.into_inner();
    let is_admin = session::check_session(&req, &pool).await?;
    let record = match db::get_record_by_ref(&pool, &reference).await? {
        Some(record) => record,
        None => return Ok(not_found()),
    };
    if record.is_private && !is_admin {
        return Ok(not_found());
    }
    if record
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == record.id
    {
        return Ok(redirect(&view::note_href(&record)));
    }
    let chrome = view::note_chrome(&pool, &record, is_admin).await?;
    let default_vim_mode = if is_admin {
        db::get_settings(&pool).await?.default_vim_mode
    } else {
        false
    };
    Ok(html(templates::note_page(
        &record,
        &chrome,
        is_admin,
        default_vim_mode,
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

fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(templates::not_found_page())
}
