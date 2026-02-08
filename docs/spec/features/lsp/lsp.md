# LSP Integration

Back: [docs/spec/features/lsp/README.md](docs/spec/features/lsp/README.md)

Built-in Language Server Protocol client.

## Overview

The editor embeds a full LSP client that communicates
with language servers via stdio or TCP. All LSP
features are first-class and integrated into the
editor's UI and keybinding system.

## Architecture

### LSP Service Task

The LSP service runs as a Tokio task, separate from
the core editor task. Communication with the core
uses the message bus.

### Multiple Servers

Multiple language servers can run simultaneously,
one per language or filetype. Server configuration
specifies which server to use for each filetype.

### Lifecycle

| Phase | Description |
|-------|-------------|
| Start | Server spawned on first open of matching filetype |
| Initialize | Capabilities exchanged, root URI set |
| Running | Requests and notifications flow bidirectionally |
| Shutdown | Graceful shutdown on `:quit` or server crash |
| Restart | Automatic restart on crash (up to 3 retries) |

## Features

### Completion

| Feature | Description |
|---------|-------------|
| Trigger | Auto-trigger or manual `Ctrl-Space` |
| Items | Filtered by fuzzy match on typed prefix |
| Detail | Documentation shown in preview pane |
| Snippets | Snippet completions with tabstops |
| Resolve | Lazy detail loading via `completionItem/resolve` |
| Commit chars | Characters that accept and insert |

### Diagnostics

| Feature | Description |
|---------|-------------|
| Inline | Diagnostic text shown at end of line |
| Signs | Gutter signs (error, warning, info, hint) |
| Float | Hover diagnostic detail on cursor line |
| List | `:Diagnostics` shows all in finder |
| Navigation | `]d` / `[d` jump to next/prev diagnostic |

### Hover

`K` shows hover information for the symbol under
the cursor. The hover popup displays documentation,
type signatures, and source information.

### Go-To

| Command | LSP Method |
|---------|------------|
| `gd` | textDocument/definition |
| `gD` | textDocument/declaration |
| `gy` | textDocument/typeDefinition |
| `gi` | textDocument/implementation |
| `gr` | textDocument/references |

When multiple results are returned, they appear
in the finder for selection.

### Rename

`<leader>rn` triggers rename. The editor prompts
for the new name. The LSP server computes all
locations, and the editor applies the workspace
edit atomically.

### Code Actions

`<leader>ca` shows available code actions for the
current cursor position or selection. Actions are
displayed in a floating menu.

### Signature Help

Signature help appears automatically when typing
function call arguments. Shows parameter names,
types, and highlights the active parameter.

### Formatting

| Command | Scope |
|---------|-------|
| `:Format` | Format entire buffer |
| `gq{motion}` | Format range (if server supports) |
| Format-on-save | Automatic (configurable) |

### Inlay Hints

Virtual text showing type annotations, parameter
names, and other hints. Configurable per language.

### Code Lens

Actionable annotations above functions/types showing
references count, test runners, etc. Clicking
(keyboard activation) executes the associated command.

### Workspace Symbols

`:Symbols` searches across the entire workspace.
`:DocumentSymbols` shows symbols in the current file
as an outline.

## Server Configuration

### Per-Language Settings

Configuration in TOML:

| Option | Type | Description |
|--------|------|-------------|
| `lsp.{lang}.cmd` | list | Server command and args |
| `lsp.{lang}.root_markers` | list | Project root markers |
| `lsp.{lang}.settings` | table | Server-specific settings |
| `lsp.{lang}.filetypes` | list | Matching filetypes |

### Built-in Defaults

Common servers have built-in default configurations:
- rust-analyzer for Rust
- typescript-language-server for TypeScript/JavaScript
- pylsp for Python
- lua-language-server for Lua
- clangd for C/C++

## Progress Notifications

LSP progress tokens (e.g., "indexing") are displayed
in the status line with a spinner animation.

## Error Handling

### Server Crashes

If a server crashes, it is restarted automatically.
After 3 consecutive crashes within 60 seconds, the
server is disabled with a warning message.

### Request Timeouts

LSP requests timeout after 30 seconds (configurable).
Timed-out requests show an error message.

## Related

- Diagnostics: [docs/spec/features/lsp/diagnostics.md](docs/spec/features/lsp/diagnostics.md)
- Completion: [docs/spec/modes/insert/completion/insert-completion.md](docs/spec/modes/insert/completion/insert-completion.md)
- Formatting: [docs/spec/features/lsp/formatting.md](docs/spec/features/lsp/formatting.md)
