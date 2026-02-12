# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- `/.github/workflows/ci.yml`

If workflow files are missing or stale, regenerate from this contract.

## Baseline State (2026-02-12)

- active profile target: `Reconstruction-prep`
- documentation is canonical
- release profile is blocked until high-severity limitations close

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation updates | link/path checks + structural policy checks |
| `Reconstructed-basic` | workspace rebuild in progress | `Docs-integrity` + `cargo fmt --all -- --check` + `cargo clippy --workspace --all-targets` + `cargo test --workspace` |
| `Blocker-revalidation` | blocker fix is claimed | `Reconstructed-basic` + targeted blocker regressions + matching PTY `*R` tests |
| `Release` | release candidate | `Blocker-revalidation` + no open high-severity limitation rows |

## Evidence Rule

CI status claims in reference ledgers must include:

- profile name
- absolute date
- key pass/fail signals
- explicit mention of open blocker rows
