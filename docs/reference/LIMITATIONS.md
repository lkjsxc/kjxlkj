# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records open mismatches between target spec and current
reconstructed state.

## Baseline Limitation

| ID | Observed State | Expected State | Severity | Next Action |
|---|---|---|---|---|
| LIM-BASELINE-01 | 18 crates reconstructed, 207 tests pass, services implemented, boundary tests pass | Full implementation conforming to all spec areas | low | close remaining C2/C3 gaps |

## Closed Blockers (This Wave)

| ID | Required Outcome | Evidence | Status |
|---|---|---|---|
| LIM-BLOCK-KEY-01 | `Shift+a` normalizes to `A` | WR-01 test passes in kjxlkj-input, shift normalization in decode.rs | **closed** |
| LIM-BLOCK-CURSOR-01 | `a` at end-of-line differs from `i` | CUR-01 through CUR-05 tests pass in kjxlkj-core-state | **closed** |
| LIM-BLOCK-EXP-01 | Explorer launch and split-open wired | HE-04, HE-05, WR-05, BD-08 tests pass | **closed** |
| LIM-BLOCK-TERM-01 | Terminal window with VT parser | ST-01 to ST-12, PE-01 to PE-06, WR-03, WR-04 tests pass; alternate screen support added | **closed** |
| LIM-BLOCK-WIN-01 | Mixed-window Ctrl-w navigation | WR-06 test passes; Ctrl-w w/W/s/v/c/q navigation for buffer/explorer/terminal | **closed** |
| LIM-BLOCK-I18N-01 | IME composition and leader isolation | JP-01 to JP-05, PE-04 tests pass; composition model verified | **closed** |
| LIM-BLOCK-WRAP-01 | Long lines never render off-screen | BD-01, BD-02, BD-03, BD-10, WR-07 tests pass; width-2 boundary padding verified | **closed** |

## Remaining Gaps (Deferred)

| ID | Gap | Severity | Status |
|---|---|---|---|
| LIM-GAP-VISUAL-01 | Visual mode selection tracking and operator application | low | **closed** |
| LIM-GAP-REPLACE-01 | Replace mode overwrite with backspace restore | low | **closed** |
| LIM-GAP-EXP-02 | Explorer navigation (h/l expand/collapse) and file ops (create/rename/delete) | low | **closed** |
| LIM-GAP-SESS-01 | Auto-session save on exit and load on startup | low | **closed** |
| LIM-GAP-PTY-01 | Terminal resize propagation to terminal instances | medium | **closed** |
| LIM-GAP-REG-01 | Named registers with numbered, named, special register model | low | **closed** |

## Lifecycle Rules

- keep limitations open until deterministic evidence closes each gap
- close limitation and TODO item in the same change
- do not remove entries without proof

## Related

- Conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Current TODO: [/docs/todo/current/README.md](/docs/todo/current/README.md)
