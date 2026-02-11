# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records the strongest verified state as of the snapshot date.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior exists but verification or reachability is incomplete |
| `blocked` | high-severity mismatch is known and not yet closed |
| `unverified` | no trustworthy evidence currently exists |

## Current Snapshot (2026-02-11)

Reconstructed foundation baseline is active. Workspace and grouped crate topology are present, while runtime feature behavior remains in early scaffold state.

Repository evidence in this snapshot:

- `src/crates/` grouped roots (`app`, `core`, `platform`, `services`) are present
- root `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml` are present
- workspace membership resolves for all required crate paths
- deterministic topology/file-size smoke tests pass in `kjxlkj-test-harness`

Because runtime feature logic is not yet reconstructed, runtime behavior domains cannot be marked `verified`.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| Workspace and grouped source tree are reconstructed | `verified` | 2026-02-11 | root `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, and `src/crates/...` tree |
| Reconstructed-basic verification profile checks pass | `verified` | 2026-02-11 | `cargo check --workspace`; `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` |
| Source topology and source-file-size smoke checks pass | `verified` | 2026-02-11 | `cargo test -p kjxlkj-test-harness` includes grouped-path, fan-out, and `<=200` line assertions |
| Shift-normalization append path is wired and PTY-verified | `verified` | 2026-02-11 | `cargo test -p kjxlkj-test-harness --test key_mode_e2e` (`KEY-TRACE-01`, `WR-01R`) |
| Runtime E2E gate for blocker closure is defined and partially reconstructed | `partial` | 2026-02-11 | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md), `src/crates/app/kjxlkj-test-harness/tests/key_mode_e2e.rs` |
| Runtime feature conformance | `unverified` | 2026-02-11 | runtime crates are scaffolded; behavior blockers remain open |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `partial` | `Shift+a -> A` normalization and append dispatch are verified; broader keymap domains remain open |
| Window tree and split lifecycle | `unverified` | blocker behavior not yet implemented |
| Mixed-window navigation (`Ctrl-w`) | `unverified` | blocker behavior not yet implemented |
| Explorer window and actions | `unverified` | blocker behavior not yet implemented |
| Terminal window integration | `unverified` | blocker behavior not yet implemented |
| Viewport wrap safety | `unverified` | blocker behavior not yet implemented |
| Cursor visibility and grapheme safety | `unverified` | blocker behavior not yet implemented |
| IME interaction around leader/window commands | `unverified` | blocker behavior not yet implemented |
| Source topology and workspace policy | `verified` | grouped tree, workspace manifests, and topology audits are passing |
| Documentation coverage and TODO integrity | `partial` | baseline blocker closed; runtime blocker closures pending |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

Release status may switch from blocked only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
   are closed
2. matching `*R` E2E tests in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
   pass deterministically
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/current/README.md](/docs/todo/current/README.md) are synchronized in the same change

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Active TODO wave: [/docs/todo/current/README.md](/docs/todo/current/README.md)
