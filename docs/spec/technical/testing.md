# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for blocker-first reconstruction waves.

## Objectives

| Objective | Requirement |
|---|---|
| Correctness | user-reachable behavior is proven by deterministic tests |
| Drift prevention | spec, code, tests, and TODO remain synchronized |
| Boundary safety | Unicode, wrapping, windows, terminal, and IME edges are covered |
| Anti-shortcut | no blocker closes without live PTY E2E evidence |

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | unit invariants | deterministic crate-level tests |
| `T1` | cross-module integration | action/state/snapshot integration tests |
| `T2` | live runtime E2E | full binary in PTY with raw input bytes |

High-severity blocker closure requires matching `T2` evidence.

## Retained High-Signal Implemented Tests

These implemented tests are high leverage and must be retained.

| Domain | Retained IDs |
|---|---|
| input decode | `WR-01`, decode key normalization tests |
| mode entry and cursor semantics | `WR-02`, `WR-08`, `CUR-01`..`CUR-06` |
| window and split core | `WR-04`, `WIN-02` |
| explorer wiring baseline | `WR-05`, explorer state tests |
| terminal parser/screen baseline | `WR-03`, `PE-01`, `PE-02`, `ST-07`..`ST-10` |
| wrap boundary baseline | `WR-07`, `BD-10` |
| IME baseline | `JP-01`..`JP-05` |

## Mandatory New Coverage Classes

| ID | Gap Closed | Required Tier |
|---|---|---|
| `TC-LIVE-01` | real PTY proof of `Shift+a` normalization and append semantics | `T2` |
| `TC-LIVE-02` | split/explorer/terminal behavior in one mixed-window flow | `T2` |
| `TC-LIVE-03` | cursor correctness under wrap and resize churn | `T2` |
| `TC-LIVE-04` | explorer operations under external FS drift | `T2` |
| `TC-LIVE-05` | terminal output flood while editing adjacent buffer | `T2` |
| `TC-LIVE-06` | IME composition races with leader/window commands | `T2` |

Canonical IDs are specified in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md).

## Boundary-First Selection Rule

Candidate tests are scored on:

- user impact
- regression probability
- unique detection power
- determinism
- maintenance cost

Select the smallest set that covers each high-risk surface at least once.

## Bug-Fix Closure Contract

For each fixed user-visible bug:

1. add or update one failing regression case that reproduces the bug
2. implement fix
3. pass regression plus matching live `*R` E2E case
4. update TODO and reference ledgers with direct evidence links

## Determinism Rules

- use bounded deadlines with explicit diagnostics
- avoid blind sleep; poll with deterministic timeout windows
- capture failure context: mode, focus, layout summary, cursor/caret, recent input trace
- avoid hidden external dependencies in blocker tests

## Completion Gate

A TODO checkbox may be marked complete only when all are true:

1. linked normative requirement exists
2. required tier evidence exists (`T2` for blockers)
3. behavior is reachable from real key/command paths
4. limitations and drift rows are synchronized in same change

## Related

- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- PTY harness contract: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
- Unit requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
