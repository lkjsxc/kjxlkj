# Mapping Modes

Mode-specific key mappings.

## Overview

Create mappings that apply to specific
modes for precise keybinding control.

## Mode Types

### Available Modes

| Mode | Config Section | Vim Equivalent |
|------|----------------|----------------|
| `normal` | `[keys.normal]` | `nmap`/`nnoremap` |
| `insert` | `[keys.insert]` | `imap`/`inoremap` |
| `visual` | `[keys.visual]` | `xmap`/`xnoremap` |
| `select` | `[keys.select]` | `smap`/`snoremap` |
| `command` | `[keys.command]` | `cmap`/`cnoremap` |
| `terminal` | `[keys.terminal]` | `tmap`/`tnoremap` |
| `operator` | `[keys.operator]` | `omap`/`onoremap` |

## Configuration

### Mode-Specific Sections

Each mode has its own TOML section. Example:
`[keys.normal]` for normal mode mappings,
`[keys.insert]` for insert mode mappings.
Keys are the trigger, values are the action.

## Normal Mode

### Common Mappings

| Key | Action | Description |
|-----|--------|-------------|
| `<Space>` | `<Leader>` | Set space as leader |
| `<Leader>w` | `:w<CR>` | Quick save |
| `<Leader>q` | `:q<CR>` | Quick quit |
| `U` | `<C-r>` | Redo (more intuitive) |
| `Y` | `y$` | Yank to end of line |

### Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `<C-d>` | `<C-d>zz` | Scroll down + center |
| `<C-u>` | `<C-u>zz` | Scroll up + center |
| `n` | `nzzzv` | Next match + center |
| `N` | `Nzzzv` | Previous match + center |

## Insert Mode

### Escape Alternatives

| Key | Action | Description |
|-----|--------|-------------|
| `jk` | `<Esc>` | Quick escape |
| `jj` | `<Esc>` | Quick escape (alternative) |

### Quick Commands

| Key | Action | Description |
|-----|--------|-------------|
| `<C-s>` | `<C-o>:w<CR>` | Save without leaving insert |
| `<C-z>` | `<C-o>u` | Undo from insert |

### Text Expansion

Custom insert mappings can trigger abbreviation-like
expansions, though abbreviations are preferred for text
replacement.

## Visual Mode

### Selection Operations

| Key | Action | Description |
|-----|--------|-------------|
| `J` | `:m '>+1<CR>gv=gv` | Move selection down |
| `K` | `:m '<-2<CR>gv=gv` | Move selection up |
| `<` | `<gv` | Indent left, keep selection |
| `>` | `>gv` | Indent right, keep selection |

### Surround

| Key | Action | Description |
|-----|--------|-------------|
| `S"` | surround with `"` | Requires surround feature |
| `S(` | surround with `()` | Adds surrounding pair |

## Select Mode

### Different from Visual

Select mode mappings handle typed characters as
replacements. Mappings here typically handle special
keys only (arrows, function keys) since printable
characters replace the selection.

## Command Mode

### History Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `<C-p>` | Previous in history | Like `<Up>` |
| `<C-n>` | Next in history | Like `<Down>` |

### Quick Paste

| Key | Action | Description |
|-----|--------|-------------|
| `<C-r>"` | Paste unnamed register | Insert register content |
| `<C-r>+` | Paste clipboard | Insert clipboard |

## Operator-Pending Mode

### Custom Text Objects

| Key | Motion | Description |
|-----|--------|-------------|
| `il` | Inner line content | Line without indentation/newline |
| `al` | Around line | Full line including newline |
| `ie` | Inner entire buffer | All buffer content |

### Quick Operations

Delete/change/yank using custom motions defined in
operator-pending mode.

## Terminal Mode

### Exit Terminal

| Key | Action | Description |
|-----|--------|-------------|
| `<Esc><Esc>` | Exit terminal mode | Double escape |
| `<C-\\><C-n>` | Exit terminal mode | Standard method |

### Window Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `<C-w>h` | Focus left window | Navigate from terminal |
| `<C-w>j` | Focus below window | Navigate from terminal |
| `<C-w>k` | Focus above window | Navigate from terminal |
| `<C-w>l` | Focus right window | Navigate from terminal |

## Multi-Mode Mappings

### Apply to Multiple

Use the same key binding in multiple mode sections.
There is no shorthand for applying one mapping to 
multiple modes in TOML config.

### All Modes

To map a key in all modes, add it to each section
individually.

## Recursive vs Non-Recursive

### Non-Recursive (Default)

All mappings are non-recursive by default (equivalent
to Vim's `noremap` variants). This prevents mapping
chains and ensures predictable behavior. Recursive
mappings can be enabled per-mapping with a `recursive`
flag if needed.
