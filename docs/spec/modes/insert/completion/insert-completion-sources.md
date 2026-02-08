# Insert Completion Sources

Where completion suggestions come from.

## Source Types

| Source | Trigger | Description |
|---|---|---|
| LSP | Auto / `<C-x><C-o>` | Language-server semantic completions |
| Buffer | `<C-x><C-n>` | Words from current and open buffers |
| Path | `<C-x><C-f>` | File system paths |
| Snippet | Auto (via LSP) | Snippet expansion triggers |
| Dictionary | `<C-x><C-k>` | Words from dictionary files |
| Tag | `<C-x><C-]>` | Symbols from tags files |
| Line | `<C-x><C-l>` | Whole matching lines |
| Include | `<C-x><C-i>` | Words from included/imported files |

## LSP Completion

Provides semantic, context-aware completions including type information, auto-imports, and documentation. Triggered automatically after typing trigger characters (`.`, `::`, etc.) or manually via `<C-x><C-o>`.

Configuration:

| Setting | Default | Description |
|---|---|---|
| `completion.lsp.enable` | `true` | Enable LSP completions |
| `completion.lsp.auto_import` | `true` | Insert missing imports |

## Buffer Completion

Scans words in open buffers. By default scans the current buffer; configurable to include all loaded buffers.

| Setting | Default | Description |
|---|---|---|
| `completion.buffer.scope` | `"current"` | `"current"` or `"all"` |
| `completion.buffer.min_word_len` | `3` | Minimum word length to index |

## Path Completion

Triggered by typing a path separator (`/`). Completes relative and absolute paths. Respects `.gitignore` patterns.

| Setting | Default | Description |
|---|---|---|
| `completion.path.enable` | `true` | Enable path completion |
| `completion.path.show_hidden` | `false` | Include dotfiles |

## Source Priority

When multiple sources return candidates, they are merged and sorted by priority:

| Priority | Source |
|---|---|
| 1 (highest) | LSP |
| 2 | Snippet |
| 3 | Path |
| 4 | Buffer |
| 5 | Tag |
| 6 | Dictionary |

Priority is configurable per-source in TOML.

## Related

- Completion UI: [/docs/spec/modes/insert/completion/insert-completion.md](/docs/spec/modes/insert/completion/insert-completion.md)
- LSP: [/docs/spec/features/lsp/code-actions.md](/docs/spec/features/lsp/code-actions.md)
