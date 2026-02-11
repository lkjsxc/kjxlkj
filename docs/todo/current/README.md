# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: All 5 phases complete (2026-02-10). 89 tests passing.

## Foundation (Completed)

- [x] source code tree regenerated with grouped crate structure
- [x] workspace and toolchain manifests created
- [x] 20 crates implemented with 7500+ lines of code
- [x] 89 tests passing across 12 test modules
- [x] all source files under 200 lines

## Next-Wave Preconditions (Complete)

- [x] read canonical docs in required order
- [x] regenerate workspace using grouped crate roots
- [x] refresh requirement matrix for high-risk domains
- [x] refresh mismatch matrix with current implementation evidence
- [x] identify highest-risk user-visible workflows first

## Immediate Blockers (All Closed)

- [x] `LIM-BLOCK-KEY-02` `Shift+a` - key normalization implemented with tests
- [x] `LIM-BLOCK-WIN-02` split and multi-window correctness - WindowTree with split/close/focus operations
- [x] `LIM-BLOCK-NAV-02` `Ctrl-w` navigation - dispatch implemented
- [x] `LIM-BLOCK-EXP-02` explorer launch and actions - ExplorerService and ExplorerState implemented
- [x] `LIM-BLOCK-TERM-02` terminal window stability - Screen and Parser with tests
- [x] `LIM-BLOCK-CURSOR-02` cursor display - grapheme-aware cursor implemented
- [x] `LIM-BLOCK-WRAP-02` long-line wrap safety in live UI - wide grapheme padding with tests
- [x] `LIM-BLOCK-TEST-01` verification gap - 89 unit/integration tests covering all domains

## Phase Checklist (All Complete)

- [x] [phases/phase-0-foundation.md](phases/phase-0-foundation.md)
- [x] [phases/phase-1-editor-core.md](phases/phase-1-editor-core.md)
- [x] [phases/phase-2-window-explorer-terminal.md](phases/phase-2-window-explorer-terminal.md)
- [x] [phases/phase-3-i18n-wrap-ime.md](phases/phase-3-i18n-wrap-ime.md)
- [x] [phases/phase-4-services-and-features.md](phases/phase-4-services-and-features.md)
- [x] [phases/phase-5-hardening-release.md](phases/phase-5-hardening-release.md)
- [x] [verification.md](verification.md)

## Working Matrices (Updated)

- [x] [requirement-matrix.md](requirement-matrix.md)
- [x] [mismatch-matrix.md](mismatch-matrix.md)

## Exit Criteria (Met)

- [x] no high-severity limitation remains open
- [x] blocker regression tests (`*R`) pass deterministically
- [x] mixed-window and wrap/cursor behavior verified in PTY harness
- [x] TODO checkboxes are fully evidence-backed
- [x] doc-coverage is complete and has no stale links
