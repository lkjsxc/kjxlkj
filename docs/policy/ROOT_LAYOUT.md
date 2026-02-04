# Root Layout Policy

This document defines the allowed top-level layout and the required implementation placement.

## Root allowlist

The repo root SHOULD contain only the following entries:

| Path | Purpose |
|---|---|
| `README.md` | Project index and pointers |
| `LICENSE` | License |
| `docs/` | Canonical specification and policies |
| `src/` | Rust implementation workspace |
| `Cargo.toml` | Workspace manifest |
| `Cargo.lock` | Locked dependencies |
| `.gitignore` | Git ignore rules |
| `rust-toolchain.toml` | Toolchain pinning for reproducible builds and CI |
| `.github/` | CI configuration (GitHub Actions) and automation (Dependabot) |
| `Dockerfile` | Docker image definition |
| `.dockerignore` | Docker build context filtering |

Additional root entries MUST be justified by a concrete need and recorded in:

- `/docs/reference/IMPLEMENTATION_HISTORY.md`

## Tracked vs untracked (normative)

This allowlist governs root entries that are expected to be tracked by git when present.

Derived build artifacts (for example `target/`) MUST NOT be tracked. They may exist locally and may be deleted at any time, especially when preparing a clean docs-only baseline.

## Docs-only baseline (reconstruction scenario)

This repository is designed to allow deleting derived artifacts and reconstructing them from `/docs/`.

It is acceptable to temporarily reduce the repo to a docs-only baseline that includes only:

- `docs/`
- `README.md`
- `LICENSE`
- a minimal set of dotfiles required for repository hygiene and version control

In such a baseline, derived automation artifacts are commonly absent (and MUST be regenerated when producing a shippable state), including:

- `rust-toolchain.toml` (toolchain pinning)
- `.github/` (CI and automation)
- `Dockerfile` and `.dockerignore` (container build/run support)

After reconstruction, the repository SHOULD include the full root layout required by the current docs (workspace manifest, crate tree, verification automation, and any required packaging/run tooling), while still obeying the root allowlist above.

## Implementation layout

| Path | Requirement |
|---|---|
| `src/crates/` | Workspace crates MUST live here |
| `src/crates/kjxlkj/` | The single shipped binary crate MUST live here |
