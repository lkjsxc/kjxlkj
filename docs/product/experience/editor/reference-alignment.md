# Editor Rendering Alignment

## Reference Sources

- Use guest note rendering as the primary output reference for admin preview.
- Use common Markdown author expectations as the input reference for the textarea authoring surface.

## Normative Behaviors

- The visible editor is the real Markdown-writing surface, not a WYSIWYG-first imitation.
- Opening an admin note should leave focus ready for typing inside the editor.
- Markdown preview is a first-class companion surface.
- Newly typed Markdown syntax should stay legible in the editor and render correctly in preview before save.
- Headings, lists, blockquotes, fenced code, links, and tables are first-class authoring paths.
- Long notes scroll with the page, not with a detached inner editor scroller.

## Intentional Product Deviations

- Keep the app shell dark, flat, and dense rather than matching the official site theme.
- Persist canonical note content as Markdown in `body`.
- Keep quiet autosave success and only show restrained failure feedback.
- Keep helper toolbars, image upload, file upload, charts, UML, and similar workflows out of scope.

## Acceptance Meaning

- `Aligned` means typing, autosave, and preview rendering feel direct and predictable inside the app shell.
- `Aligned` does not permit broken preview toggles, missing Markdown legibility, or mismatched guest-versus-preview rendering.
- If local theming conflicts with preview readability, readability wins.
