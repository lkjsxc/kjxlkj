# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for the current reconstruction state.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent or unreachable |
| `M3 undocumented behavior` | behavior exists but is not specified canonically |
| `M4 verification gap` | behavior exists but deterministic evidence is insufficient |
| `M5 stale docs` | documentation claims are contradicted by stronger evidence |

## Matrix

| Req ID | Canonical Document | Requirement | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` must dispatch exactly as `A` | `WR-01R`, `KEYMODE-01` | contradiction | `M1`, `M4`, `M5` | implement + test-add | user runtime report (2026-02-11) conflicts with passing trace-centric `key_mode_e2e` |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/close/rebalance is deterministic and visible | `WIN-01R`..`WIN-04R` | contradiction | `M1`, `M4`, `M5` | implement + test-add | user runtime report (2026-02-11) conflicts with passing trace-centric `window_nav_e2e` |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and leader routes are user-visible and reliable | `EXP-01R`..`EXP-06R` | contradiction | `M1`, `M4`, `M5` | implement + test-add | user runtime report (2026-02-11) conflicts with passing trace-centric `explorer_terminal_paths_e2e` |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure must be based on user-like E2E screen assertions | all `*R` blocker rows | test-gap | `M4`, `M5` | spec-update + test-add | old closure relied on action traces without strict frame/state assertions |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | source dirs stay near 12 children and files stay <=200 lines | topology checks | partial | `M4` | test-add | policy exists; release-level enforcement sequence needs checklist hardening |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 3 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 5 |
| `M5 stale docs` | 4 |

## Update Rules

- if a user-visible contradiction appears, downgrade status immediately
- close a row only with reproducible evidence from real runtime paths
- synchronize updates with `CONFORMANCE`, `LIMITATIONS`, and `/docs/todo/`

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
