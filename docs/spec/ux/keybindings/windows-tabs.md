# Keybindings: Windows and Tabs

Complete nvim-compatible window and tab keybindings.

## Window Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-w h` | Window left | Move to window on left |
| `Ctrl-w j` | Window down | Move to window below |
| `Ctrl-w k` | Window up | Move to window above |
| `Ctrl-w l` | Window right | Move to window on right |
| `Ctrl-w w` | Next window | Cycle to next window |
| `Ctrl-w W` | Prev window | Cycle to previous window |
| `Ctrl-w t` | Top-left | Move to top-left window |
| `Ctrl-w b` | Bottom-right | Move to bottom-right window |
| `Ctrl-w p` | Previous | Move to previous window |

## Window Splitting

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-w s` | Split horizontal | Split window horizontally |
| `Ctrl-w v` | Split vertical | Split window vertically |
| `Ctrl-w n` | New split | New empty window (horizontal) |
| `:split` | Split horizontal | Split window horizontally |
| `:vsplit` | Split vertical | Split window vertically |
| `:new` | New horizontal | New empty horizontal split |
| `:vnew` | New vertical | New empty vertical split |

## Window Closing

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-w c` | Close window | Close current window |
| `Ctrl-w q` | Quit window | Quit current window |
| `Ctrl-w o` | Only window | Close all other windows |
| `:close` | Close | Close current window |
| `:only` | Only | Close all other windows |
| `ZZ` | Save and quit | Write buffer and close |
| `ZQ` | Force quit | Close without saving |

## Window Resizing (winresizer built-in)

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-w +` | Increase height | Increase window height |
| `Ctrl-w -` | Decrease height | Decrease window height |
| `Ctrl-w >` | Increase width | Increase window width |
| `Ctrl-w <` | Decrease width | Decrease window width |
| `Ctrl-w =` | Equalize | Make all windows equal size |
| `Ctrl-w _` | Max height | Maximize window height |
| `Ctrl-w \|` | Max width | Maximize window width |
| `<N>Ctrl-w +` | Height +N | Increase height by N |
| `<N>Ctrl-w -` | Height -N | Decrease height by N |
| `<N>Ctrl-w >` | Width +N | Increase width by N |
| `<N>Ctrl-w <` | Width -N | Decrease width by N |

## Window Movement

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-w H` | Move far left | Move window to far left |
| `Ctrl-w J` | Move far bottom | Move window to far bottom |
| `Ctrl-w K` | Move far top | Move window to far top |
| `Ctrl-w L` | Move far right | Move window to far right |
| `Ctrl-w r` | Rotate down | Rotate windows downward |
| `Ctrl-w R` | Rotate up | Rotate windows upward |
| `Ctrl-w x` | Exchange | Exchange with next window |
| `Ctrl-w T` | To tab | Move window to new tab |

## Tab Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `gt` | Next tab | Go to next tab |
| `gT` | Previous tab | Go to previous tab |
| `<N>gt` | Tab N | Go to tab number N |
| `:tabnext` | Next tab | Go to next tab |
| `:tabprev` | Previous tab | Go to previous tab |
| `:tabfirst` | First tab | Go to first tab |
| `:tablast` | Last tab | Go to last tab |

## Tab Management

| Key | Action | Description |
|-----|--------|-------------|
| `:tabnew` | New tab | Create new tab |
| `:tabclose` | Close tab | Close current tab |
| `:tabonly` | Only tab | Close all other tabs |
| `:tabmove N` | Move tab | Move tab to position N |
| `Ctrl-w gf` | Tab file | Open file under cursor in new tab |
| `Ctrl-w gF` | Tab file line | Open file:line in new tab |

## Buffer Commands

| Key | Action | Description |
|-----|--------|-------------|
| `:bnext` | Next buffer | Switch to next buffer |
| `:bprev` | Prev buffer | Switch to previous buffer |
| `:buffer N` | Buffer N | Switch to buffer number N |
| `:bdelete` | Delete buffer | Delete current buffer |
| `Ctrl-^` | Alternate | Switch to alternate buffer |
| `:buffers` | List buffers | Show buffer list |
| `:ls` | List buffers | Show buffer list |

