# Mutation Testing Guidance

Back: [/docs/technical/testing/README.md](/docs/technical/testing/README.md)

Mutation testing validates whether tests can detect realistic logic defects.

## Purpose

| Outcome | Meaning |
|---|---|
| Killed mutant | Existing tests detected the behavior change. |
| Survived mutant | Test suite likely has a coverage or assertion gap. |
| Timeout mutant | Mutation produced non-terminating path; investigate test timeout handling. |

## Recommended tooling

Use Rust mutation tooling (for example `cargo-mutants`) against focused crates first, then broaden scope.

## Where mutation testing adds the most value

| Area | Why |
|---|---|
| Command parser and range parser | Small logic changes can silently alter semantics. |
| Cursor and mode transitions | Off-by-one mutations are common and high-impact. |
| Search/substitute flags | Branch behavior is dense and easy to under-test. |
| Viewport follow and wrap logic | Boundary conditions are difficult and regression-prone. |

## Execution strategy

| Step | Guidance |
|---|---|
| Scope selection | Start with touched modules and known bug-prone areas. |
| Baseline run | Ensure ordinary tests pass before running mutation suite. |
| Triage | For each survived mutant, decide: add test, or classify as equivalent. |
| Follow-up | If equivalent, document rationale; otherwise add failing test and fix. |

## CI posture

Mutation testing SHOULD run on schedule or pre-release, not necessarily every commit.

If runtime cost is high, use diff-scoped mutation runs for fast feedback and periodic full runs for confidence.

## Quality gate

A change touching high-risk logic SHOULD NOT be considered complete when obvious non-equivalent mutants survive in that area.
