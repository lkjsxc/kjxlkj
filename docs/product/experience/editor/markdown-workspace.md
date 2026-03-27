# Markdown Workspace Contract

## Editing Model

- Admin note pages use a single third-party Markdown editor.
- The editor defaults to rendered rich editing, not raw-source mode.
- There is no visible `Rich mode` or `Text mode` switch.
- The editor implementation is Toast UI Editor pinned to one exact vendored version.

## Storage Rule

- Canonical stored content remains raw Markdown in `body`.
- The editor must round-trip through Markdown via its public API.

## UI Rules

- The editor lives inside the normal note shell.
- The editor surface stays flat, dark, and document-first.
- The mode switch chrome from the third-party editor is hidden.
- Toolbar chrome is present only if it stays restrained.
- On narrow screens the toolbar compacts before the page is allowed to overflow.
- The `Public` checkbox remains outside the editor body and inside the note workspace chrome.
- The page itself may not require horizontal scrolling in order to edit.

## Asset Delivery

- Editor CSS and JS are served from local versioned asset routes.
- Admin note HTML may not depend on an external editor CDN at runtime.
- Vendored editor files must keep their upstream license text.

## Fallback Rule

- If the third-party editor fails to load, the page must remain editable with a plain local fallback field.
- The fallback may not introduce a user-facing mode switch.
