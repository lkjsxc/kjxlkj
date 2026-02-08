# Code Actions

LSP code actions for quick fixes and refactoring.

## Overview

Code actions are context-sensitive suggestions from the LSP server: quick fixes for diagnostics, refactoring operations, and source-level transformations.

## Triggering

| Key / Command | Action |
|---|---|
| `<leader>ca` | Show code actions menu |
| `<leader>.` | Apply preferred (first) action |
| `:CodeAction` | Show code actions menu |

Code actions are available when the cursor is on a diagnostic or selected text.

## Action Categories

| Kind | Examples |
|---|---|
| Quick Fix | Import missing item, fix typo, add missing field |
| Refactor | Extract function, inline variable, rename symbol |
| Refactor Extract | Extract to function/variable/constant |
| Refactor Inline | Inline function/variable |
| Source | Organize imports, format file, generate docs |

## Menu Navigation

| Key | Action |
|---|---|
| `j` / `<Down>` | Next action |
| `k` / `<Up>` | Previous action |
| `<CR>` | Apply selected action |
| `1`-`9` | Apply numbered action directly |
| `<Esc>` / `q` | Cancel |

## Lightbulb Indicator

When code actions are available at cursor position, a lightbulb icon appears in the sign column. Controlled by `lsp.code_action.lightbulb` (default: `true`).

## Visual Mode

With a visual selection active, `<leader>ca` requests actions scoped to the selected range (e.g., extract to function).

## Preferred Actions

Some actions are marked "preferred" by the server (e.g., the most likely quick-fix). `<leader>.` applies the preferred action without showing the menu.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `lsp.code_action.lightbulb` | `true` | Show lightbulb indicator |
| `lsp.code_action.auto_apply_preferred` | `false` | Auto-apply preferred action |

## Related

- Diagnostics: [/docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)
- Formatting: [/docs/spec/features/lsp/formatting.md](/docs/spec/features/lsp/formatting.md)
