pub fn inline_image_fallback_allowed(
    content_type: Option<&str>,
    original_filename: Option<&str>,
) -> bool {
    content_type.is_some_and(browser_safe_image_content_type)
        || content_type.is_none_or(|value| value == "application/octet-stream")
            && original_filename.is_some_and(browser_safe_image_filename)
}

fn browser_safe_image_content_type(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    value.starts_with("image/")
        && !value.contains("heic")
        && !value.contains("heif")
        && !value.contains("tiff")
}

fn browser_safe_image_filename(value: &str) -> bool {
    std::path::Path::new(value)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .is_some_and(|value| {
            matches!(
                value.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "avif"
            )
        })
}
