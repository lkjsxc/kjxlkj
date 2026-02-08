# Mapping Modes

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Key mappings are scoped to specific editor modes.

## Overview

Each mapping is defined for one or more modes. A key sequence can have different meanings in different modes.

## Mode Types

| Mode Flag | Mode | Mapping Command |
|---|---|---|
| `n` | Normal | `nmap` / `nnoremap` |
| `i` | Insert | `imap` / `inoremap` |
| `v` | Visual + Select | `vmap` / `vnoremap` |
| `x` | Visual only | `xmap` / `xnoremap` |
| `s` | Select only | `smap` / `snoremap` |
| `c` | Command-line | `cmap` / `cnoremap` |
| `o` | Operator-pending | `omap` / `onoremap` |
| `t` | Terminal | `tmap` / `tnoremap` |

## Normal Mode

Common normal mode mappings:

| Mapping | Function | Example |
|---|---|---|
| Leader sequences | Custom commands | `<leader>w` → `:w<CR>` |
| Navigation | Quick jumps | `gd` → goto definition |
| Window management | Split/resize | `<leader>v` → `:vsplit<CR>` |

## Insert Mode

| Mapping | Function | Example |
|---|---|---|
| Escape alternatives | Quick exit | `jk` → `<Esc>` |
| Quick commands | Without leaving insert | `<C-s>` → `<C-o>:w<CR>` |
| Text expansion | Template insert | Abbreviation-style mappings |

## Visual Mode

| Mapping | Function | Example |
|---|---|---|
| Selection operations | Act on selection | `<leader>c` → comment selection |
| Surround | Wrap selection | `S"` → surround with quotes |
| Move lines | Shift selected lines | `J` → `:m '>+1<CR>gv=gv` |

## Select Mode

Select mode is like visual mode but typing replaces the selection. Mappings defined with `smap` only apply in select mode, not visual mode.

## Command Mode

| Mapping | Function | Example |
|---|---|---|
| History nav | Quick history | `<C-p>` → `<Up>` |
| Quick paste | Insert register | `<C-r>"` → paste default register |
| Abbreviation | Expand commands | `W` → `w` (typo fix) |

## Operator-Pending Mode

| Mapping | Function | Example |
|---|---|---|
| Custom text objects | Inner/around | `omap ih` → inner heading |
| Quick operations | Shorthand | `omap s` → sentence |

## Terminal Mode

| Mapping | Function | Example |
|---|---|---|
| Exit terminal | Return to normal | `<C-\><C-n>` → normal mode |
| Window nav | Switch windows | `<C-w>h` → left window |

## Multi-Mode Mappings

Use `map` and `noremap` to define mappings for Normal, Visual, Select, and Operator-pending modes simultaneously.

## Recursive vs Non-Recursive

| Command | Recursive |
|---|---|
| `nmap` | Yes — RHS is remapped |
| `nnoremap` | No — RHS uses default keys |

Always prefer `nnoremap` (non-recursive) unless recursion is intentionally needed.

## Related

- Key mappings: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Leader key: [/docs/spec/scripting/mappings/leader-key.md](/docs/spec/scripting/mappings/leader-key.md)
