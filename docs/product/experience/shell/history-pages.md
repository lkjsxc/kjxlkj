# History Pages Contract

## Layout

- History index and revision pages reuse the note shell.
- The rail keeps current-note context, prev/next links, revision links, and actions.
- The history body stays in the main pane.

## Access Rules

- Guests may read only public revisions.
- Admins may read all revisions.
- Revision pages never expose raw note IDs as normal page chrome.

## Rail Limits

- Rail history is intentionally short and current-context focused.
- Full history browsing happens in the dedicated history index, not by rendering every revision in the rail.
