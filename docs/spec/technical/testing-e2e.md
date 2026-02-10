# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

This matrix is the required high-leverage E2E set.

## Critical Workflow Tests (Happy Path)

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| HE-01 | create, edit, save, quit | file bytes on disk match inserted content |
| HE-02 | open, navigate, quit without save | cursor reaches target line; file unchanged |
| HE-03 | split and edit | split exists; edits land in intended window |
| HE-04 | explorer open file | selected file opens in current window |
| HE-05 | explorer open split | selected file opens in horizontal and vertical targets |
| HE-06 | terminal open and command | terminal window opens and displays command output |
| HE-07 | session save/load roundtrip | layout, focused window, and cursors are restored |
| HE-08 | command-line option change | `:set` updates next rendered frame deterministically |

## Wiring Regression Tests (Known Failures)

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| WR-01 | `Shift+a` in Normal mode | dispatches as `A` append-at-EOL |
| WR-02 | `a` at end-of-line | insertion point moves after last grapheme, not `i` semantics |
| WR-03 | `:terminal` command route | parser -> action -> service -> visible terminal window |
| WR-04 | `<leader>t` route | key route reaches same terminal spawn path as `:terminal` |
| WR-05 | `:Explorer` and `<leader>e` route | both routes produce visible explorer window |
| WR-06 | `Ctrl-w` mixed windows | directional focus works across buffer/explorer/terminal |
| WR-07 | long-line display safety | no rendered text extends beyond window bounds |
| WR-08 | repeated `a` + `Esc` | no floating end-inclusive cursor in Normal mode |

## Locale and IME Tests

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| JP-01 | IME compose and commit | committed Japanese text inserted atomically |
| JP-02 | IME cancel | committed buffer unchanged |
| JP-03 | IME `Space` with leader configured | candidate cycling does not trigger leader mapping |
| JP-04 | CJK append after commit | `a` and `A` retain correct semantics |
| JP-05 | mixed ASCII/CJK search and edit | cursor and highlight remain grapheme-safe |

## Boundary and Stress Tests

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| BD-01 | 10k ASCII line with wrap | deterministic continuation rows; no overflow |
| BD-02 | 10k CJK line with wrap | no split wide grapheme across rows |
| BD-03 | no-wrap long line | horizontal follow keeps cursor visible |
| BD-04 | rapid resize storm | final geometry and cursor visibility correct |
| BD-05 | resize to 1 column and 1 row | no panic; deterministic clamping |
| BD-06 | terminal output flood + adjacent edit | editing remains responsive |
| BD-07 | terminal close during output | child reaped; no zombie process |
| BD-08 | explorer with 10k entries | navigation remains responsive |
| BD-09 | session load with missing file | warning shown; remaining layout restored |
| BD-10 | wrap boundary with width-2 remainder | padding cell inserted; no half-cell cursor |

## PTY-Specific Tests

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| PE-01 | PTY terminal spawn and output | output appears in terminal window grid |
| PE-02 | PTY resize integration | resize sends signal and updates terminal grid |
| PE-03 | PTY alternate screen app | enter and exit alternate screen safely |
| PE-04 | PTY IME leader isolation | composition keys do not trigger leader actions |
| PE-05 | PTY mixed window navigation | focus and input routes remain correct |
| PE-06 | PTY append mode churn | repeated append cycles keep cursor clamped |

## Infrastructure Requirements

| Requirement | Detail |
|---|---|
| PTY harness | spawn editor in PTY, send bytes, read frames with bounded timeout |
| Frame assertions | cell-grid helper with grapheme width checks |
| Resize helper | deterministic resize trigger and final geometry assertion |
| Diagnostic output | failures report mode, window type, cursor, and visible frame excerpt |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Explorer spec: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
