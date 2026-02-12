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
| `A` | Append at line end | move to true line end and enter Insert |
| `o` | Open below | insert new line below, then Insert |
| `O` | Open above | insert new line above, then Insert |
| `s` | Substitute char | replace char, then Insert |
| `S` | Substitute line | replace line, then Insert |
| `C` | Change to EOL | delete to EOL, then Insert |
| `cc` | Change line | change full line, then Insert |
| `gi` | Resume last insert | return to last insert position |

## Printable Shift Normalization (normative)

Normalization of printable shifted keys is mandatory before mode dispatch.

| Raw Input | Required Dispatch |
|---|---|
| `Shift+a` | `A` |
| `Shift+o` | `O` |
| `Shift+i` | `I` |

Mode handlers MUST consume normalized keys only.

## Critical Behavioral Distinction

`i`, `a`, and `A` are distinct and must remain distinct.

| Key | Pre-Insert Cursor Transition | Expected First Insert Cell |
|---|---|---|
| `i` | no move | current cursor cell |
| `a` | advance one grapheme when possible | cell after current grapheme |
| `A` | jump to true line end insertion point | end-of-line insertion point |

## End-of-Line Append Boundary (normative)

Given line `abc` with cursor on `c`:

| Input | Expected Result |
|---|---|
| `iX<Esc>` | `abXc` |
| `aX<Esc>` | `abcX` |
| `AX<Esc>` | `abcX` |

Given empty line with cursor at column `0`, `i`, `a`, and `A` all insert at
column `0`, but mode-entry trace IDs must still distinguish command identity.

## Screen-Visible Guarantees

| Guarantee | Required Observation |
|---|---|
| `A` append | inserted text appears at line end, not at cursor+1 unless cursor already at EOL |
| normalization symmetry | physical `A` and `Shift+a` produce identical state dumps and frames |
| mode switch timing | mode indicator changes to Insert before inserted text appears |

## Visual, Replace, and Command Entry

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

| ID | Scenario | Required Assertions |
|---|---|---|
| `KEYMODE-01` | `Shift+a` dispatches as `A` | normalized key is `A` and resulting frame equals physical `A` run |
| `KEYMODE-02` | `a` at EOL differs from `i` | inserted text position differs exactly as specified |
| `KEYMODE-03` | `A` appends at true EOL | final line content equals append-at-EOL expectation |
| `KEYMODE-04R` | `i` vs `a` at EOL replay | per-key dump timeline proves different cursor transition before first insert |
| `WR-01R` | raw key path for `Shift+a` | per-step state dump shows decode -> normalize -> dispatch -> frame |
| `KEY-SCREEN-01` | user-like append check | frame snapshots after each key match oracle |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- E2E testing: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
