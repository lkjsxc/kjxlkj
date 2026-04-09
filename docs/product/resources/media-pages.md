# Media Page Contract

## Create Flow

- Admin rails expose `New media` directly below `New note`.
- `New media` opens `/admin/media/new`.
- The create flow is upload-first and requires one file.
- Create may also accept optional alias, visibility, and favorite state.
- The initial media `body` is seeded from the uploaded filename stem as a `# Heading`.

## Guest View

- Media pages render the primary image or video player above the Markdown body.
- Guest media pages render the same body HTML path used by note pages and admin preview.
- Public media pages are indexable under the same discovery rules as public live notes.

## Admin View

- Admin media pages keep the shared shell rail and current-resource chrome.
- The page shows the current image or video preview, file metadata, Markdown body editor, alias, favorite, and visibility controls.
- Markdown body edits autosave like note edits.
- File replacement is an explicit action and does not piggyback on text autosave.

## File Replacement

- Replacing the file updates the live media resource.
- Successful file replacement creates one new immutable saved snapshot.
- Older saved snapshots keep their original object references.
- File replacement does not rewrite older note bodies or older media bodies.
