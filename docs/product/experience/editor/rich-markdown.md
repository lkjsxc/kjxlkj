# Rich Markdown Contract

## Default Editing Mode

- Admin note pages default to rich Markdown block editing.
- Rich mode is available only when the entire note can round-trip through the supported block set.
- The editor remains inside the note page shell; it is not a separate application mode.

## Supported Blocks

- Level 1, 2, and 3 headings.
- Paragraphs.
- Unordered lists.
- Ordered lists.
- Blockquotes.
- Fenced code blocks.

## Editing Rules

- Each block renders as Markdown output first.
- Clicking or focusing a block edits that block in place.
- Saving rebuilds the note body from the edited block sequence.
- Heading edits update visible note title chrome immediately.

## Save Rules

- Autosave remains the default save path.
- Save state stays visible as `Saving`, `Saved`, or `Save failed`.
- Visibility changes and block edits share the same save pipeline.
