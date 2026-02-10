# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: Phase 0 complete, Phase 1 in progress (2026-02-10).

## Foundation (Completed)

- [x] source code tree regenerated with grouped crate structure
- [x] workspace and toolchain manifests created
- [x] 18 crates implemented with 5300+ lines of code
- [x] 36 tests passing across 6 test modules
- [x] all source files under 200 lines

## Next-Wave Preconditions (Start Here)

- [x] read canonical docs in required order
- [x] regenerate workspace using grouped crate roots
- [ ] refresh requirement matrix for high-risk domains
- [ ] refresh mismatch matrix with current implementation evidence
- [ ] identify highest-risk user-visible workflows first

## Immediate Blockers (Progress Tracking)

- [x] `LIM-BLOCK-KEY-02` `Shift+a` - key normalization implemented with tests
- [x] `LIM-BLOCK-WIN-02` split and multi-window correctness - WindowTree with split/close/focus operations
- [x] `LIM-BLOCK-NAV-02` `Ctrl-w` navigation - dispatch implemented
- [x] `LIM-BLOCK-EXP-02` explorer launch and actions - ExplorerService and ExplorerState implemented
- [ ] `LIM-BLOCK-TERM-02` terminal window stability
- [x] `LIM-BLOCK-CURSOR-02` cursor display - grapheme-aware cursor implemented
- [ ] `LIM-BLOCK-WRAP-02` long-line wrap safety in live UI
- [ ] `LIM-BLOCK-TEST-01` verification gap (insufficient live E2E)

## Phase Checklist

- [ ] [phases/phase-0-foundation.md](phases/phase-0-foundation.md)
- [ ] [phases/phase-1-editor-core.md](phases/phase-1-editor-core.md)
- [ ] [phases/phase-2-window-explorer-terminal.md](phases/phase-2-window-explorer-terminal.md)
- [ ] [phases/phase-3-i18n-wrap-ime.md](phases/phase-3-i18n-wrap-ime.md)
- [ ] [phases/phase-4-services-and-features.md](phases/phase-4-services-and-features.md)
- [ ] [phases/phase-5-hardening-release.md](phases/phase-5-hardening-release.md)
- [ ] [verification.md](verification.md)

## Working Matrices

- [ ] [requirement-matrix.md](requirement-matrix.md)
- [ ] [mismatch-matrix.md](mismatch-matrix.md)

## Exit Criteria

- [ ] no high-severity limitation remains open
- [ ] blocker regression tests (`*R`) pass deterministically
- [ ] mixed-window and wrap/cursor behavior verified in PTY harness
- [ ] TODO checkboxes are fully evidence-backed
- [ ] doc-coverage is complete and has no stale links
