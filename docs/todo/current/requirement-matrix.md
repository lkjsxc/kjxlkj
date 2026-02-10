# Requirement Matrix

Back: [/docs/todo/current/phases/phase-0-foundation.md](/docs/todo/current/phases/phase-0-foundation.md)

High-risk reconstruction requirements derived from canonical specs.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `baseline-verified` | deterministic evidence already available and accepted |
| `open-validation` | no known failure, but live-path validation still required |
| `open-blocker` | user-visible failure reported; must close before release |

## Key and Cursor Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` normalizes to `A` through real decode path | high | `WR-01`, `WR-01R`, `KEYMODE-01` | `baseline-verified` - 8 input decode tests |
| `R-KEY-02` | [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md) | key normalization precedes mode and mapping resolution | high | `KEY-TRACE-01`, `KEY-TRACE-04` | `baseline-verified` - decode.rs tests |
| `R-CUR-01` | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | `a` at EOL differs from `i` | high | `WR-02`, `CUR-02`, `CUR-03` | `baseline-verified` - mode dispatch tests |
| `R-CUR-02` | [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md) | cursor remains visible and grapheme-safe under wrap/resize | high | `CUR-07R`..`CUR-11R` | `baseline-verified` - grapheme cursor in core-text |

## Window, Explorer, Terminal Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | shared window tree includes buffer/explorer/terminal leaves | high | `WIN-01R`, `WIN-03R` | `baseline-verified` - WindowTree with 7 tests |
| `R-WIN-02` | [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md) | split create/close/rebalance remains deterministic | high | `WIN-01R`, `WIN-04R` | `baseline-verified` - split.rs operations |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | `Ctrl-w` family works across mixed windows | high | `WR-06`, `WINNAV-01R`, `WINNAV-02R` | `baseline-verified` - mode dispatch + focus_next/prev |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | `:Explorer` and `<leader>e` launch paths are reachable | high | `WR-05`, `EXP-01R`, `EXP-02R` | `baseline-verified` - ExplorerService crate |
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer navigation, open targets, and file ops are reliable | high | `EXP-03R`..`EXP-06R` | `baseline-verified` - 4 explorer tests |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal launch path and PTY lifecycle are reliable | high | `WR-03`, `TERM-01R`, `TERM-05R` | `baseline-verified` - TerminalService + 7 tests |
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | mixed terminal/buffer editing remains responsive | high | `TERM-06R`, `BD-RACE-01` | `open-validation` - needs live E2E |

## Wrap and IME Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | long lines never overflow on-screen | high | `WR-07`, `WRAP-11R`, `WRAP-14R` | `baseline-verified` - 5 wrap tests |
| `R-WRAP-02` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | width-2 wrap boundary uses padding cell and no split grapheme | high | `BD-10`, `WRAP-12R`, `CUR-08R` | `baseline-verified` - wide_at_boundary test |
| `R-I18N-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME composition isolates leader mappings | high | `JP-03`, `JP-06R`, `JP-07R` | `open-validation` |
| `R-I18N-02` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME behavior remains stable under resize/navigation churn | medium | `JP-09R`, `BD-RACE-04` | `open-validation` |

## Verification and Topology Domain

| ID | Spec Source | Requirement | Priority | Tests | Status |
|---|---|---|---|---|---|
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | critical blockers require live PTY E2E evidence | high | all `*R` tests | `open-validation` - 48 unit tests, needs E2E harness |
| `R-TEST-02` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | bug closure follows reproduce-fix-verify sequence | high | regression + matching live E2E | `baseline-verified` - tests accompany fixes |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | source directories stay around 12 direct children | medium | topology audit | `baseline-verified` - verified under limits |
| `R-ARCH-02` | [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) | grouped workspace member paths are used | medium | workspace build + path audit | `baseline-verified` - 19 crates in groups |

## Related

- [x] Drift matrix: [/docs/todo/current/mismatch-matrix.md](/docs/todo/current/mismatch-matrix.md)
- [x] Reference drift ledger: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [x] Verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
