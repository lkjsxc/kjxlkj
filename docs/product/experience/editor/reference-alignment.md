# Local Editor Reference Alignment

## Reference Sources

- Use guest note rendering as the preview fidelity reference.
- Use the authored textarea workspace itself as the only editing reference.
- When editing and preview behavior diverge, guest note rendering is the canonical output target.

## Normative Behaviors

- The visible textarea is the real Markdown-writing surface.
- Opening an admin note should leave focus ready for typing inside the editor.
- Markdown preview is a first-class companion surface.
- Newly typed Markdown syntax should stay legible in the editor and render correctly in preview before save.
- Headings, lists, blockquotes, fenced code, links, and tables are first-class authoring paths.
- Long notes scroll with the page, not with a detached inner editor scroller.

## Intentional Product Deviations

- Keep the app shell dark, flat, and dense rather than using a generic docs-editor look.
- Expose one textarea-first authoring surface with no formatting toolbar.
- Persist canonical note content as Markdown in `body`.
- Keep quiet autosave success and only show restrained failure feedback.
- Keep image upload, file upload, charts, UML, and other plugin workflows out of scope.

## Acceptance Meaning

- `Aligned` means the textarea, autosave flow, and preview all support normal Markdown authoring without hidden editor machinery.
- `Aligned` does not permit broken preview toggles, missing Markdown legibility, or page-level overflow regressions.
- If local theming conflicts with authoring clarity or preview fidelity, local theming loses.
