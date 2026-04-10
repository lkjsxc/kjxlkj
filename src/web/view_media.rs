//! Media href helpers for HTML view models

use crate::web::db::{MediaFamily, Resource, ResourceSnapshot};
use crate::web::view::file_href;

pub fn card_file_href(resource: &Resource) -> String {
    match resource.media_family {
        Some(MediaFamily::Image) if has_variant(resource, "card") => {
            format!("{}?variant=card", file_href(resource))
        }
        Some(MediaFamily::Video) if has_variant(resource, "poster") => {
            format!("{}?variant=poster", file_href(resource))
        }
        _ => file_href(resource),
    }
}

pub fn display_file_href(resource: &Resource) -> String {
    if matches!(resource.media_family, Some(MediaFamily::Image)) && has_variant(resource, "display")
    {
        return format!("{}?variant=display", file_href(resource));
    }
    file_href(resource)
}

pub fn snapshot_display_file_href(snapshot: &ResourceSnapshot) -> String {
    let href = format!("/{}/file", snapshot.id);
    if matches!(snapshot.media_family, Some(MediaFamily::Image))
        && snapshot_has_variant(snapshot, "display")
    {
        return format!("{href}?variant=display");
    }
    href
}

pub fn snapshot_poster_href(snapshot: &ResourceSnapshot) -> Option<String> {
    snapshot_has_variant(snapshot, "poster")
        .then(|| format!("/{}/file?variant=poster", snapshot.id))
}

pub fn poster_href(resource: &Resource) -> Option<String> {
    has_variant(resource, "poster").then(|| format!("{}?variant=poster", file_href(resource)))
}

fn has_variant(resource: &Resource, variant: &str) -> bool {
    resource
        .media_variants
        .as_ref()
        .and_then(|variants| variants.get(variant))
        .is_some()
}

fn snapshot_has_variant(snapshot: &ResourceSnapshot, variant: &str) -> bool {
    snapshot
        .media_variants
        .as_ref()
        .and_then(|variants| variants.get(variant))
        .is_some()
}
