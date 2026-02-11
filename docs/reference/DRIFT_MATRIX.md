# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for the current docs-only baseline.

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
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | grouped workspace and crate tree exist | topology + build gate | verified | closed | implement | 20-crate workspace, `cargo check --workspace` passes (2026-02-11) |
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches exactly as `A` | `WR-01R`, `KEYMODE-01` | partial | `M4` | test-add | T1 headless test passes; T2 PTY harness pending |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/close/rebalance is deterministic and visible | `WIN-01R`..`WIN-05R` | partial | `M4` | test-add | T1 unit tests pass; T2 PTY harness pending |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and leader routes are user-visible and reliable | `EXP-01R`..`EXP-06R` | spec-only | `M2`, `M4` | implement + test-add | explorer crate is stub |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure must use user-like screen-state E2E assertions | all `*R` blocker rows | partial | `M2`, `M4` | implement + test-add | T1 harness exists; T2 PTY harness pending |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | source dirs stay near 12 children and files stay <=200 lines | topology checks | verified | closed | implement | all files ≤ 200 lines, multi-task runtime, topology matches spec (2026-02-11) |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 2 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 3 |
| `M5 stale docs` | 0 |

## Update Rules

- after docs-only transition, mark implementation rows as `spec-only` or `unverified`
- close rows only with reproducible runtime evidence after reconstruction
- synchronize updates with `CONFORMANCE`, `LIMITATIONS`, and `/docs/todo/`

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
