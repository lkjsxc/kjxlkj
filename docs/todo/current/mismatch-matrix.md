# Mismatch Matrix

Back: [/docs/todo/current/phases/phase-0-foundation.md](/docs/todo/current/phases/phase-0-foundation.md)

Spec-code-test drift tracking for active blocker wave.

## Matrix Schema

| Column | Meaning |
|---|---|
| Requirement ID | stable requirement reference |
| Canonical document | normative source |
| Requirement statement | expected behavior |
| Code path | expected implementation area |
| Test path | deterministic verification target |
| Observed status | `aligned`, `spec-only`, `test-gap`, `contradiction` |
| Mismatch class | `M1`..`M5` |
| Action | `implement`, `spec-update`, `test-add`, `refactor`, `defer-with-log` |
| Verification evidence | proof required to close row |

## Active Rows

| Requirement ID | Canonical document | Requirement statement | Code path | Test path | Observed status | Mismatch class | Action | Verification evidence |
|---|---|---|---|---|---|---|---|---|
| `R-KEY-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `Shift+a` dispatches as `A` | input decode + normal mode dispatch | `WR-01R` | contradiction | `M1 correctness` | implement | live PTY trace + passing regression |
| `R-WIN-01` | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) | split/focus geometry is deterministic | window tree + focus resolver | `WIN-01R`,`WIN-02R` | contradiction | `M1 correctness` | implement | golden focus traces |
| `R-WIN-03` | [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md) | mixed-window `Ctrl-w` family works | wincmd dispatch + focus history | `WINNAV-01R`,`WINNAV-02R` | contradiction | `M1 correctness` | implement | deterministic focus sequence proof |
| `R-EXP-01` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer launch route is reachable | ex dispatch + leader mapping + explorer open | `EXP-01R`,`EXP-02R` | contradiction | `M1 correctness` | implement | command/key route logs + visible explorer |
| `R-EXP-02` | [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md) | explorer actions perform expected file/window operations | explorer ops + FS service | `EXP-03R`..`EXP-06R` | contradiction | `M1 correctness` | implement | deterministic tree + filesystem assertions |
| `R-TERM-01` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal launch and lifecycle are stable | terminal service + window ops | `TERM-01R`,`TERM-05R` | contradiction | `M1 correctness` | implement | PTY lifecycle assertions |
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | terminal output does not stall editing | terminal IO + core scheduling | `TERM-06R`,`BD-RACE-01` | test-gap | `M4 verification gap` | test-add | bounded-latency assertions |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | no off-screen overflow for long lines | wrap engine + render integration | `WRAP-11R`,`WRAP-14R`,`WRAP-16R` | contradiction | `M1 correctness` | implement | frame bounds oracle |
| `R-CUR-02` | [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md) | cursor remains visible and atomic on wide chars | cursor overlay + grid painter | `CUR-07R`..`CUR-11R` | contradiction | `M1 correctness` | implement | cursor cell assertions on captured frames |
| `R-I18N-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | IME composition isolates leader mappings | IME gate + key mapping resolver | `JP-06R`,`JP-07R` | test-gap | `M4 verification gap` | test-add | PTY composition race proofs |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | critical blockers require live E2E | test harness topology | all `*R` rows | test-gap | `M4 verification gap` | test-add | blocker closure requires passing live E2E rows |
| `R-ARCH-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | workspace follows grouped crate roots | workspace layout and manifests still use flat crate paths | spec-only | `M2 missing feature` | implement | grouped path migration with build + test proof |

## Priority Rule

Resolve rows in this order:

1. `M1 correctness`
2. `M4 verification gap`
3. `M5 stale docs`

## Related

- [x] Requirement matrix: [/docs/todo/current/requirement-matrix.md](/docs/todo/current/requirement-matrix.md)
- [x] Reference drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
