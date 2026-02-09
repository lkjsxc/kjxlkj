# E2E Boundary Blueprint (2026-02-09)

Back: [/docs/log/reconstruction/testing-ideas/README.md](/docs/log/reconstruction/testing-ideas/README.md)

This blueprint adds high-signal tests that close current weak spots.

## Priority 1: Runtime Wiring Truth Tests

| ID | Test Idea | Why High Value |
|---|---|---|
| X-PTY-01 | Real `:terminal` spawn, command execution, close, child reap assertion | Proves terminal is not a stub |
| X-WIN-01 | Non-trivial split graph directional focus checks | Detects cyclic-order bug in window navigation |
| X-EXP-01 | `<leader>e` open file, split-open from explorer, close explorer | Proves explorer is wired beyond model layer |
| X-IO-01 | `:w` persists bytes and `:e` reloads modified external file | Prevents false-positive write conformance |
| X-SES-01 | `:SessionSave`/`:SessionLoad` restores layout + focused window | Validates session commands are actually wired |

## Priority 2: Japanese and Width Safety

| ID | Test Idea | Why High Value |
|---|---|---|
| X-IME-01 | Active composition with `Space` candidate cycle does not trigger leader actions | Detects IME/leader leakage |
| X-IME-02 | Commit/cancel path around `Esc` in Insert mode | Prevents accidental mode exit corruption |
| X-CJK-01 | `A` after mixed ASCII/CJK line commits correctly and cursor clamps | Detects shifted-key + width interaction bugs |
| X-WRAP-01 | 1-column remainder before width-2 grapheme inserts padding cell | Protects wrap boundary invariant |

## Priority 3: Concurrency and Stress

| ID | Test Idea | Why High Value |
|---|---|---|
| X-CONC-01 | Flood terminal output while editing in adjacent buffer window | Proves scheduler isolation and responsiveness |
| X-RESIZE-01 | 200 rapid resize events with wrapped CJK lines and terminal pane | Catches state corruption under geometry churn |
| X-MODE-01 | 2,000 mode churn cycles with macro recording enabled | Catches hidden mode-state leaks |

## Execution Guidance

- Start from smallest reproducible case, then scale dimensions.
- Keep hard deadlines and emit actionable timeout context.
- Prefer persisted-state checks over fragile screen text snapshots.
- Add one regression per bug fix in the same change.

## Spec Links

- [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
