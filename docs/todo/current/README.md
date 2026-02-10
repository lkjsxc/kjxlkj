# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: docs-only standby baseline active (2026-02-10).

## Standby Baseline (Completed)

- [x] source code tree removed from repository baseline
- [x] workspace/toolchain/build manifests removed from root baseline
- [x] reconstructed-profile CI workflow removed for regeneration
- [x] reference ledgers synchronized to docs-only state

## Next-Wave Preconditions (Start Here)

- [x] read canonical docs in required order
- [x] regenerate workspace using grouped crate roots
- [ ] refresh requirement matrix for high-risk domains
- [ ] refresh mismatch matrix with current implementation evidence
- [ ] identify highest-risk user-visible workflows first

## Immediate Blockers (Must Close First After Regeneration)

- [ ] `LIM-BLOCK-KEY-02` `Shift+a` real runtime path
- [ ] `LIM-BLOCK-WIN-02` split and multi-window correctness
- [ ] `LIM-BLOCK-NAV-02` mixed-window `Ctrl-w` navigation correctness
- [ ] `LIM-BLOCK-EXP-02` explorer launch and actions
- [ ] `LIM-BLOCK-TERM-02` terminal window stability
- [ ] `LIM-BLOCK-CURSOR-02` cursor display correctness
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
