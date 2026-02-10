# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Reconstruction program for production-grade parity.

## Global Preconditions

- [ ] read canonical docs in required order
- [ ] build/update requirement matrix for all normative specs
- [ ] build/update mismatch matrix (spec vs runtime vs tests)
- [ ] identify highest-risk user-visible failures first

## Immediate Blockers (Must Close Early)

- [ ] `Shift+a` normalization bug (`Shift+a` must dispatch as `A`)
- [ ] `a` at end-of-line bug (`a` must not behave like `i`)
- [ ] explorer launch wiring (`:Explorer`, `<leader>e`)
- [ ] terminal launch wiring (`:terminal`, `<leader>t`)
- [ ] mixed-window `Ctrl-w` navigation across buffer/explorer/terminal
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
- [ ] no closed TODO lacks associated test evidence
- [ ] no direct-link coverage gap remains in `/docs/todo/doc-coverage/`
- [ ] no open blocker above remains unresolved
