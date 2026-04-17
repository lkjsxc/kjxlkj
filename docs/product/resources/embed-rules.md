# Embed Rules

## Images

- Standard Markdown image syntax is the canonical inline image path.
- The canonical current-file pattern is `![](/{ref}/file)`.
- Snapshot-stable image embeds use `![](/{snapshot_id}/file)`.
- Rendering may serve a smaller WebP display derivative through generated HTML while preserving the authored Markdown URL.
- Generated HTML should prefer `variant=display` and then `variant=card` for local image embeds before using the raw original.
- Rendered local images remain clickable.
- Clicked local images prefer the immutable owner note page when one exists and it is not the current page.
- Clicking a local image on its own owner note page falls back to the media page.
- Media without an owner note fall back to the media page.
- Admin preview and guest rendering must display the same image output for the same Markdown.

## Videos

- The canonical inline video path is safe HTML video markup.
- The canonical current-file pattern is `<video controls src="/{ref}/file"></video>`.
- Snapshot-stable video embeds use `<video controls src="/{snapshot_id}/file"></video>`.
- Rendering may add a stored WebP poster to local video embeds while preserving the authored video URL.
- Inline videos must stay no wider than the prose surface and preserve their aspect ratio.
- Admin preview and guest rendering must display the same video output for the same Markdown.

## Files

- File-family media should not be embedded with Markdown image syntax.
- The canonical note-attachment pattern for file-family media is a page link such as `[capture.heic](/alias-or-id)`.
- Snapshot-stable file links may point at `/{snapshot_id}`.
- Direct raw-file links such as `/{ref}/file` remain valid when download-first Markdown is desired.
- Admin preview and guest rendering must apply the same local-card promotion rules to file-family links.

## Safety and Failure Rules

- Markdown rendering must sanitize unsafe HTML and script execution paths.
- The sanitizer may allow only the HTML needed for normal Markdown output plus safe media embeds.
- Missing or inaccessible targets should fail as normal broken media rather than mutating the surrounding Markdown.
- Notes may link to media pages or media files directly when inline display is not desired.
