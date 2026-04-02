//! Note page template

use super::layout::{base, html_escape, shell_page};
use super::model::{NoteAnalytics, NoteChrome};
use super::note_shell::note_rail;
use crate::core::render_markdown;
use crate::web::db::Record;

const EDITOR_CORE_JS: &str = include_str!("editor.js");
const EDITOR_SYNC_JS: &str = include_str!("editor_sync.js");
const EDITOR_UI_JS: &str = include_str!("editor_ui.js");
const NOTE_ACTIONS_JS: &str = include_str!("note_actions.js");

pub fn note_page(
    record: &Record,
    chrome: &NoteChrome,
    analytics: Option<&NoteAnalytics>,
    is_admin: bool,
) -> String {
    let content = format!(
        r#"<header class="page-head note-head">
<div class="page-title-stack"><h1 data-live-title>{}</h1></div>
<div class="page-meta">
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
</div>
</header>
{}{}"#,
        chrome.title,
        chrome.created_at,
        chrome.updated_at,
        analytics_block(analytics),
        if is_admin {
            editor_surface(record, chrome)
        } else {
            format!(
                r#"<section class="surface note-surface prose">{}</section>"#,
                render_markdown(&record.body)
            )
        }
    );
    base(
        &chrome.title,
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &note_rail(chrome, is_admin, &chrome.current_href),
            &content,
            "note-page",
        ),
        "",
        &editor_script(record, chrome, is_admin),
    )
}

fn editor_script(record: &Record, chrome: &NoteChrome, is_admin: bool) -> String {
    if !is_admin {
        return String::new();
    }
    format!(
        r#"<script>
var currentId = {};
var currentAlias = {};
var currentHref = {};
var isFavorite = {};
var isPrivate = {};
{}
{}
{}
{}
initEditor();
</script>"#,
        serde_json::to_string(&record.id).unwrap(),
        serde_json::to_string(&record.alias).unwrap(),
        serde_json::to_string(&chrome.current_href).unwrap(),
        record.is_favorite,
        record.is_private,
        NOTE_ACTIONS_JS,
        EDITOR_UI_JS,
        EDITOR_CORE_JS,
        EDITOR_SYNC_JS
    )
}

fn analytics_block(analytics: Option<&NoteAnalytics>) -> String {
    let Some(analytics) = analytics else {
        return String::new();
    };
    format!(
        r#"<section class="surface note-analytics-grid">
<article><small>Views total</small><strong>{}</strong></article>
<article><small>Views 7d</small><strong>{}</strong></article>
<article><small>Views 30d</small><strong>{}</strong></article>
<article><small>Views 90d</small><strong>{}</strong></article>
<article><small>Last viewed</small><strong>{}</strong></article>
</section>"#,
        analytics.total,
        analytics.views_7d,
        analytics.views_30d,
        analytics.views_90d,
        analytics
            .last_viewed_at
            .clone()
            .unwrap_or_else(|| "Never".to_string()),
    )
}

fn editor_surface(record: &Record, chrome: &NoteChrome) -> String {
    let alias = html_escape(record.alias.as_deref().unwrap_or(""));
    let href = html_escape(&chrome.current_href);
    let body = html_escape(&record.body);
    format!(
        r#"<section class="surface note-surface editor-shell preview-closed" id="editor-shell">
<div class="editor-toolbar-row">
<div class="editor-controls">
<button type="button" id="preview-toggle" class="btn" aria-expanded="false" onclick="togglePreview()">Show preview</button>
<label class="check-row" for="favorite-toggle"><input type="checkbox" id="favorite-toggle" {}><span>Favorite</span></label>
<label class="check-row" for="public-toggle"><input type="checkbox" id="public-toggle" {}><span>Public</span></label>
</div>
<span id="save-error" class="save-error" hidden aria-live="polite">Save failed. Retry on the next change.</span>
</div>
<div class="editor-meta-grid">
<label class="editor-url-card editor-field-card" for="alias-input">
<small>URL alias</small>
<input type="text" id="alias-input" value="{alias}" placeholder="Optional alias">
</label>
<div class="editor-url-card editor-field-card"><small>Canonical URL</small><a href="{href}" data-current-url>{href}</a></div>
</div>
<div class="editor-workspace">
<label class="editor-field-card editor-body-card" for="editor-body">
<small>Markdown body</small>
<textarea id="editor-body" class="note-editor" spellcheck="false">{body}</textarea>
</label>
<aside id="editor-preview-panel" class="editor-preview-panel" hidden>
<div class="editor-preview-frame">
<div id="editor-preview" class="editor-preview-body prose"></div>
</div>
</aside>
</div>
<button type="button" id="preview-backdrop" class="editor-preview-backdrop" hidden aria-label="Close preview" onclick="closePreview()"></button>
</section>"#,
        if chrome.is_favorite { "checked" } else { "" },
        if record.is_private { "" } else { "checked" },
    )
}
