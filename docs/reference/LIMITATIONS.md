# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current baseline state.

## Baseline Statement (2026-02-11)

Workspace reconstructed with 20 crates matching source-layout.md spec.
57 unit/integration tests pass. PTY E2E harness not yet reconstructed.
Explorer and terminal service crates are stubs.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BASELINE-IMPL-04` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace and source tree reconstructed; 20 crates, compiles clean | `M2 missing feature` | closed | n/a |
| `LIM-BLOCK-KEY-04` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` normalization implemented and T1-tested; T2 PTY verification pending | `M4 verification gap` | medium | close with `KEYMODE-01`, `WR-01R` T2 screen assertions |
| `LIM-BLOCK-WIN-04` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split lifecycle implemented and T1-tested; T2 PTY verification pending | `M4 verification gap` | medium | close with `WIN-01R`..`WIN-05R` T2 screen assertions |
| `LIM-BLOCK-EXP-04` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer service crate is stub; state model and routes not yet implemented | `M2 missing feature` + `M4 verification gap` | high | implement explorer routes/state and close with `EXP-01R`..`EXP-06R` |
| `LIM-BLOCK-E2E-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | T1 headless harness implemented; T2 PTY harness not yet rebuilt | `M2 missing feature` + `M4 verification gap` | high | rebuild PTY harness and enforce per-key state + frame assertions |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TOPO-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | topology matches spec; all files now ≤ 200 lines | closed | n/a |

## Deferred Items

Deferred items must not be correctness-critical or user-blocking.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| none | n/a | no deferred non-critical items are active | after baseline regeneration |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is reachable via real command/key paths
2. deterministic regression tests pass
3. matching live `*R` E2E tests pass using screen-state assertions
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are updated in the same change

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
