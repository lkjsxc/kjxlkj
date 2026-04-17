use crate::core::generate_id;
use crate::error::AppError;
use crate::web::db::MediaFamily;
use std::path::Path;

pub fn detect_media_family(content_type: &str, filename: &str) -> Result<MediaFamily, AppError> {
    if is_supported_file(content_type, filename) {
        return Ok(MediaFamily::File);
    }
    if content_type.starts_with("image/") {
        return Ok(MediaFamily::Image);
    }
    if content_type.starts_with("video/") {
        return Ok(MediaFamily::Video);
    }
    match extension(filename).as_deref() {
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "svg") => Ok(MediaFamily::Image),
        Some(
            "mp4" | "webm" | "mov" | "m4v" | "ogg" | "ogv" | "mkv" | "avi" | "wmv" | "mpeg" | "mpg"
            | "3gp",
        ) => Ok(MediaFamily::Video),
        _ => Err(AppError::InvalidRequest(
            "unsupported media type; use an image, video, or supported file".to_string(),
        )),
    }
}

pub fn initial_body(filename: &str) -> String {
    format!("# {}", pretty_stem(filename))
}

pub fn embed_markdown(media_ref: &str, family: MediaFamily, filename: &str) -> String {
    match family {
        MediaFamily::Image => format!("![](/{media_ref}/file)"),
        MediaFamily::Video => format!(r#"<video controls src="/{media_ref}/file"></video>"#),
        MediaFamily::File => format!("[{}](/{media_ref})", escape_link_label(filename)),
    }
}

pub fn object_key(id: &str, filename: &str) -> String {
    format!("media/{id}/{}-{}", generate_id(), safe_name(filename))
}

fn safe_name(filename: &str) -> String {
    filename
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ".-_".contains(ch) {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn pretty_stem(filename: &str) -> String {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Untitled media");
    let words = stem
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>();
    if words.is_empty() {
        "Untitled media".to_string()
    } else {
        words.join(" ")
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
}

fn is_supported_file(content_type: &str, filename: &str) -> bool {
    content_type.to_ascii_lowercase().contains("heic")
        || content_type.to_ascii_lowercase().contains("heif")
        || matches!(extension(filename).as_deref(), Some("heic" | "heif"))
}

fn escape_link_label(value: &str) -> String {
    value.replace('\\', r"\\").replace(']', r"\]")
}
