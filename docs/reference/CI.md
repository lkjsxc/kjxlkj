# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- `/.github/workflows/ci.yml`

In docs-only baseline state, this workflow may be absent and must be regenerated.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| Docs-only | source artifacts absent by design | docs link/path checks + policy checks |
| Reconstructed-basic | workspace exists, blocker work not yet complete | docs checks + `cargo fmt --all -- --check` + `cargo clippy --workspace --all-targets` + `cargo test --workspace` |
| Blocker-closure | high-severity blocker rows are being closed | reconstructed-basic checks + blocker regression tests + live PTY E2E tests (`*R`) |
| Release | preparing release tag | blocker-closure checks + no high-severity open limitations |

## Local Reproduction

Run the profile-appropriate checks from repository root.

For blocker-closure and release profiles, include live E2E scenarios from [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md).

## Evidence Rule

CI status claims in `CONFORMANCE` or release docs MUST include:

- check profile name
- absolute date
- key pass signal (for example test totals or blocker suite summary)
