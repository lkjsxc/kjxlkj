# Keybindings: Features

Built-in feature keybindings with leader key.

## Leader Key

The default leader key is `Space`. All `<leader>` bindings use Space as prefix.

## File Explorer (nvim-tree.lua built-in)

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>e` | Toggle explorer | Open/close file explorer |
| `<leader>E` | Explorer reveal | Open explorer at current file |

### Explorer Window Keys

| Key | Action | Description |
|-----|--------|-------------|
| `j` / `k` | Navigate | Move up/down in tree |
| `h` | Collapse/parent | Collapse dir or go to parent |
| `l` | Expand/open | Expand dir or open file |
| `Enter` | Open | Open file or toggle directory |
| `o` | Open | Open file in current window |
| `v` | Vertical split | Open in vertical split |
| `s` | Horizontal split | Open in horizontal split |
| `t` | New tab | Open in new tab |
| `a` | Create file | Create new file |
| `A` | Create directory | Create new directory |
| `d` | Delete | Delete file/directory (trash) |
| `D` | Force delete | Permanently delete |
| `r` | Rename | Rename file/directory |
| `x` | Cut | Cut file/directory |
| `c` | Copy | Copy file/directory |
| `p` | Paste | Paste cut/copied item |
| `y` | Copy name | Copy filename |
| `Y` | Copy path | Copy relative path |
| `gy` | Copy abs path | Copy absolute path |
| `R` | Refresh | Refresh tree |
| `H` | Toggle hidden | Toggle hidden files |
| `I` | Toggle gitignore | Toggle gitignored files |
| `/` | Search | Filter tree |
| `q` / `Esc` | Close | Close explorer |

## Terminal (toggleterm.nvim built-in)

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>t` | Toggle terminal | Open/close terminal |
| `<leader>tf` | Floating terminal | Open floating terminal |
| `<leader>th` | Horizontal term | Open horizontal split term |
| `<leader>tv` | Vertical term | Open vertical split term |
| `<leader>tn` | New terminal | Create new terminal |
| `Ctrl-\` | Toggle terminal | Quick toggle terminal |

### Terminal Mode Keys

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-\ Ctrl-n` | Normal mode | Exit to Normal mode |
| `<Esc><Esc>` | Normal mode | Exit to Normal mode |
| `Ctrl-w h` | Window left | Move to window left |
| `Ctrl-w j` | Window down | Move to window down |
| `Ctrl-w k` | Window up | Move to window up |
| `Ctrl-w l` | Window right | Move to window right |

## Fuzzy Finder

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>f` | Find files | Open file finder |
| `<leader>g` | Live grep | Search text in files |
| `<leader>b` | Buffers | Find open buffers |
| `<leader>r` | Recent files | Find recent files |
| `<leader>h` | Help | Search help tags |
| `<leader>c` | Commands | Command palette |
| `<leader>p` | Command palette | Open command palette |
| `<leader>m` | Marks | Find marks |
| `<leader>/` | Current buffer | Search in current buffer |
| `<leader>s` | Document symbols | Search document symbols |
| `<leader>S` | Workspace symbols | Search workspace symbols |

### Finder Window Keys

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-n` / `Ctrl-j` | Next item | Move to next result |
| `Ctrl-p` / `Ctrl-k` | Previous item | Move to previous result |
| `Enter` | Open | Open selected item |
| `Ctrl-s` | Split horizontal | Open in horizontal split |
| `Ctrl-v` | Split vertical | Open in vertical split |
| `Ctrl-t` | New tab | Open in new tab |
| `Ctrl-u` | Preview up | Scroll preview up |
| `Ctrl-d` | Preview down | Scroll preview down |
| `Esc` / `Ctrl-c` | Close | Close finder |

## Git Integration

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>gs` | Git status | Open git status |
| `<leader>gb` | Git blame | Toggle blame |
| `<leader>gd` | Git diff | Show diff |
| `<leader>gc` | Git commits | Browse commits |
| `<leader>gB` | Git branches | Browse branches |
| `]c` | Next hunk | Jump to next git hunk |
| `[c` | Previous hunk | Jump to previous git hunk |
| `<leader>hp` | Preview hunk | Preview current hunk |
| `<leader>hs` | Stage hunk | Stage current hunk |
| `<leader>hu` | Unstage hunk | Unstage current hunk |
| `<leader>hr` | Reset hunk | Reset current hunk |

