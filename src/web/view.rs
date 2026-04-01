//! Presentation helpers for HTML templates

use crate::core::derive_title;
use crate::error::AppError;
use crate::web::db::{self, DbPool, ListedRecord, Record, RecordRevision};
use crate::web::templates::{render_time, HistoryLink, IndexItem, NavLink, NoteChrome};

pub fn index_item(record: &ListedRecord, show_visibility: bool) -> IndexItem {
    IndexItem {
        id: record.record.id.clone(),
        href: note_href(&record.record),
        title: title_for(&record.record),
        summary: record.preview.clone(),
        created_at: render_time(&record.record.created_at),
        updated_at: render_time(&record.record.updated_at),
        is_favorite: record.record.is_favorite,
        visibility: show_visibility.then_some(visibility_label(record.record.is_private)),
    }
}

pub async fn note_chrome(
    pool: &DbPool,
    record: &Record,
    is_admin: bool,
) -> Result<NoteChrome, AppError> {
    Ok(NoteChrome {
        id: record.id.clone(),
        alias: record.alias.clone(),
        title: title_for(record),
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

pub fn history_links(record: &Record, revisions: &[RecordRevision]) -> Vec<HistoryLink> {
    revisions
        .iter()
        .map(|revision| HistoryLink {
            href: format!("{}/history/{}", note_href(record), revision.revision_number),
            label: format!("Revision {}", revision.revision_number),
            created_at: render_time(&revision.created_at),
            status: visibility_label(revision.is_private),
        })
        .collect()
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
        relation: if older { "Prev" } else { "Next" },
        title: title_for(&note),
        created_at: render_time(&note.created_at),
    }))
}

fn title_for(record: &Record) -> String {
    if record.title.trim().is_empty() {
        derive_title(&record.body)
    } else {
        record.title.clone()
    }
}

pub fn note_href(record: &Record) -> String {
    format!("/{}", record.alias.as_deref().unwrap_or(&record.id))
}

pub fn history_href(record: &Record) -> String {
    format!("{}/history", note_href(record))
}

pub fn visibility_label(is_private: bool) -> &'static str {
    if is_private {
        "Private"
    } else {
        "Public"
    }
}
