use crate::core::looks_like_id;
use crate::error::AppError;
use crate::web::db::{self, DbPool, MediaFamily, Resource, ResourceSnapshot};
use crate::web::{templates, view, view_media};
use std::collections::HashMap;

pub async fn decorate_local_cards(
    pool: &DbPool,
    html: &str,
    is_admin: bool,
) -> Result<String, AppError> {
    let mut targets = HashMap::new();
    for href in local_card_hrefs(html) {
        if let std::collections::hash_map::Entry::Vacant(entry) = targets.entry(href) {
            let key = entry.key().to_string();
            entry.insert(resolve_card(pool, &key, is_admin).await?);
        }
    }
    Ok(apply_cards(html, &targets))
}

fn local_card_hrefs(html: &str) -> Vec<String> {
    let mut rest = html;
    let mut hrefs = Vec::new();
    while let Some(start) = rest.find(r#"<div class="local-url-card"#) {
        let after = &rest[start..];
        let Some(end) = after.find("</div>") else {
            break;
        };
        if let Some(href) = extract_href(&after[..end + 6]) {
            hrefs.push(href.to_string());
        }
        rest = &after[end + 6..];
    }
    hrefs
}

async fn resolve_card(
    pool: &DbPool,
    href: &str,
    is_admin: bool,
) -> Result<Option<String>, AppError> {
    let Some((reference, _is_file)) = local_reference(href) else {
        return Ok(None);
    };
    if !looks_like_id(reference) {
        return Ok(db::get_resource_by_ref(pool, reference)
            .await?
            .filter(|resource| is_admin || !resource.is_private)
            .map(|resource| live_card(&resource)));
    }
    if let Some(resource) = db::get_resource(pool, reference).await? {
        return Ok((is_admin || !resource.is_private).then(|| live_card(&resource)));
    }
    Ok(db::get_snapshot_target(pool, reference)
        .await?
        .filter(|target| is_admin || !target.snapshot.is_private)
        .map(|target| snapshot_card(&target.snapshot)))
}

fn apply_cards(html: &str, cards: &HashMap<String, Option<String>>) -> String {
    let mut rest = html;
    let mut output = String::new();
    while let Some(start) = rest.find(r#"<div class="local-url-card"#) {
        output.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = after.find("</div>") else {
            output.push_str(after);
            return output;
        };
        let block = &after[..end + 6];
        let href = extract_href(block);
        match href.and_then(|value| cards.get(value)) {
            Some(Some(card)) => output.push_str(card),
            Some(None) => output.push_str(&plain_link(href.unwrap_or(""))),
            None => output.push_str(block),
        }
        rest = &after[end + 6..];
    }
    output.push_str(rest);
    output
}

fn live_card(resource: &Resource) -> String {
    resource_card(CardView {
        href: view::resource_href(resource),
        id: resource.id.clone(),
        kind: view::kind_badge(resource.media_family),
        title: resource.title.clone(),
        summary: resource.summary.clone(),
        created: templates::render_time(&resource.created_at),
        updated: templates::render_time(&resource.updated_at),
        favorite: resource.is_favorite,
        media_href: media_card_href(resource.media_family, || {
            view_media::card_file_href(resource)
        }),
    })
}

fn snapshot_card(snapshot: &ResourceSnapshot) -> String {
    resource_card(CardView {
        href: view::snapshot_href(snapshot),
        id: snapshot.id.clone(),
        kind: view::kind_badge(snapshot.media_family),
        title: format!("Saved snapshot {}", snapshot.snapshot_number),
        summary: snapshot.summary.clone(),
        created: templates::render_time(&snapshot.created_at),
        updated: view::visibility_label(snapshot.is_private).to_string(),
        favorite: false,
        media_href: media_card_href(snapshot.media_family, || {
            format!("/{}/file?variant=card", snapshot.id)
        }),
    })
}

struct CardView<'a> {
    href: String,
    id: String,
    kind: &'a str,
    title: String,
    summary: String,
    created: String,
    updated: String,
    favorite: bool,
    media_href: Option<String>,
}

fn resource_card(card: CardView<'_>) -> String {
    let favorite = if card.favorite {
        pill("Favorite")
    } else {
        String::new()
    };
    let cover = card.media_href.map_or_else(String::new, |src| {
        format!(
            r#"<div class="card-cover"><img class="card-cover-media" src="{}" alt=""></div>"#,
            esc(&src)
        )
    });
    format!(
        r#"<a href="{}" class="index-card resource-row local-resource-card" data-note-id="{}" data-card-title="{}">{}<div class="card-body"><p class="card-title">{}</p><p class="card-summary">{}</p></div><div class="card-meta"><div class="card-badges">{}{}</div><small><span>Created</span>{}</small><small><span>Updated</span>{}</small></div></a>"#,
        esc(&card.href),
        esc(&card.id),
        esc(&card.title),
        cover,
        esc(&card.title),
        esc(&card.summary),
        pill(card.kind),
        favorite,
        card.created,
        card.updated
    )
}

fn media_card_href<F: FnOnce() -> String>(family: Option<MediaFamily>, build: F) -> Option<String> {
    matches!(family, Some(MediaFamily::Image | MediaFamily::Video)).then(build)
}

fn pill(label: &str) -> String {
    format!(r#"<span class="status-pill">{}</span>"#, esc(label))
}

fn plain_link(href: &str) -> String {
    format!(r#"<p><a href="{}">{}</a></p>"#, esc(href), esc(href))
}

fn extract_href(block: &str) -> Option<&str> {
    let marker = r#"<a href=""#;
    let start = block.find(marker)? + marker.len();
    let end = block[start..].find('"')?;
    Some(&block[start..start + end])
}

fn local_reference(href: &str) -> Option<(&str, bool)> {
    if !href.starts_with('/') || href.starts_with("//") {
        return None;
    }
    let path = href.split('?').next()?.trim_start_matches('/');
    let segments = path.split('/').collect::<Vec<_>>();
    match segments.as_slice() {
        [reference] => Some((reference, false)),
        [reference, "file"] => Some((reference, true)),
        _ => None,
    }
}

fn esc(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
