# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)
Continuous integration for this repository.

CI is required by policy (see `/docs/policy/WORKFLOW.md`).

## Location

CI is defined in:

- `/.github/workflows/ci.yml`

## What CI verifies

CI MUST remain green for changes to be considered shippable.

Checks:

- Documentation policy checks (structure + fence rules) via `python .github/scripts/check_docs_policy.py`
- Formatting via `cargo fmt --all -- --check`
- Linting via `cargo clippy --workspace --all-targets -- -D warnings`
- Tests via `cargo test --workspace`
- Docker buildability via `docker build -t kjxlkj:ci .`

## Local reproduction

The CI checks are intended to be reproducible locally by running the same commands listed above from the repo root.
