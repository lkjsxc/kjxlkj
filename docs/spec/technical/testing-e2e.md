# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

High-leverage E2E matrix for blocker-first reconstruction.

## Harness Levels

| Harness | Description | Required For |
|---|---|---|
| Headless state harness | drives core actions directly, no PTY | baseline integration |
| PTY process harness | runs full binary in PTY, sends key bytes, captures frames | blocker closure and release gate |

All `*R` tests in this file require the PTY process harness.

## Selected Existing Tests (Keep)

| ID | Why It Stays |
|---|---|
| `WR-01` | detects key normalization regression quickly |
| `WR-03` / `WR-04` / `WR-05` | verify command/key wiring for terminal and explorer |
| `WR-06` | mixed-window focus path sanity |
| `WR-07` / `BD-10` | catches wrap overflow and width-2 boundary issues |
| `JP-03` | protects IME leader isolation |
| `PE-02` / `PE-05` | terminal resize + mixed-window behavior baseline |

## New Mandatory Live Regression Tests

Risk score format: `impact/regression/detection/determinism/cost` (0-3 each).

| ID | Risk Score | Scenario | Deterministic Assertions |
|---|---|---|---|
| `WR-01R` | `3/3/3/2/2` | send raw `Shift+a` in Normal mode at non-empty line | decode trace shows `A`; buffer result matches append-at-EOL semantics |
| `WIN-01R` | `3/3/3/2/2` | create nested splits (`s`,`v`,`s`), close and reopen leaves | no orphan focus, valid tree, deterministic focused `WindowId` |
| `WIN-02R` | `3/3/3/2/2` | directional `Ctrl-w h/j/k/l` over asymmetric mixed tree | focus trace matches geometric oracle |
| `WINNAV-01R` | `3/3/2/3/2` | `Ctrl-w w/W/p/t/b` sequence on mixed windows | sequence of focused windows exactly matches golden list |
| `EXP-01R` | `3/3/3/2/2` | run `:Explorer` then `<leader>e` toggle | explorer visibility toggles deterministically |
| `EXP-03R` | `3/3/3/2/2` | open selected file with `Enter`, `v`, `s` | target window type and file path match expected |
| `EXP-06R` | `2/3/3/2/2` | external file create/rename/delete while explorer open | refresh updates tree without focus corruption |
| `TERM-01R` | `3/3/3/2/2` | `:terminal` launch, run command, capture output | PTY output appears in terminal leaf within timeout |
| `TERM-04R` | `3/2/3/2/2` | resize terminal split repeatedly | PTY resize events observed; cursor remains visible |
| `TERM-06R` | `3/3/3/2/2` | flood terminal output while editing adjacent buffer | editor input latency stays bounded; no deadlock |
| `CUR-10R` | `3/3/3/2/1` | place cursor at wrap boundary with width-2 grapheme | no continuation-cell cursor; visible cursor highlight |
| `WRAP-14R` | `3/3/3/2/2` | resize storm with long ASCII+CJK lines | no off-screen cell writes; deterministic wrapped rows |
| `WRAP-16R` | `3/2/2/2/2` | long lines in editor/explorer/terminal simultaneously | all windows respect bounds and preserve focus state |
| `JP-06R` | `3/3/3/2/2` | IME composition with `<leader>e` sequence | no explorer toggle while composition active |
| `JP-07R` | `3/3/3/2/2` | IME composition with `<leader>t` sequence | no terminal spawn while composition active |
| `JP-09R` | `2/2/2/2/2` | IME composition during terminal resize + split navigation | composition state preserved; no accidental mode escape |

## Creative Boundary and Race Suite

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| `BD-RACE-01` | simultaneous terminal flood + explorer refresh + split resize | no panic, bounded latency, consistent focus |
| `BD-RACE-02` | alternating wrap on/off during rapid cursor motion in CJK text | no half-cell cursor state and no overflow |
| `BD-RACE-03` | repeated open/close explorer and terminal in 100-cycle loop | stable memory growth and no stale window IDs |
| `BD-RACE-04` | command-line `:Explorer` and `:terminal` issued back-to-back under IME input | command routing remains deterministic |

## Test Diagnostics (mandatory)

Every failing E2E test MUST print:

- active mode
- focused window ID and window type
- layout tree summary
- cursor/caret position
- top visible frame excerpt
- last 20 input events and resolved actions

## Release Gate Addendum

Release gate is green only when:

1. existing retained tests remain green
2. all `*R` tests in this file pass
3. no high-severity row remains open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Explorer spec: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal spec: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
