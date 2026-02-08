# Code Lens

Actionable annotations displayed above code elements.

## Overview

Code lens shows contextual information (reference counts, test runners, implementations) as inline annotations above functions, classes, and other symbols. Each lens is clickable/actionable.

## Display

Lenses appear as dimmed text above the annotated line. Each lens shows a label and is activatable via `<CR>` or mouse click.

## Common Lenses

| Lens Type | Display | Action |
|---|---|---|
| References | `3 references` | Show references list |
| Implementations | `2 implementations` | Show implementations |
| Run test | `Run` | Execute test |
| Debug test | `Debug` | Debug test |

## Keybindings (normative)

| Key | Action |
|---|---|
| `<leader>cl` | Toggle code lens visibility |
| `<CR>` | Execute lens at cursor |
| `]l` | Jump to next code lens |
| `[l` | Jump to previous code lens |

## Configuration

| Option | Default | Description |
|---|---|---|
| `code_lens` | `true` | Enable code lens display |
| `code_lens_refresh` | `"auto"` | Refresh strategy: `"auto"`, `"save"`, `"manual"` |

## LSP Integration

Code lens requires LSP server support via `textDocument/codeLens` and `codeLens/resolve`.

| Server | Code Lens |
|---|---|
| rust-analyzer | Yes (references, run/debug tests) |
| clangd | Yes (references) |
| gopls | Yes (references, implementations) |
| typescript-language-server | Limited |

## Performance

Lenses are computed lazily — only for visible lines. On scroll, new lenses are requested. The `code_lens_refresh` option controls when full recomputation happens.

## Related

- References: [/docs/spec/features/lsp/navigation/references.md](/docs/spec/features/lsp/navigation/references.md)
- Hover: [/docs/spec/features/lsp/hover.md](/docs/spec/features/lsp/hover.md)
