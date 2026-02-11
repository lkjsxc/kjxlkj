# Wave Progress: Stage 04 Late (Waves 035–039)

Archived from wave-progress.md for file-size compliance.

### Wave 035: Command and Route Wiring
- Status: COMPLETE
- Committed: cc415d57 (impl) + f9f81ec8 (tests)
- Evidence: 348 tests pass, all files ≤ 200 lines
- Key: Wincmd expanded (W/H/J/K/L/r/R/x), terminal open, explorer v/s split-open,
  focus cycle reverse. 21 new tests.

### Wave 036: Boundary and Error Semantics
- Status: COMPLETE
- Committed: b5245bc6
- Evidence: 374 tests pass, all files ≤ 200 lines
- Key: Jumplist (Ctrl-o/Ctrl-i), changelist (g;/g,), PositionList data structure.
  26 new tests.

### Wave 037: Unit and Integration Coverage
- Status: COMPLETE
- Committed: f896418a
- Evidence: 391 tests pass, all files ≤ 200 lines
- Key: Mark system (m{a-z}/'{a-z}/`{a-z}), MarkStore. 17 new tests.

### Wave 038: Live E2E and Race Validation
- Status: COMPLETE
- Committed: 3cdbc363
- Evidence: 411 tests pass, all files ≤ 200 lines
- Key: Macro recording/playback (q{a-z}/@{a-z}), MacroState. 20 new tests.

### Wave 039: Ledger Synchronization and Stage Exit
- Status: COMPLETE
- Committed: b0509062
- Evidence: 442 tests pass, all files ≤ 200 lines
- Key: Fold commands (zo/zc/za/zR/zM/zr/zm/zj/zk), FoldState, indent-based
  fold computation. 31 new tests.
