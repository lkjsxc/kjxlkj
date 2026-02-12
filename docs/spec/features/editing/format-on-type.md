# Format on Type

Back: [/docs/spec/features/editing/README.md](/docs/spec/features/editing/README.md)

Automatic formatting triggered by specific characters during insert mode.

## Overview

When enabled, typing certain trigger characters (e.g., `;`, `}`, newline) causes the editor to request formatting from the LSP server for the affected range. The formatting result is applied to the buffer immediately.

## Enabling

| Setting | Type | Default | Description |
|---|---|---|---|
| `format.on_type` | boolean | `false` | Enable format-on-type globally |

When `format.on_type = true`, the editor sends `textDocument/onTypeFormatting` requests to the LSP server after each trigger character is typed.

## Trigger characters

Characters that initiate formatting requests.

### Default triggers

Each LSP server advertises its own trigger characters in its capabilities response. Common triggers:

| Character | Typical effect |
|---|---|
| `;` | Format the completed statement |
| `}` | Format the completed block |
| `\n` (Enter) | Format the previous line and adjust indentation of the new line |
| `)` | Format the completed expression |

### Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `format.on_type_triggers` | string or null | null (use server defaults) | Override trigger characters. A string of characters, e.g., `";}"` |

## LSP integration

How format-on-type interacts with language servers.

### Server support

Format-on-type requires the LSP server to support `textDocument/onTypeFormatting`.

| Server | Format on type |
|---|---|
| rust-analyzer | Yes |
| typescript-language-server | Yes |
| gopls | Yes |
| clangd | Yes |
| pyright | No |

### Fallback

If the LSP server does not support `textDocument/onTypeFormatting`, the feature is silently disabled. No external formatter fallback is used for on-type formatting.

## Per-language configuration

| Setting | Type | Description |
|---|---|---|
| `filetype.{lang}.format.on_type` | boolean | Enable/disable per language |

## Undo

Formatting changes are grouped with the trigger keystroke.

### Single undo

The trigger character insertion and the formatting edits MUST be grouped into a single undo entry. Pressing `u` undoes both the character and the formatting change together.

## Performance

Measures to prevent formatting from disrupting typing flow.

### Debouncing

Rapid typing MUST NOT trigger multiple concurrent format requests. If a new trigger character is typed before the previous format response arrives, the pending response is discarded and only the latest request is used.

### Async

The format request runs asynchronously. The user can continue typing while the request is in flight. When the response arrives, edits are applied to the buffer and the cursor position is adjusted.

## Disable temporarily

| Command | Description |
|---|---|
| `:FormatOnTypeToggle` | Toggle format-on-type for the current buffer |

## Comparison

| Feature | Format on type | Format on save |
|---|---|---|
| When | While typing specific characters | On `:w` |
| Scope | Line or block around trigger | Full file |
| Interruption | Minimal (async) | None (runs before write) |
| Undo | Grouped with typed character | Separate undo entry |

## Related

- Format on save: [/docs/spec/features/editing/format-on-paste.md](/docs/spec/features/editing/format-on-paste.md)
- LSP formatting: [/docs/spec/features/lsp/formatting.md](/docs/spec/features/lsp/formatting.md)
- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
