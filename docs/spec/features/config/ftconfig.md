# Filetype Configuration

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Per-filetype settings and behavior.

## Overview

Filetype configuration allows different settings for different file types (Rust, Python, Markdown, etc.).

## Configuration

Filetype settings are defined in `ftconfig/` directory or inline in the main config:

| Setting | Description |
|---|---|
| `tabstop` | Tab display width |
| `shiftwidth` | Indent size |
| `expandtab` | Use spaces |
| `textwidth` | Line wrap width |
| `commentstring` | Comment format |
| `formatoptions` | Auto-formatting flags |

## LSP Configuration

Per-filetype LSP server configuration:

| Field | Description |
|---|---|
| `lsp.command` | LSP server command |
| `lsp.args` | Command arguments |
| `lsp.settings` | Server-specific settings |

## Detection

File type is detected by:

1. File extension
2. First line (shebang, modeline)
3. File path pattern

## Related

- Configuration: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- Indentation: [/docs/spec/modes/insert/insert-indentation.md](/docs/spec/modes/insert/insert-indentation.md)
