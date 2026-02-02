# Keybindings: Visual Mode

Complete nvim-compatible Visual mode keybindings.

## Entering Visual Mode

| Key | Action | Description |
|-----|--------|-------------|
| `v` | Visual char | Character-wise selection |
| `V` | Visual line | Line-wise selection |
| `Ctrl-v` | Visual block | Block/column selection |
| `gv` | Reselect | Reselect last visual area |

## Selection Movement

All Normal mode motions extend the selection:

| Key | Action |
|-----|--------|
| `h/j/k/l` | Extend by character/line |
| `w/W/e/E/b/B` | Extend by word |
| `0/^/$` | Extend to line positions |
| `gg/G` | Extend to file start/end |
| `%` | Extend to matching bracket |
| `f/F/t/T` | Extend to character |
| `iw/aw` | Select word object |
| `i"/a"` | Select quoted string |
| `i(/a(` | Select parentheses |

## Selection Commands

| Key | Action | Description |
|-----|--------|-------------|
| `o` | Swap ends | Move cursor to other end |
| `O` | Swap corners | Swap block corners (Visual Block) |
| `$` | Extend to EOL | Extend to end of all lines |

## Operators on Selection

| Key | Action | Description |
|-----|--------|-------------|
| `d` / `x` | Delete | Delete selection |
| `y` | Yank | Copy selection |
| `c` | Change | Delete and enter Insert |
| `s` | Substitute | Same as c |
| `r<char>` | Replace | Replace all chars with char |
| `J` | Join | Join selected lines |
| `gJ` | Join no space | Join without spaces |
| `>` | Indent | Indent selection |
| `<` | Outdent | Outdent selection |
| `=` | Format | Auto-indent selection |
| `gq` | Format | Format text |

## Case Operations

| Key | Action | Description |
|-----|--------|-------------|
| `u` | Lowercase | Make selection lowercase |
| `U` | Uppercase | Make selection uppercase |
| `~` | Toggle case | Toggle case of selection |
| `g~` | Toggle case | Toggle case |
| `gu` | Lowercase | Make lowercase |
| `gU` | Uppercase | Make uppercase |

## Visual Block Operations

| Key | Action | Description |
|-----|--------|-------------|
| `I` | Insert | Insert at block start |
| `A` | Append | Append at block end |
| `c` | Change | Change block content |
| `C` | Change | Change to end of block |
| `$` | Extend to EOL | Extend to longest line |
| `r<char>` | Replace all | Replace all chars |

## Increment/Decrement

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-a` | Increment | Increment numbers |
| `Ctrl-x` | Decrement | Decrement numbers |
| `g Ctrl-a` | Seq increment | Increment as sequence |
| `g Ctrl-x` | Seq decrement | Decrement as sequence |

## Exiting Visual

| Key | Action |
|-----|--------|
| `Esc` | Exit to Normal |
| `Ctrl-c` | Cancel and exit |
| `v` | Toggle to char visual |
| `V` | Toggle to line visual |
| `Ctrl-v` | Toggle to block visual |

