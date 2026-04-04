//! Shared page headers and section wrappers

use super::layout::html_escape;

pub fn page_header(title: &str, actions: Option<&str>, class_name: &str) -> String {
    let actions = actions
        .filter(|value| !value.is_empty())
        .map(|value| format!(r#"<div class="page-actions">{value}</div>"#))
        .unwrap_or_default();
    format!(
        r#"<header class="page-head {class_name}">
<div class="page-title-stack"><h1>{}</h1></div>
{actions}
</header>"#,
        html_escape(title),
    )
}

pub fn section(title: &str, body: &str, class_name: &str) -> String {
    section_with_actions(title, None, body, class_name)
}

pub fn section_with_actions(
    title: &str,
    actions: Option<&str>,
    body: &str,
    class_name: &str,
) -> String {
    let actions = actions
        .filter(|value| !value.is_empty())
        .map(|value| format!(r#"<div class="section-actions">{value}</div>"#))
        .unwrap_or_default();
    format!(
        r#"<section class="section-block {class_name}">
<div class="section-head"><h2>{}</h2>{actions}</div>
    {body}
</section>"#,
        html_escape(title),
    )
}
