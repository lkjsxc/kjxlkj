# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)
Continuous integration for this repository.

CI is required by policy (see `/docs/policy/WORKFLOW.md`).

## Status (derived artifact)

This repository may temporarily be in a docs-only baseline, or may have derived automation artifacts pruned.

CI configuration and helper scripts are treated as derived artifacts: when absent, they MUST be reconstructed to satisfy policy before treating the repo as “shippable”.

## Location

The canonical CI location is:

- `/.github/workflows/ci.yml`

## What CI verifies

CI MUST remain green for changes to be considered shippable.

Checks:

- Documentation policy checks (structure + fence rules) via `python .github/scripts/check_docs_policy.py`
- Formatting via `cargo fmt --all -- --check`
- Linting via `cargo clippy --workspace --all-targets -- -D warnings`
- Tests via `cargo test --workspace`
- Docker buildability via `docker build -t kjxlkj:ci .` (when `Dockerfile` exists per policy)

## Local reproduction

The CI checks are intended to be reproducible locally by running the same commands listed above from the repo root.

In a docs-only baseline, Cargo- and Docker-based checks apply only after the workspace and packaging artifacts are reconstructed.
