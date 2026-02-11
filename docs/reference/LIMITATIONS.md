# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current baseline state.

## Baseline Statement (2026-02-11)

The repository is intentionally prepared as docs-only baseline. Implementation
artifacts were removed so the next implementation can be regenerated from docs.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BASELINE-IMPL-04` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace and source tree are intentionally absent in docs-only baseline | `M2 missing feature` | high | regenerate grouped workspace from TODO wave chain |
| `LIM-BLOCK-KEY-04` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` runtime behavior is unverified because runtime is absent | `M2 missing feature` + `M4 verification gap` | high | implement key path and close with `KEYMODE-01`, `WR-01R` screen assertions |
| `LIM-BLOCK-WIN-04` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split lifecycle behavior is unverified because runtime is absent | `M2 missing feature` + `M4 verification gap` | high | implement split tree and close with `WIN-01R`..`WIN-05R` |
| `LIM-BLOCK-EXP-04` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch/actions are unverified because runtime is absent | `M2 missing feature` + `M4 verification gap` | high | implement explorer routes/state and close with `EXP-01R`..`EXP-06R` |
| `LIM-BLOCK-E2E-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | screen-state E2E closure is unverified because harness/runtime are absent | `M2 missing feature` + `M4 verification gap` | high | rebuild harness and enforce per-key state + frame assertions |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TOPO-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | topology constraints cannot be validated until implementation is rebuilt | medium | run topology audits after regeneration |

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
