# Keybindings: Mode Entry

Complete nvim-compatible mode entry keybindings.

## Insert Mode Entry

| Key | Action | Description |
|-----|--------|-------------|
| `i` | Insert before | Enter Insert mode before cursor |
| `I` | Insert at line start | Insert at first non-blank character |
| `gI` | Insert at column 0 | Insert at column 0 (true line start) |
| `a` | Append after | Enter Insert mode after cursor |
| `A` | Append at line end | Append at end of line (Shift+a) |
| `o` | Open below | New line below, enter Insert |
| `O` | Open above | New line above, enter Insert |
| `s` | Substitute char | Delete char, enter Insert |
| `S` | Substitute line | Delete line, enter Insert |
| `C` | Change to EOL | Delete to end, enter Insert |
| `cc` | Change line | Delete entire line, enter Insert |
| `gi` | Insert at last | Resume Insert at last position |

## Visual Mode Entry

| Key | Action | Description |
|-----|--------|-------------|
| `v` | Visual char | Character-wise visual selection |
| `V` | Visual line | Line-wise visual selection |
| `Ctrl-v` | Visual block | Block/column visual selection |
| `gv` | Reselect | Reselect last visual selection |
| `gn` | Select match | Select next search match |
| `gN` | Select prev match | Select previous search match |

## Replace Mode Entry

| Key | Action | Description |
|-----|--------|-------------|
| `R` | Replace mode | Overwrite characters |
| `gR` | Virtual replace | Replace with virtual space handling |
| `r<char>` | Replace single | Replace single character with char |
| `gr<char>` | Virtual replace | Virtual replace single character |

## Command Mode Entry

| Key | Action | Description |
|-----|--------|-------------|
| `:` | Command-line | Enter ex command mode |
| `/` | Search forward | Enter forward search mode |
| `?` | Search backward | Enter backward search mode |
| `q:` | Command history | Open command-line window |
| `q/` | Search history | Open search history window |
| `@:` | Repeat command | Repeat last ex command |

## Exiting Modes

| Key | Action | Description |
|-----|--------|-------------|
| `Esc` | Exit mode | Return to Normal mode |
| `Ctrl-[` | Exit mode | Alternative Escape |
| `Ctrl-c` | Cancel | Cancel operation, return to Normal |
| `Ctrl-o` | Single command | Execute one Normal command in Insert |

