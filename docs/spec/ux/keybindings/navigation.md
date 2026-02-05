# Keybindings: Navigation

Complete nvim-compatible navigation keybindings.

## Character Movement

| Key | Action | Description |
|-----|--------|-------------|
| `h` | Left | Move left one character |
| `l` | Right | Move right one character |
| `j` | Down | Move down one line |
| `k` | Up | Move up one line |
| `gj` | Display down | Move down one display line |
| `gk` | Display up | Move up one display line |
| `Backspace` | Back char | Move left one character |

Note: `Space` is reserved for `<leader>` by default (see [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)); use `l` for rightward motion.

## Line Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `0` | Line start | Jump to column 0 |
| `^` | First non-blank | Jump to first non-whitespace |
| `$` | Line end | Jump to end of line |
| `g_` | Last non-blank | Jump to last non-whitespace |
| `g0` | Display start | First character of display line |
| `g^` | Display non-blank | First non-blank of display line |
| `g$` | Display end | End of display line |
| `gm` | Middle of line | Jump to middle of line |
| `gM` | Middle of screen | Jump to middle of screen line |
| `+` | Next line start | First non-blank of next line |
| `-` | Prev line start | First non-blank of previous line |
| `Enter` | Next line | Same as + |
| `_` | Current line | First non-blank (with count offset) |
| `\|` | Column | Go to column N |

## Word Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `w` | Word forward | Start of next word |
| `W` | WORD forward | Start of next WORD |
| `e` | Word end | End of current/next word |
| `E` | WORD end | End of current/next WORD |
| `b` | Word back | Start of previous word |
| `B` | WORD back | Start of previous WORD |
| `ge` | Word end back | End of previous word |
| `gE` | WORD end back | End of previous WORD |

## Document Navigation

| Key | Action | Description |
|-----|--------|-------------|
| `gg` | File start | First line of file |
| `G` | File end | Last line of file |
| `<N>G` | Go to line | Go to line N |
| `<N>gg` | Go to line | Go to line N |
| `H` | Screen top | Top of visible screen |
| `M` | Screen middle | Middle of visible screen |
| `L` | Screen bottom | Bottom of visible screen |
| `<N>%` | Percentage | Go to N% of file |

## Scroll Commands

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-f` | Page down | Scroll down full page |
| `Ctrl-b` | Page up | Scroll up full page |
| `Ctrl-d` | Half down | Scroll down half page |
| `Ctrl-u` | Half up | Scroll up half page |
| `Ctrl-e` | Scroll down | Scroll down one line |
| `Ctrl-y` | Scroll up | Scroll up one line |
| `zz` | Center cursor | Center cursor line on screen |
| `zt` | Cursor to top | Move cursor line to top |
| `zb` | Cursor to bottom | Move cursor line to bottom |
| `z<CR>` | Top + first | Cursor to top, first non-blank |
| `z.` | Center + first | Center, first non-blank |
| `z-` | Bottom + first | Bottom, first non-blank |
| `zh` | Scroll right | Scroll right one character |
| `zl` | Scroll left | Scroll left one character |
| `zH` | Scroll right half | Scroll right half screen |
| `zL` | Scroll left half | Scroll left half screen |
| `zs` | Scroll to cursor | Scroll cursor to left |
| `ze` | Scroll to cursor | Scroll cursor to right |

## Character Search

| Key | Action | Description |
|-----|--------|-------------|
| `f<char>` | Find forward | Find char forward in line |
| `F<char>` | Find backward | Find char backward in line |
| `t<char>` | Till forward | Till char forward (before) |
| `T<char>` | Till backward | Till char backward (after) |
| `;` | Repeat find | Repeat last f/F/t/T |
| `,` | Reverse find | Repeat in opposite direction |

## Search

| Key | Action | Description |
|-----|--------|-------------|
| `/pattern` | Search forward | Search forward for pattern |
| `?pattern` | Search backward | Search backward for pattern |
| `n` | Next match | Go to next search match |
| `N` | Previous match | Go to previous search match |
| `*` | Word forward | Search word under cursor forward |
| `#` | Word backward | Search word under cursor backward |
| `g*` | Partial forward | Search partial word forward |
| `g#` | Partial backward | Search partial word backward |
| `gd` | Local definition | Go to local definition |
| `gD` | Global definition | Go to global definition |

## Jump List

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-o` | Jump back | Go to older jump position |
| `Ctrl-i` | Jump forward | Go to newer jump position |
| `g;` | Change back | Go to older change position |
| `g,` | Change forward | Go to newer change position |
| `''` | Last jump line | Jump to line before last jump |
| ``` `` ``` | Last jump exact | Jump to exact position before jump |
| `'.` | Last change | Jump to last change position |
| `'^` | Last insert | Jump to last insert position |
