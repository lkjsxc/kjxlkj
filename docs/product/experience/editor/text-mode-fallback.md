# Text Mode Fallback Contract

## Availability

- Admins always have a low-emphasis `Text mode` action.
- Text mode uses the raw Markdown source for the full note.
- Text mode lives on the same note page and does not navigate away.

## Automatic Fallback

- If a note contains unsupported Markdown structures, the note opens in text mode.
- Unsupported content is preserved exactly as stored.
- Rich mode is not offered when round-trip safety is not guaranteed.

## Unsupported Examples

- Tables.
- HTML blocks.
- Nested list structures beyond the supported set.
- Footnotes, task-list syntax, or other extensions not covered by the supported block set.

## Exit Rules

- Switching from text mode back to rich mode is allowed only after the current body becomes representable by the supported block set.
- The fallback state must be explicit in the page chrome so admins know why rich mode is unavailable.
