//! Context-aware Markdown rendering for HTML pages

use crate::core::looks_like_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool, Resource};
use crate::web::markdown_cards;
use crate::web::markdown_external::external_embed_cache;
use crate::web::view;
use std::collections::HashMap;

pub async fn render_markdown_page(
    pool: &DbPool,
    body: &str,
    current_resource_id: Option<&str>,
    is_admin: bool,
    public_base_url: Option<&str>,
    google_maps_embed_api_key: Option<&str>,
) -> Result<String, AppError> {
    let external_urls = crate::core::external_embed_urls(body, public_base_url);
    let external_cache = external_embed_cache(pool, &external_urls).await?;
    let html = crate::core::render_markdown_with_options(
        body,
        crate::core::MarkdownOptions {
            public_base_url,
            google_maps_embed_api_key,
            external_embed_cache: Some(&external_cache),
        },
    );
    let html = decorate_local_images(pool, &html, current_resource_id, is_admin).await?;
    markdown_cards::decorate_local_cards(pool, &html, is_admin).await
}

async fn decorate_local_images(
    pool: &DbPool,
    html: &str,
    current_resource_id: Option<&str>,
    is_admin: bool,
) -> Result<String, AppError> {
    let mut targets = HashMap::new();
    for src in local_image_sources(html) {
        if let std::collections::hash_map::Entry::Vacant(entry) = targets.entry(src) {
            let target =
                resolve_image_target(pool, entry.key(), current_resource_id, is_admin).await?;
            entry.insert(target);
        }
    }
    Ok(apply_image_targets(html, &targets))
}

fn local_image_sources(html: &str) -> Vec<String> {
    let mut rest = html;
    let mut sources = Vec::new();
    while let Some(start) = rest.find("<img") {
        let after_marker = &rest[start..];
        let Some(end) = after_marker.find('>') else {
            break;
        };
        let tag = &after_marker[..=end];
        if let Some(src) = attribute_value(tag, "src").filter(|src| is_local_file_href(src)) {
            sources.push(src.to_string());
        }
        rest = &after_marker[end + 1..];
    }
    sources
}

async fn resolve_image_target(
    pool: &DbPool,
    src: &str,
    current_resource_id: Option<&str>,
    is_admin: bool,
) -> Result<Option<String>, AppError> {
    let Some(reference) = local_file_reference(src) else {
        return Ok(None);
    };
    if !looks_like_id(reference) {
        if let Some(resource) = db::get_resource_by_ref(pool, reference).await? {
            return resolve_resource_target(pool, &resource, current_resource_id, is_admin)
                .await
                .map(Some);
        }
        return Ok(None);
    }
    if let Some(resource) = db::get_resource(pool, reference).await? {
        return resolve_resource_target(pool, &resource, current_resource_id, is_admin)
            .await
            .map(Some);
    }
    if let Some(target) = db::get_snapshot_target(pool, reference).await? {
        return resolve_resource_target(pool, &target.resource, current_resource_id, is_admin)
            .await
            .map(Some);
    }
    Ok(None)
}

async fn resolve_resource_target(
    pool: &DbPool,
    resource: &Resource,
    current_resource_id: Option<&str>,
    is_admin: bool,
) -> Result<String, AppError> {
    if let Some(owner_id) = resource.owner_note_id.as_deref() {
        if Some(owner_id) != current_resource_id {
            if let Some(owner) = db::get_resource(pool, owner_id)
                .await?
                .filter(|owner| is_admin || !owner.is_private)
            {
                return Ok(view::resource_href(&owner));
            }
        }
    }
    Ok(view::resource_href(resource))
}

fn apply_image_targets(html: &str, targets: &HashMap<String, Option<String>>) -> String {
    let mut rest = html;
    let mut output = String::new();
    while let Some(start) = rest.find("<img") {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start..];
        let Some(end) = after_marker.find('>') else {
            output.push_str(after_marker);
            return output;
        };
        let tag = &after_marker[..=end];
        let next = attribute_value(tag, "src")
            .and_then(|src| targets.get(src))
            .and_then(|target| target.as_deref())
            .map(|target| add_attribute(tag, "data-resource-image-href", target))
            .unwrap_or_else(|| tag.to_string());
        output.push_str(&next);
        rest = &after_marker[end + 1..];
    }
    output.push_str(rest);
    output
}

fn local_file_reference(href: &str) -> Option<&str> {
    let path = href.split('?').next()?.trim_start_matches('/');
    let mut segments = path.split('/');
    let reference = segments.next()?;
    matches!(segments.next(), Some("file")).then_some(reference)
}

fn is_local_file_href(href: &str) -> bool {
    href.starts_with('/') && !href.starts_with("//") && local_file_reference(href).is_some()
}

fn attribute_value<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!(r#"{name}=""#);
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

fn add_attribute(tag: &str, name: &str, value: &str) -> String {
    if tag.contains(&format!(r#"{name}=""#)) {
        return tag.to_string();
    }
    format!(
        "{} {}=\"{}\">",
        &tag[..tag.len().saturating_sub(1)],
        name,
        escape_attr(value),
    )
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
