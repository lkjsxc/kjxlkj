# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: Reconstructed foundation baseline active on 2026-02-11. Runtime blocker reconstruction pending.

## Active Blockers

- [x] `LIM-BASELINE-IMPL-03` regenerate workspace manifests and grouped source tree (evidence: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), [/docs/log/reconstruction/audits/README.md](/docs/log/reconstruction/audits/README.md))
- [x] `LIM-BLOCK-KEY-03` `Shift+a` end-to-end append semantics are correct (evidence: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), `cargo test -p kjxlkj-test-harness --test key_mode_e2e`)
- [x] `LIM-BLOCK-WIN-03` split lifecycle preserves deterministic layout/focus (evidence: `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e --test window_nav_session_terminal_e2e`)
- [x] `LIM-BLOCK-NAV-03` `Ctrl-w` navigation works across mixed windows (evidence: `cargo test -p kjxlkj-test-harness --test window_nav_e2e --test window_nav_more_e2e --test window_nav_session_terminal_e2e`)
- [ ] `LIM-BLOCK-EXP-03` explorer launch and actions are reachable and stable
- [ ] `LIM-BLOCK-TERM-03` terminal windows obey shared lifecycle and remain responsive
- [ ] `LIM-BLOCK-CURSOR-03` cursor remains visible and grapheme-safe in churn
- [ ] `LIM-BLOCK-WRAP-03` long-line wrap never overflows and never splits wide graphemes
- [x] `LIM-BLOCK-TEST-03` blocker closure is backed by matching PTY E2E tests (evidence: `src/crates/app/kjxlkj-test-harness/src/pty.rs`, `cargo test -p kjxlkj-test-harness --test key_mode_e2e`)

## Working Matrices

- [x] [requirement-matrix.md](requirement-matrix.md)
- [x] [mismatch-matrix.md](mismatch-matrix.md)

## Execution Gates

- [x] [verification.md](verification.md)
- [x] [phases/README.md](phases/README.md)

## Required Traceability Docs

- [x] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Doc Coverage Direct Links

- [x] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- [x] [/docs/todo/doc-coverage/doc-coverage-1.md](/docs/todo/doc-coverage/doc-coverage-1.md)
- [x] [/docs/todo/doc-coverage/doc-coverage-2.md](/docs/todo/doc-coverage/doc-coverage-2.md)
- [x] [/docs/todo/doc-coverage/doc-coverage-3.md](/docs/todo/doc-coverage/doc-coverage-3.md)
- [x] [/docs/todo/doc-coverage/doc-coverage-4.md](/docs/todo/doc-coverage/doc-coverage-4.md)
- [x] [/docs/todo/doc-coverage/doc-coverage-5.md](/docs/todo/doc-coverage/doc-coverage-5.md)

## Exit Criteria

- [ ] no high-severity limitation remains open
- [x] all mandatory `*R` tests for closed blockers pass deterministically
- [x] matrices and reference ledgers are synchronized in one change
- [x] doc-coverage includes every markdown document directly
