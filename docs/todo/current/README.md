# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

Status: Docs-only baseline active on 2026-02-11. Full implementation reconstruction pending.

## Active Blockers

- [ ] `LIM-BASELINE-IMPL-03` regenerate workspace manifests and grouped source tree
- [ ] `LIM-BLOCK-KEY-03` `Shift+a` end-to-end append semantics are correct
- [ ] `LIM-BLOCK-WIN-03` split lifecycle preserves deterministic layout/focus
- [ ] `LIM-BLOCK-NAV-03` `Ctrl-w` navigation works across mixed windows
- [ ] `LIM-BLOCK-EXP-03` explorer launch and actions are reachable and stable
- [ ] `LIM-BLOCK-TERM-03` terminal windows obey shared lifecycle and remain responsive
- [ ] `LIM-BLOCK-CURSOR-03` cursor remains visible and grapheme-safe in churn
- [ ] `LIM-BLOCK-WRAP-03` long-line wrap never overflows and never splits wide graphemes
- [ ] `LIM-BLOCK-TEST-03` blocker closure is backed by matching PTY E2E tests

## Working Matrices

- [ ] [requirement-matrix.md](requirement-matrix.md)
- [ ] [mismatch-matrix.md](mismatch-matrix.md)

## Execution Gates

- [ ] [verification.md](verification.md)
- [ ] [phases/README.md](phases/README.md)

## Required Traceability Docs

- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Doc Coverage Direct Links

- [ ] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- [ ] [/docs/todo/doc-coverage/doc-coverage-1.md](/docs/todo/doc-coverage/doc-coverage-1.md)
- [ ] [/docs/todo/doc-coverage/doc-coverage-2.md](/docs/todo/doc-coverage/doc-coverage-2.md)
- [ ] [/docs/todo/doc-coverage/doc-coverage-3.md](/docs/todo/doc-coverage/doc-coverage-3.md)
- [ ] [/docs/todo/doc-coverage/doc-coverage-4.md](/docs/todo/doc-coverage/doc-coverage-4.md)
- [ ] [/docs/todo/doc-coverage/doc-coverage-5.md](/docs/todo/doc-coverage/doc-coverage-5.md)

## Exit Criteria

- [ ] no high-severity limitation remains open
- [ ] all mandatory `*R` tests for closed blockers pass deterministically
- [ ] matrices and reference ledgers are synchronized in one change
- [ ] doc-coverage includes every markdown document directly
