//! Presentation helpers for HTML templates

use crate::core::extract_title;
use crate::error::AppError;
use crate::web::db::{self, DbPool, Record, RecordRevision};
use crate::web::templates::{HistoryLink, IndexItem, NavLink, NoteChrome};

const SIDEBAR_HISTORY_LIMIT: usize = 5;

pub fn index_item(record: &Record) -> IndexItem {
    IndexItem {
        href: format!("/{}", record.slug),
        title: title_for(record),
        summary: summary_for(record),
        slug: record.slug.clone(),
        meta: format!(
            "Created {} · Updated {}",
            crate::web::templates::format_date(&record.created_at),
            crate::web::templates::format_date(&record.updated_at)
        ),
        status: visibility_label(record.is_private),
    }
}

pub async fn note_chrome(
    pool: &DbPool,
    record: &Record,
    is_admin: bool,
) -> Result<NoteChrome, AppError> {
    let previous = adjacent_link(pool, &record.slug, is_admin, true).await?;
    let next = adjacent_link(pool, &record.slug, is_admin, false).await?;
    let revisions = db::get_record_revisions(pool, &record.slug).await?;
    Ok(NoteChrome {
        title: title_for(record),
        slug: record.slug.clone(),
        created_at: crate::web::templates::format_date(&record.created_at),
        updated_at: crate::web::templates::format_date(&record.updated_at),
        visibility: visibility_label(record.is_private),
        previous,
        next,
        history: filtered_history_links(record, &revisions, is_admin)
            .into_iter()
            .take(SIDEBAR_HISTORY_LIMIT)
            .collect(),
        history_href: format!("/{}/history", record.slug),
    })
}

pub fn visible_history(
    record: &Record,
    revisions: &[RecordRevision],
    is_admin: bool,
) -> Vec<HistoryLink> {
    filtered_history_links(record, revisions, is_admin)
}

async fn adjacent_link(
    pool: &DbPool,
    slug: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<NavLink>, AppError> {
    let target = if older {
        db::get_previous_slug(pool, slug, include_private).await?
    } else {
        db::get_next_slug(pool, slug, include_private).await?
    };
    match target {
        Some(target_slug) => {
            let record = db::get_record(pool, &target_slug).await?;
            Ok(record.map(|note| NavLink {
                href: format!("/{}", note.slug),
                label: title_for(&note),
                meta: format!(
                    "{} · {}",
                    if older { "Older note" } else { "Newer note" },
                    crate::web::templates::format_date(&note.created_at)
                ),
            }))
        }
        None => Ok(None),
    }
}

fn title_for(record: &Record) -> String {
    extract_title(&record.body).unwrap_or_else(|| record.slug.clone())
}

fn summary_for(record: &Record) -> String {
    record
        .body
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(shorten)
        .unwrap_or_else(|| "No summary yet.".to_string())
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
            href: format!("/{}/history/{}", record.slug, revision.revision_number),
            label: format!("Revision {}", revision.revision_number),
            meta: crate::web::templates::format_date(&revision.created_at),
            status: visibility_label(revision.is_private),
            active: false,
        })
        .collect()
}

pub fn visibility_label(is_private: bool) -> &'static str {
    if is_private {
        "Private"
    } else {
        "Public"
    }
}

fn shorten(line: &str) -> String {
    const LIMIT: usize = 96;
    if line.chars().count() <= LIMIT {
        line.to_string()
    } else {
        let prefix: String = line.chars().take(LIMIT - 1).collect();
        format!("{prefix}…")
    }
}
