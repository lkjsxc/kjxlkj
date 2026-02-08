# LSP Completion

Back: [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)

Completion powered by Language Server Protocol.

## Overview

LSP completion provides context-aware suggestions including symbols, keywords, snippets, and auto-imports.

## Trigger

| Trigger | Description |
|---|---|
| `<C-Space>` | Manual trigger |
| Auto | After typing trigger characters (`.`, `::`, etc.) |

## Popup Menu

| Key | Action |
|---|---|
| `<C-n>` / `<Down>` | Next item |
| `<C-p>` / `<Up>` | Previous item |
| `<C-y>` | Confirm selection |
| `<C-e>` | Cancel |
| `<Tab>` | Next item or expand |

## Item Kinds

| Kind | Icon | Description |
|---|---|---|
| Function | `ƒ` | Functions and methods |
| Variable | `α` | Variables and parameters |
| Struct | `S` | Structs and classes |
| Module | `M` | Modules and namespaces |
| Snippet | `⚡` | Snippet templates |

## Resolve

On item focus, the editor sends `completionItem/resolve` to get documentation and additional text edits (auto-imports).

## Configuration

| Setting | Default | Description |
|---|---|---|
| `completion.auto` | `true` | Auto-trigger |
| `completion.min_length` | `1` | Min prefix length |
| `completion.max_items` | `50` | Max popup items |

## Related

- Insert mode: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- LSP: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
