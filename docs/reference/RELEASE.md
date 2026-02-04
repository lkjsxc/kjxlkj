# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)
How to cut a release from this repository.

This repository does not currently include a committed CI/release pipeline. Treat this as a documented manual process and update it when automation is introduced.

## Versioning

Use Semantic Versioning:

- MAJOR: breaking changes
- MINOR: new features, backwards compatible
- PATCH: bug fixes

## Pre-release checklist

1. All tests pass (`cargo test`).
2. Lints are clean (`cargo clippy` as appropriate).
3. Docs are consistent with the shipped surface:
   - `/README.md` (high-level entrypoint)
   - `/docs/reference/CONFORMANCE.md` (what is implemented)
   - `/docs/reference/LIMITATIONS.md` (user-visible gaps)
4. The version is updated in the workspace manifest.

## Release steps (manual)

1. Create a release commit on the main branch.
2. Tag the commit (annotated tag recommended).
3. Build binaries for your target platforms.
4. Publish artifacts (GitHub Releases or your chosen distribution method).
5. Record release notes based on the doc set and conformance ledger.

## Post-release

- Start the next development cycle by updating the TODO iteration under `/docs/todo/`.
