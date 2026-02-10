# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

This document defines the mandatory verification gate for reconstruction.

## Objectives

| Objective | Requirement |
|---|---|
| Correctness | user-reachable behavior is covered by deterministic tests |
| Drift prevention | spec, code, tests, and TODO remain synchronized |
| Boundary safety | Unicode, wrapping, windows, terminal, and session edges are tested |
| Anti-shortcut | no feature is considered done unless runtime wiring is verified |

## Required Layers

| Layer | Minimum Scope |
|---|---|
| Unit | local invariants and parser/algorithm correctness |
| Integration | cross-module/cross-crate behavior transitions |
| Headless E2E | full editing workflows without PTY transport |
| PTY E2E | real terminal IO, resize, and lifecycle behavior |
| Boundary/Stress | long lines, CJK, IME, resize storms, concurrency |

## High-Leverage Selection Rule

Each candidate test is scored for:

- user impact
- regression probability
- unique detection power
- determinism
- maintenance cost

Only the smallest set with highest total risk coverage is mandatory.

## Preferred Existing Seed Tests

These tests are strong reconstruction anchors and SHOULD be retained first.

| Category | Seed Test |
|---|---|
| append regression | `reg01_append_eol` |
| wrap correctness | `reg02_long_line_wraps`, `reg08_wrap_padding` |
| Unicode safety | `reg06_unicode`, `reg07_cjk_half_cell` |
| split lifecycle | `he06_split` |
| session baseline | `he07_session`, `bd21_session_splits` |
| resize robustness | `pe05_resize_storm`, `bd16_resize_storm` |

## Mandatory New Coverage Classes

| ID | Gap Closed |
|---|---|
| TC-WIRE-01 | terminal launch path (`:terminal`, `<leader>t`) reaches PTY-backed window |
| TC-WIRE-02 | explorer launch path (`:Explorer`, `<leader>e`) reaches visible tree window |
| TC-WIRE-03 | `Shift+a` normalization and `a`/`i` distinction through real input decoder |
| TC-I18N-01 | Japanese IME composition does not leak into leader mappings |
| TC-WRAP-01 | long lines always stay on-screen (no off-screen overflow) |
| TC-WIN-01 | mixed buffer/explorer/terminal `Ctrl-w` navigation correctness |

## Determinism Rules

- use bounded timeouts and explicit failure diagnostics
- avoid blind sleeps; poll with deterministic deadlines
- prefer persisted state assertions over fragile screen snapshots
- add one regression test with each bug fix

## Completion Gate

A TODO item may be checked only when all are true:

1. linked normative spec requirement exists
2. deterministic test evidence exists
3. behavior is reachable from real input/command path
4. remaining user-visible gaps are listed in `LIMITATIONS`

## Related

- Unit requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- E2E and boundary matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
