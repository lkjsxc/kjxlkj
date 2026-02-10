# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction waves.

## Objectives

| Objective | Requirement |
|---|---|
| Correctness | user-reachable behavior is covered by deterministic tests |
| Drift prevention | spec, implementation, tests, and TODO stay synchronized |
| Boundary safety | Unicode, wrapping, windows, terminal, and IME edges are tested |
| Anti-shortcut | no feature is complete unless real runtime wiring is proven |

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | unit/model invariants | crate-level deterministic tests |
| `T1` | cross-module integration | state transition and dispatch integration tests |
| `T2` | live runtime E2E | binary-in-PTY scenarios covering real key/command paths |

Critical blockers require `T2` evidence. `T0` + `T1` alone are insufficient.

## High-Leverage Selection Rule

Each candidate test is scored on:

- user impact
- regression probability
- unique detection power
- determinism
- maintenance cost

Select the smallest set that maximizes total risk coverage and avoids duplicates.

## Existing High-Signal Tests To Retain

These implemented tests are strong anchors and MUST remain:

| Surface | Existing Test IDs |
|---|---|
| key/append routing | `WR-01`, `WR-02`, `WR-08`, `CUR-01`..`CUR-06` |
| explorer/terminal wiring | `WR-03`, `WR-04`, `WR-05`, `HE-04`, `HE-05`, `HE-06` |
| mixed-window movement | `WR-06`, `PE-05`, `WIN-02` |
| wrap boundaries | `BD-01`, `BD-02`, `BD-10`, `WR-07` |
| IME isolation | `JP-01`..`JP-05`, `PE-04` |
| terminal lifecycle | `PE-01`, `PE-02`, `ST-07`..`ST-10` |

## Mandatory New Coverage Classes

| ID | Gap Closed |
|---|---|
| `TC-LIVE-01` | real PTY key decoding confirms `Shift+a` -> `A` |
| `TC-LIVE-02` | split/explorer/terminal behavior validated in one mixed-window flow |
| `TC-LIVE-03` | cursor display remains correct during wrap + resize churn |
| `TC-LIVE-04` | explorer operations validated under external FS drift |
| `TC-LIVE-05` | terminal output flood while editing adjacent buffer |
| `TC-LIVE-06` | IME composition races with leader mappings and window commands |

Canonical `*R` test IDs for these classes are defined in
[/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md).

## Bug-Fix Closure Contract

For each fixed user-visible bug:

1. add one failing regression test that reproduces the bug
2. implement fix
3. pass regression + associated live E2E test
4. update TODO checkbox with evidence link

## Determinism Rules

- use bounded timeouts with explicit diagnostics
- avoid blind sleep; poll with deterministic deadlines
- use fixed seeds/fixtures where randomness exists
- include mode, focused window, cursor, and visible frame excerpt on failure

## Completion Gate

A TODO item may be checked only when all are true:

1. linked normative spec requirement exists
2. deterministic evidence exists in required tier
3. behavior is reachable from real key/command path
4. unresolved user-visible gaps remain listed in `LIMITATIONS`

## Related

- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Unit requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
