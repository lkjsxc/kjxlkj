# Reference Finder

Find all references to a symbol across the project.

## Keybindings (normative)

| Key | Action |
|---|---|
| `gr` | Show references for symbol under cursor |
| `<leader>fr` | Find references (same as `gr`) |

## Display

References appear in a quickfix-style picker listing. Each entry shows:

- File path (relative to workspace root)
- Line number and column
- Context line (the line containing the reference)
- Total reference count in the header

## Navigation in Reference List

| Key | Action |
|---|---|
| `j` / `k` | Move up/down |
| `<CR>` | Jump to reference |
| `<C-v>` | Open in vertical split |
| `<C-x>` | Open in horizontal split |
| `<Esc>` | Close reference list |
| `q` | Close reference list |

## Include Declaration

By default, the declaration of the symbol is included in the reference list. This can be toggled via `references_include_declaration = true` (default).

## Quickfix Integration

References can be sent to the quickfix list with `:copen` for batch navigation. From quickfix, use `]q` / `[q` to step through references.

## Preview

When navigating the reference list, a preview of the reference location is shown inline, displaying surrounding context lines.

## Filtering

For large result sets, type to filter references by file path or content.

## LSP Requirements

Requires LSP server with `textDocument/references` support. All major servers (rust-analyzer, typescript, gopls, clangd, pyright) support this.

## Related

- Go to definition: [/docs/spec/features/lsp/navigation/document-symbols.md](/docs/spec/features/lsp/navigation/document-symbols.md)
- Rename: [/docs/spec/features/lsp/rename.md](/docs/spec/features/lsp/rename.md)
