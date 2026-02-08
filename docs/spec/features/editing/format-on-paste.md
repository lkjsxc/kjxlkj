# Format on Paste

Automatically adjust formatting when pasting code.

## Overview

When enabled, pasted content is automatically re-indented to match the surrounding code. Optionally, full LSP formatting is applied to the pasted region.

## Configuration

| Option | Default | Description |
|---|---|---|
| `format_on_paste` | `true` | Enable format-on-paste |
| `format_on_paste_full` | `false` | Apply full LSP formatting (not just indent) |

## Behavior

How format-on-paste works.

### Indentation Adjustment (default)

Pasted content is re-indented to match the indentation level at the cursor position. The relative indentation within the pasted block is preserved.

### Full Formatting

When `format_on_paste_full = true` and an LSP server with `textDocument/rangeFormatting` is available, the pasted region receives full formatting (spacing, style, line breaks).

### Fallback

If LSP is unavailable, only indentation adjustment is applied.

## Source Detection

| Pasted From | Treatment |
|---|---|
| Same file type | Full formatting available |
| Different file type | Indentation adjustment only |
| Plain text | Indentation adjustment only |

## Undo

Paste and formatting are a single undo unit. One `u` undoes both.

## Skip Formatting

Use `<C-S-v>` (or configurable key) to paste without any formatting adjustment. `:paste!` also pastes raw content.

## Performance

For pastes exceeding 1000 lines, formatting is skipped and only indentation is adjusted, to avoid blocking the editor.

## Related

- Format on type: [/docs/spec/features/editing/format-on-type.md](/docs/spec/features/editing/format-on-type.md)
- Formatting: [/docs/spec/features/editing/formatting.md](/docs/spec/features/editing/formatting.md)
