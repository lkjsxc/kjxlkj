//! Resource page handler

use crate::core::looks_like_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::session;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use actix_web::{get, web, HttpRequest, HttpResponse};

enum RootResource {
    Current(Box<db::Resource>),
    Snapshot(Box<db::SnapshotTarget>),
}

#[get("/{reference}")]
pub async fn resource_page(
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
        RootResource::Current(resource) => {
            render_current_resource(&pool, &reference, resource.as_ref(), is_admin, &site).await
        }
        RootResource::Snapshot(resource) => {
            render_snapshot(&pool, resource.as_ref(), is_admin, &site).await
        }
    }
}

async fn resolve_root_resource(
    pool: &DbPool,
    reference: &str,
) -> Result<Option<RootResource>, AppError> {
    if !looks_like_id(reference) {
        return Ok(db::get_resource_by_alias(pool, reference)
            .await?
            .map(Box::new)
            .map(RootResource::Current));
    }
    if let Some(resource) = db::get_resource(pool, reference).await? {
        return Ok(Some(RootResource::Current(Box::new(resource))));
    }
    Ok(db::get_snapshot_target(pool, reference)
        .await?
        .map(Box::new)
        .map(RootResource::Snapshot))
}

async fn render_current_resource(
    pool: &DbPool,
    reference: &str,
    resource: &db::Resource,
    is_admin: bool,
    site: &SiteContext,
) -> Result<HttpResponse, AppError> {
    if resource.is_private && !is_admin {
        return Ok(not_found(site));
    }
    if resource
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == resource.id
    {
        return Ok(redirect(&view::resource_href(resource)));
    }
    db::count_resource_view(pool, &resource.id).await?;
    let chrome = view::resource_chrome(pool, resource, is_admin).await?;
    let analytics = if is_admin {
        Some(view::resource_analytics(
            &db::get_resource_view_stats(pool, &resource.id).await?,
        ))
    } else {
        None
    };
    Ok(html(templates::resource_page(
        resource,
        &chrome,
        analytics.as_ref(),
        is_admin,
        site,
    )))
}

async fn render_snapshot(
    pool: &DbPool,
    target: &db::SnapshotTarget,
    is_admin: bool,
    site: &SiteContext,
) -> Result<HttpResponse, AppError> {
    if target.snapshot.is_private && !is_admin {
        return Ok(not_found(site));
    }
    let chrome = view::resource_chrome(pool, &target.resource, is_admin).await?;
    Ok(html(templates::snapshot_page(
        &chrome,
        &target.snapshot,
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
            "The requested resource could not be found.",
            false,
            None,
        )))
}
