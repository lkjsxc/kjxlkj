# Preview Panel Contract

## Default State

- Admin notes open with the Markdown editor active and the preview closed.
- There is no always-visible preview on first paint.
- There is no visible Markdown/WYSIWYG mode switch.

## Toggle Behavior

- The note workspace exposes one preview toggle in the editor chrome.
- The toggle opens a live rendered preview of the current Markdown body.
- The toggle closes the preview without leaving the editor workflow.
- Preview state is ephemeral UI state, not persisted note data.
- Preview-specific work may remain dormant until preview is opened.

## Layout Rules

- Wide screens open preview as a right-side panel beside the Markdown editor.
- Narrow screens open preview as an overlay/drawer above the editor workspace.
- The preview may not cause horizontal page overflow.
- The preview must be dismissible without losing note content.

## Rendering Rules

- Preview content tracks the current unsaved Markdown body.
- Preview rendering should stay visually aligned with guest note rendering.
- Tables, lists, blockquotes, headings, and fenced code must render normally inside the preview.
