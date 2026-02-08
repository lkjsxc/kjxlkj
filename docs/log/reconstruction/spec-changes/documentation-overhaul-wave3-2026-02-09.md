# Documentation Overhaul: Wave 3

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

## Summary

Wave 3 completed skeleton expansion for all spec files. 365 spec files now exist under `/docs/spec/`, all with substantive content (minimum 30+ content lines for non-README files, with the exception of intentionally concise specs like mouse-support.md).

## Changes in this wave

### Skeleton expansion (sessions 1-3)

Approximately 60 skeleton spec files were expanded to full specification-quality content across 8 commits. Each expanded file includes:

- Normative requirements in table format
- Data model definitions with field types
- Behavioral edge cases and error handling
- Configuration options with defaults
- Cross-references to related specs
- CJK/wide character handling where applicable

### Structural improvements

| Change | Detail |
|---|---|
| startup.md added to doc-coverage | Was the only spec file missing from `/docs/todo/doc-coverage/doc-coverage-2.md` |
| startup.md linked from architecture TODO | New "Startup and shutdown" section with 10 checklist items |
| principles.md expanded | Added detailed implementation guidance for each core principle |
| mouse-support.md expanded | Added terminal emulator interaction rules and rationale |

### Documentation audit findings

| Finding | Status |
|---|---|
| Cursor/CJK spec is thorough | Verified: grapheme-boundary model, no half-cell, motion atomicity, mixed-width lines, `a`/`Esc` regression guard |
| Terminal emulator spec is thorough | Verified: cell model, PTY lifecycle, escape parser state machine, CSI dispatch, SGR colors, private modes |
| Viewport/wrapping spec is thorough | Verified: wrap algorithm, CJK boundary padding, cursor-follow algorithm, resize handling |
| Session JSON schema is thorough | Verified: LayoutNode recursion, WindowRef content_type, BufferRef, MarkRef, load algorithm, error handling |
| Testing specs are thorough | Verified: 68 unit tests, 10 integration, 9 headless E2E, 8 PTY E2E, 19 boundary tests |
| All TODOs unchecked | Verified: no `- [x]` items in any TODO file |

## Remaining thin files

After wave 3, only README index files remain with content below 50 lines:

- These are index/navigation files and are adequate at their current size
- No content spec files remain below 30 content lines

## Verification

- All 14 TODO checklist files reviewed
- Doc-coverage index verified: 1 missing file fixed (startup.md)
- No `../` links found in documentation
- All TODOs remain `- [ ]`
