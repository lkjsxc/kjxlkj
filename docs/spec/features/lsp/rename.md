# Rename Refactoring

LSP-powered rename of symbols across the project.

## Usage (normative)

| Key | Action |
|---|---|
| `<leader>rn` | Start rename for symbol under cursor |
| `:Rename {name}` | Rename to specified name directly |

## Workflow

1. Cursor on symbol to rename
2. `<leader>rn` opens inline rename prompt pre-filled with current name
3. Edit the name
4. `<Enter>` applies rename across all files; `<Esc>` cancels

## Prepare Rename

Before showing the prompt, the editor sends `textDocument/prepareRename` to the LSP server. The server validates:

- Symbol exists and is renameable
- Returns the valid range and placeholder text

If the symbol cannot be renamed, the editor shows an error message and does not open the prompt.

## Preview

When `rename_preview = true` (default), a preview panel shows all affected locations before applying. The user confirms or cancels.

## Scope

Rename affects all references in the workspace:

- Same-file references
- Cross-file references (imports, re-exports)
- The declaration itself

## Undo

All rename changes across all files are grouped as a single undo operation. A single `u` undoes the entire rename.

## LSP Server Support

| Server | Rename | Prepare | File Rename |
|---|---|---|---|
| rust-analyzer | Yes | Yes | No |
| typescript-language-server | Yes | Yes | Yes |
| gopls | Yes | Yes | No |
| clangd | Yes | Yes | No |
| pyright | Yes | Yes | No |

## File Rename

Some LSP servers (e.g., TypeScript) support renaming files when a module is renamed. The editor applies associated import path updates.

## Related

- Code actions: [/docs/spec/features/editing/code-actions.md](/docs/spec/features/editing/code-actions.md)
- References: [/docs/spec/features/lsp/navigation/references.md](/docs/spec/features/lsp/navigation/references.md)
