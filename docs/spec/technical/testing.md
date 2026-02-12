# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction.

## Objectives

| Objective | Requirement |
|---|---|
| correctness | user-visible behavior is proven by deterministic tests |
| reproducibility | same scenario yields identical traces and frame assertions |
| regression prevention | every bug fix adds a stable failing-then-passing test |
| closure discipline | blocker rows close only with matching `T0` + `T1` + `T2` evidence |

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | local invariants | deterministic unit tests |
| `T1` | cross-module behavior | state/action integration tests |
| `T2` | user-like runtime proof | PTY E2E with per-key frame/state assertions |

High-severity blocker closure requires `T2`.

## Selected Existing Suites (Retain)

These existing tests are high-signal and must be retained during rebuild.

| Domain | Preferred Existing Tests | Why High Leverage |
|---|---|---|
| mode-entry append | `src/crates/core/kjxlkj-core-state/src/editor.rs` (`shift_a_appends_at_eol`) | directly probes known `a`/`A` path |
| command-line execution | `src/crates/core/kjxlkj-core-state/src/editor_cmdline_tests.rs` | validates command pipeline editing and register side effects |
| window navigation | `src/crates/core/kjxlkj-core-state/src/editor_wincmd_tests.rs` | deterministic `Ctrl-w` behavior coverage |
| split/explorer lifecycle | `src/crates/core/kjxlkj-core-state/src/editor_stage04_tests.rs`, `editor_stage04b_tests.rs` | catches mixed-pane regressions |
| race and boundaries | `src/crates/core/kjxlkj-core-state/src/editor_race_tests.rs`, `editor_boundary_tests.rs` | high failure-detection density |
| viewport behavior | `src/crates/core/kjxlkj-core-ui/src/viewport_tests.rs` | wrap/scroll clamp invariants |
| key normalization | `src/crates/platform/kjxlkj-input/src/normalize.rs` tests | critical `Shift+a -> A` normalization proof |
| terminal parser/screen | `src/crates/services/kjxlkj-service-terminal/src/parser_tests.rs`, `screen_tests.rs` | escape/render baseline correctness |

## Mandatory Blocker Regression Pack

| ID | Target Failure | Minimum Tier | Notes |
|---|---|---|---|
| `KEYMODE-04R` | `a` at EOL behaves like `i` | `T2` | compare per-key cursor transition and resulting frame |
| `CMD-02R` | `:q`, `:e`, `:w` wrong scope | `T2` | verify focused-window-only behavior in split layout |
| `WRAP-11R` | long-line overflow | `T2` | assert no off-screen cells |
| `FS-03R` | read/write round-trip failure | `T2` | verify content persistence and modified flag transitions |
| `JP-09R` | Japanese rendering/composition instability | `T2` | combine IME composition with resize churn |
| `UI-02R` | line-number/continuation drift under wrap | `T2` | assert gutter row identity under resize and rewrap |

## Bug-Fix Closure Workflow

For every user-visible bug:

1. reproduce in `T1` integration test
2. reproduce in matching `T2` PTY test when blocker severity is high
3. implement fix
4. pass new regression tests and retained baseline suites
5. synchronize reference and TODO ledgers in the same change

## Determinism Rules

- use bounded waits with explicit timeout diagnostics
- avoid blind sleeps in `T2`
- capture failure artifacts: mode, focused pane, layout summary, cursor, frame diff, recent input
- do not rely on external network state

## Completion Gate

A TODO checkbox may be marked complete only when all are true:

1. linked spec requirement exists
2. required tier evidence exists (`T2` for blockers)
3. behavior is reachable through user-visible key/command paths
4. frame assertions pass for matching E2E rows
5. `CONFORMANCE`, `LIMITATIONS`, `DRIFT_MATRIX`, and TODO rows are synchronized

## Related

- unit baseline: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- E2E baseline: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- PTY harness: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
