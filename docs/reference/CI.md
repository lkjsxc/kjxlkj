# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- `/.github/workflows/ci.yml`

In docs-only baseline state, this workflow may be absent and must be regenerated.

## Current Baseline State (2026-02-11)

- active profile target: `Docs-only`
- implementation workspace is intentionally absent
- release profile is blocked until reconstruction closes high-severity rows

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| Docs-only | source artifacts absent by design | docs link/path checks + policy checks |
| Reconstructed-basic | workspace exists, blocker work not yet complete | docs checks + `cargo fmt --all -- --check` + `cargo clippy --workspace --all-targets` + `cargo test --workspace` |
| Blocker-revalidation | contradiction exists between user report and existing tests | reconstructed-basic + targeted bug repro + live PTY `*R` tests with screen-state assertions |
| Release | preparing release tag | blocker-revalidation + no high-severity open limitations |

## Local Reproduction

Run checks from repository root.

For docs-only profile, run documentation integrity checks only.
For reconstructed profiles, include required cases from
[/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md).

## Evidence Rule

CI status claims in `CONFORMANCE` or release docs must include:

- check profile name
- absolute date
- key pass/fail signals
- explicit mention of open blocker rows, if any
