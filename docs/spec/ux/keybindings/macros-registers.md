# Keybindings: Macros and Registers

Complete nvim-compatible macro and register keybindings.

## Macro Recording

| Key | Action | Description |
|-----|--------|-------------|
| `q<reg>` | Start recording | Begin recording macro to register |
| `q` | Stop recording | Stop current macro recording |
| `@<reg>` | Play macro | Execute macro from register |
| `@@` | Repeat macro | Replay last executed macro |
| `<N>@<reg>` | Play N times | Execute macro N times |
| `Q` | Ex mode | (Remapped to replay last macro) |

## Register Types

| Register | Description |
|----------|-------------|
| `"` | Unnamed register (default) |
| `0` | Last yank |
| `1-9` | Delete history (1=most recent) |
| `a-z` | Named registers (user storage) |
| `A-Z` | Append to named registers |
| `-` | Small delete (less than one line) |
| `.` | Last inserted text (read-only) |
| `:` | Last command (read-only) |
| `%` | Current filename (read-only) |
| `#` | Alternate filename (read-only) |
| `=` | Expression register |
| `*` | System clipboard (selection) |
| `+` | System clipboard (primary) |
| `_` | Black hole register (discard) |
| `/` | Last search pattern (read-only) |

## Using Registers

| Command | Action |
|---------|--------|
| `"ayy` | Yank line to register a |
| `"ap` | Paste from register a |
| `"Ayy` | Append yank to register a |
| `"+yy` | Yank line to system clipboard |
| `"+p` | Paste from system clipboard |
| `"*p` | Paste from selection clipboard |
| `"_dd` | Delete line (black hole) |
| `"0p` | Paste last yank |
| `:reg` | Show all registers |
| `:reg abc` | Show registers a, b, c |

## Insert Mode Register Access

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl-r <reg>` | Insert register | Insert register contents |
| `Ctrl-r Ctrl-r <reg>` | Literal | Insert literally |
| `Ctrl-r Ctrl-o <reg>` | No indent | Insert without auto-indent |
| `Ctrl-r Ctrl-p <reg>` | Fix indent | Insert with fixed indent |
| `Ctrl-r =` | Expression | Insert expression result |
| `Ctrl-r %` | Filename | Insert current filename |
| `Ctrl-r /` | Search | Insert last search pattern |
| `Ctrl-r :` | Command | Insert last command |
| `Ctrl-r .` | Last insert | Insert last inserted text |

## Marks

| Key | Action | Description |
|-----|--------|-------------|
| `m<a-z>` | Set local mark | Set mark in current buffer |
| `m<A-Z>` | Set global mark | Set mark (cross-buffer) |
| `'<a-z>` | Jump to mark line | Jump to mark (line) |
| `` `<a-z> `` | Jump to mark exact | Jump to mark (exact) |
| `'<A-Z>` | Jump global line | Jump to global mark (line) |
| `` `<A-Z> `` | Jump global exact | Jump to global mark (exact) |
| `''` | Previous line | Jump to previous position (line) |
| ``` `` ``` | Previous exact | Jump to previous position (exact) |
| `'.` | Last change line | Jump to last change (line) |
| `` `. `` | Last change exact | Jump to last change (exact) |
| `'^` | Last insert line | Jump to last insert (line) |
| `` `^ `` | Last insert exact | Jump to last insert (exact) |
| `'[` | Start of last op | Start of last operation |
| `']` | End of last op | End of last operation |
| `'<` | Start of visual | Start of last visual selection |
| `'>` | End of visual | End of last visual selection |
| `:marks` | List marks | Show all marks |
| `:delmarks a` | Delete mark | Delete mark a |
| `:delmarks!` | Delete all | Delete all lowercase marks |

## Special Marks

| Mark | Description |
|------|-------------|
| `'0-'9` | Last positions from previous sessions |
| `'(` `')` | Start/end of current sentence |
| `'{` `'}` | Start/end of current paragraph |

