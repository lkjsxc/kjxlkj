# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for blocker-first reconstruction waves.

## Objectives

| Objective | Requirement |
|---|---|
| Correctness | user-reachable behavior is proven by deterministic tests |
| Drift prevention | spec, code, tests, and TODO remain synchronized |
| Boundary safety | Unicode, wrapping, windows, terminal, and IME edges are covered |
| Anti-shortcut | no blocker closes without screen-asserted live PTY E2E evidence |

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | unit invariants | deterministic crate-level tests |
| `T1` | cross-module integration | action/state/snapshot integration tests |
| `T2` | live runtime E2E | full binary in PTY with per-key state + screen assertions |

High-severity blocker closure requires matching `T2` evidence.

## Retained High-Signal Implemented Suites

| Domain | Current Suite | Required Upgrade |
|---|---|---|
| input decode | `key_mode_e2e` | add per-key frame oracle checks |
| split/navigation | `window_nav_e2e`, `window_nav_more_e2e`, `window_nav_session_terminal_e2e` | assert pane geometry timeline per input |
| explorer/terminal routes | `explorer_terminal_paths_e2e`, `explorer_terminal_more_e2e` | assert visible pane outcomes, not only trace routes |
| churn/races | `explorer_terminal_stress_e2e` | add frame-diff oracle and stale-ID assertions |
| wrap/cursor | `cursor_wrap_e2e`, `cursor_wrap_more_e2e` | keep as baseline and add mixed-pane screen checks |
| profiling | `profiling_e2e` | keep probe checks with bounded-noise assertions |

## Mandatory New Coverage Classes

| ID | Gap Closed | Required Tier |
|---|---|---|
| `TC-LIVE-01` | real PTY proof of `Shift+a` correctness with frame-level equality vs physical `A` | `T2` |
| `TC-LIVE-02` | split/explorer/terminal behavior in one mixed-window flow with pane map checks | `T2` |
| `TC-LIVE-03` | cursor correctness under wrap and resize churn with visual cursor oracle | `T2` |
| `TC-LIVE-04` | explorer operations under external FS drift with visible tree oracle | `T2` |
| `TC-LIVE-05` | terminal flood while editing adjacent pane with latency + frame coherence checks | `T2` |
| `TC-LIVE-06` | IME composition races with leader/window commands and no leakage | `T2` |
| `TC-LIVE-07` | replay determinism by comparing complete per-key dump timelines | `T2` |

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

1. add or update a failing regression case that reproduces the bug
2. implement fix
3. pass regression plus matching live `*R` E2E case with screen-state assertions
4. update TODO and reference ledgers with direct evidence links

## Determinism Rules

- use bounded deadlines with explicit diagnostics
- avoid blind sleep; poll with deterministic timeout windows
- capture failure context: mode, focus, layout summary, cursor/caret, frame excerpt, recent input trace
- avoid hidden external dependencies in blocker tests

## Completion Gate

A TODO checkbox may be marked complete only when all are true:

1. linked normative requirement exists
2. required tier evidence exists (`T2` for blockers)
3. behavior is reachable from real key/command paths
4. screen-state assertions pass for blocker E2E tests
5. limitations and drift rows are synchronized in the same change

## Related

- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- PTY harness contract: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
- Unit requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
