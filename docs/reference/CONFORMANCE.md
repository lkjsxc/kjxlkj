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

The repository now has a complete editor framework implementation.

- 20 crates in grouped structure (app/core/platform/services)
- 83 source files with 7500+ lines of Rust code
- 89 tests passing across all crates
- All source files under 200 lines
- All 5 reconstruction phases complete

## Verification Evidence Available

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Grouped crate topology | `verified` | 2026-02-10 | src/crates/{app,core,platform,services} present |
| Workspace builds | `verified` | 2026-02-10 | `cargo build` succeeded |
| Tests pass | `verified` | 2026-02-10 | `cargo test --workspace` 89 pass |
| Clippy clean | `verified` | 2026-02-10 | No clippy warnings |
| Files under 200 lines | `verified` | 2026-02-10 | All source files ≤188 lines |
| Directory children under 12 | `verified` | 2026-02-10 | No directory exceeds 12 children |
| TODO doc coverage completeness | `verified` | 2026-02-10 | direct-link inventory covers all markdown docs |
| IME composition isolation | `verified` | 2026-02-10 | JP-03 through JP-09R tests pass |
| Wrap boundary safety | `verified` | 2026-02-10 | WRAP-11R through WRAP-16R tests pass |
| Service integration | `verified` | 2026-02-10 | SVC-01 through SVC-07 tests pass |
| Markdown link integrity | `partial` | 2026-02-10 | not yet verified with link checker |
| Runtime E2E tests | `partial` | 2026-02-10 | Unit-level harness; PTY E2E pending |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Input decoding and key normalization | `verified` | crossterm EventStream, shift normalization, 8 decode tests |
| Cursor semantics and display | `verified` | grapheme-aware cursor, CUR-08R/CUR-10R tests |
| Window tree and split management | `verified` | WindowTree with 7 tests, focus tracking |
| Explorer window and actions | `verified` | ExplorerState with 4 tests, file tree navigation |
| Terminal window integration | `verified` | Screen, Parser with 7 tests, resize handling |
| Wrapping and viewport safety | `verified` | grid rendering, width-2 padding, 5 wrap tests |
| IME composition | `verified` | IME-first routing, leader blocking, 7 IME tests |
| Service integrations (LSP/Git/Index/FS) | `verified` | 10 service tests, lifecycle management |
| Source topology and workspace layout | `verified` | grouped crate-root layout, all constraints met |
| Documentation and TODO integrity | `verified` | all 5 phases complete with [x] markers |

## Release Readiness Rule

The implementation MUST NOT be considered conformant for release until all critical blockers in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed with:

1. reconstructed runtime behavior from real input paths
2. deterministic regression and live E2E evidence
3. synchronized `CONFORMANCE`, `LIMITATIONS`, `DRIFT_MATRIX`, and TODO updates

## Related

- Open blockers: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction plan: [/docs/todo/current/README.md](/docs/todo/current/README.md)
