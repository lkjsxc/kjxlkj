# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for current state.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior is not implemented or not reachable |
| `M3 undocumented behavior` | behavior exists without canonical spec |
| `M4 verification gap` | behavior exists but deterministic coverage is insufficient |
| `M5 stale docs` | docs claim status contradicted by stronger evidence |

## Active Drift Rows

| Req ID | Canonical Spec | Expected Behavior | Current Gap | Class | Next Action | Status |
|---|---|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/README.md](/docs/spec/README.md) | reconstructed implementation exists and is runnable | docs-only standby baseline; implementation intentionally absent | `M2 missing feature` | execute TODO reconstruction phases | open |
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches as `A` through real input path | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `WR-01R` | open |
| `R-WIN-01` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split tree stable across create/focus/resize/close | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `WIN-01R`..`WIN-05R` | open |
| `R-WIN-02` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w` family works across all window types | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `WINNAV-01R`..`WINNAV-06R` | open |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and `<leader>e` are reliable | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `EXP-01R`..`EXP-06R` | open |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal windows integrate with shared layout/focus lifecycle | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `TERM-01R`..`TERM-07R` | open |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrapped lines never overflow screen bounds | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `WRAP-11R`..`WRAP-16R` | open |
| `R-CUR-01` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor display remains visible and grapheme-safe | implementation absent; prior failure remains unresolved | `M2 missing feature` | implement + verify with `CUR-07R`..`CUR-11R` | open |
| `R-I18N-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME composition does not leak into mappings | implementation absent; runtime evidence unavailable | `M2 missing feature` | implement + verify with `JP-06R`..`JP-09R` | open |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | high-risk features proven by real E2E | blocker-closure evidence unavailable in docs-only baseline | `M4 verification gap` | run full `*R` suite after reconstruction | open |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | grouped crate roots (`app/core/platform/services`) are used | workspace layout not yet regenerated | `M2 missing feature` | regenerate workspace with grouped paths | open |
| `R-LOG-01` | [/docs/log/README.md](/docs/log/README.md) | historical notes removed after promotion | stale wave logs were deleted | `M5 stale docs` | keep retention discipline | closed |

## Summary

| Class | Open Count |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 10 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Update Rules

- close a row only with reproducible evidence
- close high-severity rows before release claims
- update this file together with `CONFORMANCE`, `LIMITATIONS`, and TODO state

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- TODO verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
