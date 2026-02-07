# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)
How to cut a release from this repository.

The canonical CI pipeline location is `/.github/workflows/ci.yml` (a derived artifact that may be absent in a docs-only baseline).

Release automation (tagging and publishing binaries) is still treated as a manual process and should be updated when automation is introduced.

This process applies only when a shippable reconstructed repository state exists (Cargo workspace, packaging, and verification artifacts are present). In a docs-only baseline, reconstruct first:

- [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)

## Versioning

Use Semantic Versioning:

- MAJOR: breaking changes
- MINOR: new features, backwards compatible
- PATCH: bug fixes

## Pre-release checklist

1. CI is green (see `/docs/reference/CI.md`).
2. All tests pass (`cargo test --workspace`).
3. Lints were reviewed (`cargo clippy --workspace --all-targets`) and any accepted warnings are tracked in limitations/log records.
4. Formatting is clean (`cargo fmt --all -- --check`).
5. Docs are consistent with the intended release surface:
   - `/README.md` (high-level entrypoint)
   - `/docs/reference/CONFORMANCE.md` (what is implemented)
   - `/docs/reference/LIMITATIONS.md` (user-visible gaps)
6. Conformance claims pass the evidence gate:
   - every `implemented` claim is reachable from a user path and backed by deterministic tests
   - user-visible exceptions are recorded in `/docs/reference/LIMITATIONS.md`
   - no contradiction remains between conformance claims and release behavior
7. The version is updated in the workspace manifest (`Cargo.toml`) once it exists.

## Release steps (manual)

1. Create a release commit on the main branch.
2. Tag the commit (annotated tag recommended).
3. Build binaries for your target platforms.
4. Publish artifacts (GitHub Releases or your chosen distribution method).
5. Record release notes based on the doc set and conformance ledger.

## Post-release

- Start the next development cycle by updating the TODO iteration under `/docs/todo/`.
