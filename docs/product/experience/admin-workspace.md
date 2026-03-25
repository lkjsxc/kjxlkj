# Admin Workspace Contract

## Dashboard

- `GET /admin` is the canonical note-management index.
- The dashboard lists all non-deleted notes, including private notes.
- Each row shows title, slug, created timestamp, updated timestamp, and visibility status.

## Dashboard Actions

- Create a new note.
- Open a note editor.
- Distinguish private notes without relying on color alone.
- Preserve a fast path back to the currently opened note.

## Note Editing Context

- Admin note pages are not reduced to a single back link.
- The rail keeps dashboard access, note navigation, revision browsing, visibility state, and destructive actions in one place.
- Autosave status is explicit: idle, saving, saved, or failed.

## Visibility Semantics

- Public visibility is the user-facing term.
- Storage may continue using `is_private`, but UI and docs speak in public/private outcomes.
- New notes default to not public.
