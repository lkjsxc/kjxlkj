# Media Page Contract

## Create Flow

- The canonical UI ingest path is `Upload media` from a live note editor.
- Agent and automation callers may still create media directly through `POST /resources/media`.
- Every media create requires one uploaded image, video, or supported file-family binary.
- Current supported file-family formats are `HEIC` and `HEIF`.
- Uploads may include common browser and desktop media extensions such as `.png`, `.jpg`, `.webp`, `.gif`, `.svg`, `.heic`, `.heif`, `.mp4`, `.webm`, `.mov`, `.m4v`, `.mkv`, `.ogv`, `.avi`, `.wmv`, `.mpeg`, `.mpg`, and `.3gp`.
- Direct media create may also accept optional alias, visibility, and favorite state.
- The initial media `body` is seeded from the uploaded filename stem as a `# Heading`.
- Image uploads create WebP variants when the server can decode the image.
- Video uploads create a first-frame WebP card variant and poster when the server can decode the video.
- File-family media do not create display, card, or poster variants.

## Guest View

- Image pages render the primary image above the Markdown body.
- Video pages render the primary video player above the Markdown body.
- File-family pages render metadata plus `Download original` above the Markdown body.
- Image media pages prefer a derived WebP display image when one exists.
- Guest media pages render the same body HTML path used by note pages and admin preview.
- Guest media pages expose `Download original` for the current raw file when the resource is public.
- Public media pages are indexable under the same discovery rules as public live notes.

## Admin View

- Admin media pages keep the shared shell rail and current-resource chrome.
- Image and video pages show the current preview, file metadata, Markdown body editor, alias, favorite, and visibility controls.
- File-family pages show file metadata, `Download original`, Markdown body editor, alias, favorite, and visibility controls.
- The page shows `Download original` for the current raw file.
- The primary preview surface does not prepend `Current file` copy.
- Markdown body edits autosave like note edits.
- Existing media binaries do not change from the media edit page.
- Existing derivative metadata does not change from the media edit page.

## Snapshot View

- Saved-snapshot media pages keep the saved raw file immutable.
- Saved-snapshot media pages prefer saved display derivatives for inline image rendering when present.
- Saved-snapshot media pages expose `Download original` for the saved raw file.

## Immutability

- Existing media binaries are immutable after creation.
- Existing derivatives are immutable after creation.
- Updating media Markdown, alias, visibility, or favorite state does not replace the binary file.
- New uploads create new media resources instead of replacing files on older media pages.
- First-frame poster generation applies to new media uploads only.
- Older saved snapshots keep their original object references.
- Later note edits do not rewrite earlier media objects or earlier note embed text.
