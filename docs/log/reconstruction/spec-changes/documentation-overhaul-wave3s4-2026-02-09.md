# Documentation Overhaul: Wave 3 Session 4

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

Date: 2026-02-09

## Summary

Final quality audit and cross-link integrity pass across the entire documentation.

## Changes

Documentation changes in wave 3 session 4.

### Anti-MVP expansion (all 18 crates)

- Extended minimum code volume targets from 10 to 18 crates in [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)
- Created [/docs/log/proposals/deep-wiring-checklist-2.md](/docs/log/proposals/deep-wiring-checklist-2.md) covering 11 remaining crates:
  - Binary entrypoint (`kjxlkj`): CLI, setup, shutdown modules
  - Facade (`kjxlkj-core`): re-export inventory
  - Types (`kjxlkj-core-types`): ids, mode, action, key, color, range modules
  - Undo (`kjxlkj-core-undo`): tree, node, change, group, persistence modules
  - UI model (`kjxlkj-core-ui`): snapshot, buffer/terminal snapshot, cmdline, notification modules
  - Host (`kjxlkj-host`): terminal, event_stream, signals, pty, panic modules
  - Services supervisor (`kjxlkj-services`): supervisor, health, bus modules
  - LSP (`kjxlkj-service-lsp`): client, lifecycle, completion, diagnostics, hover, goto modules
  - Git (`kjxlkj-service-git`): subprocess, status, diff, blame modules
  - Index (`kjxlkj-service-index`): scanner, fuzzy, symbol modules
  - FS (`kjxlkj-service-fs`): read_write, watcher, encoding, line_ending modules
- Updated RECONSTRUCTION_PROMPT to reference part 2
- Updated architecture TODO to include all 18 crate minimum line targets

### Cross-link integrity fix

- Fixed 28 broken relative links across 23 spec files (missing leading `/` in absolute-style paths)
- Fixed 5 broken absolute links:
  - `file-explorer.md` -> `file_explorer.md` (underscore vs hyphen)
  - `session-management.md` -> `sessions.md`
  - `modes/cmdline/README.md` -> `modes/command.md`
  - `modes/terminal.md` -> `features/terminal/terminal.md`
  - `ui/viewport.md` -> `features/ui/viewport.md`
- Verified zero broken links remain (both absolute and relative, across all 435 doc files)

### Doc-coverage completion

- Added 2 missing entries for insert-mode files at parent level (`insert-abbreviations.md`, `insert-literal.md`)
- Verified 100% coverage: 435 doc files, 436 doc-coverage entries (covers all)

### Verification results

- Zero `../` links in documentation
- Zero broken absolute links
- Zero broken relative links
- Zero files exceeding 200 lines
- Zero checked TODO items
- All 14 TODO checklists verified complete and comprehensive

## Commits

- `9b87060`: complete 18-crate wiring inventory and anti-MVP volume targets
- `a4c866e`: fix 28 broken cross-links across 23 spec files, expand architecture TODO to 18 crates
- `cee4217`: fix doc-coverage gap (2 insert mode files at parent level)
