# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified with deterministic evidence.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | confirmed by deterministic evidence in current repo state |
| `partial` | partly available but not yet release-safe |
| `unverified` | no trustworthy runtime evidence is currently available |
| `blocked` | known high-severity mismatch remains open |

## Current Snapshot (2026-02-10)

The repository now has a working editor framework implementation.

- 18 crates in grouped structure (app/core/platform/services)
- 69+ source files with 5200+ lines of Rust code
- 11 tests passing (8 core-text, 2 input, 1 undo)
- All files under 200 lines

## Verification Evidence Available

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Grouped crate topology | `verified` | 2026-02-10 | src/crates/{app,core,platform,services} present |
| Workspace builds | `verified` | 2026-02-10 | `cargo build` succeeded |
| Tests pass | `verified` | 2026-02-10 | `cargo test --workspace` 11 pass |
| Clippy clean | `verified` | 2026-02-10 | No clippy warnings |
| Files under 200 lines | `verified` | 2026-02-10 | All source files ≤186 lines |
| TODO doc coverage completeness | `verified` | 2026-02-10 | direct-link inventory covers all markdown docs |
| Markdown link integrity | `partial` | 2026-02-10 | not yet verified with link checker |
| Runtime E2E tests | `unverified` | 2026-02-10 | No PTY E2E harness implemented yet |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Input decoding and key normalization | `partial` | crossterm EventStream integration, shift normalization implemented |
| Cursor semantics and display | `partial` | grapheme-aware cursor, basic positioning implemented |
| Window tree and split management | `partial` | window types defined, layout tree structure implemented |
| Explorer window and actions | `unverified` | stub service only |
| Terminal window integration | `partial` | portable-pty integration started |
| Wrapping and viewport safety | `partial` | basic grid rendering, width-2 handling present |
| Service integrations (LSP/Git/Index/FS) | `unverified` | stub services only |
| Source topology and workspace layout | `verified` | grouped crate-root layout regenerated |
| Documentation and TODO integrity | `partial` | blocker-first reconstruction controls active |

## Release Readiness Rule

The implementation MUST NOT be considered conformant for release until all critical blockers in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed with:

1. reconstructed runtime behavior from real input paths
2. deterministic regression and live E2E evidence
3. synchronized `CONFORMANCE`, `LIMITATIONS`, `DRIFT_MATRIX`, and TODO updates

## Related

- Open blockers: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction plan: [/docs/todo/current/README.md](/docs/todo/current/README.md)
