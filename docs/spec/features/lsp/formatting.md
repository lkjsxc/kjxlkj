# Auto-Formatting

Code formatting via LSP or external formatters.

## Overview

kjxlkj integrates with LSP formatting and external formatters (rustfmt, prettier, black, etc.) for both manual and automatic formatting.

## Format on Save

| Setting | Default | Description |
|---|---|---|
| `format_on_save` | `false` | Enable auto-format on `:w` |
| `format_on_save.timeout` | `2000` | Milliseconds before cancellation |

## Manual Formatting

| Key / Command | Action |
|---|---|
| `gq{motion}` | Format text covered by motion |
| `gqq` | Format current line |
| Visual + `gq` | Format selection |
| `<leader>lf` | Format entire buffer |
| `:Format` | Format entire buffer |

## Formatter Resolution

Resolution order:

1. **LSP** — if server supports `textDocument/formatting`
2. **External** — configured per-filetype command
3. **Built-in** — basic text wrapping via `textwidth`

| Setting | Default | Description |
|---|---|---|
| `prefer_lsp` | `true` | Try LSP first |
| `lsp_fallback` | `true` | Fall back to external if no LSP |

## Range Formatting

When a visual selection is active, `gq` sends only the selected range for formatting. LSP servers that support `textDocument/rangeFormatting` format the selection; others format the full buffer.

## Per-Filetype Configuration

Formatters are configured per filetype in TOML:

| Language | Typical Formatter |
|---|---|
| Rust | `rustfmt` |
| JavaScript/TypeScript | `prettier` |
| Python | `black` / `ruff` |
| Go | `gofmt` / `goimports` |
| C/C++ | `clang-format` |

## Error Handling

If formatting fails (non-zero exit, LSP error), the buffer is left unchanged and an error message is displayed. Formatting never corrupts the buffer.

## Undo Integration

Each format operation is a single undo step.

## Related

- LSP: [/docs/spec/features/lsp/code-actions.md](/docs/spec/features/lsp/code-actions.md)
- Text wrapping: [/docs/spec/features/lsp/formatting.md](/docs/spec/features/lsp/formatting.md)
