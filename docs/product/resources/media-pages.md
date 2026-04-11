# Media Page Contract

## Create Flow

- The canonical UI ingest path is `Upload media` from a live note editor.
- Agent and automation callers may still create media directly through `POST /resources/media`.
- Every media create requires one uploaded image or video file.
- Uploads may include common browser and desktop media extensions such as `.png`, `.jpg`, `.webp`, `.gif`, `.svg`, `.heic`, `.heif`, `.mp4`, `.webm`, `.mov`, `.m4v`, `.mkv`, `.ogv`, `.avi`, `.wmv`, `.mpeg`, `.mpg`, and `.3gp`.
- Direct media create may also accept optional alias, visibility, and favorite state.
- The initial media `body` is seeded from the uploaded filename stem as a `# Heading`.
- Image uploads create WebP variants when the server can decode the image.
- Video uploads create a first-frame WebP card variant and poster when the server can decode the video.

## Guest View

- Media pages render the primary image or video player above the Markdown body.
- Image media pages prefer a derived WebP display image when one exists, especially for originals that browsers tend to download instead of paint inline.
- Guest media pages render the same body HTML path used by note pages and admin preview.
- Public media pages are indexable under the same discovery rules as public live notes.

## Admin View

- Admin media pages keep the shared shell rail and current-resource chrome.
- The page shows the current image or video preview, file metadata, Markdown body editor, alias, favorite, and visibility controls.
- The primary preview surface does not prepend `Current file` copy.
- Markdown body edits autosave like note edits.
- Existing media binaries do not change from the media edit page.
- Existing derivative metadata does not change from the media edit page.

## Immutability

- Existing media binaries are immutable after creation.
- Existing derivatives are immutable after creation.
- Updating media Markdown, alias, visibility, or favorite state does not replace the binary file.
- New uploads create new media resources instead of replacing files on older media pages.
- First-frame poster generation applies to new media uploads only.
- Older saved snapshots keep their original object references.
- Later note edits do not rewrite earlier media objects or earlier note embed text.
