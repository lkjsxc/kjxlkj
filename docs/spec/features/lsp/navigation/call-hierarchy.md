# Call Hierarchy

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

View incoming and outgoing function calls via LSP `callHierarchy/incomingCalls` and `callHierarchy/outgoingCalls`.

## Overview

Call hierarchy displays which functions call a given function (incoming) and which functions a given function calls (outgoing). Results are presented in an expandable tree view.

## Usage

### Keybinding

| Key | Action |
|---|---|
| `<leader>ci` | Show incoming calls to function under cursor |
| `<leader>co` | Show outgoing calls from function under cursor |

### Command

| Command | Description |
|---|---|
| `:IncomingCalls` | Show incoming calls |
| `:OutgoingCalls` | Show outgoing calls |

## Incoming calls

### Display

The tree root is the function under the cursor. Each child node is a function that calls the root function. Children can be expanded to show their own callers recursively.

### Meaning

Answers the question: "Who calls this function?" Used for understanding impact before refactoring.

## Outgoing calls

### Display

The tree root is the function under the cursor. Each child node is a function that the root function calls. Children can be expanded to show their own callees recursively.

### Meaning

Answers the question: "What does this function do?" Used for understanding behavior and dependencies.

## Navigation

### Tree navigation

| Key | Action |
|---|---|
| `j` / `<Down>` | Move selection down |
| `k` / `<Up>` | Move selection up |
| `l` | Expand child node |
| `h` | Collapse node |
| `<Tab>` | Toggle expand/collapse |

### Actions

| Key | Action |
|---|---|
| `<CR>` | Jump to the selected function definition |
| `o` | Jump to function and close tree |
| `v` | Open in vertical split |
| `s` | Open in horizontal split |
| `<Esc>` | Close the tree view |

## Depth

### Expand levels

The initial display shows one level of calls. Each node can be expanded on demand. The editor MUST NOT eagerly load the entire call graph (which could be infinite for recursive functions).

### Lazy loading

Children are fetched from the LSP server only when the user expands a node. A loading indicator is shown while the request is in flight.

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `call_hierarchy.max_depth` | integer | `10` | Maximum expansion depth |
| `call_hierarchy.auto_expand` | integer | `1` | Number of levels to auto-expand |

## LSP requirements

### Server support

The LSP server must support `textDocument/prepareCallHierarchy`, `callHierarchy/incomingCalls`, and `callHierarchy/outgoingCalls`.

| Server | Call hierarchy support |
|---|---|
| rust-analyzer | Yes |
| clangd | Yes |
| typescript-language-server | Yes |
| gopls | Yes |
| pyright | Yes |

## Display options

| Setting | Type | Default | Description |
|---|---|---|---|
| `call_hierarchy.show_detail` | boolean | `true` | Show function signature detail |
| `call_hierarchy.show_kind` | boolean | `true` | Show symbol kind icon |

## Preview

### On select

When a node is highlighted, a preview of the function's source code is shown in a split pane or floating window. The preview highlights the line where the call occurs.

## Related

- Type hierarchy: [/docs/spec/features/lsp/navigation/type-hierarchy.md](/docs/spec/features/lsp/navigation/type-hierarchy.md)
- References: [/docs/spec/features/lsp/navigation/references.md](/docs/spec/features/lsp/navigation/references.md)
- LSP: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
