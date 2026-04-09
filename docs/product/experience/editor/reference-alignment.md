# Editor Rendering Alignment

## Reference Sources

- Use guest resource rendering as the primary output reference for admin preview.
- Use common Markdown author expectations as the input reference for the body editor.

## Normative Behaviors

- Notes use a real Markdown-writing surface, not a WYSIWYG-first imitation.
- Media resources use a real Markdown-writing surface plus a first-class file viewer or player.
- Opening an admin resource should leave focus ready for direct body editing.
- Markdown preview is a first-class companion surface.
- Newly typed Markdown syntax must stay legible in the editor and render correctly in preview before save.
- Headings, lists, blockquotes, fenced code, links, tables, inline images, and safe inline video embeds are first-class authoring paths.

## Intentional Product Deviations

- Keep the shell dark, flat, and dense rather than matching publishing-site chrome.
- Persist canonical note and media descriptive content as Markdown in `body`.
- Keep quiet autosave success and restrained failure feedback.
- Keep toolbars, rich-text modes, and server-side remote media import out of scope.
