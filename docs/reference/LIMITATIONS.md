# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger tracks open mismatches between target spec and current runtime confidence.

## Baseline Statement (2026-02-11)

The workspace and automated suites exist, but release confidence is blocked by
contradictory evidence: user runtime reports indicate split view, explorer, and
`Shift+a` do not work as expected in practical sessions.

## Open Critical Blockers

| ID | Requirement Link | Observed Runtime Gap | Class | Severity | Mandatory Next Action |
|---|---|---|---|---|---|
| `LIM-BLOCK-KEY-04` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` behavior is unreliable in user-like runtime sessions | `M1 correctness` + `M4 verification gap` | high | implement screen-state PTY checks for `KEYMODE-01`, `WR-01R`, `WR-02R`; close only with deterministic screen snapshots |
| `LIM-BLOCK-WIN-04` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split view behavior is reported non-working under real usage | `M1 correctness` + `M4 verification gap` | high | add user-like split lifecycle E2E (`WIN-01R`..`WIN-04R`) with frame assertions per key input |
| `LIM-BLOCK-EXP-04` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch/actions are reported non-working under real usage | `M1 correctness` + `M4 verification gap` | high | add screen-validated explorer route/open/action E2E (`EXP-01R`..`EXP-06R`) including visible panel checks |
| `LIM-BLOCK-E2E-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | existing `*R` tests are too trace-centric and can miss UI-visible breakage | `M4 verification gap` | high | make screen-state oracle mandatory and fail-fast on render/layout mismatches |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Severity | Next Action |
|---|---|---|---|---|
| `LIM-GAP-TOPO-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | topology limits are documented but not yet enforced as hard release gate for every wave | medium | require topology checkboxes in TODO phase completion before final sign-off |

## Deferred Items

Deferred items must not be correctness-critical or user-blocking.

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| none | n/a | no non-critical defer items are active | after blocker closure |

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
