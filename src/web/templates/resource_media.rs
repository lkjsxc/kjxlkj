use super::layout::html_escape;
use crate::web::db::{MediaFamily, Resource, ResourceSnapshot};
use crate::web::view;

pub fn current_media_block(resource: &Resource) -> String {
    media_surface(
        "Current file",
        resource.media_family,
        &view::file_href(resource),
        &resource.title,
    )
}

pub fn admin_media_panel(resource: &Resource) -> String {
    let file_href = view::file_href(resource);
    format!(
        r#"{}<section class="surface resource-surface">
<div class="editor-meta-grid">
<div class="editor-url-card editor-field-card"><small>File URL</small><a href="{file_href}">{file_href}</a></div>
<div class="editor-url-card editor-field-card"><small>File metadata</small><span>{} · {} · {}</span></div>
</div>
</section>"#,
        current_media_block(resource),
        html_escape(
            resource
                .original_filename
                .as_deref()
                .unwrap_or("upload.bin")
        ),
        html_escape(
            resource
                .content_type
                .as_deref()
                .unwrap_or("application/octet-stream")
        ),
        format_bytes(resource.byte_size.unwrap_or(0)),
    )
}

pub fn snapshot_media_block(snapshot: &ResourceSnapshot) -> String {
    media_surface(
        "Saved file",
        snapshot.media_family,
        &format!("/{}/file", snapshot.id),
        &snapshot.title,
    )
}

fn media_surface(
    label: &str,
    media_family: Option<MediaFamily>,
    href: &str,
    title: &str,
) -> String {
    format!(
        r#"<section class="surface resource-surface media-surface"><small>{label}</small>{}</section>"#,
        media_markup(media_family, href, title)
    )
}

fn media_markup(media_family: Option<MediaFamily>, href: &str, title: &str) -> String {
    match media_family.unwrap_or(MediaFamily::Image) {
        MediaFamily::Image => format!(
            r#"<img src="{}" alt="{}" style="width:100%;height:auto;display:block;">"#,
            html_escape(href),
            html_escape(title),
        ),
        MediaFamily::Video => format!(
            r#"<video controls preload="metadata" src="{}" style="width:100%;height:auto;display:block;"></video>"#,
            html_escape(href),
        ),
    }
}

fn format_bytes(bytes: i64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
