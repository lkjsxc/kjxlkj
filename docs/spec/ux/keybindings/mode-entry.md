# Keybindings: Mode Entry

Back: [/docs/spec/ux/keybindings/README.md](/docs/spec/ux/keybindings/README.md)

Mode-entry keybindings and normalization rules.

## Insert Mode Entry

| Key | Action | Description |
|---|---|---|
| `i` | Insert before | enter Insert at current cursor offset |
| `I` | Insert at first non-blank | enter Insert at first non-blank |
| `gI` | Insert at column 0 | enter Insert at true line start |
| `a` | Append after | enter Insert after current grapheme |
| `A` | Append at line end | move to line end and append |
| `o` | Open below | new line below, then Insert |
| `O` | Open above | new line above, then Insert |
| `s` | Substitute char | replace char, then Insert |
| `S` | Substitute line | replace line, then Insert |
| `C` | Change to EOL | delete to EOL, then Insert |
| `cc` | Change line | change full line, then Insert |
| `gi` | Resume last insert | return to last insert position |

## Printable Shift Normalization

Normalization of printable shifted keys is mandatory and happens before mode handler dispatch.

| Raw Input | Required Dispatch |
|---|---|
| `Shift+a` | `A` |
| `Shift+o` | `O` |
| `Shift+i` | `I` |

Mode handlers MUST NOT depend on raw shift modifier for printable commands.

## Critical Distinction

`a` and `A` MUST NOT share behavior.

| Key | Required Cursor Transition Before Insert |
|---|---|
| `a` | move one grapheme right if possible |
| `A` | move to true end-of-line insertion point |

## Visual, Replace, Command Entry

| Key | Action |
|---|---|
| `v` / `V` / `Ctrl-v` | Visual char / line / block |
| `gv` | reselect last visual area |
| `R` / `gR` | Replace / virtual replace |
| `:` | Ex command-line |
| `/` / `?` | search entry |
| `q:` / `q/` | command-line or search history window |

## Exit Keys

| Key | Action |
|---|---|
| `Esc` | return to Normal |
| `Ctrl-[` | return to Normal |
| `Ctrl-c` | cancel current operation |
| `Ctrl-o` | run one Normal command from Insert |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `KEYMODE-01` | `Shift+a` dispatches as `A` through real decoder path |
| `KEYMODE-02` | `a` at EOL differs from `i` |
| `KEYMODE-03` | `A` appends at true end-of-line |
| `KEYMODE-04` | normalization remains correct with IME and leader mappings |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- E2E testing: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
