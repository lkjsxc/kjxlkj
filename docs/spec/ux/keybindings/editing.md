# Keybindings: Editing Operations

Complete nvim-compatible editing keybindings.

## Delete Operations

| Key | Action | Description |
|-----|--------|-------------|
| `x` | Delete char | Delete character under cursor |
| `X` | Delete before | Delete character before cursor |
| `dd` | Delete line | Delete entire line |
| `D` | Delete to EOL | Delete from cursor to end of line |
| `d<motion>` | Delete motion | Delete over motion range |
| `d<text-obj>` | Delete object | Delete text object |
| `J` | Join lines | Join current and next line |
| `gJ` | Join no space | Join without adding space |

## Yank (Copy) Operations

| Key | Action | Description |
|-----|--------|-------------|
| `yy` | Yank line | Copy entire line |
| `Y` | Yank line | Copy entire line (nvim default) |
| `y<motion>` | Yank motion | Copy over motion range |
| `y<text-obj>` | Yank object | Copy text object |
| `"<reg>y` | Yank to register | Copy to named register |

## Put (Paste) Operations

| Key | Action | Description |
|-----|--------|-------------|
| `p` | Put after | Paste after cursor/line |
| `P` | Put before | Paste before cursor/line |
| `gp` | Put after move | Paste after, cursor at end |
| `gP` | Put before move | Paste before, cursor at end |
| `]p` | Put indent after | Paste with adjusted indent |
| `[p` | Put indent before | Paste with adjusted indent |
| `"<reg>p` | Put from register | Paste from named register |
| `Ctrl-r<reg>` | Put in Insert | Insert register in Insert mode |

## Change Operations

| Key | Action | Description |
|-----|--------|-------------|
| `cc` | Change line | Change entire line |
| `C` | Change to EOL | Change from cursor to end |
| `c<motion>` | Change motion | Change over motion range |
| `c<text-obj>` | Change object | Change text object |
| `s` | Substitute | Delete char and enter Insert |
| `S` | Substitute line | Delete line and enter Insert |
| `r<char>` | Replace char | Replace single character |
| `R` | Replace mode | Enter Replace mode |

## Case Operations

| Key | Action | Description |
|-----|--------|-------------|
| `~` | Toggle case | Toggle case of character |
| `g~<motion>` | Toggle motion | Toggle case over motion |
| `g~~` | Toggle line | Toggle case of entire line |
| `gu<motion>` | Lowercase | Make motion range lowercase |
| `guu` | Lowercase line | Make line lowercase |
| `gU<motion>` | Uppercase | Make motion range uppercase |
| `gUU` | Uppercase line | Make line uppercase |

## Indentation

| Key | Action | Description |
|-----|--------|-------------|
| `>>` | Indent line | Indent current line |
| `<<` | Outdent line | Outdent current line |
| `><motion>` | Indent motion | Indent over motion |
| `<<motion>` | Outdent motion | Outdent over motion |
| `=<motion>` | Auto-indent | Re-indent over motion |
| `==` | Auto-indent line | Re-indent current line |
| `Ctrl-t` | Indent (Insert) | Add indent in Insert mode |
| `Ctrl-d` | Outdent (Insert) | Remove indent in Insert mode |

## Undo/Redo

| Key | Action | Description |
|-----|--------|-------------|
| `u` | Undo | Undo last change |
| `U` | Undo line | Undo all changes on line |
| `Ctrl-r` | Redo | Redo undone change |
| `.` | Repeat | Repeat last change |
| `g-` | Undo older | Go to older text state |
| `g+` | Redo newer | Go to newer text state |

## Insert Mode Editing

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-h` | Backspace | Delete character before cursor |
| `Ctrl-w` | Delete word | Delete word before cursor |
| `Ctrl-u` | Delete to start | Delete to start of line |
| `Ctrl-j` | New line | Insert new line |
| `Ctrl-m` | New line | Insert new line (same as Enter) |
| `Ctrl-a` | Insert prev | Insert previously inserted text |
| `Ctrl-@` | Insert + exit | Insert prev text and exit Insert |
| `Ctrl-y` | Copy above | Copy character from line above |
| `Ctrl-e` | Copy below | Copy character from line below |
| `Ctrl-k` | Digraph | Insert digraph |
| `Ctrl-v` | Literal | Insert literal character |
| `Ctrl-n` | Keyword next | Complete keyword (next match) |
| `Ctrl-p` | Keyword prev | Complete keyword (prev match) |

