# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified with deterministic evidence.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | confirmed by deterministic evidence in current repo state |
| `partial` | partly available but user-visible reliability is not yet acceptable |
| `unverified` | no trustworthy evidence for production behavior |
| `blocked` | known mismatch with user-visible failure |

## Current Snapshot (2026-02-10)

The repository has source code and passing automated tests, but it is NOT release-ready.

User-reported runtime failures are currently treated as authoritative blockers for:

- split view behavior
- explorer launch and interaction
- `Shift+a` action path
- multi-window navigation stability
- long-line wrapping and cursor display
- explorer and terminal window reliability

## Verification Evidence Available

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| `cargo test --workspace` | `verified` | 2026-02-10 | 230 tests passed |
| Unit/service contracts | `verified` | 2026-02-10 | crate-local suites pass |
| Real interactive PTY E2E | `unverified` | 2026-02-10 | no deterministic harness proving user-facing flows end-to-end |
| Manual user acceptance | `blocked` | 2026-02-10 | user reports critical runtime failures |

## Domain Summary

| Domain | Status | Note |
|---|---|---|
| Input decoding and key normalization | `partial` | model-level tests pass, but `Shift+a` is reported broken in real usage |
| Cursor semantics and display | `partial` | grapheme tests pass, but cursor display/wrap behavior is reported unstable |
| Window tree and split management | `blocked` | split and multi-window behavior reported broken |
| Explorer window and actions | `blocked` | explorer and related actions reported non-working |
| Terminal window integration | `partial` | service tests pass; user-facing window reliability still suspect |
| Wrapping and viewport safety | `partial` | boundary tests pass; real-screen behavior still reported buggy |
| Services (LSP/Git/Index/FS) | `verified` | service-level tests pass, no new blocker report yet |
| Source topology and workspace layout | `partial` | grouped crate-root topology is specified, but current source tree remains flat and requires migration |
| Documentation and TODO integrity | `partial` | TODO previously over-claimed completion; now reset to blocker-first wave |

## Release Readiness Rule

The implementation MUST NOT be considered conformant for release until all critical blockers in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed with:

1. deterministic live E2E evidence
2. reproduced bug regression tests
3. synchronized `CONFORMANCE`, `LIMITATIONS`, `DRIFT_MATRIX`, and TODO updates

## Related

- Open blockers: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction plan: [/docs/todo/current/README.md](/docs/todo/current/README.md)
