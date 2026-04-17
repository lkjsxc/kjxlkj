use super::layout::html_escape;
use super::model::ResourceChrome;
use crate::web::db::{Resource, ResourceKind};

const EDITOR_CORE_JS: &str = include_str!("editor.js");
const EDITOR_SYNC_JS: &str = include_str!("editor_sync.js");
const EDITOR_UI_JS: &str = include_str!("editor_ui.js");
const EDITOR_UPLOAD_JS: &str = include_str!("editor_upload.js");
const NOTE_ACTIONS_JS: &str = include_str!("resource_actions.js");

pub fn editor_script(
    resource: &Resource,
    chrome: &ResourceChrome,
    is_admin: bool,
    site_name: &str,
) -> String {
    if !is_admin {
        return String::new();
    }
    format!(
        r#"<script>
var currentId = {};
var currentAlias = {};
var currentHref = {};
var currentSiteName = {};
var isFavorite = {};
var isPrivate = {};
{}
{}
{}
{}
{}
initEditor();
</script>"#,
        serde_json::to_string(&resource.id).unwrap(),
        serde_json::to_string(&resource.alias).unwrap(),
        serde_json::to_string(&chrome.current_href).unwrap(),
        serde_json::to_string(site_name).unwrap(),
        resource.is_favorite,
        resource.is_private,
        NOTE_ACTIONS_JS,
        EDITOR_UI_JS,
        EDITOR_CORE_JS,
        EDITOR_SYNC_JS,
        if resource.kind == ResourceKind::Note {
            EDITOR_UPLOAD_JS
        } else {
            ""
        },
    )
}

pub fn editor_surface(resource: &Resource, chrome: &ResourceChrome) -> String {
    let alias = html_escape(resource.alias.as_deref().unwrap_or(""));
    let href = html_escape(&chrome.current_href);
    let body = html_escape(&resource.body);
    let upload_controls = if resource.kind == ResourceKind::Note {
        r#"<button type="button" id="upload-media-trigger" class="btn">Upload media</button>
<input id="upload-media-input" type="file" accept="image/*,video/*,.heic,.heif,.mkv,.ogv,.avi,.wmv,.mpeg,.mpg,.3gp" multiple hidden>"#
    } else {
        ""
    };
    format!(
        r#"<section class="surface resource-surface editor-shell preview-closed" id="editor-shell">
<div class="editor-toolbar-row">
<div class="editor-controls">
<button type="button" id="preview-toggle" class="btn" aria-expanded="false" onclick="togglePreview()">Show preview</button>
{upload_controls}
<label class="check-row" for="favorite-toggle"><input type="checkbox" id="favorite-toggle" {favorite_checked}><span>Favorite</span></label>
<label class="check-row" for="public-toggle"><input type="checkbox" id="public-toggle" {public_checked}><span>Public</span></label>
</div>
<div class="editor-statuses">
<span id="upload-media-status" class="editor-status" hidden aria-live="polite"></span>
<span id="save-error" class="editor-status" hidden aria-live="polite">Save failed. Retry on the next change.</span>
</div>
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
<textarea id="editor-body" class="resource-editor" spellcheck="false">{body}</textarea>
</label>
<aside id="editor-preview-panel" class="editor-preview-panel" hidden>
<div class="editor-preview-frame">
<div id="editor-preview" class="editor-preview-body prose"></div>
</div>
</aside>
</div>
<button type="button" id="preview-backdrop" class="editor-preview-backdrop" hidden aria-label="Close preview" onclick="closePreview()"></button>
</section>"#,
        upload_controls = upload_controls,
        favorite_checked = if chrome.is_favorite { "checked" } else { "" },
        public_checked = if resource.is_private { "" } else { "checked" },
    )
}
