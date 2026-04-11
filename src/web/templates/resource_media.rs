use super::layout::html_escape;
use crate::web::db::{MediaFamily, Resource, ResourceSnapshot};
use crate::web::view;
use crate::web::view_media;

pub fn current_media_block(resource: &Resource) -> String {
    media_surface(
        None,
        resource.media_family,
        &view_media::display_file_href(resource),
        &view::file_href(resource),
        resource.original_filename.as_deref(),
        view_media::poster_href(resource).as_deref(),
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
        Some("Saved file"),
        snapshot.media_family,
        &view_media::snapshot_display_file_href(snapshot),
        &format!("/{}/file", snapshot.id),
        snapshot.original_filename.as_deref(),
        view_media::snapshot_poster_href(snapshot).as_deref(),
        &snapshot.title,
    )
}

fn media_surface(
    label: Option<&str>,
    media_family: Option<MediaFamily>,
    href: &str,
    raw_href: &str,
    download_name: Option<&str>,
    poster_href: Option<&str>,
    title: &str,
) -> String {
    let label = label
        .map(|value| format!("<small>{value}</small>"))
        .unwrap_or_default();
    format!(
        r#"<section class="surface resource-surface media-surface">{}{}<div class="media-surface-actions"><a href="{}" class="btn" download="{}">Download original</a></div></section>"#,
        label,
        media_markup(media_family, href, poster_href, title),
        html_escape(raw_href),
        html_escape(download_name.unwrap_or("download.bin")),
    )
}

fn media_markup(
    media_family: Option<MediaFamily>,
    href: &str,
    poster_href: Option<&str>,
    title: &str,
) -> String {
    match media_family.unwrap_or(MediaFamily::Image) {
        MediaFamily::Image => format!(
            r#"<img src="{}" alt="{}" fetchpriority="high" style="width:100%;height:auto;display:block;">"#,
            html_escape(href),
            html_escape(title),
        ),
        MediaFamily::Video => format!(
            r#"<video controls preload="metadata"{} src="{}" style="width:100%;height:auto;display:block;"></video>"#,
            poster_href
                .map(|href| format!(r#" poster="{}""#, html_escape(href)))
                .unwrap_or_default(),
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
