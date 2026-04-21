# Media Page Contract

## Create Flow

- The canonical UI ingest path is `Upload media` from a live note editor.
- Agent and automation callers may still create media directly through `POST /resources/media`.
- Every media create requires one uploaded image, video, or supported file-family binary.
- Current supported file-family formats are `HEIC` and `HEIF`.
- Uploads may include common browser and desktop media extensions such as `.png`, `.jpg`, `.webp`, `.gif`, `.svg`, `.heic`, `.heif`, `.mp4`, `.webm`, `.mov`, `.m4v`, `.mkv`, `.ogv`, `.avi`, `.wmv`, `.mpeg`, `.mpg`, and `.3gp`.
- Direct media create may also accept optional alias, visibility, and favorite state.
- The initial media `body` is seeded from the uploaded filename stem as a `# Heading`.
- Media created from note attachment persist the triggering note as immutable `owner_note_id`.
- Direct media create leaves `owner_note_id` empty.
- Image uploads create WebP variants when the server can decode the image.
- Video uploads create a first-frame WebP card variant and poster when the server can decode the video.
- File-family media do not create display, card, or poster variants.

## Guest View

- Live media pages reuse the same top-row live-resource shell as live Markdown notes.
- The rail omits live-resource context, alias, timeline cards, and the history affordance for live media pages.
- The main pane starts with the shared top-row navigation followed by live-resource metadata.
- Image pages render the primary image above the Markdown body.
- Video pages render the primary video player above the Markdown body.
- Video players expose browser-native controls.
- Video players stay contained inside the media surface and never overflow the page column.
- File-family pages render metadata plus `Download original` above the Markdown body.
- Image media pages prefer a derived WebP display image when one exists.
- Guest media pages render the same body HTML path used by note pages and admin preview.
- Guest media pages expose `Download original` for the current raw file when the resource is public.
- Public media pages are indexable under the same discovery rules as public live notes.

## Admin View

- Admin live media pages use the same live-resource shell layout as live Markdown notes.
- Image and video pages show the current preview, file metadata, Markdown body editor, alias, favorite, and visibility controls.
- File-family pages show file metadata, `Download original`, Markdown body editor, alias, favorite, and visibility controls.
- The page shows `Download original` for the current raw file.
- The primary preview surface does not prepend `Current file` copy.
- The shared top-row `Prev` / `History` / `Next` cards keep matched widths and a matched minimum height.
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
- `owner_note_id` is immutable after creation.
- Updating media Markdown, alias, visibility, or favorite state does not replace the binary file.
- New uploads create new media resources instead of replacing files on older media pages.
- First-frame poster generation applies to new media uploads only.
- Older saved snapshots keep their original object references.
- Later note edits do not rewrite earlier media objects or earlier note embed text.
