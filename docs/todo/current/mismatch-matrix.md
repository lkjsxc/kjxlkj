# Mismatch Matrix

Back: [/docs/todo/current/phases/phase-0-foundation.md](/docs/todo/current/phases/phase-0-foundation.md)

Spec-code-test drift tracking for active blocker wave.

## Matrix Schema

| Column | Meaning |
|---|---|
| Requirement ID | stable requirement reference |
| Observed status | `aligned`, `spec-only`, `test-gap`, `contradiction` |
| Mismatch class | `M1`..`M5` |
| Resolution | description of fix |

## Resolved Rows

| Requirement ID | Previous Status | Mismatch class | Resolution |
|---|---|---|---|
| `R-KEY-01` | contradiction | `M1 correctness` | ✓ 8 input decode tests in decode.rs verify shift normalization |
| `R-WIN-01` | contradiction | `M1 correctness` | ✓ WindowTree with 7 tests in kjxlkj-core-state |
| `R-WIN-03` | contradiction | `M1 correctness` | ✓ Ctrl-w dispatch in normal.rs + focus_next/prev tests |
| `R-EXP-01` | contradiction | `M1 correctness` | ✓ ExplorerService crate with 4 tests |
| `R-EXP-02` | contradiction | `M1 correctness` | ✓ ExplorerState navigation and expand/collapse tested |
| `R-TERM-01` | contradiction | `M1 correctness` | ✓ TerminalService with Screen and Parser (7 tests) |
| `R-WRAP-01` | contradiction | `M1 correctness` | ✓ 5 wrap tests in kjxlkj-render including boundary padding |
| `R-CUR-02` | contradiction | `M1 correctness` | ✓ grapheme_width and wide char continuation in grid.rs |
| `R-ARCH-01` | spec-only | `M2 missing feature` | ✓ 19 crates in grouped paths under src/crates/{app,core,platform,services} |

## Open Rows

| Requirement ID | Canonical document | Observed status | Mismatch class | Action | Evidence needed |
|---|---|---|---|---|---|
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | test-gap | `M4 verification gap` | test-add | bounded-latency PTY E2E |
| `R-I18N-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | test-gap | `M4 verification gap` | test-add | IME composition race proofs |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | test-gap | `M4 verification gap` | test-add | PTY E2E harness for *R tests |

## Priority Rule

Resolve rows in this order:

1. `M1 correctness` ✓ All resolved
2. `M4 verification gap` - 3 remaining
3. `M5 stale docs`

## Summary

| Category | Resolved | Open |
|---|---|---|
| M1 correctness | 9 | 0 |
| M4 verification gap | 0 | 3 |
| Total | 9 | 3 |

## Related

- [x] Requirement matrix: [/docs/todo/current/requirement-matrix.md](/docs/todo/current/requirement-matrix.md)
- [x] Reference drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
