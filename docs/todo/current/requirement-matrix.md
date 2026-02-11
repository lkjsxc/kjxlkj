# Requirement Matrix

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

High-risk reconstruction requirements from canonical specs.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `open-blocker` | high-severity requirement is not yet implemented or validated |
| `open-validation` | no trusted live evidence yet; must be verified in this wave |
| `aligned` | requirement is closed with deterministic evidence |

## Key, Cursor, and Wrap Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` normalizes to `A` before mode dispatch | high | `WR-01R`, `KEY-TRACE-01` | `open-blocker` |
| `R-CUR-01` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | `i`, `a`, `A` semantics differ correctly at EOL | high | `CUR-01`..`CUR-06`, `CUR-07R` | `open-blocker` |
| `R-CUR-02` | [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md) | cursor remains visible and never targets continuation cells | high | `CUR-08R`..`CUR-11R` | `open-blocker` |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | wrap never overflows and wide graphemes remain atomic | high | `WRAP-11R`..`WRAP-16R` | `open-blocker` |

## Window, Explorer, Terminal Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | shared window tree supports buffer/explorer/terminal leaves | high | `WIN-01R`, `WIN-05R` | `open-blocker` |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/close/rebalance is deterministic | high | `WIN-01R`..`WIN-04R` | `open-blocker` |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | full `Ctrl-w` family works on mixed windows | high | `WINNAV-01R`..`WINNAV-06R` | `open-blocker` |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and leader routes are reachable | high | `EXP-01R`, `EXP-02R` | `open-blocker` |
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer navigation, split-open, and file actions are reliable | high | `EXP-03R`..`EXP-06R` | `open-blocker` |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal launch and PTY lifecycle are stable | high | `TERM-01R`..`TERM-05R` | `open-blocker` |
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal and editing remain responsive under flood and churn | high | `TERM-06R`, `TERM-07R`, `BD-RACE-01` | `open-blocker` |

## Verification and Topology Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-BASELINE-01` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace manifests and grouped source tree are reconstructed | high | topology + build gate | `aligned` |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closures require matching PTY E2E evidence | high | all blocker `*R` | `open-blocker` |
| `R-TEST-02` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | bug closure follows reproduce-fix-verify discipline | high | regression + matching `*R` | `open-validation` |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | source directories remain around 12 children | medium | topology audit | `aligned` |
| `R-ARCH-02` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | workspace members follow grouped crate paths | medium | build + manifest audit | `aligned` |
| `R-DOC-01` | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | TODO directly links every markdown file | high | doc coverage audit | `aligned` |

## Related

- [x] Mismatch matrix: [/docs/todo/current/mismatch-matrix.md](/docs/todo/current/mismatch-matrix.md)
- [x] Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [x] Verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
