# Markdown Workspace Contract

## Editing Model

- Admin note pages use one in-house Markdown editor.
- The editor defaults to Markdown authoring, not rendered rich editing.
- There is no visible `Rich mode` or `Text mode` switch.
- The editor is textarea-first.
- The textarea is the only mutable source of authoring truth.
- The editor does not expose the older heading, bold, italic, strike, quote, list, table, link, or code insertion toolbar.

## Storage Rule

- Canonical stored content remains raw Markdown in `body`.
- The editor must round-trip through the textarea value without hidden rich-text state.

## UI Rules

- The editor lives inside the normal note shell.
- The editor surface stays flat, dark, and document-first.
- Opening an admin note should place typing focus into the visible editor.
- Preview starts closed and opens on demand from note chrome.
- The `Public` checkbox remains outside the editor body and inside the note workspace chrome.
- Alias, canonical URL, and mode controls should use a consistent card-like presentation.
- The note page owns vertical scrolling; the editor body may not add its own normal vertical scroll region.
- The page itself may not require horizontal scrolling in order to edit.
- Typed Markdown must stay legible in the editor and render correctly in the opened preview before save.
- The preview must use the current unsaved Markdown body, not only the last persisted state.
- Verification must target the visible textarea and preview surfaces.

## Fallback Rule

- The editor may not depend on a second hidden editing engine.
- The page must remain editable when preview requests fail.
