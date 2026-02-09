# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Defines the reproducible verification gate.

## Canonical Location

- `/.github/workflows/ci.yml`

In docs-only baseline state, this file may be intentionally absent and must be
regenerated during reconstruction.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| Docs-only | Source artifacts absent by design | internal doc link/path checks, policy checks |
| Reconstructed | Workspace exists | docs checks + `cargo fmt --check` + `cargo clippy --workspace --all-targets` + `cargo test --workspace` |

## Local Reproduction

Run the profile-appropriate checks from repository root.

If running in reconstructed profile, minimum gate is:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets`
3. `cargo test --workspace`

## Evidence Rule

CI status claims in `CONFORMANCE` or release docs MUST include a dated evidence pointer.
