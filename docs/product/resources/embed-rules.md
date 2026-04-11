# Embed Rules

## Images

- Standard Markdown image syntax is the canonical inline image path.
- The canonical current-file pattern is `![](/{ref}/file)`.
- Snapshot-stable image embeds use `![](/{snapshot_id}/file)`.
- Rendering may serve a smaller WebP display derivative through generated HTML while preserving the authored Markdown URL.
- Admin preview and guest rendering must display the same image output for the same Markdown.

## Videos

- The canonical inline video path is safe HTML video markup.
- The canonical current-file pattern is `<video controls src="/{ref}/file"></video>`.
- Snapshot-stable video embeds use `<video controls src="/{snapshot_id}/file"></video>`.
- Rendering may add a stored WebP poster to local video embeds while preserving the authored video URL.
- Inline videos must stay no wider than the prose surface and preserve their aspect ratio.
- Admin preview and guest rendering must display the same video output for the same Markdown.

## Safety and Failure Rules

- Markdown rendering must sanitize unsafe HTML and script execution paths.
- The sanitizer may allow only the HTML needed for normal Markdown output plus safe media embeds.
- Missing or inaccessible targets should fail as normal broken media rather than mutating the surrounding Markdown.
- Notes may link to media pages or media files directly when inline display is not desired.
