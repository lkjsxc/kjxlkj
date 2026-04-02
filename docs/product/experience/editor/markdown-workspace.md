# Markdown Workspace Contract

## Editing Model

- Admin note pages use one first-party Markdown textarea.
- The editor surface is Markdown authoring only.
- There is no visible `Rich mode` or `Text mode` switch.
- The app may not rely on a WYSIWYG shortcut-repair layer as its primary Markdown authoring model.

## Storage Rule

- Canonical stored content remains raw Markdown in `body`.
- The editor writes `body` directly as Markdown text.

## UI Rules

- The editor lives inside the normal note shell.
- The editor surface stays flat, dark, and document-first.
- Opening an admin note should place typing focus into the visible editor.
- Preview starts closed and opens on demand from note chrome.
- The `Public` checkbox remains outside the editor body and inside the note workspace chrome.
- Alias, canonical URL, and metadata controls should use a consistent card-like presentation.
- The note page owns vertical scrolling; the editor body may not add its own normal vertical scroll region.
- The page itself may not require horizontal scrolling in order to edit.
- Typed Markdown must stay legible in the editor and render correctly in the opened preview before save.
- The preview must use the current unsaved Markdown body, not only the last persisted state.
- Verification must target the visible Markdown editor and preview surfaces.

## Asset Delivery

- Editor behavior is implemented by local authored JS and CSS only.
- Admin note HTML may not depend on an external editor CDN at runtime.
