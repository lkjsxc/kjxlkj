# Markdown Workspace Contract

## Editing Model

- Admin resource pages use one first-party Markdown textarea for `body`.
- There is no visible `Rich mode` or `Text mode` switch.
- Notes are Markdown-first only.
- Media is file-first plus Markdown body editing.

## Storage Rule

- Canonical descriptive content remains raw Markdown in `body`.
- Media binaries live in S3-compatible object storage rather than inside Markdown.
- File replacement is explicit and separate from Markdown autosave.

## UI Rules

- The editor lives inside the normal resource shell.
- Preview starts closed and opens on demand from resource chrome.
- The `Public` checkbox remains outside the textarea and inside the workspace chrome.
- Alias, canonical URL, file URL, and metadata controls should use a consistent card-like presentation.
- The page itself may not require horizontal scrolling to edit.
- The visible textarea label `Markdown body` is absent.
- Typed Markdown must stay legible in the editor and render correctly in opened preview before save.

## Asset Delivery

- Editor behavior is implemented by local authored JS and CSS only.
- Admin pages may not depend on an external editor CDN at runtime.
