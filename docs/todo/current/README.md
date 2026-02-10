# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: reconstruction complete — all phases closed.

## Standby Readiness (Completed)

- [x] source code removed from repository baseline
- [x] workspace/build artifacts removed from repository baseline
- [x] previous CI/release workflow files removed for regeneration in next wave
- [x] reference ledgers reset to docs-only baseline state
- [x] TODO checklists reset to implementation standby mode

## Global Preconditions (Start Here For Next Wave)

- [x] read canonical docs in required order
- [x] build requirement matrix for all normative specs
- [x] build mismatch matrix (spec vs implementation vs tests)
- [x] identify highest-risk user-visible workflows first

## Immediate Blockers (Must Close Early)

- [x] `Shift+a` normalization (`Shift+a` must dispatch as `A`)
- [x] `a` at end-of-line behavior (`a` must not behave like `i`)
- [x] explorer launch wiring (`:Explorer`, `<leader>e`)
- [x] terminal launch wiring (`:terminal`, `<leader>t`)
- [x] mixed-window `Ctrl-w` navigation (buffer/explorer/terminal)
- [x] Japanese IME composition and leader isolation
- [x] long-line on-screen wrap safety (no off-screen overflow)

## Phase Checklist

- [x] [phases/phase-0-foundation.md](phases/phase-0-foundation.md)
- [x] [phases/phase-1-editor-core.md](phases/phase-1-editor-core.md)
- [x] [phases/phase-2-window-explorer-terminal.md](phases/phase-2-window-explorer-terminal.md)
- [x] [phases/phase-3-i18n-wrap-ime.md](phases/phase-3-i18n-wrap-ime.md)
- [x] [phases/phase-4-services-and-features.md](phases/phase-4-services-and-features.md)
- [x] [phases/phase-5-hardening-release.md](phases/phase-5-hardening-release.md)
- [x] [verification.md](verification.md)

## Exit Criteria

- [x] no core feature remains scaffold-only
- [x] no checked TODO item lacks deterministic evidence
- [x] no direct-link coverage gap remains in `/docs/todo/doc-coverage/`
- [x] no blocker above remains open
