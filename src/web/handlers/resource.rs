//! Resource page handler

use crate::core::looks_like_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool};
use crate::web::handlers::http;
use crate::web::handlers::session;
use crate::web::markdown;
use crate::web::routes::AppState;
use crate::web::site::SiteContext;
use crate::web::templates;
use crate::web::view;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;

enum RootResource {
    Current(Box<db::Resource>),
    Snapshot(Box<db::SnapshotTarget>),
}

pub async fn resource_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(reference): Path<String>,
) -> Result<Response, AppError> {
    let pool = &state.pool;
    if !db::is_setup(pool).await? {
        return Ok(http::redirect("/setup"));
    }
    let is_admin = session::check_session(&headers, pool).await?;
    let settings = db::get_settings(pool).await?;
    let site = SiteContext::from_settings(&settings);
    let resource = match resolve_root_resource(pool, &reference).await? {
        Some(resource) => resource,
        None => return Ok(not_found(&site)),
    };
    match resource {
        RootResource::Current(resource) => {
            render_current_resource(pool, &reference, resource.as_ref(), is_admin, &site).await
        }
        RootResource::Snapshot(resource) => {
            render_snapshot(pool, resource.as_ref(), is_admin, &site).await
        }
    }
}

pub async fn resource_page_scoped(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_user, reference)): Path<(String, String)>,
) -> Result<Response, AppError> {
    resource_page(State(state), headers, Path(reference)).await
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
) -> Result<Response, AppError> {
    if resource.is_private && !is_admin {
        return Ok(not_found(site));
    }
    if resource
        .alias
        .as_deref()
        .is_some_and(|alias| alias != reference)
        && reference == resource.id
    {
        return Ok(http::redirect(&view::resource_href(resource)));
    }
    if !is_admin {
        db::count_resource_view(pool, &resource.id).await?;
    }
    let chrome = view::resource_chrome(pool, resource, is_admin).await?;
    let maps_key = db::get_settings(pool).await?.google_maps_embed_api_key;
    let body_html = markdown::render_markdown_page(
        pool,
        &resource.body,
        Some(&resource.id),
        is_admin,
        site.public_base_url.as_deref(),
        Some(&maps_key),
    )
    .await?;
    let analytics = if is_admin {
        Some(view::resource_analytics(
            &db::get_resource_view_stats(pool, &resource.id).await?,
        ))
    } else {
        None
    };
    Ok(http::html(templates::resource_page(
        resource,
        &chrome,
        analytics.as_ref(),
        &body_html,
        is_admin,
        site,
    )))
}

async fn render_snapshot(
    pool: &DbPool,
    target: &db::SnapshotTarget,
    is_admin: bool,
    site: &SiteContext,
) -> Result<Response, AppError> {
    if target.snapshot.is_private && !is_admin {
        return Ok(not_found(site));
    }
    let chrome = view::resource_chrome(pool, &target.resource, is_admin).await?;
    let maps_key = db::get_settings(pool).await?.google_maps_embed_api_key;
    let body_html = markdown::render_markdown_page(
        pool,
        &target.snapshot.body,
        Some(&target.resource.id),
        is_admin,
        site.public_base_url.as_deref(),
        Some(&maps_key),
    )
    .await?;
    Ok(http::html(templates::snapshot_page(
        &chrome,
        &target.snapshot,
        &body_html,
        is_admin,
        site,
    )))
}

fn not_found(site: &SiteContext) -> Response {
    http::html_status(
        StatusCode::NOT_FOUND,
        templates::not_found_page(&site.page_meta(
            "Not Found",
            "The requested resource could not be found.",
            false,
            None,
        )),
    )
}
