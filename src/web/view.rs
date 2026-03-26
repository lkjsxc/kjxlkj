//! Presentation helpers for HTML templates

use crate::core::{derive_summary, derive_title};
use crate::error::AppError;
use crate::web::db::{self, DbPool, ListRequest, Record, RecordRevision};
use crate::web::templates::{render_time, HistoryLink, IndexItem, NavLink, NoteChrome, RecentLink};

const SIDEBAR_HISTORY_LIMIT: usize = 5;
const RECENT_LIMIT: i64 = 8;

pub fn index_item(record: &Record, show_visibility: bool) -> IndexItem {
    IndexItem {
        href: format!("/{}", record.id),
        title: title_for(record),
        summary: summary_for(record),
        created_at: render_time(&record.created_at),
        updated_at: render_time(&record.updated_at),
        visibility: show_visibility.then_some(visibility_label(record.is_private)),
    }
}

pub async fn note_chrome(
    pool: &DbPool,
    record: &Record,
    is_admin: bool,
) -> Result<NoteChrome, AppError> {
    let recent = recent_links(pool, is_admin).await?;
    let previous = adjacent_link(pool, &record.id, is_admin, true).await?;
    let next = adjacent_link(pool, &record.id, is_admin, false).await?;
    let revisions = db::get_record_revisions(pool, &record.id).await?;
    Ok(NoteChrome {
        id: record.id.clone(),
        title: title_for(record),
        current_href: format!("/{}", record.id),
        search_path: if is_admin { "/admin" } else { "/" },
        created_at: render_time(&record.created_at),
        updated_at: render_time(&record.updated_at),
        visibility: visibility_label(record.is_private),
        recent,
        previous,
        next,
        history: filtered_history_links(record, &revisions, is_admin)
            .into_iter()
            .take(SIDEBAR_HISTORY_LIMIT)
            .collect(),
        history_href: format!("/{}/history", record.id),
    })
}

pub fn visible_history(
    record: &Record,
    revisions: &[RecordRevision],
    is_admin: bool,
) -> Vec<HistoryLink> {
    filtered_history_links(record, revisions, is_admin)
}

pub async fn recent_links(pool: &DbPool, is_admin: bool) -> Result<Vec<RecentLink>, AppError> {
    let page = db::list_records(
        pool,
        &ListRequest {
            include_private: is_admin,
            limit: RECENT_LIMIT,
            query: None,
            cursor: None,
        },
    )
    .await?;
    Ok(page
        .records
        .iter()
        .map(|record| recent_item(record, is_admin))
        .collect())
}

async fn adjacent_link(
    pool: &DbPool,
    id: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<NavLink>, AppError> {
    let target = if older {
        db::get_previous_id(pool, id, include_private).await?
    } else {
        db::get_next_id(pool, id, include_private).await?
    };
    match target {
        Some(target_id) => {
            let record = db::get_record(pool, &target_id).await?;
            Ok(record.map(|note| NavLink {
                href: format!("/{}", note.id),
                relation: if older { "Prev" } else { "Next" },
                title: title_for(&note),
                created_at: render_time(&note.created_at),
            }))
        }
        None => Ok(None),
    }
}

fn title_for(record: &Record) -> String {
    if record.title.trim().is_empty() {
        derive_title(&record.body)
    } else {
        record.title.clone()
    }
}

fn summary_for(record: &Record) -> String {
    if record.summary.trim().is_empty() {
        derive_summary(&record.body)
    } else {
        record.summary.clone()
    }
}

fn filtered_history_links(
    record: &Record,
    revisions: &[RecordRevision],
    is_admin: bool,
) -> Vec<HistoryLink> {
    revisions
        .iter()
        .filter(|revision| is_admin || !revision.is_private)
        .map(|revision| HistoryLink {
            href: format!("/{}/history/{}", record.id, revision.revision_number),
            label: format!("Revision {}", revision.revision_number),
            created_at: render_time(&revision.created_at),
            status: visibility_label(revision.is_private),
            active: false,
        })
        .collect()
}

fn recent_item(record: &Record, show_visibility: bool) -> RecentLink {
    RecentLink {
        href: format!("/{}", record.id),
        title: title_for(record),
        updated_at: render_time(&record.updated_at),
        visibility: show_visibility.then_some(visibility_label(record.is_private)),
    }
}

pub fn visibility_label(is_private: bool) -> &'static str {
    if is_private {
        "Private"
    } else {
        "Public"
    }
}
