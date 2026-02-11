# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for the current reconstructed foundation baseline.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent or unreachable |
| `M3 undocumented behavior` | behavior exists but is not specified canonically |
| `M4 verification gap` | behavior exists but deterministic regression evidence is insufficient |
| `M5 stale docs` | documentation claims are contradicted by stronger evidence |

## Matrix

| Req ID | Canonical Document | Requirement | Expected Code Path(s) | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace manifests and grouped source tree exist | `Cargo.toml`, `src/crates/...` | topology and build gate | aligned | none | monitor | `cargo metadata --no-deps`; `cargo check --workspace`; `cargo test -p kjxlkj-test-harness` |
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches as `A` append semantics | `src/crates/platform/kjxlkj-input/src/decode.rs`, `src/crates/core/kjxlkj-core-mode/src/normal.rs` | `WR-01R`, `KEY-TRACE-01` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | shared tree preserves deterministic focus and geometry invariants | `src/crates/core/kjxlkj-core-state/src/tree.rs`, `src/crates/core/kjxlkj-core-state/src/split.rs` | `WIN-01R`, `WIN-04R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | full `Ctrl-w` family works across mixed windows | `src/crates/core/kjxlkj-core-mode/src/normal.rs` | `WINNAV-01R`..`WINNAV-06R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer command/key paths are reachable | `src/crates/services/kjxlkj-service-explorer/src/state.rs` | `EXP-01R`, `EXP-02R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal lifecycle and mixed-window behavior are stable | `src/crates/services/kjxlkj-service-terminal/src/service.rs` | `TERM-01R`..`TERM-07R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | long lines and wide graphemes wrap without overflow | `src/crates/platform/kjxlkj-render/src/grid.rs` | `WRAP-11R`..`WRAP-16R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-CUR-02` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | cursor remains visible and grapheme-safe in churn | `src/crates/core/kjxlkj-core-state/src/editor.rs`, `src/crates/platform/kjxlkj-render/src/grid.rs` | `CUR-07R`..`CUR-11R` | spec-only | `M2 missing feature` | implement + test-add | open |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure requires PTY E2E evidence | `src/crates/app/kjxlkj-test-harness/` | all `*R` | test-gap | `M4 verification gap` | implement + test-add | open |
| `R-DOC-01` | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | TODO coverage links every markdown file directly | `docs/todo/doc-coverage/` | link audit | aligned | none | monitor | 440/440 links present |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 7 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Update Rules

- close a row only with reproducible evidence
- close high-severity rows before release-oriented work
- update this file together with `CONFORMANCE`, `LIMITATIONS`, and active TODO docs

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- TODO mismatch matrix: [/docs/todo/current/mismatch-matrix.md](/docs/todo/current/mismatch-matrix.md)
