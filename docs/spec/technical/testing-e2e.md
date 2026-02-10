# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

High-leverage E2E matrix for blocker-first reconstruction.

## Harness Levels

| Harness | Description | Required For |
|---|---|---|
| Headless state harness | drives core actions directly, no PTY | baseline integration |
| PTY process harness | runs full binary in PTY, sends key bytes, captures frames | blocker closure and release gate |

All `*R` tests in this file require the PTY process harness.

## Retained Existing Tests (Keep)

| ID | Why It Stays |
|---|---|
| `WR-01` | catches key normalization regressions quickly |
| `WR-03` / `WR-04` / `WR-05` | verifies command/key wiring for terminal and explorer |
| `WR-06` | mixed-window focus baseline |
| `WR-07` / `BD-10` | catches wrap overflow and width-2 boundary issues |
| `JP-03` | protects IME leader isolation |
| `PE-02` / `PE-05` | terminal resize + mixed-window behavior baseline |

## Mandatory Live Regression Suite (`*R`)

| ID | Scenario | Deterministic Assertions |
|---|---|---|
| `WR-01R` | raw `Shift+a` in Normal mode | decoder trace shows `A`; result matches append-at-EOL semantics |
| `WIN-01R` | nested split create/close lifecycle | no orphan focus; valid tree; deterministic focused `WindowId` |
| `WIN-02R` | directional `Ctrl-w h/j/k/l` on asymmetric tree | focus trace matches geometric oracle |
| `WIN-03R` | mixed buffer/explorer/terminal directional transitions | direction moves to correct window type/ID |
| `WIN-04R` | repeated split resize/equalize/close | geometry invariants preserved |
| `WIN-05R` | session roundtrip with complex layout | focused window and split structure restored |
| `WINNAV-01R` | `Ctrl-w w/W/p/t/b` sequence on mixed windows | focus order matches golden sequence |
| `WINNAV-02R` | `Ctrl-w h/j/k/l` then cycle commands | directional and cyclic models stay consistent |
| `WINNAV-03R` | `Ctrl-w p` after close/reopen churn | previous-focus pointer remains valid |
| `WINNAV-04R` | top-left and bottom-right focus under nested splits | `t`/`b` select deterministic leaves |
| `WINNAV-05R` | `Ctrl-w` navigation with terminal insert transitions | navigation works before and after mode escape chord |
| `WINNAV-06R` | long navigation sequence replayed twice | second run focus trace exactly matches first |
| `EXP-01R` | `:Explorer` command launch | explorer leaf appears and is focused deterministically |
| `EXP-02R` | `<leader>e` toggle and `<leader>E` reveal | toggle/reveal path is wired and visible |
| `EXP-03R` | open selected entry via `Enter`, `v`, `s` | target window type and file path match expectation |
| `EXP-04R` | `Ctrl-w` navigation across explorer/buffer/terminal | focus transitions remain valid |
| `EXP-05R` | long explorer labels + badges | on-screen wrap safety; stable selection identity |
| `EXP-06R` | external create/rename/delete while explorer visible | refresh updates tree without focus corruption |
| `TERM-01R` | `:terminal` launch and output capture | PTY output appears in terminal leaf within timeout |
| `TERM-02R` | `<leader>t`, `<leader>th`, `<leader>tv` launch paths | all routes hit same spawn path semantics |
| `TERM-03R` | `Ctrl-w` navigation across focused terminal leaf | window commands work identically in mixed layout |
| `TERM-04R` | resize terminal split repeatedly | PTY resize signal observed; cursor remains visible |
| `TERM-05R` | close terminal during active output | child reaped; no zombie; editor stable |
| `TERM-06R` | terminal output flood while editing adjacent buffer | bounded input latency; no deadlock |
| `TERM-07R` | CJK output near wrap boundary in terminal | no half-cell state; correct continuation behavior |
| `CUR-07R` | mode churn with frequent redraw | primary cursor remains visible and stable |
| `CUR-08R` | width-2 grapheme cursor highlight | both grapheme cells highlighted |
| `CUR-09R` | cursor placement near continuation cell | cursor never targets continuation cell |
| `CUR-10R` | cursor at wrap boundary with width-2 grapheme | no split cursor artifact |
| `CUR-11R` | rapid focus changes across windows | exactly one primary cursor visible |
| `WRAP-11R` | 10k ASCII line in wrap mode | no overflow; deterministic continuation rows |
| `WRAP-12R` | 10k CJK line in wrap mode | no split wide grapheme |
| `WRAP-13R` | wrap -> nowrap -> wrap toggling | stable and deterministic rewrap points |
| `WRAP-14R` | resize storm with long mixed-script lines | no off-screen writes |
| `WRAP-15R` | repeated 1x1 and narrow geometries | no panic; deterministic clamping |
| `WRAP-16R` | long lines in editor/explorer/terminal simultaneously | all windows respect bounds |
| `JP-06R` | IME composition with `<leader>e` sequence | no explorer action during composition |
| `JP-07R` | IME composition with `<leader>t` sequence | no terminal action during composition |
| `JP-08R` | composition cancel then `Esc` | cancels composition first; exits Insert once |
| `JP-09R` | IME composition during resize + window navigation | composition state preserved; no accidental mode exit |

## Creative Boundary and Race Suite

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| `BD-RACE-01` | terminal flood + explorer refresh + split resize | no panic; bounded latency; consistent focus |
| `BD-RACE-02` | wrap on/off churn during rapid CJK motion | no half-cell cursor state and no overflow |
| `BD-RACE-03` | 100-cycle explorer/terminal open-close loop | stable memory profile and no stale window IDs |
| `BD-RACE-04` | `:Explorer` and `:terminal` interleaved under IME activity | routing remains deterministic |

## Test Diagnostics (mandatory)

Every failing live E2E test MUST report:

- active mode
- focused window ID and type
- layout tree summary
- cursor/caret position
- top frame excerpt
- last 20 input events and resolved actions

## Release Gate Addendum

Release gate is green only when:

1. retained baseline tests remain green
2. all `*R` tests in this file pass
3. no high-severity row remains open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Explorer spec: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
