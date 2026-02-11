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
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | test-gap | `M2 missing feature` | implement + test-add | baselines `EXP-03R` and `EXP-04R` are reachable; complete stable explorer interactions `EXP-05R` and `EXP-06R` with full target semantics |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | test-gap | `M2 missing feature` | implement + test-add | baselines `TERM-01R`..`TERM-05R` are reachable; complete real PTY lifecycle semantics (child reap and leak checks) |
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | test-gap | `M2 missing feature` | implement + test-add | baselines `TERM-06R` and `TERM-07R` are reachable; complete `BD-RACE-01` and bounded-latency proof |
| `R-CUR-02` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | test-gap | `M2 missing feature` | implement + test-add | baselines `CUR-07R`, `CUR-09R`, and `CUR-11R` are reachable; complete `CUR-08R` and `CUR-10R` wide-grapheme display semantics |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | test-gap | `M2 missing feature` | implement + test-add | baselines `WRAP-11R`..`WRAP-13R` are reachable; complete `WRAP-14R`..`WRAP-16R` resize and cross-window bounds semantics |

## Closed Rows

| Requirement ID | Canonical document | Observed status | Mismatch class | Closure Evidence |
|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | aligned | none | `cargo metadata --no-deps`; `cargo check --workspace`; `cargo test -p kjxlkj-test-harness` |
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | aligned | none | `cargo test -p kjxlkj-test-harness --test key_mode_e2e` |
| `R-CUR-01` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | aligned | none | `cargo test -p kjxlkj-core-state`; `cargo test -p kjxlkj-test-harness --test cursor_wrap_e2e` |
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | aligned | none | `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e --test window_nav_session_terminal_e2e` (`WIN-01R`, `WIN-04R`, `WIN-05R`) |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | aligned | none | `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e` (`WIN-01R`..`WIN-04R`) |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | aligned | none | `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e --test window_nav_session_terminal_e2e` (`WINNAV-01R`..`WINNAV-06R`) |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | aligned | none | `cargo test -p kjxlkj-test-harness --test explorer_terminal_paths_e2e` (`EXP-01R`, `EXP-02R`) |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | aligned | none | PTY harness operations implemented in `src/crates/app/kjxlkj-test-harness/src/pty.rs` and `WR-01R` evidence passing |
| `R-DOC-01` | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | aligned | none | 440/440 markdown files directly linked in coverage parts |

## Priority Rule

1. resolve `M2 missing feature` rows for high-severity blockers
2. resolve `M4 verification gap` rows required for blocker closure
3. keep docs and ledgers synchronized in same change

## Related

- [x] Requirement matrix: [/docs/todo/current/requirement-matrix.md](/docs/todo/current/requirement-matrix.md)
- [x] Reference drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
