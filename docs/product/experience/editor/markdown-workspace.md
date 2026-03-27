# Markdown Workspace Contract

## Editing Model

- Admin note pages use a single third-party Markdown editor.
- The editor defaults to rendered rich editing, not raw-source mode.
- There is no visible `Rich mode` or `Text mode` switch.
- The editor implementation is Toast UI Editor pinned to one exact vendored version.
- The configuration uses supported Toast UI options rather than DOM surgery.
- Live Markdown shortcut transforms inside WYSIWYG are an app-level enhancement layered on top of Toast UI.

## Storage Rule

- Canonical stored content remains raw Markdown in `body`.
- The editor must round-trip through Markdown via its public API.

## UI Rules

- The editor lives inside the normal note shell.
- The editor surface stays flat, dark, and document-first.
- The mode switch is hidden through supported editor configuration.
- Desktop keeps the official toolbar ordering.
- Narrow screens use this reduced toolbar set: `heading`, `bold`, `italic`, `strike`, `quote`, `ul`, `ol`, `task`, `link`, `code`, `codeblock`.
- Toolbar groups wrap onto additional rows before a horizontal toolbar scrollbar is allowed to appear.
- The `Public` checkbox remains outside the editor body and inside the note workspace chrome.
- The note page owns vertical scrolling; the editor body may not add its own normal vertical scroll region.
- The page itself may not require horizontal scrolling in order to edit.
- Newly typed headings, lists, blockquotes, and fenced code must render with the same visible semantics as seeded content before save.
- Verification must target the visible WYSIWYG subtree, not hidden Toast UI containers.

## Asset Delivery

- Editor CSS and JS are served from local versioned asset routes.
- Admin note HTML may not depend on an external editor CDN at runtime.
- Vendored editor files must keep their upstream license text.

## Fallback Rule

- If the third-party editor fails to load, the page must remain editable with a plain local fallback field.
- The fallback may not introduce a user-facing mode switch.
