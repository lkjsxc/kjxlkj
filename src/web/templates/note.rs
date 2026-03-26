//! Note page template

use super::editor::editor_surface;
use super::layout::{base, shell_page};
use super::model::NoteChrome;
use super::note_shell::note_rail;
use crate::core::{editor_document, render_markdown};
use crate::web::db::Record;

const EDITOR_JS: &str = include_str!("editor.js");
const RICH_EDITOR_JS: &str = include_str!("rich_editor.js");

pub fn note_page(record: &Record, chrome: &NoteChrome, is_admin: bool) -> String {
    let rail = note_rail(chrome, is_admin, &chrome.current_href);
    let editor = if is_admin {
        let document = editor_document(&record.body);
        format!(
            r#"{}<script>
var currentId = "{}";
var isPrivate = {};
{}
{}
initEditor();
</script>"#,
            editor_surface(record, &document),
            record.id,
            record.is_private,
            RICH_EDITOR_JS,
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
        "",
        "",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::templates::{HistoryLink, NoteChrome, RecentLink};
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
            search_path: "/admin",
            created_at: "2026-03-26 08:34 UTC".to_string(),
            updated_at: "2026-03-26 08:35 UTC".to_string(),
            visibility: "Public",
            recent: vec![RecentLink {
                href: "/Q29udHJhY3RSdW50aW1lMQ".to_string(),
                title: "Demo".to_string(),
                updated_at: "2026-03-26 08:35 UTC".to_string(),
                visibility: Some("Public"),
            }],
            previous: None,
            next: None,
            history: vec![HistoryLink {
                href: "/Q29udHJhY3RSdW50aW1lMQ/history/1".to_string(),
                label: "Revision 1".to_string(),
                created_at: "2026-03-26 08:00 UTC".to_string(),
                status: "Public",
                active: false,
            }],
            history_href: "/Q29udHJhY3RSdW50aW1lMQ/history".to_string(),
        }
    }

    #[test]
    fn guest_note_page_hides_editor_and_history_footer_button() {
        let html = note_page(&sample_record(), &sample_chrome(), false);
        assert!(html.contains("shell-rail"));
        assert!(!html.contains("textarea id=\"editor\""));
        assert!(!html.contains("<footer class=\"page-tail\">"));
        assert!(!html.contains("Q29udHJhY3RSdW50aW1lMQ</span>"));
    }

    #[test]
    fn admin_note_page_uses_plain_editor() {
        let html = note_page(&sample_record(), &sample_chrome(), true);
        assert!(html.contains("public-toggle"));
        assert!(html.contains("data-mode-button=\"rich\""));
        assert!(!html.contains("SimpleMDE"));
        assert!(!html.contains("Guest-readable"));
        assert!(!html.contains("editor-toolbar-row"));
    }
}
