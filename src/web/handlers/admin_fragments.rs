use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;

use super::admin_html::build_editor_form_html;
use crate::web::render::render_markdown_html;
use crate::web::state::{SaveConflict, SaveOutcome};

pub use super::admin_html::{
    escape_html, oob_attr, render_admin_editor_placeholder, render_admin_preview_empty,
};

pub const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminListItem {
    pub slug: String,
    pub private: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorDocument {
    pub slug: String,
    pub title: Option<String>,
    pub body: String,
    pub private: bool,
    pub revision: String,
}

pub fn html_fragment_response(status: StatusCode, body: String) -> HttpResponse {
    HttpResponse::build(status)
        .content_type(HTML_CONTENT_TYPE)
        .body(body)
}

pub fn render_admin_article_list(
    items: &[AdminListItem],
    active_slug: Option<&str>,
    oob: bool,
) -> String {
    let rows = items
        .iter()
        .map(|item| {
            let slug = escape_html(&item.slug);
            let is_active = active_slug.is_some_and(|value| value == item.slug);
            let active_class = if is_active { " class=\"is-active\"" } else { "" };
            let aria_current = if is_active { " aria-current=\"true\"" } else { "" };
            let private_badge = if item.private {
                " <span class=\"admin-private-badge\" data-private=\"true\">private</span>"
            } else {
                ""
            };
            format!(
                "<li{active_class}><a href=\"/admin/open/{slug}\" data-slug=\"{slug}\" hx-get=\"/admin/open/{slug}\" hx-target=\"#admin-editor-pane\" hx-swap=\"outerHTML\"{aria_current}>{slug}</a>{private_badge}</li>"
            )
        })
        .collect::<String>();

    format!(
        "<section id=\"admin-article-list\"{}><ol>{rows}</ol></section>",
        oob_attr(oob)
    )
}

pub fn render_admin_editor(document: &EditorDocument, oob: bool) -> String {
    let mut html = build_editor_form_html(
        &document.slug,
        document.title.as_deref().unwrap_or_default(),
        &document.body,
        document.private,
        &document.revision,
    );
    if oob {
        let pos = html.find("<section").unwrap();
        html.insert_str(pos + 8, " hx-swap-oob=\"outerHTML\"");
    }
    html
}

#[allow(dead_code)]
pub fn render_admin_editor_blank(oob: bool) -> String {
    render_admin_editor(
        &EditorDocument {
            slug: String::new(),
            title: None,
            body: String::new(),
            private: false,
            revision: String::new(),
        },
        oob,
    )
}

pub fn render_admin_preview(markdown_body: &str, oob: bool) -> String {
    format!(
        "<section id=\"admin-preview-pane\"{}>{}</section>",
        oob_attr(oob),
        render_markdown_html(markdown_body)
    )
}

pub fn render_admin_status_banner(message: &str, state: &str, oob: bool) -> String {
    format!(
        "<section id=\"admin-status-banner\" aria-live=\"polite\" data-status=\"{}\"{}>{}</section>",
        escape_html(state),
        oob_attr(oob),
        escape_html(message)
    )
}

pub fn render_admin_validation_banner(message: &str) -> String {
    render_admin_status_banner(message, "validation-error", false)
}

pub fn render_admin_conflict_clear(oob: bool) -> String {
    format!(
        "<section id=\"admin-conflict-banner\" role=\"alert\" aria-live=\"assertive\" data-conflict=\"false\"{}></section>",
        oob_attr(oob)
    )
}

pub fn render_admin_conflict_warning(conflict: &SaveConflict, revision: &str, oob: bool) -> String {
    format!(
        "<section id=\"admin-conflict-banner\" role=\"alert\" aria-live=\"assertive\" data-conflict=\"true\" data-telemetry=\"admin-save-conflict\"{}><p>Warning: a stale editor snapshot was saved and a newer revision was overwritten.</p><p>Overwritten revision <code>{}</code>; submitted revision <code>{}</code>; new revision <code>{}</code>.</p><p><button type=\"button\" data-action=\"reload-latest\">Reload latest</button><button type=\"button\" data-action=\"continue-editing\">Continue editing</button></p></section>",
        oob_attr(oob),
        escape_html(&conflict.persisted_revision),
        escape_html(&conflict.submitted_revision),
        escape_html(revision)
    )
}

pub fn render_revision_input(revision: &str, oob: bool) -> String {
    format!(
        "<input id=\"last_known_revision\" name=\"last_known_revision\" type=\"hidden\" value=\"{}\"{} />",
        escape_html(revision),
        oob_attr(oob)
    )
}

pub fn render_admin_save_fragments(outcome: &SaveOutcome) -> String {
    let hint = if outcome.conflict.is_some() {
        " Saved from a stale snapshot."
    } else {
        ""
    };
    let status = format!(
        "<section id=\"admin-status-banner\" aria-live=\"polite\">Saved revision <code>{}</code>.{hint}</section>",
        escape_html(&outcome.revision)
    );
    let conflict = outcome.conflict.as_ref().map_or_else(
        || render_admin_conflict_clear(true),
        |value| render_admin_conflict_warning(value, &outcome.revision, true),
    );
    format!(
        "{status}{conflict}{}",
        render_revision_input(&outcome.revision, true)
    )
}

pub fn render_admin_save_hx_trigger(outcome: &SaveOutcome) -> String {
    if let Some(conflict) = &outcome.conflict {
        return json!({
            "admin-save": { "revision": &outcome.revision, "conflict": true },
            "admin-save-conflict": {
                "marker": "stale-overwrite",
                "persisted_revision": &conflict.persisted_revision,
                "submitted_revision": &conflict.submitted_revision,
                "revision": &outcome.revision
            }
        })
        .to_string();
    }
    json!({ "admin-save": { "revision": &outcome.revision, "conflict": false } }).to_string()
}

pub fn render_admin_editor_pane(
    slug: &str,
    title: Option<&str>,
    body: &str,
    private: bool,
    revision: &str,
    oob: bool,
) -> String {
    let mut html = build_editor_form_html(slug, title.unwrap_or_default(), body, private, revision);
    if oob {
        let pos = html.find("<section").unwrap();
        html.insert_str(pos + 8, " hx-swap-oob=\"outerHTML\"");
    }
    html
}
