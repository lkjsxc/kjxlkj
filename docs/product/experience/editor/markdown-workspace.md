# Markdown Workspace Contract

## Editing Model

- Admin note pages use a single third-party Markdown editor.
- The editor defaults to Markdown authoring, not rendered rich editing.
- There is no visible `Rich mode` or `Text mode` switch.
- The editor implementation is Toast UI Editor pinned to one exact vendored version.
- The interaction target is the official Toast UI Markdown-writing feel, adapted to this app shell.
- The configuration uses supported Toast UI options rather than DOM surgery.
- The app may not rely on a WYSIWYG shortcut-repair layer as its primary Markdown authoring model.

## Storage Rule

- Canonical stored content remains raw Markdown in `body`.
- The editor must round-trip through Markdown via its public API.

## UI Rules

- The editor lives inside the normal note shell.
- The editor surface stays flat, dark, and document-first.
- The mode switch is hidden through supported editor configuration.
- Opening an admin note should place typing focus into the visible editor.
- Vim mode is available but disabled by default.
- Vim mode is controlled by a browser-local preference exposed from the admin settings UI.
- Desktop keeps an upstream-style text/table toolbar inside Markdown mode.
- Narrow screens may compact the toolbar, but `heading`, `bold`, `italic`, `strike`, `quote`, `ul`, `ol`, `task`, `table`, `link`, `code`, and `codeblock` remain available.
- Preview starts closed and opens on demand from note chrome.
- Toolbar groups wrap onto additional rows before a horizontal toolbar scrollbar is allowed to appear.
- The `Public` checkbox remains outside the editor body and inside the note workspace chrome.
- The note page owns vertical scrolling; the editor body may not add its own normal vertical scroll region.
- The page itself may not require horizontal scrolling in order to edit.
- Typed Markdown must stay legible in the editor and render correctly in the opened preview before save.
- The preview must use the current unsaved Markdown body, not only the last persisted version.
- Verification must target the visible Markdown editor and preview surfaces, not hidden Toast UI containers.

## Asset Delivery

- Editor CSS and JS are served from local versioned asset routes.
- Admin note HTML may not depend on an external editor CDN at runtime.
- Vendored editor files must keep their upstream license text.
- Any vendored Vim keymap asset must stay pinned and documented like other editor assets.

## Fallback Rule

- If the third-party editor fails to load, the page must remain editable with a plain local fallback field.
- The fallback may not introduce a user-facing mode switch.
