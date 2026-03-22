pub fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

pub fn build_editor_form_html(
    slug: &str,
    title: &str,
    body: &str,
    private_checked: bool,
    revision: &str,
) -> String {
    let checked = if private_checked { " checked" } else { "" };
    format!(
        "<section id=\"admin-editor-pane\"><form id=\"admin-editor-form\" method=\"post\" action=\"/admin/save\" hx-post=\"/admin/save\" hx-target=\"#admin-status-banner\" hx-swap=\"outerHTML\"><label for=\"slug\">Slug</label><input id=\"slug\" name=\"slug\" type=\"text\" value=\"{}\" /><label for=\"title\">Title</label><input id=\"title\" name=\"title\" type=\"text\" value=\"{}\" /><label for=\"body\">Body</label><textarea id=\"body\" name=\"body\" rows=\"16\">{}</textarea><label for=\"private\">Private</label><input id=\"private\" name=\"private\" type=\"checkbox\" value=\"true\"{} /><input id=\"last_known_revision\" name=\"last_known_revision\" type=\"hidden\" value=\"{}\" /><button type=\"submit\">Save</button><button id=\"admin-preview-button\" type=\"submit\" formaction=\"/admin/preview\" formmethod=\"post\" hx-post=\"/admin/preview\" hx-target=\"#admin-preview-pane\" hx-swap=\"outerHTML\">Preview</button></form></section>",
        escape_html(slug),
        escape_html(title),
        escape_html(body),
        checked,
        escape_html(revision)
    )
}

pub fn oob_attr(oob: bool) -> &'static str {
    if oob {
        " hx-swap-oob=\"outerHTML\""
    } else {
        ""
    }
}

pub fn render_admin_preview_empty(oob: bool) -> String {
    format!(
        "<section id=\"admin-preview-pane\"{}></section>",
        oob_attr(oob)
    )
}

pub fn render_admin_editor_placeholder(oob: bool) -> String {
    format!(
        "<section id=\"admin-editor-pane\"{}><p>No article selected.</p></section>",
        oob_attr(oob)
    )
}
