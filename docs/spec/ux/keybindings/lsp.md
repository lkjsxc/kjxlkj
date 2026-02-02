# Keybindings: LSP

Complete nvim-compatible LSP keybindings.

## Go To Commands

| Key | Action | Description |
|-----|--------|-------------|
| `gd` | Definition | Go to definition |
| `gD` | Declaration | Go to declaration |
| `gi` | Implementation | Go to implementation |
| `gr` | References | Find all references |
| `gy` | Type definition | Go to type definition |
| `gf` | File | Go to file under cursor |
| `gF` | File:line | Go to file:line under cursor |
| `Ctrl-]` | Definition | Go to definition (tag style) |
| `Ctrl-w ]` | Def split | Definition in split |
| `Ctrl-w }` | Def preview | Definition in preview |

## Hover and Info

| Key | Action | Description |
|-----|--------|-------------|
| `K` | Hover | Show hover documentation |
| `<leader>k` | Signature | Show signature help |
| `Ctrl-k` | Signature (Insert) | Signature help in Insert mode |

## Code Actions

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>ca` | Code action | Show code actions |
| `<leader>cf` | Format | Format buffer/selection |
| `<leader>rn` | Rename | Rename symbol |
| `<leader>qf` | Quickfix | Apply quickfix |

## Diagnostics

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>d` | Diagnostics | Show buffer diagnostics |
| `<leader>D` | All diagnostics | Show all diagnostics |
| `[d` | Previous diagnostic | Jump to previous diagnostic |
| `]d` | Next diagnostic | Jump to next diagnostic |
| `[e` | Previous error | Jump to previous error |
| `]e` | Next error | Jump to next error |
| `[w` | Previous warning | Jump to previous warning |
| `]w` | Next warning | Jump to next warning |
| `<leader>e` | Line diagnostic | Show line diagnostics |

## Workspace

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>wa` | Add folder | Add workspace folder |
| `<leader>wr` | Remove folder | Remove workspace folder |
| `<leader>wl` | List folders | List workspace folders |
| `<leader>ws` | Workspace symbols | Search workspace symbols |
| `<leader>ds` | Document symbols | Search document symbols |

## Completion (Insert Mode)

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-Space` | Trigger | Trigger completion |
| `Ctrl-n` | Next | Select next completion |
| `Ctrl-p` | Previous | Select previous completion |
| `Ctrl-y` | Confirm | Confirm completion |
| `Ctrl-e` | Cancel | Cancel completion |
| `Tab` | Next/expand | Next item or expand snippet |
| `Shift-Tab` | Previous | Previous item |

## Snippets

| Key | Action | Description |
|-----|--------|-------------|
| `Tab` | Expand | Expand snippet |
| `Ctrl-l` | Next placeholder | Jump to next placeholder |
| `Ctrl-h` | Prev placeholder | Jump to previous placeholder |
| `Ctrl-c` | Exit snippet | Exit snippet mode |

## Call Hierarchy

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>ci` | Incoming calls | Show incoming calls |
| `<leader>co` | Outgoing calls | Show outgoing calls |

## Type Hierarchy

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>ti` | Subtypes | Show subtypes |
| `<leader>to` | Supertypes | Show supertypes |

