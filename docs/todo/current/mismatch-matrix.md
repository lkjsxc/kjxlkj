# Mismatch Matrix

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

Spec-code-test drift tracking for the active reconstructed foundation wave.

## Matrix Schema

| Column | Meaning |
|---|---|
| Requirement ID | stable requirement reference |
| Canonical document | normative source |
| Observed status | `aligned`, `spec-only`, `test-gap`, `contradiction` |
| Mismatch class | one of `M1`..`M5` |
| Action | `implement`, `test-add`, `spec-update`, `defer-with-log` |
| Required evidence | deterministic signal required to close row |

## Open Rows

| Requirement ID | Canonical document | Observed status | Mismatch class | Action | Required evidence |
|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | spec-only | `M2 missing feature` | implement + test-add | `WR-01R` PTY trace proving `Shift+a -> A` before dispatch |
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | spec-only | `M2 missing feature` | implement + test-add | `WIN-01R` and `WIN-04R` focus/geometry invariants |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | spec-only | `M2 missing feature` | implement + test-add | `WINNAV-01R`..`WINNAV-06R` deterministic focus traces |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | spec-only | `M2 missing feature` | implement + test-add | `EXP-01R` and `EXP-02R` command/key reachability |
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | spec-only | `M2 missing feature` | implement + test-add | `EXP-03R`..`EXP-06R` stable explorer interactions |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | spec-only | `M2 missing feature` | implement + test-add | `TERM-01R`..`TERM-05R` spawn/resize/close lifecycle |
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | spec-only | `M2 missing feature` | implement + test-add | `TERM-06R` and `BD-RACE-01` responsiveness proof |
| `R-CUR-02` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | spec-only | `M2 missing feature` | implement + test-add | `CUR-07R`..`CUR-11R` visibility and half-cell exclusion |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | spec-only | `M2 missing feature` | implement + test-add | `WRAP-11R`..`WRAP-16R` no overflow and no split-wide cells |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | test-gap | `M4 verification gap` | implement + test-add | blocker closure PRs include matching `*R` evidence |

## Closed Rows

| Requirement ID | Canonical document | Observed status | Mismatch class | Closure Evidence |
|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | aligned | none | `cargo metadata --no-deps`; `cargo check --workspace`; `cargo test -p kjxlkj-test-harness` |
| `R-DOC-01` | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | aligned | none | 440/440 markdown files directly linked in coverage parts |

## Priority Rule

1. resolve `M2 missing feature` rows for high-severity blockers
2. resolve `M4 verification gap` rows required for blocker closure
3. keep docs and ledgers synchronized in same change

## Related

- [x] Requirement matrix: [/docs/todo/current/requirement-matrix.md](/docs/todo/current/requirement-matrix.md)
- [x] Reference drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
