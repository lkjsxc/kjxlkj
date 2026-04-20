use super::{looks_like_id, normalize_alias};

pub fn replace_local_resource_cards(html: &str) -> String {
    let mut rest = html;
    let mut output = String::new();
    let marker = "<p><a href=\"";
    while let Some(start) = rest.find(marker) {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start + marker.len()..];
        let Some(href_end) = after_marker.find('"') else {
            output.push_str(&rest[start..]);
            return output;
        };
        let href = &after_marker[..href_end];
        let after_href = &after_marker[href_end..];
        let Some(label_start) = after_href.find("\">") else {
            output.push_str(&rest[start..]);
            return output;
        };
        let label = &after_href[label_start + 2..];
        let Some(label_end) = label.find("</a></p>") else {
            output.push_str(&rest[start..]);
            return output;
        };
        let block_len = marker.len() + href_end + label_start + 2 + label_end + "</a></p>".len();
        if is_local_page_href(href) {
            output.push_str(&local_page_card(href, &label[..label_end]));
        } else if is_local_file_href(href) {
            output.push_str(&local_file_card(href, &label[..label_end]));
        } else {
            output.push_str(&rest[start..start + block_len]);
        }
        rest = &rest[start + block_len..];
    }
    output.push_str(rest);
    output
}

pub fn poster_href(href: &str) -> String {
    variant_href(href, "poster")
}

pub fn variant_href(href: &str, variant: &str) -> String {
    if href.contains('?') {
        format!("{href}&variant={variant}")
    } else {
        format!("{href}?variant={variant}")
    }
}

pub fn is_local_file_href(href: &str) -> bool {
    resource_link_parts(href).is_some_and(|(_, is_file)| is_file)
}

pub fn local_url_card(href: &str, label: &str) -> Option<String> {
    if is_local_page_href(href) {
        Some(local_page_card(href, label))
    } else if is_local_file_href(href) {
        Some(local_file_card(href, label))
    } else {
        None
    }
}

pub fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn is_local_page_href(href: &str) -> bool {
    resource_link_parts(href).is_some_and(|(_, is_file)| !is_file)
}

fn local_file_card(href: &str, label: &str) -> String {
    format!(
        r#"<div class="local-url-card"><a href="{0}"><img src="{1}" alt="" loading="lazy"><span>{2}</span></a></div>"#,
        escape_attr(href),
        escape_attr(&variant_href(href, "card")),
        html_text(label),
    )
}

fn local_page_card(href: &str, label: &str) -> String {
    format!(
        r#"<div class="local-url-card local-url-card-page"><a href="{0}"><span>{1}</span></a></div>"#,
        escape_attr(href),
        html_text(label),
    )
}

fn resource_link_parts(href: &str) -> Option<(&str, bool)> {
    if !href.starts_with('/') || href.starts_with("//") {
        return None;
    }
    let path = href.split('?').next().unwrap_or("").trim_start_matches('/');
    let segments = path.split('/').collect::<Vec<_>>();
    match segments.as_slice() {
        [reference] if valid_resource_reference(reference) => Some((reference, false)),
        [reference, "file"] if valid_resource_reference(reference) => Some((reference, true)),
        _ => None,
    }
}

fn valid_resource_reference(value: &str) -> bool {
    looks_like_id(value) || normalize_alias(Some(value)).is_ok()
}

pub fn html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
