//! Shared summary-card helpers

use super::layout::html_escape;

pub(crate) fn linked_card(
    href: &str,
    attrs: &str,
    classes: &str,
    body: &str,
    meta: &str,
) -> String {
    card_shell("a", Some(href), attrs, classes, body, meta)
}

pub(crate) fn static_card(attrs: &str, classes: &str, body: &str, meta: &str) -> String {
    card_shell("article", None, attrs, classes, body, meta)
}

pub(crate) fn card_body(title: &str, summary: &str) -> String {
    format!(
        r#"<div class="card-body"><p class="card-title">{}</p><p class="card-summary">{}</p></div>"#,
        html_escape(title),
        html_escape(summary)
    )
}

pub(crate) fn card_meta(badges: &str, lines: &str) -> String {
    format!(
        r#"<div class="card-meta{}"><div class="card-badges{}">{}</div>{}</div>"#,
        if badges.is_empty() && lines.is_empty() {
            " card-meta-empty"
        } else {
            ""
        },
        if badges.is_empty() {
            " card-badges-empty"
        } else {
            ""
        },
        badges,
        lines
    )
}

pub(crate) fn meta_line(label: &str, value: &str) -> String {
    format!(
        r#"<small><span>{}</span>{}</small>"#,
        html_escape(label),
        value
    )
}

pub(crate) fn status_pill(label: &str, extra_classes: &str) -> String {
    format!(
        r#"<span class="status-pill{}">{}</span>"#,
        if extra_classes.is_empty() {
            String::new()
        } else {
            format!(" {}", html_escape(extra_classes))
        },
        html_escape(label)
    )
}

fn card_shell(
    tag: &str,
    href: Option<&str>,
    attrs: &str,
    classes: &str,
    body: &str,
    meta: &str,
) -> String {
    let class_attr = if classes.is_empty() {
        "index-card note-row".to_string()
    } else {
        format!("index-card note-row {classes}")
    };
    href.map_or_else(
        || format!(r#"<{tag} class="{class_attr}"{attrs}>{body}{meta}</{tag}>"#),
        |href| format!(r#"<{tag} href="{href}" class="{class_attr}"{attrs}>{body}{meta}</{tag}>"#),
    )
}
