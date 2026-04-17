use super::layout::html_escape;
use crate::web::db::{MediaFamily, Resource, ResourceSnapshot};
use crate::web::view;
use crate::web::view_media;

pub fn current_media_block(resource: &Resource) -> String {
    media_surface(
        None,
        MediaSurface {
            media_family: resource.media_family,
            href: &view_media::display_file_href(resource),
            raw_href: &view::file_href(resource),
            download_name: resource.original_filename.as_deref(),
            content_type: resource.content_type.as_deref(),
            byte_size: resource.byte_size,
            poster_href: view_media::poster_href(resource).as_deref(),
            title: &resource.title,
        },
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
        MediaSurface {
            media_family: snapshot.media_family,
            href: &view_media::snapshot_display_file_href(snapshot),
            raw_href: &format!("/{}/file", snapshot.id),
            download_name: snapshot.original_filename.as_deref(),
            content_type: snapshot.content_type.as_deref(),
            byte_size: snapshot.byte_size,
            poster_href: view_media::snapshot_poster_href(snapshot).as_deref(),
            title: &snapshot.title,
        },
    )
}

struct MediaSurface<'a> {
    media_family: Option<MediaFamily>,
    href: &'a str,
    raw_href: &'a str,
    download_name: Option<&'a str>,
    content_type: Option<&'a str>,
    byte_size: Option<i64>,
    poster_href: Option<&'a str>,
    title: &'a str,
}

fn media_surface(label: Option<&str>, surface: MediaSurface<'_>) -> String {
    let label = label
        .map(|value| format!("<small>{value}</small>"))
        .unwrap_or_default();
    format!(
        r#"<section class="surface resource-surface media-surface">{}{}<div class="media-surface-actions"><a href="{}" class="btn" download="{}">Download original</a></div></section>"#,
        label,
        media_markup(&surface),
        html_escape(surface.raw_href),
        html_escape(surface.download_name.unwrap_or("download.bin")),
    )
}

fn media_markup(surface: &MediaSurface<'_>) -> String {
    match surface.media_family.unwrap_or(MediaFamily::Image) {
        MediaFamily::Image => format!(
            r#"<img src="{}" alt="{}" fetchpriority="high" style="width:100%;height:auto;display:block;">"#,
            html_escape(surface.href),
            html_escape(surface.title),
        ),
        MediaFamily::Video => format!(
            r#"<video controls preload="metadata"{} src="{}" style="width:100%;height:auto;display:block;"></video>"#,
            surface
                .poster_href
                .map(|href| format!(r#" poster="{}""#, html_escape(href)))
                .unwrap_or_default(),
            html_escape(surface.href),
        ),
        MediaFamily::File => format!(
            r#"<div class="file-media-panel">
<p class="card-title">{}</p>
<p class="card-summary">{}</p>
<a href="{}" class="btn">Open raw file</a>
</div>"#,
            html_escape(surface.download_name.unwrap_or(surface.title)),
            html_escape(&file_summary(surface.content_type, surface.byte_size)),
            html_escape(surface.raw_href),
        ),
    }
}

fn file_summary(content_type: Option<&str>, byte_size: Option<i64>) -> String {
    format!(
        "{} · {}",
        content_type.unwrap_or("application/octet-stream"),
        format_bytes(byte_size.unwrap_or(0)),
    )
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
