//! Note page template

use super::layout::{base, html_escape, shell_page};
use super::model::NoteChrome;
use super::note_shell::note_rail;
use crate::core::render_markdown;
use crate::web::db::Record;

const EDITOR_JS: &str = include_str!("editor.js");
const EDITOR_SHORTCUTS_JS: &str = include_str!("editor_shortcuts.js");
const TOAST_UI_ROOT: &str = "/assets/vendor/toastui/3.2.2";

pub fn note_page(record: &Record, chrome: &NoteChrome, is_admin: bool) -> String {
    let extra_head = if is_admin {
        editor_head()
    } else {
        String::new()
    };
    let rail = note_rail(chrome, is_admin, &chrome.current_href);
    let editor = if is_admin {
        format!(
            r#"{}<script>
var currentId = "{}";
var isPrivate = {};
{}
{}
initEditor();
</script>"#,
            editor_surface(record),
            record.id,
            record.is_private,
            EDITOR_SHORTCUTS_JS,
            EDITOR_JS
        )
    } else {
        format!(
            r#"<section class="surface note-surface prose">{}</section>"#,
            render_markdown(&record.body)
        )
    };
    let content = format!(
        r#"<header class="page-head">
<div class="page-title-stack">
<p class="eyebrow">{}</p>
<h1 data-live-title>{}</h1>
<p class="page-summary">{}</p>
<div class="title-tags"><span class="status-pill" data-live-visibility>{}</span></div>
</div>
<div class="page-meta">
<small>Created {}</small>
<small>Updated {}</small>
</div>
</header>
{}"#,
        if is_admin { "Admin note" } else { "Note" },
        chrome.title,
        if is_admin {
            "Rendered Markdown stays editable while Markdown remains canonical storage."
        } else {
            "Rendered Markdown only."
        },
        chrome.visibility,
        chrome.created_at,
        chrome.updated_at,
        editor
    );
    base(
        &chrome.title,
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &rail,
            &content,
            "note-page",
        ),
        &extra_head,
        "",
    )
}

fn editor_head() -> String {
    format!(
        r#"<link rel="stylesheet" href="{TOAST_UI_ROOT}/toastui-editor.min.css">
<link rel="stylesheet" href="{TOAST_UI_ROOT}/toastui-editor-dark.min.css">
<script src="{TOAST_UI_ROOT}/toastui-editor-all.min.js"></script>"#
    )
}

fn editor_surface(record: &Record) -> String {
    format!(
        r#"<section class="surface note-surface editor-shell">
<div class="editor-toolbar-row">
<label class="check-row" for="public-toggle">
<input type="checkbox" id="public-toggle" {} onchange="togglePublic()">
<span>Public</span>
</label>
<span id="save-error" class="save-error" hidden aria-live="polite">Save failed. Retry on the next change.</span>
</div>
<div id="editor-root" class="toast-host"></div>
<textarea id="editor-source" hidden>{}</textarea>
<textarea id="editor-fallback" class="note-editor" hidden>{}</textarea>
</section>"#,
        if record.is_private { "" } else { "checked" },
        html_escape(&record.body),
        html_escape(&record.body),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::NoteChrome;
    use chrono::Utc;

    fn sample_record() -> Record {
        Record {
            id: "Q29udHJhY3RSdW50aW1lMQ".to_string(),
            title: "Demo".to_string(),
            summary: "Body".to_string(),
            body: "# Demo\n\nBody".to_string(),
            is_private: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn sample_chrome() -> NoteChrome {
        NoteChrome {
            id: "Q29udHJhY3RSdW50aW1lMQ".to_string(),
            title: "Demo".to_string(),
            current_href: "/Q29udHJhY3RSdW50aW1lMQ".to_string(),
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            visibility: "Public",
            previous: None,
            next: None,
            history_href: "/Q29udHJhY3RSdW50aW1lMQ/history".to_string(),
        }
    }

    #[test]
    fn guest_note_page_hides_editor_and_history_footer_button() {
        let html = note_page(&sample_record(), &sample_chrome(), false);
        assert!(html.contains("shell-rail"));
        assert!(!html.contains("editor-root"));
        assert!(!html.contains("Rich mode"));
        assert!(!html.contains("Q29udHJhY3RSdW50aW1lMQ</span>"));
    }

    #[test]
    fn admin_note_page_renders_single_mode_workspace() {
        let html = note_page(&sample_record(), &sample_chrome(), true);
        assert!(html.contains("public-toggle"));
        assert!(html.contains("editor-root"));
        assert!(html.contains(TOAST_UI_ROOT));
        assert!(html.contains("height: 'auto'"));
        assert!(html.contains("hideModeSwitch: true"));
        assert!(!html.contains("Rich mode"));
        assert!(!html.contains("Text mode"));
        assert!(!html.contains("save-status"));
        assert!(html.contains("All history"));
        assert!(!html.contains("uicdn.toast.com"));
    }
}
