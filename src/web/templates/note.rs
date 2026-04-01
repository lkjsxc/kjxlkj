//! Note page template

use super::layout::{base, html_escape, shell_page};
use super::model::NoteChrome;
use super::note_shell::note_rail;
use crate::core::render_markdown;
use crate::web::db::Record;

const EDITOR_CORE_JS: &str = include_str!("editor.js");
const EDITOR_SYNC_JS: &str = include_str!("editor_sync.js");
const EDITOR_UI_JS: &str = include_str!("editor_ui.js");
const EDITOR_VIM_JS: &str = include_str!("editor_vim.js");
const NOTE_ACTIONS_JS: &str = include_str!("note_actions.js");
const TOAST_UI_ROOT: &str = "/assets/vendor/toastui/3.2.2";

pub fn note_page(
    record: &Record,
    chrome: &NoteChrome,
    is_admin: bool,
    default_vim_mode: bool,
) -> String {
    let content = format!(
        r#"<header class="page-head note-head">
<div class="page-title-stack"><h1 data-live-title>{}</h1></div>
<div class="page-meta">
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
</div>
</header>
{}"#,
        chrome.title,
        chrome.created_at,
        chrome.updated_at,
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
        &editor_head(is_admin),
        &editor_script(record, chrome, is_admin, default_vim_mode),
    )
}

fn editor_head(is_admin: bool) -> String {
    if !is_admin {
        return String::new();
    }
    format!(
        r#"<link rel="stylesheet" href="{TOAST_UI_ROOT}/toastui-editor.min.css">
<link rel="stylesheet" href="{TOAST_UI_ROOT}/toastui-editor-dark.min.css">
<script src="{TOAST_UI_ROOT}/toastui-editor-all.min.js"></script>"#
    )
}

fn editor_script(
    record: &Record,
    chrome: &NoteChrome,
    is_admin: bool,
    default_vim_mode: bool,
) -> String {
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
var defaultVimMode = {};
{}
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
        default_vim_mode,
        NOTE_ACTIONS_JS,
        EDITOR_UI_JS,
        EDITOR_VIM_JS,
        EDITOR_CORE_JS,
        EDITOR_SYNC_JS
    )
}

fn editor_surface(record: &Record, chrome: &NoteChrome) -> String {
    format!(
        r#"<section class="surface note-surface editor-shell preview-closed" id="editor-shell">
<div class="editor-toolbar-row">
<div class="editor-controls">
<button type="button" id="preview-toggle" class="btn" aria-expanded="false" hidden onclick="togglePreview()">Show preview</button>
<label class="check-row" for="favorite-toggle"><input type="checkbox" id="favorite-toggle" {}><span>Favorite</span></label>
<label class="check-row" for="public-toggle"><input type="checkbox" id="public-toggle" {}><span>Public</span></label>
</div>
<span id="save-error" class="save-error" hidden aria-live="polite">Save failed. Retry on the next change.</span>
</div>
<div class="editor-meta-grid">
<label class="editor-url-card editor-field-card" for="alias-input">
<small>URL alias</small>
<input type="text" id="alias-input" value="{}" placeholder="Optional alias">
</label>
<div class="editor-url-card"><small>Canonical URL</small><a href="{}" data-current-url>{}</a></div>
<div class="editor-url-card"><small>Mode</small><strong data-vim-mode-state>Vim off</strong></div>
</div>
<div id="editor-root" class="toast-host"></div>
<button type="button" id="preview-backdrop" class="editor-preview-backdrop" hidden aria-label="Close preview" onclick="closePreview()"></button>
<textarea id="editor-source" hidden>{}</textarea>
<textarea id="editor-fallback" class="note-editor" hidden>{}</textarea>
</section>"#,
        if chrome.is_favorite { "checked" } else { "" },
        if record.is_private { "" } else { "checked" },
        html_escape(record.alias.as_deref().unwrap_or("")),
        chrome.current_href,
        chrome.current_href,
        html_escape(&record.body),
        html_escape(&record.body),
    )
}
