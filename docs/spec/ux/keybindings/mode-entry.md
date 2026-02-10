# Keybindings: Mode Entry

Back: [/docs/spec/ux/keybindings/README.md](/docs/spec/ux/keybindings/README.md)

Complete mode-entry keybindings and normalization rules.

## Insert Mode Entry

| Key | Action | Description |
|---|---|---|
| `i` | Insert before | Enter Insert at current cursor offset |
| `I` | Insert at first non-blank | Enter Insert at first non-blank |
| `gI` | Insert at column 0 | Enter Insert at true line start |
| `a` | Append after | Enter Insert after current grapheme |
| `A` | Append at line end | Move to line end and append |
| `o` | Open below | New line below, then Insert |
| `O` | Open above | New line above, then Insert |
| `s` | Substitute char | Replace char, then Insert |
| `S` | Substitute line | Replace line, then Insert |
| `C` | Change to EOL | Delete to EOL, then Insert |
| `cc` | Change line | Change full line, then Insert |
| `gi` | Resume last insert | Return to last Insert position |

## Shifted Printable Normalization

| Rule | Requirement |
|---|---|
| Printable shift normalization | Shifted printable keys MUST normalize before mode handler dispatch |
| No raw-shift dependency | Mode handlers MUST NOT require raw Shift modifier matching for printable commands |

| Input | Required Behavior |
|---|---|
| `Shift+a` in Normal mode | Dispatch as `A` append-at-EOL |
| `Shift+o` in Normal mode | Dispatch as `O` open-line-above |
| `Shift+i` in Normal mode | Dispatch as `I` insert-first-non-blank |

## Critical Distinction

`a` and `A` MUST NOT share identical behavior.

| Key | Required cursor transition before Insert |
|---|---|
| `a` | move one grapheme right if possible |
| `A` | move to end-of-line insertion point |

## Visual Mode Entry

| Key | Action |
|---|---|
| `v` | Visual char |
| `V` | Visual line |
| `Ctrl-v` | Visual block |
| `gv` | Reselect last visual area |

## Replace and Command Entry

| Key | Action |
|---|---|
| `R` | Replace mode |
| `gR` | Virtual replace mode |
| `:` | Ex command-line |
| `/` | Forward search |
| `?` | Backward search |
| `q:` | Command-line window |
| `q/` | Search history window |

## Exit Keys

| Key | Action |
|---|---|
| `Esc` | Return to Normal |
| `Ctrl-[` | Return to Normal |
| `Ctrl-c` | Cancel current operation |
| `Ctrl-o` | Execute one Normal command from Insert |

## Mandatory Verification

| ID | Scenario |
|---|---|
| KEYMODE-01 | `Shift+a` dispatches as `A` |
| KEYMODE-02 | `a` at EOL differs from `i` |
| KEYMODE-03 | `A` always appends at true end-of-line |
| KEYMODE-04 | Normalization works through the real input decoder path |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- Testing matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
