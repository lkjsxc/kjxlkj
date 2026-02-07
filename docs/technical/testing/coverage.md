# Coverage Posture

Back: [/docs/technical/testing/README.md](/docs/technical/testing/README.md)

Coverage is a risk signal, not a release criterion by itself.

## Principles

| Principle | Guidance |
|---|---|
| Behavior-first | Prioritize coverage for critical behaviors and invariants over raw percentages. |
| Risk-weighted | Higher-risk areas (mode transitions, file write paths, command parsing, viewport logic) require denser tests. |
| Regression-linked | Fixed bugs MUST add tests in the exact risk area where failure occurred. |

## Suggested targets (non-blocking)

| Area | Suggested line coverage |
|---|---|
| Core state/mode/dispatch crates | >= 80% |
| Text model and editing primitives | >= 80% |
| Host/input/render integration | >= 70% |
| Service scaffolding and adapters | >= 65% |

These percentages are advisory. Missing critical scenario tests is unacceptable even with high line coverage.

## Minimum required coverage characteristics

| Characteristic | Requirement |
|---|---|
| Mode transitions | Enter/exit paths for Normal/Insert/Visual/Command/Replace are tested. |
| Cursor invariants | End-exclusive vs end-inclusive clamping is tested. |
| Wrap/viewport | Long-line wrap and cursor visibility are tested. |
| Command parsing | Range/address parsing and error flows are tested. |
| Persisted outcomes | `:w`, `:wq`, and related write paths are tested via file assertions. |

## Tooling guidance

`cargo-llvm-cov` is recommended for line/branch coverage collection in Rust workspaces.

Mutation testing and PTY E2E suites SHOULD be used together with coverage to avoid false confidence.

## Reporting

Coverage reports SHOULD be stored as CI artifacts when available.

Coverage decreases are acceptable only when:

- risk is unchanged or reduced
- critical-path behavior coverage is preserved
- rationale is recorded in the change log or PR description
