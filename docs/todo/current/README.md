# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: active blocker-closure wave (2026-02-10).

## Global Preconditions

- [x] canonical docs read in required order
- [x] requirement matrix refreshed for high-risk domains
- [x] mismatch matrix refreshed with `M1`..`M5` classes
- [x] existing automated test evidence captured (`cargo test --workspace`)
- [x] user-reported runtime failures promoted to open limitations

## Immediate Blockers (Must Close First)

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
