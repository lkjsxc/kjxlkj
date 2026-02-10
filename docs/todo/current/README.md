# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: standby baseline for full reimplementation.

## Standby Readiness (Completed)

- [x] source code removed from repository baseline
- [x] workspace/build artifacts removed from repository baseline
- [x] previous CI/release workflow files removed for regeneration in next wave
- [x] reference ledgers reset to docs-only baseline state
- [x] TODO checklists reset to implementation standby mode

## Global Preconditions (Start Here For Next Wave)

- [ ] read canonical docs in required order
- [ ] build requirement matrix for all normative specs
- [ ] build mismatch matrix (spec vs implementation vs tests)
- [ ] identify highest-risk user-visible workflows first

## Immediate Blockers (Must Close Early)

- [ ] `Shift+a` normalization (`Shift+a` must dispatch as `A`)
- [ ] `a` at end-of-line behavior (`a` must not behave like `i`)
- [ ] explorer launch wiring (`:Explorer`, `<leader>e`)
- [ ] terminal launch wiring (`:terminal`, `<leader>t`)
- [ ] mixed-window `Ctrl-w` navigation (buffer/explorer/terminal)
- [ ] Japanese IME composition and leader isolation
- [ ] long-line on-screen wrap safety (no off-screen overflow)

## Phase Checklist

- [ ] [phases/phase-0-foundation.md](phases/phase-0-foundation.md)
- [ ] [phases/phase-1-editor-core.md](phases/phase-1-editor-core.md)
- [ ] [phases/phase-2-window-explorer-terminal.md](phases/phase-2-window-explorer-terminal.md)
- [ ] [phases/phase-3-i18n-wrap-ime.md](phases/phase-3-i18n-wrap-ime.md)
- [ ] [phases/phase-4-services-and-features.md](phases/phase-4-services-and-features.md)
- [ ] [phases/phase-5-hardening-release.md](phases/phase-5-hardening-release.md)
- [ ] [verification.md](verification.md)

## Exit Criteria

- [ ] no core feature remains scaffold-only
- [ ] no checked TODO item lacks deterministic evidence
- [ ] no direct-link coverage gap remains in `/docs/todo/doc-coverage/`
- [ ] no blocker above remains open
