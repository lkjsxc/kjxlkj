//! Presentation helpers for HTML templates

use crate::core::derive_title;
use crate::error::AppError;
use crate::web::db::{
    self, DbPool, ListedResource, MediaFamily, PopularWindow, Resource, ResourceSnapshot,
};
use crate::web::templates::{
    render_time, HistoryLink, IndexItem, IndexMetric, NavLink, ResourceAnalytics, ResourceChrome,
};
use crate::web::view_media::card_file_href;

pub fn index_item(listed: &ListedResource, show_visibility: bool) -> IndexItem {
    build_index_item(listed, show_visibility, Vec::new())
}

pub fn popular_index_item(
    listed: &ListedResource,
    show_admin_details: bool,
    window: PopularWindow,
) -> IndexItem {
    let metrics = show_admin_details
        .then(|| IndexMetric {
            label: window.metric_label().to_string(),
            value: listed.popular_views.unwrap_or(0).to_string(),
        })
        .into_iter()
        .collect();
    build_index_item(listed, show_admin_details, metrics)
}

pub fn resource_analytics(stats: &db::ResourceViewStats) -> ResourceAnalytics {
    ResourceAnalytics {
        total: stats.total,
        views_7d: stats.views_7d,
        views_30d: stats.views_30d,
        views_90d: stats.views_90d,
        last_viewed_at: stats.last_viewed_at.as_ref().map(render_time),
    }
}

pub async fn resource_chrome(
    pool: &DbPool,
    resource: &Resource,
    is_admin: bool,
) -> Result<ResourceChrome, AppError> {
    Ok(ResourceChrome {
        id: resource.id.clone(),
        kind: resource.kind,
        alias: resource.alias.clone(),
        title: title_for(resource),
        summary: resource.summary.clone(),
        current_href: resource_href(resource),
        created_at: render_time(&resource.created_at),
        updated_at: render_time(&resource.updated_at),
        is_favorite: resource.is_favorite,
        visibility: visibility_label(resource.is_private),
        previous: adjacent_link(pool, &resource.id, is_admin, true).await?,
        next: adjacent_link(pool, &resource.id, is_admin, false).await?,
        history_href: history_href(resource),
    })
}

pub fn history_links(snapshots: &[ResourceSnapshot], first_page: bool) -> Vec<HistoryLink> {
    snapshots
        .iter()
        .enumerate()
        .map(|(index, snapshot)| HistoryLink {
            href: snapshot_href(snapshot),
            label: if first_page && index == 0 {
                "Latest saved snapshot".to_string()
            } else {
                format!("Saved snapshot {}", snapshot.snapshot_number)
            },
            summary: snapshot.summary.clone(),
            created_at: render_time(&snapshot.created_at),
            status: visibility_label(snapshot.is_private),
        })
        .collect()
}

fn build_index_item(
    listed: &ListedResource,
    show_visibility: bool,
    metrics: Vec<IndexMetric>,
) -> IndexItem {
    let resource = &listed.resource;
    let kind_badge = match resource.media_family {
        Some(MediaFamily::Image) => "Image",
        Some(MediaFamily::Video) => "Video",
        None => "Note",
    };
    IndexItem {
        id: resource.id.clone(),
        href: resource_href(resource),
        title: title_for(resource),
        summary: listed.preview.clone(),
        created_at: render_time(&resource.created_at),
        updated_at: render_time(&resource.updated_at),
        kind_badge,
        media_family: resource.media_family,
        media_href: resource.media_family.map(|_| card_file_href(resource)),
        is_favorite: resource.is_favorite,
        visibility: show_visibility.then_some(visibility_label(resource.is_private)),
        metrics,
    }
}

async fn adjacent_link(
    pool: &DbPool,
    id: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<NavLink>, AppError> {
    let target = if older {
        db::get_previous_resource(pool, id, include_private).await?
    } else {
        db::get_next_resource(pool, id, include_private).await?
    };
    Ok(target.map(|resource| NavLink {
        href: resource_href(&resource),
        title: title_for(&resource),
        summary: resource.summary.clone(),
        created_at: render_time(&resource.created_at),
    }))
}

fn title_for(resource: &Resource) -> String {
    title_from(&resource.title, &resource.body)
}

fn title_from(title: &str, body: &str) -> String {
    if title.trim().is_empty() {
        derive_title(body)
    } else {
        title.to_string()
    }
}

pub fn resource_href(resource: &Resource) -> String {
    format!("/{}", resource.alias.as_deref().unwrap_or(&resource.id))
}

pub fn file_href(resource: &Resource) -> String {
    format!("{}/file", resource_href(resource))
}

pub fn history_href(resource: &Resource) -> String {
    format!("{}/history", resource_href(resource))
}

pub fn snapshot_href(snapshot: &ResourceSnapshot) -> String {
    format!("/{}", snapshot.id)
}

pub fn visibility_label(is_private: bool) -> &'static str {
    if is_private {
        "Private"
    } else {
        "Public"
    }
}
