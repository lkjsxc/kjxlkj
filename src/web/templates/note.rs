//! Note page template

use super::layout::{base, html_escape, render_markdown, shell_page};
use super::model::NoteChrome;
use super::note_shell::note_rail;
use crate::web::db::Record;

const EDITOR_JS: &str = include_str!("editor.js");

pub fn note_page(record: &Record, chrome: &NoteChrome, is_admin: bool) -> String {
    let rail = note_rail(chrome, is_admin, &format!("/{}", record.slug));
    let extra_head = if is_admin {
        r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/simplemde/latest/simplemde.min.css">
<script src="https://cdn.jsdelivr.net/simplemde/latest/simplemde.min.js"></script>"#
    } else {
        ""
    };
    let editor = if is_admin {
        format!(
            r#"<section class="surface editor-surface">
<div class="editor-toolbar-row">
<label class="check-row" for="public-toggle">
<input type="checkbox" id="public-toggle" {} onchange="togglePublic()">
<span>Public</span>
</label>
<p class="visibility-hint" id="visibility-hint">{}</p>
</div>
<textarea id="editor">{}</textarea>
</section>
<script>
var currentSlug = "{}";
var isPrivate = {};
var simplemde = new SimpleMDE({{ element: document.getElementById("editor"), spellChecker: false }});
simplemde.codemirror.on("blur", function() {{ saveNote(); }});
{}
syncVisibilityHint();
</script>"#,
            if record.is_private { "" } else { "checked" },
            visibility_hint(record.is_private),
            html_escape(&record.body),
            record.slug,
            record.is_private,
            EDITOR_JS
        )
    } else {
        format!(
            r#"<section class="surface prose">{}</section>"#,
            render_markdown(&record.body)
        )
    };
    let content = format!(
        r#"<header class="page-head">
<div>
<p class="eyebrow">{}</p>
<h1>{}</h1>
</div>
<div class="page-meta">
<span class="status-pill">{}</span>
<small>{}</small>
</div>
</header>
{}
<footer class="page-tail">
<a href="{}" class="btn">History</a>
<span id="save-status" class="save-status"></span>
</footer>"#,
        if is_admin {
            "Admin note"
        } else {
            "Public note"
        },
        chrome.title,
        chrome.visibility,
        chrome.slug,
        editor,
        chrome.history_href
    );
    base(
        &chrome.title,
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &rail,
            &content,
            "note-page",
        ),
        extra_head,
        "",
    )
}

fn visibility_hint(is_private: bool) -> &'static str {
    if is_private {
        "Admin-only"
    } else {
        "Guest-readable"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::{HistoryLink, NoteChrome};
    use chrono::Utc;

    fn sample_record() -> Record {
        Record {
            slug: "demo-note".to_string(),
            body: "# Demo\n\nBody".to_string(),
            is_private: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn sample_chrome() -> NoteChrome {
        NoteChrome {
            title: "Demo".to_string(),
            slug: "demo-note".to_string(),
            created_at: "March 25, 2026 at 01:34 AM".to_string(),
            updated_at: "March 25, 2026 at 01:35 AM".to_string(),
            visibility: "Public",
            previous: None,
            next: None,
            history: vec![HistoryLink {
                href: "/demo-note/history/1".to_string(),
                label: "Revision 1".to_string(),
                meta: "March 24, 2026 at 08:00 PM".to_string(),
                status: "Public",
                active: false,
            }],
            history_href: "/demo-note/history".to_string(),
        }
    }

    #[test]
    fn guest_note_page_uses_shell_without_editor() {
        let html = note_page(&sample_record(), &sample_chrome(), false);
        assert!(html.contains("data-menu-toggle"));
        assert!(html.contains("/demo-note/history"));
        assert!(!html.contains("SimpleMDE"));
        assert!(!html.contains("public-toggle"));
    }

    #[test]
    fn admin_note_page_shows_public_checkbox() {
        let html = note_page(&sample_record(), &sample_chrome(), true);
        assert!(html.contains("public-toggle"));
        assert!(html.contains("Dashboard"));
        assert!(html.contains("Delete note"));
    }
}
