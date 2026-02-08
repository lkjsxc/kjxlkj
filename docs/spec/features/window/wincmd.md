# Window Commands (:wincmd)

Complete window command reference.

## Overview

The `:wincmd` command executes window
operations, equivalent to `<C-w>` keys.

## Syntax

| Form | Example | Notes |
|------|---------|-------|
| Normal mode | `<C-w>{char}` | Standard key sequence |
| Ex command | `:wincmd {char}` | For scripts and mappings |
| With count | `[count]<C-w>{char}` | Count applies as size or target |
| Ex with count | `:[count]wincmd {char}` | Same semantics, Ex form |

## Navigation
### Direction
| Command | Action |
|---------|--------|
| `:wincmd h` | Go to left window |
| `:wincmd j` | Go to below window |
| `:wincmd k` | Go to above window |
| `:wincmd l` | Go to right window |

### Other Navigation
| Command | Action |
|---------|--------|
| `:wincmd w` | Next window (wrap) |
| `:wincmd W` | Previous window (wrap) |
| `:wincmd t` | Go to top-left window |
| `:wincmd b` | Go to bottom-right window |
| `:wincmd p` | Go to previous window |

## Creating Windows
### Splits
| Command | Action |
|---------|--------|
| `:wincmd s` | Split horizontally |
| `:wincmd v` | Split vertically |
| `:wincmd n` | New window |

### With Count
| Command | Action |
|---------|--------|
| `[count]<C-w>s` | Split with `count`-line height |
| `[count]<C-w>v` | Split with `count`-column width |
| `[count]<C-w>n` | New window with `count`-line height |

## Closing Windows
### Close Commands
| Command | Action |
|---------|--------|
| `:wincmd c` | Close current window |
| `:wincmd q` | Quit current window |
| `:wincmd o` | Only: close others |

### Close Specific
| Command | Action |
|---------|--------|
| `:close` | Close current window; error if last |
| `:close!` | Close current, discard unsaved changes |
| `:only` | Close all other windows in tab |
| `:only!` | Close others, discard unsaved changes |
| `:[count]close` | Close window number `count` |

## Moving Windows
### Position
| Command | Action |
|---------|--------|
| `:wincmd H` | Move window far left (full height) |
| `:wincmd J` | Move window far bottom (full width) |
| `:wincmd K` | Move window far top (full width) |
| `:wincmd L` | Move window far right (full height) |
| `:wincmd T` | Move window to new tab |

## Rotating Windows
### Rotation
| Command | Action |
|---------|--------|
| `:wincmd r` | Rotate downwards/rightwards |
| `:wincmd R` | Rotate upwards/leftwards |
| `:wincmd x` | Exchange with next window |

## Resizing Windows
### Size Adjustments
| Command | Action |
|---------|--------|
| `:wincmd +` | Increase height |
| `:wincmd -` | Decrease height |
| `:wincmd >` | Increase width |
| `:wincmd <` | Decrease width |
| `:wincmd =` | Make all equal size |
| `:wincmd _` | Maximize height |
| `:wincmd \|` | Maximize width |

### With Count
| Command | Action |
|---------|--------|
| `[count]<C-w>+` | Increase height by `count` rows |
| `[count]<C-w>-` | Decrease height by `count` rows |
| `[count]<C-w>>` | Increase width by `count` columns |
| `[count]<C-w><` | Decrease width by `count` columns |
| `[count]<C-w>_` | Set height to exactly `count` rows |
| `[count]<C-w>\|` | Set width to exactly `count` columns |

## Special Windows
### Preview Window
| Command | Action |
|---------|--------|
| `:wincmd P` | Go to preview window |
| `:wincmd z` | Close preview window |

### Quickfix
| Command | Action |
|---------|--------|
| `:copen` | Open quickfix window |
| `:cclose` | Close quickfix window |
| `:lopen` | Open location list window |
| `:lclose` | Close location list window |

## Window Information
### Current Window
| Function | Returns |
|----------|---------|
| `winnr()` | Current window number |
| `win_getid()` | Current window unique ID |
| `winheight(0)` | Height of current window in rows |
| `winwidth(0)` | Width of current window in columns |

### All Windows
| Command/Function | Action |
|------------------|--------|
| `:windo {cmd}` | Execute `cmd` in every window |
| `<C-w>=` | Equalize all window sizes |
| `winnr('$')` | Total window count in current tab |
| `getwininfo()` | List of info dicts for all windows |

## Keybinding Equivalents
### Normal Mode
| Key | Equivalent |
|-----|------------|
| `<C-w>h` | `:wincmd h` |
| `<C-w>j` | `:wincmd j` |
| `<C-w>k` | `:wincmd k` |
| `<C-w>l` | `:wincmd l` |
| `<C-w>s` | `:wincmd s` |
| `<C-w>v` | `:wincmd v` |
| `<C-w>c` | `:wincmd c` |
| `<C-w>o` | `:wincmd o` |

## Scripting
### In Commands
| Usage | Example |
|-------|---------|
| Mapping | `nnoremap <Leader>h :wincmd h<CR>` |
| Function call | `execute 'wincmd l'` |
| Chained | `:wincmd s \| wincmd j` |

### In Autocommands
| Event | Example |
|-------|---------|
| Resize equalize | `autocmd VimResized * wincmd =` |
| Help placement | `autocmd FileType help wincmd K` |
| Log right-side | `autocmd BufWinEnter *.log wincmd L` |

## Window Variables
### Set Variable
| Command | Action |
|---------|--------|
| `:let w:name = val` | Set window-local variable |
| `setwinvar(nr, 'name', val)` | Set variable in window `nr` |

### Check Variable
| Expression | Returns |
|------------|---------|
| `w:name` | Value of window-local variable |
| `exists('w:name')` | `1` if exists, `0` otherwise |
| `getwinvar(nr, 'name')` | Variable from window `nr` |

## Window Options
### List Options
| Option | Default | Description |
|--------|---------|-------------|
| `winheight` | `1` | Min height for current window |
| `winwidth` | `1` | Min width for current window |
| `winminheight` | `1` | Min height for non-current windows |
| `winminwidth` | `1` | Min width for non-current windows |

### Window-Local
| Option | Scope | Description |
|--------|-------|-------------|
| `wrap` | Window | Enable line wrapping |
| `number` | Window | Show line numbers |
| `signcolumn` | Window | Sign column display |
| `foldmethod` | Window | Folding method |
| `scrolloff` | Window | Vertical scroll margin |
