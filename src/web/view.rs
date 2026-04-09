//! Presentation helpers for HTML templates

use crate::core::derive_title;
use crate::error::AppError;
use crate::web::db::{
    self, DbPool, ListedRecord, MediaFamily, PopularWindow, Record, RecordSnapshot,
};
use crate::web::templates::{
    render_time, HistoryLink, IndexItem, IndexMetric, NavLink, NoteAnalytics, NoteChrome,
};

pub fn index_item(record: &ListedRecord, show_visibility: bool) -> IndexItem {
    build_index_item(record, show_visibility, Vec::new())
}

pub fn popular_index_item(
    record: &ListedRecord,
    show_admin_details: bool,
    window: PopularWindow,
) -> IndexItem {
    let metrics = show_admin_details
        .then(|| IndexMetric {
            label: window.metric_label().to_string(),
            value: record.popular_views.unwrap_or(0).to_string(),
        })
        .into_iter()
        .collect();
    build_index_item(record, show_admin_details, metrics)
}

pub fn note_analytics(stats: &db::NoteViewStats) -> NoteAnalytics {
    NoteAnalytics {
        total: stats.total,
        views_7d: stats.views_7d,
        views_30d: stats.views_30d,
        views_90d: stats.views_90d,
        last_viewed_at: stats.last_viewed_at.as_ref().map(render_time),
    }
}

pub async fn note_chrome(
    pool: &DbPool,
    record: &Record,
    is_admin: bool,
) -> Result<NoteChrome, AppError> {
    Ok(NoteChrome {
        id: record.id.clone(),
        kind: record.kind,
        alias: record.alias.clone(),
        title: title_for(record),
        summary: record.summary.clone(),
        current_href: note_href(record),
        created_at: render_time(&record.created_at),
        updated_at: render_time(&record.updated_at),
        is_favorite: record.is_favorite,
        visibility: visibility_label(record.is_private),
        previous: adjacent_link(pool, &record.id, is_admin, true).await?,
        next: adjacent_link(pool, &record.id, is_admin, false).await?,
        history_href: history_href(record),
    })
}

pub fn history_links(snapshots: &[RecordSnapshot], first_page: bool) -> Vec<HistoryLink> {
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
    record: &ListedRecord,
    show_visibility: bool,
    metrics: Vec<IndexMetric>,
) -> IndexItem {
    let kind_badge = match record.record.media_family {
        Some(MediaFamily::Image) => "Image",
        Some(MediaFamily::Video) => "Video",
        None => "Note",
    };
    IndexItem {
        id: record.record.id.clone(),
        href: note_href(&record.record),
        title: title_for(&record.record),
        summary: record.preview.clone(),
        created_at: render_time(&record.record.created_at),
        updated_at: render_time(&record.record.updated_at),
        kind_badge,
        media_family: record.record.media_family,
        media_href: record
            .record
            .media_family
            .map(|_| file_href(&record.record)),
        is_favorite: record.record.is_favorite,
        visibility: show_visibility.then_some(visibility_label(record.record.is_private)),
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
        db::get_previous_record(pool, id, include_private).await?
    } else {
        db::get_next_record(pool, id, include_private).await?
    };
    Ok(target.map(|note| NavLink {
        href: note_href(&note),
        title: title_for(&note),
        summary: note.summary.clone(),
        created_at: render_time(&note.created_at),
    }))
}

fn title_for(record: &Record) -> String {
    title_from(&record.title, &record.body)
}

fn title_from(title: &str, body: &str) -> String {
    if title.trim().is_empty() {
        derive_title(body)
    } else {
        title.to_string()
    }
}

pub fn note_href(record: &Record) -> String {
    format!("/{}", record.alias.as_deref().unwrap_or(&record.id))
}

pub fn file_href(record: &Record) -> String {
    format!("{}/file", note_href(record))
}

pub fn history_href(record: &Record) -> String {
    format!("{}/history", note_href(record))
}

pub fn snapshot_href(snapshot: &RecordSnapshot) -> String {
    format!("/{}", snapshot.id)
}

pub fn visibility_label(is_private: bool) -> &'static str {
    if is_private {
        "Private"
    } else {
        "Public"
    }
}
