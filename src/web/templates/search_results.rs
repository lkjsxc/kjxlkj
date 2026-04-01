//! Search result listing section

use super::index::{note_row, pager};
use super::model::IndexItem;
use super::search_form::scope_meta;
use super::sections::section_with_actions;
use crate::web::db::{ListScope, PopularWindow};

pub(super) struct ResultsSection<'a> {
    pub(super) notes: &'a [IndexItem],
    pub(super) previous_cursor: Option<&'a str>,
    pub(super) next_cursor: Option<&'a str>,
    pub(super) query: &'a str,
    pub(super) limit: i64,
    pub(super) sort: &'a str,
    pub(super) has_query: bool,
    pub(super) scope: &'a ListScope,
    pub(super) popular_window: PopularWindow,
}

pub(super) fn results_section(model: ResultsSection<'_>) -> String {
    let cards = if model.notes.is_empty() {
        format!(
            r#"<p class="surface-empty">{}</p>"#,
            if model.has_query {
                "No matching notes."
            } else {
                "No notes yet."
            }
        )
    } else {
        model
            .notes
            .iter()
            .map(note_row)
            .collect::<Vec<_>>()
            .join("")
    };
    section_with_actions(
        results_title(model.has_query, model.scope),
        scope_meta(model.scope, model.popular_window),
        &format!(
            r#"<div class="note-list note-grid">{cards}</div>
{}"#,
            pager(
                "/search",
                model.previous_cursor,
                model.next_cursor,
                &[
                    ("q", model.query),
                    ("scope", model.scope.as_str()),
                    ("popular_window", model.popular_window.as_str()),
                    ("sort", model.sort),
                    ("limit", &model.limit.to_string()),
                ],
            )
        ),
        "note-section",
    )
}

fn results_title(has_query: bool, scope: &ListScope) -> &'static str {
    if has_query {
        "Results"
    } else {
        match scope {
            ListScope::Favorites => "Favorites",
            ListScope::Popular => "Popular notes",
            ListScope::All => "Notes",
        }
    }
}
