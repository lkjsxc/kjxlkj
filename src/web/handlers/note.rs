//! Note page handler

use crate::core::looks_like_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

enum RootResource {
    Current(db::Record),
    Snapshot(db::SnapshotResource),
}

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
    let settings = db::get_settings(&pool).await?;
    let site = SiteContext::from_settings(&settings);
    let resource = match resolve_root_resource(&pool, &reference).await? {
        Some(resource) => resource,
        None => return Ok(not_found(&site)),
    };
    match resource {
        RootResource::Current(record) => {
            render_current_note(&pool, &reference, &record, is_admin, &site).await
        }
        RootResource::Snapshot(resource) => {
            render_snapshot(&pool, &resource, is_admin, &site).await
        }
    }
}

async fn resolve_root_resource(
    pool: &DbPool,
    reference: &str,
) -> Result<Option<RootResource>, AppError> {
    if !looks_like_id(reference) {
        return Ok(db::get_record_by_alias(pool, reference)
            .await?
            .map(RootResource::Current));
    }
    if let Some(record) = db::get_record(pool, reference).await? {
        return Ok(Some(RootResource::Current(record)));
    }
    Ok(db::get_snapshot_resource(pool, reference)
        .await?
        .map(RootResource::Snapshot))
}

async fn render_current_note(
    pool: &DbPool,
    reference: &str,
    record: &db::Record,
    is_admin: bool,
    site: &SiteContext,
) -> Result<HttpResponse, AppError> {
    if record.is_private && !is_admin {
        return Ok(not_found(site));
    }
    if record
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == record.id
    {
        return Ok(redirect(&view::note_href(record)));
    }
    db::record_note_view(pool, &record.id).await?;
    let chrome = view::note_chrome(pool, record, is_admin).await?;
    let analytics = if is_admin {
        Some(view::note_analytics(
            &db::get_note_view_stats(pool, &record.id).await?,
        ))
    } else {
        None
    };
    Ok(html(templates::note_page(
        record,
        &chrome,
        analytics.as_ref(),
        is_admin,
        site,
    )))
}

async fn render_snapshot(
    pool: &DbPool,
    resource: &db::SnapshotResource,
    is_admin: bool,
    site: &SiteContext,
) -> Result<HttpResponse, AppError> {
    if (resource.record.is_private || resource.snapshot.is_private) && !is_admin {
        return Ok(not_found(site));
    }
    let chrome = view::note_chrome(pool, &resource.record, is_admin).await?;
    Ok(html(templates::snapshot_page(
        &chrome,
        &resource.snapshot,
        is_admin,
        site,
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

fn not_found(site: &SiteContext) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(templates::not_found_page(&site.page_meta(
            "Not Found",
            "The requested note could not be found.",
            false,
            None,
        )))
}
