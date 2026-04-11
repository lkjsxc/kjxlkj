//! Media href helpers for HTML view models

use crate::media::MediaVariants;
use crate::web::db::{MediaFamily, Resource, ResourceSnapshot};
use crate::web::view::file_href;

pub fn card_file_href(resource: &Resource) -> String {
    current_variant_href(resource, &["card"]).unwrap_or_else(|| file_href(resource))
}

pub fn display_file_href(resource: &Resource) -> String {
    if matches!(resource.media_family, Some(MediaFamily::Image)) {
        return current_variant_href(resource, &["display", "card"])
            .unwrap_or_else(|| file_href(resource));
    }
    file_href(resource)
}

pub fn snapshot_display_file_href(snapshot: &ResourceSnapshot) -> String {
    let href = format!("/{}/file", snapshot.id);
    if matches!(snapshot.media_family, Some(MediaFamily::Image))
        && snapshot_variant_href(snapshot, &["display", "card"]).is_some()
    {
        return snapshot_variant_href(snapshot, &["display", "card"]).unwrap_or(href);
    }
    href
}

pub fn social_card_href(resource: &Resource) -> Option<String> {
    match resource.media_family {
        Some(MediaFamily::Image) => current_variant_href(resource, &["display", "card"]),
        Some(MediaFamily::Video) => current_variant_href(resource, &["card"]),
        None => None,
    }
}

pub fn snapshot_poster_href(snapshot: &ResourceSnapshot) -> Option<String> {
    snapshot_variant_href(snapshot, &["poster"])
}

pub fn poster_href(resource: &Resource) -> Option<String> {
    current_variant_href(resource, &["poster"])
}

fn current_variant_href(resource: &Resource, variants: &[&str]) -> Option<String> {
    variant_href(
        &file_href(resource),
        resource.media_variants.as_ref(),
        variants,
    )
}

fn snapshot_variant_href(snapshot: &ResourceSnapshot, variants: &[&str]) -> Option<String> {
    variant_href(
        &format!("/{}/file", snapshot.id),
        snapshot.media_variants.as_ref(),
        variants,
    )
}

fn variant_href(
    base_href: &str,
    variants: Option<&MediaVariants>,
    names: &[&str],
) -> Option<String> {
    names.iter().find_map(|name| {
        variants
            .and_then(|variants| variants.get(name))
            .map(|_| format!("{base_href}?variant={name}"))
    })
}
