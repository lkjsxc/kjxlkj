//! Note page template

use super::layout::{base, html_escape, shell_page};
use super::model::{ResourceAnalytics, ResourceChrome};
use super::resource_media::{admin_media_panel, current_media_block};
use super::resource_shell::resource_rail;
use crate::core::render_markdown;
use crate::web::db::{Resource, ResourceKind};
use crate::web::site::SiteContext;

const EDITOR_CORE_JS: &str = include_str!("editor.js");
const EDITOR_SYNC_JS: &str = include_str!("editor_sync.js");
const EDITOR_UI_JS: &str = include_str!("editor_ui.js");
const EDITOR_UPLOAD_JS: &str = include_str!("editor_upload.js");
const NOTE_ACTIONS_JS: &str = include_str!("resource_actions.js");

pub fn resource_page(
    resource: &Resource,
    chrome: &ResourceChrome,
    analytics: Option<&ResourceAnalytics>,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let content = format!(
        r#"<header class="page-head resource-head">
<div class="page-meta">
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
</div>
</header>
{}{}"#,
        chrome.created_at,
        chrome.updated_at,
        analytics_block(analytics),
        if is_admin {
            format!(
                "{}{}",
                if resource.kind == ResourceKind::Media {
                    admin_media_panel(resource)
                } else {
                    String::new()
                },
                editor_surface(resource, chrome),
            )
        } else {
            format!(
                r#"{}<section class="surface resource-surface prose">{}</section>"#,
                if resource.kind == ResourceKind::Media {
                    current_media_block(resource)
                } else {
                    String::new()
                },
                render_markdown(&resource.body)
            )
        }
    );
    base(
        &site.page_meta(
            &chrome.title,
            resource.summary.clone(),
            !is_admin && !resource.is_private,
            (!is_admin && !resource.is_private).then_some(chrome.current_href.as_str()),
        ),
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &resource_rail(chrome, is_admin, &chrome.current_href),
            &content,
            "resource-page",
            &site.site_name,
        ),
        "",
        &editor_script(resource, chrome, is_admin, &site.site_name),
    )
}

fn editor_script(
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

fn analytics_block(analytics: Option<&ResourceAnalytics>) -> String {
    let Some(analytics) = analytics else {
        return String::new();
    };
    format!(
        r#"<section class="surface resource-analytics-grid">
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

fn editor_surface(resource: &Resource, chrome: &ResourceChrome) -> String {
    let alias = html_escape(resource.alias.as_deref().unwrap_or(""));
    let href = html_escape(&chrome.current_href);
    let body = html_escape(&resource.body);
    let upload_controls = if resource.kind == ResourceKind::Note {
        r#"<button type="button" id="upload-media-trigger" class="btn">Upload media</button>
<input id="upload-media-input" type="file" accept="image/*,video/*" multiple hidden>"#
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
