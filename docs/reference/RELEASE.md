# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)
How to cut a release from this repository.

This repository includes a committed CI pipeline under `/.github/workflows/ci.yml`.

Release automation (tagging and publishing binaries) is still treated as a manual process and should be updated when automation is introduced.

## Versioning

Use Semantic Versioning:

- MAJOR: breaking changes
- MINOR: new features, backwards compatible
- PATCH: bug fixes

## Pre-release checklist

1. CI is green (see `/docs/reference/CI.md`).
2. All tests pass (`cargo test --workspace`).
3. Lints are clean (`cargo clippy --workspace --all-targets -- -D warnings`).
4. Formatting is clean (`cargo fmt --all -- --check`).
5. Docs are consistent with the shipped surface:
   - `/README.md` (high-level entrypoint)
   - `/docs/reference/CONFORMANCE.md` (what is implemented)
   - `/docs/reference/LIMITATIONS.md` (user-visible gaps)
6. The version is updated in the workspace manifest.

## Release steps (manual)

1. Create a release commit on the main branch.
2. Tag the commit (annotated tag recommended).
3. Build binaries for your target platforms.
4. Publish artifacts (GitHub Releases or your chosen distribution method).
5. Record release notes based on the doc set and conformance ledger.

## Post-release

- Start the next development cycle by updating the TODO iteration under `/docs/todo/`.
