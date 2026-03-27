# Toast UI Reference Alignment

## Reference Sources

- Use `https://ui.toast.com/tui-editor/` as the product-feel reference.
- Use `https://nhn.github.io/tui.editor/latest/` as the runtime/API reference.
- When the two differ, prefer the concrete runtime behavior shown by the official live examples.

## Normative Upstream Behaviors

- The visible editor is one WYSIWYG surface ready for direct typing.
- Opening an admin note should leave focus ready for typing inside the editor.
- Desktop toolbar flow should stay close to the official text authoring order.
- Table authoring is part of the supported editing surface.
- Toolbar buttons should feel upstream, not custom-invented.
- Newly typed structure should become visibly formatted before save.
- Headings, lists, blockquotes, fenced code, links, and tables are first-class authoring paths.
- Long notes scroll with the page, not with a detached inner editor scroller.

## Intentional Product Deviations

- Keep the app shell dark, flat, and dense rather than matching the official site theme.
- Hide the Markdown/WYSIWYG mode switch.
- Serve Toast UI assets from local vendored routes only.
- Persist canonical note content as Markdown in `body`.
- Keep quiet autosave success and only show restrained failure feedback.
- Keep image upload, file upload, charts, UML, and other plugin workflows out of scope.

## Acceptance Meaning

- `Near-upstream` means the editor should feel like official Toast UI inside the app shell.
- `Near-upstream` does not permit detached toolbars, broken block transforms, or plain-looking newly typed content.
- If local theming conflicts with official editing semantics, local theming loses.
