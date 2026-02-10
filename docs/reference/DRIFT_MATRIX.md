# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for current state.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior is not implemented or not reachable |
| `M3 undocumented behavior` | behavior exists without canonical spec |
| `M4 verification gap` | behavior exists but deterministic regression coverage is insufficient |
| `M5 stale docs` | docs claim status contradicted by stronger evidence |

## Active Drift Rows

| Req ID | Canonical Spec | Expected Behavior | Current Gap | Class | Next Action | Status |
|---|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches as `A` through real input path | user reports action not working in implementation | `M1 correctness` | add `WR-01R` live PTY key normalization test and fix decode route | open |
| `R-WIN-01` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split tree remains stable across create/focus/resize/close | split behavior reported broken | `M1 correctness` | implement geometry-driven window focus/resize checks + `WIN-01R`..`WIN-05R` | open |
| `R-WIN-02` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w` family works across all window types | mixed-window movement reported unreliable | `M1 correctness` | add directional oracle E2E (`WINNAV-01R`..`WINNAV-06R`) | open |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and `<leader>e` open explorer reliably | reported non-working | `M1 correctness` | add launch-wire E2E and command/key route traces (`EXP-01R`,`EXP-02R`) | open |
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer navigation and open-in-split actions work | user reports explorer actions unstable | `M1 correctness` | add `EXP-03R`..`EXP-06R` for navigation, split-open, and file ops | open |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows integrate with shared layout/focus lifecycle | terminal window reliability reported weak | `M1 correctness` | add mixed terminal/buffer/explorer stress E2E (`TERM-01R`..`TERM-07R`) | open |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrapped lines never overflow screen bounds | wrap bugs reported in real usage | `M1 correctness` | add interactive long-line wrap and resize tests (`WRAP-11R`..`WRAP-16R`) | open |
| `R-CUR-01` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor display remains visible and grapheme-safe | cursor display bugs reported | `M1 correctness` | add cursor overlay + wide-char wrap E2E (`CUR-07R`..`CUR-11R`) | open |
| `R-I18N-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME composition does not leak into mappings | high-risk due key-routing overlap | `M4 verification gap` | extend PTY IME scenarios with leader/keymap conflicts (`JP-06R`..`JP-09R`) | open |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | high-risk features must be proven by real E2E | current suite passes but misses field failures | `M4 verification gap` | enforce live-path mandatory E2E gate in TODO phase closure | open |
| `R-DOC-01` | [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | conformance claims reflect strongest evidence | prior docs over-claimed full closure | `M5 stale docs` | keep blocker-first status until closure evidence exists | closed |
| `R-LOG-01` | [/docs/log/README.md](/docs/log/README.md) | historical notes removed after promotion | stale wave logs remained | `M5 stale docs` | delete past records after canonical promotion | in_progress |

## Summary

| Class | Open Count |
|---|---:|
| `M1 correctness` | 7 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 2 |
| `M5 stale docs` | 1 |

## Update Rules

- close a row only with reproducible evidence
- keep high-severity user-visible rows at top priority
- update this file together with `CONFORMANCE`, `LIMITATIONS`, and TODO state

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- TODO verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
